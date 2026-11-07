import os
import sys
import urllib.error
import urllib.request
from html.parser import HTMLParser


class PageInspector(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = set()
        self.classes = set()
        self.links = []
        self.headings = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if attrs.get("id"):
            self.ids.add(attrs["id"])
        if attrs.get("class"):
            self.classes.update(attrs["class"].split())
        if tag == "a" and attrs.get("href"):
            self.links.append(attrs["href"])
        if tag in {"h1", "h2", "h3", "h4"}:
            self.headings.append(tag)


def main():
    url = os.environ.get("PREVIEW_URL")
    if not url:
        raise SystemExit("PREVIEW_URL is required")

    request = urllib.request.Request(url, headers={"User-Agent": "gfabasic-preview-qa/1.0"})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            if response.status != 200:
                raise AssertionError(f"Preview returned HTTP {response.status}")
            html = response.read().decode("utf-8")
    except (urllib.error.URLError, TimeoutError) as error:
        raise SystemExit(f"Could not load deployed preview {url}: {error}") from error

    inspector = PageInspector()
    inspector.feed(html)

    required_ids = {"dashboard", "roadmap", "api-catalogue", "main-nav"}
    missing_ids = required_ids - inspector.ids
    assert not missing_ids, f"Missing page sections/landmarks: {sorted(missing_ids)}"

    required_classes = {
        "dashboard-overview",
        "dashboard-stat",
        "dashboard-roadmap-progress",
        "dashboard-api-progress",
        "dashboard-progress-fill",
        "dashboard-component",
        "roadmap-phase",
    }
    missing_classes = required_classes - inspector.classes
    assert not missing_classes, f"Missing generated dashboard/roadmap HTML: {sorted(missing_classes)}"

    for text in ("Implementation dashboard", "Project maturity", "Product target",
                 "Workspace", "Implemented 3.5 addendum API", "0 / 28",
                 "not the complete Amiga GFA BASIC 3.5 language API",
                 "Roadmap progress", "Implementation states"):
        assert text in html, f"Missing expected dashboard content: {text}"

    assert "Loading generated repository data" not in html
    assert "Loading canonical roadmap phases" not in html
    assert "Loading…" not in html
    assert "data/project.json" not in html, "Dashboard should be rendered into static HTML"

    print(f"Preview HTML QA passed: {url}")
    print("Verified dashboard cards, scoped API and roadmap progress bars, implementation states, roadmap and API catalogue.")


if __name__ == "__main__":
    main()
