"""Static regression checks for the dedicated Rust mutation workflow."""
from pathlib import Path
import re
import unittest


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
        cls.text = WORKFLOW_PATH.read_text(encoding="utf-8")

    def test_schedule_is_exactly_one_daily_0200_utc_entry(self):
        cron_values = re.findall(r"^\s+- cron:\s*['\"]([^'\"]+)['\"]\s*$", self.text, re.MULTILINE)
        self.assertEqual(cron_values, [EXPECTED_CRON])

    def test_only_schedule_and_manual_dispatch_are_trigger_events(self):
        events = re.search(r"(?ms)^on:\n(.*?)^permissions:", self.text)
        self.assertIsNotNone(events)
        event_names = re.findall(r"^  ([a-z_]+):", events.group(1), re.MULTILINE)
        self.assertEqual(set(event_names), {"schedule", "workflow_dispatch"})

    def test_job_guard_accepts_manual_dispatch_and_exact_schedule_payload(self):
        match = re.search(r"(?m)^    if: >-\n((?:^      .*\n)+)", self.text)
        self.assertIsNotNone(match)
        condition = " ".join(line.strip() for line in match.group(1).splitlines())
        self.assertEqual(condition, EXPECTED_JOB_GUARD)

    def test_previous_combined_and_dst_specific_schedules_are_absent(self):
        self.assertNotIn("0 1,2 * * *", self.text)
        self.assertNotIn("0 1 * * *", self.text)


if __name__ == "__main__":
    unittest.main()
