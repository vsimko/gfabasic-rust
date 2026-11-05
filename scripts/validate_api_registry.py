#!/usr/bin/env python3
"""Validate the declarative GFA BASIC API registry without requiring the Rust workspace."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

if __package__:
    from .corpus_paths import has_symlink_component
    from .repository_paths import (
        has_symlink_component as has_repository_symlink_component,
        repository_relative_path,
    )
else:
    from corpus_paths import has_symlink_component
    from repository_paths import (
        has_symlink_component as has_repository_symlink_component,
        repository_relative_path,
    )

from jsonschema import Draft202012Validator


ROOT = Path(__file__).resolve().parents[1]
REGISTRY_ROOT = ROOT / "api" / "registry"
NAMESPACE_PREFIX = {
    "language-core": "gfa.language.",
    "amiga": "gfa.amiga.",
    "atari-st": "gfa.atari-st.",
}
SUPPORTED = {"supported", "partial", "adapter-backed", "emulated"}
PRIMARY_EVIDENCE = {
    "book-documented",
    "primary-reference-cross-checked",
    "runtime-observed",
}


def read_source_ids(root: Path) -> set[str]:
    text = (root / "docs" / "reference-sources.md").read_text(encoding="utf-8")
    source_ids = set()
    for line in text.splitlines():
        match = re.match(r"^\|\s*([A-Z][A-Z0-9-]+)\s*\|", line)
        if match is not None and match.group(1) != "ID":
            source_ids.add(match.group(1))
    return source_ids


def read_test_ids(root: Path, errors: list[str] | None = None) -> set[str]:
    corpus_root = root / "tests" / "corpus"
    manifest_path = corpus_root / "manifest.toml"
    if not manifest_path.exists():
        return set()
    try:
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except UnicodeDecodeError as exc:
        if errors is not None:
            errors.append(f"{manifest_path}: invalid UTF-8: {exc}")
        return set()
    except (OSError, tomllib.TOMLDecodeError):
        return set()

    fixtures = manifest.get("fixtures", [])
    if not isinstance(fixtures, list):
        return set()

    fixture_ids: set[str] = set()
    for fixture in fixtures:
        if not isinstance(fixture, str) or not fixture or "\\" in fixture:
            continue
        parts = fixture.split("/")
        if len(parts) != 4 or any(part in {"", ".", ".."} for part in parts):
            continue
        relative = Path(*parts)
        if relative.is_absolute() or relative.suffix:
            continue
        metadata_path = corpus_root / relative.with_suffix(".toml")
        try:
            if has_symlink_component(metadata_path, corpus_root):
                continue
            metadata_path.resolve().relative_to(corpus_root.resolve())
            if not metadata_path.is_file():
                continue
            metadata = tomllib.loads(metadata_path.read_text(encoding="utf-8"))
        except UnicodeDecodeError as exc:
            if errors is not None:
                errors.append(f"{metadata_path}: invalid UTF-8: {exc}")
            continue
        except (OSError, RuntimeError, ValueError, tomllib.TOMLDecodeError):
            continue
        fixture_id = metadata.get("id") if isinstance(metadata, dict) else None
        if isinstance(fixture_id, str) and fixture_id:
            fixture_ids.add(fixture_id)
    return fixture_ids


def heading_anchors(path: Path) -> set[str]:
    text = path.read_text(encoding="utf-8")
    counts: dict[str, int] = defaultdict(int)
    anchors: set[str] = set()
    for line in text.splitlines():
        match = re.match(r"^\s{0,3}#{1,6}\s+(.+?)\s*#*\s*$", line)
        if not match:
            continue
        heading = match.group(1).strip().lower()
        heading = re.sub(r"<[^>]+>", "", heading)
        heading = re.sub(r"[^\w\- ]", "", heading)
        base = re.sub(r"\s+", "-", heading).strip("-")
        count = counts[base]
        counts[base] += 1
        anchors.add(base if count == 0 else f"{base}-{count}")
    return anchors


def validate_registry(root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    registry_root = root / "api" / "registry"
    schema_path = root / "api" / "schema" / "registry-v1.json"
    if not schema_path.exists():
        return ["missing api/schema/registry-v1.json"]

    try:
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
    except UnicodeDecodeError as exc:
        return [f"{schema_path}: invalid UTF-8: {exc}"]
    except (OSError, json.JSONDecodeError) as exc:
        return [f"{schema_path}: invalid JSON: {exc}"]
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)

    try:
        source_ids = read_source_ids(root)
    except FileNotFoundError:
        source_ids = set()
        errors.append("missing docs/reference-sources.md")
    except UnicodeDecodeError as exc:
        source_ids = set()
        errors.append(f"docs/reference-sources.md: invalid UTF-8: {exc}")
    test_ids = read_test_ids(root, errors)

    files = sorted(registry_root.rglob("*.toml")) if registry_root.exists() else []
    seen_ids: dict[str, str] = {}
    seen_names: dict[tuple[str, str, str], tuple[str, str | None]] = {}

    for path in files:
        relative = path.relative_to(registry_root)
        namespace = relative.parts[0] if relative.parts else ""
        if namespace not in NAMESPACE_PREFIX:
            errors.append(f"{path}: unknown registry namespace directory {namespace!r}")
            continue
        try:
            data = tomllib.loads(path.read_text(encoding="utf-8"))
        except UnicodeDecodeError as exc:
            errors.append(f"{path}: invalid UTF-8: {exc}")
            continue
        except (OSError, tomllib.TOMLDecodeError) as exc:
            errors.append(f"{path}: invalid TOML: {exc}")
            continue

        schema_errors = sorted(validator.iter_errors(data), key=lambda item: list(item.path))
        for schema_error in schema_errors:
            location = ".".join(str(part) for part in schema_error.absolute_path) or "<root>"
            errors.append(f"{path}:{location}: {schema_error.message}")
        if schema_errors:
            # Semantic checks below rely on the schema's nested object/list
            # shapes. Report all schema violations first, then skip this file
            # rather than raising on malformed TOML values.
            continue

        for declaration in data.get("declarations", []):
            declaration_id = declaration.get("id", "")
            name = declaration.get("name", "")
            prefix = NAMESPACE_PREFIX[namespace]
            if declaration_id and not declaration_id.startswith(prefix):
                errors.append(f"{path}: {declaration_id}: ID does not match namespace {namespace!r}")
            if declaration_id in seen_ids:
                errors.append(f"{path}: duplicate ID {declaration_id} also in {seen_ids[declaration_id]}")
            elif declaration_id:
                seen_ids[declaration_id] = str(path.relative_to(root))

            for availability in declaration.get("availability", []):
                profile = availability.get("profile")
                if namespace == "language-core" and profile is not None:
                    errors.append(f"{path}: language-core declaration {declaration_id} must not select an API profile")
                if namespace in {"amiga", "atari-st"} and profile != namespace:
                    errors.append(f"{path}: declaration {declaration_id} must use profile {namespace!r}")
                dialect = availability.get("dialect", "")
                for version in availability.get("versions", []):
                    key = (name.casefold(), dialect, version)
                    group = declaration.get("overload-group")
                    if key in seen_names:
                        previous_file, previous_group = seen_names[key]
                        if not group or group != previous_group:
                            errors.append(
                                f"{path}: conflicting declaration name {name!r} for {dialect} {version}; "
                                f"also declared in {previous_file}"
                            )
                    else:
                        seen_names[key] = (str(path.relative_to(root)), group)

            evidence = declaration.get("evidence", {})
            for source_id in evidence.get("sources", []):
                if source_id not in source_ids:
                    errors.append(f"{path}: {declaration_id}: unknown source ID {source_id!r}")

            docs = declaration.get("docs", {})
            docs_path_value = docs.get("path", "")
            relative_docs_path = repository_relative_path(docs_path_value)
            if relative_docs_path is None:
                errors.append(
                    f"{path}: {declaration_id}: documentation path must be a canonical repository-relative path: {docs_path_value!r}"
                )
                anchors: set[str] = set()
            else:
                docs_path = root.joinpath(*relative_docs_path.parts)
                if has_repository_symlink_component(docs_path, root):
                    errors.append(
                        f"{path}: {declaration_id}: documentation path must not traverse symlinks"
                    )
                    anchors = set()
                elif not docs_path.is_file():
                    errors.append(f"{path}: {declaration_id}: missing documentation path {docs_path_value!r}")
                    anchors = set()
                else:
                    try:
                        anchors = heading_anchors(docs_path)
                    except UnicodeDecodeError as exc:
                        errors.append(f"{path}: {declaration_id}: documentation file has invalid UTF-8: {exc}")
                        anchors = set()
                    for anchor in docs.get("anchors", []):
                        if anchor not in anchors:
                            errors.append(f"{path}: {declaration_id}: missing documentation anchor {anchor!r}")

            implementation = declaration.get("implementation", {})
            if implementation.get("status") in {"implemented", "partial"} and not implementation.get("handler"):
                errors.append(f"{path}: {declaration_id}: implemented entry has no handler ID")

            for support in declaration.get("support", []):
                classification = support.get("classification")
                tests = support.get("tests", [])
                for test_id in tests:
                    if test_id not in test_ids:
                        errors.append(f"{path}: {declaration_id}: unknown test ID {test_id!r}")
                if classification in SUPPORTED:
                    if evidence.get("status") not in PRIMARY_EVIDENCE:
                        errors.append(f"{path}: {declaration_id}: supported entry lacks primary/observed evidence")
                    if implementation.get("status") not in {"implemented", "partial"}:
                        errors.append(f"{path}: {declaration_id}: supported entry has no implementation")
                    if support.get("conformance") != "passing" or not tests:
                        errors.append(f"{path}: {declaration_id}: supported entry needs passing conformance tests")
                    if not docs.get("anchors"):
                        errors.append(f"{path}: {declaration_id}: supported entry needs at least one documentation anchor")
                if classification in {"partial", "adapter-backed", "emulated"} and not implementation.get("handler"):
                    errors.append(f"{path}: {declaration_id}: partial/adapter/emulated support needs a handler ID")

    return errors


def main() -> int:
    errors = validate_registry()
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    count = sum(
        len(tomllib.loads(path.read_text(encoding="utf-8")).get("declarations", []))
        for path in REGISTRY_ROOT.rglob("*.toml")
    ) if REGISTRY_ROOT.exists() else 0
    files = len(list(REGISTRY_ROOT.rglob("*.toml"))) if REGISTRY_ROOT.exists() else 0
    print(f"API registry validation passed: {count} declaration(s) across {files} file(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
