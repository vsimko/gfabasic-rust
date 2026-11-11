import unittest

from scripts.pr_gate import WORKFLOWS, expected_workflows, workflow_result


class PullRequestGateTests(unittest.TestCase):
    def test_documentation_change_selects_documentation_workflow(self):
        self.assertEqual(
            expected_workflows(["docs/architecture/CI.md"]),
            {".github/workflows/docs.yml"},
        )

    def test_root_markdown_file_selects_documentation_workflow(self):
        self.assertEqual(
            expected_workflows(["README.md"]),
            {".github/workflows/docs.yml"},
        )

    def test_cargo_change_selects_all_dependency_and_rust_checks(self):
        self.assertEqual(
            expected_workflows(["Cargo.lock"]),
            {
                ".github/workflows/rust.yml",
                ".github/workflows/security-audit.yml",
                ".github/workflows/release-sbom.yml",
            },
        )

    def test_source_loader_change_selects_rust_fuzz_and_benchmarks(self):
        self.assertEqual(
            expected_workflows(["crates/gfabasic-compiler/src/source.rs"]),
            {
                ".github/workflows/rust.yml",
                ".github/workflows/fuzz.yml",
                ".github/workflows/benchmarks.yml",
                ".github/workflows/release-sbom.yml",
            },
        )

    def test_unknown_path_fails_closed_to_all_workflows(self):
        self.assertEqual(
            expected_workflows(["new/unknown/component.file"]),
            set(WORKFLOWS),
        )

    def test_empty_file_list_selects_all_workflows(self):
        self.assertEqual(expected_workflows([]), set(WORKFLOWS))

    def test_only_successful_completed_run_passes(self):
        path = ".github/workflows/docs.yml"
        base = {"path": path + "@refs/pull/1/merge", "event": "pull_request"}
        self.assertEqual(
            workflow_result(path, [{**base, "status": "completed", "conclusion": "success"}]),
            "success",
        )
        self.assertEqual(
            workflow_result(path, [{**base, "status": "completed", "conclusion": "skipped"}]),
            "skipped",
        )
        self.assertEqual(
            workflow_result(path, [{**base, "status": "in_progress"}]),
            "pending",
        )
        self.assertEqual(workflow_result(path, []), "missing")


if __name__ == "__main__":
    unittest.main()
