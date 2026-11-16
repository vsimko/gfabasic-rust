#!/usr/bin/env python3
"""Run repository checks defined in JSON files; uses only the Python standard library."""
import argparse
import json
import sys
from pathlib import Path

FIELDS = {
    "file_exists": {"path"},
    "text_contains": {"path", "text"},
}
COMMON = {"id", "description", "enabled", "severity", "type", "evidence"}


def load_rules(paths):
    rules, ids = [], set()
    for source in sorted(paths):
        try:
            doc = json.loads(source.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            raise ValueError(f"{source}: {exc}") from exc
        if not isinstance(doc, dict) or set(doc) != {"version", "rules"} or doc["version"] != 1 or not isinstance(doc["rules"], list):
            raise ValueError(f"{source}: expected version 1 and a rules array")
        for index, rule in enumerate(doc["rules"]):
            where = f"{source}: rules[{index}]"
            if not isinstance(rule, dict) or rule.get("type") not in FIELDS:
                raise ValueError(f"{where}: invalid rule object or unsupported type")
            expected = COMMON | FIELDS[rule["type"]]
            if set(rule) != expected:
                raise ValueError(f"{where}: fields must be exactly {sorted(expected)}")
            if any(not isinstance(rule[k], str) or not rule[k].strip() for k in ("id", "description", "evidence", "path")):
                raise ValueError(f"{where}: id, description, evidence, and path must be non-empty strings")
            if rule["id"] in ids:
                raise ValueError(f"{where}: duplicate rule id {rule['id']}")
            ids.add(rule["id"])
            if not isinstance(rule["enabled"], bool) or rule["severity"] not in ("error", "warning"):
                raise ValueError(f"{where}: enabled must be boolean and severity error or warning")
            if rule["type"] == "text_contains" and (not isinstance(rule["text"], str) or not rule["text"]):
                raise ValueError(f"{where}: text must be a non-empty string")
            rules.append(rule)
    return rules


def check(root, rule):
    path = Path(rule["path"])
    if path.is_absolute():
        raise ValueError(f"{rule['id']}: path must be relative")
    target = (root / path).resolve()
    try:
        target.relative_to(root.resolve())
    except ValueError as exc:
        raise ValueError(f"{rule['id']}: path escapes repository root") from exc
    if not target.is_file():
        return False
    return rule["type"] != "text_contains" or rule["text"] in target.read_text(encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--rules", type=Path, action="append")
    args = parser.parse_args()
    paths = args.rules or sorted((args.root / "validation/rules").glob("*.json"))
    if not paths:
        print("ERROR validation: no rule files found", file=sys.stderr)
        return 2
    try:
        rules = load_rules(paths)
        failures = []
        for rule in rules:
            if rule["enabled"] and not check(args.root, rule):
                failures.append(f"{rule['severity'].upper()} {rule['id']}: {rule['description']} ({rule['path']})")
    except (ValueError, OSError, UnicodeError) as exc:
        print(f"ERROR validation: {exc}", file=sys.stderr)
        return 2
    for item in failures:
        print(item)
    if not failures:
        print(f"OK validation: {sum(r['enabled'] for r in rules)} rule(s) passed")
    return 1 if any(r["enabled"] and r["severity"] == "error" and not check(args.root, r) for r in rules) else 0


if __name__ == "__main__":
    raise SystemExit(main())
