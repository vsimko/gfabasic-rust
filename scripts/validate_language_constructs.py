#!/usr/bin/env python3
"""Validate construct-scoped GFA-BASIC manifests; never execute BASIC probes."""

from __future__ import annotations

import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
CATALOGUE = ROOT / "tests" / "language-constructs"
VERSIONS = {"3.041U", "3.5", "3.52"}
KINDS = {"statement", "function", "operator", "declaration", "special_form"}
CLASSIFICATIONS = {"language_core", "runtime", "graphics", "io", "system_interface"}
EVIDENCE = {"documented", "partially_documented", "unverified", "unknown"}
APPLICABILITY = {"applicable", "not_applicable", "unknown"}
EXPECTATION = {"documented", "inferred", "unknown"}
RESULTS = {"PASS", "FAIL", "DIFFERENT", "UNTESTED"}
IMPLEMENTATION = {"implemented", "partial", "planned", "not_implemented", "unknown"}
ID_RE = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")


class UniqueKeyLoader(yaml.SafeLoader):
    pass


def _mapping(loader, node, deep=False):
    result = {}
    for key_node, value_node in node.value:
        key = loader.construct_object(key_node, deep=deep)
        if not isinstance(key, (str, int, float, bool, type(None))):
            raise yaml.constructor.ConstructorError("while constructing a mapping", node.start_mark,
                "mapping key must be scalar", key_node.start_mark)
        if key in result:
            raise yaml.constructor.ConstructorError("while constructing a mapping", node.start_mark,
                f"duplicate key {key!r}", key_node.start_mark)
        result[key] = loader.construct_object(value_node, deep=deep)
    return result


UniqueKeyLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, _mapping)


def source_register_ids(root: Path) -> set[str]:
    path = root / "docs" / "reference-sources.md"
    ids = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        match = re.match(r"^\|\s*([A-Z][A-Z0-9-]+)\s*\|", line)
        if match and match.group(1) != "ID":
            ids.add(match.group(1))
    return ids


def safe_probe_path(value: object, construct_dir: Path) -> Path | None:
    if not isinstance(value, str) or not value.endswith(".LST") or "\\" in value:
        return None
    relative = Path(value)
    if relative.is_absolute() or any(part in {"", ".", ".."} for part in value.split("/")):
        return None
    target = (construct_dir / relative).resolve()
    try:
        target.relative_to(construct_dir.resolve())
    except ValueError:
        return None
    current = construct_dir
    for part in relative.parts:
        current = current / part
        if current.is_symlink():
            return None
    return target if target.is_file() else None


def validate(root: Path = ROOT) -> list[str]:
    catalogue = root / "tests" / "language-constructs"
    errors: list[str] = []
    if catalogue.is_symlink() or not catalogue.is_dir():
        return ["tests/language-constructs: catalogue directory is missing or is a symlink"]
    try:
        source_ids = source_register_ids(root)
    except (OSError, UnicodeDecodeError) as exc:
        return [f"docs/reference-sources.md: cannot read source register: {exc}"]

    seen_ids: set[str] = set()
    referenced: set[Path] = set()
    for manifest_path in sorted(catalogue.glob("*/manifest.yaml")):
        directory = manifest_path.parent
        construct_id = directory.name
        if directory.is_symlink() or manifest_path.is_symlink():
            errors.append(f"{manifest_path.relative_to(root)}: symlink paths are forbidden")
            continue
        if not ID_RE.fullmatch(construct_id):
            errors.append(f"{directory.relative_to(root)}: directory name must be lowercase kebab-case")
        try:
            data = yaml.load(manifest_path.read_text(encoding="utf-8"), Loader=UniqueKeyLoader)
        except (OSError, UnicodeDecodeError, yaml.YAMLError) as exc:
            errors.append(f"{manifest_path.relative_to(root)}: invalid YAML: {exc}")
            continue
        if not isinstance(data, dict):
            errors.append(f"{manifest_path.relative_to(root)}: manifest must be a YAML mapping")
            continue

        required = {"schema_version", "id", "name", "kind", "classification", "evidence", "versions", "implementation"}
        if set(data) != required:
            errors.append(f"{manifest_path.relative_to(root)}: fields must be exactly {sorted(required)}")
        if data.get("schema_version") != 1:
            errors.append(f"{manifest_path.relative_to(root)}: schema_version must be 1")
        if data.get("id") != construct_id or not isinstance(data.get("id"), str) or not ID_RE.fullmatch(str(data.get("id"))):
            errors.append(f"{manifest_path.relative_to(root)}: id must match its lowercase kebab-case directory")
        if construct_id in seen_ids:
            errors.append(f"{manifest_path.relative_to(root)}: duplicate construct id")
        seen_ids.add(construct_id)
        if not isinstance(data.get("name"), str) or not data["name"].strip():
            errors.append(f"{manifest_path.relative_to(root)}: name must be non-empty")
        if data.get("kind") not in KINDS:
            errors.append(f"{manifest_path.relative_to(root)}: invalid kind")
        if data.get("classification") not in CLASSIFICATIONS:
            errors.append(f"{manifest_path.relative_to(root)}: invalid classification")

        evidence = data.get("evidence")
        if not isinstance(evidence, dict) or set(evidence) != {"status", "sources"}:
            errors.append(f"{manifest_path.relative_to(root)}: evidence must contain status and sources")
        else:
            if evidence.get("status") not in EVIDENCE:
                errors.append(f"{manifest_path.relative_to(root)}: invalid evidence.status")
            sources = evidence.get("sources")
            if not isinstance(sources, list):
                errors.append(f"{manifest_path.relative_to(root)}: evidence.sources must be an array")
            else:
                for source in sources:
                    if not isinstance(source, dict) or set(source) != {"source_key", "locator", "claim", "version_scope"}:
                        errors.append(f"{manifest_path.relative_to(root)}: malformed evidence source")
                        continue
                    if source.get("source_key") not in source_ids:
                        errors.append(f"{manifest_path.relative_to(root)}: unknown source_key {source.get('source_key')!r}")
                    for field in ("locator", "claim"):
                        if not isinstance(source.get(field), str) or not source[field].strip():
                            errors.append(f"{manifest_path.relative_to(root)}: evidence source {field} must be non-empty")
                    scope = source.get("version_scope")
                    if not isinstance(scope, list) or not scope or len(scope) != len(set(scope)) or not set(scope) <= VERSIONS | {"unknown"}:
                        errors.append(f"{manifest_path.relative_to(root)}: invalid evidence version_scope")

        versions = data.get("versions")
        if not isinstance(versions, dict) or set(versions) != VERSIONS:
            errors.append(f"{manifest_path.relative_to(root)}: versions must contain exactly {sorted(VERSIONS)}")
            versions = {}
        for version in VERSIONS:
            entry = versions.get(version)
            if not isinstance(entry, dict) or set(entry) != {"applicability", "probe", "expectation", "observed"}:
                errors.append(f"{manifest_path.relative_to(root)}: malformed versions.{version}")
                continue
            if entry.get("applicability") not in APPLICABILITY:
                errors.append(f"{manifest_path.relative_to(root)}: invalid versions.{version}.applicability")
            probe = entry.get("probe")
            if probe is not None:
                target = safe_probe_path(probe, directory)
                if target is None:
                    errors.append(f"{manifest_path.relative_to(root)}: unsafe or missing probe path for {version}")
                else:
                    referenced.add(target.resolve())
            expectation = entry.get("expectation")
            if not isinstance(expectation, dict) or set(expectation) != {"status", "summary"}:
                errors.append(f"{manifest_path.relative_to(root)}: malformed versions.{version}.expectation")
            elif expectation.get("status") not in EXPECTATION or not isinstance(expectation.get("summary"), str) or not expectation["summary"].strip():
                errors.append(f"{manifest_path.relative_to(root)}: invalid versions.{version}.expectation")
            observed = entry.get("observed")
            if not isinstance(observed, dict) or set(observed) != {"result", "artifact", "run_id"}:
                errors.append(f"{manifest_path.relative_to(root)}: malformed versions.{version}.observed")
            elif observed.get("result") not in RESULTS or any(observed.get(k) is not None and not isinstance(observed.get(k), str) for k in ("artifact", "run_id")):
                errors.append(f"{manifest_path.relative_to(root)}: invalid versions.{version}.observed")

        implementation = data.get("implementation")
        if not isinstance(implementation, dict) or set(implementation) != {"status", "notes"}:
            errors.append(f"{manifest_path.relative_to(root)}: implementation must contain status and notes")
        elif implementation.get("status") not in IMPLEMENTATION or not isinstance(implementation.get("notes"), str):
            errors.append(f"{manifest_path.relative_to(root)}: invalid implementation status or notes")

    for probe in sorted(catalogue.glob("*/*.LST")):
        if probe.is_symlink():
            errors.append(f"{probe.relative_to(root)}: symlink probe is forbidden")
        elif probe.resolve() not in referenced:
            errors.append(f"{probe.relative_to(root)}: probe is not referenced by a version entry")
    if not seen_ids:
        errors.append("tests/language-constructs: no construct manifests found")
    return errors


def main() -> int:
    errors = validate()
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print("Language-construct manifest validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
