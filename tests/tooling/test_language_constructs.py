import tempfile
import unittest
from pathlib import Path

import yaml

from scripts.select_language_constructs import select_jobs, validate_catalogue


class LanguageConstructSelectionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.catalogue = self.root / "tests/language-constructs"
        self.catalogue.mkdir(parents=True)
        source_schema = Path(__file__).resolve().parents[2] / "tests/language-constructs/manifest.schema.json"
        (self.catalogue / "manifest.schema.json").write_text(source_schema.read_text(encoding="utf-8"), encoding="utf-8")
        self.construct("alpha", {"3.5": "probe-3.5.LST"})

    def tearDown(self):
        self.temp.cleanup()

    def construct(self, name, applicable):
        versions = {}
        for version in ("3.041U", "3.5", "3.52"):
            probe = applicable.get(version)
            versions[version] = {
                "applicability": "applicable" if probe else "unknown",
                "expectation": {"status": "unknown", "summary": "Not established.", "evidence_refs": []},
                "probe": probe,
                "execution": {"status": "UNTESTED", "observed": None, "run_url": None,
                              "probe_source": None, "evidence_refs": []},
                "implementation": {"status": "unknown", "refs": []},
            }
        data = {
            "manifest_schema_version": 1,
            "construct": {"id": name, "kind": "statement", "name": name, "summary": "Test construct.",
                          "syntax_forms": [{"text": name, "versions": list(applicable) or ["3.5"]}]},
            "evidence": [],
            "versions": versions,
        }
        directory = self.catalogue / name
        directory.mkdir(exist_ok=True)
        (directory / "manifest.yaml").write_text(yaml.safe_dump(data, sort_keys=False), encoding="utf-8")
        for probe in applicable.values():
            (directory / probe).write_text("END\n", encoding="utf-8")

    def test_changed_construct_selects_only_applicable_version(self):
        result = select_jobs(["tests/language-constructs/alpha/manifest.yaml"], self.root)
        self.assertEqual(result["include"], [
            {"construct": "alpha", "version": "3.5", "probe": "probe-3.5.LST"}
        ])
        self.assertFalse(result["validate_all"])

    def test_unrelated_construct_is_not_selected(self):
        self.construct("beta", {"3.52": "probe-3.52.LST"})
        result = select_jobs(["tests/language-constructs/alpha/probe-3.5.LST"], self.root)
        self.assertEqual([job["construct"] for job in result["include"]], ["alpha"])

    def test_schema_or_validator_change_validates_all_without_native_jobs(self):
        for path in ("tests/language-constructs/manifest.schema.json", "scripts/select_language_constructs.py"):
            result = select_jobs([path], self.root)
            self.assertTrue(result["validate_all"])
            self.assertEqual(result["include"], [])

    def test_deleted_construct_is_not_selected(self):
        result = select_jobs(["tests/language-constructs/removed/manifest.yaml"], self.root)
        self.assertEqual(result["include"], [])

    def test_explicit_all_selects_all_applicable_versions(self):
        self.construct("beta", {"3.52": "probe-3.52.LST"})
        result = select_jobs([], self.root, all_constructs=True)
        self.assertEqual(
            [(item["construct"], item["version"]) for item in result["include"]],
            [("alpha", "3.5"), ("beta", "3.52")],
        )

    def test_path_traversal_is_rejected(self):
        with self.assertRaises(ValueError):
            select_jobs(["tests/language-constructs/../secret"], self.root)

    def test_duplicate_yaml_keys_fail_validation(self):
        manifest = self.catalogue / "alpha/manifest.yaml"
        manifest.write_text("manifest_schema_version: 1\nmanifest_schema_version: 1\n", encoding="utf-8")
        self.assertTrue(any("duplicate key" in error for error in validate_catalogue(self.root)))


    def test_renamed_construct_selects_only_new_directory(self):
        renamed = self.catalogue / "beta"
        (self.catalogue / "alpha").rename(renamed)
        manifest = renamed / "manifest.yaml"
        data = yaml.safe_load(manifest.read_text(encoding="utf-8"))
        data["construct"]["id"] = "beta"
        manifest.write_text(yaml.safe_dump(data, sort_keys=False), encoding="utf-8")
        result = select_jobs([
            "tests/language-constructs/alpha/manifest.yaml",
            "tests/language-constructs/beta/manifest.yaml",
        ], self.root)
        self.assertEqual([job["construct"] for job in result["include"]], ["beta"])

    def test_malformed_yaml_fails_validation(self):
        (self.catalogue / "alpha/manifest.yaml").write_text("construct: [unterminated", encoding="utf-8")
        self.assertTrue(any("invalid YAML" in error for error in validate_catalogue(self.root)))

    def test_missing_probe_fails_validation(self):
        (self.catalogue / "alpha/probe-3.5.LST").unlink()
        self.assertTrue(any("missing or unsafe probe" in error for error in validate_catalogue(self.root)))

    def test_unknown_version_key_fails_schema_validation(self):
        manifest = self.catalogue / "alpha/manifest.yaml"
        data = yaml.safe_load(manifest.read_text(encoding="utf-8"))
        data["versions"]["3.53"] = data["versions"]["3.5"]
        manifest.write_text(yaml.safe_dump(data, sort_keys=False), encoding="utf-8")
        self.assertTrue(any("3.53" in error for error in validate_catalogue(self.root)))

    def test_duplicate_construct_ids_fail_validation(self):
        self.construct("beta", {"3.52": "probe-3.52.LST"})
        manifest = self.catalogue / "beta/manifest.yaml"
        data = yaml.safe_load(manifest.read_text(encoding="utf-8"))
        data["construct"]["id"] = "alpha"
        manifest.write_text(yaml.safe_dump(data, sort_keys=False), encoding="utf-8")
        errors = validate_catalogue(self.root)
        self.assertTrue(any("duplicate construct ID" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
