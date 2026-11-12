#!/usr/bin/env python3
"""Validate the source-derived Amiga GFA BASIC function inventory."""

from __future__ import annotations

import argparse
import tomllib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = Path("docs/research/amiga-gfa-basic-functions.toml")

EXPECTED_35_ADDENDUM_FUNCTIONS = {"FACT", "VARIAT", "COMBIN", "GURU"}
EXPECTED_35_SOURCE_EVIDENCE = {"HEX$":["AM-INTERPRETER-35","3.5 addendum",["13-36"],"direct-example-use"],"STR$":["AM-INTERPRETER-35","3.5 addendum",["13-3"],"direct-documentation-reference"],"RAND":["AM-INTERPRETER-35","3.5 addendum",["13-2"],"direct-example-use"],"SUCC":["AM-INTERPRETER-35","3.5 addendum",["13-5"],"direct-example-use"],"STRING$":["AM-INTERPRETER-35","3.5 addendum",["13-21"],"direct-example-use"],"RND":["AM-BOOK-DE-1990","3.5 companion book Chapter 18",["458"],"direct-example-use"],"INT":["AM-BOOK-DE-1990","3.5 companion book Chapter 18",["458"],"direct-example-use"],"SQR":["AM-BOOK-DE-1990","3.5 companion book Chapter 18",["473"],"direct-example-use"]}
EXPECTED_35_SOURCE_EVIDENCE_FUNCTIONS = set(EXPECTED_35_SOURCE_EVIDENCE)

EXPECTED = {
    "all-identifiers": 117,
    "interpreter-identifiers-including-disputed-slot": 115,
    "unambiguous-interpreter-identifiers": 113,
    "compiler-only-identifiers": 2,
    "base-manual-identifiers": 113,
    "explicit-3.5-addendum-identifiers": 4,
    "disputed-name-slots": 2,
    "confirmed-by-3.5-source-evidence-identifiers": 8,
}


def validate(path: Path) -> dict[str, int]:
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    if data.get("schema-version") != 1:
        raise ValueError("unsupported function inventory schema-version")
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise ValueError("manifest must contain [[functions]] entries")

    names: set[str] = set()
    for item in functions:
        for field in ("name", "origin", "applicability", "evidence-status", "printed-pages"):
            if field not in item:
                raise ValueError(f"function entry missing {field}")
        name = item["name"]
        if not isinstance(name, str) or not name.strip():
            raise ValueError("function name must be a non-empty string")
        if name in names:
            raise ValueError(f"duplicate function identifier: {name}")
        names.add(name)
        if not item["printed-pages"] or not all(isinstance(p, str) and p for p in item["printed-pages"]):
            raise ValueError(f"{name}: printed page locators are required")
        if item["origin"] not in {"base-manual-3.0", "3.5-addendum"}:
            raise ValueError(f"{name}: invalid source origin")
        if item["applicability"] not in {"interpreter", "compiler-only"}:
            raise ValueError(f"{name}: invalid applicability")
        origin = item["origin"]
        evidence_status = item["evidence-status"]
        if origin == "base-manual-3.0" and evidence_status not in {
            "base-manual-entry",
            "disputed-name",
            "confirmed-by-3.5-source-evidence",
        }:
            raise ValueError(
                f"{name}: 3.0 base-manual evidence cannot be labelled as 3.5 addendum evidence"
            )
        if origin == "3.5-addendum" and evidence_status != "explicit-3.5-addendum":
            raise ValueError(
                f"{name}: 3.5 addendum entries require explicit-3.5-addendum evidence status"
            )

        if evidence_status == "confirmed-by-3.5-source-evidence":
            expected_evidence = EXPECTED_35_SOURCE_EVIDENCE.get(name)
            if expected_evidence is None:
                raise ValueError(f"{name}: no audited 3.5 source evidence")
            if (
                item.get("confirmation-source") != expected_evidence[0]
                or item.get("confirmation-source-version") != expected_evidence[1]
                or item.get("confirmation-printed-pages") != expected_evidence[2]
                or item.get("confirmation-basis") != expected_evidence[3]
            ):
                raise ValueError(f"{name}: 3.5 source evidence needs source and printed pages")
        elif origin == "base-manual-3.0" and name in EXPECTED_35_SOURCE_EVIDENCE_FUNCTIONS:
            raise ValueError(f"{name}: audited 3.5 addendum-use evidence status is required")

    addendum_names = {
        item["name"] for item in functions if item["origin"] == "3.5-addendum"
    }
    if addendum_names != EXPECTED_35_ADDENDUM_FUNCTIONS:
        raise ValueError(
            "3.5 addendum function set differs from the source-verified identifiers: "
            f"{sorted(addendum_names)}"
        )

    counts = Counter(item["origin"] for item in functions)
    compiler_only = sum(item["applicability"] == "compiler-only" for item in functions)
    interpreter = sum(item["applicability"] == "interpreter" for item in functions)
    disputed = sum(item["evidence-status"] == "disputed-name" for item in functions)
    result = {
        "all-identifiers": len(functions),
        "interpreter-identifiers-including-disputed-slot": interpreter,
        "unambiguous-interpreter-identifiers": interpreter - disputed,
        "compiler-only-identifiers": compiler_only,
        "base-manual-identifiers": counts["base-manual-3.0"],
        "explicit-3.5-addendum-identifiers": counts["3.5-addendum"],
        "disputed-name-slots": disputed,
        "confirmed-by-3.5-source-evidence-identifiers": sum(
            item["evidence-status"] == "confirmed-by-3.5-source-evidence" for item in functions
        ),
    }
    if result != EXPECTED:
        raise ValueError(f"inventory counts differ from expected values: {result}")
    stored_counts = data.get("expected-counts", {})
    if not isinstance(stored_counts, dict) or any(stored_counts.get(key) != value for key, value in EXPECTED.items()):
        raise ValueError(f"manifest expected-counts do not match the validated inventory: {stored_counts!r}")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    args = parser.parse_args()
    try:
        counts = validate((ROOT / args.manifest).resolve())
    except (OSError, ValueError, tomllib.TOMLDecodeError) as exc:
        parser.error(str(exc))
    print(
        f"Amiga GFA BASIC function inventory: {counts['all-identifiers']} unique names; "
        f"{counts['interpreter-identifiers-including-disputed-slot']} interpreter candidates "
        f"({counts['unambiguous-interpreter-identifiers']} unambiguous), "
        f"{counts['compiler-only-identifiers']} compiler-only."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
