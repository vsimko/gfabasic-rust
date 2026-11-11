"""Regression tests for the Chapter 2–4 evidence-only candidate audit."""

from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/research/amiga-gfa-basic-candidates.toml"


class ChapterTwoToFourCandidateAuditTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.candidates = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["candidates"]
        cls.audit = [
            item for item in cls.candidates
            if item["id"].startswith(("amiga.base2.", "amiga.base3.", "amiga.base4."))
        ]

    def test_each_candidate_has_page_provenance_and_3_0_scope(self):
        self.assertEqual(len(self.audit), 57)
        for item in self.audit:
            with self.subTest(candidate=item["id"]):
                self.assertEqual(item["source"], "AM-INTERPRETER-35")
                self.assertEqual(item["source-version"], "3.0 base manual")
                self.assertTrue(item["printed-pages"])
                self.assertEqual(item["status"], "documented-in-3.0-only")

    def test_all_three_chapters_are_individually_tracked(self):
        for prefix in ("amiga.base2.", "amiga.base3.", "amiga.base4."):
            with self.subTest(chapter=prefix):
                self.assertTrue(any(item["id"].startswith(prefix) for item in self.audit))

    def test_compiler_only_and_ocr_sensitive_cases_remain_qualified(self):
        by_id = {item["id"]: item for item in self.audit}
        self.assertIn("compiler-only", by_id["amiga.base2.cint-cfloat"]["notes"])
        self.assertIn("mismatch", by_id["amiga.base2.cvx"]["notes"])
        self.assertIn("alternate spellings", by_id["amiga.base3.relational"]["notes"])
        self.assertIn("distinct", by_id["amiga.base4.integer-arithmetic-functions"]["notes"])


if __name__ == "__main__":
    unittest.main()
