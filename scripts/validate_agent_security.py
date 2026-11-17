#!/usr/bin/env python3
"""Check repository-enforceable AI-agent workflow security invariants."""
from __future__ import annotations

import argparse
from pathlib import Path
import re

import yaml


def _normalise(expression: object) -> str:
    return re.sub(r"\s+", "", expression if isinstance(expression, str) else "").lower()


def _same_repository_guard(expression: object) -> bool:
    value = _normalise(expression)
    return "github.event.pull_request.head.repo.full_name==github.repository" in value


def _main_dispatch_guard(expression: object) -> bool:
    value = _normalise(expression)
    if "github.ref=='refs/heads/main'" not in value and 'github.ref==\"refs/heads/main\"' not in value:
        return False
    if "workflow_dispatch" not in value:
        return True
    return "github.event_name!='workflow_dispatch'" in value or 'github.event_name!=\"workflow_dispatch\"' in value


def _contains_secret(value: object) -> bool:
    if isinstance(value, str):
        return "secrets." in value.lower() or value.strip().lower() == "inherit"
    if isinstance(value, dict):
        return any(_contains_secret(key) or _contains_secret(item) for key, item in value.items())
    if isinstance(value, (list, tuple)):
        return any(_contains_secret(item) for item in value)
    return False


def _has_write_permission(permissions: object) -> bool:
    if isinstance(permissions, str):
        return permissions.lower() in {"write-all"}
    if not isinstance(permissions, dict):
        return False
    return any(str(value).lower() == "write" for value in permissions.values())


def validate_document(document: object, path: str) -> list[str]:
    errors: list[str] = []
    if not isinstance(document, dict):
        return [f"{path}: workflow document must be a YAML mapping"]

    events = document.get("on", {})
    if not isinstance(events, dict):
        events = {str(item): {} for item in events} if isinstance(events, list) else {}

    forbidden = sorted({"pull_request_target", "workflow_run"} & set(events))
    for event in forbidden:
        errors.append(f"{path}: forbidden trigger {event!r}; use an explicitly reviewed trust boundary")

    workflow_permissions = document.get("permissions")
    if workflow_permissions is None:
        errors.append(f"{path}: top-level permissions must be explicit")
    elif isinstance(workflow_permissions, str) and workflow_permissions.lower() == "write-all":
        errors.append(f"{path}: top-level permissions must not use broad write-all")

    jobs = document.get("jobs", {})
    if not isinstance(jobs, dict):
        errors.append(f"{path}: jobs must be a YAML mapping")
        return errors

    has_pull_request = "pull_request" in events
    has_dispatch = "workflow_dispatch" in events
    for job_id, job in jobs.items():
        if not isinstance(job, dict):
            continue
        job_permissions = job.get("permissions", workflow_permissions)
        writes = _has_write_permission(job_permissions)
        if isinstance(job_permissions, str) and job_permissions.lower() == "write-all":
            errors.append(f"{path}: job {job_id!r} must not use broad write-all permissions")
        secrets = _contains_secret(job)
        guard = job.get("if")

        if has_pull_request and (writes or secrets) and not _same_repository_guard(guard):
            reason = "write permissions" if writes else "secret access"
            errors.append(
                f"{path}: job {job_id!r} has {reason} on pull_request without a same-repository guard"
            )

        if has_dispatch and (writes or secrets) and not _main_dispatch_guard(guard):
            reason = "write permissions" if writes else "secret access"
            errors.append(
                f"{path}: job {job_id!r} has {reason} on workflow_dispatch without a main-ref guard"
            )

    return errors


def validate_repository(root: Path) -> list[str]:
    workflows = root / ".github" / "workflows"
    if not workflows.is_dir():
        return [".github/workflows/: directory is missing"]
    paths = sorted((*workflows.glob("*.yml"), *workflows.glob("*.yaml")))
    if not paths:
        return [".github/workflows/: no workflow files found"]

    errors: list[str] = []
    for path in paths:
        relative = path.relative_to(root).as_posix()
        try:
            document = yaml.load(path.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)
        except (OSError, UnicodeError, yaml.YAMLError) as exc:
            errors.append(f"{relative}: cannot parse workflow YAML: {exc}")
            continue
        errors.extend(validate_document(document, relative))
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    errors = validate_repository(args.root.resolve())
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    count = len(list((args.root / ".github" / "workflows").glob("*.yml"))) + len(
        list((args.root / ".github" / "workflows").glob("*.yaml"))
    )
    print(f"OK: checked AI-agent trust-boundary invariants in {count} workflow(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
