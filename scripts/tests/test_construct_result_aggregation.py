import argparse
import json
import tempfile
import unittest
from pathlib import Path

import yaml

from scripts.aggregate_language_construct_results import aggregate, persist_result_records
from scripts.write_construct_result import build_result


class ConstructResultTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.expected = self.root / "expected-ids.txt"
        self.results = self.root / "GFATEST.OUT"
        self.screenshot = self.root / "editor-version-menu.png"
        self.ocr = self.root / "editor-version-menu-ocr.txt"
        self.output = self.root / "result.json"
        self.expected.write_text("fact-function\n", encoding="utf-8")
        self.screenshot.write_bytes(b"png")
        self.ocr.write_text("GFA-BASIC 3.52", encoding="utf-8")

    def tearDown(self):
        self.temp.cleanup()

    def writer(self, status="PASS", harness="success", version="3.52", ocr=None):
        self.results.write_text(f"GFATEST:fact-function:{status}\n", encoding="utf-8")
        if ocr is not None:
            self.ocr.write_text(ocr, encoding="utf-8")
        args = argparse.Namespace(
            construct_id="fact-function", version=version, probe="probe-3.52.LST",
            harness_outcome=harness, expected_ids=str(self.expected), results=str(self.results),
            identity_screenshot=str(self.screenshot), identity_ocr=str(self.ocr), output=str(self.output),
        )
        return build_result(args)

    def test_pass_requires_successful_harness_and_matching_identity(self):
        result = self.writer()
        self.assertEqual(result["conformance_status"], "PASS")
        self.assertEqual(result["harness_status"], "success")
        self.assertTrue(result["editor_identity"]["ocr_matched"])

    def test_fail_is_a_conformance_failure(self):
        self.assertEqual(self.writer(status="FAIL")["conformance_status"], "FAIL")

    def test_different_is_preserved(self):
        self.assertEqual(self.writer(status="DIFFERENT")["conformance_status"], "DIFFERENT")

    def test_harness_failure_never_becomes_pass(self):
        result = self.writer(harness="failure")
        self.assertEqual(result["conformance_status"], "UNTESTED")

    def test_missing_result_or_identity_is_untested(self):
        self.results.unlink(missing_ok=True)
        args = argparse.Namespace(
            construct_id="fact-function", version="3.52", probe="probe-3.52.LST",
            harness_outcome="failure", expected_ids=str(self.expected), results=str(self.results),
            identity_screenshot=str(self.root / "missing.png"), identity_ocr=str(self.ocr), output=str(self.output),
        )
        self.assertEqual(build_result(args)["conformance_status"], "UNTESTED")

    def test_identity_mismatch_is_untested(self):
        result = self.writer(ocr="GFA-BASIC 3.5")
        self.assertEqual(result["conformance_status"], "UNTESTED")

    def make_catalogue(self):
        directory = self.root / "tests/language-constructs/fact-function"
        directory.mkdir(parents=True)
        manifest = {
            "versions": {
                "3.041U": {"applicability": "unknown"},
                "3.5": {"applicability": "applicable"},
                "3.52": {"applicability": "applicable"},
            }
        }
        (directory / "manifest.yaml").write_text(yaml.safe_dump(manifest), encoding="utf-8")
        return directory

    def write_artifact(self, artifacts, version, status):
        directory = artifacts / f"amiga-construct-fact-function-{version}"
        directory.mkdir(parents=True, exist_ok=True)
        data = {
            "schema_version": 1, "construct_id": "fact-function", "version": version,
            "probe": f"probe-{version}.LST", "expected_ids": ["fact-function"], "harness_status": "success",
            "conformance_status": status, "reason": None,
            "observed": "fact-function returned the expected value",
            "outcomes": {"fact-function": status},
            "gfatest": {"fact-function": status},
            "diagnostic_refs": ["artifacts/native/GFATEST.OUT"],
            "editor_identity": {"expected_version": version, "ocr_matched": True},
        }
        (directory / "result.json").write_text(json.dumps(data), encoding="utf-8")

    def test_aggregate_reports_all_versions_and_separates_harness(self):
        self.make_catalogue()
        artifacts = self.root / "artifacts"
        self.write_artifact(artifacts, "3.041U", "DIFFERENT")
        self.write_artifact(artifacts, "3.5", "PASS")
        self.write_artifact(artifacts, "3.52", "DIFFERENT")
        selection = {
            "selection": "changed",
            "changed_constructs": ["fact-function"],
            "include": [
                {"construct_id": "fact-function", "version": "3.041U", "probe": "probe-3.041U.LST"},
                {"construct_id": "fact-function", "version": "3.5", "probe": "probe-3.5.LST"},
                {"construct_id": "fact-function", "version": "3.52", "probe": "probe-3.52.LST"},
            ],
        }
        result = aggregate(selection, self.root, artifacts, True, "")
        rows = {row["version"]: row for row in result["rows"]}
        self.assertEqual(rows["3.5"]["conformance_status"], "PASS")
        self.assertEqual(rows["3.52"]["conformance_status"], "DIFFERENT")
        self.assertEqual(rows["3.041U"]["conformance_status"], "DIFFERENT")
        self.assertEqual(rows["3.5"]["observed"], "fact-function returned the expected value")
        self.assertEqual(rows["3.5"]["outcomes"], {"fact-function": "PASS"})
        self.assertEqual(rows["3.5"]["gfatest"], {"fact-function": "PASS"})
        self.assertEqual(rows["3.5"]["diagnostic_refs"], ["artifacts/native/GFATEST.OUT"])
        self.assertEqual(result["summary_status"], "PASS")

    def test_persist_writes_immutable_run_and_latest_records(self):
        directory = self.make_catalogue()
        probe = directory / "probe-3.5.LST"
        probe.write_text("PRINT 1\\n", encoding="ascii")
        selection = {"selection": "manual", "changed_constructs": ["fact-function"], "include": [{"construct_id": "fact-function", "version": "3.5", "probe": "tests/language-constructs/fact-function/probe-3.5.LST"}]}
        artifacts = self.root / "artifacts"
        self.write_artifact(artifacts, "3.5", "PASS")
        result = aggregate(selection, self.root, artifacts, True, "")
        persist_result_records(result, selection, self.root, "12345", "2", "https://example.invalid/run/12345")
        output = directory / "results/3.5"
        self.assertTrue((output / "run-12345-attempt-2.json").is_file())
        record = json.loads((output / "latest.json").read_text(encoding="utf-8"))
        self.assertEqual(record["conformance_status"], "UNTESTED")
        self.assertEqual(record["run_attempt"], "2")
        self.assertTrue(record["manifest_sha256"])
        self.assertTrue(record["probe_sha256"])
        self.assertEqual(record["observed"], "fact-function returned the expected value")
        self.assertEqual(record["outcomes"], {"fact-function": "PASS"})
        self.assertEqual(record["gfatest"], {"fact-function": "PASS"})
        self.assertEqual(record["diagnostic_refs"], ["artifacts/native/GFATEST.OUT"])

    def test_missing_private_assets_are_untested_not_pass(self):
        self.make_catalogue()
        selection = {
            "selection": "changed",
            "changed_constructs": ["fact-function"],
            "include": [{"construct_id": "fact-function", "version": "3.5", "probe": "probe-3.5.LST"}],
        }
        result = aggregate(selection, self.root, self.root / "missing-artifacts", False, "fork PR cannot receive private assets")
        row = next(row for row in result["rows"] if row["version"] == "3.5")
        self.assertEqual(row["conformance_status"], "UNTESTED")
        self.assertIn("fork PR", row["reason"])

    def test_any_fail_makes_summary_fail(self):
        self.make_catalogue()
        artifacts = self.root / "artifacts"
        self.write_artifact(artifacts, "3.5", "FAIL")
        selection = {
            "selection": "changed",
            "changed_constructs": ["fact-function"],
            "include": [{"construct_id": "fact-function", "version": "3.5", "probe": "probe-3.5.LST"}],
        }
        result = aggregate(selection, self.root, artifacts, True, "")
        self.assertEqual(result["summary_status"], "FAIL")

    def test_empty_selection_has_no_native_results(self):
        result = aggregate({"selection": "validate-only", "changed_constructs": [], "include": []},
                           self.root, self.root / "missing", False, "not requested")
        self.assertEqual(result["summary_status"], "NO_SELECTION")
        self.assertEqual(result["rows"], [])


if __name__ == "__main__":
    unittest.main()
