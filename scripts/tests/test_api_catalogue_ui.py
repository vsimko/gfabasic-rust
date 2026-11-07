import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class ApiCatalogueRenderingTests(unittest.TestCase):
    def test_collapsed_categories_create_no_function_or_detail_dom(self):
        source = (ROOT / "site/script.js").read_text(encoding="utf-8")
        render_entries = source.split("  const renderEntries = () => {", 1)[1].split(
            "\n  fetch(\"data/api-catalogue.json\")", 1
        )[0]
        before_population = render_entries.split("const populateNames = () => {", 1)[0]
        populate_names = render_entries.split("const populateNames = () => {", 1)[1].split(
            "\n      };", 1
        )[0]
        load_detail = source.split("  const loadDetail = async", 1)[1].split(
            "\n  const renderEntries", 1
        )[0]
        render_detail = source.split("  const renderDetail =", 1)[1].split(
            "\n  const loadDetail", 1
        )[0]

        self.assertIn('document.createElement("details")', before_population)
        self.assertNotIn('className = "api-function"', before_population)
        self.assertNotIn('document.createElement("details")', before_population)
        self.assertIn('category.addEventListener("toggle"', render_entries)
        self.assertIn('if (category.open) populateNames()', render_entries)
        self.assertIn('category.dataset.namesLoaded = "true"', populate_names)
        self.assertIn('className = "api-function"', populate_names)
        self.assertIn('className = "api-function-panel"', populate_names)
        self.assertNotIn("api-entry-detail", populate_names)
        self.assertIn("if (item.open) loadDetail(entry, panel)", populate_names)
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
