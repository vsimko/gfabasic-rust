"""Static regression checks for the dedicated Rust mutation workflow."""
from pathlib import Path
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW_PATH = ROOT / ".github" / "workflows" / "rust-mutation.yml"
EXPECTED_CRON = "0 2 * * *"
EXPECTED_JOB_GUARD = (
    "github.event_name == 'workflow_dispatch' || "
    "(github.event_name == 'schedule' && github.event.schedule == '0 2 * * *')"
)


class RustMutationWorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = yaml.load(WORKFLOW_PATH.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)

    def test_schedule_is_exactly_one_daily_0200_utc_entry(self):
        schedules = self.workflow["on"]["schedule"]
        self.assertEqual(schedules, [{"cron": EXPECTED_CRON}])

    def test_only_schedule_and_manual_dispatch_are_trigger_events(self):
        self.assertEqual(set(self.workflow["on"]), {"schedule", "workflow_dispatch"})

    def test_job_guard_accepts_manual_dispatch_and_exact_schedule_payload(self):
        condition = " ".join(self.workflow["jobs"]["mutants"]["if"].split())
        self.assertEqual(condition, EXPECTED_JOB_GUARD)

    def test_previous_combined_and_dst_specific_schedules_are_absent(self):
        cron_values = [entry["cron"] for entry in self.workflow["on"]["schedule"]]
        self.assertNotIn("0 1,2 * * *", cron_values)
        self.assertNotIn("0 1 * * *", cron_values)


if __name__ == "__main__":
    unittest.main()
