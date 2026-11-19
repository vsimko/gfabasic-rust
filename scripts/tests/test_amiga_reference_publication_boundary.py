from __future__ import annotations

from pathlib import Path
import unittest

import yaml


WORKFLOW = Path(__file__).resolve().parents[2] / ".github" / "workflows" / "amiga-reference.yml"


class AmigaReferencePublicationBoundaryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = yaml.load(WORKFLOW.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)

    def test_public_publication_requires_independent_default_false_opt_in(self):
        inputs = self.workflow["on"]["workflow_dispatch"]["inputs"]
        self.assertEqual("false", inputs["publish_public_analysis"]["default"])
        self.assertEqual("boolean", inputs["publish_public_analysis"]["type"])
        self.assertEqual("false", inputs["publish_private_analysis"]["default"])
        self.assertIn("public", inputs["publish_public_analysis"]["description"].lower())

    def test_analysis_job_has_no_public_write_permissions_or_publication_steps(self):
        job = self.workflow["jobs"]["prepare-and-analyze"]
        self.assertEqual("read", job["permissions"]["contents"])
        self.assertNotIn("pull-requests", job["permissions"])
        steps = job["steps"]
        self.assertFalse(any("Publish reviewed analysis data through a pull request" == step.get("name") for step in steps))
        private_publish = next(step for step in steps if "Publish complete analysis" in step.get("name", ""))
        self.assertEqual("inputs.publish_private_analysis", private_publish["if"])
        self.assertTrue(any(step.get("name") == "Upload reviewed public analysis files only" for step in steps))

    def test_public_write_permissions_are_confined_to_opt_in_job(self):
        job = self.workflow["jobs"]["publish-public-analysis"]
        self.assertEqual("github.ref == 'refs/heads/main' && inputs.publish_public_analysis", job["if"])
        self.assertEqual("prepare-and-analyze", job["needs"])
        self.assertEqual("write", job["permissions"]["contents"])
        self.assertEqual("write", job["permissions"]["pull-requests"])
        self.assertNotIn("GFA_ASSETS_TOKEN", str(job))
        self.assertTrue(any(step.get("name") == "Publish reviewed analysis data through a pull request" for step in job["steps"]))

    def test_only_public_publication_job_has_repository_write_permissions(self):
        for job_id, job in self.workflow["jobs"].items():
            permissions = job.get("permissions", {})
            if any(value == "write" for value in permissions.values()):
                self.assertEqual("publish-public-analysis", job_id)


if __name__ == "__main__":
    unittest.main()
