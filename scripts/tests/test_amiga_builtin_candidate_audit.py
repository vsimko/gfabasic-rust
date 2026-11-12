"""Regression tests for granular Chapter 2–4 evidence records."""

from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]
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
        self.assertEqual(len(self.audit), 69)
        for item in self.audit:
            with self.subTest(candidate=item["id"]):
                self.assertEqual(item["source"], "AM-INTERPRETER-35")
                self.assertEqual(item["source-version"], "3.0 base manual")
                self.assertTrue(item["printed-pages"])
                if item["status"] == "confirmed-in-3.5":
                    expected_sources = {
                        "amiga.base2.hex": ("AM-INTERPRETER-35", "3.5 addendum", ["13-36"], "direct-example-use"),
                        "amiga.base2.str": ("AM-INTERPRETER-35", "3.5 addendum", ["13-3"], "direct-documentation-reference"),
                        "amiga.base4.rand": ("AM-INTERPRETER-35", "3.5 addendum", ["13-2"], "direct-example-use"),
                        "amiga.base4.succ": ("AM-INTERPRETER-35", "3.5 addendum", ["13-5"], "direct-example-use"),
                        "amiga.base4.string": ("AM-INTERPRETER-35", "3.5 addendum", ["13-21"], "direct-example-use"),
                        "amiga.base4.rnd": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18", ["458"], "direct-example-use"),
                        "amiga.base4.int": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18", ["458"], "direct-example-use"),
                        "amiga.base4.sqr": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18", ["473"], "direct-example-use"),
                        "amiga.base3.numeric-add-sub": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18 example", ["467", "470"], "direct-example-use"),
                        "amiga.base3.multiply-divide": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18 example", ["470", "473"], "direct-example-use"),
                        "amiga.base3.unary-minus": ("AM-BOOK-DE-1990", "3.5 companion book Chapter 18 example", ["470"], "direct-example-use"),
                    }
                    expected = expected_sources[item["id"]]
                    self.assertEqual(item.get("confirmation-source"), expected[0])
                    self.assertEqual(item.get("confirmation-source-version"), expected[1])
                    self.assertEqual(item.get("confirmation-printed-pages"), expected[2])
                    self.assertEqual(item.get("confirmation-basis"), expected[3])
                else:
                    self.assertEqual(item["status"], "documented-in-3.0-only")

    def test_all_three_chapters_have_evidence_records(self):
        for prefix in ("amiga.base2.", "amiga.base3.", "amiga.base4."):
            with self.subTest(chapter=prefix):
                self.assertTrue(any(item["id"].startswith(prefix) for item in self.audit))

    def test_compiler_only_and_ocr_sensitive_cases_remain_qualified(self):
        by_id = {item["id"]: item for item in self.audit}
        self.assertIn("only to the compiler", by_id["amiga.base2.cint-cfloat"]["notes"])
        self.assertIn("mismatch", by_id["amiga.base2.cvx"]["notes"])
        self.assertIn("alternate spellings", by_id["amiga.base3.relational"]["notes"])
        self.assertIn("distinct", by_id["amiga.base4.integer-arithmetic-functions"]["notes"])
        self.assertIn("DPEEK / LPEEK", by_id["amiga.base2.peek"]["name"])
        self.assertIn("DPOKE x.y", by_id["amiga.base2.poke-family"]["notes"])
        self.assertIn("BYTE{x}=y", by_id["amiga.base2.typed-memory-access"]["notes"])
        self.assertIn("overlapping", by_id["amiga.base2.bmove"]["notes"])
        self.assertIn("variables are erased", by_id["amiga.base2.reserve"]["notes"])
        self.assertIn("distinct from the SWAP()", by_id["amiga.base2.swap-command"]["notes"])


class GermanBookCrossCheckTests(unittest.TestCase):

    def test_split_statement_and_operator_groups_keep_undemonstrated_forms_unconfirmed(self):
        candidates = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["candidates"]
        by_id = {item["id"]: item for item in candidates}
        expected_unconfirmed = {
            "amiga.base3.unary-plus": "Unary +",
            "amiga.base5.print-write": "PRINT AT / WRITE",
            "amiga.base8.setcolor": "SETCOLOR",
            "amiga.base8.plot-line-draw": "PLOT / LINE / DRAW forms other than DRAW TO",
        }
        for candidate_id, name in expected_unconfirmed.items():
            with self.subTest(candidate=candidate_id):
                self.assertEqual(by_id[candidate_id]["status"], "documented-in-3.0-only")
                self.assertEqual(by_id[candidate_id]["name"], name)

    def test_statement_operator_evidence_has_exact_source_pages(self):
        candidates = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["candidates"]
        by_id = {item["id"]: item for item in candidates}
        expected = {
            "amiga.base5.print": ("3.5 companion book Chapters 18–19 examples", ["458", "471", "480"]),
            "amiga.base5.locate": ("3.5 companion book Chapter 18 example", ["471"]),
            "amiga.base7.for-next": ("3.5 companion book Chapters 18–19 examples", ["463", "464", "471", "480"]),
            "amiga.base7.while-wend": ("3.5 companion book Chapter 19 example", ["480"]),
            "amiga.base8.color": ("3.5 companion book Chapter 18 example", ["471"]),
            "amiga.base8.draw-to": ("3.5 companion book Chapter 18 example", ["471"]),
        }
        for candidate_id, (version, pages) in expected.items():
            item = by_id[candidate_id]
            self.assertEqual(item["status"], "confirmed-in-3.5")
            self.assertEqual(item["confirmation-source"], "AM-BOOK-DE-1990")
            self.assertEqual(item["confirmation-source-version"], version)
            self.assertEqual(item["confirmation-printed-pages"], pages)
            self.assertEqual(item["confirmation-basis"], "direct-example-use")

    def test_explicit_3_5_families_have_independent_page_level_witness(self):
        by_id = {item["id"]: item for item in tomllib.loads(
            MANIFEST.read_text(encoding="utf-8")
        )["candidates"]}
        expected_pages = {
            "amiga.35.mat-base": ["454"],
            "amiga.35.mat-generation": ["454–455"],
            "amiga.35.mat-io": ["456–458"],
            "amiga.35.mat-copy": ["459–464"],
            "amiga.35.mat-arithmetic": ["465–470"],
            "amiga.35.mat-analysis": ["472–475"],
            "amiga.35.combinatorics": ["477–479"],
            "amiga.35.data-pointer": ["479–480"],
        }
        for candidate_id, pages in expected_pages.items():
            with self.subTest(candidate=candidate_id):
                item = by_id[candidate_id]
                self.assertEqual(item["status"], "confirmed-in-3.5")
                self.assertEqual(item["corroborating-source"], "AM-BOOK-DE-1990")
                self.assertEqual(item["corroborating-printed-pages"], pages)

    def test_german_cross_check_only_promotes_candidates_with_direct_3_5_evidence(self):
        candidates = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["candidates"]
        base_candidates = [item for item in candidates if item["id"].startswith("amiga.base")]
        self.assertTrue(base_candidates)
        self.assertTrue(all(item["source-version"] == "3.0 base manual" for item in base_candidates))
        corroborated = {item["id"]: item for item in base_candidates if "corroborating-source" in item}
        self.assertEqual(set(corroborated), {"amiga.base2.str"})
        self.assertEqual(corroborated["amiga.base2.str"]["corroborating-source"], "AM-BOOK-DE-1990")
        self.assertEqual(corroborated["amiga.base2.str"]["corroborating-printed-pages"], ["457"])
        confirmed = {item["id"] for item in base_candidates if item["status"] == "confirmed-in-3.5"}
        self.assertEqual(confirmed, {
            "amiga.base2.hex", "amiga.base2.str", "amiga.base4.rand",
            "amiga.base4.succ", "amiga.base4.string",
            "amiga.base4.rnd", "amiga.base4.int", "amiga.base4.sqr",
            "amiga.base3.numeric-add-sub", "amiga.base3.multiply-divide",
            "amiga.base3.unary-minus", "amiga.base5.print", "amiga.base5.locate",
            "amiga.base7.for-next", "amiga.base7.while-wend",
            "amiga.base8.color", "amiga.base8.draw-to",
        })


if __name__ == "__main__":
    unittest.main()
