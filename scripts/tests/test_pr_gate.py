import unittest
from pathlib import Path

import yaml

from scripts.pr_gate import WORKFLOWS, expected_workflows


FUZZ_WORKFLOW = ".github/workflows/fuzz.yml"
NATIVE_WORKFLOW = ".github/workflows/amiga-35u-native-merge-run.yml"

class NativeInterpreterProbePathTests(unittest.TestCase):
    def test_profile_files_select_native_workflow_without_unrelated_fuzzing(self):
        for path in (
            "tests/interpreter-probes/amiga-3.5u/README.md",
            "tests/interpreter-probes/amiga-3.5u/expected-ids.txt",
            "tests/interpreter-probes/amiga-3.5u/regression-suite.LST",
        ):
            with self.subTest(path=path):
                selected = expected_workflows([path])
                self.assertIn(NATIVE_WORKFLOW, selected)
                self.assertNotIn(FUZZ_WORKFLOW, selected)

    def test_native_workflow_paths_match_pr_gate_classification(self):
        root = Path(__file__).resolve().parents[2]
        workflow_path = root / ".github" / "workflows" / "amiga-35u-native-merge-run.yml"
        workflow = yaml.load(workflow_path.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)
        configured_paths = set(workflow["on"]["pull_request"]["paths"])
        self.assertEqual(configured_paths, set(WORKFLOWS[NATIVE_WORKFLOW]))




class FuzzWorkflowPathTests(unittest.TestCase):
    def assert_fuzz_selected(self, path):
        self.assertIn(FUZZ_WORKFLOW, expected_workflows([path]), path)

    def assert_fuzz_not_selected(self, path):
        self.assertNotIn(FUZZ_WORKFLOW, expected_workflows([path]), path)

    def test_source_loader_and_compiler_dependency_paths_select_fuzzing(self):
        for path in (
            "crates/gfabasic-compiler/src/source.rs",
            "crates/gfabasic-compiler/src/lib.rs",
            "crates/gfabasic-compiler/src/frontend/mod.rs",
            "crates/gfabasic-compiler/Cargo.toml",
            "crates/gfabasic-runtime/src/core.rs",
            "crates/gfabasic-runtime/Cargo.toml",
            "Cargo.toml",
            "Cargo.lock",
            "rust-toolchain.toml",
        ):
            with self.subTest(path=path):
                self.assert_fuzz_selected(path)

    def test_harness_corpus_lockfile_and_policy_select_fuzzing(self):
        for path in (
            "fuzz/fuzz_targets/source_loader.rs",
            "fuzz/corpus/source_loader/seed",
            "fuzz/Cargo.toml",
            "fuzz/Cargo.lock",
            ".github/workflows/fuzz.yml",
            "docs/architecture/FUZZING.md",
        ):
            with self.subTest(path=path):
                self.assert_fuzz_selected(path)

    def test_unrelated_documentation_website_benchmarks_and_api_registry_do_not(self):
        for path in (
            "docs/architecture/README.md",
            "README.md",
            "site/index.html",
            "site/src/pages/index.astro",
            "bench/source_loader.rs",
            "api/amiga/3.5/functions.json",
            "scripts/pr_gate.py",
        ):
            with self.subTest(path=path):
                self.assert_fuzz_not_selected(path)

    def test_workflow_paths_match_pr_gate_classification(self):
        root = Path(__file__).resolve().parents[2]
        workflow_path = root / ".github" / "workflows" / "fuzz.yml"
        workflow = yaml.load(workflow_path.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)
        configured_paths = set(workflow["on"]["pull_request"]["paths"])
        self.assertEqual(configured_paths, set(WORKFLOWS[FUZZ_WORKFLOW]))

    def test_mixed_pull_request_selects_fuzz_only_for_relevant_paths(self):
        self.assertIn(
            FUZZ_WORKFLOW,
            expected_workflows(
                ["site/index.html", "crates/gfabasic-compiler/src/source.rs"]
            ),
        )
        self.assertNotIn(
            FUZZ_WORKFLOW,
            expected_workflows(["site/index.html", "api/amiga/3.5/functions.json"]),
        )


if __name__ == "__main__":
    unittest.main()
