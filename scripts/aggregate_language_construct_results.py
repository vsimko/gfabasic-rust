#!/usr/bin/env python3
"""Aggregate per-construct native artifacts without treating missing runs as PASS."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path

import yaml

VERSIONS = ("3.041U", "3.5", "3.52")
STATUSES = {"PASS", "FAIL", "DIFFERENT", "UNTESTED"}


def _read_result_artifacts(artifact_root: Path) -> dict[tuple[str, str], dict]:
    found: dict[tuple[str, str], dict] = {}
    if not artifact_root.is_dir():
        return found
    for path in sorted(artifact_root.rglob("result.json")):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
            key = (data["construct_id"], data["version"])
        except (OSError, UnicodeDecodeError, json.JSONDecodeError, KeyError, TypeError):
            continue
        if key in found:
            raise ValueError(f"duplicate result artifact for {key[0]}/{key[1]}")
        if data.get("schema_version") != 1 or data.get("conformance_status") not in STATUSES:
            continue
        found[key] = data
    return found


def aggregate(selection: dict, root: Path, artifacts: Path, native_allowed: bool, unavailable_reason: str) -> dict:
    changed = selection.get("changed_constructs", [])
    if not isinstance(changed, list) or any(not isinstance(item, str) for item in changed):
        raise ValueError("selection.changed_constructs must be a list of construct IDs")
    selected_items = {
        (item.get("construct_id"), item.get("version")): item
        for item in selection.get("include", [])
        if isinstance(item, dict)
    }
    selected = set(selected_items)
    artifacts_by_pair = _read_result_artifacts(artifacts)
    rows = []
    for construct_id in sorted(set(changed)):
        manifest_path = root / "tests/language-constructs" / construct_id / "manifest.yaml"
        if not manifest_path.is_file():
            raise ValueError(f"selected construct manifest is missing: {construct_id}")
        manifest = yaml.safe_load(manifest_path.read_text(encoding="utf-8"))
        versions = manifest["versions"]
        for version in VERSIONS:
            pair = (construct_id, version)
            artifact = artifacts_by_pair.get(pair)
            if pair not in selected:
                row = {
                    "construct_id": construct_id, "version": version,
                    "harness_status": "not-run", "conformance_status": "UNTESTED",
                    "reason": "version was not selected by this invocation",
                }
            elif artifact is None:
                reason = unavailable_reason if not native_allowed else (
                    "native job was skipped, unavailable, or failed before uploading its result"
                )
                row = {
                    "construct_id": construct_id, "version": version,
                    "harness_status": "unavailable", "conformance_status": "UNTESTED",
                    "reason": reason,
                    "probe": selected_items[pair].get("probe"),
                }
            else:
                expected_probe = selected_items[pair].get("probe")
                identity = artifact.get("editor_identity") or {}
                status = artifact["conformance_status"]
                integrity_ok = (
                    artifact.get("probe") == expected_probe
                    and artifact.get("expected_ids") == [construct_id]
                    and identity.get("ocr_matched") is True
                    and identity.get("expected_version") == version
                )
                if status in {"PASS", "FAIL", "DIFFERENT"}:
                    integrity_ok = integrity_ok and artifact.get("harness_status") == "success"
                if not integrity_ok and status != "UNTESTED":
                    row = {
                        "construct_id": construct_id, "version": version,
                        "harness_status": artifact.get("harness_status", "unknown"),
                        "conformance_status": "UNTESTED",
                        "reason": "native result metadata, harness, or editor identity did not match the selected pair",
                        "probe": artifact.get("probe"),
                        "editor_identity": identity,
                    }
                else:
                    row = {
                        "construct_id": construct_id, "version": version,
                        "harness_status": artifact.get("harness_status", "unknown"),
                        "conformance_status": status,
                        "reason": artifact.get("reason"),
                        "probe": artifact.get("probe"),
                        "editor_identity": identity,
                    }
            rows.append(row)
    rows.sort(key=lambda row: (row["construct_id"], VERSIONS.index(row["version"])))
    if not rows:
        status = "NO_SELECTION"
    elif any(row["conformance_status"] == "FAIL" for row in rows):
        status = "FAIL"
    elif any(row["conformance_status"] == "UNTESTED" for row in rows):
        status = "INCOMPLETE"
    else:
        status = "PASS"
    return {
        "schema_version": 1,
        "selection": selection.get("selection", "unknown"),
        "changed_constructs": sorted(set(changed)),
        "native_allowed": native_allowed,
        "summary_status": status,
        "rows": rows,
    }


def persist_result_records(result: dict, selection: dict, root: Path, run_id: str, run_attempt: str, run_url: str | None = None) -> None:
    selected_items = {
        (item.get("construct_id"), item.get("version")): item
        for item in selection.get("include", []) if isinstance(item, dict)
    }
    for row in result["rows"]:
        pair = (row["construct_id"], row["version"])
        if pair not in selected_items:
            continue
        construct_dir = root.resolve() / "tests/language-constructs" / row["construct_id"]
        manifest_bytes = (construct_dir / "manifest.yaml").read_bytes()
        probe_path = selected_items[pair].get("probe")
        probe_bytes = (root.resolve() / probe_path).read_bytes() if probe_path else b""
        record = {
            "result_schema_version": 1,
            **row,
            "run_url": row.get("run_url") or run_url,
            "manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
            "expected_probe": probe_path,
            "probe_sha256": hashlib.sha256(probe_bytes).hexdigest() if probe_bytes else None,
            "selection": selection.get("selection", "unknown"),
            "run_attempt": run_attempt,
        }
        output_dir = construct_dir / "results" / row["version"]
        output_dir.mkdir(parents=True, exist_ok=True)
        payload = json.dumps(record, indent=2, sort_keys=True) + "\n"
        (output_dir / f"run-{run_id}-attempt-{run_attempt}.json").write_text(payload, encoding="utf-8")
        (output_dir / "latest.json").write_text(payload, encoding="utf-8")

def render_markdown(result: dict) -> str:
    lines = [
        "## Amiga GFA-BASIC construct matrix",
        "",
        f"**Summary:** {result['summary_status']}",
        "",
        "| Construct | Version | Harness | Conformance | Probe | Reason |",
        "|---|---|---|---|---|---|",
    ]
    for row in result["rows"]:
        reason = (row.get("reason") or "").replace("|", "\\|").replace("\n", " ")
        lines.append(
            f"| {row['construct_id']} | {row['version']} | {row['harness_status']} | "
            f"{row['conformance_status']} | {row.get('probe') or '—'} | {reason or '—'} |"
        )
    lines.extend([
        "",
        "UNTESTED means no verified native conformance result was produced; it is never a PASS.",
        "Harness execution and language-conformance outcomes are reported separately.",
        "",
    ])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--selection", required=True, type=Path)
    parser.add_argument("--artifacts", required=True, type=Path)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--native-allowed", choices=("true", "false"), default="true")
    parser.add_argument("--unavailable-reason", default="private assets or native workflow are unavailable")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--persist-results", action="store_true")
    parser.add_argument("--run-id")
    args = parser.parse_args()
    try:
        selection = json.loads(args.selection.read_text(encoding="utf-8"))
        result = aggregate(selection, args.root.resolve(), args.artifacts, args.native_allowed == "true",
                           args.unavailable_reason)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, yaml.YAMLError, KeyError, TypeError, ValueError) as exc:
        parser.exit(2, f"ERROR: cannot aggregate construct results: {exc}\n")
    if args.persist_results:
        run_id = args.run_id or os.environ.get("GITHUB_RUN_ID", "local")
        run_attempt = os.environ.get("GITHUB_RUN_ATTEMPT", "1")
        repository = os.environ.get("GITHUB_REPOSITORY", "")
        server = os.environ.get("GITHUB_SERVER_URL", "https://github.com")
        run_url = f"{server}/{repository}/actions/runs/{run_id}" if repository else None
        persist_result_records(result, selection, args.root.resolve(), run_id, run_attempt, run_url)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    markdown = render_markdown(result)
    print(markdown)
    summary_path = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary_path:
        with open(summary_path, "a", encoding="utf-8") as stream:
            stream.write(markdown)
    return 1 if result["summary_status"] == "FAIL" else 0


if __name__ == "__main__":
    raise SystemExit(main())
