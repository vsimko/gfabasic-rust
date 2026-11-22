#!/usr/bin/env python3
"""Fail-closed allowlist for public GFA-BASIC 3.5 discovery artifacts."""

from __future__ import annotations

import argparse
import binascii
import pathlib
import stat
import struct
import sys
import zlib


# These are the only paths produced by the 3.5 discovery workflow and probe.
# New evidence files must be reviewed and added here deliberately.
ALLOWED_TEXT = {
    "archive-info.txt",
    "archive-members.txt",
    "adf-info.txt",
    "adf-files.txt",
    "interpreter-sha256.txt",
    "interpreter-size.txt",
    "interpreter-file.txt",
    "interpreter-strings.txt",
    "adf-sha256.txt",
    "adf-size.txt",
    "adf-member.txt",
    "boot-adf-info.txt",
    "kickstart-sha256.txt",
    "amiberry.log",
    "editor-version-menu-ocr.txt",
    "editor-version-result.txt",
    "native-run/amiberry.log",
    "native-run/test-disk-layout.txt",
    "native-run/test-disk-boot.txt",
    "native-run/gfatest.LST",
    "native-run/insert-floppy-ipc.txt",
    "native-run/list-floppies-ipc.txt",
    "native-run/after-shortcut-ocr.txt",
    "native-run/after-merge-ocr.txt",
    "native-run/result.txt",
    "native-run/gfatest-results.txt",
    "native-run/stdin-save.LST",
    "native-run/insert-floppy-ipc.txt.missing",
    "native-run/list-floppies-ipc.txt.missing",
    "native-run/stdin-save.LST.missing",
}

ALLOWED_PNG = {
    "editor-version-menu.png",
    "editor-after-menu.png",
    "editor-version-menu-ocr.png",
    "native-run/editor-before-merge.png",
    "native-run/merge-dialog.png",
    "native-run/merge-file-row-pointer.png",
    "native-run/merge-file-selected.png",
    "native-run/merge-gadget-pointer.png",
    "native-run/after-shortcut.png",
    "native-run/after-merge.png",
    "native-run/after-merge-ocr-input.png",
    "native-run/after-run-dialog.png",
    "native-run/after-run.png",
}

ALLOWED_PATHS = ALLOWED_TEXT | ALLOWED_PNG
MAX_FILE_BYTES = 10 * 1024 * 1024
MAX_TOTAL_BYTES = 40 * 1024 * 1024


def valid_png(data: bytes) -> bool:
    """Check PNG framing, chunk CRCs, required chunks, and IDAT zlib stream."""
    signature = b"\\x89PNG\\r\\n\\x1a\\n"
    if not data.startswith(signature):
        return False
    offset = len(signature)
    seen_ihdr = False
    seen_idat = False
    seen_iend = False
    compressed = bytearray()
    while offset + 12 <= len(data):
        length = struct.unpack(">I", data[offset:offset + 4])[0]
        chunk_type = data[offset + 4:offset + 8]
        end = offset + 12 + length
        if end > len(data):
            return False
        chunk_data = data[offset + 8:offset + 8 + length]
        expected_crc = struct.unpack(">I", data[offset + 8 + length:end])[0]
        if (binascii.crc32(chunk_type + chunk_data) & 0xFFFFFFFF) != expected_crc:
            return False
        if not seen_ihdr:
            if chunk_type != b"IHDR" or length != 13:
                return False
            width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", chunk_data)
            depths = {0: {1, 2, 4, 8, 16}, 2: {8, 16}, 3: {1, 2, 4, 8}, 4: {8, 16}, 6: {8, 16}}
            if (width == 0 or height == 0 or depth not in depths.get(color, set())
                    or compression != 0 or filtering != 0 or interlace not in {0, 1}):
                return False
            seen_ihdr = True
        elif chunk_type == b"IHDR":
            return False
        if chunk_type == b"IDAT":
            if seen_iend:
                return False
            seen_idat = True
            compressed.extend(chunk_data)
        elif chunk_type == b"IEND":
            if length != 0 or not seen_idat:
                return False
            seen_iend = True
            offset = end
            break
        offset = end
    if not seen_ihdr or not seen_idat or not seen_iend or offset != len(data):
        return False
    try:
        decoder = zlib.decompressobj()
        decoder.decompress(bytes(compressed))
        decoder.flush()
        return decoder.eof and not decoder.unused_data
    except zlib.error:
        return False


def validate_artifact(root: pathlib.Path) -> list[str]:
    """Return violations; an unknown path or unrecognized file always fails."""
    violations: list[str] = []
    if root.is_symlink() or not root.is_dir():
        return [f"artifact root is missing, not a directory, or a symlink: {root}"]

    total_bytes = 0
    seen: set[str] = set()
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root).as_posix()
        if path.is_symlink():
            violations.append(f"symlink: {relative}")
            continue
        if path.is_dir():
            if relative != "native-run":
                violations.append(f"unapproved directory: {relative}")
            continue
        if not path.is_file():
            violations.append(f"unsupported filesystem entry: {relative}")
            continue
        if relative not in ALLOWED_PATHS:
            violations.append(f"unapproved artifact path: {relative}")
            continue

        seen.add(relative)
        mode = path.stat().st_mode
        if not stat.S_ISREG(mode) or mode & (stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH):
            violations.append(f"not a non-executable regular file: {relative}")
            continue

        size = path.stat().st_size
        total_bytes += size
        if size > MAX_FILE_BYTES:
            violations.append(f"file exceeds {MAX_FILE_BYTES} bytes: {relative}")
            continue

        data = path.read_bytes()
        if relative in ALLOWED_PNG:
            if not valid_png(data):
                violations.append(f"invalid PNG evidence: {relative}")
            continue

        if b"\x00" in data:
            violations.append(f"NUL byte in text evidence: {relative}")
            continue
        try:
            data.decode("utf-8")
        except UnicodeDecodeError:
            violations.append(f"non-UTF-8 text evidence: {relative}")

    if total_bytes > MAX_TOTAL_BYTES:
        violations.append(f"artifact exceeds {MAX_TOTAL_BYTES} total bytes")

    if not seen:
        violations.append("artifact contains no approved evidence files")
    return violations


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=pathlib.Path, help="artifact staging directory")
    args = parser.parse_args()
    violations = validate_artifact(args.root)
    if violations:
        print("Artifact boundary violations:", file=sys.stderr)
        for violation in violations:
            print(f" - {violation}", file=sys.stderr)
        return 1
    print("Artifact contains only explicitly approved evidence paths and valid content.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
