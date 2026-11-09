#!/usr/bin/env python3
"""Generate a static, build-time snapshot of GitHub Actions health."""

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
import html
import json
import os
from pathlib import Path
import urllib.error
import urllib.parse
import urllib.request

REPOSITORY = "vsimko/gfabasic-rust"
API_ROOT = "https://api.github.com/repos/" + REPOSITORY
STALE_AFTER_HOURS = 24 * 7
WORKFLOWS = (
    ("Documentation checks", "docs.yml", "Markdown, JavaScript, API registry, corpus metadata and local-link validation."),
    ("Rust checks", "rust.yml", "Formatting, Clippy, workspace tests/build and formatter fixtures."),
    ("Security audit", "security-audit.yml", "RustSec audits for workspace, fuzz and benchmark lockfiles."),
    ("Source-loader fuzzing", "fuzz.yml", "Fuzzes the source loader only; lexer/parser fuzzing is not configured."),
    ("Benchmarks", "benchmarks.yml", "Source-loading benchmark; pull requests validate compilation without recording measurements."),
    ("Release SBOM", "release-sbom.yml", "Validates PR SBOMs and uploads on published releases; release attachment is unproven until a release runs."),
    ("GitHub Pages", "pages.yml", "Builds and publishes the static production website."),
    ("Pull request preview", "pr-preview.yml", "Builds and smoke-tests same-repository pull request previews."),
    ("Conformance suite", "conformance.yml", "No dedicated compiler conformance workflow is configured."),
)


class ApiError(RuntimeError):
    def __init__(self, status):
        self.status = status
        super().__init__(f"GitHub Actions API returned HTTP {status}")


def api_json(url, token):
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "gfabasic-ci-health/1.0",
    }
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            return json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as error:
        raise ApiError(error.code) from error
    except (urllib.error.URLError, TimeoutError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise RuntimeError("GitHub Actions API request failed") from error


def timestamp(value):
    if not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except (AttributeError, ValueError):
        return None


def summarize_run(run, now):
    updated = timestamp(run.get("updated_at")) or timestamp(run.get("created_at"))
    age_hours = max(0, int((now - updated).total_seconds() // 3600)) if updated else None
    status = run.get("status", "unknown")
    conclusion = run.get("conclusion")
    if status != "completed":
        result = status if status in {"queued", "in_progress", "waiting", "requested", "pending"} else "unknown"
    else:
        result = conclusion or "unknown"
    return {
        "id": run.get("id"),
        "name": run.get("name") or run.get("display_title") or "Workflow run",
        "status": result,
        "event": run.get("event"),
        "branch": run.get("head_branch"),
        "created_at": run.get("created_at"),
        "updated_at": run.get("updated_at"),
        "url": run.get("html_url"),
        "age_hours": age_hours,
        "freshness": "stale" if age_hours is not None and age_hours > STALE_AFTER_HOURS else "fresh" if age_hours is not None else "unknown",
    }


def collect(root, token, now=None):
    now = now or datetime.now(timezone.utc)
    snapshot = {
        "repository": REPOSITORY,
        "generated_at": now.isoformat().replace("+00:00", "Z"),
        "freshness_threshold_hours": STALE_AFTER_HOURS,
        "workflows": [],
    }
    configured = []
    for name, filename, note in WORKFLOWS:
        path = root / ".github" / "workflows" / filename
        configured.append((name, filename, note, path.is_file()))

    if not token:
        api_failure = "GH_TOKEN is unavailable; workflow status was not queried."
        workflow_index = {}
    else:
        try:
            response = api_json(f"{API_ROOT}/actions/workflows?per_page=100", token)
            workflow_index = {item.get("path", "").removeprefix(".github/workflows/"): item
                              for item in response.get("workflows", [])}
            api_failure = None
        except (ApiError, RuntimeError) as error:
            workflow_index = {}
            api_failure = str(error)

    workflow_data = {}
    for name, filename, note, file_exists in configured:
        item = {"name": name, "file": filename, "note": note}
        if file_exists:
            item["workflow_url"] = f"https://github.com/{REPOSITORY}/blob/main/.github/workflows/{filename}"
        workflow = workflow_index.get(filename) if file_exists else None
        workflow_data[filename] = {"item": item, "workflow": workflow, "file_exists": file_exists}

    runs_by_workflow = {}
    runs_errors = {}
    workflow_ids = {
        entry["workflow"]["id"]
        for entry in workflow_data.values()
        if entry["file_exists"] and entry["workflow"]
    }

    def fetch_workflow_runs(workflow_id):
        query = urllib.parse.urlencode({"per_page": 100})
        try:
            response = api_json(f"{API_ROOT}/actions/workflows/{workflow_id}/runs?{query}", token)
            return workflow_id, response.get("workflow_runs", []), None
        except (ApiError, RuntimeError) as error:
            return workflow_id, [], str(error)

    if token and not api_failure and workflow_ids:
        with ThreadPoolExecutor(max_workers=min(8, len(workflow_ids))) as executor:
            futures = [executor.submit(fetch_workflow_runs, workflow_id) for workflow_id in workflow_ids]
            for future in as_completed(futures):
                workflow_id, runs, error = future.result()
                if error:
                    runs_errors[workflow_id] = error
                else:
                    runs_by_workflow[workflow_id] = runs

    for name, filename, note, file_exists in configured:
        entry = workflow_data[filename]
        item = entry["item"]
        workflow = entry["workflow"]
        if not file_exists:
            item.update({"state": "not-configured", "latest": None, "last_success": None})
        elif api_failure:
            item.update({"state": "unknown", "error": api_failure, "latest": None, "last_success": None})
        elif not workflow:
            item.update({"state": "unknown", "error": "Workflow file exists but GitHub did not return it.", "latest": None, "last_success": None})
        elif workflow["id"] in runs_errors:
            item.update({"state": "unknown", "error": runs_errors[workflow["id"]], "latest": None, "last_success": None})
        else:
            runs = runs_by_workflow.get(workflow["id"], [])
            runs.sort(key=lambda run: run.get("created_at") or "", reverse=True)
            latest = summarize_run(runs[0], now) if runs else None
            successful = next((run for run in runs if run.get("status") == "completed" and run.get("conclusion") == "success"), None)
            last_success = summarize_run(successful, now) if successful else None
            state = latest["status"] if latest else "unknown"
            if latest and latest["freshness"] == "stale" and state == "success":
                state = "stale"
            item.update({"state": state, "latest": latest, "last_success": last_success})
        snapshot["workflows"].append(item)
    return snapshot


def render(snapshot):
    esc = html.escape
    cards = []
    for item in snapshot["workflows"]:
        state = item["state"]
        latest = item.get("latest")
        success = item.get("last_success")
        latest_text = "No workflow run recorded" if not latest else (
            f'Latest: {esc(latest["status"].replace("_", " "))} · '
            f'{esc(latest.get("event") or "event unknown")} · '
            f'{esc(latest.get("branch") or "branch unknown")}'
        )
        if latest and latest.get("age_hours") is not None:
            latest_text += f' · {latest["age_hours"]}h ago · {latest["freshness"]}'
        latest_link = f'<a href="{esc(item["workflow_url"])}">Workflow definition ↗</a>' if item.get("workflow_url") else ""
        if latest and latest.get("url"):
            latest_link += f' <a href="{esc(latest["url"])}">Latest run ↗</a>'
        success_text = "No successful run recorded"
        if success:
            success_text = f'Last success: {esc(success.get("updated_at") or success.get("created_at") or "time unknown")}'
            if success.get("age_hours") is not None:
                success_text += f' · {success["age_hours"]}h ago'
            if success.get("url"):
                success_text += f' · <a href="{esc(success["url"])}">Open successful run ↗</a>'
        error = f'<p class="ci-health-error">{esc(item["error"])}</p>' if item.get("error") else ""
        cards.append(
            f'<article class="ci-health-card" data-state="{esc(state)}">'
            f'<div class="ci-health-card-top"><h3>{esc(item["name"])}</h3>'
            f'<span class="ci-health-state state-{esc(state)}">{esc(state.replace("-", " ").replace("_", " ").upper())}</span></div>'
            f'<p class="ci-health-latest">{latest_text}</p>'
            f'<p class="ci-health-success">{success_text}</p>'
            f'<p class="ci-health-note">{esc(item["note"])}</p>'
            f'<div class="ci-health-links">{latest_link}</div>{error}</article>'
        )
    generated = esc(snapshot["generated_at"])
    return (
        '<section id="ci-health" class="ci-health-section section-wrap" aria-labelledby="ci-health-title">'
        '<div class="section-heading"><div class="eyebrow">MAINTAINER VIEW · GITHUB ACTIONS</div>'
        '<h2 id="ci-health-title">CI &amp; supply-chain health.</h2></div>'
        '<p class="ci-health-intro">Build-time snapshot from GitHub Actions. Latest result and last successful run are shown separately; missing runs, stale results and unconfigured workflows are never counted as healthy.</p>'
        '<div class="ci-health-grid">' + "".join(cards) + '</div>'
        f'<p class="ci-health-generated">Snapshot generated {generated}. Freshness threshold: {STALE_AFTER_HOURS} hours. '
        '<a href="https://github.com/vsimko/gfabasic-rust/actions">View all workflow runs ↗</a></p>'
        '</section>'
    )


def write_snapshot(root, snapshot):
    data_path = root / "site/data/ci-health.json"
    data_path.parent.mkdir(parents=True, exist_ok=True)
    data_path.write_text(json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    index_path = root / "site/index.html"
    document = index_path.read_text(encoding="utf-8")
    start, end = "<!-- GENERATED CI HEALTH START -->", "<!-- GENERATED CI HEALTH END -->"
    if start not in document or end not in document:
        raise ValueError("site/index.html is missing CI health generation markers")
    before, remainder = document.split(start, 1)
    _, after = remainder.split(end, 1)
    document = before + start + "\n" + render(snapshot) + "\n" + end + after
    index_path.write_text(document, encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    args = parser.parse_args()
    root = args.root.resolve()
    snapshot = collect(root, os.environ.get("GH_TOKEN"))
    write_snapshot(root, snapshot)
    counts = {}
    for item in snapshot["workflows"]:
        counts[item["state"]] = counts.get(item["state"], 0) + 1
    print("Generated CI health snapshot: " + ", ".join(f"{key}={value}" for key, value in sorted(counts.items())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
