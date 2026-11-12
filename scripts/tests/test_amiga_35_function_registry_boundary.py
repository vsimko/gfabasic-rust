"""Keep the 3.5 function inventory and profile-specific registries in sync."""

from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]
FUNCTIONS = ROOT / "docs/research/amiga-gfa-basic-functions.toml"
LANGUAGE_CORE = ROOT / "api/registry/language-core/math-and-matrix.toml"
AMIGA_PROFILE = ROOT / "api/registry/amiga/special-and-platform-commands.toml"


def load(path):
    return tomllib.loads(path.read_text(encoding="utf-8"))


class Amiga35FunctionRegistryBoundaryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.inventory = load(FUNCTIONS)["functions"]
        cls.language = load(LANGUAGE_CORE)["declarations"]
        cls.amiga = load(AMIGA_PROFILE)["declarations"]

    def test_explicit_35_additions_are_exactly_three_language_functions_and_guru(self):
        additions = {
            item["name"]
            for item in self.inventory
            if item["origin"] == "3.5-addendum"
        }
        self.assertEqual(additions, {"FACT", "VARIAT", "COMBIN", "GURU"})

        language_functions = {
            item["name"]
            for item in self.language
            if item["kind"] == "function"
            and item["id"].startswith("gfa.language.")
        }
        self.assertEqual(language_functions, {"FACT", "VARIAT", "COMBIN"})

        amiga_guru_functions = {
            item["name"]
            for item in self.amiga
            if item["kind"] == "function" and item["name"] == "GURU"
        }
        self.assertEqual(amiga_guru_functions, {"GURU"})

    def test_each_language_function_has_structured_signature_and_provenance(self):
        by_name = {item["name"]: item for item in self.language}
        for name in ("FACT", "VARIAT", "COMBIN"):
            with self.subTest(function=name):
                declaration = by_name[name]
                self.assertTrue(declaration["id"].startswith("gfa.language."))
                self.assertTrue(declaration["syntax"])
                self.assertTrue(declaration["signature"])
                self.assertTrue(declaration["availability"])
                evidence = declaration["evidence"]
                self.assertEqual(evidence["source-scope"], "amiga-3.5-addendum")
                self.assertIn("AM-INTERPRETER-35", evidence["sources"])
                self.assertTrue(evidence["printed-pages"])
                self.assertTrue(declaration["docs"]["path"])
                self.assertTrue(declaration["docs"]["anchors"])

    def test_guru_function_is_not_duplicated_in_language_core(self):
        self.assertFalse(
            any(item["name"] == "GURU" for item in self.language),
            "GURU() is classified in the Amiga API profile, not language-core",
        )
        guru = next(
            item for item in self.amiga
            if item["name"] == "GURU" and item["kind"] == "function"
        )
        self.assertEqual(guru["id"], "gfa.amiga.system.guru-number")
        self.assertIn("AM-INTERPRETER-35", guru["evidence"]["sources"])


if __name__ == "__main__":
    unittest.main()
