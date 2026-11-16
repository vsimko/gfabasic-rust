---
type: "Research Note"
title: "Amiga GFA BASIC 3.0 expression, intrinsic and control-flow evidence"
---

# Amiga GFA BASIC 3.0 expression, intrinsic and control-flow evidence

> **Version boundary:** This document records page-reviewed facts from the 3.0 base body of the supplied *GFA BASIC 3.5 Interpreter* scan. It is not a claim that these declarations or semantics are present unchanged in interpreter 3.5. The manual identifies its base body as 3.0 and its 3.5-specific material as a separate addendum. Keep these findings as inherited candidates until independently confirmed for 3.5.

Primary source: `AM-INTERPRETER-35`, *GFA BASIC 3.5 Interpreter*, base-manual Chapters 2, 4 and 7. Page references below are printed page numbers, not PDF viewer page numbers.

## Block 1 — expressions and conversions

### Expression forms and evaluation

Chapter 3's precedence table (printed p. 3-2) orders operators from highest to lowest as:

1. Parentheses.
2. String addition.
3. String comparisons.
4. Unary prefixes `+` and `-`.
5. Power `^`.
6. Multiplication and division `*` and `/`.
7. Integer division and remainder `DIV` and `MOD`.
8. Addition and subtraction.
9. Numeric comparisons.
10. `AND`, `OR`, `XOR`, `IMP`, `EQV`.
11. `NOT`.

The source examples show multiplication before addition and power before multiplication. Parentheses override the listed order. The source separately lists string-comparison and numeric-comparison rows; do not collapse them into a single grammar production. Its printed comparison row contains OCR-sensitive spellings, while the detailed comparison pages (pp. 3-13–3-16) explicitly document `=`, `<>`, `<`, `>`, `<=`, `=<`, `>=` and `=>`. The numeric `==` operator is separately described as approximate equality (p. 3-12).

The manual does not define a complete formal expression grammar or operator associativity in these pages. It also does not specify a general eager/lazy evaluation rule. Those points remain unresolved rather than inferred from examples.

### Numeric, string and binary conversions

| Form | Source-described behavior | Printed page |
|---|---|---|
| `ASC(a$)` | Returns the character code of the first character; empty string returns zero. | 2-13 |
| `CHR$(x)` | Returns the character for the low byte of the integer expression (`x AND 255`). | 2-13 |
| `BIN$(x[,y])` | Converts an integer expression to base 2; optional width is 1–32. | 2-14 |
| `OCT$(x[,y])` | Converts to base 8; optional width is 0–11. | 2-14 |
| `HEX$(x[,y])` | Converts to base 16; optional width specifies hexadecimal digits. | 2-14 |
| `STR$(x)` | Converts a numeric expression to its decimal text representation. | 2-15 |
| `STR$(x,y)` | Uses a requested output width/rightmost character count, with leading spaces as needed. | 2-15 |
| `STR$(x,y,z)` | Returns a selected formatted portion of the numeric representation. | 2-15 |
| `VAL(a$)` | Parses a numeric prefix from a string; returns zero if no number starts the string. Supports `&H`, `&X` and `&O` radix prefixes. | 2-16 |
| `VAL?(a$)` | Returns the number of characters consumed by the numeric parse; zero if none. | 2-16 |
| `CINT(x)` | Rounds a floating-point value to an integer; described as compiler-only. | 2-17 |
| `CFLOAT(y)` | Converts an integer value to floating point; described as compiler-only. | 2-17 |
| `CVI(a$)`, `CVL(a$)` | Interpret two-byte and four-byte strings as 16-bit and 32-bit integers. | 2-18 |
| `MKI$(x)`, `MKL$(x)`, `MKS$(x)`, `MKD$(x)` | Produce internal binary numeric representations in strings, rather than ASCII decimal text. | 2-18–2-19 |

**Source ambiguity:** the conversion section's prose refers to `CVF` and `CVS` inconsistently, and its reverse-function list names `MKS$` and `MKD$`. Do not silently normalize these names into a one-to-one CVx/MKx family. Preserve the printed declarations as candidates and resolve the disputed slots against a clear page image and 3.5 evidence. The text also distinguishes binary numeric representation from the textual conversion performed by `VAL`/`STR$`.

### Expression and conversion implications

- The source distinguishes numeric expressions (`aexp`), integer expressions (`iexp`) and string expressions (`sexp`) in its declaration notation. These are source labels, not yet a complete type grammar.
- `VAL` is permissive prefix parsing, not a strict whole-string parser; trailing nonnumeric characters do not invalidate an already parsed prefix.
- `CINT` and `CFLOAT` are explicitly said to be meaningful only to the compiler, so they must not be assumed to be interpreter intrinsics.
- The binary conversion functions are intended for interchange with other programming languages; their byte layout, endianness and disputed float representation require independent confirmation before defining a portable ABI.

## Block 2 — built-in functions and intrinsic operations

This is a page-reviewed selection of Chapter 4, not a complete or 3.5-verified built-in catalogue. Function names and behavior below are the 3.0 base-manual evidence.

### Numeric and mathematical functions

| Function family | Source-described behavior | Printed page |
|---|---|---|
| `ADD(x,y)`, `SUB(x,y)`, `MUL(x,y)`, `DIV(x,y)`, `MOD(x,y)` | Integer arithmetic functions; decimal places are ignored. The similarly named commands mutate a variable. | 4-2–4-3 |
| `SIN`, `COS`, `TAN` and inverse functions | Trigonometric functions using radians. | 4-5 |
| `SINQ`, `COSQ` | Interpolated sine/cosine lookup functions using degrees. | 4-6 |
| `DEG(x)`, `RAD(x)` | Convert radians to degrees and degrees to radians. | 4-7 |
| `SQR(x)` | Square root; negative input produces an error. | 4-8 |
| `EXP(x)`, `LOG(x)`, `LOG10(x)` | Exponential, natural logarithm and base-10 logarithm. | 4-9 |
| `PRED(i)`, `SUCC(i)` | Integer predecessor/successor; decimal parts are ignored. | 4-11 |
| `ABS(x)`, `SGN(x)` | Absolute value and sign (-1, 0 or 1). | 4-12 |
| `INT(x)`, `TRUNC(x)`, `FIX(x)`, `FRAC(x)` | Integer/fraction decomposition; TRUNC and FIX truncate toward zero, while INT returns the greatest integer not exceeding x. | 4-13 |
| `ROUND(x[,n])` | Rounds to an integer or to n decimal places; negative n rounds before the decimal point. | 4-14 |
| `ODD(x)`, `EVEN(x)` | Return -1 for true and 0 for false; zero is even. | 4-15 |
| `MIN(...)`, `MAX(...)` | Return the minimum/maximum of numeric or string argument lists. | 4-16 |
| `RND`, `RANDOM(x)`, `RAND(y)` | Produce random floating or integer values with different documented ranges. | 4-17–4-18 |

The source also documents bit manipulation and integer operations:

- `BCLR(x,y)`, `BSET(x,y)`, `BCHG(x,y)`, `BTST(x,y)`: clear, set, toggle or test bit y; bit numbering starts at zero and is masked to 0–31 (pp. 4-19–4-20).
- `SHL`, `SHR`, `ROL`, `ROR`, including `&` and `|` width variants: shift or rotate 32-bit, 16-bit or 8-bit values (pp. 4-21–4-22).
- `AND(x,y)`, `OR(x,y)`, `XOR(x,y)`, `IMP(x,y)`, `EQV(x,y)`: bitwise integer operations (pp. 4-23–4-24).
- `SWAP(x)`: exchanges the upper and lower 16-bit words of a 32-bit value. This is a function and is distinct from the variable/array `SWAP` command in Chapter 2 (p. 4-25).
- `BYTE(x)`, `CARD(x)`, `WORD(x)`: extract/extend low 8- or 16-bit portions (p. 4-26).

### String functions

| Function or command | Source-described behavior | Printed page |
|---|---|---|
| `LEFT$(a$[,x])`, `RIGHT$(a$[,x])` | Return x characters from the left or right; default is one character. | 4-27 |
| `MID$(a$,x[,y])` | Returns a substring beginning at character x; also has a command form that overwrites part of a string variable without changing its length. | 4-28–4-29 |
| `INSTR(a$,b$[,x])` | Searches from the left or an optional start position; zero means not found. | 4-30 |
| `RINSTR(a$,b$[,x])` | Searches from the right or an optional position. | 4-31 |
| `STRING$(x,a$)`, `STRING$(x,code)` | Repeats a string or character code x times. | 4-32 |
| `SPACE$(x)`, `SPC(x)` | Return a string of spaces or insert spaces in PRINT output, respectively. | 4-32 |
| `PRED(a$)`, `SUCC(a$)` | Return the preceding/following character based on the first character's code. | 4-33 |
| `LEN(a$)`, `TRIM$(a$)` | Return string length or remove leading/trailing spaces. | 4-34 |
| `UPPER$(a$)` | Converts lowercase characters to uppercase. | 4-35 |
| `LSET a$=b$`, `RSET a$=b$` | Left/right justify text in a destination string whose length is unchanged. | 4-36 |

### Related commands that are not functions

The same chapter describes `DEC` and `INC` as mutating numeric commands (p. 4-10), and `SWAP a,b` / array forms as value or descriptor exchange in Chapter 2 (pp. 2-20–2-21). Do not confuse these with the expression function `SWAP(x)` (p. 4-25). Likewise, the arithmetic command forms `ADD x,y`, etc., are distinct from their expression-function forms.

The chapter contents list these families as its 3.0 scope. This page review covers the headings above but does not establish that every overload, alias, error case or edge condition has been fully extracted. The existing project function count/inventory must remain a separate reconciliation task; do not derive a complete count from this selected table.

## Block 3 — conditional flow, loops, procedures and functions

Chapter 7 is the 3.0 base-manual program-handling chapter. The following semantics were checked against the corresponding page text. They remain 3.0 candidates, not 3.5 guarantees.

### Conditional branching

- `IF condition [THEN]` begins a conditional block; `ELSE` selects the alternative branch and `ENDIF` closes it (printed pp. 7-4–7-5). The manual says THEN is optional. A nonzero numeric condition is true.
- `ELSE IF condition` / `ELSEIF condition` permits a chain of conditions within the same IF construction (pp. 7-6–7-7). The editor's spelling/normalization behavior is described in the source; preserve both observed spellings as candidates.
- `SELECT expression` with `CASE`, optional `TO` ranges or comma-separated values, optional `DEFAULT`, and `ENDSELECT` selects a branch (pp. 7-9–7-12). The source also accepts `OTHERWISE` as an editor-replaced alias for DEFAULT. Numeric selectors and strings up to four characters are documented. `CONT` has a special effect when placed before a CASE/DEFAULT and continues selection processing.
- `ON x GOSUB proc1,proc2,...` selects the xth parameterless procedure; decimal places are ignored, and out-of-range values call no procedure (p. 7-8).

### Loop forms

| Form | Source-described control behavior | Printed page |
|---|---|---|
| `FOR i=start TO end [STEP step] ... NEXT i` | Numbered loop; default step is 1. The source says the body executes at least once and the counter remains at the first value that exceeds the end criterion. | 7-13–7-14 |
| `FOR i=start DOWNTO end ... NEXT i` | Descending form; STEP is not used with DOWNTO. | 7-13–7-14 |
| `REPEAT ... UNTIL condition` | Post-tested loop: body executes before the condition is checked, and repeats while condition is false. | 7-15 |
| `WHILE condition ... WEND` | Pre-tested loop: body executes while condition is true. | 7-16 |
| `DO ... LOOP` | Unconditional loop; exit requires EXIT IF, GOTO or a program-ending command. | 7-17 |
| `DO [WHILE condition | UNTIL condition] ... LOOP [WHILE condition | UNTIL condition]` | The source describes nine combinations of the three DO forms and three LOOP forms. DO conditions are tested at entry; LOOP conditions control whether execution branches back. | 7-18–7-19 |
| `EXIT IF condition` | Leaves the current loop when the condition is true; the source also documents use inside IF/ENDIF and SELECT/ENDSELECT structures. | 7-20 |

The manual lists `ENDFOR`, `ENDREPEAT`, `END WHILE` and `ENDDO` as editor/interpreter replacement spellings for the corresponding closing commands. Treat these as source-documented aliases, not necessarily distinct runtime tokens.

### Procedures and user-defined functions

- `GOSUB proc[(args)]` calls a named procedure defined by `PROCEDURE proc[(parameters)] ... RETURN` (printed pp. 7-21–7-22). `SUB` is accepted in place of PROCEDURE; `ENDPROC` or `ENDSUB` can replace RETURN. Parameters may be values or variables passed by reference using VAR. The source notes that names colliding with command words require a prefix form such as `@` or GOSUB.
- `LOCAL var1[,var2,...]` limits the visibility of listed variables to the procedure and procedures it calls; a same-named global is hidden during that scope (p. 7-23).
- `FUNCTION name[(parameters)] ... RETURN expression ... ENDFUNC` defines a user function called with `@name(...)` (pp. 7-24–7-25). Parameters may be values or VAR references. A function may return numeric or string values; a trailing `$` marks a string-valued function. The source says RETURN may appear multiple times but a function must return a value rather than simply reaching ENDFUNC.
- `DEFFN name[(parameters)]=expression` defines a one-line numeric or string function, called with `FN name(...)` or `@name(...)` (pp. 7-26–7-27). The source says these functions cannot be recursive.

### Control-flow facts still needing resolution

This block is limited to the page-reviewed constructs above. Chapter 7's contents also list GOTO/labels, ON BREAK, ON ERROR, ERROR/ERR, EVERY/AFTER, REM, PAUSE/DELAY, termination and program-loading commands. Their exact declarations and edge cases were not extracted in this block. Do not infer them from this partial list.

## Version and evidence handling

- All declarations in this document are scoped to the **Amiga GFA BASIC 3.0 base manual**.
- The fact that the same scan also contains a 3.5 addendum does not prove that a base-manual feature was retained unchanged in 3.5.
- Do not promote these names into the normative 3.5 API registry without 3.5-specific primary evidence or reproducible observation on the original 3.5 interpreter.
- The 3.5 interpreter binary and lawful execution environment are not currently evidenced by this document; interpreter behavior is therefore not claimed as observed.
- The source's notation (`aexp`, `iexp`, `sexp`, `bexp`, `avar`, `svar`) should be preserved in source notes until its full grammar and exact type constraints are independently resolved.
