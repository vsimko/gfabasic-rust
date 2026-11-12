import tempfile
import unittest
from pathlib import Path

from scripts.validate_amiga_function_inventory import validate


class FunctionInventoryTests(unittest.TestCase):
    def test_repository_inventory_counts(self):
        root = Path(__file__).resolve().parents[2]
        result = validate(root / "docs/research/amiga-gfa-basic-functions.toml")
        self.assertEqual(result["all-identifiers"], 117)
        self.assertEqual(result["interpreter-identifiers-including-disputed-slot"], 115)
        self.assertEqual(result["unambiguous-interpreter-identifiers"], 113)
        self.assertEqual(result["disputed-name-slots"], 2)
        self.assertEqual(result["confirmed-by-3.5-example-identifiers"], 5)

    def test_only_the_five_audited_example_functions_have_3_5_use_evidence(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        import tomllib
        functions = tomllib.loads(source.read_text(encoding="utf-8"))["functions"]
        expected = {"HEX$", "STR$", "RAND", "SUCC", "STRING$"}
        actual = {item["name"] for item in functions if item["evidence-status"] == "confirmed-by-3.5-example"}
        self.assertEqual(actual, expected)
        for item in functions:
            if item["name"] in expected:
                self.assertEqual(item["origin"], "base-manual-3.0")
                self.assertEqual(item["confirmation-source-version"], "3.5 addendum")
                self.assertTrue(item["confirmation-printed-pages"])

    def test_non_audited_base_function_cannot_claim_3_5_example_evidence(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        text = source.read_text(encoding="utf-8")
        candidate = text.replace(
            'name = "ABS"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "base-manual-entry"',
            'name = "ABS"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "confirmed-by-3.5-example"\nconfirmation-source = "AM-INTERPRETER-35"\nconfirmation-source-version = "3.5 addendum"\nconfirmation-printed-pages = ["13-2"]',
            1,
        )
        self.assertNotEqual(candidate, text)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "functions.toml"
            path.write_text(candidate, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "no audited 3.5 addendum example-use evidence"):
                validate(path)

    def test_duplicate_names_are_rejected(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        text = source.read_text(encoding="utf-8")
        duplicate = text + "\n[[functions]]\nname = \"ABS\"\norigin = \"base-manual-3.0\"\napplicability = \"interpreter\"\nevidence-status = \"base-manual-entry\"\nprinted-pages = [\"4-12\"]\n"
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "functions.toml"
            path.write_text(duplicate, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate function identifier"):
                validate(path)

    def test_3_0_base_entry_cannot_be_promoted_by_changing_its_evidence_status(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        text = source.read_text(encoding="utf-8")
        candidate = text.replace(
            'name = "ABS"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "base-manual-entry"',
            'name = "ABS"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "explicit-3.5-addendum"',
            1,
        )
        self.assertNotEqual(candidate, text)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "functions.toml"
            path.write_text(candidate, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "3.0 base-manual evidence cannot be labelled"):
                validate(path)

    def test_3_5_addendum_identifier_set_is_exact(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        text = source.read_text(encoding="utf-8")
        candidate = text.replace(
            'name = "ABS"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "base-manual-entry"',
            'name = "ABS"\norigin = "3.5-addendum"\napplicability = "interpreter"\nevidence-status = "explicit-3.5-addendum"',
            1,
        ).replace(
            'name = "FACT"\norigin = "3.5-addendum"\napplicability = "interpreter"\nevidence-status = "explicit-3.5-addendum"',
            'name = "FACT"\norigin = "base-manual-3.0"\napplicability = "interpreter"\nevidence-status = "base-manual-entry"',
            1,
        )
        self.assertNotEqual(candidate, text)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "functions.toml"
            path.write_text(candidate, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "3.5 addendum function set differs"):
                validate(path)

    def test_3_5_addendum_entries_require_explicit_version_evidence(self):
        root = Path(__file__).resolve().parents[2]
        source = root / "docs/research/amiga-gfa-basic-functions.toml"
        text = source.read_text(encoding="utf-8")
        candidate = text.replace(
            'name = "FACT"\norigin = "3.5-addendum"\napplicability = "interpreter"\nevidence-status = "explicit-3.5-addendum"',
            'name = "FACT"\norigin = "3.5-addendum"\napplicability = "interpreter"\nevidence-status = "base-manual-entry"',
            1,
        )
        self.assertNotEqual(candidate, text)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "functions.toml"
            path.write_text(candidate, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "3.5 addendum entries require"):
                validate(path)


if __name__ == "__main__":
    unittest.main()
