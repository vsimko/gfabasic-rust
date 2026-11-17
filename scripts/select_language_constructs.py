#!/usr/bin/env python3
"""Validate construct manifests and select exact construct/version probe jobs."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path, PurePosixPath

import yaml
from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
CATALOGUE = Path("tests/language-constructs")
SCHEMA = CATALOGUE / "manifest.schema.json"
VALIDATOR = Path("scripts/select_language_constructs.py")
VERSIONS = ("3.041U", "3.5", "3.52")
ID_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")


class UniqueKeyLoader(yaml.SafeLoader):
    pass


def _construct_mapping(loader, node, deep=False):
    result = {}
    for key_node, value_node in node.value:
        key = loader.construct_object(key_node, deep=deep)
        try:
            duplicate = key in result
        except TypeError as exc:
            raise yaml.constructor.ConstructorError(
                "while constructing a mapping", node.start_mark,
                "mapping keys must be hashable", key_node.start_mark,
            ) from exc
        if duplicate:
            raise yaml.constructor.ConstructorError(
                "while constructing a mapping", node.start_mark,
                f"duplicate key {key!r}", key_node.start_mark,
            )
        result[key] = loader.construct_object(value_node, deep=deep)
    return result


UniqueKeyLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, _construct_mapping)


def _safe_repo_file(root: Path, value: str) -> Path | None:
    if not isinstance(value, str) or not value or "\\" in value:
        return None
    rel = PurePosixPath(value)
    if rel.is_absolute() or any(part in ("", ".", "..") for part in rel.parts):
        return None
    target = root.joinpath(*rel.parts)
    current = root
    for part in rel.parts:
        current = current / part
        if current.is_symlink():
            return None
    try:
        target.resolve().relative_to(root.resolve())
    except (OSError, ValueError):
        return None
    return target if target.is_file() else None


def _load_yaml(path: Path):
    return yaml.load(path.read_text(encoding="utf-8"), Loader=UniqueKeyLoader)


def validate_catalogue(root: Path = ROOT) -> list[str]:
    root = root.resolve()
    catalogue = root / CATALOGUE
    schema_path = root / SCHEMA
    errors: list[str] = []
    if catalogue.is_symlink() or not catalogue.is_dir():
        return [f"{CATALOGUE}: catalogue directory is missing or a symlink"]
    if schema_path.is_symlink() or not schema_path.is_file():
        return [f"{SCHEMA}: schema is missing or a symlink"]
    try:
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        validator = Draft202012Validator(schema, format_checker=FormatChecker())
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as exc:
        return [f"{SCHEMA}: invalid JSON Schema: {exc}"]

    seen_ids: set[str] = set()
    referenced_probes: set[Path] = set()
    for manifest_path in sorted(catalogue.glob("*/manifest.yaml")):
        directory = manifest_path.parent
        relative = manifest_path.relative_to(root).as_posix()
        if directory.is_symlink() or manifest_path.is_symlink():
            errors.append(f"{relative}: symlink paths are forbidden")
            continue
        construct_id = directory.name
        if not ID_RE.fullmatch(construct_id):
            errors.append(f"{relative}: directory name must be lowercase kebab-case")
        try:
            data = _load_yaml(manifest_path)
        except (OSError, UnicodeDecodeError, yaml.YAMLError) as exc:
            errors.append(f"{relative}: invalid YAML: {exc}")
            continue
        if not isinstance(data, dict):
            errors.append(f"{relative}: manifest must be a YAML mapping")
            continue
        for error in sorted(validator.iter_errors(data), key=lambda item: list(map(str, item.absolute_path))):
            location = ".".join(str(part) for part in error.absolute_path) or "<root>"
            errors.append(f"{relative}:{location}: {error.message}")

        construct = data.get("construct", {})
        if not isinstance(construct, dict):
            continue
        manifest_id = construct.get("id")
        if manifest_id != construct_id:
            errors.append(f"{relative}: construct.id must match its directory name")
        if manifest_id in seen_ids:
            errors.append(f"{relative}: duplicate construct ID {manifest_id!r}")
        if isinstance(manifest_id, str):
            seen_ids.add(manifest_id)

        evidence = data.get("evidence", [])
        evidence_ids = [item.get("id") for item in evidence if isinstance(item, dict)]
        if len(evidence_ids) != len(set(evidence_ids)):
            errors.append(f"{relative}: evidence IDs must be unique")
        known_evidence = set(evidence_ids)

        versions = data.get("versions", {})
        if not isinstance(versions, dict):
            continue
        for version in VERSIONS:
            entry = versions.get(version)
            if not isinstance(entry, dict):
                continue
            expectation = entry.get("expectation", {})
            execution = entry.get("execution", {})
            if not isinstance(expectation, dict) or not isinstance(execution, dict):
                continue
            exp_refs = expectation.get("evidence_refs", [])
            run_refs = execution.get("evidence_refs", [])
            for ref in list(exp_refs) + list(run_refs):
                if ref not in known_evidence:
                    errors.append(f"{relative}: versions.{version} references unknown evidence ID {ref!r}")
            if expectation.get("status") in ("documented", "observed") and not exp_refs:
                errors.append(f"{relative}: versions.{version} expectation requires evidence_refs")
            if execution.get("status") == "UNTESTED":
                if any(execution.get(key) is not None for key in ("observed", "run_url", "probe_source")) or run_refs:
                    errors.append(f"{relative}: UNTESTED execution must not claim observed output, run URL, probe source, or evidence")
            else:
                if not execution.get("observed") or not execution.get("run_url") or not run_refs:
                    errors.append(f"{relative}: completed execution requires observed output, run_url, and evidence_refs")
            applicability = entry.get("applicability")
            probe = entry.get("probe")
            if applicability == "applicable" and not probe:
                errors.append(f"{relative}: applicable version {version} requires a probe")
            if applicability != "applicable" and probe is not None:
                errors.append(f"{relative}: non-applicable/unknown version {version} must not have a probe")
            if probe is not None:
                if not isinstance(probe, str) or Path(probe).name != probe or not probe.endswith(".LST"):
                    errors.append(f"{relative}: probe must be a basename ending in .LST")
                else:
                    probe_path = directory / probe
                    if probe_path.is_symlink() or not probe_path.is_file():
                        errors.append(f"{relative}: missing or unsafe probe {probe!r}")
                    else:
                        referenced_probes.add(probe_path.resolve())
            source = execution.get("probe_source")
            if source is not None and _safe_repo_file(root, source) is None:
                errors.append(f"{relative}: execution.probe_source is not an existing safe repository file")
            implementation = entry.get("implementation", {})
            if isinstance(implementation, dict):
                for ref in implementation.get("refs", []):
                    if _safe_repo_file(root, ref) is None:
                        errors.append(f"{relative}: implementation reference is not an existing safe repository file: {ref!r}")

    for directory in sorted(path for path in catalogue.iterdir() if path.is_dir()):
        if directory.is_symlink():
            errors.append(f"{directory.relative_to(root)}: construct directory must not be a symlink")
            continue
        for probe in sorted(directory.glob("*.LST")):
            if probe.is_symlink():
                errors.append(f"{probe.relative_to(root)}: symlink probe is forbidden")
            elif probe.resolve() not in referenced_probes:
                errors.append(f"{probe.relative_to(root)}: probe is not referenced by any applicable version")
    if not seen_ids:
        errors.append(f"{CATALOGUE}: no construct manifests found")
    return errors


def select_jobs(changed_paths: list[str], root: Path = ROOT, all_constructs: bool = False) -> dict:
    """Changed construct files select only that construct; shared tooling selects validation only."""
    root = root.resolve()
    normalized = []
    for raw in changed_paths:
        path = PurePosixPath(raw)
        if path.is_absolute() or ".." in path.parts or "\\" in raw:
            raise ValueError(f"unsafe changed path: {raw!r}")
        normalized.append(path.as_posix())

    shared_change = any(path in (
        SCHEMA.as_posix(), VALIDATOR.as_posix(), (CATALOGUE / "README.md").as_posix()
    ) for path in normalized)
    full_validation = not normalized or shared_change
    selected_ids: set[str] = set()
    if all_constructs:
        full_validation = True
        selected_ids = {
            path.parent.name for path in (root / CATALOGUE).glob("*/manifest.yaml")
        }
    elif not full_validation:
        for path in normalized:
            parts = PurePosixPath(path).parts
            if len(parts) < 3 or parts[0:2] != CATALOGUE.parts:
                continue
            construct_id = parts[2]
            manifest = root / CATALOGUE / construct_id / "manifest.yaml"
            if manifest.is_file() and not manifest.is_symlink():
                selected_ids.add(construct_id)

    jobs = []
    for construct_id in sorted(selected_ids):
        manifest_path = root / CATALOGUE / construct_id / "manifest.yaml"
        data = _load_yaml(manifest_path)
        for version in VERSIONS:
            entry = data["versions"][version]
            if entry["applicability"] == "applicable":
                probe = entry["probe"]
                if not probe:
                    raise ValueError(f"{construct_id}/{version}: applicable version has no probe")
                jobs.append({"construct": construct_id, "version": version, "probe": probe})
    return {"validate_all": full_validation, "include": jobs}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--changed-path", action="append", default=[])
    parser.add_argument("--all-constructs", action="store_true",
                        help="explicitly select every applicable construct/version pair")
    parser.add_argument("--validate-only", action="store_true",
                        help="validate all manifests and emit no native jobs")
    args = parser.parse_args()
    root = args.root.resolve()
    errors = validate_catalogue(root)
    if errors:
        print("\n".join(f"ERROR: {item}" for item in errors), file=sys.stderr)
        return 1
    try:
        selection = select_jobs(args.changed_path, root, args.all_constructs)
    except (OSError, UnicodeDecodeError, yaml.YAMLError, KeyError, TypeError, ValueError) as exc:
        print(f"ERROR: cannot select construct jobs: {exc}", file=sys.stderr)
        return 1
    if args.validate_only:
        selection = {"validate_all": True, "include": []}
    print(json.dumps(selection, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
