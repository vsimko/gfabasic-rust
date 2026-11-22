#!/usr/bin/env python3
"""Fail-closed allowlist for public GFA-BASIC 3.5 discovery artifacts."""

from __future__ import annotations

import argparse
import base64
import binascii
import pathlib
import re
import stat
import struct
import sys
import unicodedata
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
MAX_PNG_PIXELS = 16_000_000
MAX_PNG_RASTER_BYTES = 64 * 1024 * 1024


def valid_png(data: bytes) -> bool:
    """Validate PNG framing, CRCs, complete raster data, and bounded decoding."""
    signature = b"\x89PNG\r\n\x1a\n"
    if not data.startswith(signature):
        return False
    offset = len(signature)
    seen_ihdr = False
    seen_idat = False
    seen_iend = False
    compressed = bytearray()
    width = height = depth = color = interlace = 0
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
        # Only core raster chunks are needed; ancillary chunks can hide payloads.
        if chunk_type not in {b"IHDR", b"IDAT", b"IEND"}:
            return False
        if not seen_ihdr:
            if chunk_type != b"IHDR" or length != 13:
                return False
            width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", chunk_data)
            depths = {0: {1, 2, 4, 8, 16}, 2: {8, 16}, 3: {1, 2, 4, 8}, 4: {8, 16}, 6: {8, 16}}
            if (width == 0 or height == 0 or depth not in depths.get(color, set())
                    or compression != 0 or filtering != 0 or interlace not in {0, 1}
                    or width * height > MAX_PNG_PIXELS):
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

    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[color]
    bits_per_pixel = channels * depth
    passes = (
        ((0, 0, 1, 1),) if interlace == 0 else
        ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8),
         (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2))
    )
    pass_layout: list[tuple[int, int]] = []
    expected_raster_bytes = 0
    for x_start, y_start, x_step, y_step in passes:
        pass_width = (width - x_start + x_step - 1) // x_step if width > x_start else 0
        pass_height = (height - y_start + y_step - 1) // y_step if height > y_start else 0
        if not pass_width or not pass_height:
            continue
        row_bytes = (pass_width * bits_per_pixel + 7) // 8
        pass_layout.append((row_bytes, pass_height))
        expected_raster_bytes += (row_bytes + 1) * pass_height
        if expected_raster_bytes > MAX_PNG_RASTER_BYTES:
            return False

    if expected_raster_bytes == 0:
        return False
    try:
        decoder = zlib.decompressobj()
        raster = decoder.decompress(bytes(compressed), expected_raster_bytes + 1)
        # The extra byte detects expansion beyond the IHDR-derived bound.
        if (len(raster) != expected_raster_bytes or not decoder.eof
                or decoder.unused_data or decoder.unconsumed_tail):
            return False
    except (zlib.error, struct.error, OverflowError):
        return False

    cursor = 0
    for row_bytes, pass_height in pass_layout:
        scanline_size = row_bytes + 1
        for _ in range(pass_height):
            if raster[cursor] > 4:
                return False
            cursor += scanline_size
    return cursor == len(raster)


def contains_encoded_payload(text: str) -> bool:
    """Reject long Base64/hex blobs, including whitespace-obfuscated encodings."""
    base64_alphabet = r"A-Za-z0-9+/_-"
    candidates = [
        match.group(0)
        for match in re.finditer(
            rf"(?<![{base64_alphabet}])[{base64_alphabet}]{{128,}}={{0,2}}(?![{base64_alphabet}=])",
            text,
        )
    ]
    # Attackers can split a single encoded value with spaces, tabs, or newlines.
    # Keep separators limited to whitespace and encoded-character groups so prose
    # punctuation cannot be silently discarded while reconstructing a candidate.
    candidates.extend(
        match.group(0)
        for match in re.finditer(
            rf"(?<![{base64_alphabet}])[{base64_alphabet}]{{1,76}}"
            rf"(?:[ \\t\\r\\n]+[{base64_alphabet}]{{1,76}})+={{0,2}}"
            rf"(?![{base64_alphabet}=])",
            text,
        )
    )
    wrapped_lines: list[str] = []
    for line in text.splitlines():
        candidate = line.strip()
        if candidate and re.fullmatch(rf"[{base64_alphabet}]+={{0,2}}", candidate):
            wrapped_lines.append(candidate)
        elif wrapped_lines:
            candidates.append("".join(wrapped_lines))
            wrapped_lines = []
    if wrapped_lines:
        candidates.append("".join(wrapped_lines))

    for candidate in candidates:
        compact = "".join(candidate.split())
        if len(compact) < 128:
            continue
        try:
            decoded = base64.b64decode(
                compact + "=" * (-len(compact) % 4), altchars=b"-_", validate=True
            )
        except (ValueError, binascii.Error):
            continue
        if len(decoded) >= 96:
            return True

    hex_candidates = [
        match.group(0)
        for match in re.finditer(
            r"(?<![0-9A-Fa-f])[0-9A-Fa-f]{1,64}(?:[ \\t\\r\\n]+[0-9A-Fa-f]{1,64}){2,}(?![0-9A-Fa-f])",
            text,
        )
    ]
    if any(
        len(match.group(0)) >= 192
        for match in re.finditer(r"(?<![0-9A-Fa-f])[0-9A-Fa-f]{192,}(?![0-9A-Fa-f])", text)
    ):
        return True
    return any(len("".join(candidate.split())) >= 192 for candidate in hex_candidates)

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
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            violations.append(f"non-UTF-8 text evidence: {relative}")
            continue

        if any(unicodedata.category(char) == "Cc" and char not in "\t\n\r" for char in text):
            violations.append(f"control character in text evidence: {relative}")
            continue
        if relative == "archive-info.txt" and not re.fullmatch(r"archive_size=[0-9]+(?:\\n|\r?\n)?", text):
            violations.append("invalid text format: archive-info.txt")
            continue
        if contains_encoded_payload(text):
            violations.append(f"encoded payload in text evidence: {relative}")

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
