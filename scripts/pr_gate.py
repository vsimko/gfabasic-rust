#!/usr/bin/env python3
"""Aggregate path-relevant GitHub Actions workflows into one stable PR gate."""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Any

WORKFLOWS = {
    ".github/workflows/amiga-reference-matrix.yml": (
        ".github/workflows/amiga-reference-matrix.yml",
        ".github/workflows/amiga-native-merge-run.yml",
        ".github/workflows/amiga-3041u-native-merge-run.yml",
        "tests/interpreter-probes/amiga-3.52/**",
        "tests/interpreter-probes/amiga-3.041u/**",
        "tests/interpreter-probes/amiga-3.5/**",
        "scripts/amiga-cli-probe-35.sh",
        "scripts/amiga-cli-probe-3041u.sh",
        "scripts/validate_gfatest_results.py",
    ),
    ".github/workflows/docs.yml": (
        "**/*.md", ".markdownlint-cli2.jsonc", ".lychee.toml",
        ".github/workflows/docs.yml", "api/**", "scripts/**", "site/**",
        "tests/corpus/**", "tests/language-constructs/**", "tests/tooling/**", "docs/**", "references/pdf-sources.json",
        "scripts/validate_okf.py", "scripts/generate_docs_site.py",
        "scripts/tests/test_validate_okf.py", "scripts/tests/test_generate_docs_site.py",
    ),
    ".github/workflows/rust.yml": (
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "crates/**",
        "api/**", "tests/corpus/amiga/3.5/combinatorics/**",
        "tests/corpus/amiga/3.5/matrix/**", "tests/corpus/amiga/3.5/data/**",
        ".github/workflows/rust.yml",
    ),
    ".github/workflows/fuzz.yml": (
        ".github/workflows/fuzz.yml", "Cargo.toml", "Cargo.lock",
        "crates/gfabasic-compiler/**", "crates/gfabasic-runtime/**",
        "docs/architecture/FUZZING.md", "fuzz/**", "rust-toolchain.toml",
    ),
    ".github/workflows/security-audit.yml": (
        "Cargo.toml", "Cargo.lock", "**/Cargo.toml", "**/Cargo.lock",
        "rust-toolchain.toml", "audit.toml", ".github/workflows/security-audit.yml",
    ),
    ".github/workflows/workflow-lint.yml": (
        ".github/workflows/**", ".github/skills/**",
    ),
    ".github/workflows/benchmarks.yml": (
        ".github/workflows/benchmarks.yml", ".github/workflows/security-audit.yml",
        "bench/**", "crates/gfabasic-compiler/src/source.rs",
        "docs/architecture/BENCHMARKING.md",
    ),
    ".github/workflows/release-sbom.yml": (
        "Cargo.toml", "Cargo.lock", "crates/**",
        ".github/workflows/release-sbom.yml",
    ),
}
POLL_SECONDS = 15
MAX_WAIT_SECONDS = 15 * 60


def matches_pattern(path: str, pattern: str) -> bool:
    """Match GitHub-style **/ patterns, including files at the repository root."""
    if fnmatch.fnmatchcase(path, pattern):
        return True
    return pattern.startswith("**/") and fnmatch.fnmatchcase(path, pattern[3:])


def workflows_for_path(path: str) -> set[str]:
    """Return workflows whose active path filters include this path."""
    return {
        workflow for workflow, patterns in WORKFLOWS.items()
        if any(matches_pattern(path, pattern) for pattern in patterns)
    }


def expected_workflows(paths: list[str]) -> set[str]:
    """Return relevant workflows; unknown paths conservatively select all."""
    if not paths:
        return set(WORKFLOWS)
    expected: set[str] = set()
    for path in paths:
        matched = workflows_for_path(path)
        if not matched:
            print(f"Unclassified path {path!r}; selecting every available PR workflow.")
            return set(WORKFLOWS)
        expected.update(matched)
    return expected


def expected_workflows_for_files(files: list[dict[str, Any]]) -> set[str]:
    """Classify changed files without requiring workflows for unclassified deletions."""
    paths = []
    for item in files:
        path = str(item["filename"])
        if item.get("status") == "removed" and not workflows_for_path(path):
            continue
        paths.append(path)
    return expected_workflows(paths) if paths else set()


def workflow_path(run: dict[str, Any]) -> str:
    return str(run.get("path", "")).split("@", 1)[0]


def workflow_result(path: str, runs: list[dict[str, Any]]) -> str:
    matching = [
        run for run in runs
        if workflow_path(run) == path and run.get("event") == "pull_request"
    ]
    if not matching:
        return "missing"
    latest = max(
        matching,
        key=lambda run: (
            run.get("run_number", 0),
            run.get("run_attempt", 1),
            run.get("created_at", ""),
        ),
    )
    if latest.get("status") != "completed":
        return "pending"
    return "success" if latest.get("conclusion") == "success" else str(
        latest.get("conclusion") or "failed"
    )


class GitHub:
    def __init__(self, token: str, api_url: str, repository: str):
        self.base = api_url.rstrip("/") + "/repos/" + repository
        self.token = token

    def get(self, path: str) -> dict[str, Any]:
        request = urllib.request.Request(
            self.base + path,
            headers={
                "Accept": "application/vnd.github+json",
                "Authorization": f"Bearer {self.token}",
                "X-GitHub-Api-Version": "2022-11-28",
            },
        )
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.loads(response.read().decode("utf-8"))

    def paginated(self, path: str, key: str) -> list[dict[str, Any]]:
        page = 1
        result: list[dict[str, Any]] = []
        while True:
            query = urllib.parse.urlencode({"per_page": 100, "page": page})
            separator = "&" if "?" in path else "?"
            payload = self.get(f"{path}{separator}{query}")
            items = payload if isinstance(payload, list) else payload.get(key, [])
            result.extend(items)
            if len(items) < 100:
                return result
            page += 1


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repository", required=True)
    parser.add_argument("--pull-request", required=True, type=int)
    parser.add_argument("--head-sha", required=True)
    parser.add_argument("--timeout", type=int, default=MAX_WAIT_SECONDS)
    args = parser.parse_args()

    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        print("GITHUB_TOKEN is required.", file=sys.stderr)
        return 2
    api = GitHub(
        token,
        os.environ.get("GITHUB_API_URL", "https://api.github.com"),
        args.repository,
    )
    pull = api.get(f"/pulls/{args.pull_request}")
    base_ref = str(pull["base"]["ref"])
    compare_ref = (
        f"/compare/{urllib.parse.quote(base_ref, safe='')}..."
        f"{urllib.parse.quote(args.head_sha, safe='')}"
    )
    # The PR files endpoint can include commits already merged into the base
    # when a long-lived PR's base has advanced. Compare against the current
    # base ref so inherited changes do not spuriously require unrelated runs.
    files = api.paginated(compare_ref, "files")
    paths = [item["filename"] for item in files]
    expected = expected_workflows_for_files(files)
    print("Changed paths:")
    for path in paths:
        print(f"  - {path}")
    print("Required workflow runs:")
    for workflow in sorted(expected):
        print(f"  - {workflow}")

    deadline = time.monotonic() + args.timeout
    while True:
        runs = api.paginated(
            f"/actions/runs?head_sha={urllib.parse.quote(args.head_sha)}",
            "workflow_runs",
        )
        states = {path: workflow_result(path, runs) for path in sorted(expected)}
        for path, state in states.items():
            print(f"{path}: {state}")
        failures = {
            path: state for path, state in states.items()
            if state not in {"success", "pending", "missing"}
        }
        if failures:
            print("Required workflow failure(s): " + json.dumps(failures), file=sys.stderr)
            return 1
        if all(state == "success" for state in states.values()):
            print("All path-relevant PR workflows succeeded.")
            return 0
        if time.monotonic() >= deadline:
            pending = {
                path: state for path, state in states.items() if state != "success"
            }
            print("Timed out waiting for required workflows: " + json.dumps(pending), file=sys.stderr)
            return 1
        time.sleep(POLL_SECONDS)


if __name__ == "__main__":
    raise SystemExit(main())
