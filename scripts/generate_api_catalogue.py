#!/usr/bin/env python3
"""Generate a compact static API catalogue index and per-declaration details."""

from __future__ import annotations

import argparse
import json
import re
import tomllib
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_OUTPUT = Path("site/data/api-catalogue.json")
DEFAULT_DETAILS = Path("site/data/api-catalogue-details")
ID_PATTERN = re.compile(r"^[a-zA-Z0-9._-]+$")


def load_entries(root: Path) -> list[dict]:
    registry_root = root / "api" / "registry"
    entries = []
    seen_ids = set()
    for path in sorted(registry_root.rglob("*.toml")):
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        namespace = path.parent.name
        category = path.stem
        for declaration in data.get("declarations", []):
            declaration_id = declaration["id"]
            if not ID_PATTERN.fullmatch(declaration_id):
                raise ValueError(f"invalid declaration ID for static detail filename: {declaration_id}")
            if declaration_id in seen_ids:
                raise ValueError(f"duplicate declaration ID: {declaration_id}")
            seen_ids.add(declaration_id)
            evidence = declaration.get("evidence", {})
            implementation = declaration.get("implementation", {})
            entries.append({
                "id": declaration_id,
                "name": declaration["name"],
                "kind": declaration["kind"],
                "namespace": namespace,
                "category": category,
                "availability": declaration.get("availability", []),
                "syntax": declaration.get("syntax", []),
                "signature": declaration.get("signature", {}),
                "semantics": declaration.get("semantics", {}),
                "evidence": evidence,
                "implementation": implementation,
                "support": declaration.get("support", []),
                "docs": declaration.get("docs", {}),
            })
    return sorted(entries, key=lambda item: (item["namespace"], item["category"], item["name"].casefold(), item["id"]))


def summarize(entries: list[dict]) -> dict:
    by_namespace = Counter(entry["namespace"] for entry in entries)
    by_evidence = Counter(entry.get("evidence", {}).get("status", "unknown") for entry in entries)
    by_implementation = Counter(entry.get("implementation", {}).get("status", "unknown") for entry in entries)
    by_target_support: dict[str, Counter] = defaultdict(Counter)
    by_target_conformance: dict[str, Counter] = defaultdict(Counter)
    for entry in entries:
        for support in entry.get("support", []):
            target = support.get("target", "unspecified")
            by_target_support[target][support.get("classification", "unknown")] += 1
            by_target_conformance[target][support.get("conformance", "not-tested")] += 1
    return {
        "total": len(entries),
        "by_namespace": dict(sorted(by_namespace.items())),
        "by_evidence_status": dict(sorted(by_evidence.items())),
        "by_implementation_status": dict(sorted(by_implementation.items())),
        "by_target_support": {
            target: dict(sorted(counts.items()))
            for target, counts in sorted(by_target_support.items())
        },
        "by_target_conformance": {
            target: dict(sorted(counts.items()))
            for target, counts in sorted(by_target_conformance.items())
        },
    }


def detail_filename(declaration_id: str) -> str:
    return f"{declaration_id}.json"


def render_index(root: Path) -> tuple[str, dict[str, str]]:
    entries = load_entries(root)
    details = {}
    index_entries = []
    for entry in entries:
        declaration_id = entry["id"]
        details[detail_filename(declaration_id)] = json.dumps(
            entry, indent=2, ensure_ascii=False
        ) + "\n"
        index_entries.append({
            "id": declaration_id,
            "name": entry["name"],
            "kind": entry["kind"],
            "namespace": entry["namespace"],
            "category": entry["category"],
            "detail": f"api-catalogue-details/{detail_filename(declaration_id)}",
            "syntax": [item.get("form", "") for item in entry.get("syntax", [])],
            "summary": entry.get("semantics", {}).get("summary", ""),
            "evidence_status": entry.get("evidence", {}).get("status", "unknown"),
            "implementation_status": entry.get("implementation", {}).get("status", "unknown"),
            "support": [
                {"target": item.get("target", "unspecified"),
                 "classification": item.get("classification", "unknown"),
                 "conformance": item.get("conformance", "not-tested")}
                for item in entry.get("support", [])
            ],
        })
    result = {
        "schema_version": 2,
        "generated_from": "api/registry/**/*.toml",
        "counts": summarize(entries),
        "entries": index_entries,
        "coverage_note": (
            "Evidence, implementation, target support, and conformance are independent dimensions. "
            "Unknown, disputed, and not-tested entries remain visible; no aggregate implementation "
            "percentage is calculated. Full declaration details are separate static JSON files."
        ),
    }
    return json.dumps(result, indent=2, ensure_ascii=False) + "\n", details


def render(root: Path) -> str:
    """Compatibility helper returning the generated compact index."""
    return render_index(root)[0]


def generate(root: Path, output: Path, details_dir: Path | None = None, check: bool = False) -> bool:
    details_dir = details_dir or (output.parent / DEFAULT_DETAILS.name)
    expected_index, expected_details = render_index(root)
    if check:
        if not output.is_file() or output.read_text(encoding="utf-8") != expected_index:
            return False
        if not details_dir.is_dir():
            return False
        actual_names = {path.name for path in details_dir.glob("*.json")}
        if actual_names != set(expected_details):
            return False
        return all(
            (details_dir / name).read_text(encoding="utf-8") == content
            for name, content in expected_details.items()
        )
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(expected_index, encoding="utf-8")
    details_dir.mkdir(parents=True, exist_ok=True)
    for name, content in expected_details.items():
        (details_dir / name).write_text(content, encoding="utf-8")
    for path in details_dir.glob("*.json"):
        if path.name not in expected_details:
            path.unlink()
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--details-dir", type=Path, default=DEFAULT_DETAILS)
    parser.add_argument("--check", action="store_true", help="fail if generated data is missing or stale")
    args = parser.parse_args()
    root = args.root.resolve()
    output = args.output if args.output.is_absolute() else root / args.output
    details_dir = args.details_dir if args.details_dir.is_absolute() else root / args.details_dir
    try:
        current = generate(root, output, details_dir, check=args.check)
    except (OSError, KeyError, ValueError, tomllib.TOMLDecodeError) as exc:
        parser.error(str(exc))
    if args.check:
        if not current:
            print(f"ERROR: generated API catalogue is missing or stale: {output} or {details_dir}")
            return 1
        print(f"API catalogue data is up to date: {output.relative_to(root)}")
    else:
        print(f"Generated API catalogue index and {len(list(details_dir.glob('*.json')))} static details")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
