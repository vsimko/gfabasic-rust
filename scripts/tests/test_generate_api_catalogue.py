import json
import tempfile
import unittest
from pathlib import Path

from scripts.generate_api_catalogue import generate, render, render_index


class ApiCatalogueGeneratorTests(unittest.TestCase):
    def make_root(self, count=2):
        temp = tempfile.TemporaryDirectory()
        root = Path(temp.name)
        registry = root / "api/registry/language-core"
        registry.mkdir(parents=True)
        declarations = []
        for index in range(count):
            declarations.append(
                f'''[[declarations]]
id = "gfa.language.test.fn{index:04d}"
name = "FN{index:04d}"
kind = "function"
syntax = [{{ form = "FN{index:04d}()" }}]
[declarations.evidence]
status = "book-documented"
[declarations.implementation]
status = "not-implemented"
[[declarations.support]]
target = "x86_64-unknown-linux-gnu"
classification = "unknown"
conformance = "not-tested"
'''
            )
        (registry / "functions.toml").write_text(
            "schema-version = 1\n" + "\n".join(declarations),
            encoding="utf-8",
        )
        return temp, root

    def test_index_and_details_are_deterministic_and_complete(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        first_index, first_details = render_index(root)
        second_index, second_details = render_index(root)
        self.assertEqual(first_index, second_index)
        self.assertEqual(first_details, second_details)
        index = json.loads(first_index)
        self.assertEqual(index["counts"]["total"], 2)
        self.assertEqual([item["name"] for item in index["entries"]], ["FN0000", "FN0001"])
        self.assertEqual(index["entries"][0]["syntax"], ["FN0000()"])
        detail = json.loads(first_details["gfa.language.test.fn0000.json"])
        self.assertEqual(detail["id"], index["entries"][0]["id"])
        self.assertEqual(detail["evidence"]["status"], "book-documented")

    def test_real_registry_generates_all_current_declarations(self):
        from scripts.generate_api_catalogue import ROOT
        data = json.loads(render(ROOT))
        self.assertGreaterEqual(data["counts"]["total"], 29)
        self.assertEqual(data["counts"]["by_namespace"]["language-core"], 32)
        self.assertEqual(data["counts"]["by_namespace"]["amiga"], 4)
        self.assertEqual(data["counts"]["by_namespace"]["atari-st"], 1)
        self.assertEqual(data["counts"]["by_evidence_status"].get("disputed", 0), 0)

        _, details = render_index(ROOT)
        inverse = json.loads(details["gfa.language.matrix.mat-inv.json"])
        self.assertEqual([syntax["form"] for syntax in inverse["syntax"]], ["MAT INV destination()=source()"])
        self.assertEqual(inverse["evidence"]["status"], "book-documented")
        self.assertTrue(any("source typo" in note for note in inverse["evidence"]["notes"]))

    def test_empty_and_partial_registries(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "api/registry").mkdir(parents=True)
            index, details = render_index(root)
            self.assertEqual(json.loads(index)["entries"], [])
            self.assertEqual(details, {})
        temp, root = self.make_root(1)
        self.addCleanup(temp.cleanup)
        index, details = render_index(root)
        self.assertEqual(json.loads(index)["counts"]["total"], 1)
        self.assertEqual(len(details), 1)

    def test_large_synthetic_registry_keeps_index_compact_and_resolves_details(self):
        temp, root = self.make_root(1000)
        self.addCleanup(temp.cleanup)
        index_text, details = render_index(root)
        index = json.loads(index_text)
        self.assertEqual(len(index["entries"]), 1000)
        self.assertEqual(len(details), 1000)
        self.assertLess(len(index_text), sum(map(len, details.values())))
        for entry in index["entries"]:
            filename = entry["detail"].rsplit("/", 1)[1]
            self.assertIn(filename, details)
            self.assertEqual(json.loads(details[filename])["id"], entry["id"])

    def test_check_mode_detects_missing_stale_and_extra_details(self):
        temp, root = self.make_root()
        self.addCleanup(temp.cleanup)
        output = root / "site/data/api-catalogue.json"
        details = root / "site/data/api-catalogue-details"
        self.assertTrue(generate(root, output, details))
        self.assertTrue(generate(root, output, details, check=True))
        (details / "gfa.language.test.fn0000.json").write_text("{}")
        self.assertFalse(generate(root, output, details, check=True))
        self.assertTrue(generate(root, output, details))
        (details / "stale.json").write_text("{}")
        self.assertFalse(generate(root, output, details, check=True))


if __name__ == "__main__":
    unittest.main()
