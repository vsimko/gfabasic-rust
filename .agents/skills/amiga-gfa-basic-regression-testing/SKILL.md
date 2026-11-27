---
name: amiga-gfa-basic-regression-testing
description: Create, debug, and validate construct-scoped, version-specific probes for original Amiga GFA BASIC reference interpreters.
---

# Amiga GFA BASIC regression testing

Use this skill when authoring or changing language-construct probes, investigating native Amiberry Merge + Run behavior, or comparing original interpreter releases.

## Required context

1. Read `AGENTS.md` and follow the repository agent workflow.
2. Read [the probe-writing guide](../../../docs/testing/amiga-gfa-basic-probes.md).
3. Read [ADR 0017](../../../docs/architecture/adr/0017-multi-version-amiga-gfa-basic-conformance.md) and [ADR 0018](../../../docs/architecture/adr/0018-construct-scoped-amiga-gfa-basic-probes.md).
4. Read [the construct manifest contract](../../../tests/language-constructs/README.md) and run `python scripts/validate_language_constructs.py validate` when changing manifests. The schema/contract is tracked by [#570](https://github.com/vsimko/gfabasic-rust/issues/570), changed-construct selection is implemented by [#571](https://github.com/vsimko/gfabasic-rust/issues/571), and parallel native matrix execution remains tracked by [#572](https://github.com/vsimko/gfabasic-rust/issues/572).
5. Inspect the native workflow that will execute the probe and the latest relevant issue/CI run.
6. Identify the exact interpreter version, platform, binary/asset source, and manual edition. Never assume observations transfer between versions.

## Version policy

- GFA BASIC 3.52 is the primary reference baseline and default expected behavior for current development.
- GFA BASIC 3.041U is the available 3.0-era native comparison profile; label it exactly `3.041U`, not `3.0`.
- Amiga GFA BASIC 3.5 remains the normative source-language target. Its editor identity and initial three-case native smoke profile are verified; broader runtime conformance remains incomplete.
- Exact version identifiers are `3.041U`, `3.5`, and `3.52`. Keep one result per exact version.
- Use `PASS`, `FAIL`, `DIFFERENT`, and `UNTESTED` per probe/version. Missing assets or unavailable runs are `UNTESTED`, never PASS.
- Separate manual-documented behavior, directly observed behavior, expected behavior, and unverified assumptions.

## Construct directory contract

New construct-scoped probes live under:

```text
tests/language-constructs/<construct-id>/
  manifest.yaml
  probe.LST
  README.md       # optional
```

- Use one directory per independently testable language construct and a stable lowercase kebab-case ID.
- Use YAML for `manifest.yaml`. Do not create TOML construct manifests.
- Keep evidence scope, per-version expected behavior, observed run result, and Rust implementation/conformance status as separate fields.
- Declare the exact interpreter versions for which the construct is applicable. Do not assume all versions apply.
- Reuse one source probe when its syntax and intent are equivalent. Add version-specific variants only when required and explain why they remain equivalent tests.
- Keep original probes minimal and independently authored. Never copy historical program listings.
- Keep native run outputs and screenshots in workflow artifacts; do not commit generated results as if they were source evidence.
- The existing `tests/interpreter-probes/` suites remain supported during migration. Do not move or delete them as part of an unrelated probe change.

## Procedure

### 1. Establish the source of truth

- Find the relevant primary-manual example and record its edition and printed page numbers.
- Identify exact dialect, platform, interpreter version, and tested component.
- Search existing construct directories, API declarations, research inventories, issues, and fixtures to avoid duplicates.
- Separate documentary evidence from runtime expectations. A source sighting is not proof that a spelling is built in or supported by a given release.

### 2. Inspect the harness

- Identify the native workflow and exact interpreter profile.
- Check drive assignments, working directory, input listing placement, output capture, completion detection, and expected result format.
- Check whether the same probe can run on other exact versions.
- Keep each construct/version pair independently identifiable in CI and artifacts.

### 3. Write a diagnostic probe

- Include a stable test identifier and deterministic machine-readable result records.
- Emit useful intermediate diagnostics separately from pass/fail records.
- Open every output file before writing to its file number. For example:

  ```basic
  OPEN "O",#1,"DF0:FACT.OUT"
  OPEN "O",#2,"DF0:GFATEST.OUT"
  ```

- Use the exact drive/path expected by the workflow, close files, and ensure normal execution reaches the close statements.
- Do not infer a language/runtime defect from a missing output file. Check file-open errors, drive mapping, emulator logs/screenshots, and program completion first.

### 4. Identify the running interpreter version

Use the version shown by the running editor, not the disk or archive filename.

1. Open the GFA-BASIC editor in Amiberry.
2. Press and keep holding the right mouse button to open the editor menu.
3. Capture the screenshot while the right button remains held.
4. Read the first menu item in the upper-left corner. OCR is supporting evidence; inspect and transcribe manually if uncertain.
5. Preserve the screenshot and exact version text with the test evidence.

### 5. Run and interpret the probe

1. Run each selected construct/version pair in its matching native environment.
2. Wait for every workflow and job to finish.
3. Inspect raw diagnostic and result files, not only the workflow summary.
4. Compare each result with that version's own expected behavior.
5. Record a verified cross-version difference as `DIFFERENT`; it is not automatically a defect.
6. On failure, reduce the fixture to the smallest reproducer and compare it with the applicable manual.

### 6. Report precisely

Record the construct ID, exact interpreter version and platform, manual edition and printed page references, probe intent, expected value, workflow/run evidence, per-version outcome, and unresolved assumptions. A successful host-side validator or green workflow does not establish complete language conformance.

## Safety and repository boundaries

- Never commit commercial interpreter binaries, Kickstart ROMs, Workbench disks, or disk images.
- Use the repository's private-asset workflow for licensed files.
- Retain only permitted source fixtures and diagnostics.
- Do not bypass access controls or invent native test results. Record human blockers according to `AGENTS.md` and continue independent work.
