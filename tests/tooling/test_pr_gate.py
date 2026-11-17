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
                ".github/workflows/fuzz.yml",
            },
        )

    def test_root_cargo_manifest_change_selects_fuzzing(self):
        self.assertIn(".github/workflows/fuzz.yml", expected_workflows(["Cargo.toml"]))

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

    def test_construct_probe_change_selects_docs_and_native_matrix(self):
        self.assertEqual(
            expected_workflows(["tests/language-constructs/fact-function/probe-3.5.LST"]),
            {
                ".github/workflows/docs.yml",
                ".github/workflows/amiga-construct-matrix.yml",
            },
        )

    def test_construct_validator_change_selects_docs_and_native_matrix(self):
        self.assertEqual(
            expected_workflows(["scripts/validate_language_constructs.py"]),
            {
                ".github/workflows/docs.yml",
                ".github/workflows/amiga-construct-matrix.yml",
            },
        )

    def test_native_matrix_workflow_change_selects_matrix_and_workflow_lint(self):
        self.assertEqual(
            expected_workflows([".github/workflows/amiga-construct-matrix.yml"]),
            {
                ".github/workflows/amiga-construct-matrix.yml",
                ".github/workflows/workflow-lint.yml",
            },
        )

    def test_reference_matrix_workflow_change_routes_to_construct_matrix_and_lint(self):
        self.assertEqual(
            expected_workflows([".github/workflows/amiga-reference-matrix.yml"]),
            {
                ".github/workflows/amiga-construct-matrix.yml",
                ".github/workflows/workflow-lint.yml",
            },
        )

    def test_interpreter_probe_change_keeps_full_suite_baseline(self):
        self.assertEqual(
            expected_workflows(["tests/interpreter-probes/amiga-3.52/probe.LST"]),
            {".github/workflows/amiga-reference-matrix.yml"},
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
