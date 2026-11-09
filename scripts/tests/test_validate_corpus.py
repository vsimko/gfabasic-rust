import tempfile
import unittest
from pathlib import Path

from scripts.validate_corpus import validate_corpus


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
        (root / "tests/corpus/manifest.toml").write_text(
            'schema_version = 1\ncorpus_status = "research_only"\n'
            'description = "Test corpus."\n'
            'fixtures = ["amiga/3.0/control-flow/fixture-one"]\n'
        )
        (corpus / "fixture-one.lst").write_text("PRINT 1\n")
        (corpus / "fixture-one.toml").write_text(
            'id = "fixture-one"\n'
            'dialect = "GFA BASIC Amiga"\nversion = "3.0"\napi_profiles = []\n'
            'kind = "semantic"\nstatus = "candidate"\nsource_key = "TEST-SOURCE"\n'
            'source_pages = "printed p. 1"\nevidence_status = "reviewed"\n'
            'expectation = "Prints one."\nexpected_stdout = "1"\nexpected_exit = "success"\n'
            'notes = ["Independently authored."]\n'
        )
        return temp, root

    def make_symlink(self, link, target, *, target_is_directory=False):
        try:
            link.symlink_to(target, target_is_directory=target_is_directory)
        except (OSError, NotImplementedError) as exc:
            self.skipTest(f"symbolic links are unavailable: {exc}")

    def test_invalid_utf8_manifest_reports_error_without_crashing(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest = root / "tests/corpus/manifest.toml"
        manifest.write_bytes(b"\xff")

        errors = validate_corpus(root)
        self.assertTrue(any("manifest.toml: invalid UTF-8" in item for item in errors))

    def test_invalid_utf8_fixture_metadata_reports_error_without_crashing(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        metadata.write_bytes(b"\xff")

        errors = validate_corpus(root)
        self.assertTrue(any("fixture-one.toml: invalid UTF-8" in item for item in errors))

    def test_invalid_utf8_source_register_reports_error_without_crashing(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        source_register = root / "docs/reference-sources.md"
        source_register.write_bytes(b"\xff")

        errors = validate_corpus(root)
        self.assertTrue(any("docs/reference-sources.md: invalid UTF-8" in item for item in errors))

    def test_valid_corpus_passes(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_corpus(root), [])

    def test_non_string_kind_and_status_report_errors_without_crashing(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        original = metadata.read_text()
        invalid_values = (
            ('kind = "semantic"', "kind = []", "kind must be one of"),
            ('status = "candidate"', 'status = { value = "candidate" }', "status must be one of"),
        )
        for valid_value, invalid_value, expected_error in invalid_values:
            with self.subTest(value=invalid_value):
                metadata.write_text(original.replace(valid_value, invalid_value))
                errors = validate_corpus(root)
                self.assertTrue(any(expected_error in item for item in errors))

    def test_missing_companion_file_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/amiga/3.0/control-flow/fixture-one.lst").unlink()
        self.assertTrue(any("missing source file" in item for item in validate_corpus(root)))

    def test_symlinked_source_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        source = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.lst"
        outside = root / "outside.lst"
        outside.write_text("PRINT 99\\n")
        source.unlink()
        self.make_symlink(source, outside)

        errors = validate_corpus(root)
        self.assertTrue(any("source path must not traverse symlinks" in item for item in errors))

    def test_symlinked_metadata_is_rejected_without_reading_target(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        outside = root / "outside.toml"
        outside.write_text("this is not valid TOML = [")
        metadata.unlink()
        self.make_symlink(metadata, outside)

        errors = validate_corpus(root)
        self.assertTrue(any("metadata path must not traverse symlinks" in item for item in errors))
        self.assertFalse(any("invalid TOML" in item for item in errors))

    def test_symlinked_parent_directory_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        corpus_dir = root / "tests/corpus/amiga/3.0/control-flow"
        outside = root / "outside-corpus"
        corpus_dir.rename(outside)
        self.make_symlink(corpus_dir, outside, target_is_directory=True)

        errors = validate_corpus(root)
        self.assertTrue(any("source path must not traverse symlinks" in item for item in errors))
        self.assertTrue(any("metadata path must not traverse symlinks" in item for item in errors))

    def test_symlinked_corpus_root_is_rejected_before_reading_manifest(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        corpus_root = root / "tests/corpus"
        outside = root / "outside-corpus"
        corpus_root.rename(outside)
        self.make_symlink(corpus_root, outside, target_is_directory=True)

        self.assertEqual(
            validate_corpus(root),
            ["tests/corpus/manifest.toml: path must not traverse symlinks"],
        )

    def test_symlinked_tests_parent_is_rejected_before_reading_manifest(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        tests_root = root / "tests"
        outside = root / "outside-tests"
        tests_root.rename(outside)
        self.make_symlink(tests_root, outside, target_is_directory=True)

        self.assertEqual(
            validate_corpus(root),
            ["tests/corpus/manifest.toml: path must not traverse symlinks"],
        )

    def test_broken_source_symlink_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        source = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.lst"
        source.unlink()
        self.make_symlink(source, root / "missing.lst")

        errors = validate_corpus(root)
        self.assertTrue(any("source path must not traverse symlinks" in item for item in errors))

    def test_duplicate_fixture_id_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest = root / "tests/corpus/manifest.toml"
        manifest.write_text(
            manifest.read_text().replace(
                'fixtures = ["amiga/3.0/control-flow/fixture-one"]',
                'fixtures = ["amiga/3.0/control-flow/fixture-one", "amiga/3.0/control-flow/fixture-two"]',
            )
        )
        fixture = root / "tests/corpus/amiga/3.0/control-flow"
        (fixture / "fixture-two.lst").write_text("PRINT 2\n")
        (fixture / "fixture-two.toml").write_text((fixture / "fixture-one.toml").read_text())
        self.assertTrue(any("duplicate id" in item for item in validate_corpus(root)))

    def test_dialect_directory_must_match_metadata(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        metadata.write_text(metadata.read_text().replace("GFA BASIC Amiga", "GFA BASIC Atari ST"))
        self.assertTrue(any("does not match directory 'amiga'" in item for item in validate_corpus(root)))

    def test_version_directory_must_match_metadata(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        metadata.write_text(metadata.read_text().replace('version = "3.0"', 'version = "3.5"'))
        self.assertTrue(any("does not match directory '3.0'" in item for item in validate_corpus(root)))

    def test_fixture_path_requires_canonical_four_components(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        manifest = root / "tests/corpus/manifest.toml"
        original = manifest.read_text()
        valid_path = "amiga/3.0/control-flow/fixture-one"
        for invalid_path in (
            "amiga/3.0/fixture-one",
            "amiga/3.0/control-flow/nested/fixture-one",
            "amiga//3.0/control-flow/fixture-one",
            "amiga/./3.0/control-flow/fixture-one",
            "amiga/3.0/control-flow/fixture-one/",
            "amiga/3.0/control-flow/fixture\\\\one",
        ):
            with self.subTest(path=invalid_path):
                manifest.write_text(original.replace(valid_path, invalid_path))
                self.assertTrue(any("invalid fixture path" in item for item in validate_corpus(root)))

    def test_missing_source_register_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "docs/reference-sources.md").unlink()
        self.assertIn("missing docs/reference-sources.md", validate_corpus(root))

    def test_unknown_source_key_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        metadata.write_text(metadata.read_text().replace("TEST-SOURCE", "MISSING-SOURCE"))
        self.assertTrue(any("unknown source_key" in item for item in validate_corpus(root)))

    def test_source_register_heading_is_not_a_valid_source_key(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        metadata = root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml"
        metadata.write_text(metadata.read_text().replace('source_key = "TEST-SOURCE"', 'source_key = "ID"'))
        self.assertTrue(any("unknown source_key 'ID'" in item for item in validate_corpus(root)))

    def test_unlisted_fixture_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        fixture = root / "tests/corpus/amiga/3.0/control-flow"
        (fixture / "unlisted.lst").write_text("PRINT 3\n")
        self.assertTrue(any("not listed" in item for item in validate_corpus(root)))


if __name__ == "__main__":
    unittest.main()
