---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — inherited statement and operator evidence"
---

# Amiga GFA BASIC 3.5 — inherited statement and operator evidence

## Scope and source boundary

This pass checks selected statement and operator candidates against the version-labelled material in Chapters 18–19 of *Das große GFA-BASIC-Buch zum Amiga* (AM-BOOK-DE-1990). Chapter 18 opens on printed p. 451 by identifying its matrix/vector commands as new in Version 3.5 and stating that old commands have basically not changed. The examples in these chapters therefore provide direct 3.5 use evidence for the exact inherited forms shown. They do not prove every overload, grammar alternative, edge case, or full compatibility of the corresponding 3.0 chapter.

Page references below are printed book pages. The vector example on p. 471 and the DATA-pointer example on p. 480 were checked against page images, in addition to the extracted text.

## Directly evidenced inherited operators

| Candidate | 3.5 evidence | Demonstrated form | Boundary |
|---|---|---|---|
| Numeric `+` and `-` | pp. 467, 470 | `MAT SUB a()=b()-c()`; the matrix-multiplication explanation gives numeric sums and signed operands | Confirms numeric arithmetic use only; not precedence, comparison, or string concatenation |
| `*` and `/` | pp. 470, 473 | Numeric products in the matrix-multiplication explanation; `1/SQR(l)` in vector normalization | Confirms the shown numeric forms only; not precedence or all operand types |
| Unary `-` | p. 470 | A negative operand appears in `0.5*-1` | Unary `+` is not demonstrated and remains 3.0-only evidence |

The arithmetic evidence does not establish the operator hierarchy, associativity, string concatenation, exponentiation, integer division, modulo, comparison operators, or logical operators. The assignment sign in the examples is not treated as evidence for equality comparison.

## Directly evidenced inherited statements

| Candidate | 3.5 evidence | Demonstrated form | Boundary |
|---|---|---|---|
| `PRINT` | pp. 458, 471, 480 | Ordinary screen output in matrix and DATA-pointer examples | Does not confirm `PRINT AT` or `WRITE` |
| `LOCATE` | p. 471 | `LOCATE 1,12` in the vector-animation example | Confirms this cursor-positioning form only |
| `FOR / NEXT` | pp. 463–464, 471, 480 | Ascending `FOR ... TO ... NEXT` and descending `FOR j=i TO 1 STEP -1 ... NEXT` | Does not confirm the `DOWNTO` spelling or editor `ENDFOR` replacement |
| `WHILE / WEND` | p. 480 | `WHILE _DATA ... WEND`; the example exits when the DATA pointer reaches zero | Confirms the demonstrated condition form, not every expression or loop edge case |
| `COLOR` | p. 471 | `COLOR 3` and `COLOR 1` | Does not confirm the distinct `SETCOLOR` palette-component form |
| `DRAW TO` | p. 471 | A procedure draws calculated endpoints using `DRAW TO` | Does not confirm `PLOT`, `LINE`, or other `DRAW` forms |

The candidate manifest splits grouped records where the example demonstrates only one member or form: `PRINT` and `LOCATE` are separated from `PRINT AT` and `WRITE`; `COLOR` from `SETCOLOR`; `DRAW TO` from `PLOT`, `LINE`, and other `DRAW` forms; unary minus from unary plus.

## Similar-looking forms deliberately not conflated

- `MAT PRINT #1` and `MAT INPUT #1` are matrix-specific commands; they do not confirm ordinary `PRINT #`, `INPUT #`, or `LINE INPUT #` file statements.
- `MAT READ` is not used as evidence for ordinary `READ`.
- `MAT ADD`, `MAT SUB`, and `MAT MUL` are matrix statements; their names do not confirm same-named Chapter 4 functions.
- `COLOR` is not treated as interchangeable with `SETCOLOR`.
- The `DRAW TO` example does not confirm the full `DRAW` command family.

## Result

Nine inherited statement/operator candidate records receive narrow direct 3.5 evidence in this pass: numeric `+`/`-`, `*`/`/`, unary `-`, `PRINT`, `LOCATE`, `FOR/NEXT`, `WHILE/WEND`, `COLOR`, and `DRAW TO`. Their original source origin remains the 3.0 base manual; confirmation source and printed pages are recorded separately.

No API registry declaration, implementation state, runtime-conformance claim, or language-core count is changed. Platform-specific graphics forms remain candidates for the Amiga API profile, not language-core declarations. Unsupported siblings and operators remain documented in the 3.0 base only; their absence from these examples is not evidence of removal or incompatibility.
