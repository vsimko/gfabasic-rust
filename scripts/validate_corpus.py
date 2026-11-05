#!/usr/bin/env python3
"""Validate the declarative GFA BASIC fixture corpus without executing fixtures."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

if __package__:
    from .corpus_paths import has_symlink_component
else:
    from corpus_paths import has_symlink_component


ROOT = Path(__file__).resolve().parents[1]
CORPUS_ROOT = ROOT / "tests" / "corpus"
FIXTURE_KINDS = {"positive", "negative", "semantic"}
FIXTURE_STATUSES = {"candidate", "verified", "blocked", "retired"}
FIXTURE_ID = re.compile(r"[a-z0-9]+(?:[.-][a-z0-9]+)*\Z")
DIALECT_DIRECTORY_NAMES = {
    "amiga": "GFA BASIC Amiga",
}
REQUIRED_FIELDS = {
    "id",
    "dialect",
    "version",
    "api_profiles",
    "kind",
    "status",
    "source_key",
    "source_pages",
    "evidence_status",
    "expectation",
    "notes",
}


def source_ids(root: Path) -> set[str] | None:
    """Return source-register IDs without importing the registry validator."""
    source_register = root / "docs" / "reference-sources.md"
    if not source_register.is_file():
        return None
    source_ids: set[str] = set()
    for line in source_register.read_text(encoding="utf-8").splitlines():
        match = re.match(r"^\|\s*([A-Z][A-Z0-9-]+)\s*\|", line)
        if match is None:
            continue
        source_id = match.group(1)
        if source_id != "ID":
            source_ids.add(source_id)
    return source_ids


def error(errors: list[str], path: Path, message: str, corpus_root: Path) -> None:
    errors.append(f"{path.relative_to(corpus_root)}: {message}")


def is_non_empty_string(value: object) -> bool:
    return isinstance(value, str) and bool(value.strip())


def fixture_path(value: object) -> Path | None:
    if not isinstance(value, str) or not value:
        return None
    if "\\" in value:
        return None
    parts = value.split("/")
    if len(parts) != 4 or any(part in {"", ".", ".."} for part in parts):
        return None
    path = Path(*parts)
    if path.is_absolute() or path.suffix:
        return None
    return path


def validate_fixture(
    metadata_path: Path,
    expected_stem: str,
    source_register_ids: set[str] | None,
    errors: list[str],
    corpus_root: Path,
) -> str | None:
    try:
        metadata = tomllib.loads(metadata_path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        error(errors, metadata_path, f"invalid TOML: {exc}", corpus_root)
        return None

    if not isinstance(metadata, dict):
        error(errors, metadata_path, "metadata must be a TOML table", corpus_root)
        return None

    for field in sorted(REQUIRED_FIELDS - metadata.keys()):
        error(errors, metadata_path, f"missing required field {field!r}", corpus_root)

    fixture_id = metadata.get("id")
    if not isinstance(fixture_id, str) or not FIXTURE_ID.fullmatch(fixture_id):
        error(errors, metadata_path, "id must use lowercase ASCII letters, digits, dots, and hyphens", corpus_root)
        fixture_id = None

    for field in (
        "dialect",
        "version",
        "source_key",
        "source_pages",
        "evidence_status",
        "expectation",
    ):
        if field in metadata and not is_non_empty_string(metadata[field]):
            error(errors, metadata_path, f"{field} must be a non-empty string", corpus_root)

    path_parts = Path(expected_stem).parts
    if len(path_parts) == 4:
        dialect_directory, version_directory = path_parts[:2]
        expected_dialect = DIALECT_DIRECTORY_NAMES.get(dialect_directory)
        if expected_dialect is None:
            error(errors, metadata_path, f"unknown dialect directory {dialect_directory!r}", corpus_root)
        elif metadata.get("dialect") != expected_dialect:
            error(
                errors,
                metadata_path,
                f"dialect {metadata.get('dialect')!r} does not match directory {dialect_directory!r} (expected {expected_dialect!r})",
                corpus_root,
            )
        if metadata.get("version") != version_directory:
            error(
                errors,
                metadata_path,
                f"version {metadata.get('version')!r} does not match directory {version_directory!r}",
                corpus_root,
            )

    profiles = metadata.get("api_profiles")
    if not isinstance(profiles, list) or any(not is_non_empty_string(item) for item in profiles):
        error(errors, metadata_path, "api_profiles must be an array of non-empty strings", corpus_root)
    elif len(profiles) != len(set(profiles)):
        error(errors, metadata_path, "api_profiles must not contain duplicates", corpus_root)

    kind = metadata.get("kind")
    if not isinstance(kind, str) or kind not in FIXTURE_KINDS:
        error(errors, metadata_path, f"kind must be one of {sorted(FIXTURE_KINDS)}", corpus_root)
    status = metadata.get("status")
    if not isinstance(status, str) or status not in FIXTURE_STATUSES:
        error(errors, metadata_path, f"status must be one of {sorted(FIXTURE_STATUSES)}", corpus_root)

    notes = metadata.get("notes")
    if not isinstance(notes, list) or not notes or any(not is_non_empty_string(note) for note in notes):
        error(errors, metadata_path, "notes must be a non-empty array of non-empty strings", corpus_root)

    for field in ("expected_stdout", "expected_stderr", "expected_exit"):
        if field in metadata and not is_non_empty_string(metadata[field]):
            error(errors, metadata_path, f"{field} must be a non-empty string when present", corpus_root)

    source_key = metadata.get("source_key")
    if isinstance(source_key, str) and source_register_ids is not None and source_key not in source_register_ids:
        error(errors, metadata_path, f"unknown source_key {source_key!r}", corpus_root)

    if metadata_path.with_suffix("").relative_to(corpus_root).as_posix() != expected_stem:
        error(errors, metadata_path, "metadata path does not match its manifest fixture path", corpus_root)
    return fixture_id


def validate_corpus(root: Path = ROOT) -> list[str]:
    """Return deterministic metadata errors; never compile or execute BASIC input."""
    corpus_root = root / "tests" / "corpus"
    manifest_path = corpus_root / "manifest.toml"
    errors: list[str] = []
    if has_symlink_component(manifest_path, corpus_root):
        return ["tests/corpus/manifest.toml: path must not traverse symlinks"]
    if not manifest_path.is_file():
        return ["missing tests/corpus/manifest.toml"]

    try:
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        return [f"tests/corpus/manifest.toml: invalid TOML: {exc}"]

    if manifest.get("schema_version") != 1:
        errors.append("tests/corpus/manifest.toml: schema_version must be 1")
    if not is_non_empty_string(manifest.get("corpus_status")):
        errors.append("tests/corpus/manifest.toml: corpus_status must be a non-empty string")
    if not is_non_empty_string(manifest.get("description")):
        errors.append("tests/corpus/manifest.toml: description must be a non-empty string")

    fixtures = manifest.get("fixtures")
    if not isinstance(fixtures, list):
        return errors + ["tests/corpus/manifest.toml: fixtures must be an array"]

    seen_stems: set[str] = set()
    seen_ids: dict[str, str] = {}
    referenced_metadata: set[Path] = set()
    register_ids = source_ids(root)
    if register_ids is None:
        errors.append("missing docs/reference-sources.md")

    for fixture in fixtures:
        fixture_relative = fixture_path(fixture)
        if fixture_relative is None:
            errors.append(f"tests/corpus/manifest.toml: invalid fixture path {fixture!r}")
            continue
        stem = fixture_relative.as_posix()
        if stem in seen_stems:
            errors.append(f"tests/corpus/manifest.toml: duplicate fixture path {stem!r}")
            continue
        seen_stems.add(stem)

        source_path = corpus_root / fixture_relative.with_suffix(".bas")
        metadata_path = corpus_root / fixture_relative.with_suffix(".toml")
        if has_symlink_component(source_path, corpus_root):
            errors.append(
                f"tests/corpus/manifest.toml: {stem}: source path must not traverse symlinks"
            )
        elif not source_path.is_file():
            errors.append(f"tests/corpus/manifest.toml: {stem}: missing source file")
        if has_symlink_component(metadata_path, corpus_root):
            error(errors, metadata_path, "metadata path must not traverse symlinks", corpus_root)
            referenced_metadata.add(metadata_path)
            continue
        if not metadata_path.is_file():
            errors.append(f"tests/corpus/manifest.toml: {stem}: missing metadata file")
            continue
        referenced_metadata.add(metadata_path)
        fixture_id = validate_fixture(metadata_path, stem, register_ids, errors, corpus_root)
        if fixture_id is not None:
            if fixture_id in seen_ids:
                error(
                    errors,
                    metadata_path,
                    f"duplicate id {fixture_id!r} also in {seen_ids[fixture_id]}",
                    corpus_root,
                )
            else:
                seen_ids[fixture_id] = metadata_path.relative_to(corpus_root).as_posix()

    for metadata_path in sorted(corpus_root.rglob("*.toml")):
        if metadata_path == manifest_path:
            continue
        if metadata_path not in referenced_metadata:
            error(errors, metadata_path, "metadata file is not listed in manifest.toml", corpus_root)

    for source_path in sorted(corpus_root.rglob("*.bas")):
        metadata_path = source_path.with_suffix(".toml")
        if metadata_path not in referenced_metadata:
            error(errors, source_path, "source file is not listed in manifest.toml", corpus_root)

    return errors


def main() -> int:
    errors = validate_corpus()
    if errors:
        for item in errors:
            print(f"ERROR: {item}", file=sys.stderr)
        return 1
    print("Corpus metadata validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
