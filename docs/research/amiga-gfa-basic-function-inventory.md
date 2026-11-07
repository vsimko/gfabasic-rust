# Amiga GFA BASIC 3.5 built-in function inventory

> Source-derived inventory, not an implementation or runtime-conformance claim.

## Count

| Population | Unique names |
|---|---:|
| Base-manual functions (3.0 text included in the supplied 3.5 manual) | 112 |
| Explicit functions added in the 3.5 addendum | 4 |
| **All names listed across the combined manual** | **116** |
| Compiler-only names excluded from the interpreter population (`CINT`, `CFLOAT`) | 2 |
| **Interpreter function candidates** | **114** |

Of the 114 interpreter candidates, 113 have an unambiguous name in the source. One slot is disputed: the appendix table lists `CVD()`, while the Chapter 2-18 prose refers to `CVF` and the signature list does not settle the name. It is counted as one candidate, not two.

The count includes the manual's function-like typed conversions (`BYTE{}`, `CARD{}`, `CHAR{}`, `DOUBLE{}`, `FLOAT{}`, `INT{}`, `LONG{}`, `SINGLE{}`, `WORD{}`). Alternate forms such as `BYTE()` and `BYTE{}` are grouped under one identifier. `CINT` and `CFLOAT` are documented but explicitly described as meaningful only to the compiler, so they are not included in the interpreter count.

## Counting boundary

- Included: built-in expression functions and the manual's typed-conversion forms.
- Excluded: statements/commands, operators used only as operators, constants (`PI`, `TRUE`, `FALSE`), predefined variables (`TIMER`, `DATES$`, mouse state, etc.), editor commands, and Amiga OS library routines.
- User-defined `FUNCTION`/`DEFFN` procedures are not built-in functions and are not counted.
- Overloads and alternate spellings do not create additional unique function names.
- This is a manual inventory, not proof that every base-manual 3.0 detail is unchanged in every 3.5 interpreter build. Runtime observations and disputed source details remain separate evidence dimensions.

## Base-manual entries

The following names are taken from the alphabetical *Table of commands* (printed pp. 12-8–12-18) and the corresponding feature pages in the 3.0 base text of the supplied 3.5 interpreter manual.

| Function | Printed source page(s) | Evidence note |
|---|---|---|
| `ABS` | 4-12 | Base-manual function entry |
| `ACOS` | 4-5 | Base-manual function entry |
| `ADD` | 4-3 | Base-manual function entry |
| `AND` | 4-23 | Base-manual function entry |
| `ARRPTR` | 2-33 | Base-manual function entry |
| `ASC` | 2-13 | Base-manual function entry |
| `ASIN` | 4-5 | Base-manual function entry |
| `ATN` | 4-5 | Base-manual function entry |
| `BCHG` | 4-19 | Base-manual function entry |
| `BCLR` | 4-19 | Base-manual function entry |
| `BIN$` | 2-14 | Base-manual function entry |
| `BSET` | 4-19 | Base-manual function entry |
| `BTST` | 4-19 | Base-manual function entry |
| `BYTE` | 2-37, 4-26 | Alternate typed-conversion form grouped under this name |
| `CARD` | 2-37, 4-26 | Alternate typed-conversion form grouped under this name |
| `CFLOAT` | 2-17 | The manual explicitly says this conversion is only meaningful to the compiler; excluded from the interpreter-specific count. |
| `CHAR` | 2-37 | Alternate typed-conversion form grouped under this name |
| `CHR$` | 2-13 | Base-manual function entry |
| `CINT` | 2-17 | The manual explicitly says this conversion is only meaningful to the compiler; excluded from the interpreter-specific count. |
| `COS` | 4-5 | Base-manual function entry |
| `COSQ` | 4-6 | Base-manual function entry |
| `CVD` | 2-18 | The appendix table lists CVD(); the Chapter 2-18 prose says CVF and its signature list omits CVD/CVF. Counted as one unresolved CVx function slot, not as two confirmed names. |
| `CVI` | 2-18 | Base-manual function entry |
| `CVL` | 2-18 | Base-manual function entry |
| `CVS` | 2-18 | Base-manual function entry |
| `DEG` | 4-7 | Base-manual function entry |
| `DFREE` | 6-4 | Base-manual function entry |
| `DIM?` | 2-9 | Base-manual function entry |
| `DIR$` | 6-4 | Base-manual function entry |
| `DIV` | 4-3 | Base-manual function entry |
| `DOUBLE` | 2-37 | Alternate typed-conversion form grouped under this name |
| `DPEEK` | 2-36 | Base-manual function entry |
| `EOF` | 6-9 | Base-manual function entry |
| `EQV` | 4-24 | Base-manual function entry |
| `EVEN` | 4-15 | Base-manual function entry |
| `EXIST` | 6-7 | Base-manual function entry |
| `EXP` | 4-9 | Base-manual function entry |
| `FIX` | 4-13 | Base-manual function entry |
| `FLOAT` | 2-37 | Alternate typed-conversion form grouped under this name |
| `FRAC` | 4-13 | Base-manual function entry |
| `FRE` | 2-29 | Base-manual function entry |
| `HEX$` | 2-14 | Base-manual function entry |
| `IMP` | 4-24 | Base-manual function entry |
| `INKEY$` | 5-2 | Base-manual function entry |
| `INP` | 6-13 | Base-manual function entry |
| `INPUT$` | 6-14 | Base-manual function entry |
| `INSTR` | 4-30 | Base-manual function entry |
| `INT` | 2-37, 4-13 | Alternate typed-conversion form grouped under this name |
| `LEFT$` | 4-27 | Base-manual function entry |
| `LEN` | 4-34 | Base-manual function entry |
| `LOC` | 6-9 | Base-manual function entry |
| `LOF` | 6-9 | Base-manual function entry |
| `LOG` | 4-9 | Base-manual function entry |
| `LOG10` | 4-9 | Base-manual function entry |
| `LONG` | 2-37 | Alternate typed-conversion form grouped under this name |
| `LPEEK` | 2-36 | Base-manual function entry |
| `LPOS` | 5-19 | Base-manual function entry |
| `MALLOC` | 2-41 | Base-manual function entry |
| `MAX` | 4-16 | Base-manual function entry |
| `MFREE` | 2-41 | Base-manual function entry |
| `MID$` | 4-28 | Base-manual function entry |
| `MIN` | 4-16 | Base-manual function entry |
| `MKD$` | 2-18 | Base-manual function entry |
| `MKI$` | 2-18 | Base-manual function entry |
| `MKL$` | 2-18 | Base-manual function entry |
| `MKS$` | 2-18 | Base-manual function entry |
| `MOD` | 4-3 | Base-manual function entry |
| `MUL` | 4-3 | Base-manual function entry |
| `OCT$` | 2-14 | Base-manual function entry |
| `ODD` | 4-15 | Base-manual function entry |
| `OR` | 4-23 | Base-manual function entry |
| `PEEK` | 2-36 | Base-manual function entry |
| `POINT` | 8-18 | Base-manual function entry |
| `POS` | 5-13 | Base-manual function entry |
| `PRED` | 4-11 | Base-manual function entry |
| `RAD` | 4-7 | Base-manual function entry |
| `RAND` | 4-17 | Base-manual function entry |
| `RANDOM` | 4-17 | Base-manual function entry |
| `RIGHT$` | 4-27 | Base-manual function entry |
| `RINSTR` | 4-31 | Base-manual function entry |
| `RND` | 4-17 | Base-manual function entry |
| `ROL` | 4-22 | Base-manual function entry |
| `ROR` | 4-22 | Base-manual function entry |
| `ROUND` | 4-14 | Base-manual function entry |
| `SCREEN` | 9-24 | Base-manual function entry |
| `SGN` | 4-12 | Base-manual function entry |
| `SHL` | 4-21 | Base-manual function entry |
| `SHR` | 4-21 | Base-manual function entry |
| `SIN` | 4-5 | Base-manual function entry |
| `SINGLE` | 2-37 | Alternate typed-conversion form grouped under this name |
| `SINQ` | 4-6 | Base-manual function entry |
| `SPACE$` | 4-32 | Base-manual function entry |
| `SPC` | 4-32 | Base-manual function entry |
| `SQR` | 4-8 | Base-manual function entry |
| `STR$` | 2-15 | Base-manual function entry |
| `STRING$` | 4-32 | Base-manual function entry |
| `SUB` | 4-3 | Base-manual function entry |
| `SUCC` | 4-11 | Base-manual function entry |
| `SWAP` | 4-25 | Base-manual function entry |
| `TAB` | 5-13 | Base-manual function entry |
| `TAN` | 4-5 | Base-manual function entry |
| `TRANSLATE$` | 10-9 | Base-manual function entry |
| `TRIM$` | 4-34 | Base-manual function entry |
| `TRUNC` | 4-13 | Base-manual function entry |
| `TYPE` | 2-30 | Base-manual function entry |
| `UPPER$` | 4-35 | Base-manual function entry |
| `VAL` | 2-16 | Base-manual function entry |
| `VAL?` | 2-16 | Base-manual function entry |
| `VARPTR` | 2-33 | Base-manual function entry |
| `WINDOW` | 9-35 | Base-manual function entry |
| `WORD` | 2-37, 4-26 | Alternate typed-conversion form grouped under this name |
| `XOR` | 4-23 | Base-manual function entry |

## Explicit 3.5 additions

| Function | Printed source page(s) | Source description |
|---|---|---|
| `COMBIN` | 13-33–13-34 | Combination function. |
| `FACT` | 13-33–13-34 | Factorial function; 0! is explicitly defined as 1. |
| `GURU` | 13-35–13-36 | GURU-number function; distinct from the GURU enable/disable statement. |
| `VARIAT` | 13-33–13-34 | Permutation function. |

## Primary source and caveats

- `GFA BASIC 3.5 Interpreter`, supplied 469-page manual: introductory page identifies the included software as GFA BASIC 3.5 and says Chapter 13 is an addendum describing extra features. The base text is the earlier 3.0 manual; the addendum identifies the additional 3.5 features.
- Base function index: printed pp. 12-8–12-18. Detailed base pages are listed per function above.
- 3.5 additions: printed pp. 13-33–13-36 (FACT, VARIAT, COMBIN, GURU).
- The separate 1990 German Amiga book says its main command reference documents version 3.0; it is supplementary and is not used to inflate the 3.5 count.
- Chapter 11's Amiga OS library index is deliberately excluded: it catalogs OS calls, not built-in GFA BASIC language functions.

Machine-readable source: `amiga-gfa-basic-functions.toml`. Validation: `python scripts/validate_amiga_function_inventory.py`.
