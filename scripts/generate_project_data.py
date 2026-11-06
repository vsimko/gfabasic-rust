#!/usr/bin/env python3
"""Generate deterministic, evidence-checked website data from repository sources."""

import argparse
import json
import tomllib
from pathlib import Path


def load_model(path: Path, root: Path) -> dict:
    model = json.loads(path.read_text(encoding="utf-8"))
    if model.get("schema_version") != 1:
        raise ValueError("unsupported project model schema_version")
    for key in ("project", "prototype", "planned_components"):
        if key not in model:
            raise ValueError(f"project model is missing {key!r}")
    evidence = list(model["project"].get("maturity_evidence", []))
    evidence += list(model["prototype"].get("evidence", []))
    for component in model["planned_components"]:
        evidence += component.get("evidence", [])
    for item in evidence:
        target = (root / item.split("#", 1)[0]).resolve()
        try:
            target.relative_to(root.resolve())
        except ValueError as exc:
            raise ValueError(f"evidence path escapes repository: {item}") from exc
        if not target.is_file():
            raise ValueError(f"evidence source does not exist: {item}")
    return model


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
        "project": model["project"],
        "prototype": model["prototype"],
        "planned_components": model["planned_components"],
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
    except (OSError, ValueError, KeyError, tomllib.TOMLDecodeError, json.JSONDecodeError) as exc:
        parser.error(str(exc))
    print(f"Generated {output.relative_to(root)} ({len(result['workspace_packages'])} workspace packages)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
