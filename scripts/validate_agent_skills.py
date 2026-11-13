#!/usr/bin/env python3
"""Validate Agent Skill metadata, local links, and discovery entries."""
from __future__ import annotations

import argparse
import re
from pathlib import Path
from urllib.parse import unquote, urlsplit

LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
FRONTMATTER_RE = re.compile(r"\A---\s*\n(.*?)\n---\s*\n", re.S)


def validate_skill(path: Path, root: Path, errors: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    match = FRONTMATTER_RE.match(text)
    if not match:
        errors.append(f"{path.relative_to(root)}: missing YAML frontmatter")
        return
    fields = {}
    for line in match.group(1).splitlines():
        key, sep, value = line.partition(":")
        if sep:
            fields[key.strip()] = value.strip().strip("'\"")
    expected = path.parent.name
    if fields.get("name") != expected:
        errors.append(f"{path.relative_to(root)}: frontmatter name must be {expected!r}")
    if not fields.get("description"):
        errors.append(f"{path.relative_to(root)}: description is required")
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", expected):
        errors.append(f"{path.relative_to(root)}: directory name is not kebab-case")

    for target in LINK_RE.findall(text):
        parsed = urlsplit(target)
        if parsed.scheme or target.startswith("#"):
            continue
        linked = unquote(parsed.path)
        if not linked:
            continue
        resolved = (path.parent / linked).resolve()
        try:
            resolved.relative_to(root)
        except ValueError:
            errors.append(f"{path.relative_to(root)}: link escapes repository: {target}")
            continue
        if not resolved.exists():
            errors.append(f"{path.relative_to(root)}: broken link: {target}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    root = parser.parse_args().root.resolve()
    skills = root / ".agents" / "skills"
    errors: list[str] = []

    if not skills.is_dir():
        print("ERROR: .agents/skills directory is missing")
        return 1
    skill_files = sorted(p for p in skills.glob("*/SKILL.md") if p.is_file())
    if not skill_files:
        errors.append("no SKILL.md files found")
    for path in skill_files:
        validate_skill(path, root, errors)

    for source in (skills / "README.md", root / "AGENTS.md"):
        if not source.is_file():
            errors.append(f"{source.relative_to(root)} is missing")
            continue
        content = source.read_text(encoding="utf-8")
        for skill in skill_files:
            rel = skill.relative_to(root).as_posix()
            if rel not in content:
                errors.append(f"{source.relative_to(root)} does not discover {rel}")

    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    print(f"OK: validated {len(skill_files)} Agent Skill(s), metadata, links, and discovery entries")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
