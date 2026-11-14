import unittest

from scripts.pr_gate import expected_workflows


FUZZ_WORKFLOW = ".github/workflows/fuzz.yml"


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
        ):
            with self.subTest(path=path):
                self.assert_fuzz_not_selected(path)

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
