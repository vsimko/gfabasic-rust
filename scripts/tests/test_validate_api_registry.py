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

[declarations.implementation]
status = "not-implemented"

[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unsupported"
conformance = "not-tested"
tests = []

[declarations.docs]
path = "docs/reference.md"
anchors = []
'''

    def test_research_entry_is_valid(self):
        temp, root = self.make_root(self.valid_declaration())
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_registry(root), [])

    def test_namespace_mismatch_fails(self):
        content = self.valid_declaration().replace("gfa.atari-st.system.timer", "gfa.amiga.system.timer")
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("does not match namespace" in error for error in validate_registry(root)))

    def test_supported_entry_requires_tests_and_implementation(self):
        content = self.valid_declaration().replace('classification = "unsupported"', 'classification = "supported"')
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        errors = validate_registry(root)
        self.assertTrue(any("supported entry has no implementation" in error for error in errors))
        self.assertTrue(any("supported entry needs passing conformance tests" in error for error in errors))

    def test_unknown_source_fails(self):
        content = self.valid_declaration().replace("TEST-SOURCE", "MISSING-SOURCE")
        temp, root = self.make_root(content)
        self.addCleanup(temp.cleanup)
        self.assertTrue(any("unknown source ID" in error for error in validate_registry(root)))


if __name__ == "__main__":
    unittest.main()
