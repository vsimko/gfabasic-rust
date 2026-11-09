#!/usr/bin/env python3
"""Validate the repository's canonical docs/ bundle against OKF v0.2 and project rules."""

from __future__ import annotations

import argparse
import re
import sys
from datetime import date, datetime
from pathlib import Path
from urllib.parse import unquote, urlsplit

import yaml


ROOT = Path(__file__).resolve().parents[1]
BUNDLE = Path("docs")
RESERVED = {"index.md", "log.md"}
LINK_RE = re.compile(r"!?(?:\[[^\]]*\])\(\s*(<[^>]+>|[^)\s]+)(?:\s+[^)]*)?\)")
REFERENCE_RE = re.compile(r"^\s*\[[^\]]+\]:\s*(\S+)", re.MULTILINE)
HEADING_RE = re.compile(r"^(#{1,6})\s+(.+?)\s*#*\s*$", re.MULTILINE)
HTML_ID_RE = re.compile(r"""\b(?:id|name)\s*=\s*["']([^"']+)["']""", re.IGNORECASE)
DATE_HEADING_RE = re.compile(r"^## (\d{4}-\d{2}-\d{2})\s*$")


def _error(errors: list[str], path: Path, message: str) -> None:
    errors.append(f"{path.as_posix()}: {message}")


def _frontmatter(path: Path, errors: list[str]) -> tuple[dict | None, str]:
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as exc:
        _error(errors, path, f"cannot read UTF-8 document: {exc}")
        return None, ""
    if not text.startswith("---\n"):
        _error(errors, path, "missing YAML frontmatter opening delimiter")
        return None, text
    match = re.search(r"^---\s*$", text[4:], re.MULTILINE)
    if not match:
        _error(errors, path, "missing YAML frontmatter closing delimiter")
        return None, text
    end = 4 + match.start()
    raw = text[4:end]
    body = text[4 + match.end():].lstrip("\r\n")
    try:
        data = yaml.safe_load(raw)
    except yaml.YAMLError as exc:
        _error(errors, path, f"invalid YAML frontmatter: {exc}")
        return None, body
    if not isinstance(data, dict):
        _error(errors, path, "frontmatter must be a YAML mapping")
        return None, body
    return data, body


def _timestamp(value: object) -> bool:
    if not isinstance(value, str) or not value.strip():
        return False
    try:
        parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return False
    return parsed.tzinfo is not None and parsed.utcoffset() is not None


def _validate_metadata(path: Path, data: dict, errors: list[str]) -> None:
    if not isinstance(data.get("type"), str) or not data["type"].strip():
        _error(errors, path, "frontmatter field 'type' must be a non-empty string")
    for key in ("title", "description", "resource"):
        if key in data and (not isinstance(data[key], str) or not data[key].strip()):
            _error(errors, path, f"frontmatter field '{key}' must be a non-empty string")
        if key == "resource" and isinstance(data.get(key), str) and "://" in data[key]:
            parsed = urlsplit(data[key])
            if not parsed.scheme or not parsed.netloc:
                _error(errors, path, "frontmatter field 'resource' must be a syntactically valid URL or bundle path")
    if "tags" in data and (
        not isinstance(data["tags"], list)
        or any(not isinstance(tag, str) or not tag.strip() for tag in data["tags"])
    ):
        _error(errors, path, "frontmatter field 'tags' must be a list of non-empty strings")
    if "sources" in data:
        sources = data["sources"]
        if not isinstance(sources, list):
            _error(errors, path, "frontmatter field 'sources' must be a list")
        else:
            ids = set()
            for index, source in enumerate(sources):
                where = f"sources[{index}]"
                if not isinstance(source, dict):
                    _error(errors, path, f"{where} must be a mapping")
                    continue
                resource = source.get("resource")
                if not isinstance(resource, str) or not resource.strip():
                    _error(errors, path, f"{where}.resource must be a non-empty string")
                elif "://" in resource:
                    parsed = urlsplit(resource)
                    if not parsed.scheme or not parsed.netloc:
                        _error(errors, path, f"{where}.resource must be a syntactically valid URL or bundle path")
                if "id" in source:
                    source_id = source["id"]
                    if not isinstance(source_id, str) or not source_id.strip():
                        _error(errors, path, f"{where}.id must be a non-empty string")
                    elif source_id in ids:
                        _error(errors, path, f"duplicate source id '{source_id}'")
                    else:
                        ids.add(source_id)
                for key in ("title", "author"):
                    if key in source and (not isinstance(source[key], str) or not source[key].strip()):
                        _error(errors, path, f"{where}.{key} must be a non-empty string")
                if "usage_count" in source and (
                    isinstance(source["usage_count"], bool)
                    or not isinstance(source["usage_count"], int)
                    or source["usage_count"] < 0
                ):
                    _error(errors, path, f"{where}.usage_count must be a non-negative integer")
                if "last_modified" in source and not _timestamp(source["last_modified"]):
                    _error(errors, path, f"{where}.last_modified must be an ISO 8601 datetime with offset")
    if "generated" in data:
        generated = data["generated"]
        if not isinstance(generated, dict) or not isinstance(generated.get("by"), str) or not generated["by"].strip():
            _error(errors, path, "frontmatter field 'generated' must be a mapping with non-empty 'by'")
        elif "at" in generated and not _timestamp(generated["at"]):
            _error(errors, path, "frontmatter field 'generated.at' must be an ISO 8601 datetime with offset")
    if "verified" in data:
        events = data["verified"] if isinstance(data["verified"], list) else [data["verified"]]
        for index, event in enumerate(events):
            if not isinstance(event, dict) or not isinstance(event.get("by"), str) or not event["by"].strip():
                _error(errors, path, f"verified[{index}] must be a mapping with non-empty 'by'")
            elif not _timestamp(event.get("at")):
                _error(errors, path, f"verified[{index}].at must be an ISO 8601 datetime with offset")
    if "status" in data and (
        not isinstance(data["status"], str)
        or data["status"] not in {"draft", "stable", "deprecated"}
    ):
        _error(errors, path, "frontmatter field 'status' must be draft, stable, or deprecated")
    if "stale_after" in data and not _timestamp(data["stale_after"]):
        _error(errors, path, "frontmatter field 'stale_after' must be an ISO 8601 datetime with offset")
    if "usage_window" in data:
        window = data["usage_window"]
        if not isinstance(window, dict) or not _timestamp(window.get("from")) or not _timestamp(window.get("to")):
            _error(errors, path, "frontmatter field 'usage_window' must contain ISO 8601 'from' and 'to' datetimes")
    if data.get("type") == "Attested Computation":
        if not isinstance(data.get("runtime"), str) or not data["runtime"].strip():
            _error(errors, path, "Attested Computation requires a non-empty 'runtime'")
        for key in ("executor", "attester"):
            if key in data and (
                not isinstance(data[key], dict)
                or not isinstance(data[key].get("resource"), str)
                or not data[key]["resource"].strip()
            ):
                _error(errors, path, f"frontmatter field '{key}' must be a mapping with non-empty 'resource'")
        if "parameters" in data:
            params = data["parameters"]
            if not isinstance(params, list):
                _error(errors, path, "frontmatter field 'parameters' must be a list")
            else:
                for index, param in enumerate(params):
                    if not isinstance(param, dict) or not all(
                        isinstance(param.get(key), str) and param[key].strip()
                        for key in ("name", "type")
                    ) or not isinstance(param.get("required"), bool):
                        _error(errors, path, f"parameters[{index}] requires string name/type and boolean required")


def _heading_ids(text: str) -> set[str]:
    text = _without_code(text)
    ids = set(HTML_ID_RE.findall(text))
    used: dict[str, int] = {}
    for match in HEADING_RE.finditer(text):
        heading = re.sub(r"\s+\{#[^}]+\}\s*$", "", match.group(2))
        explicit = re.search(r"\{#([^}]+)\}\s*$", match.group(2))
        if explicit:
            slug = explicit.group(1)
        else:
            slug = re.sub(r"[^w -]", "", heading.lower(), flags=re.UNICODE)
            slug = re.sub(r"\s+", "-", slug.strip())
        count = used.get(slug, 0)
        used[slug] = count + 1
        ids.add(slug if count == 0 else f"{slug}-{count}")
    return ids


def _without_code(text: str) -> str:
    lines = []
    fence_char = None
    fence_size = 0
    for line in text.splitlines():
        marker = re.match(r"^ {0,3}([~\x60]{3,})", line)
        if fence_char is None and marker:
            fence_char, fence_size = marker.group(1)[0], len(marker.group(1))
            continue
        if fence_char is not None:
            closing = re.match(r"^ {0,3}([~\x60]+)\s*$", line)
            if closing and closing.group(1)[0] == fence_char and len(closing.group(1)) >= fence_size:
                fence_char = None
                fence_size = 0
            continue
        lines.append(line)
    return re.sub(r"[\x60]+[^\x60]*[\x60]+", "", "\n".join(lines))


def _markdown_links(text: str) -> list[str]:
    text = _without_code(text)
    links = [m.group(1).strip("<>") for m in LINK_RE.finditer(text)]
    links.extend(REFERENCE_RE.findall(text))
    return links


def _resolve_link(source: Path, target: str, root: Path, bundle: Path) -> Path | None:
    target = unquote(target.split("?", 1)[0])
    if not target or target.startswith("#"):
        return source
    parsed = urlsplit(target)
    if parsed.scheme or target.startswith("//"):
        return None
    raw = Path(target)
    if target.startswith("/"):
        resolved = bundle / target.lstrip("/")
    else:
        resolved = source.parent / raw
    if resolved.is_dir() or target.endswith("/"):
        resolved = resolved / "index.md"
    elif not resolved.suffix:
        if resolved.with_suffix(".md").is_file():
            resolved = resolved.with_suffix(".md")
    return resolved


def _validate_reserved(path: Path, body: str, data: dict | None, bundle: Path, errors: list[str]) -> None:
    relative = path.relative_to(bundle)
    if path.name == "index.md":
        if relative == Path("index.md"):
            if data != {"okf_version": "0.2"}:
                _error(errors, path, "root index frontmatter must contain only okf_version: \"0.2\"")
        elif data is not None:
            _error(errors, path, "nested index.md must not contain frontmatter")
        headings = HEADING_RE.findall(body)
        if not headings:
            _error(errors, path, "index.md must contain at least one section heading")
        if not _markdown_links(body):
            _error(errors, path, "index.md must list concepts or subdirectories with Markdown links")
    elif path.name == "log.md":
        if data is not None:
            _error(errors, path, "log.md is reserved and must not contain frontmatter")
        dates = []
        for match in DATE_HEADING_RE.finditer(body):
            try:
                dates.append(date.fromisoformat(match.group(1)))
            except ValueError:
                _error(errors, path, f"invalid ISO date heading '{match.group(1)}'")
        if not dates:
            _error(errors, path, "log.md must contain one or more ## YYYY-MM-DD headings")
        elif dates != sorted(dates, reverse=True):
            _error(errors, path, "log.md date sections must be newest first")


def validate_bundle(root: Path = ROOT) -> list[str]:
    root = root.resolve()
    bundle = root / BUNDLE
    errors: list[str] = []
    if not bundle.is_dir():
        return ["docs/: canonical documentation bundle directory is missing"]
    markdown = sorted(bundle.rglob("*.md"))
    if not markdown:
        return ["docs/: no Markdown documents found"]
    concept_paths = {path for path in markdown if path.name not in RESERVED}
    canonical_ids = {}
    for path in concept_paths:
        concept_id = path.relative_to(bundle).with_suffix("").as_posix().casefold()
        if concept_id in canonical_ids:
            _error(errors, path, f"duplicate canonical concept path (case-insensitive): {canonical_ids[concept_id]}")
        else:
            canonical_ids[concept_id] = path
    bodies: dict[Path, str] = {}
    for path in markdown:
        relative = path.relative_to(root)
        if path.name in RESERVED:
            if path.name == "index.md" and relative == BUNDLE / "index.md":
                data, body = _frontmatter(path, errors)
                _validate_reserved(path, body, data, bundle, errors)
            else:
                try:
                    text = path.read_text(encoding="utf-8")
                except (OSError, UnicodeError) as exc:
                    _error(errors, path, f"cannot read UTF-8 document: {exc}")
                    continue
                if text.startswith("---\n"):
                    data, body = _frontmatter(path, errors)
                else:
                    data, body = None, text
                _validate_reserved(path, body, data, bundle, errors)
            bodies[path] = body
            continue
        data, body = _frontmatter(path, errors)
        bodies[path] = body
        if data is not None:
            _validate_metadata(path, data, errors)

    # Check every Markdown link, including links outside docs/ that are used as references.
    for source, body in bodies.items():
        for target in _markdown_links(body):
            raw = target.split("#", 1)[0]
            anchor = target.split("#", 1)[1] if "#" in target else ""
            resolved = _resolve_link(source, raw, root, bundle)
            if resolved is None:
                continue
            if not resolved.exists():
                _error(errors, source, f"broken local link '{target}' (resolved to {resolved.relative_to(root) if resolved.is_relative_to(root) else resolved})")
                continue
            if anchor and resolved.is_file() and resolved.suffix.lower() == ".md":
                try:
                    target_text = resolved.read_text(encoding="utf-8")
                except (OSError, UnicodeError):
                    continue
                if anchor not in _heading_ids(target_text):
                    _error(errors, source, f"broken local anchor '{target}'")

    # Every directory index must enumerate its direct concepts and child directories.
    for index in (p for p in markdown if p.name == "index.md"):
        body = bodies.get(index, "")
        listed = set()
        for link in _markdown_links(body):
            resolved = _resolve_link(index, link.split("#", 1)[0], root, bundle)
            if resolved is not None:
                listed.add(resolved.resolve())
        directory = index.parent
        required = {
            p.resolve() for p in directory.glob("*.md")
            if p.name not in RESERVED
        }
        required.update(
            child.resolve() / "index.md"
            for child in directory.iterdir()
            if child.is_dir() and any(child.rglob("*.md"))
        )
        missing = sorted(required - listed, key=str)
        for path in missing:
            _error(errors, index, f"index is missing direct entry '{path.relative_to(root)}'")

    # Every concept must be reachable from the bundle root index.
    root_index = bundle / "index.md"
    reachable: set[Path] = set()
    pending = [root_index]
    while pending:
        current = pending.pop()
        current = current.resolve()
        if current in reachable or current not in bodies:
            continue
        reachable.add(current)
        for link in _markdown_links(bodies[current]):
            resolved = _resolve_link(current, link.split("#", 1)[0], root, bundle)
            if resolved is not None and resolved.resolve() in bodies:
                pending.append(resolved.resolve())
    for concept in sorted(concept_paths):
        if concept.resolve() not in reachable:
            _error(errors, concept, "orphaned concept is not reachable from docs/index.md")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT, help="repository root")
    args = parser.parse_args()
    errors = validate_bundle(args.root)
    if errors:
        for error in errors:
            print(f"ERROR {error}", file=sys.stderr)
        return 1
    count = sum(1 for _ in (args.root / BUNDLE).rglob("*.md"))
    print(f"OK OKF v0.2: validated {count} Markdown files in docs/")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
