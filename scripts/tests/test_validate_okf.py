import tempfile
import unittest
from pathlib import Path

from scripts.validate_okf import _heading_ids, validate_bundle


class OkfValidationTests(unittest.TestCase):
    def make_bundle(self, concept_body="# Guide\n"):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        docs = root / "docs"
        docs.mkdir()
        (docs / "index.md").write_text(
            '---\nokf_version: "0.2"\n---\n\n# Documentation\n\n- [Guide](guide.md) - Test guide.\n',
            encoding="utf-8",
        )
        (docs / "guide.md").write_text(
            "---\ntype: Guide\ntitle: Guide\n---\n\n" + concept_body,
            encoding="utf-8",
        )
        return root, docs

    def test_valid_minimal_bundle(self):
        root, _ = self.make_bundle()
        self.assertEqual(validate_bundle(root), [])

    def test_missing_frontmatter_is_rejected(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").write_text("# Guide\n", encoding="utf-8")
        errors = validate_bundle(root)
        self.assertTrue(any("missing YAML frontmatter" in item for item in errors))

    def test_malformed_yaml_is_rejected(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").write_text("---\ntype: [broken\n---\n# Guide\n", encoding="utf-8")
        errors = validate_bundle(root)
        self.assertTrue(any("invalid YAML frontmatter" in item for item in errors))

    def test_missing_or_empty_type_is_rejected(self):
        for value in ( "type: ''\n", "title: Guide\n"):
            with self.subTest(value=value):
                root, docs = self.make_bundle()
                (docs / "guide.md").write_text(f"---\n{value}---\n# Guide\n", encoding="utf-8")
                errors = validate_bundle(root)
                self.assertTrue(any("type" in item for item in errors))

    def test_root_index_requires_exact_okf_version(self):
        root, docs = self.make_bundle()
        (docs / "index.md").write_text("---\nokf_version: '0.1'\n---\n# Docs\n- [Guide](guide.md)\n")
        errors = validate_bundle(root)
        self.assertTrue(any("okf_version" in item for item in errors))

    def test_nested_index_must_not_have_frontmatter(self):
        root, docs = self.make_bundle()
        section = docs / "guides"
        section.mkdir()
        (section / "index.md").write_text("---\ntype: Index\n---\n# Guides\n- [Guide](guide.md)\n")
        (section / "guide.md").write_text("---\ntype: Guide\n---\n# Guide\n")
        (docs / "index.md").write_text('---\nokf_version: "0.2"\n---\n# Docs\n- [Guides](guides/)\n')
        errors = validate_bundle(root)
        self.assertTrue(any("nested index.md must not contain frontmatter" in item for item in errors))

    def test_index_must_list_direct_concepts(self):
        root, docs = self.make_bundle()
        (docs / "index.md").write_text('---\nokf_version: "0.2"\n---\n# Docs\n')
        errors = validate_bundle(root)
        self.assertTrue(any("index is missing direct entry" in item for item in errors))

    def test_orphaned_concepts_are_rejected(self):
        root, docs = self.make_bundle()
        (docs / "orphan.md").write_text("---\ntype: Note\n---\n# Orphan\n")
        errors = validate_bundle(root)
        self.assertTrue(any("orphaned concept" in item for item in errors))

    def test_broken_local_link_is_rejected(self):
        root, docs = self.make_bundle("[Missing](missing.md)\n")
        errors = validate_bundle(root)
        self.assertTrue(any("broken local link" in item for item in errors))

    def test_heading_ids_preserve_ascii_letters(self):
        self.assertIn("contributing-and-licensing", _heading_ids("# Contributing and licensing"))

    def test_broken_anchor_is_rejected(self):
        root, docs = self.make_bundle("[Missing heading](#missing-heading)\n")
        errors = validate_bundle(root)
        self.assertTrue(any("broken local anchor" in item for item in errors))

    def test_valid_nested_progressive_disclosure(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").unlink()
        section = docs / "guides"
        section.mkdir()
        (section / "index.md").write_text("# Guides\n\n- [Guide](guide.md) - Test guide.\n")
        (section / "guide.md").write_text("---\ntype: Guide\n---\n# Guide\n")
        (docs / "index.md").write_text(
            '---\nokf_version: "0.2"\n---\n# Documentation\n\n- [Guides](guides/) - Guide concepts.\n'
        )
        self.assertEqual(validate_bundle(root), [])

    def test_log_requires_newest_first_iso_date_headings(self):
        root, docs = self.make_bundle()
        (docs / "log.md").write_text(
            "# Update log\n\n## 2026-01-01\n- Older\n\n## 2026-02-01\n- Newer\n",
            encoding="utf-8",
        )
        errors = validate_bundle(root)
        self.assertTrue(any("newest first" in item for item in errors))

    def test_metadata_shapes_and_timestamp_offsets_are_checked(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").write_text(
            "---\ntype: Guide\nstatus: someday\ngenerated: {by: human:test, at: 2026-01-01}\n"
            "tags: [valid, 42]\n---\n# Guide\n",
            encoding="utf-8",
        )
        errors = validate_bundle(root)
        self.assertTrue(any("status" in item for item in errors))
        self.assertTrue(any("generated.at" in item for item in errors))
        self.assertTrue(any("tags" in item for item in errors))

    def test_sources_require_resource_and_unique_ids(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").write_text(
            "---\ntype: Guide\nsources:\n  - {id: book, resource: https://example.com/book}\n"
            "  - {id: book, resource: ''}\n---\n# Guide\n",
            encoding="utf-8",
        )
        errors = validate_bundle(root)
        self.assertTrue(any("duplicate source id" in item for item in errors))
        self.assertTrue(any("resource" in item for item in errors))

    def test_attested_computation_requires_runtime(self):
        root, docs = self.make_bundle()
        (docs / "guide.md").write_text("---\ntype: Attested Computation\n---\n# Guide\n")
        errors = validate_bundle(root)
        self.assertTrue(any("requires a non-empty 'runtime'" in item for item in errors))


if __name__ == "__main__":
    unittest.main()
