import hashlib
import json
import tempfile
import unittest
from pathlib import Path

from scripts.validate_api_reviews import validate_reviews


class ReviewResultValidationTests(unittest.TestCase):
    def make_root(self):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        (root / "api/registry/amiga").mkdir(parents=True)
        (root / "api/schema").mkdir(parents=True)
        (root / "api/reviews").mkdir(parents=True)
        (root / "docs").mkdir()
        (root / "api/registry/amiga/example.toml").write_text(
            """schema-version = 1

[[declarations]]
id = "gfa.amiga.test.example"
name = "EXAMPLE"
kind = "statement"

[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.5"]
profile = "amiga"

[[declarations.syntax]]
form = "EXAMPLE"
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
        )
        schema_source = (
            Path(__file__).resolve().parents[2]
            / "api/schema/agent-review-result-v1.json"
        )
        (root / "api/schema/agent-review-result-v1.json").write_text(
            schema_source.read_text(encoding="utf-8")
        )
        (root / "docs/reference-sources.md").write_text(
            "| ID | Title | Scope |\n|---|---|---|\n| TEST-SOURCE | Test manual | Amiga 3.5 |\n"
        )
        (root / "docs/research.md").write_text("# Chapter 3\n")
        return temp, root

    def review_result(self, root, verdict="supported"):
        registry = root / "api/registry/amiga/example.toml"
        source_register = root / "docs/reference-sources.md"
        docs = root / "docs/research.md"
        digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
        return {
            "schema_version": 1,
            "declaration_id": "gfa.amiga.test.example",
            "reviewed_inputs": {
                "registry_sha256": digest(registry),
                "source_register_sha256": digest(source_register),
                "documentation_sha256": digest(docs),
            },
            "verdict": verdict,
            "confidence": "high",
            "reviewed_scope": "Amiga GFA BASIC interpreter 3.5",
            "reasoning": "The primary source directly supports the reviewed syntax.",
            "evidence": [
                {
                    "kind": "primary-source",
                    "reference": "TEST-SOURCE",
                    "locator": "printed p. 1",
                    "relevance": "Documents the statement syntax.",
                }
            ],
            "counter_evidence": [],
            "limitations": [],
            "follow_up": [],
            "human_review_required": True,
            "reviewer": {
                "agent": "test-agent",
                "model": "test-model",
                "procedure_version": "1",
            },
        }

    def test_no_review_files_is_valid(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        self.assertEqual(validate_reviews(root), ([], []))

    def test_valid_review_result_is_accepted(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        result = self.review_result(root)
        (root / "api/reviews/example.json").write_text(json.dumps(result))
        self.assertEqual(validate_reviews(root), ([], []))

    def test_stale_documentation_hash_is_rejected(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        result = self.review_result(root)
        result["reviewed_inputs"]["documentation_sha256"] = "0" * 64
        (root / "api/reviews/example.json").write_text(json.dumps(result))
        errors, _ = validate_reviews(root)
        self.assertTrue(any("documentation_sha256 no longer matches" in error for error in errors))

    def test_primary_source_must_be_linked_from_declaration(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        result = self.review_result(root)
        result["evidence"][0]["reference"] = "UNKNOWN-SOURCE"
        (root / "api/reviews/example.json").write_text(json.dumps(result))
        errors, _ = validate_reviews(root)
        self.assertTrue(any("unknown primary source ID" in error for error in errors))

    def test_conflicting_verdicts_are_reported_and_escalated(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        first = self.review_result(root, "supported")
        second = self.review_result(root, "contradicted")
        (root / "api/reviews/agent-a.json").write_text(json.dumps(first))
        (root / "api/reviews/agent-b.json").write_text(json.dumps(second))
        errors, warnings = validate_reviews(root)
        self.assertEqual(errors, [])
        self.assertTrue(any("conflicting AI review verdicts" in warning for warning in warnings))


if __name__ == "__main__":
    unittest.main()
