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
        (corpus / "fixture-one.bas").write_text("PRINT 1\n")
        (corpus / "fixture-one.toml").write_text(
            'id = "fixture-one"\n'
            'dialect = "GFA BASIC Amiga"\nversion = "3.0"\napi_profiles = []\n'
            'kind = "semantic"\nstatus = "candidate"\nsource_key = "TEST-SOURCE"\n'
            'source_pages = "printed p. 1"\nevidence_status = "reviewed"\n'
            'expectation = "Prints one."\nexpected_stdout = "1"\nexpected_exit = "success"\n'
            'notes = ["Independently authored."]\n'
        )
        return temp, root

    def test_valid_corpus_passes(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_corpus(root), [])

    def test_missing_companion_file_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/amiga/3.0/control-flow/fixture-one.bas").unlink()
        self.assertTrue(any("missing source file" in item for item in validate_corpus(root)))

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
        (fixture / "fixture-two.bas").write_text("PRINT 2\n")
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

    def test_unlisted_fixture_fails(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        fixture = root / "tests/corpus/amiga/3.0/control-flow"
        (fixture / "unlisted.bas").write_text("PRINT 3\n")
        self.assertTrue(any("not listed" in item for item in validate_corpus(root)))


if __name__ == "__main__":
    unittest.main()
