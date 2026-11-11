import importlib.util
import json
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path
from unittest.mock import patch

MODULE_PATH = Path(__file__).resolve().parents[2] / "scripts" / "generate_repository_activity.py"
SPEC = importlib.util.spec_from_file_location("generate_repository_activity", MODULE_PATH)
activity = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(activity)


class RepositoryActivityTests(unittest.TestCase):
    def test_collects_merged_prs_and_commits_sorted_and_capped(self):
        prs = [
            {"merged_at": f"2026-11-{day:02d}T10:00:00Z", "title": f"PR {day}", "number": day,
             "html_url": f"https://github.com/vsimko/gfabasic-rust/pull/{day}"}
            for day in range(1, 16)
        ]
        commits = [
            {"sha": f"{day:040x}", "html_url": f"https://github.com/vsimko/gfabasic-rust/commit/{day}",
             "commit": {"message": f"Commit {day}", "committer": {"date": f"2026-11-{day:02d}T12:00:00Z"}}}
            for day in range(1, 16)
        ]
        with patch.object(activity, "request_json", side_effect=[prs, commits]):
            result = activity.collect_activity("token", datetime(2026, 11, 20, tzinfo=timezone.utc))
        self.assertEqual(len(result["items"]), activity.LIMIT)
        self.assertEqual(result["items"][0]["title"], "Commit 15")
        self.assertEqual(result["items"][-1]["timestamp"], "2026-11-10T12:00:00Z")
        self.assertEqual({item["type"] for item in result["items"]}, {"commit"})

    def test_ignores_unmerged_pull_requests_and_marks_empty(self):
        with patch.object(activity, "request_json", side_effect=[
            [{"merged_at": None, "title": "Open PR"}], []
        ]):
            result = activity.collect_activity(None, datetime(2026, 11, 20, tzinfo=timezone.utc))
        self.assertEqual(result["status"], "empty")
        self.assertEqual(result["items"], [])

    def test_api_error_preserves_previous_snapshot_as_stale(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "activity.json"
            previous = {"schema_version": 1, "status": "fresh", "last_successful_at": "2026-11-10T00:00:00+00:00",
                        "items": [{"type": "commit", "title": "Known", "url": "https://github.com/x/y/commit/a",
                                   "timestamp": "2026-11-10T00:00:00Z", "summary": "Commit a"}]}
            path.write_text(json.dumps(previous), encoding="utf-8")
            with patch.object(activity, "collect_activity", side_effect=activity.GitHubAPIError("offline")):
                result = activity.build_activity(None, path, datetime(2026, 11, 20, tzinfo=timezone.utc))
            self.assertEqual(result["status"], "stale")
            self.assertEqual(result["items"][0]["title"], "Known")
            self.assertEqual(result["last_successful_at"], previous["last_successful_at"])

    def test_permission_denial_without_snapshot_is_explicit(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "activity.json"
            with patch.object(activity, "collect_activity",
                               side_effect=activity.GitHubAPIError("forbidden", permission_denied=True)):
                result = activity.build_activity("token", path)
            self.assertEqual(result["status"], "permission-denied")
            self.assertEqual(result["items"], [])

    def test_render_escapes_content_and_does_not_claim_feature_completion(self):
        rendered = activity.render_activity({
            "status": "fresh", "last_successful_at": "2026-11-10T00:00:00Z",
            "items": [{"type": "merged_pull_request", "title": "<script>alert(1)</script>",
                       "url": "https://github.com/vsimko/gfabasic-rust/pull/1",
                       "timestamp": "2026-11-10T00:00:00Z", "summary": "PR #1 merged"}]
        })
        self.assertIn("&lt;script&gt;", rendered)
        self.assertNotIn("<script>", rendered)
        self.assertIn("activity events, not feature-completion claims", rendered)


if __name__ == "__main__":
    unittest.main()
