#!/usr/bin/env python3
"""Write a per-construct/version result without confusing harness failure with conformance."""

from __future__ import annotations

import argparse
import json
import os
import re
from pathlib import Path

from scripts.validate_gfatest_results import validate_result_bytes

VERSION_PATTERNS = {
    "3.041U": re.compile(r"GFA[ -]?BASIC[\s\S]*3[.]041U", re.IGNORECASE),
    "3.5": re.compile(r"GFA[ -]?BASIC[\s\S]*3[.]5(?![0-9U])", re.IGNORECASE),
    "3.52": re.compile(r"GFA[ -]?BASIC[\s\S]*3[.]52", re.IGNORECASE),
}


def build_result(args: argparse.Namespace) -> dict:
    expected_path = Path(args.expected_ids)
    result_path = Path(args.results)
    screenshot = Path(args.identity_screenshot)
    ocr_path = Path(args.identity_ocr)
    expected_ids = []
    errors = []
    if expected_path.is_file():
        expected_ids = [line.strip() for line in expected_path.read_text(encoding="utf-8").splitlines()
                        if line.strip() and not line.lstrip().startswith("#")]
    else:
        errors.append("expected-ID file is missing")
    if expected_ids != [args.construct_id]:
        errors.append("expected-ID file must contain exactly the construct ID")
    if result_path.is_file():
        validation = validate_result_bytes(result_path.read_bytes(), set(expected_ids))
        errors.extend(validation.errors)
        outcomes = dict(validation.outcomes)
        passed, failed, different = validation.passed, validation.failed, validation.different
    else:
        outcomes, passed, failed, different = {}, 0, 0, 0
        errors.append("GFATEST result file is missing")
    ocr_text = ocr_path.read_text(encoding="utf-8", errors="replace") if ocr_path.is_file() else ""
    identity_ok = screenshot.is_file() and bool(VERSION_PATTERNS[args.version].search(ocr_text))
    if not screenshot.is_file():
        errors.append("running-editor version-menu screenshot is missing")
    elif not identity_ok:
        errors.append("running-editor version-menu OCR did not identify the requested version")
    harness_ok = args.harness_outcome == "success"

    if not identity_ok or errors or not outcomes:
        status = "UNTESTED"
    elif failed:
        status = "FAIL"
    elif different and harness_ok:
        status = "DIFFERENT"
    elif passed == 1 and harness_ok:
        status = "PASS"
    else:
        status = "UNTESTED"
    if not harness_ok and status == "PASS":
        status = "UNTESTED"
    reason = "; ".join(errors)
    if not harness_ok:
        reason = (reason + "; " if reason else "") + f"native harness step concluded {args.harness_outcome}"
    return {
        "schema_version": 1,
        "construct_id": args.construct_id,
        "version": args.version,
        "probe": args.probe,
        "expected_ids": expected_ids,
        "harness_status": "success" if harness_ok else args.harness_outcome,
        "conformance_status": status,
        "reason": reason or None,
        "outcomes": outcomes,
        "editor_identity": {
            "expected_version": args.version,
            "screenshot": str(screenshot),
            "ocr_file": str(ocr_path),
            "ocr_matched": identity_ok,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--construct-id", required=True)
    parser.add_argument("--version", required=True, choices=tuple(VERSION_PATTERNS))
    parser.add_argument("--probe", required=True)
    parser.add_argument("--harness-outcome", required=True, choices=("success", "failure", "cancelled", "skipped"))
    parser.add_argument("--expected-ids", required=True)
    parser.add_argument("--results", required=True)
    parser.add_argument("--identity-screenshot", required=True)
    parser.add_argument("--identity-ocr", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = build_result(args)
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    summary = f"GFATEST {args.construct_id}/{args.version}: harness={result['harness_status']}, conformance={result['conformance_status']}"
    print(summary)
    if result["reason"]:
        print(result["reason"])
    github_output = os.environ.get("GITHUB_OUTPUT")
    if github_output:
        with open(github_output, "a", encoding="utf-8") as stream:
            stream.write(f"conformance_status={result['conformance_status']}\n")
    return 1 if result["conformance_status"] == "FAIL" else 0


if __name__ == "__main__":
    raise SystemExit(main())
