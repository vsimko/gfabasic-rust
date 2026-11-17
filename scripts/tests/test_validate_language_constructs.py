import tempfile
import unittest
from pathlib import Path

import yaml

from scripts.validate_language_constructs import validate


class LanguageConstructValidationTests(unittest.TestCase):
    def make_root(self):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        catalogue = root / "tests/language-constructs/example"
        catalogue.mkdir(parents=True)
        (root / "docs").mkdir()
        (root / "docs/reference-sources.md").write_text("| ID | Title |\n|---|---|\n| TEST-SOURCE | Example |\n")
        versions = {}
        for version in ("3.041U", "3.5", "3.52"):
            versions[version] = {
                "applicability": "unknown",
                "probe": "probe.LST" if version == "3.52" else None,
                "expectation": {"status": "unknown", "summary": "Not established."},
                "observed": {"result": "UNTESTED", "artifact": None, "run_id": None},
            }
        manifest = {
            "schema_version": 1, "id": "example", "name": "Example construct",
            "kind": "statement", "classification": "language_core",
            "evidence": {"status": "unknown", "sources": []},
            "versions": versions,
            "implementation": {"status": "unknown", "notes": ""},
        }
        (catalogue / "manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        (catalogue / "probe.LST").write_text("END\n")
        return temp, root, catalogue, manifest

    def test_valid_manifest_passes(self):
        temp, root, _, _ = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate(root), [])

    def test_duplicate_yaml_keys_fail(self):
        temp, root, catalogue, _ = self.make_root()
        self.addCleanup(temp.cleanup)
        (catalogue / "manifest.yaml").write_text("schema_version: 1\nschema_version: 1\n")
        self.assertTrue(any("duplicate key" in item for item in validate(root)))

    def test_manifest_root_must_be_mapping(self):
        temp, root, catalogue, _ = self.make_root()
        self.addCleanup(temp.cleanup)
        (catalogue / "manifest.yaml").write_text("- item\n")
        self.assertTrue(any("YAML mapping" in item for item in validate(root)))

    def test_exact_version_set_is_required(self):
        temp, root, catalogue, manifest = self.make_root()
        self.addCleanup(temp.cleanup)
        del manifest["versions"]["3.5"]
        (catalogue / "manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        self.assertTrue(any("versions must contain exactly" in item for item in validate(root)))

    def test_unknown_source_key_fails(self):
        temp, root, catalogue, manifest = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest["evidence"] = {"status": "documented", "sources": [{
            "source_key": "MISSING", "locator": "p. 1", "claim": "Syntax",
            "version_scope": ["3.52"],
        }]}
        (catalogue / "manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        self.assertTrue(any("unknown source_key" in item for item in validate(root)))

    def test_probe_path_cannot_escape_construct_directory(self):
        temp, root, catalogue, manifest = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest["versions"]["3.52"]["probe"] = "../outside.LST"
        (catalogue / "manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        self.assertTrue(any("unsafe or missing probe path" in item for item in validate(root)))

    def test_unreferenced_probe_fails(self):
        temp, root, catalogue, manifest = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest["versions"]["3.52"]["probe"] = None
        (catalogue / "manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        self.assertTrue(any("not referenced" in item for item in validate(root)))


if __name__ == "__main__":
    unittest.main()
