#!/usr/bin/env python3
"""Validate stored AI review results against current API registry evidence."""

from __future__ import annotations

import hashlib
import json
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

from jsonschema import Draft202012Validator

if __package__:
    from .repository_paths import has_symlink_component, repository_relative_path
    from .validate_api_registry import read_source_ids
else:
    from repository_paths import has_symlink_component, repository_relative_path
    from validate_api_registry import read_source_ids


ROOT = Path(__file__).resolve().parents[1]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_declarations(root: Path) -> dict[str, tuple[Path, dict]]:
    result: dict[str, tuple[Path, dict]] = {}
    registry_root = root / "api" / "registry"
    for path in sorted(registry_root.rglob("*.toml")):
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        for declaration in data.get("declarations", []):
            result[declaration["id"]] = (path, declaration)
    return result


def validate_reviews(root: Path = ROOT) -> tuple[list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    schema_path = root / "api" / "schema" / "agent-review-result-v1.json"
    review_root = root / "api" / "reviews"
    if not schema_path.is_file():
        return ["missing api/schema/agent-review-result-v1.json"], warnings

    try:
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        Draft202012Validator.check_schema(schema)
        validator = Draft202012Validator(schema)
        declarations = load_declarations(root)
        source_ids = read_source_ids(root)
        source_register = root / "docs" / "reference-sources.md"
        source_register_hash = sha256(source_register)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, tomllib.TOMLDecodeError, ValueError) as exc:
        return [f"could not load review validation inputs: {exc}"], warnings

    if not review_root.exists():
        return errors, warnings

    verdicts: dict[str, list[tuple[str, bool, str]]] = defaultdict(list)
    for path in sorted(review_root.rglob("*.json")):
        if has_symlink_component(path, root):
            errors.append(f"{path}: review result path must not traverse symlinks")
            continue
        try:
            path.resolve().relative_to(review_root.resolve())
            result = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, RuntimeError, ValueError, UnicodeDecodeError, json.JSONDecodeError) as exc:
            errors.append(f"{path}: cannot read review result: {exc}")
            continue

        schema_errors = sorted(validator.iter_errors(result), key=lambda item: list(item.path))
        for schema_error in schema_errors:
            location = ".".join(str(part) for part in schema_error.absolute_path) or "<root>"
            errors.append(f"{path}:{location}: {schema_error.message}")
        if schema_errors:
            continue

        declaration_id = result["declaration_id"]
        if declaration_id not in declarations:
            errors.append(f"{path}: unknown declaration ID {declaration_id!r}")
            continue

        registry_file, declaration = declarations[declaration_id]
        docs_value = declaration["docs"]["path"]
        docs_relative = repository_relative_path(docs_value)
        if docs_relative is None:
            errors.append(f"{path}: declaration has invalid documentation path {docs_value!r}")
            continue
        docs_file = root.joinpath(*docs_relative.parts)
        current_inputs = {
            "registry_sha256": sha256(registry_file),
            "source_register_sha256": source_register_hash,
            "documentation_sha256": sha256(docs_file),
        }
        for key, current_hash in current_inputs.items():
            if result["reviewed_inputs"][key] != current_hash:
                errors.append(
                    f"{path}: stale review for {declaration_id}: {key} no longer matches current input"
                )

        declared_sources = set(declaration["evidence"].get("sources", []))
        declared_tests = {
            test_id
            for support in declaration.get("support", [])
            for test_id in support.get("tests", [])
        }
        for evidence in result["evidence"]:
            kind = evidence["kind"]
            reference = evidence["reference"]
            if kind == "primary-source":
                if reference not in source_ids:
                    errors.append(f"{path}: unknown primary source ID {reference!r}")
                elif reference not in declared_sources:
                    errors.append(
                        f"{path}: primary source {reference!r} is not linked from declaration {declaration_id}"
                    )
            elif kind == "counter-source" and reference not in source_ids:
                errors.append(f"{path}: unknown counter-source ID {reference!r}")
            elif kind == "test" and reference not in declared_tests:
                errors.append(
                    f"{path}: test evidence {reference!r} is not registered for declaration {declaration_id}"
                )

        verdict = result["verdict"]
        if verdict in {"supported", "contradicted", "partially-supported"} and not result["evidence"]:
            errors.append(f"{path}: verdict {verdict!r} requires at least one evidence reference")
        if verdict in {"ambiguous", "insufficient-evidence"} and not result["human_review_required"]:
            errors.append(f"{path}: {verdict!r} verdict must require human review")
        verdicts[declaration_id].append((verdict, result["human_review_required"], str(path)))

    for declaration_id, results in sorted(verdicts.items()):
        distinct = sorted({verdict for verdict, _, _ in results})
        if len(distinct) > 1:
            locations = ", ".join(path for _, _, path in results)
            warnings.append(
                f"conflicting AI review verdicts for {declaration_id}: {', '.join(distinct)} ({locations})"
            )
            if not all(human_required for _, human_required, _ in results):
                errors.append(
                    f"{declaration_id}: conflicting AI reviews must all require human review"
                )

    return errors, warnings


def main() -> int:
    errors, warnings = validate_reviews()
    for warning in warnings:
        print(f"WARNING: {warning}")
    for error in errors:
        print(f"ERROR: {error}")
    if errors:
        return 1
    if not warnings:
        print("AI review results are valid (no stored review results yet).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
