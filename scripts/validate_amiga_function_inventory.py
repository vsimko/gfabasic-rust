#!/usr/bin/env python3
"""Validate the source-derived Amiga GFA BASIC function inventory."""

from __future__ import annotations

import argparse
import tomllib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = Path("docs/research/amiga-gfa-basic-functions.toml")

EXPECTED = {
    "all-identifiers": 116,
    "interpreter-identifiers-including-disputed-slot": 114,
    "unambiguous-interpreter-identifiers": 113,
    "compiler-only-identifiers": 2,
    "base-manual-identifiers": 112,
    "explicit-3.5-addendum-identifiers": 4,
    "disputed-name-slots": 1,
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
    }
    if result != EXPECTED:
        raise ValueError(f"inventory counts differ from expected values: {result}")
    if data.get("expected-counts") != EXPECTED:
        raise ValueError("manifest expected-counts do not match the validated inventory")
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
