import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path
from unittest.mock import patch

from scripts.generate_ci_health import ApiError, collect, render, summarize_run


NOW = datetime(2026, 11, 9, 12, 0, tzinfo=timezone.utc)


class CiHealthGeneratorTests(unittest.TestCase):
    def make_root(self):
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name)
        workflows = root / ".github/workflows"
        workflows.mkdir(parents=True)
        (workflows / "docs.yml").write_text("name: Documentation checks\n", encoding="utf-8")
        return temporary, root

    def test_latest_failure_and_last_success_are_reported_separately(self):
        temporary, root = self.make_root()
        self.addCleanup(temporary.cleanup)
        responses = [
            {"workflows": [{"id": 11, "path": ".github/workflows/docs.yml"}]},
            {"workflow_runs": [
                {"id": 2, "name": "Documentation checks", "status": "completed",
                 "conclusion": "failure", "event": "push", "head_branch": "main",
                 "created_at": "2026-11-09T10:00:00Z", "updated_at": "2026-11-09T10:10:00Z",
                 "html_url": "https://github.com/vsimko/gfabasic-rust/actions/runs/2"},
                {"id": 1, "name": "Documentation checks", "status": "completed",
                 "conclusion": "success", "event": "push", "head_branch": "main",
                 "created_at": "2026-11-08T10:00:00Z", "updated_at": "2026-11-08T10:10:00Z",
                 "html_url": "https://github.com/vsimko/gfabasic-rust/actions/runs/1"},
            ]},
        ]
        with patch("scripts.generate_ci_health.api_json", side_effect=responses):
            snapshot = collect(root, "test-token", NOW)
        docs = next(item for item in snapshot["workflows"] if item["file"] == "docs.yml")
        self.assertEqual(docs["state"], "failure")
        self.assertEqual(docs["latest"]["id"], 2)
        self.assertEqual(docs["last_success"]["id"], 1)
        self.assertEqual(docs["last_success"]["status"], "success")

    def test_configured_workflow_without_runs_is_unknown(self):
        temporary, root = self.make_root()
        self.addCleanup(temporary.cleanup)
        responses = [
            {"workflows": [{"id": 11, "path": ".github/workflows/docs.yml"}]},
            {"workflow_runs": []},
        ]
        with patch("scripts.generate_ci_health.api_json", side_effect=responses):
            snapshot = collect(root, "test-token", NOW)
        docs = next(item for item in snapshot["workflows"] if item["file"] == "docs.yml")
        self.assertEqual(docs["state"], "unknown")
        self.assertIsNone(docs["latest"])
        self.assertTrue(docs["workflow_url"])

    def test_old_success_is_marked_stale(self):
        run = {
            "status": "completed", "conclusion": "success",
            "created_at": "2026-10-30T00:00:00Z", "updated_at": "2026-10-30T00:00:00Z",
        }
        self.assertEqual(summarize_run(run, NOW)["freshness"], "stale")

    def test_missing_workflow_is_not_configured_and_missing_token_is_unknown(self):
        temporary, root = self.make_root()
        self.addCleanup(temporary.cleanup)
        snapshot = collect(root, None, NOW)
        docs = next(item for item in snapshot["workflows"] if item["file"] == "docs.yml")
        conformance = next(item for item in snapshot["workflows"] if item["file"] == "conformance.yml")
        self.assertEqual(docs["state"], "unknown")
        self.assertIn("GH_TOKEN", docs["error"])
        self.assertEqual(conformance["state"], "not-configured")

    def test_api_permission_error_is_visible_but_does_not_break_generation(self):
        temporary, root = self.make_root()
        self.addCleanup(temporary.cleanup)
        with patch("scripts.generate_ci_health.api_json", side_effect=ApiError(403)):
            snapshot = collect(root, "test-token", NOW)
        docs = next(item for item in snapshot["workflows"] if item["file"] == "docs.yml")
        self.assertEqual(docs["state"], "unknown")
        self.assertIn("HTTP 403", docs["error"])

    def test_render_includes_latest_success_and_unconfigured_workflow(self):
        snapshot = {
            "generated_at": "2026-11-09T12:00:00Z",
            "workflows": [
                {"name": "Documentation checks", "state": "failure", "note": "Docs",
                 "latest": {"status": "failure", "event": "push", "branch": "main",
                            "age_hours": 1, "url": "https://example.test/latest"},
                 "last_success": {"updated_at": "2026-11-08T12:00:00Z",
                                  "url": "https://example.test/success"}},
                {"name": "Conformance suite", "state": "not-configured",
                 "note": "Not yet configured", "latest": None, "last_success": None},
            ],
        }
        output = render(snapshot)
        self.assertIn("Latest: failure", output)
        self.assertIn("Last success:", output)
        self.assertIn("Conformance suite", output)
        self.assertIn("NOT CONFIGURED", output)
        self.assertIn("https://example.test/latest", output)
        self.assertIn("https://example.test/success", output)


if __name__ == "__main__":
    unittest.main()
