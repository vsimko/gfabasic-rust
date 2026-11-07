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
