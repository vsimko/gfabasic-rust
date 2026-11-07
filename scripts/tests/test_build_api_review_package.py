import json
import tempfile
import unittest
from pathlib import Path

from scripts.build_api_review_package import build_package


DECLARATION = """schema-version = 1

[[declarations]]
id = "gfa.amiga.test.example"
name = "EXAMPLE"
kind = "statement"

[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.5"]
profile = "amiga"

[[declarations.syntax]]
form = "EXAMPLE value"
contexts = ["statement"]

[declarations.signature]
type = "no return value"

[declarations.semantics]
summary = "A test declaration."

[declarations.evidence]
status = "book-documented"
sources = ["TEST-SOURCE"]
printed-pages = ["p. 1"]

[declarations.implementation]
status = "not-implemented"

[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unknown"
conformance = "not-tested"
tests = []
pending-reason = "Not implemented."

[declarations.docs]
path = "docs/research.md"
anchors = ["chapter-3"]
"""


class ReviewPackageTests(unittest.TestCase):
    def make_root(self):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        (root / "api/registry/amiga").mkdir(parents=True)
        (root / "docs").mkdir()
        (root / "tests/corpus").mkdir(parents=True)
        (root / "api/registry/amiga/example.toml").write_text(DECLARATION)
        (root / "docs/reference-sources.md").write_text(
            "| ID | Title | Scope |\n|---|---|---|\n| TEST-SOURCE | Test manual | Amiga 3.5 |\n"
        )
        (root / "docs/research.md").write_text(
            "# Chapter 3\n\nThe source documents this example.\n\n## Other section\n"
        )
        (root / "tests/corpus/manifest.toml").write_text("fixtures = []\n")
        return temp, root

    def test_package_is_deterministic_and_contains_evidence_context(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)

        first = build_package(root, "gfa.amiga.test.example")
        second = build_package(root, "gfa.amiga.test.example")

        self.assertEqual(first, second)
        self.assertEqual(first["source_records"][0]["id"], "TEST-SOURCE")
        self.assertIn("The source documents this example.", first["documentation_sections"][0]["content"])
        self.assertEqual(first["inputs"]["registry_path"], "api/registry/amiga/example.toml")
        self.assertTrue(first["review_result_template"]["human_review_required"])

    def test_unknown_declaration_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        with self.assertRaisesRegex(ValueError, "unknown declaration ID"):
            build_package(root, "gfa.amiga.test.missing")


if __name__ == "__main__":
    unittest.main()
