#!/usr/bin/env python3
"""Preflight private GitHub Contents API paths without downloading asset bytes."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import quote
from urllib.request import Request, urlopen


def read_manifest(path: Path) -> list[str]:
    """Read label|filename rows and return the configured filenames."""
    filenames: list[str] = []
    for line_number, raw_line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        try:
            label, filename = line.split("|", 1)
        except ValueError as exc:
            raise ValueError(f"{path}:{line_number}: expected label|filename") from exc
        if not label.strip() or not filename.strip():
            raise ValueError(f"{path}:{line_number}: label and filename must be non-empty")
        filenames.append(filename.strip())
    if not filenames:
        raise ValueError(f"{path}: no asset paths configured")
    return filenames


def check_asset_paths(
    repo: str,
    ref: str,
    paths: list[str],
    token: str,
    opener=urlopen,
) -> list[str]:
    """Return paths that are absent or are not regular files; never fetch file bytes."""
    missing: list[str] = []
    for path in paths:
        encoded = quote(path, safe="")
        url = f"https://api.github.com/repos/{repo}/contents/{encoded}?ref={quote(ref, safe='')}"
        request = Request(
            url,
            headers={
                "Authorization": f"Bearer {token}",
                "Accept": "application/vnd.github+json",
                "X-GitHub-Api-Version": "2022-11-28",
            },
        )
        try:
            with opener(request, timeout=20) as response:
                metadata = json.load(response)
        except HTTPError as exc:
            if exc.code == 404:
                missing.append(path)
                continue
            raise RuntimeError(
                f"GitHub Contents API returned HTTP {exc.code} while checking asset path {path!r}"
            ) from None
        except (URLError, TimeoutError, OSError, json.JSONDecodeError):
            raise RuntimeError(f"Could not verify private asset path {path!r} through GitHub Contents API") from None
        if not isinstance(metadata, dict) or metadata.get("type") != "file":
            missing.append(path)
    return missing


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, help="Private repository in owner/name form")
    parser.add_argument("--ref", default="main", help="Git ref to check")
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--manifest", type=Path, help="UTF-8 label|filename manifest")
    source.add_argument("--path", action="append", help="Asset path; may be repeated")
    args = parser.parse_args()

    token = os.environ.get("GFA_ASSETS_TOKEN", "")
    if not token:
        parser.error("GFA_ASSETS_TOKEN is required in the environment")

    try:
        paths = read_manifest(args.manifest) if args.manifest else args.path
        missing = check_asset_paths(args.repo, args.ref, paths, token)
    except (OSError, ValueError, RuntimeError) as exc:
        print(f"ERROR: {exc}")
        return 1

    if missing:
        for path in missing:
            print(f"MISSING: {path}")
        return 1

    print(f"OK: verified {len(paths)} private asset path(s) in {args.repo}@{args.ref}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
