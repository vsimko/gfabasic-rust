from __future__ import annotations

import io
import json
import tempfile
import unittest
from pathlib import Path
from urllib.error import HTTPError

from scripts.check_private_github_assets import check_asset_paths, read_manifest


class PrivateGithubAssetPreflightTests(unittest.TestCase):
    def test_reads_manifest_filenames(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / "assets.tsv"
            manifest.write_text("# comment\n3.5|GFA Basic 3.5.zip\n3.52|Disk 1.zip\n", encoding="utf-8")
            self.assertEqual(read_manifest(manifest), ["GFA Basic 3.5.zip", "Disk 1.zip"])

    def test_rejects_malformed_manifest_row(self):
        with tempfile.TemporaryDirectory() as directory:
            manifest = Path(directory) / "assets.tsv"
            manifest.write_text("missing-separator\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "expected label\|filename"):
                read_manifest(manifest)

    def test_checks_encoded_path_and_requires_file_metadata(self):
        observed = []

        def opener(request, timeout):
            observed.append((request, timeout))
            return io.BytesIO(json.dumps({"type": "file"}).encode("utf-8"))

        missing = check_asset_paths(
            "owner/private-assets",
            "main",
            ["GFA Basic 3.5.zip"],
            "test-token",
            opener=opener,
        )

        self.assertEqual(missing, [])
        request, timeout = observed[0]
        self.assertIn("GFA%20Basic%203.5.zip", request.full_url)
        self.assertEqual(request.get_header("Authorization"), "Bearer test-token")
        self.assertEqual(timeout, 20)

    def test_reports_404_as_missing_path(self):
        def opener(request, timeout):
            raise HTTPError(request.full_url, 404, "Not Found", None, None)

        self.assertEqual(
            check_asset_paths("owner/private-assets", "main", ["missing.zip"], "test-token", opener),
            ["missing.zip"],
        )

    def test_rejects_directories_as_asset_files(self):
        def opener(request, timeout):
            return io.BytesIO(json.dumps({"type": "dir"}).encode("utf-8"))

        self.assertEqual(
            check_asset_paths("owner/private-assets", "main", ["folder"], "test-token", opener),
            ["folder"],
        )

    def test_other_http_errors_do_not_expose_response_or_token(self):
        def opener(request, timeout):
            raise HTTPError(request.full_url, 403, "Forbidden", None, None)

        with self.assertRaises(RuntimeError) as raised:
            check_asset_paths("owner/private-assets", "main", ["private.zip"], "secret-value", opener)

        self.assertIn("HTTP 403", str(raised.exception))
        self.assertNotIn("secret-value", str(raised.exception))


if __name__ == "__main__":
    unittest.main()
