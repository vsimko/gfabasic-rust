import json
import tempfile
import unittest
from pathlib import Path

from scripts.generate_project_data import generate, load_model

ROOT = Path(__file__).resolve().parents[2]
MODEL = ROOT / "validation/project.json"


class ProjectDataGeneratorTests(unittest.TestCase):
    def test_output_is_deterministic(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "project.json"
            first = generate(ROOT, MODEL, output)
            first_bytes = output.read_bytes()
            second = generate(ROOT, MODEL, output)
            self.assertEqual(first, second)
            self.assertEqual(first_bytes, output.read_bytes())

    def test_workspace_packages_come_from_cargo_manifest(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = generate(ROOT, MODEL, Path(tmp) / "project.json")
            self.assertTrue(result["workspace_packages"])
            self.assertTrue(all(item["name"] and item["path"] for item in result["workspace_packages"]))

    def test_component_status_and_identity_are_validated(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "model.json"
            model = json.loads(MODEL.read_text(encoding="utf-8"))
            model["components"][0]["status"] = "almost-done"
            path.write_text(json.dumps(model), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "invalid status"):
                load_model(path, ROOT)

    def test_duplicate_component_ids_are_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "model.json"
            model = json.loads(MODEL.read_text(encoding="utf-8"))
            model["components"][1]["id"] = model["components"][0]["id"]
            path.write_text(json.dumps(model), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate component id"):
                load_model(path, ROOT)

    def test_missing_rationale_or_evidence_is_rejected(self):
        for field in ("rationale", "evidence"):
            with self.subTest(field=field), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp) / "model.json"
                model = json.loads(MODEL.read_text(encoding="utf-8"))
                model["components"][0].pop(field)
                path.write_text(json.dumps(model), encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "missing"):
                    load_model(path, ROOT)

    def test_missing_evidence_file_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "model.json"
            model = json.loads(MODEL.read_text(encoding="utf-8"))
            model["components"][0]["evidence"] = ["missing/evidence.md"]
            path.write_text(json.dumps(model), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "evidence source does not exist"):
                load_model(path, ROOT)


if __name__ == "__main__":
    unittest.main()
