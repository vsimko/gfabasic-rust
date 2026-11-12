---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — inherited function use in addendum examples"
---

# Amiga GFA BASIC 3.5 — inherited function use in addendum examples

## Scope and method

The supplied *GFA BASIC 3.5 Interpreter* labels Chapter 13 as the version-specific 3.5 addendum. I compared the interpreter-function identifiers in `amiga-gfa-basic-functions.toml` with function-call forms used in the addendum prose and examples (printed pp. 13-1–13-36).

A function call appearing in a 3.5-specific example is direct documentary evidence that the shown call form is usable in the 3.5 interpreter. It does not establish that every overload, edge case, numeric boundary, or error behavior from the 3.0 base chapter was revalidated. The function retains its original `base-manual-3.0` source origin; 3.5 use is recorded as a separate evidence dimension.

## Directly evidenced inherited functions

| Function | 3.5 addendum evidence | What the addendum demonstrates | What remains unconfirmed |
|---|---|---|---|
| `RAND` | p. 13-2 | `RAND(10)` populates elements of a floating-point matrix in the `MAT SET` example. | Distribution details, seed behavior, and the other random functions are not restated. |
| `STR$` | p. 13-3 | The `MAT PRINT` description says optional formatting parameters use `STR$(x,g,n)`. | The one- and two-argument base-manual forms are not independently restated. |
| `SUCC` | p. 13-5 | A matrix-copy example assigns values using `SUCC(i%)`. | The string overload is not independently demonstrated. |
| `STRING$` | p. 13-21 | Matrix examples use `STRING$(n, "-")` to print separator lines. | The numeric character-code overload is not independently demonstrated. |
| `HEX$` | p. 13-36 | The GURU error-handler example prints `HEX$(GURU)`. | The optional width argument is not independently demonstrated. |

These five identifiers remain recorded with their 3.0 base-manual origin and receive the separate `confirmed-by-3.5-example` evidence status. Their function inventory entries retain the original 3.0 printed-page references and add the 3.5 addendum page locators.

## Similar-looking occurrence deliberately excluded

The addendum matrix examples contain `~INP(2)` (pp. 13-21 onward). The base-manual function candidate is documented as `INP(#n)`, which reads a byte from an opened data channel. The addendum spelling and context do not establish that it is the same call form. Therefore `INP` is not promoted by this audit and remains documented-in-3.0-only pending clarification.

Likewise, the addendum’s English words “function”, “screen”, “type”, “point”, “and”, or “or”, and its `MAT ADD`/`MAT SUB`/`MAT MUL` operations, are not treated as occurrences of same-named built-in functions.

## Result and boundary

The five direct-use confirmations are limited to `RAND`, `STR$`, `SUCC`, `STRING$`, and `HEX$`. Together with the four functions explicitly introduced by the addendum (`FACT`, `VARIAT`, `COMBIN`, and `GURU`), these are nine function identifiers with direct 3.5 documentary evidence in this pass. The four explicit additions remain a distinct category; the five inherited functions are not reclassified as 3.5 additions.

No other base-manual function identifier was promoted by this pass. Lack of an occurrence in the addendum is not evidence of removal or incompatibility. The German companion book’s general Chapters 1–17 are historical 3.0/3.03 material and are not used to promote inherited functions to 3.5. No runtime-conformance or Rust-implementation claim is made.

See also `amiga-gfa-basic-functions.toml`, `amiga-gfa-basic-candidates.toml`, and the generated `amiga-gfa-basic-candidate-report.md`.
