---
type: "Testing Guide"
title: "Writing Amiga GFA BASIC reference-interpreter probes"
---

# Writing Amiga GFA BASIC reference-interpreter probes

## Purpose

Use this guide when writing or changing .LST regression programs for the original Amiga GFA BASIC interpreter. A probe must distinguish a language/runtime failure from a test-harness or program-setup failure.

## Interpreter version strategy

The project uses three reference versions. See [ADR 0017](../architecture/adr/0017-multi-version-amiga-gfa-basic-conformance.md) and [issue #557](https://github.com/vsimko/gfabasic-rust/issues/557).

| Version | Role |
|---|---|
| GFA-BASIC 3.52 | Active native runtime baseline and default expected behavior for current development |
| GFA-BASIC 3.041U | Available native comparison profile for the 3.0-era dialect; always report the exact `3.041U` version |
| Amiga GFA-BASIC 3.5 | Normative source-language target (ADR 0009); initial native smoke probes (arithmetic, FOR/NEXT, strings) passed; broader conformance remains incomplete |

The unified entry point is `.github/workflows/amiga-reference-matrix.yml`. It calls the two active native workflows as reusable workflows and captures the running 3.5 editor's version menu for identity evidence. An initial 3.5 native smoke suite (arithmetic, FOR/NEXT, strings) ran using a separate writable test disk in [workflow run 44094135503](https://github.com/vsimko/gfabasic-rust/actions/runs/44094135503). The inspected artifact `amiga-gfa-basic-3.5-asset-discovery` contains the held-right-button menu screenshot visibly identifying `GFA-BASIC 3.5 Block`, Merge/Run screenshots, and three raw `GFATEST:*:PASS` records. These three cases are PASS for that exact candidate and run; this limited smoke set is not complete language conformance. Run this matrix for a combined view. The individual native workflows remain manually dispatchable for focused diagnosis. A green matrix means the configured native jobs and GFATEST collection protocol completed; it is not a claim of complete language conformance.

Run each applicable probe against active native profiles for which the licensed asset and workflow are available. The current native profiles are 3.52 and 3.041U. Keep results separate; never relabel 3.041U as exactly 3.0 or transfer its results to 3.5. Amiga GFA-BASIC 3.5 remains the normative documented target; its identity and the three initial native smoke results have been verified from the retained screenshot and raw result artifact. Broader native conformance remains incomplete.

Use these outcomes per probe/version:

- `PASS`: observed behavior matches the version-specific expectation.
- `FAIL`: observed behavior contradicts the version-specific expectation.
- `DIFFERENT`: a verified cross-version difference is recorded; this is not automatically a defect.
- `UNTESTED`: no adequate run/evidence is available. Missing assets are not failures or passes.

A maintainer-reported example motivating this policy is `M O a()`: reported as `MAT SET a()` in 3.5 and `MAT ONE a()` in 3.52. Treat it as reported evidence until exact-version screenshots/native runs are preserved.

## Verify native editor Merge

For a probe that depends on editor shortcut expansion (for example `M O identity()`), distinguish editor behavior from an explicitly typed runtime command such as `MAT ONE identity()`.

1. Capture the Merge requester and confirm the exact `.LST` filename selected.
2. After Merge, capture the editor and inspect the source statement itself before running.
3. Compare the displayed statement with the staged fixture byte-for-byte where possible; OCR is supporting evidence only.
4. If the editor shows a changed or unrecognized token, stop before Run and preserve the screenshot and source fixture. Do not attribute a later runtime error to the interpreter until the Merge discrepancy is understood.
5. Run with the workflow's verified key sequence only after the source is confirmed. Do not reuse mouse coordinates from a different emulator layout.

A successful Merge operation does not establish that a statement was expanded or accepted correctly.

## Probe metadata

The standalone metadata for the Amiga 3.5 FACT smoke probe is stored in
`tests/interpreter-probes/amiga-3.5/fact-zero.yaml`. It is deliberately
separate from the conformance-corpus manifest: it records the target component,
input format, documentary expectation, and observed editor/runtime results.
Keep expected output separate from observed output, and do not mark a probe as
loaded or executed unless the corresponding native run produced that evidence.
The corpus validator does not validate this probe-specific metadata file.

## Before writing a probe

1. Identify the exact interpreter version and platform under test. Do not transfer observations between 3.041U, 3.5, and 3.52 without evidence.
2. Find the relevant primary-manual example and record its edition and printed page numbers.
3. Preserve the documented statement order and control-flow structure unless the test explicitly targets a variation.
4. Check how the native harness starts the program, assigns drives, captures output, and detects completion. Read the matching workflow and existing passing .LST probes.
5. Keep the fixture minimal. Add one behavior at a time and write diagnostic values before asserting them.
6. Where possible, use the same source probe and harness procedure across versions. If syntax or setup must differ, document the reason and keep the test intent equivalent.

## Identify the running interpreter version

Use the version shown by the running editor, not the disk or archive filename.

1. Open the GFA-BASIC editor in Amiberry.
2. Press and keep holding the right mouse button to open the editor menu.
3. While the right mouse button is still held down, capture the screenshot. Do not release it before or during capture: releasing the button closes the menu, so the screenshot would no longer show the version.
4. OCR the screenshot and read the first menu item in the upper-left corner; it displays the running GFA-BASIC interpreter version.
5. Record the exact text and preserve the screenshot as evidence. If OCR is uncertain, inspect or crop the upper-left menu area and transcribe it manually.

Use this procedure whenever the executable version is uncertain or a test result must be tied to an exact interpreter version. Disk labels and archive names are not substitutes for inspecting the running editor.

**Interpretation of the `==>` editor prefix:** In the manually observed reference editor, an imported `.LST` statement prefixed with `==>` is an unsupported or unrecognized command. The editor can retain and display the statement after MERGE, while attempting to execute it can fail later with a runtime `syntax error ==> program aborted`. Therefore, successful MERGE or a visible source line does not establish that the command is supported. Inspect the prefix before Run and record both the editor screenshot and any runtime error. Treat this as observed behavior for the tested interpreter, not a universal rule for every GFA-BASIC release.
