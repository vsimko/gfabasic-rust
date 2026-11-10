#!/usr/bin/env python3
"""Smoke-test the production OKF documentation pages after GitHub Pages deployment."""

from __future__ import annotations

import os
import time
import urllib.error
import urllib.request


def fetch(url: str) -> str:
    request = urllib.request.Request(url, headers={"User-Agent": "gfabasic-production-docs-qa/1.0"})
    last_error = None
    for attempt in range(6):
        try:
            with urllib.request.urlopen(request, timeout=15) as response:
                if response.status != 200:
                    raise urllib.error.URLError(f"HTTP {response.status}")
                return response.read().decode("utf-8")
        except (urllib.error.URLError, TimeoutError) as error:
            last_error = error
            if attempt < 5:
                time.sleep(10)
    raise SystemExit(f"Could not load {url}: {last_error}") from last_error


def main() -> None:
    index_url = os.environ.get("DOCS_URL", "https://vsimko.github.io/gfabasic-rust/docs/index.html")
    index_html = fetch(index_url)
    assert "<title>Project documentation · GFA BASIC Rust documentation</title>" in index_html
    assert "OKF v0.2" in index_html
    assert 'href="architecture/index.html"' in index_html

    adr_url = index_url.rsplit("/", 1)[0] + "/architecture/adr/0011-amiga-35-linux-sdl3-product-scope.html"
    adr_html = fetch(adr_url)
    assert "Amiga 3.5 and Linux SDL3 product scope" in adr_html
    assert "<script" not in adr_html.lower()

    css_href = 'href="../../../styles.css"'
    assert css_href in adr_html
    css_url = adr_url.rsplit("/", 4)[0] + "/styles.css"
    fetch(css_url)
    print(f"Production documentation smoke test passed: {index_url}")


if __name__ == "__main__":
    main()
