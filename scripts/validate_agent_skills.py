#!/usr/bin/env python3
"""Validate Agent Skill metadata, discovery links, and declared source dependencies."""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
SKILLS = ROOT / ".agents" / "skills"
INDEX = SKILLS / "README.md"
AGENTS = ROOT / "AGENTS.md"
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
FRONTMATTER_RE = re.compile(r"\A---\s*\n(.*?)\n---\s*\n", re.S)


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def validate_skill(path: Path, errors: list[str]) -> None:
    text = path.read_text(encoding="utf-8")
    match = FRONTMATTER_RE.match(text)
    if not match:
        fail(errors, f"{path.relative_to(ROOT)}: missing YAML frontmatter")
        return
    fields = {}
    for line in match.group(1).splitlines():
        key, sep, value = line.partition(":")
        if sep:
            fields[key.strip()] = value.strip().strip("\\"'")
    expected = path.parent.name
    if fields.get("name") != expected:
        fail(errors, f"{path.relative_to(ROOT)}: frontmatter name must be {expected!r}")
    if not fields.get("description"):
        fail(errors, f"{path.relative_to(ROOT)}: description is required")
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", expected):
        fail(errors, f"{path.relative_to(ROOT)}: directory name is not kebab-case")

    for target in LINK_RE.findall(text):
        parsed = urlsplit(target)
        if parsed.scheme or target.startswith(("#", "mailto:")):
            continue
        linked = unquote(parsed.path)
        if not linked:
            continue
        resolved = (path.parent / linked).resolve()
        try:
            resolved.relative_to(ROOT)
        except ValueError:
            fail(errors, f"{path.relative_to(ROOT)}: link escapes repository: {target}")
            continue
        if not resolved.exists():
            fail(errors, f"{path.relative_to(ROOT)}: broken link: {target}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    root = args.root.resolve()
    global ROOT, SKILLS, INDEX, AGENTS
    ROOT = root
    SKILLS = root / ".agents" / "skills"
    INDEX = SKILLS / "README.md"
    AGENTS = root / "AGENTS.md"
    errors: list[str] = []

    if not SKILLS.is_dir():
        print("ERROR: .agents/skills directory is missing")
        return 1
    skill_files = sorted(p for p in SKILLS.glob("*/SKILL.md") if p.is_file())
    if not skill_files:
        fail(errors, "no SKILL.md files found")
    for path in skill_files:
        validate_skill(path, errors)

    if not INDEX.is_file():
        fail(errors, ".agents/skills/README.md is missing")
    if not AGENTS.is_file():
        fail(errors, "AGENTS.md is missing")
    for source in (INDEX, AGENTS):
        if source.is_file():
            content = source.read_text(encoding="utf-8")
            for skill in skill_files:
                rel = skill.relative_to(root).as_posix()
                if rel not in content:
                    fail(errors, f"{source.relative_to(root)} does not discover {rel}")

    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    print(f"OK: validated {len(skill_files)} Agent Skill(s), metadata, links, and discovery entries")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
