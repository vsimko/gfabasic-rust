#!/usr/bin/env python3
"""Validate the strict GFATEST result-file protocol emitted by Amiga fixtures."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

FIXTURE_ID = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]*\\Z")
RECORD = re.compile(r"GFATEST:([A-Za-z0-9][A-Za-z0-9._-]*):(PASS|FAIL)\\Z")


@dataclass(frozen=True)
class Validation:
    outcomes: dict[str, str]
    errors: tuple[str, ...]

    @property
    def passed(self) -> int:
        return sum(outcome == "PASS" for outcome in self.outcomes.values())

    @property
    def failed(self) -> int:
        return sum(outcome == "FAIL" for outcome in self.outcomes.values())

    @property
    def valid(self) -> bool:
        return not self.errors


def validate_result_bytes(expected_ids: list[str], raw: bytes) -> Validation:
    """Validate complete results for exactly the requested fixture IDs."""
    errors: list[str] = []
    expected: set[str] = set()

    for fixture_id in expected_ids:
        if not FIXTURE_ID.fullmatch(fixture_id):
            errors.append(f"invalid expected fixture ID: {fixture_id!r}")
        elif fixture_id in expected:
            errors.append(f"duplicate expected fixture ID: {fixture_id}")
        expected.add(fixture_id)

    if not raw:
        errors.append("result file is empty or missing")
        return Validation({}, tuple(errors))

    try:
        content = raw.decode("ascii")
    except UnicodeDecodeError as exc:
        return Validation({}, (f"result file is not ASCII: byte {exc.start}",))

    # splitlines() accepts CRLF from Amiga text files while still exposing
    # blank records and malformed/truncated final records for strict checks.
    lines = content.splitlines()
    outcomes: dict[str, str] = {}
    for line_number, line in enumerate(lines, 1):
        match = RECORD.fullmatch(line)
        if not match:
            errors.append(f"line {line_number}: malformed record")
            continue
        fixture_id, outcome = match.groups()
        if fixture_id not in expected:
            errors.append(f"line {line_number}: unknown fixture ID: {fixture_id}")
        if fixture_id in outcomes:
            errors.append(f"line {line_number}: duplicate result for fixture: {fixture_id}")
            continue
        outcomes[fixture_id] = outcome

    for fixture_id in expected_ids:
        if fixture_id not in outcomes:
            errors.append(f"missing result for fixture: {fixture_id}")

    return Validation(outcomes, tuple(errors))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expected-ids", required=True, type=Path,
                        help="ASCII text file with one requested fixture ID per line")
    parser.add_argument("--results", required=True, type=Path,
                        help="raw Amiga GFATEST result file")
    args = parser.parse_args()

    try:
        expected_text = args.expected_ids.read_text(encoding="ascii")
        expected_ids = expected_text.splitlines()
        raw = args.results.read_bytes()
    except (OSError, UnicodeError) as exc:
        print(f"GFATEST collection invalid: {exc}", file=sys.stderr)
        return 2

    result = validate_result_bytes(expected_ids, raw)
    print(f"GFATEST: {result.passed} PASS, {result.failed} FAIL, "
          f"{len(result.errors)} protocol error(s)")
    for error in result.errors:
        print(f"ERROR: {error}", file=sys.stderr)

    if not result.valid:
        return 2
    if result.failed:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
