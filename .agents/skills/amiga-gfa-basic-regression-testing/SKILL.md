---
name: amiga-gfa-basic-regression-testing
description: Create, debug, and validate regression probes for original Amiga GFA BASIC reference interpreters. Use when editing .LST fixtures, investigating native Amiberry Merge + Run failures, or validating version-specific language behavior.
---

# Amiga GFA BASIC regression testing

Use this skill when working on regression probes for the original Amiga GFA BASIC interpreters in this repository.

## Required context

1. Read `AGENTS.md` and follow the repository agent workflow.
2. Read [the probe-writing guide](../../../docs/testing/amiga-gfa-basic-probes.md).
3. Read the applicable version-specific testing guide and inspect the native workflow that will execute the probe.
4. Identify the exact interpreter version, platform, binary/asset source, and manual edition. Never assume that observations transfer between GFA BASIC 3.0, 3.5, 3.5U, and 3.52.

## Procedure

### 1. Establish the source of truth

- Find the relevant example in the matching primary manual.
- Record the manual edition and printed page numbers (not only PDF page indices).
- Preserve statement order and control flow from the manual unless the test explicitly exercises a variation.
- Separate three kinds of evidence in notes and test comments: manual-documented behavior, behavior observed in the reference interpreter, and unverified assumptions.

### 2. Inspect the harness before changing a fixture

- Find the workflow/job that performs native Merge + Run for the target interpreter.
- Check drive assignments, working directory, input listing placement, output capture, completion detection, and expected result format.
- Inspect existing passing probes for the same interpreter and harness.
- Keep the probe minimal and add one behavior at a time.

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

### 4. Apply version-specific knowledge carefully

For the tested GFA BASIC 3.5U DATA-pointer addendum example:

- Place DATA declarations after the executable body, following END, as in the tested fixture.
- Execute RESTORE before the first DATA availability check/read.
- Use `_DATA` to inspect or save the DATA pointer. Do not substitute the DATA availability expression for `_DATA` in the `DO WHILE` guard.
- Save pointer positions before READ and restore them with `_DATA=...` before reading again.
- The documented reverse traversal uses `FOR ... DOWNTO`.
- The tested fixture's reverse sequence is `0, 1, 3242, 328, 24, 13, 9, 8, 7, 6, 5, 4, 3, 2, 1`.

These are observations for that exact 3.5U addendum example and fixture, not universal claims about all GFA BASIC versions or complete DATA semantics. Consult the guide and manual before reusing them.

### 5. Run and diagnose the native test

1. Run the native Merge + Run workflow for the exact target interpreter.
2. Wait for the workflow and relevant job to finish; do not report a change as validated while the run is still pending.
3. If the expected output file is missing, inspect the emulator log/screenshot, file opens, drive assignment, and program completion first.
4. If output exists, inspect the raw diagnostic and result files, not only the workflow summary.
5. Compare every diagnostic value and result identifier with the manual-derived expectation.
6. On failure, reduce the fixture to the smallest reproducer and compare it with the manual statement by statement.
7. Fix the test or implementation according to the evidence, then rerun the native workflow. A target probe is validated only when its native run is green.

### 6. Report precisely

In the PR description or test notes, state:

- exact interpreter version and platform;
- manual edition and printed page references;
- what the probe verifies and its expected values;
- native workflow/run evidence;
- any limitations or unresolved assumptions.

Do not describe a single probe as complete language conformance.

## Safety and repository boundaries

- Never commit commercial interpreter binaries, Kickstart ROMs, Workbench disks, or disk images.
- Use the repository's private-asset workflow for licensed files.
- Retain only permitted source fixtures and diagnostics.
- Do not bypass access controls or invent native test results. If credentials or a private asset are missing, record the blocker according to `AGENTS.md` and continue independent work.
