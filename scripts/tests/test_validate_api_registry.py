import json
import tempfile
import unittest
from pathlib import Path

from scripts.validate_api_registry import validate_registry


VALID_SCHEMA = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "required": ["schema-version", "declarations"],
    "properties": {
        "schema-version": {"const": 1},
        "declarations": {"type": "array", "items": {"type": "object"}},
    },
}


class RegistryValidationTests(unittest.TestCase):
    def make_root(self, declarations):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        (root / "api/schema").mkdir(parents=True)
        (root / "api/registry/atari-st").mkdir(parents=True)
        (root / "docs").mkdir()
        (root / "tests/corpus").mkdir(parents=True)
        (root / "api/schema/registry-v1.json").write_text(json.dumps(VALID_SCHEMA))
        (root / "docs/reference-sources.md").write_text("| ID | Title |\n|---|---|\n| TEST-SOURCE | Test source |\n")
        (root / "tests/corpus/manifest.toml").write_text('fixtures = ["fixture-one"]\n')
        (root / "docs/reference.md").write_text("# Timer\n")
        (root / "api/registry/atari-st/time.toml").write_text(declarations)
        return temp, root

    def valid_declaration(self):
        return '''schema-version = 1

[[declarations]]
id = "gfa.atari-st.system.timer"
name = "TIMER"
kind = "variable"

[[declarations.availability]]
dialect = "atari-st-gfa-basic"
versions = ["3.0"]
profile = "atari-st"

[[declarations.syntax]]
form = "variable-reference"
contexts = ["expression"]

[declarations.signature]
type = "integer"

[declarations.semantics]
summary = "A reserved counter."

[declarations.evidence]
status = "book-documented"
sources = ["TEST-SOURCE"]
printed-pages = ["printed p. 1"]

[declarations.implementation]
status = "not-implemented"

[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unsupported"
conformance = "not-tested"
tests = []
pending-reason = "No implementation or conformance test exists."

[declarations.docs]
path = "docs/reference.md"
anchors = []
'''

    def test_source_register_heading_is_not_a_valid_source_id(self):
        content = self.valid_declaration().replace(
            'sources = ["TEST-SOURCE"]', 'sources = ["ID"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("unknown source ID 'ID'" in error for error in errors))

    def test_invalid_utf8_source_register_reports_error(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        (root / "docs/reference-sources.md").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("reference-sources.md: invalid UTF-8" in error for error in errors))

    def test_invalid_utf8_schema_reports_error(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        (root / "api/schema/registry-v1.json").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("registry-v1.json: invalid UTF-8" in error for error in errors))

    def test_invalid_utf8_registry_file_reports_error(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        (root / "api/registry/atari-st/time.toml").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("time.toml: invalid UTF-8" in error for error in errors))

    def test_invalid_utf8_documentation_file_reports_error(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        (root / "docs/reference.md").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("documentation file has invalid UTF-8" in error for error in errors))

    def test_invalid_utf8_corpus_manifest_reports_error(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        (root / "tests/corpus/manifest.toml").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("manifest.toml: invalid UTF-8" in error for error in errors))

    def test_invalid_utf8_corpus_sidecar_reports_error(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["fixture-one-id"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.add_manifest_fixture(root, "fixture-one-id")
        (root / "tests/corpus/amiga/3.0/control-flow/fixture-one.toml").write_bytes(b"\xff")

        errors = validate_registry(root)
        self.assertTrue(any("fixture-one.toml: invalid UTF-8" in error for error in errors))

    def test_research_entry_is_valid(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_registry(root), [])

    def test_schema_invalid_declaration_shape_reports_error_without_crashing(self):
        content = 'schema-version = 1\ndeclarations = ["not-a-table"]\n'
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("not of type 'object'" in error for error in errors))

    def test_malformed_corpus_fixture_entries_do_not_crash_registry_validation(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        manifest = root / "tests/corpus/manifest.toml"
        manifest.write_text('fixtures = [["nested"]]\n')
        self.assertEqual(validate_registry(root), [])

    def add_manifest_fixture(self, root, fixture_id):
        corpus = root / "tests/corpus/amiga/3.0/control-flow"
        corpus.mkdir(parents=True, exist_ok=True)
        (corpus / "fixture-one.toml").write_text(f'id = "{fixture_id}"\n')
        (root / "tests/corpus/manifest.toml").write_text(
            'fixtures = ["amiga/3.0/control-flow/fixture-one"]\n'
        )

    def test_registry_test_references_use_stable_fixture_ids(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["fixture-one-id"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.add_manifest_fixture(root, "fixture-one-id")
        self.assertEqual(validate_registry(root), [])

    def test_registry_test_references_do_not_use_manifest_paths_as_ids(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["amiga/3.0/control-flow/fixture-one"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.add_manifest_fixture(root, "fixture-one-id")
        errors = validate_registry(root)
        self.assertTrue(any("unknown test ID 'amiga/3.0/control-flow/fixture-one'" in error for error in errors))

    def test_manifest_paths_with_backslashes_are_not_registry_test_references(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["fixture-one-id"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        corpus = root / "tests/corpus/amiga/3.0/control-flow"
        corpus.mkdir(parents=True, exist_ok=True)
        (corpus / "fixture\\one.toml").write_text('id = "fixture-one-id"\n')
        manifest = root / "tests/corpus/manifest.toml"
        manifest.write_text('fixtures = ["amiga/3.0/control-flow/fixture\\one"]\n')
        errors = validate_registry(root)
        self.assertTrue(any("unknown test ID 'fixture-one-id'" in error for error in errors))

    def test_symlinked_sidecar_is_not_a_registry_test_reference(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["fixture-one-id"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.add_manifest_fixture(root, "fixture-one-id")
        corpus = root / "tests/corpus/amiga/3.0/control-flow"
        metadata = corpus / "fixture-one.toml"
        target = corpus / "alternate-sidecar.toml"
        target.write_text(metadata.read_text())
        metadata.unlink()
        try:
            metadata.symlink_to(target)
        except (OSError, NotImplementedError) as exc:
            self.skipTest(f"symbolic links are unavailable: {exc}")

        errors = validate_registry(root)
        self.assertTrue(any("unknown test ID 'fixture-one-id'" in error for error in errors))

    def test_unlisted_sidecar_ids_are_not_registry_test_references(self):
        content = self.valid_declaration().replace(
            "tests = []", 'tests = ["unlisted-fixture-id"]'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.add_manifest_fixture(root, "listed-fixture-id")
        unlisted = root / "tests/corpus/amiga/3.0/control-flow/unlisted.toml"
        unlisted.write_text('id = "unlisted-fixture-id"\n')
        errors = validate_registry(root)
        self.assertTrue(any("unknown test ID 'unlisted-fixture-id'" in error for error in errors))

    def test_absolute_documentation_path_is_rejected(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        outside = root.parent / f"{root.name}-outside.md"
        outside.write_text("# Timer\\n")
        self.addCleanup(lambda: outside.unlink(missing_ok=True))
        registry = root / "api/registry/atari-st/time.toml"
        registry.write_text(
            registry.read_text().replace('path = "docs/reference.md"', f'path = "{outside}"')
        )

        errors = validate_registry(root)
        self.assertTrue(any("canonical repository-relative path" in error for error in errors))

    def test_parent_traversal_documentation_path_is_rejected(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        outside = root.parent / f"{root.name}-outside.md"
        outside.write_text("# Timer\\n")
        self.addCleanup(lambda: outside.unlink(missing_ok=True))
        registry = root / "api/registry/atari-st/time.toml"
        relative = f"../{outside.name}"
        registry.write_text(
            registry.read_text().replace('path = "docs/reference.md"', f'path = "{relative}"')
        )

        errors = validate_registry(root)
        self.assertTrue(any("canonical repository-relative path" in error for error in errors))

    def test_noncanonical_documentation_paths_are_rejected(self):
        for invalid_path in (
            "./docs/reference.md",
            "docs//reference.md",
            "docs/../reference.md",
            r"docs\\reference.md",
            "C:/outside.md",
        ):
            with self.subTest(path=invalid_path):
                content = self.valid_declaration().replace(
                    'path = "docs/reference.md"', f"path = {json.dumps(invalid_path)}"
                )
                temp, root = self.make_root(content)
                self.addCleanup(temp.cleanup)
                errors = validate_registry(root)
                self.assertTrue(
                    any("canonical repository-relative path" in error for error in errors)
                )

    def test_symlinked_documentation_file_is_rejected(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        docs = root / "docs/reference.md"
        target = root / "docs/alternate.md"
        target.write_text("# Timer\\n")
        docs.unlink()
        try:
            docs.symlink_to(target)
        except (OSError, NotImplementedError) as exc:
            self.skipTest(f"symbolic links are unavailable: {exc}")

        errors = validate_registry(root)
        self.assertTrue(any("documentation path must not traverse symlinks" in error for error in errors))

    def test_symlinked_documentation_directory_is_rejected(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        docs_dir = root / "docs"
        target = root / "outside-docs"
        docs_dir.rename(target)
        try:
            docs_dir.symlink_to(target, target_is_directory=True)
        except (OSError, NotImplementedError) as exc:
            self.skipTest(f"symbolic links are unavailable: {exc}")

        errors = validate_registry(root)
        self.assertTrue(any("documentation path must not traverse symlinks" in error for error in errors))

    def test_namespace_mismatch_fails(self):
        content = self.valid_declaration().replace("gfa.atari-st.system.timer", "gfa.amiga.system.timer")
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("does not match namespace" in error for error in validate_registry(root)))

    def test_book_documented_evidence_requires_printed_page_locators(self):
        content = self.valid_declaration().replace('printed-pages = ["printed p. 1"]\n', "")
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("book/primary-reference evidence needs printed-page locators" in error for error in errors))

    def test_disputed_evidence_cannot_claim_implementation_support_or_conformance(self):
        content = self.valid_declaration().replace(
            'status = "book-documented"', 'status = "disputed"'
        ).replace(
            'status = "not-implemented"', 'status = "implemented"\nhandler = "example.handler"'
        ).replace(
            'classification = "unsupported"', 'classification = "supported"'
        ).replace(
            'conformance = "not-tested"', 'conformance = "passing"'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("disputed evidence cannot claim an implemented behavior" in error for error in errors))
        self.assertTrue(any("uncertain/disputed evidence cannot claim supported target behavior" in error for error in errors))
        self.assertTrue(any("disputed evidence cannot have passing conformance" in error for error in errors))

    def test_supported_entry_requires_tests_and_implementation(self):
        content = self.valid_declaration().replace('classification = "unsupported"', 'classification = "supported"')
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("supported entry has no implementation" in error for error in errors))
        self.assertTrue(any("supported entry needs passing conformance tests" in error for error in errors))


    def test_unknown_support_requires_an_explicit_pending_reason(self):
        content = self.valid_declaration().replace(
            'classification = "unsupported"', 'classification = "unknown"'
        ).replace(
            'pending-reason = "No implementation or conformance test exists."\n', ""
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(
            any("unknown/unsupported classification needs a pending reason" in error for error in errors)
        )

        registry = root / "api/registry/atari-st/time.toml"
        registry.write_text(
            registry.read_text().replace(
                "tests = []",
                'tests = []\npending-reason = "No implementation or conformance test exists."'
            )
        )
        self.assertEqual(validate_registry(root), [])


    def test_duplicate_ids_fail(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        duplicate = root / "api/registry/atari-st/duplicate.toml"
        duplicate.write_text(self.valid_declaration())
        self.assertTrue(any("duplicate ID" in error for error in validate_registry(root)))

    def test_duplicate_name_for_same_dialect_and_version_fails(self):
        first = self.valid_declaration()
        second = self.valid_declaration().replace(
            "gfa.atari-st.system.timer", "gfa.atari-st.system.timer-copy"
        )
        temp, root = self.make_root(first)
        self.addCleanup(temp.cleanup)
        duplicate = root / "api/registry/atari-st/timer-copy.toml"
        duplicate.write_text(second)
        self.assertTrue(any("conflicting declaration name" in error for error in validate_registry(root)))

    def test_profile_mismatch_fails(self):
        content = self.valid_declaration().replace('profile = "atari-st"', 'profile = "amiga"')
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("must use profile" in error for error in validate_registry(root)))

    def test_missing_documentation_anchor_fails(self):
        content = self.valid_declaration().replace("anchors = []", 'anchors = ["missing-section"]')
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("missing documentation anchor" in error for error in validate_registry(root)))

    def test_documentation_anchor_normalizes_en_dash_in_page_ranges(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        docs = root / "docs/reference.md"
        docs.write_text("# Printed pp. 13-1–13-2\n")
        registry = root / "api/registry/atari-st/time.toml"
        registry.write_text(
            registry.read_text().replace(
                "anchors = []",
                'anchors = ["printed-pp-13-1-13-2"]'
            )
        )
        self.assertEqual(validate_registry(root), [])

    def test_unknown_test_id_fails(self):
        content = self.valid_declaration().replace("tests = []", 'tests = ["missing-fixture"]')
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("unknown test ID" in error for error in validate_registry(root)))

    def test_implemented_entry_requires_handler_id(self):
        content = self.valid_declaration().replace(
            'status = "not-implemented"', 'status = "implemented"'
        )
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("implemented entry has no handler ID" in error for error in validate_registry(root)))

    def test_unknown_source_fails(self):
        content = self.valid_declaration().replace("TEST-SOURCE", "MISSING-SOURCE")
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("unknown source ID" in error for error in validate_registry(root)))


if __name__ == "__main__":
    unittest.main()
