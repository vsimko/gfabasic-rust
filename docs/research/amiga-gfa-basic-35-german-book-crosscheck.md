---
type: "Research Note"
title: "Amiga GFA-BASIC 3.5 — independent German-book cross-check"
---

# Amiga GFA-BASIC 3.5 — independent German-book cross-check

## Source and version boundary

Source: Wolf-Gideon Bleek, Martin Hecht, Uwe Litzkendorf, *Das große GFA-BASIC-Buch zum Amiga*, DATA BECKER, 1990, ISBN 3-89011-399-0. Supplied scan: 515 PDF pages.

The foreword scopes the book's main command reference to Amiga GFA-BASIC 3.0 (the introduction also refers to V3.03). This must not be generalized to the entire book: printed p. 451 opens Chapter 18 with the explicit statement that the following pages explain commands newly added in **Version 3.5**, and that the old commands have basically not changed. Printed p. 477 opens Chapter 19 and introduces additional commands and editor changes. These chapters are therefore a separately version-labelled 3.5 witness, not merely 3.0 material inferred from the cover.

The page images for printed pp. 451, 454, 477–480, and 488 were checked in the supplied scan. Page references below are printed book pages, which also happen to align with the PDF page numbers for these excerpts.

## 1. Matrix declarations — Chapter 18

Chapter 18, printed pp. 451–476, is explicitly labelled as new Version 3.5 material on p. 451. The page-image checked p. 454 visibly documents:

- `MAT BASE 0` and `MAT BASE 1` (index offset).
- `MAT CLR m()` (clear matrix elements to zero).

The chapter's extracted command descriptions additionally cover the matrix families already recorded from the English 3.5 addendum:

| Declaration family | German-book pages | Cross-check status |
|---|---:|---|
| `MAT BASE`, `MAT CLR`, `MAT SET`, `MAT ONE` | 454–456 | Chapter 18; pp. 454–455 image checked |
| `MAT READ`, `MAT PRINT`, `MAT INPUT` | 456–458 | Page images checked; all three command headings and descriptions visible |
| `MAT CPY`, `MAT XCPY`, `MAT TRANS` | 459–464 | Page images checked; command headings and worked examples visible |
| `MAT ADD`, `MAT SUB`, `MAT MUL` | 465–470 | Page images checked; command headings and worked examples visible |
| `MAT NORM`, `MAT DET`, `MAT QDET`, `MAT RANG`, `MAT INV` | 472–475 | Page images checked; command headings and definitions visible |

This is corroboration of the 3.5 declaration families already present in the primary addendum inventory; it does not create additional registry entries. The page-image review confirms the command headings and the displayed forms/examples on the cited pages. The detailed English-manual audit remains authoritative for exact syntax and edge-case semantics.

## 2. Combinatorics and DATA-pointer declarations — Chapter 19

Chapter 19 begins on printed p. 477. Section 19.1, “Mathematische Befehle”, documents:

| Declaration | German-book pages | Source-derived detail |
|---|---:|---|
| `FACT(n)` | 477–478 | Factorial; explicitly defines `0!` as 1; p. 477 image checked |
| `VARIAT(m,k)` | 478 | Variations without repetition; requires `m >= k`; p. 478 image checked |
| `COMBIN(m,k)` | 479 | Combinations without repetition; requires `m >= k`; p. 479 image checked |
| `_DATA`, `_DATA=` | 479–480 | Read and restore a previously saved DATA-pointer position; p. 479–480 images checked |

The examples and prose independently corroborate the three 3.5 combinatorics functions and both DATA-pointer declarations in the English addendum. The German book calls the read-only pointer value `DATA` in prose and presents the actual BASIC identifier as `_DATA`; retain the exact underscore spelling from the command heading and examples.

## 3. Source discrepancy: error 86 and PERMUT

The same book's error appendix, printed p. 488, says:

> 86 FACT/COMBIN/PERMUT nicht definiert

This conflicts internally with Chapter 19, which explicitly documents `FACT` and `COMBIN` as new 3.5 functions. The English 3.5 addendum documents `FACT`, `VARIAT`, and `COMBIN`; it does not document `PERMUT` as a function or alias.

**Disposition:** treat error 86's function-name list as internally inconsistent/stale catalogue text. Do not add `PERMUT` to the registry or treat it as an alias. Whether the interpreter accepts that spelling remains unverified and requires a version-identified runtime observation.

## 4. Registry and compatibility impact

- No new 3.5 declaration is added by this cross-check: the declarations above already exist in the English-addendum-derived inventory/registry.
- The German book provides independent, explicitly 3.5-labelled corroboration for the matrix, combinatorics, and DATA-pointer families.
- Do not reclassify the book's general command reference as 3.5. Its foreword remains explicit that the main reference documents interpreter 3.0 (with V3.03 also mentioned in the introduction).
- The 3.5 chapters do not establish runtime conformance, all overload edge cases, or compatibility of every base-manual command.
- The earlier OCR-only warning for the Chapter 18 matrix-multiplication examples is superseded by the page-image checks recorded in the 3.5 primary addendum inventory. The additional check of German-book p. 455 confirms the printed `MAT SET m()=x` and `MAT ONE m()` forms and their descriptions. The book remains independent 3.5-labelled corroboration, not runtime-conformance evidence.

## 5. Page-image verification completed

- p. 451: Chapter 18 explicitly labels its additions as Version 3.5 commands.
- p. 454: `MAT BASE` and `MAT CLR`.
- p. 455: `MAT SET m()=x` and `MAT ONE m()` headings, syntax, and descriptions.
- pp. 456–458: `MAT READ`, `MAT PRINT`, and `MAT INPUT` headings, syntax, and descriptions.
- pp. 459–464: `MAT CPY`, `MAT XCPY`, and `MAT TRANS` headings and examples.
- pp. 465–470: `MAT ADD`, `MAT SUB`, and `MAT MUL` headings and examples.
- pp. 472–475: `MAT NORM`, `MAT DET`, `MAT QDET`, `MAT RANG`, and `MAT INV` headings and definitions.
- p. 477: Chapter 19 and `FACT`.
- p. 478: `VARIAT`.
- p. 479: `COMBIN`, `_DATA`, and `_DATA=`.
- p. 480: DATA-pointer semantics and example.
- p. 488: page image confirms error 86's exact wording, `FACT/COMBIN/PERMUT nicht definiert`.

This note records documentary corroboration only. It does not claim that the original interpreter has been executed.

## 5. Conversion functions — general reference (printed pp. 285–286)

The page images for printed pp. 285–286 were checked directly. These pages are in the book's general command reference, whose foreword scopes the main reference to Amiga GFA-BASIC 3.0; they are **not** part of the explicitly 3.5-labelled Chapters 18–19.

The printed p. 285 heading lists `CVI()`, `CVL()`, `CVS()`, and `CVD()`. Its table specifies:

- `CVI("2 Zeichen")`: 16-bit integer.
- `CVL("4 Zeichen")`: 32-bit integer.
- `CVS("4 Zeichen")`: real value in Amiga-BASIC format.
- `CVD("8 Zeichen")`: real value in MBASIC or GFA-BASIC format.

The same paragraph says these are the inverse functions of `MKI$/MKL$/MKS$/MKD$`.

Printed p. 286 separately lists `MKI$()`, `MKL$()`, `MKS$()`, and `MKD$()`, with corresponding output widths/formats: 2-byte integer string, 4-byte integer string, 4-byte Amiga-BASIC real string, and 8-byte MBASIC/GFA-BASIC real string.

This independently supports the `CVS` ↔ `MKS$` pairing for 4-byte Amiga-BASIC real values, and the `CVD` ↔ `MKD$` pairing for 8-byte MBASIC/GFA-BASIC real values, in the book's 3.0 general-reference scope. It does **not** establish 3.5 availability.

**Cross-source discrepancy remains open:** the English base-manual passage on printed pp. 2-18–2-19 contains `CVF` in prose and `MKF$` in its reverse-function sentence, while its syntax/table also show `CVS`, `MKS$`, and `MKD$`. The German book's page images support `CVS` and `MKS$`, and separately `CVD` and `MKD$`; they do not document `CVF` or `MKF$` on these pages. Preserve the English spellings as conflicting printed evidence. Do not silently correct them, infer aliases, or promote any of these base-reference names as 3.5 declarations on this evidence alone.


## 5. Inherited statement/operator forms used in 3.5 examples

The page-image-checked Chapter 18 vector example (printed p. 471) directly uses ordinary `PRINT`, `LOCATE 1,12`, `COLOR 3`/`COLOR 1`, `DRAW TO`, and an ascending `FOR ... NEXT` loop. The Chapter 19 DATA-pointer example (p. 480) uses `WHILE _DATA ... WEND` and a descending `FOR ... STEP -1 ... NEXT` loop. Chapter 18 MAT ADD/SUB examples use numeric `+` and `-` (pp. 466–467), and MAT MUL uses `*` (p. 469); the normalization example uses `1/SQR(l)` (p. 473). These are direct-use witnesses for only the forms shown.

The candidate manifest splits grouped families so unshown siblings are not promoted: `PRINT AT`/`WRITE`, `SETCOLOR`, `PLOT`/`LINE`/other `DRAW` forms, and prefix unary plus/minus remain 3.0-only evidence. The detailed form-by-form limits are in [the inherited statement/operator audit](amiga-gfa-basic-35-statement-operator-audit.md). No runtime-conformance or full compatibility claim follows from these examples.


## 6. Additional inherited statements used in 3.5 examples

The Chapter 18 matrix example on printed p. 458 uses `OPTION BASE 0`, `DIM`, `OPEN "O"`, `OPEN "I"`, and `CLOSE #1`. The vector example on p. 471 uses numeric `DATA` lines, `PAUSE 50`, a `PROCEDURE drawp` definition, `@drawp` procedure calls, `RETURN`, and top-level `END`. The Chapter 19 DATA-pointer example on p. 480 uses `DIM`, `INC`, `RESTORE`, and scalar `READ`.

The new candidate records are deliberately form-limited: `DIM` covers the shown numeric dimensions; `OPTION BASE` only base 0; `OPEN` only output/input modes; `CLOSE` only the numbered channel form; `DATA` only numeric values; `READ` only a scalar read; `RESTORE` only the bare reset; `PAUSE` only the shown numeric argument; `PROCEDURE / RETURN` only the parameterless example; `@procedure-call` only the shown prefix-call form; and `END` only the shown termination form. Unused siblings (`DEC`, `DELAY`, `GOSUB`, `EDIT`, `STOP`) remain 3.0-only candidates. Details and exact locators are in [the example-used statement audit](amiga-gfa-basic-35-example-used-statements-audit.md).


## 7. English 3.5 addendum screen-clear use

The Chapter 13 normalization example begins with `CLS` before `MAT READ` (printed p. 13-24; page image checked). This directly witnesses the ordinary screen-clear statement in the 3.5 addendum. It does not demonstrate the optional `CLS #n` redirection form documented in the 3.0 base manual; the candidate record retains that qualification.
