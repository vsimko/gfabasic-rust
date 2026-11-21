import unittest
from pathlib import Path

import yaml

from scripts.pr_gate import WORKFLOWS, expected_workflows, expected_workflows_for_files


FUZZ_WORKFLOW = ".github/workflows/fuzz.yml"
NATIVE_352_WORKFLOW = ".github/workflows/amiga-reference-matrix.yml"

class ChangedFileStatusTests(unittest.TestCase):
    def test_unclassified_deletions_do_not_require_workflow_runs(self):
        selected = expected_workflows_for_files(
            [{"filename": "tests/interpreter-probes/retired-profile/regression-suite.LST", "status": "removed"}]
        )
        self.assertEqual(selected, set())

    def test_deletions_matching_shared_workflow_paths_still_select_that_workflow(self):
        selected = expected_workflows_for_files(
            [{"filename": "docs/testing/retired-profile.md", "status": "removed"}]
        )
        self.assertEqual(selected, {".github/workflows/docs.yml"})

    def test_unclassified_additions_remain_conservative(self):
        selected = expected_workflows_for_files(
            [{"filename": "new/unclassified-input.bin", "status": "added"}]
        )
        self.assertEqual(selected, set(WORKFLOWS))


class Native352InterpreterProbePathTests(unittest.TestCase):
    def test_legacy_interpreter_probes_select_whole_suite_baseline(self):
        for path in (
            "tests/interpreter-probes/amiga-3.52/expected-ids.txt",
            "tests/interpreter-probes/amiga-3.52/regression-suite.LST",
            "tests/interpreter-probes/amiga-3.5/expected-ids.txt",
            "tests/interpreter-probes/amiga-3.5/regression-suite.LST",
        ):
            with self.subTest(path=path):
                self.assertEqual(expected_workflows([path]), {NATIVE_352_WORKFLOW})

    def test_native_harness_script_selects_construct_matrix_and_docs(self):
        self.assertEqual(
            expected_workflows(["scripts/amiga-cli-probe-35.sh"]),
            {
                ".github/workflows/amiga-construct-matrix.yml",
                ".github/workflows/docs.yml",
            },
        )

    def test_reference_matrix_paths_match_pr_gate_classification(self):
        root = Path(__file__).resolve().parents[2]
        workflow_path = root / ".github" / "workflows" / "amiga-reference-matrix.yml"
        workflow = yaml.load(workflow_path.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)
        configured_paths = set(workflow["on"]["pull_request"]["paths"])
        self.assertEqual(configured_paths, set(WORKFLOWS[NATIVE_352_WORKFLOW]))


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



class LanguageConstructManifestPathTests(unittest.TestCase):
    def test_manifest_and_probe_paths_select_docs_and_construct_matrix(self):
        for path in (
            "tests/language-constructs/README.md",
            "tests/language-constructs/fact-function/manifest.yaml",
            "tests/language-constructs/fact-function/probe-3.5.LST",
            "tests/language-constructs/mat-one-statement/probe-3.52.LST",
        ):
            with self.subTest(path=path):
                self.assertEqual(
                    expected_workflows([path]),
                    {".github/workflows/docs.yml", ".github/workflows/amiga-construct-matrix.yml"},
                )


class PdfSourceManifestPathTests(unittest.TestCase):
    def test_pdf_source_manifest_selects_documentation_workflow_only(self):
        selected = expected_workflows(["references/pdf-sources.json"])
        self.assertEqual(selected, {".github/workflows/docs.yml"})

if __name__ == "__main__":
    unittest.main()
