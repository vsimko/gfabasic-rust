import unittest
from pathlib import Path

from scripts.validate_amiga_chapter_inventories import extract_inventory, validate


class ChapterInventoryTests(unittest.TestCase):
    def test_repository_inventories_have_cited_rows(self):
        root = Path(__file__).resolve().parents[2]
        counts = validate(root)
        self.assertGreaterEqual(counts[7], 20)
        self.assertGreaterEqual(counts[8], 15)

    def test_duplicate_construct_labels_are_rejected(self):
        text = """## Inventory
| Construct | Syntax | Semantics | Printed pages |
|---|---|---|---|
| IF | `IF` | conditional | 7-4 |
| IF | `IF` | conditional | 7-5 |
"""
        errors = extract_inventory(text, "## Inventory", 7, 2, 55)
        self.assertTrue(any("duplicate inventory label" in error for error in errors))

    def test_out_of_chapter_page_is_rejected(self):
        text = """## Inventory
| Construct | Syntax | Semantics | Printed pages |
|---|---|---|---|
| IF | `IF` | conditional | 8-4 |
"""
        errors = extract_inventory(text, "## Inventory", 7, 2, 55)
        self.assertTrue(any("no printed Chapter 7 page citation" in error for error in errors))

    def test_missing_required_cell_is_rejected(self):
        text = """## Inventory
| Construct | Syntax | Semantics | Printed pages |
|---|---|---|---|
| IF | `IF` |  | 7-4 |
"""
        errors = extract_inventory(text, "## Inventory", 7, 2, 55)
        self.assertTrue(any("must not be empty" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
