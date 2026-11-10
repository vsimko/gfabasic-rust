import importlib.util
import sys
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "validate_gfatest_results.py"
SPEC = importlib.util.spec_from_file_location("validate_gfatest_results", MODULE_PATH)
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class ValidateGfaTestResults(unittest.TestCase):
    def validate(self, raw, expected=("hello", "fact-zero")):
        return MODULE.validate_result_bytes(list(expected), raw)

    def test_complete_batch(self):
        result = self.validate(b"GFATEST:hello:PASS\nGFATEST:fact-zero:PASS\n")
        self.assertTrue(result.valid, result.errors)
        self.assertEqual((result.passed, result.failed), (2, 0))

    def test_fail_is_valid_result_but_not_success(self):
        result = self.validate(b"GFATEST:hello:FAIL\nGFATEST:fact-zero:PASS\n")
        self.assertTrue(result.valid, result.errors)
        self.assertEqual((result.passed, result.failed), (1, 1))

    def test_missing_result(self):
        result = self.validate(b"GFATEST:hello:PASS\n")
        self.assertFalse(result.valid)
        self.assertTrue(any("missing result" in error for error in result.errors))

    def test_duplicate_result(self):
        result = self.validate(
            b"GFATEST:hello:PASS\nGFATEST:hello:FAIL\nGFATEST:fact-zero:PASS\n"
        )
        self.assertFalse(result.valid)
        self.assertTrue(any("duplicate result" in error for error in result.errors))

    def test_unknown_id(self):
        result = self.validate(b"GFATEST:other:PASS\nGFATEST:fact-zero:PASS\n")
        self.assertFalse(result.valid)
        self.assertTrue(any("unknown fixture ID" in error for error in result.errors))

    def test_malformed_and_truncated_record(self):
        for raw in (b"GFATEST:hello:PASS\nGFATEST:fact-zero:",
                    b"GFATEST:hello:PASS extra\nGFATEST:fact-zero:PASS\n",
                    b"GFATEST:hello:UNKNOWN\nGFATEST:fact-zero:PASS\n"):
            with self.subTest(raw=raw):
                self.assertFalse(self.validate(raw).valid)

    def test_non_ascii(self):
        result = self.validate(b"GFATEST:hello:PASS\nGFATEST:fact-z\xff:PASS\n")
        self.assertFalse(result.valid)
        self.assertIn("not ASCII", result.errors[0])

    def test_empty_result(self):
        result = self.validate(b"")
        self.assertFalse(result.valid)
        self.assertIn("empty or missing", result.errors[0])

    def test_duplicate_expected_ids(self):
        result = self.validate(b"GFATEST:hello:PASS\n", ("hello", "hello"))
        self.assertFalse(result.valid)
        self.assertTrue(any("duplicate expected" in error for error in result.errors))

    def test_crlf_is_accepted(self):
        result = self.validate(b"GFATEST:hello:PASS\r\nGFATEST:fact-zero:PASS\r\n")
        self.assertTrue(result.valid, result.errors)


if __name__ == "__main__":
    unittest.main()
