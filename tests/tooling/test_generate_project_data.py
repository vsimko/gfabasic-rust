import tempfile
import unittest
from pathlib import Path

from scripts.generate_project_data import generate

ROOT = Path(__file__).resolve().parents[2]


class ProjectDataGeneratorTests(unittest.TestCase):
    def test_output_is_deterministic(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "project.json"
            first = generate(ROOT, ROOT / "validation/project.json", output)
            first_bytes = output.read_bytes()
            second = generate(ROOT, ROOT / "validation/project.json", output)
            self.assertEqual(first, second)
            self.assertEqual(first_bytes, output.read_bytes())

    def test_workspace_packages_come_from_cargo_manifest(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = generate(ROOT, ROOT / "validation/project.json", Path(tmp) / "project.json")
            self.assertTrue(result["workspace_packages"])
            self.assertTrue(all(item["name"] and item["path"] for item in result["workspace_packages"]))


if __name__ == "__main__":
    unittest.main()
