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
