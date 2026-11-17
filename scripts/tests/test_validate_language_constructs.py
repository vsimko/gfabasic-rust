from __future__ import annotations

import contextlib
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest

import yaml

from scripts.validate_language_constructs import (
    FileChange,
    main,
    parse_name_status_z,
    select_jobs,
    validate_language_constructs,
)


ROOT = Path(__file__).resolve().parents[2]


def copy_test_root(destination: Path) -> Path:
    """Copy only repository paths referenced by the sample manifests."""
    shutil.copytree(ROOT / "tests" / "language-constructs", destination / "tests" / "language-constructs")
    shutil.copytree(ROOT / "tests" / "interpreter-probes", destination / "tests" / "interpreter-probes")
    shutil.copytree(ROOT / "crates" / "gfabasic-compiler" / "src", destination / "crates" / "gfabasic-compiler" / "src")
    return destination


class LanguageConstructManifestValidationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = copy_test_root(Path(self.temporary.name))

    def tearDown(self):
        self.temporary.cleanup()

    def manifest_path(self, construct_id: str = "fact-function") -> Path:
        return self.root / "tests" / "language-constructs" / construct_id / "manifest.yaml"

    def load_manifest(self, construct_id: str = "fact-function") -> dict:
        return yaml.safe_load(self.manifest_path(construct_id).read_text(encoding="utf-8"))

    def write_manifest(self, manifest: dict, construct_id: str = "fact-function") -> None:
        self.manifest_path(construct_id).write_text(
            yaml.safe_dump(manifest, sort_keys=False, allow_unicode=True),
            encoding="utf-8",
        )

    def test_repository_manifests_validate(self):
        self.assertEqual(validate_language_constructs(ROOT), [])

    def test_duplicate_yaml_key_fails(self):
        path = self.manifest_path()
        path.write_text(
            path.read_text(encoding="utf-8") + "\nmanifest_schema_version: 1\n",
            encoding="utf-8",
        )
        self.assertTrue(any("duplicate key" in error for error in validate_language_constructs(self.root)))

    def test_malformed_yaml_fails(self):
        self.manifest_path().write_text("manifest_schema_version: [\n", encoding="utf-8")
        self.assertTrue(any("invalid YAML" in error for error in validate_language_constructs(self.root)))

    def test_duplicate_construct_id_fails(self):
        duplicate = self.root / "tests" / "language-constructs" / "duplicate-fact"
        shutil.copytree(
            self.root / "tests" / "language-constructs" / "fact-function",
            duplicate,
        )
        errors = validate_language_constructs(self.root)
        self.assertTrue(any("duplicate construct ID 'fact-function'" in error for error in errors))

    def test_directory_and_construct_id_must_match(self):
        duplicate = self.root / "tests" / "language-constructs" / "other-name"
        shutil.copytree(
            self.root / "tests" / "language-constructs" / "fact-function",
            duplicate,
        )
        errors = validate_language_constructs(self.root)
        self.assertTrue(any("must match directory name" in error for error in errors))

    def test_missing_manifest_fails(self):
        directory = self.root / "tests" / "language-constructs" / "missing-manifest"
        directory.mkdir()
        self.assertTrue(any("missing manifest.yaml" in error for error in validate_language_constructs(self.root)))

    def test_missing_probe_fails(self):
        (self.root / "tests" / "language-constructs" / "fact-function" / "probe-3.5.LST").unlink()
        self.assertTrue(any("does not exist as a regular file" in error for error in validate_language_constructs(self.root)))

    def test_unreferenced_probe_fails(self):
        (self.root / "tests" / "language-constructs" / "fact-function" / "extra.LST").write_text("PRINT 1\n", encoding="ascii")
        self.assertTrue(any("not referenced by any version" in error for error in validate_language_constructs(self.root)))

    def test_path_traversal_fails(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["probe"] = "../probe.LST"
        self.write_manifest(manifest)
        self.assertTrue(any("safe relative POSIX path" in error for error in validate_language_constructs(self.root)))

    def test_absolute_probe_path_fails(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["probe"] = "/tmp/probe.LST"
        self.write_manifest(manifest)
        self.assertTrue(any("safe relative POSIX path" in error for error in validate_language_constructs(self.root)))

    def test_backslash_probe_path_fails(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["probe"] = r"subdir\\probe.LST"
        self.write_manifest(manifest)
        self.assertTrue(any("safe relative POSIX path" in error for error in validate_language_constructs(self.root)))

    def test_symlink_probe_fails(self):
        directory = self.root / "tests" / "language-constructs" / "fact-function"
        probe = directory / "probe-3.5.LST"
        probe.unlink()
        probe.symlink_to(directory / "probe-3.52.LST")
        self.assertTrue(any("must not traverse symlinks" in error for error in validate_language_constructs(self.root)))

    def test_unknown_version_key_fails(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.9"] = manifest["versions"]["3.5"]
        self.write_manifest(manifest)
        errors = validate_language_constructs(self.root)
        self.assertTrue(any("unknown version key '3.9'" in error for error in errors))

    def test_missing_version_key_fails(self):
        manifest = self.load_manifest()
        del manifest["versions"]["3.52"]
        self.write_manifest(manifest)
        self.assertTrue(any("missing exact version key '3.52'" in error for error in validate_language_constructs(self.root)))

    def test_unknown_syntax_version_fails(self):
        manifest = self.load_manifest()
        manifest["construct"]["syntax_forms"][0]["versions"].append("3.9")
        self.write_manifest(manifest)
        self.assertTrue(any("contains unknown version '3.9'" in error for error in validate_language_constructs(self.root)))

    def test_dangling_evidence_reference_fails(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["expectation"]["evidence_refs"] = ["missing-evidence"]
        self.write_manifest(manifest)
        self.assertTrue(any("references unknown evidence ID" in error for error in validate_language_constructs(self.root)))

    def test_untested_execution_cannot_claim_output(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["execution"]["observed"] = "FACT(0)=1"
        self.write_manifest(manifest)
        self.assertTrue(any("must be null when UNTESTED" in error for error in validate_language_constructs(self.root)))

    def test_native_result_requires_source_and_run(self):
        manifest = self.load_manifest()
        execution = manifest["versions"]["3.5"]["execution"]
        execution["status"] = "PASS"
        execution["observed"] = "PASS"
        execution["run_url"] = None
        execution["probe_source"] = None
        execution["evidence_refs"] = []
        self.write_manifest(manifest)
        errors = validate_language_constructs(self.root)
        self.assertTrue(any("run_url is required" in error for error in errors))
        self.assertTrue(any("probe_source is required" in error for error in errors))
        self.assertTrue(any("must not be empty" in error for error in errors))

    def test_implementation_reference_must_exist(self):
        manifest = self.load_manifest()
        manifest["versions"]["3.5"]["implementation"]["refs"] = ["crates/missing.rs"]
        self.write_manifest(manifest)
        self.assertTrue(any("does not exist as a regular file" in error for error in validate_language_constructs(self.root)))

    def test_unknown_kind_fails(self):
        manifest = self.load_manifest()
        manifest["construct"]["kind"] = "mystery"
        self.write_manifest(manifest)
        self.assertTrue(any("construct.kind must be one of" in error for error in validate_language_constructs(self.root)))


class LanguageConstructSelectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = copy_test_root(Path(self.temporary.name))

    def tearDown(self):
        self.temporary.cleanup()

    def select(self, *changes, **kwargs):
        result, errors = select_jobs(self.root, list(changes), **kwargs)
        self.assertEqual(errors, [])
        self.assertIsNotNone(result)
        return result

    def test_changed_manifest_selects_only_its_applicable_versions(self):
        result = self.select(FileChange("M", new_path="tests/language-constructs/fact-function/manifest.yaml"))
        self.assertEqual(result["changed_constructs"], ["fact-function"])
        self.assertEqual([item["version"] for item in result["include"]], ["3.5", "3.52"])

    def test_changed_probe_selects_only_its_version(self):
        result = self.select(FileChange("M", new_path="tests/language-constructs/fact-function/probe-3.5.LST"))
        self.assertEqual([(item["construct_id"], item["version"]) for item in result["include"]], [("fact-function", "3.5")])

    def test_shared_schema_change_validates_all_but_schedules_no_native_jobs(self):
        result = self.select(FileChange("M", new_path="tests/language-constructs/README.md"))
        self.assertTrue(result["full_manifest_validation"])
        self.assertTrue(result["shared_metadata_changed"])
        self.assertEqual(result["selection"], "none")
        self.assertEqual(result["include"], [])

    def test_validator_change_does_not_schedule_all_native_jobs(self):
        result = self.select(FileChange("M", new_path="scripts/validate_language_constructs.py"))
        self.assertEqual(result["include"], [])
        self.assertEqual(result["selection"], "none")

    def test_multiple_changed_constructs_are_sorted_and_deduplicated(self):
        changes = [
            FileChange("M", new_path="tests/language-constructs/mat-one-statement/manifest.yaml"),
            FileChange("M", new_path="tests/language-constructs/fact-function/manifest.yaml"),
            FileChange("M", new_path="tests/language-constructs/fact-function/probe-3.5.LST"),
        ]
        result = self.select(*changes)
        self.assertEqual(result["changed_constructs"], ["fact-function", "mat-one-statement"])
        self.assertEqual(
            [(item["construct_id"], item["version"]) for item in result["include"]],
            [
                ("fact-function", "3.5"),
                ("fact-function", "3.52"),
                ("mat-one-statement", "3.5"),
                ("mat-one-statement", "3.52"),
            ],
        )

    def test_deleted_construct_does_not_schedule_native_jobs(self):
        shutil.rmtree(self.root / "tests" / "language-constructs" / "fact-function")
        result = self.select(
            FileChange("D", new_path="tests/language-constructs/fact-function/manifest.yaml")
        )
        self.assertEqual(result["include"], [])
        self.assertEqual(result["selection"], "none")

    def test_renamed_construct_selects_only_new_directory(self):
        catalogue = self.root / "tests" / "language-constructs"
        old = catalogue / "fact-function"
        new = catalogue / "factorial-function"
        shutil.copytree(old, new)
        manifest_path = new / "manifest.yaml"
        manifest = yaml.safe_load(manifest_path.read_text(encoding="utf-8"))
        manifest["construct"]["id"] = "factorial-function"
        manifest_path.write_text(yaml.safe_dump(manifest, sort_keys=False), encoding="utf-8")
        shutil.rmtree(old)
        result = self.select(
            FileChange(
                "R100",
                old_path="tests/language-constructs/fact-function/manifest.yaml",
                new_path="tests/language-constructs/factorial-function/manifest.yaml",
            )
        )
        self.assertEqual(result["changed_constructs"], ["factorial-function"])
        self.assertEqual({item["construct_id"] for item in result["include"]}, {"factorial-function"})

    def test_added_construct_selects_only_its_declared_versions(self):
        catalogue = self.root / "tests" / "language-constructs"
        source = catalogue / "fact-function"
        target = catalogue / "fact-function-copy"
        shutil.copytree(source, target)
        manifest_path = target / "manifest.yaml"
        manifest = yaml.safe_load(manifest_path.read_text(encoding="utf-8"))
        manifest["construct"]["id"] = "fact-function-copy"
        manifest_path.write_text(yaml.safe_dump(manifest, sort_keys=False), encoding="utf-8")
        result = self.select(
            FileChange("A", new_path="tests/language-constructs/fact-function-copy/manifest.yaml")
        )
        self.assertEqual(result["changed_constructs"], ["fact-function-copy"])
        self.assertEqual({item["construct_id"] for item in result["include"]}, {"fact-function-copy"})

    def test_manual_dispatch_all_is_explicit_and_deterministic(self):
        result = self.select(run_all=True)
        self.assertEqual(result["selection"], "all")
        self.assertEqual(
            [(item["construct_id"], item["version"]) for item in result["include"]],
            [
                ("fact-function", "3.5"),
                ("fact-function", "3.52"),
                ("mat-one-statement", "3.5"),
                ("mat-one-statement", "3.52"),
            ],
        )

    def test_manual_dispatch_can_select_one_construct_and_version(self):
        result = self.select(construct_id="fact-function", versions=["3.52"])
        self.assertEqual(result["selection"], "manual")
        self.assertEqual([(item["construct_id"], item["version"]) for item in result["include"]], [("fact-function", "3.52")])

    def test_manual_dispatch_rejects_unknown_construct_or_version(self):
        result, errors = select_jobs(self.root, construct_id="missing")
        self.assertIsNone(result)
        self.assertTrue(any("unknown construct ID" in error for error in errors))
        result, errors = select_jobs(self.root, construct_id="fact-function", versions=["3.041u"])
        self.assertIsNone(result)
        self.assertTrue(any("unknown version identifier" in error for error in errors))

    def test_git_name_status_parser_handles_add_modify_delete_and_rename(self):
        output = (
            "A\0tests/language-constructs/new/manifest.yaml\0"
            "M\0tests/language-constructs/fact-function/manifest.yaml\\0"
            "D\0tests/language-constructs/old/manifest.yaml\\0"
            "R100\0tests/language-constructs/old/manifest.yaml\\0"
            "tests/language-constructs/new/manifest.yaml\\0"
        )
        changes = parse_name_status_z(output)
        self.assertEqual([item.status for item in changes], ["A", "M", "D", "R100"])
        self.assertEqual(changes[-1].old_path, "tests/language-constructs/old/manifest.yaml")
        self.assertEqual(changes[-1].new_path, "tests/language-constructs/new/manifest.yaml")

    def test_selection_output_is_independent_of_input_order(self):
        first = self.select(
            FileChange("M", new_path="tests/language-constructs/mat-one-statement/manifest.yaml"),
            FileChange("M", new_path="tests/language-constructs/fact-function/manifest.yaml"),
        )
        second = self.select(
            FileChange("M", new_path="tests/language-constructs/fact-function/manifest.yaml"),
            FileChange("M", new_path="tests/language-constructs/mat-one-statement/manifest.yaml"),
        )
        self.assertEqual(first, second)


class LanguageConstructCliTests(unittest.TestCase):
    def test_github_output_is_machine_readable(self):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            result = main(["select", "--root", str(ROOT), "--construct", "fact-function", "--version", "3.5", "--format", "github"])
        self.assertEqual(result, 0)
        lines = output.getvalue().splitlines()
        matrix = json.loads(lines[0].split("=", 1)[1])
        self.assertEqual(matrix["include"][0]["construct_id"], "fact-function")
        self.assertEqual(matrix["include"][0]["version"], "3.5")
        self.assertEqual(lines[1], "selection=manual")
        self.assertEqual(lines[2], "full_manifest_validation=true")


if __name__ == "__main__":
    unittest.main()
