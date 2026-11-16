import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from scripts import generate_docs_site


class DocumentationSiteTests(unittest.TestCase):
    def test_draft_is_not_published_and_deprecated_is_labelled(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            bundle = root / "docs"
            site = root / "site"
            bundle.mkdir()
            site.mkdir()
            (site / "styles.css").write_text("", encoding="utf-8")
            (site / "index.html").write_text("", encoding="utf-8")
            (bundle / "index.md").write_text(
                '---\nokf_version: "0.2"\n---\n# Documentation\n'
                '- [Draft guide](draft.md)\n- [Old guide](old.md)\n',
                encoding="utf-8",
            )
            (bundle / "draft.md").write_text(
                '---\ntype: Guide\nstatus: draft\n---\n# Private draft\n',
                encoding="utf-8",
            )
            (bundle / "old.md").write_text(
                '---\ntype: Guide\nstatus: deprecated\n---\n# Old guide\n',
                encoding="utf-8",
            )
            output = site / "docs"
            with patch.object(generate_docs_site, "ROOT", root), patch.object(
                generate_docs_site, "BUNDLE", bundle
            ), patch.object(generate_docs_site, "OUTPUT", output):
                generate_docs_site.build()
                self.assertFalse((output / "draft.html").exists())
                self.assertIn("Deprecated concept", (output / "old.html").read_text(encoding="utf-8"))
                index_html = (output / "index.html").read_text(encoding="utf-8")
                self.assertNotIn('href="draft.html"', index_html)
                self.assertIn("Draft guide", index_html)

    def test_generates_safe_deterministic_pages_and_rewrites_links(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            bundle = root / "docs"
            site = root / "site"
            bundle.mkdir()
            site.mkdir()
            (site / "styles.css").write_text("body { color: black; }", encoding="utf-8")
            (site / "index.html").write_text("<!doctype html>", encoding="utf-8")
            (bundle / "index.md").write_text(
                '---\nokf_version: "0.2"\n---\n\n# Documentation\n\n- [Guide](guide.md) - A test guide.\n',
                encoding="utf-8",
            )
            (bundle / "guide.md").write_text(
                '---\ntype: Guide\ntitle: Test guide\ndescription: A test guide.\n---\n\n'
                '# Guide\n\nSee the [documentation index](index.md).\n\n'
                '<script>alert("unsafe")</script>\n',
                encoding="utf-8",
            )
            output = site / "docs"
            with patch.object(generate_docs_site, "ROOT", root), patch.object(
                generate_docs_site, "BUNDLE", bundle
            ), patch.object(generate_docs_site, "OUTPUT", output):
                self.assertEqual(generate_docs_site.build(), [])
                guide_html = (output / "guide.html").read_text(encoding="utf-8")
                index_html = (output / "index.html").read_text(encoding="utf-8")
                self.assertIn('href="index.html"', guide_html)
                self.assertIn("Test guide", guide_html)
                self.assertNotIn("<script>", guide_html)
                self.assertIn("&lt;script&gt;", guide_html)
                self.assertIn('href="guide.html"', index_html)
                self.assertEqual(generate_docs_site.build(check=True), [])
                first = guide_html
                generate_docs_site.build()
                self.assertEqual((output / "guide.html").read_text(encoding="utf-8"), first)


if __name__ == "__main__":
    unittest.main()
