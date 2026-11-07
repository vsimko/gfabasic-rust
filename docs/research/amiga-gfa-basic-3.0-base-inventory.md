# Amiga GFA BASIC 3.0 base-manual inventory

> **Version boundary:** This is a research inventory, not a 3.5 compatibility claim. The supplied *GFA BASIC 3.5 Interpreter* manual says that it was originally printed for version 3.0 and that version 3.5 additions are described in its addendum (Chapter 13 in this scan). The material below comes from base-manual chapters. Keep it outside the normative 3.5 registry until each item is reconciled against a 3.5-specific source or runtime observation.

Primary source: AM-INTERPRETER-35, *GFA BASIC 3.5 Interpreter*, base-manual chapters 2–12. References below are printed page numbers, not PDF viewer pages.

## Chapter inventory from the contents

| Chapter | Subject | Initial printed-page range / topics | 3.5 status |
|---|---|---|---|
| 2 | Variables and memory handling | pp. 2-2–2-44: variable types, arrays, DEFxxx, LET, CLEAR/CLR, ERASE, DIM, OPTION BASE, INSERT/DELETE, ARRAY FILL, conversions, sorting, dates/time, memory access and allocation | Base-manual 3.0; not independently confirmed for 3.5 |
| 3 | Operators | pp. 3-2–3-16: precedence, logical, arithmetic, string and comparison operators | Base-manual 3.0; not independently confirmed for 3.5 |
| 4 | Numeric functions and strings | starts p. 4-2; arithmetic, trigonometry, conversion, numeric and string functions | Base-manual 3.0; not independently confirmed for 3.5 |
| 5 | Input and output | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 6 | Disk handling and file organization | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 7 | Program handling | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 8 | Graphics | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 9 | Event, menu, screen and window handling | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 10 | Advanced programming | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 11 | System routines | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 12 | Appendices | Chapter listed in contents | Tables/index still to be extracted |

## Chapter 2 — variables and memory

The contents list variable types (p. 2-2), arrays (p. 2-4), DEFxxx (p. 2-5), LET (p. 2-6), CLEAR/CLR (p. 2-7), ERASE (p. 2-8), DIM (p. 2-9), OPTION BASE (p. 2-10), INSERT/DELETE (p. 2-11), ARRAY FILL (p. 2-12), conversion and string functions (pp. 2-13–2-18), SWAP (p. 2-20), QSORT/SSORT (p. 2-22), constants (p. 2-25), dates/time (pp. 2-26–2-28), FRE/TYPE/DUMP/VARPTR/ARRPTR, POKE/PEEK, ABSOLUTE, VOID, MALLOC/MFREE, BMOVE and RESERVE (pp. 2-29–2-44).

Page-level excerpts reviewed so far:

- Arrays (printed pp. 2-3–2-4): arrays may use any variable type and are created with DIM; DIM? returns the number of elements. The manual describes a six-byte array descriptor and in-memory layout. These are implementation-relevant representation details, but remain 3.0 source facts.
- DEFxxx (p. 2-5): DEFBIT, DEFBYT, DEFINT, DEFWRD, DEFFLT and DEFSTR define default variable types based on variable-name prefixes.
- LET (p. 2-6): assigns an arithmetic or string expression to a matching numeric or string variable; the manual says LET is normally optional.
- CLEAR/CLR (p. 2-7): CLEAR erases all variables and fields and has documented execution restrictions; CLR clears listed scalar variables and cannot erase arrays.
- ERASE (p. 2-8): erases complete arrays; erased arrays must be dimensioned again before use.
- DIM/DIM? (p. 2-9): dimensions numeric/string arrays; the manual documents multidimensional arrays and a 65,535-element limit on the last dimension.
- OPTION BASE (p. 2-10): controls whether array indexing starts at zero or one. Its interaction with matrix commands is separately documented in the 3.5 addendum and must not be conflated with the base-manual description.

## Chapter 3 — operators

The operator hierarchy is documented on printed p. 3-2. The manual orders parentheses, string operations, prefixes/power, multiplication/division, integer division/modulo, addition/subtraction, comparisons, logical operations and NOT. The scan's OCR corrupts some operator glyphs; do not use OCR text alone to normalize the complete precedence table.

Page-level excerpts reviewed:

- Logical operators (pp. 3-3–3-9): NOT, AND, OR, XOR, IMP and EQV operate bitwise on 32-bit integer values. In logical contexts, zero is false and every nonzero number is true; TRUE is -1. The manual supplies truth tables and examples.
- AND (p. 3-5): true only when both logical operands are true; for numeric operands it operates bitwise.
- OR (p. 3-6): true when at least one operand is true; numeric use is bitwise.
- XOR (p. 3-7): true when exactly one logical operand is true; bitwise use sets bits present in exactly one operand.
- IMP (p. 3-8): implication is false only for true followed by false; operand order matters.
- EQV (p. 3-9): equivalence is true when both logical values match; bitwise behavior is described as the complement of XOR.
- String comparisons (pp. 3-13–3-16): comparison uses character codes in sequence and stops at the first differing character or when one string ends. The manual documents >, <, >=, <=, and <>, including alternate spellings for greater/less-or-equal operators.

The exact precedence and operator token spellings should be checked against page images before adding individual operator declarations. No 3.5 availability is inferred here.

## Chapter 4 — numeric functions and strings

Initial page-level excerpts reviewed:

- ADD/SUB/MUL/DIV commands (pp. 4-2–4-3): mutate the first numeric variable by adding, subtracting, multiplying or dividing by the second expression. The manual warns that integer forms do not perform overflow checking and that fractional operands are treated as integers for integer variables.
- Integer arithmetic functions (p. 4-3): ADD(x,y), SUB(x,y), MUL(x,y), DIV(x,y), and MOD(x,y) are also described as expression functions and may be grouped in Polish notation. Decimal places are ignored by these integer arithmetic functions.
- SIN/COS/TAN and inverse functions (p. 4-5): use radians.
- SINQ/COSQ (p. 4-6): use degrees and an interpolated lookup table; the manual describes interpolation in 1/16-degree steps.
- DEG/RAD (p. 4-7): convert between radians and degrees.
- SQR (p. 4-8): square-root function; a negative argument produces an error.
- EXP/LOG/LOG10 (p. 4-9): exponential, natural logarithm and base-10 logarithm.
- DEC/INC (p. 4-10): decrement/increment a numeric variable; the manual warns of missing integer overflow checking.
- PRED/SUCC (p. 4-11): predecessor/successor for integer expressions; string forms are documented separately.
- ABS/SGN (p. 4-12): absolute value and sign (-1, 0, 1).
- INT/TRUNC/FIX/FRAC (p. 4-13 onward): the manual distinguishes truncation from floor for negative values and documents the relation between TRUNC and FRAC.

The remainder of Chapter 4, including the string-function families and exact syntax/argument forms, still needs systematic page review. The list above is not a complete built-in catalogue.

## Promotion policy

A candidate can enter api/registry/language-core/ for version 3.5 only after one of the following:

1. A 3.5-specific primary source confirms the declaration and its semantics; or
2. A controlled observation on the 3.5 interpreter establishes behavior, with reproducible test conditions and recorded binary/version provenance.

Otherwise retain it as a version-3.0 source fact or an unresolved compatibility candidate. Do not infer 3.5 compatibility merely because a base-manual topic appears in the same PDF as the 3.5 addendum.

## Remaining work

- Extract the full Chapter 2 and 4 alphabetical/index tables.
- Review the page images for all operator spellings and precedence.
- Extract Chapters 5–12 into topic inventories, then select declarations for detailed review.
- Reconcile each candidate against the 3.5 interpreter and, where possible, the supplied German editions without merging their pagination or version claims.
- Keep the public 3.5 catalogue separate from this 3.0 baseline until evidence supports promotion.

## Complete Chapter 4 topic index (base-manual 3.0)

The contents pages enumerate the following additional topics. This list is a discovery index, not proof of detailed syntax or 3.5 availability.

| Printed page | Topic headings |
|---|---|
| 4-2 | ADD / SUB / MUL / DIV / MOD |
| 4-5 | SIN / COS / TAN |
| 4-6 | SINQ / COSQ |
| 4-7 | DEG / RAD |
| 4-8 | SQR |
| 4-9 | EXP / LOG |
| 4-10 | DEC / INC |
| 4-11 | PRED / SUCC |
| 4-12 | ABS / SGN |
| 4-13 | INT / TRUNC / FIX / FRAC |
| 4-14 | ROUND |
| 4-15 | ODD / EVEN |
| 4-16 | MIN / MAX |
| 4-17 | RND / RANDOM |
| 4-19 | BCLR / BSET / BCHG / BTST |
| 4-21 | SHL / SHR |
| 4-22 | ROL / ROR |
| 4-23 | AND / OR / XOR |
| 4-24 | IMP / EQV |
| 4-25 | SWAP |
| 4-26 | BYTE / CARD / WORD |
| 4-27 | LEFT$ / RIGHT$ |
| 4-28 | MID$ |
| 4-30 | INSTR |
| 4-31 | RINSTR |
| 4-32 | STRING$ / SPACE$ / SPC |
| 4-33 | PRED / SUCC (string forms also discussed) |
| 4-34 | LEN / TRIM$ |
| 4-35 | UPPER$ |
| 4-36 | LSET / RSET |

The Chapter 4 index contains 29 topic rows. Several entries are related to Chapter 2 (for example SWAP, conversions and string operations); preserve separate source sections and do not assume the same syntax/semantics merely from repeated headings.

## Remaining chapter contents — discovery index

The supplied manual's contents also identify the following base-manual 3.0 feature groups. Exact command declarations, semantics and page images still need review.

- **Chapter 5, Input and output:** INKEY$, INPUT, LINE INPUT, FORM INPUT, PRINT, PRINT USING, MODE, DEFNUM, CRSxxx/POS/TAB, DATA, MOUSE, LPRINT/LPOS, HARDCOPY, SOUND/WAVE (printed pp. 5-2–5-22).
- **Chapter 6, Disk handling and file organization:** file specification, random access, CHDIR/DIRS/DFREE, DIR/FILES, EXIST, OPEN, LOF/LOC/EOF/CLOSE/TOUCH, NAME/RENAME/KILL, BLOAD/BSAVE/BGET/BPUT, INP/OUT, INPITOO, INPUT #/LINE INPUT, PRINT #/WRITE #, STORE/RECALL, SEEK/RELSEEK, FIELD, GET/PUT/RECORD (printed pp. 6-2–6-21).
- **Chapter 7, Program handling:** loops; procedures/functions; IF/ELSE/ENDIF; ELSE IF; ONGOSUB; SELECT CASE; FOR/NEXT; REPEAT/UNTIL; WHILE/WEND; DO/LOOP; DO WHILE/UNTIL; EXIT IF; GOSUB/PROCEDURE; LOCAL; @func/FUNCTION; DEFFN; ON BREAK; ON ERROR; ERROR/ERR; EVERY/AFTER; REM; GOTO; PAUSE/DELAY; END/EDIT/STOP; NEW; LOAD; SAVE/PSAVE; LIST/LLIST; CHAIN; RUN; SYSTEM/QUIT; TRON/TROFF/TRACE; $, INLINE, C:, MONITOR, CALL and RCALL (printed pp. 7-2–7-55).
- **Chapter 8, Graphics:** SETCOLOR, DEFFILL, BOUNDARY, DEFLINE, GRAPHMODE, RASTPORT, CLIP, PLOT/LINE/DRAW, DRAW/SETDRAW, BOX/PBOX, CIRCLE/ELLIPSE, POLYLINE, POINT, FILL, CLS, TEXT, GET/PUT, VSYNC, DISPLAY (printed pp. 8-2–8-24).
- **Chapter 9, Event/menu/screen/window handling:** event handling, pull-down menus, MENU/MENUO, ON MENU and ON MENU...GOSUB, MENU KILL/MENU X,Y, OPENS/CLOSES, FRONTS/BACKS, MOVES, TITLES, SETSPEN, SCREEN, OPENW, CLEARW/TITLEW/BACKW/FULLW, MOVEW/SIZEW/LIMITW, SETWPEN, WINDOW, ALERT and FILESELECT (printed pp. 9-2–9-37).
- **Chapter 10, Advanced programming:** CLI call/_dosCmd$, EXEC, library variables, SPRITE, STICK/STRIG, SAY/TRANSLATE$, FD-files, blitter objects and OBJECT commands (printed pp. 10-2–10-14).
- **Chapter 11, System routines:** system routine calling plus Exec, Graphics, Workbench, Intuition, DiskFont, Layers and DOS libraries (printed pp. 11-2–11-85). Treat these as platform/OS API surface, not automatically as language-core declarations.
- **Chapter 12, Appendices:** keyboard scan codes, ASCII table, special ASCII codes, fill-in patterns, Amiga-DOS error messages, table of commands and table of operating-system functions (printed pp. 12-2–12-19).

This is the source table-of-contents extraction only. It does not assert that every listed 3.0 item exists unchanged in 3.5. In particular, Chapter 11 must be split into language-level wrappers versus Amiga OS library calls when the detailed inventory is created.
