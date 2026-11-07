import tempfile
import unittest
from pathlib import Path

from validate_amiga_function_inventory import validate


class FunctionInventoryTests(unittest.TestCase):
    def test_repository_inventory_counts(self):
        root = Path(__file__).resolve().parents[2]
        result = validate(root / "docs/research/amiga-gfa-basic-functions.toml")
        self.assertEqual(result["all-identifiers"], 116)
        self.assertEqual(result["interpreter-identifiers-including-disputed-slot"], 114)
        self.assertEqual(result["unambiguous-interpreter-identifiers"], 113)

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


if __name__ == "__main__":
    unittest.main()
