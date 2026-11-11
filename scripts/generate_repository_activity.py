#!/usr/bin/env python3
"""Build a bounded, source-linked GitHub activity feed for the static website."""

import argparse
import html
import json
import os
import urllib.error
import urllib.request
from datetime import datetime, timedelta, timezone
from pathlib import Path

API = "https://api.github.com"
REPOSITORY = "vsimko/gfabasic-rust"
LIMIT = 12
STALE_AFTER = timedelta(days=30)


class GitHubAPIError(RuntimeError):
    def __init__(self, message: str, permission_denied: bool = False):
        super().__init__(message)
        self.permission_denied = permission_denied


def iso_datetime(value: str | None) -> datetime | None:
    if not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except (TypeError, ValueError):
        return None


def request_json(path: str, token: str | None) -> list[dict]:
    headers = {"Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = urllib.request.Request(f"{API}/repos/{REPOSITORY}/{path}", headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as exc:
        raise GitHubAPIError(f"GitHub API returned HTTP {exc.code}", exc.code in (401, 403)) from exc
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
        raise GitHubAPIError(f"GitHub API unavailable: {exc}") from exc
    if not isinstance(payload, list):
        raise GitHubAPIError("GitHub API returned an unexpected response")
    return payload


def collect_activity(token: str | None, now: datetime | None = None) -> dict:
    now = now or datetime.now(timezone.utc)
    pull_requests = request_json("pulls?state=closed&sort=updated&direction=desc&per_page=100", token)
    commits = request_json("commits?per_page=100", token)
    items = []
    for pr in pull_requests:
        merged_at = pr.get("merged_at")
        if not merged_at:
            continue
        items.append({
            "type": "merged_pull_request",
            "title": pr.get("title") or "Merged pull request",
            "url": pr.get("html_url"),
            "number": pr.get("number"),
            "timestamp": merged_at,
            "summary": f"PR #{pr.get('number')} merged",
        })
    for commit in commits:
        details = commit.get("commit") or {}
        author = details.get("author") or {}
        committer = details.get("committer") or {}
        timestamp = committer.get("date") or author.get("date")
        if not timestamp:
            continue
        sha = commit.get("sha", "")
        items.append({
            "type": "commit",
            "title": (details.get("message") or "Commit").splitlines()[0],
            "url": commit.get("html_url"),
            "number": None,
            "timestamp": timestamp,
            "summary": f"Commit {sha[:7]}",
        })
    items = [item for item in items if isinstance(item["url"], str) and item["url"].startswith("https://github.com/")]
    items.sort(key=lambda item: iso_datetime(item["timestamp"]) or datetime.min.replace(tzinfo=timezone.utc), reverse=True)
    items = items[:LIMIT]
    latest = iso_datetime(items[0]["timestamp"]) if items else None
    status = "empty" if not items else "stale" if latest and now - latest > STALE_AFTER else "fresh"
    return {
        "schema_version": 1,
        "status": status,
        "generated_at": now.isoformat(),
        "last_successful_at": now.isoformat(),
        "items": items,
    }


def load_previous(path: Path) -> dict | None:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    if isinstance(data, dict) and isinstance(data.get("items"), list):
        return data
    return None


def build_activity(token: str | None, output: Path, now: datetime | None = None) -> dict:
    previous = load_previous(output)
    try:
        data = collect_activity(token, now)
    except GitHubAPIError as exc:
        if previous:
            data = previous
            data["status"] = "permission-denied" if exc.permission_denied else "stale"
            data["error"] = str(exc)
            data["generated_at"] = (now or datetime.now(timezone.utc)).isoformat()
        else:
            data = {
                "schema_version": 1,
                "status": "permission-denied" if exc.permission_denied else "unavailable",
                "generated_at": (now or datetime.now(timezone.utc)).isoformat(),
                "last_successful_at": None,
                "items": [],
                "error": str(exc),
            }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return data


def render_activity(data: dict) -> str:
    esc = html.escape
    labels = {
        "fresh": "Recently updated",
        "stale": "Showing last available snapshot · refresh is overdue",
        "empty": "No recent repository activity was found",
        "unavailable": "Repository activity is temporarily unavailable",
        "permission-denied": "GitHub denied access to repository activity",
    }
    parts = [
        '<section id="recent-activity" class="activity-section section-wrap" aria-labelledby="activity-title">',
        '<div class="section-heading"><div class="eyebrow">REPOSITORY · DELIVERY HISTORY</div><h2 id="activity-title">Recent activity.</h2></div>',
        f'<p class="activity-state">{esc(labels.get(data.get("status"), "Activity status unknown"))}</p>',
    ]
    items = data.get("items", [])
    if items:
        parts.append('<ol class="activity-list">')
        for item in items[:LIMIT]:
            kind = "Merged pull request" if item.get("type") == "merged_pull_request" else "Commit"
            stamp = item.get("timestamp", "")
            parts.append(
                '<li class="activity-item">'
                f'<span class="activity-kind">{kind}</span>'
                f'<time datetime="{esc(stamp)}">{esc(stamp[:10])}</time>'
                f'<a href="{esc(item["url"])}" target="_blank" rel="noreferrer">{esc(item["title"])} ↗</a>'
                f'<span class="activity-summary">{esc(item.get("summary", ""))}</span></li>'
            )
        parts.append('</ol>')
    if data.get("error"):
        parts.append('<p class="activity-note">The displayed items are a retained build-time snapshot; no browser-side GitHub request is made.</p>')
    last_success = data.get("last_successful_at")
    if last_success:
        parts.append(f'<p class="activity-provenance">Last successful source fetch: <time datetime="{esc(last_success)}">{esc(last_success[:10])}</time> · Maximum {LIMIT} items · GitHub PR merges and commits are activity events, not feature-completion claims.</p>')
    else:
        parts.append('<p class="activity-provenance">No successful source fetch is available yet. GitHub API credentials are never exposed to the browser.</p>')
    parts.append('</section>')
    return "\n".join(parts)


def render_site(root: Path, data: dict) -> None:
    path = root / "site/index.html"
    document = path.read_text(encoding="utf-8")
    start, end = "<!-- GENERATED ACTIVITY START -->", "<!-- GENERATED ACTIVITY END -->"
    if start not in document or end not in document:
        raise ValueError("site/index.html is missing activity generation markers")
    before, rest = document.split(start, 1)
    _, after = rest.split(end, 1)
    path.write_text(before + start + "\n" + render_activity(data) + "\n" + end + after, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--output", type=Path, default=Path("site/data/activity.json"))
    args = parser.parse_args()
    root = args.root.resolve()
    output = args.output if args.output.is_absolute() else root / args.output
    data = build_activity(os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN"), output)
    render_site(root, data)
    print(f"Generated repository activity ({data['status']}, {len(data['items'])} items)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
