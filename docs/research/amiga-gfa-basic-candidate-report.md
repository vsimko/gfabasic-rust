# Amiga GFA BASIC 3.5 candidate verification report

> Generated from `amiga-gfa-basic-candidates.toml`. Status describes evidence of 3.5 applicability, not implementation or runtime support.

## Summary

| Status | Entries |
|---|---:|
| candidate | 0 |
| confirmed-in-3.5 | 6 |
| contradicted | 0 |
| unresolved | 11 |

## Unresolved and pending candidates

| ID | Candidate | Source scope | Printed pages | Notes |
|---|---|---|---|---|
| `amiga.base.advanced` | Advanced programming and hardware features | 3.0 base manual | 10-2–10-14 | SPRITE, STICK and STRIG have selected notes; CLI, EXEC, SAY, FD-files and object commands need reconciliation. |
| `amiga.base.appendices` | Appendix tables and command index | 3.0 base manual | 12-2–12-19 | Reference material is not a declaration candidate by itself; index entries must be checked against their cited chapter pages. |
| `amiga.base.events-windows` | Events, menus, screens and windows | 3.0 base manual | 9-2–9-37 | FILESELECT and selected screen/window topics are inventoried; event/menu/window behavior needs systematic 3.5 verification. |
| `amiga.base.files` | Disk handling and file organization | 3.0 base manual | 6-2–6-21 | File-channel I/O is partially extracted; other disk and random-access operations need separate 3.5 checks. |
| `amiga.base.graphics` | Graphics language commands | 3.0 base manual | 8-2–8-24 | Selected drawing and fill operations have page-backed notes; full graphics surface is not individually reconciled. |
| `amiga.base.input-output` | Input and output | 3.0 base manual | 5-2–5-22 | Selected command forms have page-backed notes; complete declaration-by-declaration 3.5 reconciliation remains outstanding. |
| `amiga.base.numeric-string` | Numeric and string functions | 3.0 base manual | 4-2–4-36 | The base inventory extracts selected semantics only; the full Chapter 4 family is not individually verified for 3.5. |
| `amiga.base.operators` | Operators and precedence | 3.0 base manual | 3-2–3-16 | OCR-sensitive operator glyphs and precedence spellings require page-image and 3.5 reconciliation. |
| `amiga.base.os-libraries` | Amiga OS library calls | 3.0 base manual | 11-2–11-92 | Seven library families and many routines are inventoried; exact wrapper contracts, OCR details and 3.5 availability remain unverified. |
| `amiga.base.program-control` | Program handling and control flow | 3.0 base manual | 7-2–7-55 | Selected conditionals, loops and branching forms are extracted; procedures, events, errors and editor commands remain to be reconciled. |
| `amiga.base.variables-memory` | Variables, arrays and memory | 3.0 base manual | 2-2–2-44 | Chapter 2 contains multiple declarations; this family row is a tracking boundary, not a claim that every listed feature is absent from 3.5. |

## Explicitly confirmed by the 3.5 addendum

| ID | Declaration family | Printed pages | Qualification |
|---|---|---|---|
| `amiga.35.mat-analysis` | MAT NORM, MAT DET, MAT QDET, MAT RANG, MAT INV | 13-31–13-39 | The family is documented in Chapter 13; MAT INV operand direction is explicitly disputed between summary and detailed example. |
| `amiga.35.mat-arithmetic` | MAT ADD, MAT SUB, MAT MUL | 13-22–13-30 | Element-wise, scalar, matrix-product and vector-product forms are documented; MAT MUL a(),x semantics remain uncertain in the research notes. |
| `amiga.35.mat-base` | MAT BASE | 13-1–13-2 | Explicitly documented in the version-specific Chapter 13 addendum; applicability is limited to the documented matrix operations. |
| `amiga.35.mat-copy` | MAT CPY, MAT XCPY, MAT TRANS | 13-8–13-20 | Detailed source-checked shape, offset, extent, transpose and vector-orientation rules are recorded in the addendum inventory. |
| `amiga.35.mat-generation` | MAT CLR, MAT SET, MAT ONE | 13-2–13-3 | Generation forms and floating-point field constraints are documented in Chapter 13. |
| `amiga.35.mat-io` | MAT READ, MAT PRINT, MAT INPUT | 13-3–13-6 | Chapter 13 explicitly documents vector/matrix data I/O forms. |

## Contradicted candidates

No candidates are currently classified as contradicted. This is not evidence that all remaining candidates are compatible.

## Interpretation and limitations

- The 3.0 base-manual entries are retained as unresolved until each declaration or feature family is checked against explicit 3.5 addendum evidence or a reproducible runtime observation.
- A confirmed family does not settle every overload or edge case; qualifications in the manifest and detailed research notes remain authoritative.
- Chapter 12 indexes and reference tables are navigation aids, not declarations or proof of availability.
- No absence-based removal or compatibility inference is made.

Source: `AM-INTERPRETER-35` in `docs/reference-sources.md`; detailed page notes are in `amiga-gfa-basic-3.0-base-inventory.md` and `amiga-gfa-basic-35-addendum-inventory.md`.
