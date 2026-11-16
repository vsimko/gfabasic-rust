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

## Identify the running interpreter version

Use the version shown by the running editor, not the disk or archive filename.

1. Open the GFA-BASIC editor in Amiberry.
2. Press the right mouse button to open the editor menu.
3. Capture a screenshot with the menu open.
4. OCR the screenshot and read the first menu item in the upper-left corner; it displays the running GFA-BASIC interpreter version.
5. Record the exact text and preserve the screenshot as evidence. If OCR is uncertain, inspect or crop the upper-left menu area and transcribe it manually.

Use this procedure whenever the executable version is uncertain or a test result must be tied to an exact interpreter version. Disk labels and archive names are not substitutes for inspecting the running editor.

**Interpretation of the `==>` editor prefix:** In the manually observed reference editor, an imported `.LST` statement prefixed with `==>` is an unsupported or unrecognized command. The editor can retain and display the statement after MERGE, while attempting to execute it can fail later with a runtime `syntax error ==> program aborted`. Therefore, successful MERGE or a visible source line does not establish that the command is supported. Inspect the prefix before Run and record both the editor screenshot and any runtime error. Treat this as observed behavior for the tested interpreter, not a universal rule for every GFA-BASIC release.

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
2. If the output file is missing, inspect the emulator log/screenshot and verify file opens, drive assignment, and program completion first.
3. If output exists, inspect raw FACT.OUT and GFATEST.OUT; do not rely only on the workflow summary.
4. Compare every diagnostic value and result ID with the documented expectation.
5. When a probe fails, reduce it to the smallest reproducer and compare it with the manual's example statement by statement.
6. After a fix, rerun the native workflow and wait for its completed result. A code change is not validated until the target native run is green.
7. Keep the PR description precise about the tested version, source pages, observed behavior, and remaining limits.

## Evidence and safety boundaries

- Cite the manual edition/version and printed pages in the test comments or linked documentation.
- Separate manual-documented behavior from behavior observed in the emulator and from assumptions.
- Do not claim a runtime implementation bug until the test setup and output path have been verified.
- Do not commit commercial interpreter binaries, Kickstart ROMs, Workbench disks, or disk images. Use the private asset workflow and retain only permitted diagnostics.
