#!/usr/bin/env python3
"""Generate an evidence-scoped Amiga 3.5 candidate verification report."""

from __future__ import annotations

import argparse
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = Path("docs/research/amiga-gfa-basic-candidates.toml")
DEFAULT_OUTPUT = Path("docs/research/amiga-gfa-basic-candidate-report.md")
STATUSES = {"candidate", "confirmed-in-3.5", "contradicted", "documented-in-3.0-only", "unresolved"}
REQUIRED = {"id", "name", "kind", "source", "source-version", "printed-pages", "status", "notes"}


def load_candidates(path: Path) -> list[dict]:
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    if data.get("schema-version") != 1:
        raise ValueError("unsupported candidate manifest schema-version")
    candidates = data.get("candidates")
    if not isinstance(candidates, list):
        raise ValueError("manifest must contain [[candidates]] entries")
    seen: set[str] = set()
    for item in candidates:
        missing = REQUIRED - item.keys()
        if missing:
            raise ValueError(f"candidate missing fields: {', '.join(sorted(missing))}")
        if item["id"] in seen:
            raise ValueError(f"duplicate candidate ID: {item['id']}")
        seen.add(item["id"])
        if item["status"] not in STATUSES:
            raise ValueError(f"{item['id']}: invalid status {item['status']!r}")
        if not item["printed-pages"] or not item["source"] or not item["source-version"]:
            raise ValueError(f"{item['id']}: source and printed page locators are required")
    return sorted(candidates, key=lambda item: item["id"])


def render(candidates: list[dict]) -> str:
    counts = {status: sum(item["status"] == status for item in candidates) for status in sorted(STATUSES)}
    lines = [
        "# Amiga GFA BASIC 3.5 candidate verification report",
        "",
        "> Generated from `amiga-gfa-basic-candidates.toml`. Status describes evidence of 3.5 applicability, not implementation or runtime support.",
        "",
        "## Summary",
        "",
        "| Status | Entries |",
        "|---|---:|",
    ]
    lines.extend(f"| {status} | {counts[status]} |" for status in sorted(counts))
    lines.extend(["", "## Unresolved and pending candidates", "",
                  "| ID | Candidate | Source scope | Printed pages | Notes |",
                  "|---|---|---|---|---|"])
    pending = [item for item in candidates if item["status"] in {"candidate", "unresolved"}]
    for item in pending:
        pages = ", ".join(item["printed-pages"])
        note = item["notes"].replace("|", "\\|")
        lines.append(f"| `{item['id']}` | {item['name']} | {item['source-version']} | {pages} | {note} |")
    if not pending:
        lines.append("| — | None | — | — | — |")
    lines.extend(["", "## Documented in the 3.0 base only (3.5 applicability not established)", "",
                  "| ID | Candidate | Printed pages | Evidence boundary |",
                  "|---|---|---|---|"])
    base_only = [item for item in candidates if item["status"] == "documented-in-3.0-only"]
    for item in base_only:
        note = item["notes"].replace("|", "\\|")
        pages = ", ".join(item["printed-pages"])
        lines.append(f"| `{item['id']}` | {item['name']} | {pages} | {note} |")
    if not base_only:
        lines.append("| — | None | — | — |")
    lines.extend(["", "## Explicitly confirmed by the 3.5 addendum", "",
                  "| ID | Declaration family | Printed pages | Qualification |",
                  "|---|---|---|---|"])
    confirmed = [item for item in candidates if item["status"] == "confirmed-in-3.5"]
    for item in confirmed:
        note = item["notes"].replace("|", "\\|")
        lines.append(f"| `{item['id']}` | {item['name']} | {', '.join(item['printed-pages'])} | {note} |")
    if not confirmed:
        lines.append("| — | None | — | — | — |")
    contradicted = [item for item in candidates if item["status"] == "contradicted"]
    lines.extend(["", "## Contradicted candidates", ""])
    if contradicted:
        for item in contradicted:
            lines.append(f"- `{item['id']}` — {item['name']}: {item['notes']}")
    else:
        lines.append("No candidates are currently classified as contradicted. This is not evidence that all remaining candidates are compatible.")
    lines.extend([
        "",
        "## Interpretation and limitations",
        "",
        "- `documented-in-3.0-only` closes the source audit: the cited evidence is from the 3.0 base manual, and neither the 3.5 addendum nor a reproducible runtime observation establishes 3.5 applicability. This does not mean removed, incompatible, or confirmed in 3.5.",
        "- `unresolved` is reserved for candidates whose source scope or declaration identity still needs research.",
        "- A confirmed family does not settle every overload or edge case; qualifications in the manifest and detailed research notes remain authoritative.",
        "- Chapter 12 indexes and reference tables are navigation aids, not declarations or proof of availability.",
        "- No absence-based removal or compatibility inference is made.",
        "",
        "Source: `AM-INTERPRETER-35` in `docs/reference-sources.md`; detailed page notes are in `amiga-gfa-basic-3.0-base-inventory.md` and `amiga-gfa-basic-35-addendum-inventory.md`.",
        "",
    ])
    return "\n".join(lines)


def generate(root: Path, manifest: Path, output: Path, check: bool = False) -> bool:
    candidates = load_candidates(root / manifest)
    expected = render(candidates)
    destination = root / output
    if check:
        return destination.is_file() and destination.read_text(encoding="utf-8") == expected
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(expected, encoding="utf-8")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        valid = generate(args.root.resolve(), args.manifest, args.output, args.check)
    except (OSError, ValueError, tomllib.TOMLDecodeError) as exc:
        parser.error(str(exc))
    if args.check:
        if not valid:
            print("ERROR: Amiga candidate report is missing or stale")
            return 1
        print("Amiga candidate report is up to date")
    else:
        print(f"Generated {args.output}")
    return 0 if valid else 1


if __name__ == "__main__":
    raise SystemExit(main())
