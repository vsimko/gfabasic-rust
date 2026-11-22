#!/usr/bin/env python3
"""Fail-closed allowlist for public GFA-BASIC 3.5 discovery artifacts."""

from __future__ import annotations

import argparse
import io
import pathlib
import stat
import sys

from PIL import Image


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
            try:
                with Image.open(io.BytesIO(data)) as image:
                    if image.format != "PNG":
                        raise ValueError("content is not PNG")
                    image.verify()
            except Exception as error:
                violations.append(f"invalid PNG evidence: {relative} ({error})")
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
