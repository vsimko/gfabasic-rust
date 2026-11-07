import json
import tempfile
import unittest
from pathlib import Path

from scripts.generate_api_catalogue import generate, render


class ApiCatalogueGeneratorTests(unittest.TestCase):
    def make_root(self):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        (root / "api/registry/language-core").mkdir(parents=True)
        (root / "api/registry/amiga").mkdir(parents=True)
        (root / "api/registry/language-core/math.toml").write_text(
            """schema-version = 1
[[declarations]]
id = "gfa.language.math.fact"
name = "FACT"
kind = "function"
[declarations.evidence]
status = "book-documented"
[declarations.implementation]
status = "not-implemented"
[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unknown"
conformance = "not-tested"
"""
        )
        (root / "api/registry/amiga/graphics.toml").write_text(
            """schema-version = 1
[[declarations]]
id = "gfa.amiga.graphics.pcolor"
name = "PCOLOR"
kind = "statement"
[declarations.evidence]
status = "disputed"
[declarations.implementation]
status = "not-implemented"
[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unknown"
conformance = "not-tested"
"""
        )
        return temp, root

    def test_output_is_deterministic_and_counts_dimensions_separately(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        first = render(root)
        second = render(root)
        self.assertEqual(first, second)
        data = json.loads(first)
        self.assertEqual(data["counts"]["total"], 2)
        self.assertEqual(data["counts"]["by_namespace"], {"amiga": 1, "language-core": 1})
        self.assertEqual(data["counts"]["by_evidence_status"], {"book-documented": 1, "disputed": 1})
        self.assertEqual(data["counts"]["by_implementation_status"], {"not-implemented": 2})
        self.assertEqual(data["counts"]["by_target_support"]["x86_64-unknown-linux-gnu"], {"unknown": 2})
        self.assertEqual(data["entries"][0]["id"], "gfa.amiga.graphics.pcolor")

    def test_empty_registry_generates_valid_empty_catalogue(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "api/registry").mkdir(parents=True)
            data = json.loads(render(root))
            self.assertEqual(data["entries"], [])
            self.assertEqual(data["counts"]["total"], 0)

    def test_check_mode_detects_stale_output(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        output = root / "site/data/api-catalogue.json"
        self.assertTrue(generate(root, output))
        self.assertTrue(generate(root, output, check=True))
        output.write_text("{}")
        self.assertFalse(generate(root, output, check=True))


if __name__ == "__main__":
    unittest.main()
