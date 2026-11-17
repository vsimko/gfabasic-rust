#!/usr/bin/env python3
"""Incrementally build the public PDF source index from references/pdf-sources.json."""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import urllib.request
from io import BytesIO
from pathlib import Path
from typing import Any

from pypdf import PdfReader

MANIFEST = "references/pdf-sources.json"
INDEX_VERSION = 3


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8")) if path.is_file() else {}


def previous_manifest(base_sha: str | None) -> dict[str, Any]:
    if not base_sha or set(base_sha) == {"0"}:
        return {}
    result = subprocess.run(
        ["git", "show", f"{base_sha}:{MANIFEST}"],
        check=False, capture_output=True, text=True,
    )
    if result.returncode:
        return {}
    return json.loads(result.stdout)


def source_map(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    sources = manifest.get("sources", [])
    if not isinstance(sources, list):
        raise ValueError("Manifest must contain a 'sources' array")
    result: dict[str, dict[str, Any]] = {}
    for source in sources:
        source_id = source.get("source_id")
        if not isinstance(source_id, str) or not source_id:
            raise ValueError("Every source needs a non-empty source_id")
        if source_id in result:
            raise ValueError(f"Duplicate source_id: {source_id}")
        result[source_id] = source
    return result


def download(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": "gfabasic-reference-index/1.0"})
    with urllib.request.urlopen(request, timeout=90) as response:
        data = response.read()
    if not data.startswith(b"%PDF-"):
        raise ValueError(f"URL did not return a PDF: {url}")
    return data


def build_document(source: dict[str, Any]) -> dict[str, Any]:
    url = source["url"]
    data = download(url)
    reader = PdfReader(BytesIO(data), strict=False)
    if reader.is_encrypted and not reader.decrypt(""):
        raise ValueError(f"Encrypted PDF: {url}")
    page_count = len(reader.pages)
    if page_count < 1:
        raise ValueError(f"PDF has no pages: {url}")
    metadata = reader.metadata
    document: dict[str, Any] = {
        "source_id": source["source_id"],
        "title": str(metadata.title).strip() if metadata and metadata.title else source["title"],
        "source_url": url,
        "filename": source["filename"],
        "sha256": hashlib.sha256(data).hexdigest(),
        "page_count": page_count,
        "platform": source.get("platform"),
        "version": source.get("version"),
        "edition": source.get("edition"),
        "text_extraction": "included" if source.get("include_text") else "not-included",
        "page_locators": [
            {"pdf_page": number, "anchor": f"pdf-page-{number:04d}"}
            for number in range(1, page_count + 1)
        ],
    }
    if source.get("include_text"):
        rights_basis = source.get("rights_basis")
        if not rights_basis:
            raise ValueError(f"{source['source_id']} enables text extraction without rights_basis")
        document["rights_basis"] = rights_basis
        document["page_text"] = [
            {"pdf_page": number, "text": page.extract_text() or ""}
            for number, page in enumerate(reader.pages, start=1)
        ]
    return {key: value for key, value in document.items() if value is not None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=Path(MANIFEST))
    parser.add_argument("--base-sha", default="")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    current = source_map(read_json(args.manifest))
    previous = source_map(previous_manifest(args.base_sha))
    index_path = Path("references/source-index.json")
    existing = read_json(index_path)
    existing_docs = {d["source_id"]: d for d in existing.get("documents", [])}
    if not previous or not existing_docs:
        changed = set(current)
    else:
        changed = {
            source_id for source_id, source in current.items()
            if source != previous.get(source_id) or source_id not in existing_docs
        }
    removed = set(existing_docs) - set(current)
    documents = {key: value for key, value in existing_docs.items() if key not in removed}

    for source_id in sorted(changed):
        source = current[source_id]
        if not source.get("enabled", True):
            documents.pop(source_id, None)
            print(f"SKIP {source_id}: {source.get('pending_reason', 'disabled in manifest')}")
            continue
        documents[source_id] = build_document(source)
        print(f"UPDATED {source_id}: {documents[source_id]['page_count']} pages")

    payload = {
        "format_version": INDEX_VERSION,
        "generated_by": "scripts/build_reference_source_index.py",
        "manifest": MANIFEST,
        "purpose": "Bibliographic metadata, page locators, and manifest-authorized per-page extracted text",
        "documents": [documents[key] for key in sorted(documents)],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
