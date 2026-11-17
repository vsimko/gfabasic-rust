from __future__ import annotations

import unittest

from scripts.validate_agent_security import validate_document


class AgentSecurityValidationTests(unittest.TestCase):
    def test_rejects_pull_request_target(self):
        document = {
            "on": {"pull_request_target": {}},
            "permissions": {"contents": "read"},
            "jobs": {},
        }
        self.assertTrue(any("pull_request_target" in error for error in validate_document(document, "test.yml")))

    def test_requires_same_repository_guard_for_pr_secret(self):
        document = {
            "on": {"pull_request": {}},
            "permissions": {"contents": "read"},
            "jobs": {"probe": {"env": {"TOKEN": "${{ secrets.PRIVATE_TOKEN }}"}}},
        }
        self.assertTrue(any("same-repository guard" in error for error in validate_document(document, "test.yml")))

    def test_detects_secret_in_step_environment(self):
        document = {
            "on": {"pull_request": {}},
            "permissions": {"contents": "read"},
            "jobs": {
                "probe": {
                    "steps": [{"env": {"TOKEN": "${{ secrets.PRIVATE_TOKEN }}"}}],
                }
            },
        }
        self.assertTrue(any("same-repository guard" in error for error in validate_document(document, "test.yml")))

    def test_accepts_same_repository_guard_for_pr_secret(self):
        document = {
            "on": {"pull_request": {}},
            "permissions": {"contents": "read"},
            "jobs": {
                "probe": {
                    "if": "github.event.pull_request.head.repo.full_name == github.repository",
                    "env": {"TOKEN": "${{ secrets.PRIVATE_TOKEN }}"},
                }
            },
        }
        self.assertEqual([], validate_document(document, "test.yml"))

    def test_requires_same_repository_guard_for_pr_write(self):
        document = {
            "on": {"pull_request": {}},
            "permissions": {"contents": "write"},
            "jobs": {"publish": {}},
        }
        self.assertTrue(any("same-repository guard" in error for error in validate_document(document, "test.yml")))

    def test_requires_main_guard_for_manual_secret_access(self):
        document = {
            "on": {"workflow_dispatch": {}},
            "permissions": {"contents": "read"},
            "jobs": {"probe": {"env": {"TOKEN": "${{ secrets.PRIVATE_TOKEN }}"}}},
        }
        self.assertTrue(any("main-ref guard" in error for error in validate_document(document, "test.yml")))

    def test_accepts_main_guard_for_manual_secret_access(self):
        document = {
            "on": {"workflow_dispatch": {}},
            "permissions": {"contents": "read"},
            "jobs": {
                "probe": {
                    "if": "github.event_name != 'workflow_dispatch' || github.ref == 'refs/heads/main'",
                    "env": {"TOKEN": "${{ secrets.PRIVATE_TOKEN }}"},
                }
            },
        }
        self.assertEqual([], validate_document(document, "test.yml"))

    def test_ignores_write_job_restricted_to_release_event(self):
        document = {
            "on": {"pull_request": {}, "release": {}},
            "permissions": {"contents": "read"},
            "jobs": {"publish": {"if": "github.event_name == 'release'", "permissions": {"contents": "write"}}},
        }
        self.assertEqual([], validate_document(document, "test.yml"))

    def test_ignores_pr_cleanup_job_for_manual_dispatch(self):
        document = {
            "on": {"pull_request": {}, "workflow_dispatch": {}},
            "permissions": {"contents": "read"},
            "jobs": {
                "cleanup": {
                    "if": "github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository",
                    "permissions": {"actions": "write"},
                }
            },
        }
        self.assertEqual([], validate_document(document, "test.yml"))

    def test_requires_explicit_workflow_permissions(self):
        document = {"on": {"push": {}}, "jobs": {}}
        self.assertTrue(any("permissions must be explicit" in error for error in validate_document(document, "test.yml")))


if __name__ == "__main__":
    unittest.main()
