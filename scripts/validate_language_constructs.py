#!/usr/bin/env python3
"""Validate version-scoped language-construct manifests and select native probe jobs."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import json
import re
from pathlib import Path, PurePosixPath
import subprocess
import sys
from typing import Iterable

import yaml


ROOT = Path(__file__).resolve().parents[1]
CATALOGUE_REL = "tests/language-constructs"
SUPPORTED_VERSIONS = ("3.041U", "3.5", "3.52")
CONSTRUCT_ID = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*")
KINDS = {"statement", "function", "operator", "declaration", "special-form", "command"}
EVIDENCE_KINDS = {
    "primary-manual",
    "repository-adr",
    "native-observation",
    "repository-code",
    "secondary-reference",
}
EVIDENCE_STATUSES = {
    "page-image-verified",
    "text-verified",
    "direct-observation",
    "repository-record",
    "secondary",
    "unknown",
    "disputed",
}
EXPECTATION_STATUSES = {"documented", "observed", "unknown", "not-applicable"}
APPLICABILITY = {"applicable", "not-applicable", "unknown"}
IMPLEMENTATION_STATUSES = {
    "not-targeted",
    "not-started",
    "syntax-only",
    "prototype",
    "production",
    "unknown",
}
ROOT_SHARED_PATHS = {
    f"{CATALOGUE_REL}/README.md",
    "scripts/validate_language_constructs.py",
    "scripts/tests/test_validate_language_constructs.py",
    "scripts/pr_gate.py",
    ".github/workflows/docs.yml",
}
TOP_LEVEL_FIELDS = {"manifest_schema_version", "construct", "evidence", "versions"}
CONSTRUCT_FIELDS = {"id", "kind", "name", "summary", "syntax_forms"}
EVIDENCE_FIELDS = {"id", "source_id", "location", "kind", "status", "claim", "url"}
VERSION_FIELDS = {"applicability", "expectation", "probe", "probe_reason", "implementation"}
EXPECTATION_FIELDS = {"status", "summary", "evidence_refs"}
IMPLEMENTATION_FIELDS = {"status", "refs"}
SYNTAX_FORM_FIELDS = {"text", "versions"}


class UniqueKeySafeLoader(yaml.SafeLoader):
    """Safe YAML loader that rejects duplicate mapping keys."""


def _construct_unique_mapping(loader, node, deep=False):
    mapping = {}
    for key_node, value_node in node.value:
        key = loader.construct_object(key_node, deep=deep)
        try:
            duplicate = key in mapping
        except TypeError as exc:
            raise yaml.constructor.ConstructorError(
                "while constructing a mapping",
                node.start_mark,
                "found an unhashable mapping key",
                key_node.start_mark,
            ) from exc
        if duplicate:
            raise yaml.constructor.ConstructorError(
                "while constructing a mapping",
                node.start_mark,
                f"found duplicate key {key!r}",
                key_node.start_mark,
            )
        mapping[key] = loader.construct_object(value_node, deep=deep)
    return mapping


UniqueKeySafeLoader.add_constructor(
    yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG,
    _construct_unique_mapping,
)


@dataclass(frozen=True)
class FileChange:
    """One Git name-status record."""

    status: str
    old_path: str | None = None
    new_path: str | None = None


def _is_non_empty_string(value: object) -> bool:
    return isinstance(value, str) and bool(value.strip())


def _relative_name(path: Path, root: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def _add_error(errors: list[str], root: Path, path: Path | str, message: str) -> None:
    if isinstance(path, Path):
        name = _relative_name(path, root)
    else:
        name = path
    errors.append(f"{name}: {message}")


def _check_mapping_fields(
    value: object,
    required: set[str],
    allowed: set[str],
    errors: list[str],
    root: Path,
    path: Path,
    label: str,
) -> bool:
    if not isinstance(value, dict):
        _add_error(errors, root, path, f"{label} must be a YAML mapping")
        return False
    for field in sorted(required - value.keys()):
        _add_error(errors, root, path, f"{label} is missing required field {field!r}")
    for field in sorted(value.keys() - allowed, key=str):
        _add_error(errors, root, path, f"{label} has unsupported field {field!r}")
    return True


def _has_symlink_component(path: Path, root: Path) -> bool:
    try:
        relative = path.relative_to(root)
    except ValueError:
        return True
    current = root
    for part in relative.parts:
        current = current / part
        if current.is_symlink():
            return True
    return False


def _safe_relative_parts(value: object) -> tuple[str, ...] | None:
    if not isinstance(value, str) or not value or "\\" in value:
        return None
    if value.startswith("/"):
        return None
    parts = value.split("/")
    if any(part in {"", ".", ".."} for part in parts):
        return None
    return tuple(parts)


def _check_file_path(
    value: object,
    *,
    base: Path,
    root: Path,
    errors: list[str],
    manifest_path: Path,
    label: str,
    suffix: str | None = None,
    must_be_under_tests: bool = False,
) -> Path | None:
    parts = _safe_relative_parts(value)
    if parts is None:
        _add_error(errors, root, manifest_path, f"{label} must be a safe relative POSIX path")
        return None
    candidate = base.joinpath(*parts)
    try:
        candidate.relative_to(root)
    except ValueError:
        _add_error(errors, root, manifest_path, f"{label} escapes the repository root")
        return None
    if must_be_under_tests:
        try:
            candidate.relative_to(root / "tests")
        except ValueError:
            _add_error(errors, root, manifest_path, f"{label} must point inside tests/")
            return None
    if suffix is not None and candidate.suffix != suffix:
        _add_error(errors, root, manifest_path, f"{label} must use the {suffix} extension")
        return None
    if _has_symlink_component(candidate, root):
        _add_error(errors, root, manifest_path, f"{label} must not traverse symlinks")
        return None
    if not candidate.is_file():
        _add_error(errors, root, manifest_path, f"{label} does not exist as a regular file: {value!r}")
        return None
    return candidate


def _validate_reference_list(
    value: object,
    known_ids: set[str],
    errors: list[str],
    root: Path,
    path: Path,
    label: str,
    *,
    required: bool,
) -> list[str]:
    if not isinstance(value, list) or any(not _is_non_empty_string(item) for item in value):
        _add_error(errors, root, path, f"{label} must be an array of non-empty strings")
        return []
    if len(value) != len(set(value)):
        _add_error(errors, root, path, f"{label} must not contain duplicates")
    if required and not value:
        _add_error(errors, root, path, f"{label} must not be empty")
    for item in value:
        if item not in known_ids:
            _add_error(errors, root, path, f"{label} references unknown evidence ID {item!r}")
    return value


def _load_manifest(path: Path, root: Path, errors: list[str]) -> dict | None:
    if _has_symlink_component(path, root):
        _add_error(errors, root, path, "manifest path must not traverse symlinks")
        return None
    try:
        raw = path.read_text(encoding="utf-8")
        manifest = yaml.load(raw, Loader=UniqueKeySafeLoader)
    except UnicodeDecodeError as exc:
        _add_error(errors, root, path, f"invalid UTF-8: {exc}")
        return None
    except (OSError, yaml.YAMLError) as exc:
        _add_error(errors, root, path, f"invalid YAML: {exc}")
        return None

    if not _check_mapping_fields(
        manifest, TOP_LEVEL_FIELDS, TOP_LEVEL_FIELDS, errors, root, path, "manifest"
    ):
        return None
    if type(manifest.get("manifest_schema_version")) is not int or manifest.get("manifest_schema_version") != 2:
        _add_error(errors, root, path, "manifest_schema_version must be integer 2")

    construct = manifest.get("construct")
    if not _check_mapping_fields(
        construct, CONSTRUCT_FIELDS, CONSTRUCT_FIELDS, errors, root, path, "construct"
    ):
        return None

    construct_id = construct.get("id")
    if not isinstance(construct_id, str) or not CONSTRUCT_ID.fullmatch(construct_id):
        _add_error(errors, root, path, "construct.id must be lowercase kebab-case ASCII")
    elif path.parent.name != construct_id:
        _add_error(
            errors,
            root,
            path,
            f"construct.id {construct_id!r} must match directory name {path.parent.name!r}",
        )

    kind = construct.get("kind")
    if not isinstance(kind, str) or kind not in KINDS:
        _add_error(errors, root, path, f"construct.kind must be one of {sorted(KINDS)}")
    for field in ("name", "summary"):
        if not _is_non_empty_string(construct.get(field)):
            _add_error(errors, root, path, f"construct.{field} must be a non-empty string")

    syntax_forms = construct.get("syntax_forms")
    syntax_versions: set[str] = set()
    if not isinstance(syntax_forms, list) or not syntax_forms:
        _add_error(errors, root, path, "construct.syntax_forms must be a non-empty array")
        syntax_forms = []
    for index, form in enumerate(syntax_forms):
        label = f"construct.syntax_forms[{index}]"
        if not _check_mapping_fields(
            form, SYNTAX_FORM_FIELDS, SYNTAX_FORM_FIELDS, errors, root, path, label
        ):
            continue
        if not _is_non_empty_string(form.get("text")):
            _add_error(errors, root, path, f"{label}.text must be a non-empty string")
        versions = form.get("versions")
        if not isinstance(versions, list) or not versions or any(
            not isinstance(version, str) for version in versions
        ):
            _add_error(errors, root, path, f"{label}.versions must be a non-empty array of exact version strings")
            continue
        if len(versions) != len(set(versions)):
            _add_error(errors, root, path, f"{label}.versions must not contain duplicates")
        for version in versions:
            if version not in SUPPORTED_VERSIONS:
                _add_error(errors, root, path, f"{label} contains unknown version {version!r}")
            syntax_versions.add(version)

    evidence = manifest.get("evidence")
    evidence_ids: set[str] = set()
    if not isinstance(evidence, list):
        _add_error(errors, root, path, "evidence must be an array")
        evidence = []
    for index, item in enumerate(evidence):
        label = f"evidence[{index}]"
        if not _check_mapping_fields(
            item, EVIDENCE_FIELDS - {"url"}, EVIDENCE_FIELDS, errors, root, path, label
        ):
            continue
        evidence_id = item.get("id")
        if not isinstance(evidence_id, str) or not CONSTRUCT_ID.fullmatch(evidence_id):
            _add_error(errors, root, path, f"{label}.id must be lowercase kebab-case ASCII")
        elif evidence_id in evidence_ids:
            _add_error(errors, root, path, f"duplicate evidence ID {evidence_id!r}")
        else:
            evidence_ids.add(evidence_id)
        for field in ("source_id", "location", "claim"):
            if not _is_non_empty_string(item.get(field)):
                _add_error(errors, root, path, f"{label}.{field} must be a non-empty string")
        if not isinstance(item.get("kind"), str) or item.get("kind") not in EVIDENCE_KINDS:
            _add_error(errors, root, path, f"{label}.kind must be one of {sorted(EVIDENCE_KINDS)}")
        if not isinstance(item.get("status"), str) or item.get("status") not in EVIDENCE_STATUSES:
            _add_error(errors, root, path, f"{label}.status must be one of {sorted(EVIDENCE_STATUSES)}")
        if "url" in item and not _is_non_empty_string(item.get("url")):
            _add_error(errors, root, path, f"{label}.url must be a non-empty string when present")

    versions = manifest.get("versions")
    if not isinstance(versions, dict):
        _add_error(errors, root, path, "versions must be a YAML mapping")
        versions = {}
    actual_versions = set(versions.keys())
    expected_versions = set(SUPPORTED_VERSIONS)
    for version in sorted(expected_versions - actual_versions):
        _add_error(errors, root, path, f"versions is missing exact version key {version!r}")
    for version in sorted(actual_versions - expected_versions, key=str):
        _add_error(errors, root, path, f"versions contains unknown version key {version!r}")

    referenced_probes: set[Path] = set()
    probe_by_version: dict[str, str] = {}
    probe_reason_by_version: dict[str, str | None] = {}
    for version in SUPPORTED_VERSIONS:
        if version not in versions:
            continue
        entry = versions[version]
        label = f"versions[{version!r}]"
        if not _check_mapping_fields(
            entry, VERSION_FIELDS - {"probe_reason"}, VERSION_FIELDS, errors, root, path, label
        ):
            continue

        applies = entry.get("applicability")
        if not isinstance(applies, str) or applies not in APPLICABILITY:
            _add_error(errors, root, path, f"{label}.applicability must be one of {sorted(APPLICABILITY)}")
            applies = "unknown"

        expectation = entry.get("expectation")
        if _check_mapping_fields(
            expectation, EXPECTATION_FIELDS, EXPECTATION_FIELDS, errors, root, path, f"{label}.expectation"
        ):
            expectation_status = expectation.get("status")
            if not isinstance(expectation_status, str) or expectation_status not in EXPECTATION_STATUSES:
                _add_error(
                    errors, root, path,
                    f"{label}.expectation.status must be one of {sorted(EXPECTATION_STATUSES)}",
                )
            if not _is_non_empty_string(expectation.get("summary")):
                _add_error(errors, root, path, f"{label}.expectation.summary must be a non-empty string")
            refs = _validate_reference_list(
                expectation.get("evidence_refs"),
                evidence_ids,
                errors,
                root,
                path,
                f"{label}.expectation.evidence_refs",
                required=isinstance(expectation_status, str) and expectation_status in {"documented", "observed"},
            )
            if isinstance(expectation_status, str) and expectation_status in {"unknown", "not-applicable"} and refs:
                _add_error(
                    errors, root, path,
                    f"{label}.expectation.evidence_refs must be empty for {expectation_status!r}",
                )
            if applies == "not-applicable" and expectation_status != "not-applicable":
                _add_error(errors, root, path, f"{label} not-applicable version must have a not-applicable expectation")
            if applies == "unknown" and expectation_status != "unknown":
                _add_error(errors, root, path, f"{label} unknown applicability must have an unknown expectation")
            if applies == "applicable" and expectation_status == "not-applicable":
                _add_error(errors, root, path, f"{label} applicable version cannot have a not-applicable expectation")

        probe = entry.get("probe")
        probe_reason = entry.get("probe_reason")
        if probe_reason is not None and not _is_non_empty_string(probe_reason):
            _add_error(errors, root, path, f"{label}.probe_reason must be a non-empty string when present")
        probe_reason_by_version[version] = probe_reason
        if isinstance(probe, str):
            probe_by_version[version] = probe
        if probe is None:
            _add_error(errors, root, path, f"{label}.probe is required for every declared version")
        else:
            probe_path = _check_file_path(
                probe, base=path.parent, root=root, errors=errors,
                manifest_path=path, label=f"{label}.probe", suffix=".LST",
            )
            if probe_path is not None:
                referenced_probes.add(probe_path)

        implementation = entry.get("implementation")
        if _check_mapping_fields(
            implementation,
            IMPLEMENTATION_FIELDS,
            IMPLEMENTATION_FIELDS,
            errors,
            root,
            path,
            f"{label}.implementation",
        ):
            implementation_status = implementation.get("status")
            if not isinstance(implementation_status, str) or implementation_status not in IMPLEMENTATION_STATUSES:
                _add_error(
                    errors, root, path,
                    f"{label}.implementation.status must be one of {sorted(IMPLEMENTATION_STATUSES)}",
                )
            refs = implementation.get("refs")
            if not isinstance(refs, list) or any(not _is_non_empty_string(ref) for ref in refs):
                _add_error(errors, root, path, f"{label}.implementation.refs must be an array of non-empty repository paths")
                refs = []
            if len(refs) != len(set(refs)):
                _add_error(errors, root, path, f"{label}.implementation.refs must not contain duplicates")
            if isinstance(implementation_status, str) and implementation_status in {"syntax-only", "prototype", "production"} and not refs:
                _add_error(errors, root, path, f"{label}.implementation.refs must not be empty for {implementation_status!r}")
            if isinstance(implementation_status, str) and implementation_status in {"not-targeted", "not-started", "unknown"} and refs:
                _add_error(errors, root, path, f"{label}.implementation.refs must be empty for {implementation_status!r}")
            for ref in refs:
                _check_file_path(
                    ref,
                    base=root,
                    root=root,
                    errors=errors,
                    manifest_path=path,
                    label=f"{label}.implementation.refs entry",
                )

    if len(set(probe_by_version.values())) > 1:
        probe_counts = {probe: list(probe_by_version.values()).count(probe) for probe in set(probe_by_version.values())}
        max_probe_count = max(probe_counts.values())
        for version, probe in probe_by_version.items():
            if (probe_counts[probe] < max_probe_count or max_probe_count == 1) and not _is_non_empty_string(probe_reason_by_version.get(version)):
                _add_error(errors, root, path, f"versions[{version!r}].probe_reason is required when using a version-specific probe")

    construct_dir = path.parent
    for item in sorted(construct_dir.rglob("*")):
        if item.is_symlink():
            _add_error(errors, root, item, "construct directory must not contain symlinks")
            continue
        if item.is_dir():
            continue
        if item == path or item.name == "README.md":
            continue
        if item.suffix == ".LST":
            if item not in referenced_probes:
                _add_error(errors, root, item, "probe file is not referenced by any version")
        elif "results" in item.relative_to(construct_dir).parts:
            continue
        else:
            _add_error(errors, root, item, "unexpected file; only manifest.yaml, README.md, referenced .LST probes, and results/ outputs are allowed")

    return manifest


def _load_manifests(root: Path) -> tuple[dict[str, dict], list[str]]:
    root = root.resolve()
    catalogue = root / CATALOGUE_REL
    errors: list[str] = []
    manifests: dict[str, dict] = {}
    if not catalogue.is_dir():
        return {}, [f"{CATALOGUE_REL}: catalogue directory is missing"]
    if _has_symlink_component(catalogue, root):
        return {}, [f"{CATALOGUE_REL}: catalogue directory must not traverse symlinks"]

    for child in sorted(catalogue.iterdir(), key=lambda item: item.name):
        if child.is_symlink():
            _add_error(errors, root, child, "catalogue entries must not be symlinks")
            continue
        if child.is_file():
            if child.name != "README.md":
                _add_error(errors, root, child, "unexpected root-level catalogue file")
            continue
        if not child.is_dir():
            _add_error(errors, root, child, "catalogue entry must be a directory or README.md")
            continue
        manifest_path = child / "manifest.yaml"
        if not manifest_path.is_file():
            _add_error(errors, root, manifest_path, "construct directory is missing manifest.yaml")
            continue
        manifest = _load_manifest(manifest_path, root, errors)
        if manifest is None:
            continue
        construct = manifest.get("construct")
        construct_id = construct.get("id") if isinstance(construct, dict) else None
        if isinstance(construct_id, str):
            if construct_id in manifests:
                _add_error(errors, root, manifest_path, f"duplicate construct ID {construct_id!r}")
            else:
                manifests[construct_id] = manifest

    return manifests, errors


def validate_language_constructs(root: Path = ROOT) -> list[str]:
    """Validate every construct manifest and all referenced paths."""
    _, errors = _load_manifests(root)
    return sorted(errors)


def parse_name_status_z(output: str) -> list[FileChange]:
    """Parse git diff --name-status -z output, including rename/copy records."""
    fields = output.split("\0")
    changes: list[FileChange] = []
    index = 0
    while index < len(fields):
        status = fields[index]
        index += 1
        if not status:
            continue
        if status[0] in {"R", "C"}:
            if index + 1 >= len(fields):
                raise ValueError(f"incomplete {status} rename/copy record")
            old_path, new_path = fields[index], fields[index + 1]
            index += 2
            changes.append(FileChange(status=status, old_path=old_path, new_path=new_path))
        else:
            if index >= len(fields):
                raise ValueError(f"incomplete {status} file-change record")
            path = fields[index]
            index += 1
            changes.append(FileChange(status=status, new_path=path))
    return changes


def git_changes(root: Path, base: str, head: str) -> list[FileChange]:
    command = [
        "git",
        "diff",
        "--name-status",
        "-z",
        "--find-renames",
        f"{base}...{head}",
    ]
    completed = subprocess.run(
        command,
        cwd=root,
        check=False,
        capture_output=True,
    )
    if completed.returncode != 0:
        message = completed.stderr.decode("utf-8", errors="replace").strip()
        raise RuntimeError(f"git diff failed: {message}")
    return parse_name_status_z(completed.stdout.decode("utf-8", errors="strict"))


def _declared_versions(manifest: dict) -> list[str]:
    versions = manifest.get("versions", {})
    return [
        version
        for version in SUPPORTED_VERSIONS
        if isinstance(versions.get(version), dict)
        
    ]


def _matrix_entry(construct_id: str, version: str, manifest: dict) -> dict:
    entry = manifest["versions"][version]
    probe = entry["probe"]
    return {
        "construct_id": construct_id,
        "version": version,
        "probe": f"{CATALOGUE_REL}/{construct_id}/{probe}",
        "expectation_status": entry["expectation"]["status"],
        "expectation_summary": entry["expectation"]["summary"],
    }


def select_jobs(
    root: Path,
    changes: Iterable[FileChange] = (),
    *,
    run_all: bool = False,
    construct_id: str | None = None,
    versions: Iterable[str] = (),
) -> tuple[dict | None, list[str]]:
    """Validate manifests and return a deterministic native-job matrix."""
    root = root.resolve()
    manifests, errors = _load_manifests(root)
    if errors:
        return None, sorted(errors)

    requested_versions = list(versions)
    if run_all and (construct_id is not None or requested_versions):
        return None, ["--all cannot be combined with --construct or --version"]
    if requested_versions and construct_id is None:
        return None, ["--version requires --construct"]
    for version in requested_versions:
        if version not in SUPPORTED_VERSIONS:
            errors.append(f"unknown version identifier {version!r}")
    if errors:
        return None, sorted(errors)

    selected: dict[str, set[str]] = {}
    shared_change = False

    if run_all:
        for item_id, manifest in manifests.items():
            selected[item_id] = set(_declared_versions(manifest))
        selection = "all"
    elif construct_id is not None:
        manifest = manifests.get(construct_id)
        if manifest is None:
            return None, [f"unknown construct ID {construct_id!r}"]
        available = set(_declared_versions(manifest))
        wanted = set(requested_versions) if requested_versions else available
        invalid = sorted(wanted - available, key=lambda value: SUPPORTED_VERSIONS.index(value) if value in SUPPORTED_VERSIONS else 99)
        if invalid:
            return None, [
                f"construct {construct_id!r} is not applicable to requested version(s): {', '.join(invalid)}"
            ]
        selected[construct_id] = wanted
        selection = "manual"
    else:
        for change in changes:
            status = change.status[:1]
            paths: list[tuple[str, bool]] = []
            if status == "R":
                if change.old_path:
                    paths.append((change.old_path, True))
                if change.new_path:
                    paths.append((change.new_path, False))
            elif status == "C":
                if change.new_path:
                    paths.append((change.new_path, False))
            elif change.new_path:
                paths.append((change.new_path, status == "D"))

            for raw_path, deleted in paths:
                path = raw_path
                if _safe_relative_parts(path) is None:
                    errors.append(f"changed path {path!r} is not a safe relative POSIX path")
                    continue
                if path in ROOT_SHARED_PATHS:
                    shared_change = True
                    continue
                parts = path.split("/")
                if len(parts) < 3 or "/".join(parts[:2]) != CATALOGUE_REL:
                    continue
                item_id = parts[2]
                manifest = manifests.get(item_id)
                if manifest is None:
                    if deleted:
                        continue
                    return None, [f"changed path {path!r} is inside an unknown construct directory"]
                if deleted:
                    continue

                version_map = manifest["versions"]
                relative_in_construct = "/".join(parts[3:])
                if relative_in_construct == "results" or relative_in_construct.startswith("results/"):
                    continue
                if relative_in_construct == "manifest.yaml":
                    selected.setdefault(item_id, set()).update(_declared_versions(manifest))
                    continue
                matched_versions = {
                    version
                    for version in SUPPORTED_VERSIONS
                    if isinstance(version_map.get(version), dict)
                    and version_map[version].get("probe") == relative_in_construct
                    
                }
                if matched_versions:
                    selected.setdefault(item_id, set()).update(matched_versions)
                else:
                    selected.setdefault(item_id, set()).update(_declared_versions(manifest))

        if errors:
            return None, sorted(errors)
        selection = "targeted" if selected else "none"

    include = []
    for item_id in sorted(selected):
        manifest = manifests[item_id]
        for version in SUPPORTED_VERSIONS:
            if version in selected[item_id]:
                include.append(_matrix_entry(item_id, version, manifest))

    result = {
        "full_manifest_validation": True,
        "selection": selection,
        "shared_metadata_changed": shared_change,
        "changed_constructs": sorted(selected),
        "include": include,
    }
    return result, []


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    validate_parser = subparsers.add_parser("validate", help="validate all construct manifests")
    validate_parser.add_argument("--root", type=Path, default=ROOT)

    select_parser = subparsers.add_parser("select", help="validate manifests and select native jobs")
    select_parser.add_argument("--root", type=Path, default=ROOT)
    selection = select_parser.add_mutually_exclusive_group()
    selection.add_argument("--all", action="store_true", help="explicitly select all applicable construct/version pairs")
    selection.add_argument("--construct", help="explicitly select one construct")
    selection.add_argument("--validate-only", action="store_true", help="validate all manifests and intentionally select no native jobs")
    select_parser.add_argument("--version", action="append", default=[], help="limit --construct to this exact version (repeatable)")
    select_parser.add_argument("--base", help="base commit/ref for git diff")
    select_parser.add_argument("--head", help="head commit/ref for git diff")
    select_parser.add_argument("--changed-path", action="append", default=[], help="changed repository-relative path (repeatable)")
    select_parser.add_argument("--format", choices=("json", "github"), default="json")

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    root = args.root.resolve()

    if args.command == "validate":
        errors = validate_language_constructs(root)
        if errors:
            for item in errors:
                print(f"ERROR: {item}", file=sys.stderr)
            return 1
        manifests, _ = _load_manifests(root)
        print(f"Language-construct manifest validation passed ({len(manifests)} constructs).")
        return 0

    if bool(args.base) != bool(args.head):
        parser.error("--base and --head must be supplied together")
    if args.version and args.construct is None:
        parser.error("--version requires --construct")
    if args.all and args.version:
        parser.error("--version cannot be combined with --all")
    if args.changed_path and (args.base or args.head or args.all or args.construct or args.validate_only):
        parser.error("--changed-path cannot be combined with --base/--head, --all, --construct, or --validate-only")
    if (args.base or args.head) and (args.all or args.construct or args.validate_only):
        parser.error("--base/--head cannot be combined with --all, --construct, or --validate-only")
    if len(args.version) != len(set(args.version)):
        parser.error("--version values must not contain duplicates")
    if not (args.base or args.all or args.construct or args.changed_path or args.validate_only):
        parser.error("select requires --base/--head, --changed-path, --all, --construct, or --validate-only")

    try:
        if args.base:
            changes = git_changes(root, args.base, args.head)
        else:
            changes = [FileChange(status="M", new_path=path) for path in args.changed_path]
    except (OSError, UnicodeDecodeError, RuntimeError, ValueError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1

    result, errors = select_jobs(
        root,
        changes,
        run_all=args.all,
        construct_id=args.construct,
        versions=args.version,
    )
    if errors or result is None:
        for item in errors:
            print(f"ERROR: {item}", file=sys.stderr)
        return 1

    if args.format == "github":
        matrix = json.dumps({"include": result["include"]}, separators=(",", ":"), sort_keys=True)
        print(f"matrix={matrix}")
        print(f"selection={result['selection']}")
        print("full_manifest_validation=true")
    else:
        print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
