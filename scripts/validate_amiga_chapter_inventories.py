#!/usr/bin/env python3
"""Structurally validate page-cited Amiga base-manual chapter inventories.

This is a documentation QA check, not proof that an inventory is complete
against the printed manual or that listed aliases are semantically identical.
"""
from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INVENTORIES = (
    (7, Path("docs/research/amiga-gfa-basic-chapter-7-program-handling-evidence.md"),
     "## Construct inventory", 2, 55, "Independent 3.52 compatibility evidence"),
    (8, Path("docs/research/amiga-gfa-basic-chapter-8-graphics-evidence.md"),
     "## Chapter 8 inventory", 2, 24, "## 3.52 interpreter evidence"),
    (9, Path("docs/research/amiga-gfa-basic-chapter-9-event-window-evidence.md"),
     "## Chapter 9 inventory", 2, 39, "## 3.52 runtime evidence"),
    (10, Path("docs/research/amiga-gfa-basic-chapter-10-advanced-programming-evidence.md"),
     "## Chapter 10 inventory", 2, 16, "## 3.52 runtime evidence"),
    (11, Path("docs/research/amiga-gfa-basic-chapter-11-system-routines-evidence.md"),
     "## Chapter 11 inventory", 2, 92, "## 3.52 runtime evidence"),
    (12, Path("docs/research/amiga-gfa-basic-chapter-12-appendices-index-evidence.md"),
     "## Appendix inventory", 2, 27, "## Version and runtime evidence"),
)


def table_cells(line: str) -> list[str]:
    cells: list[str] = []
    current: list[str] = []
    in_code = False
    for character in line.strip().strip("|"):
        if character == "`":
            in_code = not in_code
        if character == "|" and not in_code:
            cells.append("".join(current).strip())
            current = []
        else:
            current.append(character)
    cells.append("".join(current).strip())
    return cells


def extract_inventory(text: str, heading: str, chapter: int,
                      min_page: int, max_page: int) -> list[str]:
    errors: list[str] = []
    lines = text.splitlines()
    try:
        heading_index = lines.index(heading)
    except ValueError:
        return [f"missing inventory section {heading!r}"]

    header_index = next(
        (i for i in range(heading_index + 1, len(lines))
         if lines[i].lstrip().startswith("|")),
        None,
    )
    if header_index is None or header_index + 1 >= len(lines):
        return [f"{heading}: missing Markdown inventory table"]
    headers = [value.casefold() for value in table_cells(lines[header_index])]
    if len(headers) != 4 or not any("construct" in value or "area" in value for value in headers):
        errors.append(f"{heading}: expected a four-column construct inventory table")
    if not any("page" in value for value in headers):
        errors.append(f"{heading}: table must identify printed-page citations")

    rows = 0
    seen_labels: dict[str, int] = {}
    for line_number, line in enumerate(lines[header_index + 2:], header_index + 3):
        if not line.lstrip().startswith("|"):
            if rows:
                break
            continue
        cells = table_cells(line)
        if len(cells) != 4:
            errors.append(f"{heading}:{line_number}: expected four table cells")
            continue
        label, syntax, semantics, page_cell = cells
        if not all((label, syntax, semantics, page_cell)):
            errors.append(f"{heading}:{line_number}: inventory fields must not be empty")
            continue
        key = re.sub(r"\s+", " ", label.casefold()).strip()
        if key in seen_labels:
            errors.append(
                f"{heading}:{line_number}: duplicate inventory label {label!r} "
                f"(first at line {seen_labels[key]})"
            )
        else:
            seen_labels[key] = line_number
        page_numbers = [int(value) for value in re.findall(rf"(?<!\d){chapter}-(\d+)", page_cell)]
        if not page_numbers:
            errors.append(f"{heading}:{line_number}: no printed Chapter {chapter} page citation")
        elif any(page < min_page or page > max_page for page in page_numbers):
            errors.append(
                f"{heading}:{line_number}: page citation outside Chapter {chapter} "
                f"range {min_page}–{max_page}: {page_cell!r}"
            )
        rows += 1

    if rows == 0:
        errors.append(f"{heading}: inventory table contains no data rows")
    return errors


def validate(root: Path = ROOT) -> dict[int, int]:
    counts: dict[int, int] = {}
    for chapter, relative, heading, first, last, evidence_heading in INVENTORIES:
        path = root / relative
        text = path.read_text(encoding="utf-8")
        if "3.0" not in text or "3.5" not in text:
            raise ValueError(f"{relative}: must state the 3.0/3.5 source boundary")
        if evidence_heading not in text:
            raise ValueError(f"{relative}: missing version-labelled runtime evidence section")
        errors = extract_inventory(text, heading, chapter, first, last)
        if errors:
            raise ValueError("\n".join(f"{relative}: {item}" for item in errors))
        lines = text.splitlines()
        start = lines.index(heading)
        table_start = next(i for i in range(start + 1, len(lines)) if lines[i].lstrip().startswith("|"))
        rows = 0
        for line in lines[table_start + 2:]:
            if not line.lstrip().startswith("|"):
                break
            rows += 1
        counts[chapter] = rows
    return counts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    try:
        counts = validate(args.root.resolve())
    except (OSError, UnicodeDecodeError, ValueError) as exc:
        parser.error(str(exc))
    print("Amiga chapter inventory structure validated: " +
          ", ".join(f"Chapter {chapter}: {count} rows" for chapter, count in sorted(counts.items())))
    print("Note: structural validation does not establish manual-wide completeness or semantic alias equivalence.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
