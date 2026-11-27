---
name: github-actions-workflow-maintenance
description: Audit, modify, and troubleshoot GitHub Actions workflows, including construct-scoped native interpreter matrices and CI evidence.
---

# GitHub Actions workflow maintenance

Use this skill before changing, diagnosing, or reviewing workflows under `.github/workflows/`.

## Required context

1. Read `AGENTS.md` and follow the repository issue/blocker policy.
2. Inspect all workflow files, not only the workflow that appears to have failed.
3. Read the relevant scripts, validators, fixtures, and documentation used by each workflow.
4. Inspect recent workflow runs for the affected commit and compare successful and failed runs.
5. For construct-scoped Amiga GFA-BASIC runs, read [ADR 0018](../../../docs/architecture/adr/0018-construct-scoped-amiga-gfa-basic-probes.md), [ADR 0017](../../../docs/architecture/adr/0017-multi-version-amiga-gfa-basic-conformance.md), and the regression-testing skill.

## Audit procedure

### 1. Inventory and triggers

- Enumerate every `.yml` and `.yaml` file under `.github/workflows/`.
- For each workflow, record its events, branch filters, path filters, permissions, concurrency group, runner, timeout, jobs, artifacts, and external actions.
- Verify that every source file which materially affects a workflow is included in its `push.paths` and `pull_request.paths` filters.
- Confirm that a workflow which validates workflow definitions itself runs when any workflow file changes.
- Check whether a path-filtered workflow can leave a required status pending or unexpectedly skip a relevant change.

### 2. Construct-scoped native execution

For the construct catalogue defined by ADR 0018:

- Read and validate each changed `tests/language-constructs/<id>/manifest.yaml` before producing a matrix.
- Select only changed construct IDs and the exact interpreter versions declared by each manifest.
- Emit one independent matrix entry/job for each selected construct/version pair. Jobs for different versions should run in parallel.
- Do not schedule unrelated constructs. A schema/selector change can validate all manifests without automatically scheduling every native probe.
- Keep exact-version results separate and preserve `PASS`, `FAIL`, `DIFFERENT`, and `UNTESTED`.
- Treat missing assets, unavailable runners, and skipped version profiles as `UNTESTED`, not PASS.
- Upload raw output and screenshots as workflow artifacts. Do not commit generated run results as canonical evidence.
- Retain a manual whole-suite path for baseline and diagnosis while the construct-scoped system is introduced.

### 3. Security and reliability

- Prefer least-privilege top-level permissions and narrower job-level permissions for write operations.
- Pin third-party actions to full commit SHAs; use comments or documentation to identify the pinned release.
- Never expose private-asset credentials to untrusted fork pull requests.
- Keep licensed binaries, ROMs, disk images, and private source assets out of committed files and uploaded artifacts.
- Use explicit timeouts and concurrency groups where duplicate runs can waste resources or race on shared deployment targets.
- Make artifact uploads run with `if: always()` when failure diagnostics are needed, and exclude private or licensed payloads explicitly.
- Pin tool and dependency versions used in CI; verify downloaded assets with trusted hashes when available.
- Do not broaden permissions or weaken checks merely to make a failing workflow green.

### 4. Dependency caching (required)

Every workflow that installs or downloads reusable build/test dependencies MUST use a sensible cache unless the dependency is tiny, ephemeral, supplied by the runner, or caching is demonstrably unsafe. Treat caching as a default design requirement, not an optional later optimization.

Before adding or changing a workflow:

1. Identify every dependency download/install step (OS packages, Python packages, Node packages, Rust crates/toolchains, browser or emulator tooling, SDKs, and build outputs).
2. Reuse the repository's established cache action/version and patterns where suitable. Pin cache actions to full commit SHAs, just like every other third-party action.
3. Cache package-manager download stores rather than installed system directories:
   - Rust: Cargo registry/git plus build output only when the key and restore behavior are appropriate; use an additional cache action only if approved in the repository, otherwise follow the existing actions/cache pattern.
   - Python: pip's download/wheel cache (normally ~/.cache/pip).
   - Node: npm/yarn/pnpm package cache, preferably via setup-node's built-in cache when its lockfile support is clear.
   - APT: archive cache (including its partial directory), not the mutable installed package database; run apt-get update and install normally.
   - Other ecosystems: cache the official package-manager download/cache directory, not credentials or machine-specific state.
4. Build keys from the operating system/runner image, relevant runtime or toolchain version, and dependency lockfiles or package manifests. For APT or workflows without lockfiles, include a deliberate cache schema/version and hash the workflow/package list that defines installed packages. Use restore prefixes only for compatible older caches.
5. Keep cache paths narrow and deterministic. Never cache secrets, tokens, private licensed assets, emulator disk images, generated test results, or mutable machine state.
6. Do not let a cache become a correctness dependency: installs must still work on a cold cache, cache misses and eviction must be harmless, and package installation/update commands must still validate/install required packages.
7. Avoid pointless caches and explain justified exceptions.
8. For matrix/reusable workflows, ensure cache keys do not cause incompatible jobs to share state; include version/architecture/toolchain dimensions where relevant.

During review, check cache keys, restore prefixes, cold-cache correctness, and matrix isolation.

### 5. Run diagnosis

1. Resolve the exact run, commit SHA, attempt, job, and failed step.
2. Read the complete job log and inspect artifacts from that same run.
3. Separate infrastructure/setup failures, harness failures, test-program failures, and assertion failures.
4. Compare the failed run with the latest successful run for the same workflow and target version.
5. Reproduce with the smallest safe change and rerun the exact workflow.
6. Do not report a fix as verified while the replacement run is queued or in progress.

For native Amiga runs, also follow the regression-testing skill.

### 6. Validation

- Run `actionlint` against every workflow file, not a hand-picked subset.
- Run `bash -n` against every repository shell script affected by the change.
- Run `python scripts/validate_agent_skills.py` when skill metadata, links, discovery entries, or skill instructions change.
- Run the workflow and inspect its completed result after pushing.
- Keep workflow summaries explicit about skipped jobs, missing credentials, unavailable licensed assets, and unverified behavior.

## Repository-specific invariants

- `main` is the production branch; preview and production deployments must remain isolated.
- PR preview jobs that write to repository Pages must not run for untrusted fork code.
- Private Amiga assets are fetched only through the private asset repository and must be hash-verified where a trusted hash is recorded.
- Native interpreter evidence is version-specific. Never combine 3.041U, 3.5U, and 3.52 results.
- The host-side GFATEST validator checks protocol completeness; it does not prove that interpreter semantics are correct.
- A missing output file is a harness/program-completion symptom until logs, screenshots, file opens, drive mapping, and source execution have been inspected.

## Completion report

Report workflow files reviewed, exact changes, validation commands, run URLs and conclusions, and unresolved risks. Distinguish queued/running runs from completed green runs.
