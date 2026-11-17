---
name: amiga-gfa-basic-regression-testing
description: Create, debug, and validate version-specific probes for original Amiga GFA BASIC reference interpreters, with 3.52 as the active primary runtime. Use when editing .LST fixtures, investigating native Amiberry Merge + Run behavior, or comparing language behavior across releases.
---

# Amiga GFA BASIC regression testing

Use this skill when working on regression probes for the original Amiga GFA BASIC interpreters.

## Required context

1. Read `AGENTS.md` and follow the repository agent workflow.
2. Read [the probe-writing guide](../../../docs/testing/amiga-gfa-basic-probes.md).
3. Read [ADR 0017](../../../docs/architecture/adr/0017-multi-version-amiga-gfa-basic-conformance.md).
4. Read the applicable version-specific testing guide and inspect the native workflow that will execute the probe.
5. Identify the exact interpreter version, platform, binary/asset source, and manual edition. Never assume that observations transfer between versions. Check ADR 0017, current workflow files, and the latest relevant issue/CI run before selecting a runtime profile; do not revive the retired 3.5U profile based on old notes or inventory entries.

## Version policy

- GFA BASIC 3.52 is the primary reference baseline and default expected behavior for current development.
- GFA BASIC 3.52 is the active native runtime baseline.
- GFA BASIC 3.041U is the available 3.0-era native comparison profile; label it exactly `3.041U`, not `3.0`.
- Amiga GFA BASIC 3.5 remains the normative source-language target under ADR 0009, but there is no active native 3.5 runtime profile in the current workflow set.
- GFA BASIC 3.5U is retired: do not use its workflow or assets for new runs. Preserve old 3.5U observations as historical evidence only.
- Run the same applicable probe against all versions when the licensed assets and native harness are available.
- Keep one result per exact interpreter version. Do not infer one version's behavior from another.
- Use `PASS`, `FAIL`, `DIFFERENT`, and `UNTESTED` per probe/version. A verified version difference is not automatically a defect. Missing assets or unavailable runs are `UNTESTED`, never PASS.
- Separate manual-documented behavior, directly observed behavior, and unverified assumptions.

## Procedure

### 1. Establish the source of truth

- Find the relevant example in the matching primary manual.
- Record the manual edition and printed page numbers (not only PDF page indices).
- Preserve statement order and control flow from the manual unless the test explicitly exercises a variation.
- Where possible, reuse one probe across versions. If version-specific syntax/setup requires variants, explain why the test intent remains equivalent.

### 2. Inspect the harness before changing a fixture

- Find the workflow/job that performs native Merge + Run for the target interpreter.
- Check drive assignments, working directory, input listing placement, output capture, completion detection, and expected result format.
- Inspect existing passing probes for the same interpreter and harness.
- Keep the fixture minimal and add one behavior at a time.

### 3. Write a diagnostic probe

- Include a clear test identifier and deterministic machine-readable result records in the format expected by the repository harness.
- Emit useful intermediate diagnostics separately from machine-readable pass/fail records.
- Open every output file before writing to its file number. For example:

  ```basic
  OPEN "O",#1,"DF0:FACT.OUT"
  OPEN "O",#2,"DF0:GFATEST.OUT"
  ```

- Use the exact drive/path expected by the workflow, close files, and ensure normal execution reaches the close statements.
- Do not infer a language/runtime defect from a missing output file. First check file-open errors, drive mapping, emulator logs/screenshots, and whether the program completed.

### 4. Identify the running interpreter version
Use the version shown by the running editor, not the disk or archive filename.
1. Open the GFA-BASIC editor in Amiberry.
2. Press and keep holding the right mouse button to open the editor menu.
3. While the right mouse button is still held down, capture the screenshot. Do not release it before or during capture.
4. Read the first menu item in the upper-left corner. OCR is supporting evidence; inspect/crop the screenshot and transcribe it manually if uncertain.
5. Preserve the screenshot and exact version text with the test evidence.

### 5. Merge an LST file in the native editor

1. Run the native Merge + Run workflow for the exact target interpreter version.
2. Run the same applicable probe on other reference versions when assets and workflows are available.
3. Wait for each workflow and relevant job to finish; do not report pending work as validated.
4. Inspect raw diagnostic and result files, not only the workflow summary.
5. Compare results with version-specific expectations. Record confirmed cross-version behavior as `DIFFERENT`; do not treat it as a failure unless that version's own expected behavior is contradicted.
6. On failure, reduce the fixture to the smallest reproducer and compare it with the manual statement by statement.

### 7. Report precisely

In the PR description or test notes, state:

- exact interpreter version and platform for each run;
- manual edition and printed page references;
- probe intent and version-specific expected values;
- native workflow/run evidence;
- per-version outcome (`PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`);
- limitations or unresolved assumptions.

Do not describe a single probe as complete language conformance.

## Safety and repository boundaries

- Never commit commercial interpreter binaries, Kickstart ROMs, Workbench disks, or disk images.
- Use the repository's private-asset workflow for licensed files.
- Retain only permitted source fixtures and diagnostics.
- Do not bypass access controls or invent native test results. If credentials or a private asset are missing, record the blocker according to `AGENTS.md` and continue independent work.
