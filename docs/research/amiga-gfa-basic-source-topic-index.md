---
type: "Research Note"
title: "Amiga GFA BASIC source and topic index"
---

# Amiga GFA BASIC source and topic index

**Purpose:** page-addressable navigation through the supplied Amiga references and the project's source-derived extraction notes. This is a research index, not a claim that every listed feature is implemented or runtime-tested.

## Source/version key

| Source ID | Source and scope | How to use it |
|---|---|---|
| `AM-INTERPRETER-35` | *GFA BASIC 3.5 Interpreter*; 3.0 base manual plus explicit 3.5 update material | Normative source for explicit 3.5 addendum features. Base-manual material remains 3.0 evidence unless separately confirmed by the addendum or runtime. |
| `AM-INTERPRETER-30-35-UPDATE-SCAN` | Combined 3.0 + 3.5 update scan, including explicit Chapter 13, Chapter 14 and editor update | Independent pagination witness for the same 3.5 update. Chapter 13 matrix material and Chapter 14 additions use printed pp. 14-1–14-7 in this scan. |
| `AM-BOOK-DE-1990` | Bleek/Hecht/Litzkendorf, *Das große GFA-BASIC-Buch zum Amiga*, Data Becker 1990, ISBN 3-89011-399-0 | Broad historical Amiga reference. Foreword says interpreter 3.0; section 1 identifies V3.03. Do not treat the cover's “bis Version 3.5” wording as proof of 3.5 interpreter semantics. |
| `AM-BOOK-FR` | French translation of the 1989 Amiga book, ISBN 2-86899-207-2 | Historical 3.0 corroboration; keep its page numbering separate. |

Bibliographic provenance, scan access and copyright notes are in [the source register](../reference-sources.md), [the German OCR index](amiga-gfa-basic-3.03-ocr-index.md), and [the 3.5 primary reference notes](amiga-gfa-basic-35-primary-reference.md). The supplied scans are external research material and are not stored in this repository.

## Topic index: German 1990 book

All pages below are **printed page numbers**, not PDF viewer page numbers. Detailed source-derived summaries and page-image corrections are in [the command/API extraction](amiga-gfa-basic-german-book-command-api-extraction.md), with section anchors.

| Topic / command family | Printed pages | Extraction |
|---|---:|---|
| Book applicability, BASIC introduction, language foundations | 13–65 | [§1–§6](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Screen/input/output, files, channels, ports | 75–119 | [§1](amiga-gfa-basic-german-book-command-api-extraction.md), [§14](amiga-gfa-basic-german-book-command-api-extraction.md) |
| AmigaDOS library calls | 120–132 | [§11](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Printer and hardcopy | 132–138 | [§12](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Sound, WAVE, SAY and speech | 139–143 | [§13](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Program structure, loops, branches, procedures and functions | 145–191 | [§7](amiga-gfa-basic-german-book-command-api-extraction.md), [§15](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Text and string operations | 193–200 | [§16](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Operators, arithmetic, numeric and bit functions | 201–224 | [§17](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Graphics definitions, drawing, DRAW$, SCROLL, GRAPHMODE | 225–256 | [§18](amiga-gfa-basic-german-book-command-api-extraction.md), [§37](amiga-gfa-basic-german-book-command-api-extraction.md) |
| GET/PUT bitmap strings, hardware sprites, BOB/OBJECT | 257–278 | [§19](amiga-gfa-basic-german-book-command-api-extraction.md), [§38](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Number conversion and formatting | 279–290 | [§20](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Arrays, OPTION BASE, sorting | 291–304 | [§21](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Memory, typed access, allocation, pointers | 305–316 | [§22](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Amiga Exec tasks, signals, ports, lists, libraries, devices | 317–328 | [§23](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Program control, errors, debugging, DUMP, MODE | 331–361 | [§8](amiga-gfa-basic-german-book-command-api-extraction.md), [§24](amiga-gfa-basic-german-book-command-api-extraction.md), [§39](amiga-gfa-basic-german-book-command-api-extraction.md) |
| ALERT, FILESELECT, mouse, joystick, STICK/STRIG | 363–374 | [§25](amiga-gfa-basic-german-book-command-api-extraction.md), [§40–§41](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Windows, screens, IDCMP, Window flags | 375–388 | [§26](amiga-gfa-basic-german-book-command-api-extraction.md), [§36](amiga-gfa-basic-german-book-command-api-extraction.md) |
| MENU creation and attributes | 389–393 | [§27](amiga-gfa-basic-german-book-command-api-extraction.md), [§36](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Intuition event handling, MENU() payload, dispatch | 395–404 | [§28–§29](amiga-gfa-basic-german-book-command-api-extraction.md), [§42–§43](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Compiler and linker (not interpreter API) | 405–450 | [§33](amiga-gfa-basic-german-book-command-api-extraction.md) |
| 3.5 matrix/vector operations (compare with separate 3.5 addendum) | 451–476 | [§30](amiga-gfa-basic-german-book-command-api-extraction.md), [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) |
| FACT, VARIAT, COMBIN, _DATA, editor additions | 477–482 | [§31](amiga-gfa-basic-german-book-command-api-extraction.md) |
| Error appendices, command index and back matter | 483–508 | [§32](amiga-gfa-basic-german-book-command-api-extraction.md), [§35](amiga-gfa-basic-german-book-command-api-extraction.md) |

## Version-specific 3.5 addendum index

The *GFA BASIC 3.5 Interpreter* manual contains a 3.0 base plus explicitly identified 3.5 additions. Use its printed Chapter 13 pagination; the combined update scan uses Chapter 13 for matrix operations and Chapter 14 for further commands.

| 3.5 feature | Supplied interpreter manual pages | Combined update scan pages | Evidence note |
|---|---:|---:|---|
| Editor row layout, closed procedure/function search, listing changes, tab keys | 1-25 | 1-25 | Explicit 3.5 editor update; not interpreter language syntax |
| Matrix base and initialization: MAT BASE, MAT CLR/SET/ONE | 13-1–13-3 | 13-1–13-3 | Floating-point 1D/2D fields only |
| Matrix READ/PRINT/INPUT | 13-3–13-6 | 13-3–13-6 | MAT PRINT punctuation visually checked; MAT INPUT is ASCII-file input |
| Matrix copy, transpose and vector special cases | 13-8–13-20 | 13-8–13-20 | MAT CPY/XCPY/TRANS and separate MAT COPY forms are distinguished by source |
| Matrix arithmetic and multiplication | 13-22–13-33 | 13-22–13-33 | Element-wise, scalar, matrix and vector forms; preserve uncertain operand orientation |
| Matrix norm, determinant, quick determinant, rank and inverse | 13-31–13-41 | 13-31–13-41 | MAT INV destination-first form resolved from detailed definition/example; summary on p. 13-16 is reversed typo |
| FACT, VARIAT, COMBIN | 13-33–13-34 | 14-1–14-2 | Explicit added mathematical functions |
| Negative-screen OPENW, PCOLOR | 13-34 | 14-3–14-4 | Additional interpreter commands |
| _DATA, _DATA=, _0…_9, GURU statement/function | 13-34–13-36 | 14-5–14-7 | Special variables and GURU behavior; preserve error-code caveats |

Detailed definitions, examples, disputes and evidence boundaries are in the [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md). The German 1990 book's Chapter 18–19 is useful corroboration, but it is not substituted for the version-specific 3.5 source.

## Explicit cross-source reconciliation

| Topic | 3.5 source | German 1990 book | Treatment |
|---|---|---|---|
| Core command reference version | 3.0 base + explicit 3.5 additions | Foreword 3.0; section 1 V3.03 | Keep base behavior historical unless independently confirmed |
| Matrix array types | Floating-point fields, 1D/2D | Broader “numeric fields” phrasing | Use narrower explicit 3.5 constraint |
| Matrix copy | Distinct MAT CPY and MAT COPY forms | MAT CPY documented | Do not merge spellings |
| Vector multiplication | Outer/dyadic, inner/scalar, vector–matrix–vector | Less complete treatment | Preserve explicit 3.5 addendum forms |
| Inverse | Summary p. 13-16 reverses operands; detailed definition/example p. 13-39 specifies destination first | German book p. 475 independently agrees | Use MAT INV destination()=source(); summary line is a typo |
| FACT/VARIAT/COMBIN vs error #86 | 3.5 addendum documents FACT/VARIAT/COMBIN | Error appendix retains stale FACT/COMBIN/PERMUT wording | VARIAT is documented; PERMUT alias is not established; error text is inconsistent |
| MENU attribute value 64 | Not part of 3.5 addendum | German page 389 prints 64 for both clear-checkmark and activate-item | Preserve source duplicate; do not silently repair |
| Intuition timer event | English manual prints 4194303 beside &H400000 in its IDCMP table | German p.378 gives 4194304; event descriptions repeat 4194303 | Resolve as decimal typo: &H400000 = 4194304 |

## Page-image validation register

| Printed pages | Items checked | Result |
|---|---|---|
| 13-1–13-41 and 14-1–14-7 (3.5 addendum) | Matrix syntax/operation examples, further 3.5 commands and editor changes | Full addendum pass recorded in issue #65; unresolved source disagreements remain explicitly flagged |
| 1-25 (3.5 editor update) | Editor changes and Tab/Ctrl+Tab/Shift+Tab operations | Visually checked; ambiguous deletion boundary preserved |
| 378–379, 389 | IDCMP flags, Window flags, MENU attributes | Page-image checked; duplicate MENU value 64 retained |
| 240, 245–250, 272–273 | GRAPHMODE examples, SCROLL, DRAW$, OBJECT.VY/OBJECT.PLANES | Page-image checked; only documented GRAPHMODE values asserted |
| 257, 261–268 | GET/PUT header/modes/bitplanes, hardware SPRITE | Page-image checked; PUT mask semantics remain unspecified |
| 352–354, 358 | DUMP selectors/output and MODE table | Page-image checked; runtime details remain unverified |
| 363–368, 371–373 | ALERT, FILESELECT, MOUSE, STICK/STRIG | Page-image checked; source-specific hardware behavior not runtime-tested |
| 398–404 | MENU() event payloads, ON MENU, SLEEP and handler registration | Page-image checked; printed timer mismatch retained |

## Research and implementation boundaries

- This index describes source evidence, not parser support, implementation status or runtime conformance.
- The German 1990 book's 3.0/V3.03 material is not silently promoted to 3.5.
- Compiler/linker tools are separate from interpreter language/API entries.
- Direct Amiga OS library wrappers need ABI/layout/runtime validation before implementation decisions.
- Known source disagreements and unresolved OCR/runtime questions remain visible in the extraction notes and the candidate manifest.
- Historical scans and OCR dumps must not be committed or redistributed. Only concise citations, source metadata, original summaries and independently authored tests belong in the repository.


## Selected command/function navigation

This is a compact name-to-page index for the principal interpreter families extracted in this pass. It is intentionally a **selected index**, not a claim to reproduce every entry of Appendix D. Where a command is part of the 3.5 addendum, the version-specific 3.5 table above takes precedence over the older book's page number.

| Names / family | Printed pages | Scope |
|---|---:|---|
| `INPUT`, `LINE INPUT`, `FORM INPUT`, `INKEY$`, `PRINT`, `WRITE`, `LOCATE`, `HTAB`, `VTAB` | 75–92 | Screen and keyboard I/O; 3.0/V3.03 source |
| `OPEN`, `CLOSE`, `INPUT #`, `LINE INPUT #`, `PRINT #`, `WRITE #`, `FIELD`, `GET`, `PUT`, `SEEK`, `RELSEEK`, `RECORD`, `BGET`, `BPUT` | 92–119 | File and channel operations; 3.0/V3.03 source |
| `INP`, `OUT` | 119 | Port I/O; 3.0/V3.03 source |
| `OPENW`, `CLOSEW`, `MOVEW`, `FULLW`, `LIMITW`, `TITLEW`, `FRONTW`, `SIZEW`, `WINDOW()` | 375–388 | Amiga window API; negative-screen OPENW variant separately confirmed in 3.5 addendum |
| `OPENS`, `CLOSES`, `SCREEN()`, `SCREEN` and screen geometry/mode operations | 383–388 | Amiga screen API; 3.0/V3.03 source |
| `MENU`, `MENUTEXT`, `MENUMENU`, `MENUKEY`, `MENUKILL`, `ON MENU`, `ON MENU GOSUB`, `ON MENU BUTTON GOSUB`, `ON MENU KEY GOSUB`, `SLEEP` | 389–404 | Menu creation and Intuition event dispatch; page-image verified selected event tables |
| `ALERT`, `FILESELECT`, `MOUSE`, `MOUSEX`, `MOUSEY`, `MOUSEK`, `STICK`, `STICK()`, `STRIG()` | 363–373 | Requesters and input devices; 3.0/V3.03 source |
| `PLOT`, `LINE`, `DRAW`, `DRAW$`, `BOX`, `PBOX`, `CIRCLE`, `PCIRCLE`, `ELLIPSE`, `PELLIPSE`, `POLYLINE`, `POLYFILL`, `POINT`, `SCROLL`, `TEXT`, `GRAPHMODE` | 225–256 | Raster and vector graphics; selected syntax page-image verified |
| `GET`, `PUT`, `SPRITE`, `VSYNC`, `OBJECT.*`, `ON COLLISION GOSUB` | 257–278 | Bitmap transfer, hardware sprites and Blitter objects; mask/runtime caveats retained |
| `OPTION BASE`, `DIM`, `ERASE`, `INSERT`, `QSORT`, `SSORT`, `ARRAYFILL`, `ARRPTR` | 291–304 | Arrays and sorting |
| `PEEK`, `POKE`, `DPEEK`, `DPOKE`, `LPEEK`, `LPOKE`, `MALLOC`, `MFREE`, `VARPTR`, `ABSOLUTE`, `INLINE` | 305–316 | Memory and pointers; low-level ABI/runtime validation required |
| `SOUND`, `WAVE`, `SAY`, `TRANSLATE$` | 139–144 | Sound and speech output |
| `DATA`, `READ`, `RESTORE`, `DEFBIT`, `DEFBYT`, `DEFINT`, `DEFSTR`, `DEFWRD`, `LOCAL`, `PROCEDURE`, `FUNCTION`, `DEFFN`, `CALL`, `EXEC`, `MONITOR`, `RCALL` | 145–191 | Program structure, declarations and external code |
| `DUMP`, `MODE`, `TRACE$`, `TRON`, `TROFF`, `ERROR`, `ERR`, `ERR$`, `RESUME` | 331–361 | Diagnostics, error handling and interpreter control |
| `MAT BASE`, `MAT CLR`, `MAT SET`, `MAT ONE`, `MAT READ`, `MAT PRINT`, `MAT INPUT`, `MAT CPY`, `MAT XCPY`, `MAT TRANS`, `MAT ADD`, `MAT SUB`, `MAT MUL`, `MAT NORM`, `MAT DET`, `MAT QDET`, `MAT RANG`, `MAT INV` | 13-1–13-41 (3.5 addendum) | Version-specific matrix/vector API; floating-point fields only |
| `FACT`, `VARIAT`, `COMBIN`, `PCOLOR`, `GURU`, `_DATA`, `_DATA=`, `_0`…`_9` | 13-33–13-36 (3.5 addendum) | Explicit additional 3.5 interpreter features |

Names are normalized only for navigation; this table does not settle OCR spellings where the source is damaged or internally inconsistent. For example, the book's error appendix uses `PERMUT` where Chapter 19 describes `VARIAT`, and the function inventory retains disputed `CVD/CVF` and `MKD/MKF` name slots. Consult the [candidate manifest](amiga-gfa-basic-candidates.toml) and the detailed source notes before using any spelling as a parser requirement.


## Resolved source discrepancies (2026-11-08)

- **MAT INV:** `MAT INV destination()=source()` is established by the 3.5 addendum's detailed definition and worked example (printed p. 13-39), and independently by the German book (p. 475). The reversed summary line on 3.5 p. 13-16 is a typo.
- **INTUITICKS:** use decimal `4194304` / hexadecimal `&H400000`. The repeated decimal `4194303` in event prose is a typo; the hexadecimal representation and German IDCMP table agree.
- **Error #86 / PERMUT:** `VARIAT` is the documented function name; `PERMUT` occurs only in the inconsistent error catalogue entry. No alias is established.
- **MENU attribute 64:** duplicate values are confirmed in the source. The runtime effect remains unknown and requires interpreter testing.
