import unittest
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

class SiteStructure(HTMLParser):
    def __init__(self):
        super().__init__()
        self.attributes = {}

    def handle_starttag(self, tag, attrs):
        for key, value in attrs:
            if key == "id" and value:
                self.attributes[value] = tag

class DashboardAccessibilityTests(unittest.TestCase):
    def test_page_has_skip_link_landmarks_and_heading_hierarchy(self):
        html = (ROOT / "site/index.html").read_text(encoding="utf-8")
        parser = SiteStructure()
        parser.feed(html)
        self.assertIn('<a class="skip-link" href="#main-content">', html)
        self.assertEqual(parser.attributes.get("main-content"), "main")
        self.assertIn('aria-label="Main navigation"', html)
        self.assertIn('aria-labelledby="dashboard-title"', html)
        self.assertIn('<h1>', html)
        self.assertIn('<h2 id="dashboard-title">', html)

    def test_mobile_menu_exposes_state_and_escape_closes_from_any_focus(self):
        html = (ROOT / "site/index.html").read_text(encoding="utf-8")
        script = (ROOT / "site/script.js").read_text(encoding="utf-8")
        self.assertIn('aria-expanded="false"', html)
        self.assertIn('aria-controls="main-nav"', html)
        self.assertIn('document.addEventListener("keydown"', script)
        self.assertIn('event.key === "Escape"', script)
        self.assertIn('menuButton.focus()', script)

    def test_mobile_layout_and_visible_keyboard_focus_are_defined(self):
        css = (ROOT / "site/styles.css").read_text(encoding="utf-8")
        self.assertIn("@media(max-width:700px)", css)
        self.assertIn(":focus-visible", css)
        self.assertIn(".skip-link:focus", css)
        self.assertIn("@media(prefers-reduced-motion:reduce)", css)

if __name__ == "__main__":
    unittest.main()
