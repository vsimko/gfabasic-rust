---
type: "Testing Guide"
title: "Writing Amiga GFA BASIC reference-interpreter probes"
---

# Writing Amiga GFA BASIC reference-interpreter probes

## Purpose

Use this guide when writing or changing .LST regression programs for the original Amiga GFA BASIC interpreter. A probe must distinguish a language/runtime failure from a test-harness or program-setup failure.

## Before writing a probe

1. Identify the exact interpreter version and platform under test. Do not transfer observations between 3.0, 3.5, 3.5U, and 3.52 without evidence.
2. Find the relevant primary-manual example and record its edition and printed page numbers.
3. Preserve the documented statement order and control-flow structure unless the test explicitly targets a variation.
4. Check how the native harness starts the program, assigns drives, captures output, and detects completion. Read the matching workflow and existing passing .LST probes.
5. Keep the fixture minimal. Add one behavior at a time and write diagnostic values before asserting them.

## Native editor: merging an LST file

For the current Amiberry 3.5U native Merge + Run harness, the verified sequence is:

1. Open the editor's Merge file requester with the harness's existing menu/key sequence.
2. Take a screenshot while the requester is open. Verify the intended `.LST` file row and the pointer's horizontal alignment over that row.
3. Keep the pointer's X coordinate unchanged. Move only Y to **215 px**, which is the `gfatest.LST` row in the current emulator/window layout. The IPC relative Y correction is approximately four raw units per screen pixel.
4. Capture the pre-click screenshot with the pointer on the row.
5. Double-click without moving the pointer: left-button down, release, down, release.
6. Wait for the requester to close and capture the editor. Confirm the merged listing is visible.
7. Compare the editor contents with the exact staged `.LST` file before running. Check significant tokens literally; e.g. `MAT ONE a()` is not equivalent evidence to `MAT SET a()`. OCR is only supporting evidence—inspect the screenshot if a token is unclear.
8. Before launching Amiberry, extract `gfatest.LST` from the prepared DF0 ADF and compare it byte-for-byte with the staged fixture. Keep both SHA-256 values and the extracted file as artifacts. This verifies disk preparation, not editor Merge.
9. After Merge, use OCR only to detect known contradictions. For the current `M O` / MAT ONE probe, recognized `MAT SET` is a hard stop before Run; preserve the screenshot and inspect it manually. A clean OCR result does not prove byte identity or semantic correctness.
10. Only after the merged source has no known contradiction may the harness start Run with **Shift+F10**. In the current Amiberry IPC harness, press left Shift raw keycode `0x60` (96), press F10 `0x59` (89), release F10, then release Shift. Do not click an estimated Run-gadget coordinate.
9. Save screenshots before the double-click, after Merge, and after Run as diagnostic artifacts.

The Y=215 coordinate is specific to the current harness layout and must be revalidated if the emulator/window layout changes. A closed requester or visible listing proves that Merge completed, but not that the intended source was merged unchanged. If the editor shows a different token or statement, stop before diagnosing runtime behavior. Compare the prepared disk's source file, the staged fixture, and the editor display; record the discrepancy without assuming whether fixture preparation, tokenization, or display/OCR caused it.

## DATA pointer probes (3.5U)

The native probe for the 3.5U addendum DATA-pointer example exposed setup details that must be preserved:

- Put the DATA declarations after the executable program body, following END, for this tested layout.
- Execute RESTORE before the first DATA availability check/read.
- Use _DATA to inspect/save the DATA pointer position. Do not substitute the DATA availability expression for _DATA in the DO WHILE guard.
- Save each pointer before READ, then restore saved positions with _DATA=... before reading again.
- The documented reverse traversal uses FOR ... DOWNTO.
- The addendum example on printed pages 13-34–13-35 documents the reverse values 0, 1, 3242, 328, 24, 13, 9, 8, 7, 6, 5, 4, 3, 2, 1 for the fixture used in the regression test. Keep the fixture and expected values together; do not generalize this result to other versions.

These are observed rules for the exact 3.5U probe and its documented example, not a claim of complete DATA semantics across all GFA BASIC versions.

## Output and harness rules

- Open every output file before writing to its file number. For example, issue OPEN "O",#1,"DF0:FACT.OUT" and OPEN "O",#2,"DF0:GFATEST.OUT" before any PRINT #1 or PRINT #2.
- Use the drive/path expected by the native workflow. A missing output file can mean the program never opened or created it; it does not by itself prove a language feature failed.
- Keep diagnostic output (for example, initial pointer state and each value read) separate from machine-readable GFATEST: result records.
- Close files after writing and ensure all normal paths reach the close statements.
- When the host reports Node not found: FACT.OUT, first inspect the emulator screenshot/log and program file-open path before diagnosing interpreter semantics.

## Required validation loop

1. Run the exact native Merge + Run workflow for the target interpreter version.
2. Verify the merged editor contents against the staged .LST before interpreting any Run result.
3. If the output file is missing, inspect the emulator log/screenshot and verify file opens, drive assignment, and program completion first.
4. If output exists, inspect raw FACT.OUT and GFATEST.OUT; do not rely only on the workflow summary.
5. Compare every diagnostic value and result ID with the documented expectation.
6. When a probe fails, reduce it to the smallest reproducer and compare it with the manual's example statement by statement.
7. After a fix, rerun the native workflow and wait for its completed result. A code change is not validated until the target native run is green.
8. Keep the PR description precise about the tested version, source pages, observed behavior, and remaining limits.

## Evidence and safety boundaries

- Cite the manual edition/version and printed pages in the test comments or linked documentation.
- Separate manual-documented behavior from behavior observed in the emulator and from assumptions.
- Do not claim a runtime implementation bug until the test setup and output path have been verified.
- Do not commit commercial interpreter binaries, Kickstart ROMs, Workbench disks, or disk images. Use the private asset workflow and retain only permitted diagnostics.
