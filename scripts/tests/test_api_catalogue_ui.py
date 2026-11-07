import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class ApiCatalogueRenderingTests(unittest.TestCase):
    def test_collapsed_function_items_have_no_detail_dom(self):
        source = (ROOT / "site/script.js").read_text(encoding="utf-8")
        render_entries = source.split("  const renderEntries = () => {", 1)[1].split(
            "\n  fetch(\"data/api-catalogue.json\")", 1
        )[0]
        load_detail = source.split("  const loadDetail = async", 1)[1].split(
            "\n  const renderEntries", 1
        )[0]
        render_detail = source.split("  const renderDetail =", 1)[1].split(
            "\n  const loadDetail", 1
        )[0]

        self.assertIn('document.createElement("details")', render_entries)
        self.assertIn('document.createElement("summary")', render_entries)
        self.assertIn('document.createElement("div")', render_entries)
        self.assertIn('panel.className = "api-function-panel"', render_entries)
        self.assertNotIn("api-entry-detail", render_entries)
        self.assertNotIn("entry.syntax", render_entries)
        self.assertIn("if (item.open) loadDetail(entry, panel)", render_entries)
        self.assertIn('fetch(`data/${entry.detail}`)', load_detail)
        self.assertIn("renderDetail(panel", load_detail)
        self.assertIn('content.className = "api-entry-detail"', render_detail)

    def test_catalogue_uses_only_static_json_paths(self):
        source = (ROOT / "site/script.js").read_text(encoding="utf-8")
        self.assertIn('fetch("data/api-catalogue.json")', source)
        self.assertIn('fetch(`data/${entry.detail}`)', source)
        self.assertNotIn("/api/", source)


if __name__ == "__main__":
    unittest.main()
