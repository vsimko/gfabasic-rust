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
| Book applicability, BASIC introduction, language foundations | 13–65 | [§1–§6](amiga-gfa-basic-german-book-command-api-extraction.md#1-high-value-command-families) |
| Screen/input/output, files, channels, ports | 75–119 | [§1](amiga-gfa-basic-german-book-command-api-extraction.md#1-high-value-command-families), [§14](amiga-gfa-basic-german-book-command-api-extraction.md#14-program-text-data-records-and-variable-declarations-chapter-63-64) |
| AmigaDOS library calls | 120–132 | [§11](amiga-gfa-basic-german-book-command-api-extraction.md#11-amigados-library-detailed-chapter-57-extraction) |
| Printer and hardcopy | 132–138 | [§12](amiga-gfa-basic-german-book-command-api-extraction.md#12-printer-output-and-hardcopy-chapter-58) |
| Sound, WAVE, SAY and speech | 139–143 | [§13](amiga-gfa-basic-german-book-command-api-extraction.md#13-sound-and-speech-output-chapter-59-510) |
| Program structure, loops, branches, procedures and functions | 145–191 | [§7](amiga-gfa-basic-german-book-command-api-extraction.md#7-interpreter-language-constructs-chapter-6), [§15](amiga-gfa-basic-german-book-command-api-extraction.md#15-procedures-functions-and-external-code-chapter-65-66) |
| Text and string operations | 193–200 | [§16](amiga-gfa-basic-german-book-command-api-extraction.md#16-text-operations-chapter-7-printed-pp-193-200) |
| Operators, arithmetic, numeric and bit functions | 201–224 | [§17](amiga-gfa-basic-german-book-command-api-extraction.md#17-arithmetic-numeric-and-bit-operations-chapter-8-printed-pp-201-224) |
| Graphics definitions, drawing, DRAW$, SCROLL, GRAPHMODE | 225–256 | [§18](amiga-gfa-basic-german-book-command-api-extraction.md#18-graphics-definitions-and-drawing-operations-chapter-9-printed-pp-225-256), [§37](amiga-gfa-basic-german-book-command-api-extraction.md#37-page-image-validation-graphics-scrolling-draw-grammar-and-bob-velocityplanes-printed-pp-240-245250-272-273) |
| GET/PUT bitmap strings, hardware sprites, BOB/OBJECT | 257–278 | [§19](amiga-gfa-basic-german-book-command-api-extraction.md#19-bitmap-transfer-sprites-and-blitter-objects-chapter-941-95-printed-pp-261-278), [§38](amiga-gfa-basic-german-book-command-api-extraction.md#38-page-image-validation-getput-bitmap-format-and-hardware-sprites-printed-pp-257-268) |
| Number conversion and formatting | 279–290 | [§20](amiga-gfa-basic-german-book-command-api-extraction.md#20-data-conversion-and-number-format-functions-chapter-10-printed-pp-279-290) |
| Arrays, OPTION BASE, sorting | 291–304 | [§21](amiga-gfa-basic-german-book-command-api-extraction.md#21-array-operations-and-sorting-chapter-111-printed-pp-291-304) |
| Memory, typed access, allocation, pointers | 305–316 | [§22](amiga-gfa-basic-german-book-command-api-extraction.md#22-memory-access-allocation-and-pointer-operations-chapter-112-114-printed-pp-305-316) |
| Amiga Exec tasks, signals, ports, lists, libraries, devices | 317–328 | [§23](amiga-gfa-basic-german-book-command-api-extraction.md#23-amiga-exec-library-overview-chapter-115-printed-pp-317-328) |
| Program control, errors, debugging, DUMP, MODE | 331–361 | [§8](amiga-gfa-basic-german-book-command-api-extraction.md#8-program-control-and-error-handling-chapter-12), [§24](amiga-gfa-basic-german-book-command-api-extraction.md#24-program-control-debugging-and-interpreter-configuration-chapter-12-printed-pp-331-361), [§39](amiga-gfa-basic-german-book-command-api-extraction.md#39-page-image-validation-dump-diagnostics-and-mode-locale-bits-printed-pp-352-354-358) |
| ALERT, FILESELECT, mouse, joystick, STICK/STRIG | 363–374 | [§25](amiga-gfa-basic-german-book-command-api-extraction.md#25-user-interaction-and-input-devices-chapter-13-printed-pp-363-374-partial), [§40–§41](amiga-gfa-basic-german-book-command-api-extraction.md#40-page-image-validation-alert-fileselect-mouse-and-joystick-input-printed-pp-363-373) |
| Windows, screens, IDCMP, Window flags | 375–388 | [§26](amiga-gfa-basic-german-book-command-api-extraction.md#26-window-and-screen-programming-detailed-flags-and-behavior-from-chapters-14142-printed-pp-375-388), [§36](amiga-gfa-basic-german-book-command-api-extraction.md#36-page-image-validation-idcmp-flags-window-flags-and-menu-attributes-printed-pp-378379-389) |
| MENU creation and attributes | 389–393 | [§27](amiga-gfa-basic-german-book-command-api-extraction.md#27-basic-menu-programming-chapter-15-printed-pp-389-393), [§36](amiga-gfa-basic-german-book-command-api-extraction.md#36-page-image-validation-idcmp-flags-window-flags-and-menu-attributes-printed-pp-378379-389) |
| Intuition event handling, MENU() payload, dispatch | 395–404 | [§28–§29](amiga-gfa-basic-german-book-command-api-extraction.md#29-intuition-event-classes-and-dispatch-chapter-16-continuation-printed-pp-397-404), [§42–§43](amiga-gfa-basic-german-book-command-api-extraction.md#42-page-image-validation-menu1-event-values-and-payload-fields-printed-pp-398-402) |
| Compiler and linker (not interpreter API) | 405–450 | [§33](amiga-gfa-basic-german-book-command-api-extraction.md#33-compiler-tooling-not-interpreter-language-api-chapter-17-printed-pp-405-450) |
| 3.5 matrix/vector operations (compare with separate 3.5 addendum) | 451–476 | [§30](amiga-gfa-basic-german-book-command-api-extraction.md#30-gfa-basic-35-additions-vector-and-matrix-operations-chapter-18-printed-pp-451-476), [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) |
| FACT, VARIAT, COMBIN, _DATA, editor additions | 477–482 | [§31](amiga-gfa-basic-german-book-command-api-extraction.md#31-gfa-basic-35-additions-mathematical-helpers-data-pointer-and-editor-commands-chapter-19-printed-pp-477-482) |
| Error appendices, command index and back matter | 483–508 | [§32](amiga-gfa-basic-german-book-command-api-extraction.md#32-version-35-error-catalogue-and-command-index-cross-check-appendices-bd-printed-pp-487-497), [§35](amiga-gfa-basic-german-book-command-api-extraction.md#35-final-german-book-index-and-appendix-audit-printed-pp-486-508) |

## Version-specific 3.5 addendum index

The *GFA BASIC 3.5 Interpreter* manual contains a 3.0 base plus explicitly identified 3.5 additions. Use its printed Chapter 13 pagination; the combined update scan uses Chapter 13 for matrix operations and Chapter 14 for further commands.

| 3.5 feature | Supplied interpreter manual pages | Combined update scan pages | Evidence note |
|---|---:|---:|---|
| Editor row layout, closed procedure/function search, listing changes, tab keys | 1-25 | 1-25 | Explicit 3.5 editor update; not interpreter language syntax |
| Matrix base and initialization: MAT BASE, MAT CLR/SET/ONE | 13-1–13-3 | 13-1–13-3 | Floating-point 1D/2D fields only |
| Matrix READ/PRINT/INPUT | 13-3–13-6 | 13-3–13-6 | MAT PRINT punctuation visually checked; MAT INPUT is ASCII-file input |
| Matrix copy, transpose and vector special cases | 13-8–13-20 | 13-8–13-20 | MAT CPY/XCPY/TRANS and separate MAT COPY forms are distinguished by source |
| Matrix arithmetic and multiplication | 13-22–13-33 | 13-22–13-33 | Element-wise, scalar, matrix and vector forms; preserve uncertain operand orientation |
| Matrix norm, determinant, quick determinant, rank and inverse | 13-31–13-41 | 13-31–13-41 | MAT INV direction remains disputed between summary and detailed example |
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
| Inverse | MAT INV operand direction differs between summary and worked example | Matrix chapter adds a separate witness | Keep disputed; do not choose a direction |
| FACT/VARIAT/COMBIN vs error #86 | Chapter 14 function list | Error appendix says FACT/COMBIN/PERMUT; Chapter 19 says FACT/VARIAT/COMBIN | Keep PERMUT/VARIAT discrepancy unresolved |
| MENU attribute value 64 | Not part of 3.5 addendum | German page 389 prints 64 for both clear-checkmark and activate-item | Preserve source duplicate; do not silently repair |
| Intuition timer event | Not part of 3.5 addendum | p.378 IDCMP INTUITICKS=4194304; p.402 timer request=4194303 | Preserve and flag source mismatch |

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
