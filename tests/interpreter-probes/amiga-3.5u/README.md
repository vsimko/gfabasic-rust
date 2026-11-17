# GFA-BASIC 3.5 regression probe on the 3.52 reference interpreter

The regression listing is based on the Amiga GFA BASIC 3.5 interpreter manual and its addenda. This native Merge + Run workflow executes it with the **GFA-BASIC 3.52** reference interpreter, not the 3.5U binary.

The workflow uses the private asset `GFA-BASIC v3.52 (1990)(GFA Systemtechnik)(DE)(Disk 1 of 2)[WB].zip` (Git blob SHA `3c14e037559c2c2e1c0ae96088a7a7689594e4f5`). Amiberry mounts this ZIP directly as read-only DF1. The writable DF0 contains the regression listing and captures runtime output. The private ZIP, commercial executable, ROM and disk images are never committed or uploaded.

## Version-sensitive editor abbreviation

The short editor form `M O a()` is version-sensitive:

| Interpreter | Editor expansion | Result |
|---|---|---|
| 3.52 | `MAT ONE a()` | Correct |
| 3.5 | `MAT SET a()` | Incorrect |
| 3.041 | leaves `==> M O a()` unchanged | Not expanded |

Therefore the 3.52 ZIP is intentional and required for the native editor Merge test. Do not replace it with the 3.5/3.5U disk: that version expands the abbreviation incorrectly. This editor-expansion observation is distinct from executing the explicit runtime statement `MAT ONE a()`.

## Regression coverage

The fixture retains the broad 3.5 regression batch: FACT boundary values, arithmetic and precedence, FOR/NEXT, strings, DATA-pointer handling, matrix determinant/inverse/add/subtract/transpose/multiply/identity, combinatorics, special variables and the prioritized DQH language and graphics subset. The explicit `MAT ONE` test checks all nine elements of a 3x3 identity matrix.

Results are observations for the exact 3.52 ZIP asset and emulator/Kickstart setup recorded in each workflow artifact. They provide compatibility evidence for the 3.5-based test cases, not proof that 3.5 and 3.52 are identical or that the entire language conforms.

## Isolated MAT ONE investigation (issue #537)

[`ab-c-isolation.LST`](ab-c-isolation.LST) is a diagnostic-only A/B/C listing. It is intentionally separate from the stable regression batch and runs with the exact GFA-BASIC 3.5U interpreter asset:

- Private archive: `GFA-BASIC v3.5U (1990)(GFA Systemtechnik)(DE)(Disk 1 of 2)[b corrupt files][WB].zip`
- Archive SHA-256: `004c20a73d1fe715112b5b2672138e7e7c9ea1e11f6900004e5466dc15382413`
- Extracted `GFABasic` SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Emulator profile: Amiberry, 68000, OCS, Kickstart 1.3 rev 34.5.

The dedicated [A/B/C workflow](../../../.github/workflows/amiga-35u-ab-c-diagnostic.yml) keeps the original commercial disk and ROM private and uploads only diagnostic logs, screenshots, and output files.

- **A** runs a trivial PRINT program and closes its output file.
- **B** declares and accesses a square 3×3 array without MAT statements, then closes its output file.
- **C** executes explicit `MAT ONE a()` on that same array and records the identity matrix if it returns.

A and B are written to separate files before C starts, so a C task-held failure cannot erase their completed output. The workflow distinguishes a confirmed task-held requester, an emulator exception, an unclassified no-output result, and a successful or incorrect matrix result; a missing `MAT.OUT` alone is not treated as proof of a task-held requester or its cause.

### Latest native result

Run [#44046096736](https://github.com/vsimko/gfabasic-rust/actions/runs/44046096736) completed successfully with:

- `CASE_A_BASELINE=PASS`
- `CASE_B_DIM_ARRAY=PASS`
- `CASE_C_MAT_ONE=TASK_HELD_CONFIRMED`

The A and B output files contain their expected markers. C produced no `MAT.OUT`; the captured Workbench screenshot OCR reads `Software error - task held / Finish ALL disk activity`. This reproduces the symptom in the exact 3.5U setup after the same-run controls pass. It does not identify the interpreter's internal cause. Keep this probe out of the stable passing suite, and do not infer 3.5U behavior from 3.52 results.

### Source-disk integrity caveat

The 3.5U source archive is explicitly tagged `[b corrupt files]`. The workflow extracts the sole ADF from that archive, reads only `GFABasic` from it, and verifies the extracted executable against a pinned SHA-256. Amiberry then mounts that original ADF as DF1; the A/B/C listing runs from a separately created and formatted writable DF0. Thus the filename is a legitimate confounder, but it does **not** establish that the `GFABasic` executable itself is corrupt: the tag may refer to other files on the disk, and the pinned hash establishes identity rather than known-good integrity.

The task-held result should therefore be treated as a reproduction against this particular potentially damaged 3.5U disk image, not yet as a clean-image interpreter defect. A decisive follow-up is to repeat the same A/B/C probe using a separately sourced, independently verified clean 3.5U disk/executable (or compare the extracted executable against a trusted dump). Until then, disk-image/file corruption remains an unresolved alternative explanation.
