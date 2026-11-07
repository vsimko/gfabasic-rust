#!/usr/bin/env python3
"""Generate deterministic public API catalogue data from api/registry/*.toml."""

from __future__ import annotations

import argparse
import json
import tomllib
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_OUTPUT = Path("site/data/api-catalogue.json")


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


def render(root: Path) -> str:
    entries = load_entries(root)
    result = {
        "schema_version": 1,
        "generated_from": "api/registry/**/*.toml",
        "counts": summarize(entries),
        "entries": entries,
        "coverage_note": (
            "Evidence, implementation, target support, and conformance are independent dimensions. "
            "Unknown, disputed, and not-tested entries remain visible; no aggregate implementation "
            "percentage is calculated."
        ),
    }
    return json.dumps(result, indent=2, ensure_ascii=False) + "\n"


def generate(root: Path, output: Path, check: bool = False) -> bool:
    expected = render(root)
    if check:
        if not output.is_file():
            return False
        return output.read_text(encoding="utf-8") == expected
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(expected, encoding="utf-8")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--check", action="store_true", help="fail if generated data is missing or stale")
    args = parser.parse_args()
    root = args.root.resolve()
    output = args.output if args.output.is_absolute() else root / args.output
    try:
        current = generate(root, output, check=args.check)
    except (OSError, KeyError, ValueError, tomllib.TOMLDecodeError) as exc:
        parser.error(str(exc))
    if args.check:
        if not current:
            print(f"ERROR: generated API catalogue is missing or stale: {output}")
            return 1
        print(f"API catalogue data is up to date: {output.relative_to(root)}")
    else:
        print(f"Generated API catalogue data: {output.relative_to(root)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
