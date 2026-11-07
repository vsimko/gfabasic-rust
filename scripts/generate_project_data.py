#!/usr/bin/env python3
"""Generate deterministic, evidence-checked website data from repository sources."""

import argparse
import json
import subprocess
import tomllib
from pathlib import Path

COMPONENT_STATUSES = {"implemented", "partial", "prototype", "research", "planned", "blocked"}


def validate_component(component: dict, identifiers: set[str]) -> list[str]:
    required = ("id", "name", "status", "rationale", "evidence", "related_issues")
    for key in required:
        if key not in component:
            raise ValueError(f"component is missing {key!r}")
    identifier = component["id"]
    if not isinstance(identifier, str) or not identifier.strip():
        raise ValueError("component id must be a non-empty string")
    if identifier in identifiers:
        raise ValueError(f"duplicate component id: {identifier}")
    identifiers.add(identifier)
    if component["status"] not in COMPONENT_STATUSES:
        raise ValueError(f"invalid status for component {identifier}: {component['status']}")
    if not isinstance(component["name"], str) or not component["name"].strip():
        raise ValueError(f"component {identifier} has an invalid name")
    if not isinstance(component["rationale"], str) or not component["rationale"].strip():
        raise ValueError(f"component {identifier} has no rationale")
    if not isinstance(component["evidence"], list) or not component["evidence"]:
        raise ValueError(f"component {identifier} must reference at least one evidence file")
    if not isinstance(component["related_issues"], list) or any(
        not isinstance(number, int) or isinstance(number, bool) or number < 1
        for number in component["related_issues"]
    ):
        raise ValueError(f"component {identifier} has invalid related issue numbers")
    if len(component["related_issues"]) != len(set(component["related_issues"])):
        raise ValueError(f"component {identifier} has duplicate related issue numbers")
    return component["evidence"]


def load_model(path: Path, root: Path) -> dict:
    model = json.loads(path.read_text(encoding="utf-8"))
    if model.get("schema_version") != 2:
        raise ValueError("unsupported project model schema_version")
    if "project" not in model or "components" not in model:
        raise ValueError("project model requires 'project' and 'components'")
    if not isinstance(model["components"], list) or not model["components"]:
        raise ValueError("project model components must be a non-empty array")
    if not isinstance(model["project"].get("maturity_evidence"), list):
        raise ValueError("project maturity_evidence must be an array")

    identifiers: set[str] = set()
    evidence = list(model["project"]["maturity_evidence"])
    for component in model["components"]:
        evidence.extend(validate_component(component, identifiers))
    for item in evidence:
        if not isinstance(item, str) or not item.strip():
            raise ValueError("evidence references must be non-empty strings")
        target = (root / item.split("#", 1)[0]).resolve()
        try:
            target.relative_to(root.resolve())
        except ValueError as exc:
            raise ValueError(f"evidence path escapes repository: {item}") from exc
        if not target.is_file():
            raise ValueError(f"evidence source does not exist: {item}")
    return model


def source_revision(root: Path) -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else "unknown"


def generate(root: Path, model_path: Path, output: Path) -> dict:
    model = load_model(model_path, root)
    manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    packages = []
    for member in manifest.get("workspace", {}).get("members", []):
        package_manifest = tomllib.loads((root / member / "Cargo.toml").read_text(encoding="utf-8"))
        packages.append({
            "name": package_manifest["package"]["name"],
            "path": member,
        })
    result = {
        "schema_version": model["schema_version"],
        "source_revision": source_revision(root),
        "project": model["project"],
        "components": model["components"],
        "workspace_packages": packages,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--model", type=Path, default=Path("validation/project.json"))
    parser.add_argument("--output", type=Path, default=Path("site/data/project.json"))
    args = parser.parse_args()
    root = args.root.resolve()
    model = args.model if args.model.is_absolute() else root / args.model
    output = args.output if args.output.is_absolute() else root / args.output
    try:
        result = generate(root, model, output)
    except (OSError, ValueError, KeyError, TypeError, tomllib.TOMLDecodeError, json.JSONDecodeError) as exc:
        parser.error(str(exc))
    print(f"Generated {output.relative_to(root)} ({len(result['workspace_packages'])} workspace packages)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
