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
EXPECTED_35_SOURCE_EVIDENCE = {
    "amiga.base2.hex": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-36"],"basis":"direct-example-use"},
    "amiga.base2.str": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-3"],"basis":"direct-documentation-reference"},
    "amiga.base4.rand": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-2"],"basis":"direct-example-use"},
    "amiga.base4.succ": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-5"],"basis":"direct-example-use"},
    "amiga.base4.string": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-21"],"basis":"direct-example-use"},
    "amiga.base4.rnd": {"source":"AM-BOOK-DE-1990","version":"3.5 companion book Chapter 18","pages":["458"],"basis":"direct-example-use"},
    "amiga.base4.int": {"source":"AM-BOOK-DE-1990","version":"3.5 companion book Chapter 18","pages":["458"],"basis":"direct-example-use"},
    "amiga.base4.sqr": {"source":"AM-BOOK-DE-1990","version":"3.5 companion book Chapter 18","pages":["473"],"basis":"direct-example-use"},
    "amiga.base4.inc-dec": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-35"],"basis":"direct-example-use"},
    "amiga.base5.data-read": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-35"],"basis":"direct-example-use"},
    "amiga.base5.print-write": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-2","13-34","13-35","13-36"],"basis":"direct-example-use"},
    "amiga.base7.for-next": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-35"],"basis":"direct-example-use"},
    "amiga.base7.do-loop": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-35"],"basis":"direct-example-use"},
    "amiga.base7.on-error": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-36"],"basis":"direct-documentation-reference"},
    "amiga.base7.resume": {"source":"AM-INTERPRETER-35","version":"3.5 addendum","pages":["13-36"],"basis":"direct-documentation-restriction"},
}


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
    # Selected Chapter 2–5 and 7 candidate families are checked against direct 3.5 evidence.
    # These records retain their 3.0 source origin; direct 3.5 evidence establishes
    # only the forms and contexts actually demonstrated or referenced.
    core_candidates = [
        item for item in candidates
        if item["id"].startswith(("amiga.base2.", "amiga.base3.", "amiga.base4.", "amiga.base5.", "amiga.base7."))
    ]
    if len(core_candidates) < 50:
        raise ValueError("Selected base-language evidence audit is incomplete")
    for item in core_candidates:
        if item["source"] != "AM-INTERPRETER-35" or item["source-version"] != "3.0 base manual":
            raise ValueError(f'{item["id"]}: Audited base candidate must retain 3.0 source scope')
        if item["status"] == "confirmed-in-3.5":
            if (
                item.get("confirmation-source") != EXPECTED_35_SOURCE_EVIDENCE.get(item["id"], {}).get("source")
                or item.get("confirmation-source-version") != EXPECTED_35_SOURCE_EVIDENCE.get(item["id"], {}).get("version")
                or item.get("confirmation-basis") != EXPECTED_35_SOURCE_EVIDENCE.get(item["id"], {}).get("basis")
                or item.get("confirmation-printed-pages") != EXPECTED_35_SOURCE_EVIDENCE.get(item["id"], {}).get("pages")
            ):
                raise ValueError(f'{item["id"]}: 3.5 confirmation requires direct source-evidence provenance')
        elif item["status"] != "documented-in-3.0-only":
            raise ValueError(f'{item["id"]}: invalid status for an audited 3.0 base candidate')
    return sorted(candidates, key=lambda item: item["id"])


def render(candidates: list[dict]) -> str:
    counts = {status: sum(item["status"] == status for item in candidates) for status in sorted(STATUSES)}
    lines = [
        "---",
        'type: "Research Report"',
        'title: "Amiga GFA BASIC 3.5 candidate verification report"',
        "tags: [amiga, gfa-basic, evidence, research]",
        "---",
        "",
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
    lines.extend(["", "## Confirmed by 3.5 sources", "",
                  "| ID | Declaration family | Original source | 3.5 evidence source | 3.5 evidence pages | Evidence basis | Qualification |",
                  "|---|---|---|---|---|---|---|"])
    confirmed = [item for item in candidates if item["status"] == "confirmed-in-3.5"]
    for item in confirmed:
        note = item["notes"].replace("|", "\\|")
        evidence_pages = ", ".join(item.get("confirmation-printed-pages", item["printed-pages"]))
        evidence_source = item.get("confirmation-source", item["source"])
        basis = {"direct-example-use": "Direct example use", "direct-documentation-reference": "Direct documentation reference", "direct-documentation-restriction": "Direct documentation restriction"}.get(item.get("confirmation-basis"), "Explicit 3.5 source declaration")
        lines.append(f"| `{item['id']}` | {item['name']} | {item['source-version']} | {evidence_source} | {evidence_pages} | {basis} | {note} |")
    if not confirmed:
        lines.append("| — | None | — | — | — | — |")
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
        "- `documented-in-3.0-only` means the cited declaration is present in the 3.0 base manual but no 3.5 evidence establishes its applicability. This does not mean removed or incompatible.",
        "- `confirmed-in-3.5` can be based on an explicit 3.5 source declaration, direct use, direct documentation reference, or a specific documented restriction. For inherited functions and statements, direct evidence confirms only the form and context actually shown or referenced; it does not revalidate every overload or variant.",
        "- `unresolved` is reserved for candidates whose source scope or declaration identity still needs research.",
        "- A confirmed family does not settle every overload or edge case; qualifications in the manifest and detailed research notes remain authoritative.",
        "- Chapter 12 indexes and reference tables are navigation aids, not declarations or proof of availability.",
        "- No absence-based removal or compatibility inference is made.",
        "",
        "Source: `AM-INTERPRETER-35` and `AM-BOOK-DE-1990` in `docs/reference-sources.md`; detailed page notes are in `amiga-gfa-basic-3.0-base-inventory.md`, `amiga-gfa-basic-35-addendum-inventory.md`, `amiga-gfa-basic-35-base-function-use-audit.md`, and `amiga-gfa-basic-35-base-statement-use-audit.md`.",
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
