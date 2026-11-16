import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[2] / "scripts" / "validate_repository.py"


class ValidatorTests(unittest.TestCase):
    def run_validator(self, root, rule):
        rules = root / "rules.json"
        rules.write_text(json.dumps({"version": 1, "rules": [rule]}), encoding="utf-8")
        return subprocess.run([sys.executable, str(SCRIPT), "--root", str(root), "--rules", str(rules)], text=True, capture_output=True)

    def rule(self, **overrides):
        rule = {"id": "test.exists", "description": "fixture exists", "enabled": True,
                "severity": "error", "type": "file_exists", "path": "present.txt",
                "evidence": "test fixture"}
        rule.update(overrides)
        return rule

    def test_existing_file_passes(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "present.txt").write_text("hello", encoding="utf-8")
            result = self.run_validator(root, self.rule())
            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)

    def test_missing_file_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_validator(Path(tmp), self.rule())
            self.assertEqual(result.returncode, 1)
            self.assertIn("test.exists", result.stdout)

    def test_disabled_rule_is_skipped(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_validator(Path(tmp), self.rule(enabled=False))
            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)

    def test_invalid_rule_fails_clearly(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_validator(Path(tmp), self.rule(unexpected=True))
            self.assertEqual(result.returncode, 2)
            self.assertIn("fields must be exactly", result.stderr)

    def test_text_contains_passes_and_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "present.txt").write_text("alpha beta", encoding="utf-8")
            rule = self.rule(type="text_contains", text="beta")
            passed = self.run_validator(root, rule)
            self.assertEqual(passed.returncode, 0, passed.stderr + passed.stdout)

            failed = self.run_validator(root, self.rule(type="text_contains", text="missing"))
            self.assertEqual(failed.returncode, 1)
            self.assertIn("test.exists", failed.stdout)

    def test_warning_failure_does_not_fail_run(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_validator(Path(tmp), self.rule(severity="warning"))
            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
            self.assertIn("WARNING test.exists", result.stdout)

    def test_path_escape_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_validator(Path(tmp), self.rule(path="../outside.txt"))
            self.assertEqual(result.returncode, 2)
            self.assertIn("path escapes repository root", result.stderr)

    def test_duplicate_rule_ids_are_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            first = self.rule()
            second = self.rule(path="other.txt")
            rules = root / "rules.json"
            rules.write_text(json.dumps({"version": 1, "rules": [first, second]}), encoding="utf-8")
            result = subprocess.run([sys.executable, str(SCRIPT), "--root", str(root), "--rules", str(rules)], text=True, capture_output=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("duplicate rule id", result.stderr)


if __name__ == "__main__":
    unittest.main()
