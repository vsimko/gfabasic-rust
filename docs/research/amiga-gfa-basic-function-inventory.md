---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 built-in function inventory"
---

# Amiga GFA BASIC 3.5 built-in function inventory

> Source-derived inventory, not an implementation or runtime-conformance claim. The machine-readable source of truth is `amiga-gfa-basic-functions.toml`.

## Count

| Population | Unique names |
|---|---:|
| Base-manual functions (3.0 text included in the supplied 3.5 manual) | 113 |
| Explicit functions added in the 3.5 addendum | 4 |
| **All names listed across the combined manual** | **117** |
| Compiler-only names excluded from the interpreter population (`CINT`, `CFLOAT`) | 2 |
| **Interpreter function candidates (including disputed slots)** | **115** |
| Unambiguous interpreter function names | 113 |
| Disputed source-name slots | 2 |

Of the 113 base-manual identifiers, eight (`RAND`, `STR$`, `SUCC`, `STRING$`, `HEX$`, `RND`, `INT`, and `SQR`) have direct 3.5 use/reference evidence. Five are evidenced by the English interpreter addendum and three by the German companion book's 3.5-specific Chapter 18. These are overlapping evidence confirmations, not additional identifiers, and do not change the 117/115 population counts.

The two disputed slots are the CVx conversion spelling (`CVD` in the index versus `CVF` in the Chapter 2-18 prose) and the MKx$ conversion spelling (`MKD$` in the p. 2-18 signature block versus `MKF$` in the p. 2-19 prose). Each slot is counted once; neither spelling conflict is silently resolved.

## Counting boundary

- Included: built-in expression functions and the manual's function-like typed-conversion forms.
- Excluded from the interpreter population: `CINT` and `CFLOAT`, which the manual describes as compiler-only.
- Excluded from this function inventory: statements/commands, operators used only as operators, constants, predefined variables, editor commands, user-defined procedures, and Amiga OS library routines.
- Alternate syntactic forms and overloads do not create additional unique names.
- Base-manual entries originate in the 3.0 text included in the supplied 3.5 manual. They are candidates, not proof that every detail remains supported by the 3.5 interpreter.
- Runtime observations and source discrepancies remain separate evidence dimensions.

## Base-manual entries and separate 3.5 evidence

| Function | Printed source page(s) | Evidence |
|---|---|---|
| `ABS` | 4-12 | base-manual-entry |
| `ACOS` | 4-5 | base-manual-entry |
| `ADD` | 4-3 | base-manual-entry |
| `AND` | 4-23 | base-manual-entry |
| `ARRPTR` | 2-33 | base-manual-entry |
| `ASC` | 2-13 | base-manual-entry |
| `ASIN` | 4-5 | base-manual-entry |
| `ATN` | 4-5 | base-manual-entry |
| `BCHG` | 4-19 | base-manual-entry |
| `BCLR` | 4-19 | base-manual-entry |
| `BIN$` | 2-14 | base-manual-entry |
| `BSET` | 4-19 | base-manual-entry |
| `BTST` | 4-19 | base-manual-entry |
| `BYTE` | 2-37, 4-26 | base-manual-entry |
| `CARD` | 2-37, 4-26 | base-manual-entry |
| `CFLOAT` | 2-17 | base-manual-entry (compiler-only): The manual explicitly says this conversion is only meaningful to the compiler; excluded from the interpreter-specific count. |
| `CHAR` | 2-37 | base-manual-entry |
| `CHR$` | 2-13 | base-manual-entry |
| `CINT` | 2-17 | base-manual-entry (compiler-only): The manual explicitly says this conversion is only meaningful to the compiler; excluded from the interpreter-specific count. |
| `COS` | 4-5 | base-manual-entry |
| `COSQ` | 4-6 | base-manual-entry |
| `CVD` | 2-18 | disputed-name: The printed source itself is inconsistent: the command index points to CVD (printed p. 2-18); the p. 2-18 prose names CVF; the following application list names CVS. The function signature block lists CVI/CVL but no floating-point CVx spelling. Retain one unresolved CVx slot; do not treat CVD, CVF, and CVS as three confirmed identifiers. This is a source discrepancy, not merely OCR uncertainty. |
| `CVI` | 2-18 | base-manual-entry |
| `CVL` | 2-18 | base-manual-entry |
| `CVS` | 2-18 | base-manual-entry |
| `DEG` | 4-7 | base-manual-entry |
| `DFREE` | 6-4 | base-manual-entry |
| `DIM?` | 2-9 | base-manual-entry |
| `DIR$` | 6-4 | base-manual-entry |
| `DIV` | 4-3 | base-manual-entry |
| `DOUBLE` | 2-37 | base-manual-entry |
| `DPEEK` | 2-36 | base-manual-entry |
| `EOF` | 6-9 | base-manual-entry |
| `EQV` | 4-24 | base-manual-entry |
| `EVEN` | 4-15 | base-manual-entry |
| `EXIST` | 6-7 | base-manual-entry |
| `EXP` | 4-9 | base-manual-entry |
| `FIX` | 4-13 | base-manual-entry |
| `FLOAT` | 2-37 | base-manual-entry |
| `FRAC` | 4-13 | base-manual-entry |
| `FRE` | 2-29 | base-manual-entry |
| `HEX$` | 2-14 | confirmed-by-3.5-addendum-use |
| `IMP` | 4-24 | base-manual-entry |
| `INKEY$` | 5-2 | base-manual-entry |
| `INP` | 6-13 | base-manual-entry |
| `INPUT$` | 6-14 | base-manual-entry |
| `INSTR` | 4-30 | base-manual-entry |
| `INT` | 2-37, 4-13 | confirmed-by-3.5-addendum-use |
| `LEFT$` | 4-27 | base-manual-entry |
| `LEN` | 4-34 | base-manual-entry |
| `LOC` | 6-9 | base-manual-entry |
| `LOF` | 6-9 | base-manual-entry |
| `LOG` | 4-9 | base-manual-entry |
| `LOG10` | 4-9 | base-manual-entry |
| `LONG` | 2-37 | base-manual-entry |
| `LPEEK` | 2-36 | base-manual-entry |
| `LPOS` | 5-19 | base-manual-entry |
| `MALLOC` | 2-41 | base-manual-entry |
| `MAX` | 4-16 | base-manual-entry |
| `MFREE` | 2-41 | base-manual-entry |
| `MID$` | 4-28 | base-manual-entry |
| `MIN` | 4-16 | base-manual-entry |
| `MKD$` | 2-18, 2-19 | base-manual-entry: MKD$ appears in both the printed p. 2-18 signature block and the p. 2-19 prose list; its identifier is not the disputed MKx spelling. |
| `MKF$` | 2-18, 2-19 | disputed-name: The p. 2-18 signature block lists MKS$ and MKD$, while the p. 2-19 prose additionally lists MKF$. Retain MKF$ as one unresolved MKx spelling slot; do not assume it is an alias or a confirmed additional identifier without runtime/edition evidence. |
| `MKI$` | 2-18 | base-manual-entry |
| `MKL$` | 2-18 | base-manual-entry |
| `MKS$` | 2-18 | base-manual-entry |
| `MOD` | 4-3 | base-manual-entry |
| `MUL` | 4-3 | base-manual-entry |
| `OCT$` | 2-14 | base-manual-entry |
| `ODD` | 4-15 | base-manual-entry |
| `OR` | 4-23 | base-manual-entry |
| `PEEK` | 2-36 | base-manual-entry |
| `POINT` | 8-18 | base-manual-entry |
| `POS` | 5-13 | base-manual-entry |
| `PRED` | 4-11 | base-manual-entry |
| `RAD` | 4-7 | base-manual-entry |
| `RAND` | 4-17 | confirmed-by-3.5-addendum-use |
| `RANDOM` | 4-17 | base-manual-entry |
| `RIGHT$` | 4-27 | base-manual-entry |
| `RINSTR` | 4-31 | base-manual-entry |
| `RND` | 4-17 | confirmed-by-3.5-addendum-use |
| `ROL` | 4-22 | base-manual-entry |
| `ROR` | 4-22 | base-manual-entry |
| `ROUND` | 4-14 | base-manual-entry |
| `SCREEN` | 9-24 | base-manual-entry |
| `SGN` | 4-12 | base-manual-entry |
| `SHL` | 4-21 | base-manual-entry |
| `SHR` | 4-21 | base-manual-entry |
| `SIN` | 4-5 | base-manual-entry |
| `SINGLE` | 2-37 | base-manual-entry |
| `SINQ` | 4-6 | base-manual-entry |
| `SPACE$` | 4-32 | base-manual-entry |
| `SPC` | 4-32 | base-manual-entry |
| `SQR` | 4-8 | confirmed-by-3.5-addendum-use |
| `STR$` | 2-15 | confirmed-by-3.5-addendum-use |
| `STRING$` | 4-32 | confirmed-by-3.5-addendum-use |
| `SUB` | 4-3 | base-manual-entry |
| `SUCC` | 4-11 | confirmed-by-3.5-addendum-use |
| `SWAP` | 4-25 | base-manual-entry |
| `TAB` | 5-13 | base-manual-entry |
| `TAN` | 4-5 | base-manual-entry |
| `TRANSLATE$` | 10-9 | base-manual-entry |
| `TRIM$` | 4-34 | base-manual-entry |
| `TRUNC` | 4-13 | base-manual-entry |
| `TYPE` | 2-30 | base-manual-entry |
| `UPPER$` | 4-35 | base-manual-entry |
| `VAL` | 2-16 | base-manual-entry |
| `VAL?` | 2-16 | base-manual-entry |
| `VARPTR` | 2-33 | base-manual-entry |
| `WINDOW` | 9-35 | base-manual-entry |
| `WORD` | 2-37, 4-26 | base-manual-entry |
| `XOR` | 4-23 | base-manual-entry |

The base-manual table above is a 3.0-derived candidate inventory. Its entries deliberately have no 3.5 registry IDs: the shared PDF's base chapters do not independently establish 3.5 availability, and absence from the addendum is not evidence of absence. Keep these names out of the 3.5 API registry until a separate 3.5-specific source or a correctly versioned runtime observation supports the declaration.

## Explicit 3.5 additions

| Function | Printed source page(s) | Registry declaration | Evidence |
|---|---|---|---|
| `COMBIN` | 13-33–13-34 | `gfa.language.math.combin` | explicit-3.5-addendum: Combination function. |
| `FACT` | 13-33–13-34 | `gfa.language.math.fact` | explicit-3.5-addendum: Factorial function; 0! is explicitly defined as 1. |
| `GURU` | 13-36 | `gfa.amiga.system.guru-number` (Amiga profile) | explicit-3.5-addendum: GURU-number function; distinct from the GURU enable/disable statement. |
| `VARIAT` | 13-33–13-34 | `gfa.language.math.variat` | explicit-3.5-addendum: Permutation function. |

## Primary source and caveats

- `GFA BASIC 3.5 Interpreter`, supplied 469-page manual: the introductory page identifies the included software as GFA BASIC 3.5 and identifies Chapter 13 as an addendum describing extra features. The base text is the earlier 3.0 manual.
- Base function index: printed pp. 12-8–12-18. Detailed base pages are listed per function above.
- Explicit 3.5 additions: printed pp. 13-33–13-36.
- The separate 1990 German Amiga book's general Chapters 1–17 are historical 3.0/3.03 material. Chapters 18–19 explicitly cover 3.5-specific features: Chapter 18 confirms RND, INT, and SQR by example and corroborates the STR$ formatting reference; these are overlapping evidence, not new identifiers.
- Chapter 11's Amiga OS library index is excluded: it catalogs OS calls, not built-in GFA BASIC language functions.

Machine-readable source: `amiga-gfa-basic-functions.toml`. Validation: `python scripts/validate_amiga_function_inventory.py`.


## 3.5 evidence audit

Eight inherited functions (`RAND`, `STR$`, `SUCC`, `STRING$`, `HEX$`, `RND`, `INT`, and `SQR`) are directly used or referenced in the version-specific 3.5 sources. Their demonstrated forms and limits are recorded in [the declaration-level audit](amiga-gfa-basic-35-base-function-use-audit.md). This evidence confirms only the forms shown or referenced, not every overload or edge case; the entries retain their 3.0 base-manual origin.
