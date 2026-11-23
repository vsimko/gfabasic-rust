from pathlib import Path
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]


def load_workflow(path):
    return yaml.load(path.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)


class NativeMatrixWorkflowBoundaryTests(unittest.TestCase):
    def test_3041u_construct_id_file_uses_a_real_newline(self):
        workflow = load_workflow(
            ROOT / ".github/workflows/amiga-3041u-native-merge-run.yml"
        )
        steps = workflow["jobs"]["cli-probe"]["steps"]
        prepare = next(
            step
            for step in steps
            if step.get("name") == "Prepare DF0 test disk and preserve the original GFA BASIC disk in DF1"
        )
        run = prepare["run"]
        self.assertIn(
            r'''printf '%s\n' "$GFA_CONSTRUCT_ID_INPUT" > "$RUNNER_TEMP/expected-ids.txt"''',
            run,
        )
        self.assertNotIn(
            r'''printf '%s\\n' "$GFA_CONSTRUCT_ID_INPUT"''',
            run,
        )
        self.assertIn(
            r'''test "$(wc -l < "$RUNNER_TEMP/expected-ids.txt")" -eq 1''',
            run,
        )
        self.assertIn(
            r'''printf '%s\n' "DF1:gfabasic"''',
            run,
        )
        self.assertNotIn(
            r'''printf '%s\\n' "DF1:gfabasic"''',
            run,
        )

    def test_35_private_asset_token_is_confined_to_download_steps(self):
        workflow = load_workflow(
            ROOT / ".github/workflows/amiga-reference-matrix.yml"
        )
        job = workflow["jobs"]["amiga-35"]
        self.assertNotIn("GFA_ASSETS_TOKEN", job.get("env", {}))

        steps = job["steps"]
        secret_steps = [
            step
            for step in steps
            if "GFA_ASSETS_TOKEN" in step.get("env", {})
        ]
        self.assertTrue(secret_steps)
        for step in secret_steps:
            self.assertIn("Download", step.get("name", ""))

        for name in (
            "Capture the running editor version from its right-button menu",
            "Prepare writable 3.5 test disk",
            "Merge and run the 3.5 BASIC probe",
            "Write per-construct result record",
        ):
            step = next(step for step in steps if step.get("name") == name)
            self.assertNotIn("GFA_ASSETS_TOKEN", step.get("env", {}))


if __name__ == "__main__":
    unittest.main()
