#!/usr/bin/env python3
"""Generate deterministic, evidence-checked website data from repository sources."""

import argparse
import html
import json
import re
import subprocess
import tomllib
from pathlib import Path

COMPONENT_STATUSES = {"implemented", "partial", "prototype", "research", "planned", "blocked"}
PHASE_HEADING = re.compile(r"^### Phase (\d+) — (.+)$", re.MULTILINE)
TASK_CHECKBOX = re.compile(r"^\s*-\s+\[([ xX])\]", re.MULTILINE)


def parse_roadmap_progress(markdown: str) -> list[dict]:
    """Extract canonical phase summaries, task progress, and issue links."""
    headings = list(PHASE_HEADING.finditer(markdown))
    phases = []
    issue_link = re.compile(r"\[([^\]]+)\]\((https://github\.com/vsimko/gfabasic-rust/issues/[^)]+)\)")
    for index, heading in enumerate(headings):
        end = headings[index + 1].start() if index + 1 < len(headings) else len(markdown)
        next_section = re.search(r"^### ", markdown[heading.end():end], re.MULTILINE)
        if next_section:
            end = heading.end() + next_section.start()
        section = markdown[heading.end():end]
        first_task = TASK_CHECKBOX.search(section)
        description_text = section[:first_task.start()] if first_task else section
        description = " ".join(line.strip() for line in description_text.splitlines() if line.strip())
        task_text = section[first_task.start():] if first_task else ""
        tasks = TASK_CHECKBOX.findall(task_text)
        completed = sum(mark.lower() == "x" for mark in tasks)
        total = len(tasks)
        issue_links = []
        seen_urls = set()
        for label, url in issue_link.findall(task_text):
            if url not in seen_urls:
                issue_links.append({"label": label, "url": url})
                seen_urls.add(url)
        phases.append({
            "number": int(heading.group(1)),
            "name": heading.group(2).strip(),
            "description": description,
            "completed": completed,
            "total": total,
            "percent": round(completed * 100 / total) if total else 0,
            "status": "complete" if total and completed == total else "in-progress" if completed else "planned",
            "issue_links": issue_links,
        })
    return phases


def parse_roadmap_blockers(markdown: str) -> list[dict]:
    """Extract the explicit sequencing rules and their linked issue references."""
    heading = re.search(r"^## Blockers and sequencing rules\s*$", markdown, re.MULTILINE)
    if not heading:
        return []
    remainder = markdown[heading.end():]
    next_section = re.search(r"^## ", remainder, re.MULTILINE)
    section = remainder[:next_section.start()] if next_section else remainder
    issue_link = re.compile(r"\[([^\]]+)\]\((https://github\.com/vsimko/gfabasic-rust/issues/[^)]+)\)")
    blockers = []
    for line in section.splitlines():
        item = re.match(r"^\d+\.\s+(.*)$", line)
        if not item:
            continue
        source = item.group(1).strip()
        links = [{"label": label, "url": url} for label, url in issue_link.findall(source)]
        summary = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", source)
        blockers.append({"summary": summary, "issue_links": links})
    return blockers

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


def load_addendum_api_progress(root: Path) -> dict:
    """Count only declarations explicitly scoped to the Amiga 3.5 addendum."""
    registry_root = root / "api" / "registry"
    declarations = []
    for path in sorted(registry_root.rglob("*.toml")):
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        declarations.extend(data.get("declarations", []))

    population = [
        declaration
        for declaration in declarations
        if declaration.get("evidence", {}).get("source-scope") == "amiga-3.5-addendum"
        and any(
            availability.get("dialect") == "amiga-gfa-basic-interpreter"
            and "3.5" in availability.get("versions", [])
            for availability in declaration.get("availability", [])
        )
    ]
    if not population:
        raise ValueError("the Amiga 3.5 addendum API inventory is empty")

    implemented = sum(
        declaration.get("implementation", {}).get("status") == "implemented"
        for declaration in population
    )
    total = len(population)
    return {
        "implemented": implemented,
        "total": total,
        "percent": round(implemented * 100 / total),
        "population": "Explicitly documented interpreter declarations from the Amiga 3.5 addendum only; includes language-core and Amiga platform declarations. Excludes inherited 3.0 base-manual features, editor-only changes, compiler-only features, and Atari ST declarations.",
    }

def source_revision(root: Path) -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else "unknown"



def render_project_dashboard(data: dict) -> tuple[str, str]:
    """Render repository status and roadmap as crawlable, build-time HTML."""
    esc = html.escape
    project = data["project"]
    stats = [
        ("Project maturity", project.get("maturity", "Unknown"), "Experimental project"),
        ("Product target", project.get("target", "Unknown"), project.get("source_dialect", "Dialect unknown")),
        ("Workspace", str(len(data["workspace_packages"])), "Rust crates in Cargo workspace"),
    ]
    overview = '<div class="dashboard-overview">' + "".join(
        f'<article class="dashboard-stat"><span class="dashboard-stat-label">{esc(label)}</span>'
        f'<strong>{esc(value)}</strong><small>{esc(note)}</small></article>'
        for label, value, note in stats
    ) + '</div>'
    api = data["api_progress"]
    api_percent = max(0, min(100, int(api["percent"])))
    api_progress = (
        '<h3 class="dashboard-subheading">Implemented 3.5 addendum API</h3>'
        '<div class="dashboard-api-progress"><div class="dashboard-progress-top">'
        '<span class="dashboard-progress-name">Fully implemented declarations</span>'
        f'<span class="dashboard-progress-value">{api["implemented"]} / {api["total"]}</span></div>'
        f'<div class="dashboard-progress-track" role="progressbar" aria-label="Implemented Amiga GFA BASIC 3.5 addendum API" '
        f'aria-valuemin="0" aria-valuemax="100" aria-valuenow="{api_percent}">'
        f'<span class="dashboard-progress-fill" style="width:{api_percent}%"></span></div>'
        f'<p class="dashboard-provenance">{esc(api["population"])} This is a confirmed addendum subset, not the complete Amiga GFA BASIC 3.5 language API.</p></div>'
    )
    progress = ['<h3 class="dashboard-subheading">Roadmap progress</h3><div class="dashboard-roadmap-progress">']
    for phase in data["roadmap_progress"]:
        percent = max(0, min(100, int(phase["percent"])))
        label = esc("Phase " + str(phase["number"]) + ": " + phase["name"])
        progress.append(
            '<div class="dashboard-phase-progress"><div class="dashboard-progress-top">'
            f'<span class="dashboard-progress-name">Phase {phase["number"]} · {esc(phase["name"])}</span>'
            f'<span class="dashboard-progress-value">{phase["completed"]}/{phase["total"]} · {percent}%</span>'
            '</div>'
            f'<div class="dashboard-progress-track" role="progressbar" aria-label="{label}" aria-valuemin="0" aria-valuemax="100" aria-valuenow="{percent}">'
            f'<span class="dashboard-progress-fill" style="width:{percent}%"></span></div></div>'
        )
    progress.append('</div>')
    components = ['<h3 class="dashboard-subheading">Implementation states</h3><div class="dashboard-components">']
    for item in data["components"]:
        evidence = "".join(
            f'<a href="https://github.com/vsimko/gfabasic-rust/blob/main/{esc(ref.split("#", 1)[0])}'
            f'{("#" + esc(ref.split("#", 1)[1])) if "#" in ref else ""}" target="_blank" rel="noreferrer">{esc(ref)} ↗</a>'
            for ref in item["evidence"]
        )
        issues = "".join(
            f'<a href="https://github.com/vsimko/gfabasic-rust/issues/{number}" target="_blank" rel="noreferrer">Issue #{number} ↗</a>'
            for number in item["related_issues"]
        )
        components.append(
            f'<article class="dashboard-component"><div class="dashboard-component-top"><h4>{esc(item["name"])}</h4>'
            f'<span class="dashboard-status">{esc(item["status"].replace("-", " ").title())}</span></div>'
            f'<p>{esc(item["rationale"])}</p><div class="dashboard-evidence">{evidence}{issues}</div></article>'
        )
    components.append('</div>')
    footer = (
        f'<div class="dashboard-footer"><a href="https://github.com/vsimko/gfabasic-rust/blob/main/validation/project.json">Canonical status model ↗</a>'
        '<a href="https://github.com/vsimko/gfabasic-rust/blob/main/ROADMAP.md">Delivery roadmap ↗</a>'
        f'<a href="https://github.com/vsimko/gfabasic-rust/commit/{esc(data["source_revision"])}">Source revision: {esc(data["source_revision"][:7])} ↗</a>'
        '<a href="https://github.com/vsimko/gfabasic-rust/issues">Open implementation issues ↗</a></div>'
    )
    dashboard = overview + api_progress + "".join(progress) + "".join(components) + footer

    roadmap = []
    for phase in data["roadmap_progress"]:
        percent = max(0, min(100, int(phase["percent"])))
        state = "COMPLETE" if phase["status"] == "complete" else "IN PROGRESS" if phase["status"] == "in-progress" else "PLANNED"
        links = "".join(
            f'<a href="{esc(link["url"])}" target="_blank" rel="noreferrer">{esc(link["label"])} ↗</a>'
            for link in phase["issue_links"]
        )
        roadmap.append(
            f'<article class="roadmap-phase"><span class="roadmap-state {esc(phase["status"])}">{state}</span>'
            f'<div class="roadmap-phase-body"><h3>Phase {phase["number"]} · {esc(phase["name"])}</h3>'
            f'<p>{esc(phase["description"])}</p><div class="roadmap-phase-progress">'
            f'<div class="dashboard-progress-track" role="progressbar" aria-label="{esc("Phase " + str(phase["number"]) + ": " + phase["name"])}" '
            f'aria-valuemin="0" aria-valuemax="100" aria-valuenow="{percent}"><span class="dashboard-progress-fill" style="width:{percent}%"></span></div>'
            f'<span class="roadmap-phase-count">{phase["completed"]}/{phase["total"]} tasks · {percent}%</span></div>'
            f'<div class="roadmap-phase-issues">{links}</div></div></article>'
        )
    if data["roadmap_blockers"]:
        roadmap.append('<h3 class="roadmap-blockers-heading">Dependencies &amp; sequencing rules</h3><div class="roadmap-blockers">')
        for blocker in data["roadmap_blockers"]:
            links = "".join(
                f'<a href="{esc(link["url"])}" target="_blank" rel="noreferrer">{esc(link["label"])} ↗</a>'
                for link in blocker["issue_links"]
            )
            roadmap.append(f'<article class="roadmap-blocker"><p>{esc(blocker["summary"])}</p><div class="roadmap-phase-issues">{links}</div></article>')
        roadmap.append('</div>')
    return dashboard, "".join(roadmap)


def render_static_site(root: Path, data: dict) -> None:
    """Insert generated content into the HTML so it is present without JavaScript."""
    path = root / "site/index.html"
    document = path.read_text(encoding="utf-8")
    dashboard, roadmap = render_project_dashboard(data)
    replacements = {
        "DASHBOARD": dashboard,
        "ROADMAP": roadmap,
    }
    for name, content in replacements.items():
        start = f"<!-- GENERATED {name} START -->"
        end = f"<!-- GENERATED {name} END -->"
        if start not in document or end not in document:
            raise ValueError(f"site/index.html is missing {name} generation markers")
        before, remainder = document.split(start, 1)
        _, after = remainder.split(end, 1)
        document = before + start + "\n" + content + "\n" + end + after
    path.write_text(document, encoding="utf-8")

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
        "roadmap_progress": parse_roadmap_progress((root / "ROADMAP.md").read_text(encoding="utf-8")),
        "roadmap_blockers": parse_roadmap_blockers((root / "ROADMAP.md").read_text(encoding="utf-8")),
        "workspace_packages": packages,
        "api_progress": load_addendum_api_progress(root),
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    if (root / "site/index.html").is_file():
        render_static_site(root, result)
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
