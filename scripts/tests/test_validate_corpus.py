import tempfile
import unittest
from pathlib import Path

import yaml

from scripts.validate_corpus import manifest_summary, validate_corpus


class CorpusValidationTests(unittest.TestCase):
    def make_root(self):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        corpus = root / "tests/corpus/amiga/3.0/control-flow"
        corpus.mkdir(parents=True)
        (root / "docs").mkdir()
        (root / "docs/reference-sources.md").write_text(
            "| ID | Title |\n|---|---|\n| TEST-SOURCE | Test source |\n"
        )
        manifest = {
            "schema_version": 1,
            "corpus_status": "research_only",
            "description": "Test corpus.",
            "fixtures": ["amiga/3.0/control-flow/fixture-one"],
        }
        (root / "tests/corpus/manifest.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False))
        metadata = {
            "id": "fixture-one", "dialect": "GFA BASIC Amiga", "version": "3.0",
            "api_profiles": [], "kind": "semantic", "status": "candidate",
            "source_key": "TEST-SOURCE", "source_pages": "printed p. 1",
            "evidence_status": "reviewed", "expectation": "Prints one.",
            "expected_stdout": "1", "expected_exit": "success",
            "notes": ["Independently authored."],
        }
        (corpus / "fixture-one.lst").write_text("PRINT 1\n")
        (corpus / "fixture-one.yaml").write_text(yaml.safe_dump(metadata, sort_keys=False))
        summary = manifest_summary(root / "tests/corpus", manifest["fixtures"])
        (root / "tests/corpus/README.md").write_text(
            "<!-- fixture-summary:start -->\n" + summary + "\n<!-- fixture-summary:end -->\n"
        )
        return temp, root

    def make_symlink(self, link, target, *, target_is_directory=False):
        try:
            link.symlink_to(target, target_is_directory=target_is_directory)
        except (OSError, NotImplementedError) as exc:
            self.skipTest(f"symbolic links are unavailable: {exc}")

    def test_valid_corpus_passes(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_corpus(root), [])

    def test_invalid_utf8_manifest_reports_error(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/manifest.yaml").write_bytes(b"\xff")
        self.assertTrue(any("manifest.yaml: invalid UTF-8" in e for e in validate_corpus(root)))

    def test_invalid_utf8_metadata_reports_error(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/amiga/3.0/control-flow/fixture-one.yaml").write_bytes(b"\xff")
        self.assertTrue(any("fixture-one.yaml: invalid UTF-8" in e for e in validate_corpus(root)))

    def test_duplicate_yaml_mapping_key_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus/manifest.yaml"
        path.write_text("schema_version: 1\nschema_version: 1\n")
        self.assertTrue(any("invalid YAML" in e and "duplicate key" in e for e in validate_corpus(root)))

    def test_manifest_must_be_mapping(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/manifest.yaml").write_text("- item\n")
        self.assertTrue(any("manifest must be a YAML mapping" in e for e in validate_corpus(root)))

    def test_non_string_kind_and_status_report_errors(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.yaml"
        metadata = yaml.safe_load(path.read_text())
        metadata["kind"] = []
        metadata["status"] = {"value": "candidate"}
        path.write_text(yaml.safe_dump(metadata, sort_keys=False))
        errors = validate_corpus(root)
        self.assertTrue(any("kind must be one of" in e for e in errors))
        self.assertTrue(any("status must be one of" in e for e in errors))

    def test_missing_companion_file_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/amiga/3.0/control-flow/fixture-one.lst").unlink()
        self.assertTrue(any("missing source file" in e for e in validate_corpus(root)))

    def test_symlinked_source_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        source = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.lst"
        outside = root / "outside.lst"
        outside.write_text("PRINT 99\n")
        source.unlink()
        self.make_symlink(source, outside)
        self.assertTrue(any("source path must not traverse symlinks" in e for e in validate_corpus(root)))

    def test_symlinked_metadata_is_rejected_without_reading_target(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.yaml"
        outside = root / "outside.yaml"
        outside.write_text("not: [valid")
        metadata.unlink()
        self.make_symlink(metadata, outside)
        errors = validate_corpus(root)
        self.assertTrue(any("metadata path must not traverse symlinks" in e for e in errors))
        self.assertFalse(any("invalid YAML" in e for e in errors))

    def test_symlinked_parent_directory_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus/amiga/3.0/control-flow"
        outside = root / "outside-corpus"
        path.rename(outside)
        self.make_symlink(path, outside, target_is_directory=True)
        errors = validate_corpus(root)
        self.assertTrue(any("source path must not traverse symlinks" in e for e in errors))
        self.assertTrue(any("metadata path must not traverse symlinks" in e for e in errors))

    def test_symlinked_corpus_root_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus"
        outside = root / "outside-corpus"
        path.rename(outside)
        self.make_symlink(path, outside, target_is_directory=True)
        self.assertEqual(validate_corpus(root), ["tests/corpus/manifest.yaml: path must not traverse symlinks"])

    def test_duplicate_fixture_id_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest_path = root / "tests/corpus/manifest.yaml"
        manifest = yaml.safe_load(manifest_path.read_text())
        manifest["fixtures"].append("amiga/3.0/control-flow/fixture-two")
        manifest_path.write_text(yaml.safe_dump(manifest, sort_keys=False))
        fixture = root / "tests/corpus/amiga/3.0/control-flow"
        (fixture / "fixture-two.lst").write_text("PRINT 2\n")
        (fixture / "fixture-two.yaml").write_text((fixture / "fixture-one.yaml").read_text())
        self.assertTrue(any("duplicate id" in e for e in validate_corpus(root)))

    def test_dialect_and_version_must_match_directory(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.yaml"
        metadata = yaml.safe_load(path.read_text())
        metadata["dialect"] = "GFA BASIC Atari ST"
        metadata["version"] = "3.5"
        path.write_text(yaml.safe_dump(metadata, sort_keys=False))
        errors = validate_corpus(root)
        self.assertTrue(any("does not match directory 'amiga'" in e for e in errors))
        self.assertTrue(any("does not match directory '3.0'" in e for e in errors))

    def test_fixture_path_requires_canonical_four_components(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        path = root / "tests/corpus/manifest.yaml"
        original = yaml.safe_load(path.read_text())
        for invalid in ("amiga/3.0/fixture-one", "amiga/3.0/control-flow/nested/fixture-one",
                        "amiga//3.0/control-flow/fixture-one", "amiga/./3.0/control-flow/fixture-one",
                        "amiga/3.0/control-flow/fixture-one/", "amiga/3.0/control-flow/fixture\\\\one"):
            with self.subTest(path=invalid):
                manifest = dict(original, fixtures=[invalid])
                path.write_text(yaml.safe_dump(manifest, sort_keys=False))
                self.assertTrue(any("invalid fixture path" in e for e in validate_corpus(root)))

    def test_missing_and_unknown_source_register_entries_fail(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        register = root / "docs/reference-sources.md"
        register.unlink()
        self.assertIn("missing docs/reference-sources.md", validate_corpus(root))
        register.write_text("| ID | Title |\n|---|---|\n| TEST-SOURCE | Test source |\n")
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.yaml"
        data = yaml.safe_load(metadata.read_text())
        data["source_key"] = "MISSING-SOURCE"
        metadata.write_text(yaml.safe_dump(data, sort_keys=False))
        self.assertTrue(any("unknown source_key" in e for e in validate_corpus(root)))

    def test_unlisted_fixture_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/amiga/3.0/control-flow/unlisted.lst").write_text("PRINT 3\n")
        self.assertTrue(any("not listed" in e for e in validate_corpus(root)))

    def test_manifest_summary_matches_sidecars(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_corpus(root), [])

    def test_stale_manifest_summary_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/README.md").write_text(
            "<!-- fixture-summary:start -->\nManifest summary: 99 fixtures; 99 total.\n<!-- fixture-summary:end -->\n"
        )
        self.assertTrue(any("manifest-derived fixture summary is missing or stale" in e for e in validate_corpus(root)))


if __name__ == "__main__":
    unittest.main()
