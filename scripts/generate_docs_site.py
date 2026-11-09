#!/usr/bin/env python3
"""Build static documentation pages from the canonical OKF v0.2 docs/ bundle."""

from __future__ import annotations

import argparse
import html
import re
import shutil
from pathlib import Path
from urllib.parse import unquote, urlsplit

import yaml
from markdown_it import MarkdownIt


ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs"
OUTPUT = ROOT / "site" / "docs"
LINK_RE = re.compile(r"\[([^\]]+)\]\((<[^>]+>|[^)\s]+)(?:\s+[^)]*)?\)")
REPO_URL = "https://github.com/vsimko/gfabasic-rust"
MARKDOWN = MarkdownIt("commonmark", {"html": False, "linkify": False, "typographer": False}).enable(
    ["table", "strikethrough"]
)


def read_document(path: Path) -> tuple[dict, str]:
    text = path.read_text(encoding="utf-8")
    if path.name == "index.md" and not text.startswith("---\n"):
        return {}, text
    if not text.startswith("---\n"):
        raise ValueError(f"{path.relative_to(ROOT)}: missing OKF frontmatter")
    match = re.search(r"^---\s*$", text[4:], re.MULTILINE)
    if not match:
        raise ValueError(f"{path.relative_to(ROOT)}: missing frontmatter closing delimiter")
    data = yaml.safe_load(text[4:4 + match.start()])
    if not isinstance(data, dict):
        raise ValueError(f"{path.relative_to(ROOT)}: frontmatter must be a mapping")
    return data, text[4 + match.end():].lstrip("\r\n")


def slugify(value: str) -> str:
    value = re.sub(r"\s+\{#[^}]+\}\s*$", "", value)
    return re.sub(r"\s+", "-", re.sub(r"[^\w -]", "", value.lower()).strip())


def rewrite_target(target: str, source: Path, output_file: Path) -> str:
    parsed = urlsplit(target)
    if parsed.scheme or target.startswith("//"):
        return target
    if not parsed.path:
        return target
    raw_path = unquote(parsed.path)
    if raw_path.startswith("/"):
        resolved = BUNDLE / raw_path.lstrip("/")
    else:
        resolved = source.parent / raw_path
    if resolved.is_dir() or raw_path.endswith("/"):
        resolved = resolved / "index.md"
    elif not resolved.suffix and resolved.with_suffix(".md").is_file():
        resolved = resolved.with_suffix(".md")
    try:
        relative = resolved.resolve().relative_to(BUNDLE.resolve())
    except ValueError:
        try:
            repo_relative = resolved.resolve().relative_to(ROOT.resolve()).as_posix()
        except ValueError:
            return target
        return f"{REPO_URL}/blob/main/{repo_relative}" + (f"#{parsed.fragment}" if parsed.fragment else "")
    if resolved.suffix.lower() == ".md":
        destination = (OUTPUT / relative).with_suffix(".html")
        path = Path(__import__("os").path.relpath(destination, output_file.parent)).as_posix()
        return path + (f"#{parsed.fragment}" if parsed.fragment else "")
    return target


def render_body(body: str, source: Path, output_file: Path) -> str:
    tokens = MARKDOWN.parse(body)
    used: dict[str, int] = {}
    for index, token in enumerate(tokens):
        if token.type == "heading_open" and index + 1 < len(tokens):
            inline = tokens[index + 1]
            if inline.type == "inline":
                slug = slugify(inline.content)
                count = used.get(slug, 0)
                used[slug] = count + 1
                token.attrSet("id", slug if count == 0 else f"{slug}-{count}")
        if token.type == "inline" and token.children:
            for child in token.children:
                if child.type == "link_open":
                    href = child.attrGet("href")
                    if href:
                        child.attrSet("href", rewrite_target(href, source, output_file))
                elif child.type == "image":
                    src = child.attrGet("src")
                    if src:
                        child.attrSet("src", rewrite_target(src, source, output_file))
    return MARKDOWN.renderer.render(tokens, MARKDOWN.options, {})


def title_for(path: Path, data: dict, body: str) -> str:
    if isinstance(data.get("title"), str) and data["title"].strip():
        return data["title"].strip()
    heading = re.search(r"^#\s+(.+)$", body, re.MULTILINE)
    if heading:
        return heading.group(1).strip()
    return path.parent.name.replace("-", " ").title() or "Documentation"


def page_nav(source: Path, docs: list[tuple[Path, dict, str]], output_file: Path) -> str:
    parent = source.parent
    parent_index = parent / "index.md"
    index_entry = next((item for item in docs if item[0] == parent_index), None)
    index_out = (OUTPUT / parent_index.relative_to(BUNDLE)).with_suffix(".html")
    parent_link = Path(__import__("os").path.relpath(index_out, output_file.parent)).as_posix()
    links = []
    if index_entry:
        for match in LINK_RE.finditer(index_entry[2]):
            label, target = match.group(1), match.group(2).strip("<>")
            href = rewrite_target(target, parent_index, output_file)
            links.append(
                f'<li><a href="{html.escape(href, quote=True)}">{html.escape(label)}</a></li>'
            )
    return (
        '<nav class="docs-sidebar" aria-label="Documentation navigation">'
        f'<a class="docs-home" href="{html.escape(parent_link, quote=True)}">Section index</a>'
        + ("<ul>" + "".join(links) + "</ul>" if links else "")
        + "</nav>"
    )

def render_page(source: Path, data: dict, body: str, docs: list[tuple[Path, dict, str]]) -> str:
    output_file = (OUTPUT / source.relative_to(BUNDLE)).with_suffix(".html")
    output_file.parent.mkdir(parents=True, exist_ok=True)
    title = title_for(source, data, body)
    description = data.get("description", "")
    if not isinstance(description, str):
        description = ""
    stylesheet = Path(__import__("os").path.relpath(ROOT / "site" / "styles.css", output_file.parent)).as_posix()
    site_home = Path(__import__("os").path.relpath(ROOT / "site" / "index.html", output_file.parent)).as_posix()
    content = render_body(body, source, output_file)
    nav = page_nav(source, docs, output_file)
    metadata = ""
    if data.get("type"):
        metadata = f'<p class="docs-meta"><span>{html.escape(str(data["type"]))}</span></p>'
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="{html.escape(description, quote=True)}">
  <title>{html.escape(title)} · GFA BASIC Rust documentation</title>
  <link rel="stylesheet" href="{html.escape(stylesheet, quote=True)}">
</head>
<body class="docs-site">
  <header class="docs-header">
    <a href="{html.escape(site_home, quote=True)}">GFA BASIC Rust</a>
    <span aria-hidden="true">/</span>
    <a href="{html.escape(Path(__import__('os').path.relpath(OUTPUT / 'index.html', output_file.parent)).as_posix(), quote=True)}">Documentation</a>
  </header>
  <div class="docs-layout">
    {nav}
    <main class="docs-content">
      <article>
        <h1>{html.escape(title)}</h1>
        {metadata}
        {content}
      </article>
    </main>
  </div>
</body>
</html>
"""


def build(check: bool = False) -> list[Path]:
    sources = sorted(BUNDLE.rglob("*.md"))
    docs = []
    for path in sources:
        data, body = read_document(path)
        docs.append((path, data, body))
    generated = {}
    for path, data, body in docs:
        output_file = (OUTPUT / path.relative_to(BUNDLE)).with_suffix(".html")
        generated[output_file] = render_page(path, data, body, docs)
    if check:
        stale = []
        for path, content in generated.items():
            if not path.is_file() or path.read_text(encoding="utf-8") != content:
                stale.append(path)
        existing = set(OUTPUT.rglob("*.html")) if OUTPUT.exists() else set()
        stale.extend(sorted(existing - set(generated)))
        return stale
    if OUTPUT.exists():
        shutil.rmtree(OUTPUT)
    for path, content in generated.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8", newline="\n")
    return []


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if generated pages are stale")
    args = parser.parse_args()
    stale = build(check=args.check)
    if stale:
        for path in stale:
            print(f"STALE {path.relative_to(ROOT)}")
        return 1
    count = sum(1 for _ in OUTPUT.rglob("*.html")) if OUTPUT.exists() else len(list(BUNDLE.rglob("*.md")))
    print(f"OK documentation site: {count} generated page(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
