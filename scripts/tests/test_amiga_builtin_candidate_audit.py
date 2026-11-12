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
                    expected_source = "AM-BOOK-DE-1990" if item["id"] in {"amiga.base4.rnd", "amiga.base4.int", "amiga.base4.sqr"} else "AM-INTERPRETER-35"
                    expected_version = "3.5 companion book Chapter 18" if expected_source == "AM-BOOK-DE-1990" else "3.5 addendum"
                    self.assertEqual(item.get("confirmation-source"), expected_source)
                    self.assertEqual(item.get("confirmation-source-version"), expected_version)
                    expected_basis = "direct-documentation-reference" if item["id"] == "amiga.base2.str" else "direct-example-use"
                    self.assertEqual(item.get("confirmation-basis"), expected_basis)
                    expected_pages = {"amiga.base2.hex": ["13-36"], "amiga.base2.str": ["13-3"], "amiga.base4.rand": ["13-2"], "amiga.base4.succ": ["13-5"], "amiga.base4.string": ["13-21"], "amiga.base4.rnd": ["458"], "amiga.base4.int": ["458"], "amiga.base4.sqr": ["473"], "amiga.base4.inc-dec": ["13-35"]}
                    self.assertEqual(item.get("confirmation-printed-pages"), expected_pages[item["id"]])
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


class StatementEvidenceTests(unittest.TestCase):
    def test_3_5_addendum_statement_evidence_is_exact_and_scoped(self):
        candidates = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["candidates"]
        by_id = {item["id"]: item for item in candidates}
        expected = {
            "amiga.base5.data-read": (["13-35"], "direct-example-use"),
            "amiga.base5.print-write": (["13-2", "13-34", "13-35", "13-36"], "direct-example-use"),
            "amiga.base7.for-next": (["13-35"], "direct-example-use"),
            "amiga.base7.do-loop": (["13-35"], "direct-example-use"),
            "amiga.base7.on-error": (["13-36"], "direct-documentation-reference"),
            "amiga.base7.resume": (["13-36"], "direct-documentation-restriction"),
        }
        for candidate_id, (pages, basis) in expected.items():
            with self.subTest(candidate=candidate_id):
                item = by_id[candidate_id]
                self.assertEqual(item["status"], "confirmed-in-3.5")
                self.assertEqual(item["confirmation-source"], "AM-INTERPRETER-35")
                self.assertEqual(item["confirmation-source-version"], "3.5 addendum")
                self.assertEqual(item["confirmation-printed-pages"], pages)
                self.assertEqual(item["confirmation-basis"], basis)
        for candidate_id in (
            "amiga.base5.print-variants",
            "amiga.base7.for-next-endfor",
            "amiga.base7.do-loop-variants",
            "amiga.base7.on-error-reset",
            "amiga.base7.resume-plain",
        ):
            with self.subTest(unconfirmed=candidate_id):
                self.assertEqual(by_id[candidate_id]["status"], "documented-in-3.0-only")


class GermanBookCrossCheckTests(unittest.TestCase):
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

    def test_cross_manual_source_evidence_is_pinned_for_base_candidates(self):
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
            "amiga.base4.inc-dec", "amiga.base5.data-read", "amiga.base5.print-write",
            "amiga.base7.for-next", "amiga.base7.do-loop", "amiga.base7.on-error",
            "amiga.base7.resume",
        })


if __name__ == "__main__":
    unittest.main()
