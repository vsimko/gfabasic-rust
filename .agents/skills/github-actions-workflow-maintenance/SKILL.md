---
name: github-actions-workflow-maintenance
description: Audit, modify, and troubleshoot GitHub Actions workflows in this repository, including triggers, permissions, concurrency, CI evidence, artifacts, and agent-skill validation.
---

# GitHub Actions workflow maintenance

Use this skill before changing, diagnosing, or reviewing workflows under `.github/workflows/`.

## Required context

1. Read `AGENTS.md` and follow the repository's issue/blocker policy.
2. Inspect all workflow files, not only the workflow that appears to have failed.
3. Read the relevant scripts, validators, fixtures, and documentation used by each workflow.
4. Inspect recent workflow runs for the affected commit and compare successful and failed runs.

## Audit procedure

### 1. Inventory and triggers

- Enumerate every `.yml` and `.yaml` file under `.github/workflows/`.
- For each workflow, record its events, branch filters, path filters, permissions, concurrency group, runner, timeout, jobs, artifacts, and external actions.
- Verify that every source file which materially affects a workflow is included in its `push.paths` and `pull_request.paths` filters.
- Check that shared scripts, validators, fixtures, and emulator harnesses are not omitted from path filters.
- Confirm that a workflow which validates workflow definitions itself runs when any workflow file changes.
- Check whether a path-filtered workflow can leave a required status pending or unexpectedly skip a relevant change.

### 2. Security and reliability

- Prefer least-privilege top-level permissions and narrower job-level permissions for write operations.
- Pin third-party actions to full commit SHAs; use comments or documentation to identify the pinned release.
- Never expose private-asset credentials to untrusted fork pull requests.
- Keep licensed binaries, ROMs, disk images, and private source assets out of committed files and uploaded artifacts.
- Use explicit timeouts and concurrency groups where duplicate runs can waste resources or race on shared deployment targets.
- Make artifact uploads run with `if: always()` when failure diagnostics are needed, and exclude private or licensed payloads explicitly.
- Pin tool and dependency versions used in CI; verify downloaded assets with trusted hashes when available.
- Do not broaden permissions or weaken checks merely to make a failing workflow green.

### 3. Run diagnosis

For a failing run:

1. Resolve the exact run, commit SHA, attempt, job, and failed step.
2. Read the complete job log and inspect artifacts from that same run.
3. Separate infrastructure/setup failures, harness failures, test-program failures, and assertion failures.
4. Compare the failed run with the latest successful run for the same workflow and target version.
5. Reproduce with the smallest safe change and rerun the exact workflow.
6. Do not report a fix as verified while the replacement run is queued or in progress.

For native Amiga runs, follow the `amiga-gfa-basic-regression-testing` skill for emulator and fixture diagnosis.

### 4. Validation

- Run `actionlint` against every workflow file, not a hand-picked subset.
- Run `bash -n` against every repository shell script affected by the change.
- Run `python scripts/validate_agent_skills.py` when skill metadata, links, discovery entries, or skill instructions change.
- Run the workflow and inspect its completed result after pushing.
- Keep workflow summaries and PR descriptions explicit about skipped jobs, missing credentials, unavailable licensed assets, and unverified behavior.

## Repository-specific invariants

- `main` is the production branch; preview and production deployments must remain isolated.
- PR preview jobs that write to repository Pages must not run for untrusted fork code.
- Private Amiga assets are fetched only through the private asset repository and must be hash-verified where a trusted hash is recorded.
- Native interpreter evidence is version-specific. Never combine 3.041U, 3.5U, and 3.52 results.
- The host-side GFATEST validator checks protocol completeness; it does not prove that the interpreter semantics are correct.
- A missing output file is a harness/program-completion symptom until logs, screenshots, file opens, drive mapping, and source execution have been inspected.

## Completion report

Report the workflow files reviewed, exact changes, validation commands, run URLs and conclusions, and unresolved risks. Distinguish a queued/running run from a completed green run.
