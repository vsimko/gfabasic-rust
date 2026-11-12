from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from scripts.generate_amiga_candidate_report import generate, load_candidates, render


class AmigaCandidateReportTests(unittest.TestCase):
    def test_report_separates_3_0_only_from_addendum_confirmed(self) -> None:
        root = Path(__file__).resolve().parents[2]
        manifest = root / "docs/research/amiga-gfa-basic-candidates.toml"
        candidates = load_candidates(manifest)
        report = render(candidates)
        self.assertIn("amiga.base.variables-memory", report)
        self.assertIn("documented-in-3.0-only", report)
        self.assertIn("3.5 applicability not established", report)
        self.assertIn("amiga.35.mat-copy", report)
        self.assertIn("amiga.base4.rand", report)
        self.assertIn("amiga.base2.hex", report)
        self.assertIn("Direct example use", report)
        self.assertIn("13-36", report)
        self.assertIn("3.0 base manual", report)
        self.assertIn("3.5 addendum", report)
        self.assertIn("not implementation or runtime support", report)
        self.assertTrue(generate(root, Path("docs/research/amiga-gfa-basic-candidates.toml"), Path("docs/research/amiga-gfa-basic-candidate-report.md"), check=True))

    def test_duplicate_ids_are_rejected(self) -> None:
        source = """schema-version = 1
[[candidates]]
id = "same"
name = "One"
kind = "declaration"
source = "AM-INTERPRETER-35"
source-version = "3.0 base manual"
printed-pages = ["2-2"]
status = "unresolved"
notes = "one"
[[candidates]]
id = "same"
name = "Two"
kind = "declaration"
source = "AM-INTERPRETER-35"
source-version = "3.0 base manual"
printed-pages = ["2-3"]
status = "unresolved"
notes = "two"
"""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "manifest.toml"
            path.write_text(source, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate candidate ID"):
                load_candidates(path)

    def test_missing_page_locator_is_rejected(self) -> None:
        source = """schema-version = 1
[[candidates]]
id = "missing-pages"
name = "No pages"
kind = "declaration"
source = "AM-INTERPRETER-35"
source-version = "3.0 base manual"
printed-pages = []
status = "unresolved"
notes = "locator required"
"""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "manifest.toml"
            path.write_text(source, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "printed page locators"):
                load_candidates(path)


if __name__ == "__main__":
    unittest.main()
