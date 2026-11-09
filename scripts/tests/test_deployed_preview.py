import os
import sys
import time
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
    html = None
    last_error = None
    for attempt in range(6):
        try:
            with urllib.request.urlopen(request, timeout=10) as response:
                if response.status != 200:
                    raise urllib.error.URLError(f"Preview returned HTTP {response.status}")
                html = response.read().decode("utf-8")
            break
        except (urllib.error.URLError, TimeoutError) as error:
            last_error = error
            if attempt < 5:
                print(f"Preview is not ready yet (attempt {attempt + 1}/6); retrying in 10 seconds.")
                time.sleep(10)
    if html is None:
        raise SystemExit(f"Could not load deployed preview {url} after 6 attempts: {last_error}") from last_error

    inspector = PageInspector()
    inspector.feed(html)

    required_ids = {"dashboard", "ci-health", "roadmap", "api-catalogue", "main-nav"}
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
        "ci-health-grid",
        "ci-health-card",
    }
    missing_classes = required_classes - inspector.classes
    assert not missing_classes, f"Missing generated dashboard/roadmap HTML: {sorted(missing_classes)}"

    for text in ("Implementation dashboard", "CI &amp; supply-chain health", "Conformance suite", "NOT CONFIGURED", "Project maturity", "Product target",
                 "Workspace", "Implemented 3.5 addendum API", "0 / 28",
                 "not the complete Amiga GFA BASIC 3.5 language API",
                 "Roadmap progress", "Implementation states"):
        assert text in html, f"Missing expected dashboard content: {text}"

    assert "Loading generated repository data" not in html
    assert "Loading canonical roadmap phases" not in html
    assert "Loading…" not in html
    assert "data/project.json" not in html, "Dashboard should be rendered into static HTML"


    docs_url = url.rstrip("/") + "/docs/index.html"
    docs_request = urllib.request.Request(docs_url, headers={"User-Agent": "gfabasic-preview-qa/1.0"})
    with urllib.request.urlopen(docs_request, timeout=10) as response:
        docs_html = response.read().decode("utf-8")
    assert response.status == 200
    assert "<title>Project documentation · GFA BASIC Rust documentation</title>" in docs_html
    assert "OKF v0.2" in docs_html
    assert 'href="architecture/index.html"' in docs_html

    adr_url = url.rstrip("/") + "/docs/architecture/adr/0011-amiga-35-linux-sdl3-product-scope.html"
    with urllib.request.urlopen(adr_url, timeout=10) as response:
        adr_html = response.read().decode("utf-8")
    assert response.status == 200
    assert "Amiga 3.5 and Linux SDL3 product scope" in adr_html
    assert "<script" not in adr_html.lower(), "Generated documentation must not execute embedded scripts"

    print(f"Verified generated documentation index and ADR page: {docs_url}")

    print(f"Preview HTML QA passed: {url}")
    print("Verified dashboard cards, CI workflow health, scoped API and roadmap progress bars, implementation states, roadmap and API catalogue.")


if __name__ == "__main__":
    main()
