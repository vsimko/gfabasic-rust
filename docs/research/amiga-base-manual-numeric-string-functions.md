---
type: "Research Note"
title: "Amiga GFA BASIC base-manual numeric and string function extraction"
---

# Numeric and string functions from the Amiga interpreter manual

> Source-derived extraction from the 3.0 base text included in the supplied *GFA BASIC 3.5 Interpreter* manual. These entries are **not** proof of availability in every 3.5 interpreter build. They remain 3.0-source candidates until independently reconciled with 3.5-specific evidence.

## Scope and notation

This batch transcribes the numeric, bitwise, and string-function sections on printed pp. 4-3–4-35. The manual's argument type labels are retained: `aexp` = arithmetic expression, `iexp` = integer expression, and `sexp` = string expression. Optional arguments are shown in square brackets. Syntax alternatives are separate forms where the source distinguishes them.

The source uses “command” informally in several descriptions even when the item is callable as an expression. This note classifies only the function forms shown in the manual; command-only forms such as `RANDOMIZE`, `DEC`, `INC`, and the assignment form of `MID$` are called out separately and are not silently treated as functions.

## Arithmetic and trigonometric functions

| Name | Source form(s) | Result and source-derived behavior | Printed page |
|---|---|---|---|
| `ADD` | `ADD(x,y)` | Arithmetic addition operation. The source groups ADD with DIV, MOD, MUL and SUB; its detailed numeric operator semantics are covered by the arithmetic-operator section. | 4-3 |
| `DIV` | `DIV(x,y)` | Integer division operation in the arithmetic operator family. | 4-3 |
| `MOD` | `MOD(x,y)` | Remainder/modulo operation in the arithmetic operator family. | 4-3 |
| `MUL` | `MUL(x,y)` | Arithmetic multiplication operation. | 4-3 |
| `SUB` | `SUB(x,y)` | Arithmetic subtraction operation. | 4-3 |
| `SIN` | `SIN(x)` | Sine of an angle expressed in radians. Argument: `aexp`. | 4-5 |
| `COS` | `COS(x)` | Cosine of an angle expressed in radians. Argument: `aexp`. | 4-5 |
| `TAN` | `TAN(x)` | Tangent of an angle expressed in radians. Argument: `aexp`. | 4-5 |
| `ASIN` | `ASIN(x)` | Inverse sine. Argument: `aexp`. | 4-5 |
| `ACOS` | `ACOS(x)` | Inverse cosine. Argument: `aexp`. | 4-5 |
| `ATN` | `ATN(x)` | Inverse tangent. Argument: `aexp`. | 4-5 |
| `SINQ` | `SINQ(degree)` | Fast interpolated sine using a degree-indexed internal table; the manual says interpolation is in 1/16-degree steps. Equivalent to `SIN(RAD(degree))` within the stated approximation. | 4-6 |
| `COSQ` | `COSQ(degree)` | Fast interpolated cosine using the same degree-indexed table; equivalent to `COS(RAD(degree))` within the stated approximation. | 4-6 |
| `DEG` | `DEG(x)` | Converts radians to degrees; the source gives `x * 180 / PI`. Argument: `aexp`. | 4-7 |
| `RAD` | `RAD(degree)` | Converts degrees to radians; inverse of DEG. Argument: `aexp`. | 4-7 |
| `SQR` | `SQR(x)` | Returns the square root of x. The manual explicitly says a negative argument produces an error. Argument: `aexp`. | 4-8 |
| `EXP` | `EXP(x)` | Returns e raised to the power x. Argument: `aexp`. | 4-9 |
| `LOG` | `LOG(x)` | Returns the natural logarithm (base e). Argument: `aexp`. | 4-9 |
| `LOG10` | `LOG10(x)` | Returns the base-10 logarithm. Argument: `aexp`. | 4-9 |

## Numeric conversion, rounding, and ordering

| Name | Source form(s) | Result and source-derived behavior | Printed page |
|---|---|---|---|
| `PRED` | `PRED(i)` | Returns the predecessor (next smaller value) using integer arithmetic; decimal values are ignored. The manual also documents a string overload on p. 4-33. Argument: `iexp`. | 4-11, 4-33 |
| `SUCC` | `SUCC(i)` | Returns the successor (next larger value) using integer arithmetic; decimal values are ignored. The manual also documents a string overload on p. 4-33. Argument: `iexp`. | 4-11, 4-33 |
| `ABS` | `ABS(x)` | Absolute value: negative x becomes -x; zero remains zero; positive x is unchanged. Argument: `aexp`. | 4-12 |
| `SGN` | `SGN(x)` | Sign function: -1 for negative x, 0 for zero, and 1 for positive x. Argument: `aexp`. | 4-12 |
| `INT` | `INT(x)` | Returns the greatest whole number less than or equal to x. Unlike TRUNC/FIX, negative fractional values round toward the next smaller integer. Argument: `aexp`. | 4-13 |
| `TRUNC` | `TRUNC(x)` | Removes the fractional part (truncates toward zero). The source says TRUNC is identical to FIX. Argument: `aexp`. | 4-13 |
| `FIX` | `FIX(x)` | Same behavior as TRUNC: removes the fractional part. Argument: `aexp`. | 4-13 |
| `FRAC` | `FRAC(x)` | Returns the fractional part, retaining the sign of x. The source states `x = TRUNC(x) + FRAC(x)`. Argument: `aexp`. | 4-13 |
| `ROUND` | `ROUND(x[,n])` | Rounds x; optional n selects decimal places. n=0 rounds to an integer; negative n rounds to positions before the decimal point (example: ROUND(155,-1) gives 160). Arguments: x=`aexp`, n=`iexp`. | 4-14 |
| `ODD` | `ODD(x)` | Returns -1 (TRUE) for odd values and 0 (FALSE) for even values; zero is even. Argument: `aexp`. | 4-15 |
| `EVEN` | `EVEN(x)` | Returns -1 (TRUE) for even values and 0 (FALSE) for odd values; zero is even. Argument: `aexp`. | 4-15 |
| `MIN` | `MIN(x[,y,z,...])`; `MIN(x$[,y$,z$,...])` | Returns the smallest item in the supplied list; both numeric and string forms are documented. Numeric arguments: `aexp`; string arguments: `sexp`. | 4-16 |
| `MAX` | `MAX(x[,y,z,...])`; `MAX(x$[,y$,z$,...])` | Returns the largest item in the supplied list; both numeric and string forms are documented. Numeric arguments: `aexp`; string arguments: `sexp`. | 4-16 |
| `RND` | `RND`; `RND(x)` | Returns a random real number in [0,1). The optional x parameter is explicitly said to have no meaning. | 4-17–4-18 |
| `RANDOM` | `RANDOM(x)` | Returns a random integer from 0 inclusive to x exclusive. The source warns that x should not be a whole number if all returned numbers are to have equal probability. Argument: `aexp`. | 4-17–4-18 |
| `RAND` | `RAND(y)` | Returns a random 16-bit integer from 0 to y-1. Argument: `iexp`. | 4-17–4-18 |

The same section documents `RANDOMIZE y` as a **statement**, not a function: it seeds the random generator; repeating the same seed reproduces the same sequence. The manual also permits RANDOMIZE without a parameter or with zero to initialize the generator.

## Bit manipulation and logical functions

| Name | Source form(s) | Result and source-derived behavior | Printed page |
|---|---|---|---|
| `BCLR` | `BCLR(x,y)` | Clears bit y of x. Bit numbering starts at zero; y is in the range 0–31 and is masked by the processor with AND 31. Arguments: `iexp`. | 4-19 |
| `BSET` | `BSET(x,y)` | Sets bit y of x to 1. Same bit numbering and masking rules as BCLR. Arguments: `iexp`. | 4-19 |
| `BCHG` | `BCHG(x,y)` | Toggles bit y of x. Same bit numbering and masking rules as BCLR. Arguments: `iexp`. | 4-19 |
| `BTST` | `BTST(x,y)` | Returns 1 (TRUE) if bit y is set and 0 (FALSE) otherwise. Same bit numbering and masking rules as BCLR. Arguments: `iexp`. | 4-19 |
| `SHL` | `SHL(x,y)`; `SHL&(x,y)`; `SHL|(x,y)` | Shifts x left by y bits. Default width is a long word (4 bytes); `&` selects word width (2 bytes), and `|` selects byte width. | 4-21 |
| `SHR` | `SHR(x,y)`; `SHR&(x,y)`; `SHR|(x,y)` | Shifts x right by y bits, with the same default/word/byte width variants as SHL. | 4-21 |
| `ROL` | `ROL(x,y)`; `ROL&(x,y)`; `ROL|(x,y)` | Rotates bits left within the selected long-word, word, or byte width; bits shifted out at one end re-enter at the other. | 4-22 |
| `ROR` | `ROR(x,y)`; `ROR&(x,y)`; `ROR|(x,y)` | Rotates bits right within the selected long-word, word, or byte width. | 4-22 |
| `AND` | `AND(x,y)` | Bitwise AND: a result bit is set only when the corresponding bits in both operands are set. Arguments: `iexp`. | 4-23 |
| `OR` | `OR(x,y)` | Bitwise OR: a result bit is set when the corresponding bit is set in either operand. Arguments: `iexp`. | 4-23 |
| `XOR` | `XOR(x,y)` | Bitwise exclusive OR: a result bit is set when the operand bits differ. Arguments: `iexp`. | 4-23 |
| `IMP` | `IMP(x,y)` | Bitwise implication: clears a bit when it is set in x and not set in y. Arguments: `iexp`. | 4-24 |
| `EQV` | `EQV(x,y)` | Bitwise equivalence: sets a bit when the corresponding operand bits have the same value. Arguments: `iexp`. | 4-24 |
| `SWAP` | `SWAP(x)` | Treats x as a 4-byte long word and exchanges its upper and lower 16-bit words. The manual distinguishes this function from the SWAP command. Argument: `iexp`. | 4-25 |
| `BYTE` | `BYTE(x)` | Returns the lower 8 bits of x. Argument: `iexp`. | 4-26 |
| `CARD` | `CARD(x)` | Returns the lower 16 bits of x. Argument: `iexp`. | 4-26 |
| `WORD` | `WORD(x)` | Extends a 16-bit word to a long word by copying bit 15 into bits 16–31. Argument: `iexp`. | 4-26 |

## String functions

| Name | Source form(s) | Result and source-derived behavior | Printed page |
|---|---|---|---|
| `LEFT$` | `LEFT$(a$[,x])` | Returns x characters from the left of a$. If x is omitted, returns the first character; if x exceeds the string length, returns the whole string. Arguments: a$=`sexp`, x=`iexp`. | 4-27 |
| `RIGHT$` | `RIGHT$(a$[,x])` | Returns x characters from the right of a$. If x is omitted, returns the last character; if x exceeds the string length, returns the whole string. Arguments: a$=`sexp`, x=`iexp`. | 4-27 |
| `MID$` | `MID$(a$,x[,y])` (function) | Returns y characters starting at position x. If y is omitted, returns the remainder of the string; if x exceeds the string length, returns an empty string. | 4-28 |
| `INSTR` | `INSTR(a$,b$)`; `INSTR(a$,b$[,x])`; `INSTR([x],a$,b$)` | Searches for b$ inside a$. Optional x sets the starting position; without it search starts at the first character. Returns the position after which b$ is contained, or zero if not found. Two empty/space strings are documented to return one. | 4-30 |
| `RINSTR` | `RINSTR(a$,b$)`; `RINSTR(a$,b$[,x])`; `RINSTR([x],a$,b$)` | Searches like INSTR but from the rightmost character, or from the supplied position. | 4-31 |
| `STRING$` | `STRING$(x,a$)`; `STRING$(x,code)` | Creates a string containing the supplied string or character code x times; x is documented in the range 0–32767. Arguments: x/code=`iexp`; a$=`sexp`. | 4-32 |
| `SPACE$` | `SPACE$(x)` | Returns a string containing x spaces. Argument: `iexp`. | 4-32 |
| `SPC` | `SPC(x)` | Emits x spaces in a PRINT command; unlike SPACE$, it does not create a string value. Argument: `iexp`. | 4-32 |
| `PRED` | `PRED(a$)` | Returns the character one ASCII value lower than the first character of a$. | 4-33 |
| `SUCC` | `SUCC(a$)` | Returns the character one ASCII value higher than the first character of a$. | 4-33 |
| `LEN` | `LEN(a$)` | Returns the number of characters in a$. Argument: `sexp`. | 4-34 |
| `TRIM$` | `TRIM$(a$)` | Removes spaces from the left and right ends of a string. Argument: `sexp`. | 4-34 |
| `UPPER$` | `UPPER$(a$)` | Converts lowercase characters in a$ to uppercase. Argument: `sexp`. | 4-35 |

The manual also documents `MID$(a$,x[,y])=b$` as a **statement/assignment form** on printed p. 4-29. It overwrites characters in the existing string variable without changing its length; it is not the function form catalogued above.

## Registry and compatibility follow-up

- These are source-extracted forms and semantics, not implementation claims.
- All entries in this batch originate in the manual's 3.0 base text. Do not tag them as confirmed 3.5 solely because the file is titled *GFA BASIC 3.5 Interpreter*.
- The source's integer/real representation, overflow, rounding edge cases, exact error behavior, and interpreter-version availability still need independent evidence or executable reference tests before conformance can be claimed.
- `PRED` and `SUCC` have numeric and string overloads; `MIN` and `MAX` have numeric and string forms; `MID$` has distinct function and assignment forms; the typed conversions and width-suffixed bit operations have explicit syntax variants. These distinctions must survive registry normalization.
- The inventory file `docs/research/amiga-gfa-basic-function-inventory.md` remains the index for the complete extracted name set and its printed page locators. This batch advances the Chapter 4 numeric/string portion; it does not close the remaining Chapters 2, 5, 6, 8, 9, and 10 work.
