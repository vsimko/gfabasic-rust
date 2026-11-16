#!/usr/bin/env python3
"""Build a copyright-safe page locator index; never export PDF page text."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from pypdf import PdfReader


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def build(path: Path, source_id: str, source_url: str, output: dict) -> dict:
    reader = PdfReader(str(path), strict=False)
    if reader.is_encrypted and not reader.decrypt(""):
        raise ValueError(f"Encrypted PDF: {path}")
    metadata = reader.metadata
    title = str(metadata.title).strip() if metadata and metadata.title else path.stem
    pages = len(reader.pages)
    if pages < 1:
        raise ValueError(f"No pages in {path}")
    return {
        "source_id": source_id,
        "title": title,
        "source_url": source_url,
        "filename": path.name,
        "sha256": sha256(path),
        "page_count": pages,
        "page_locators": [
            {"pdf_page": number, "anchor": f"pdf-page-{number:04d}"}
            for number in range(1, pages + 1)
        ],
        "content_policy": "locator-only; no page text, OCR, scans, or source excerpts",
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--english", type=Path, required=True)
    parser.add_argument("--german", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--english-url", required=True)
    parser.add_argument("--german-url", required=True)
    args = parser.parse_args()
    docs = [
        build(args.english, "AM-INTERPRETER-35", args.english_url, {}),
        build(args.german, "AM-BOOK-DE-1989", args.german_url, {}),
    ]
    payload = {
        "format_version": 1,
        "generated_by": "scripts/build_reference_source_index.py",
        "purpose": "Public bibliographic and physical-page locator index",
        "documents": docs,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
