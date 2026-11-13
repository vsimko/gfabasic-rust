---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — inherited function evidence from both manuals"
---

# Amiga GFA BASIC 3.5 — inherited function evidence from both manuals

## Scope and method

This audit compares all 117 identifiers in `amiga-gfa-basic-functions.toml` with the version-specific Chapter 13 addendum of *GFA BASIC 3.5 Interpreter* and the 3.5-specific material in Chapters 18–19 of *Das Grosse Buch zu GFA-Basic 3.5 Amiga*.

A function call in a 3.5-specific example is direct documentary evidence that the shown call form is usable in the 3.5 interpreter. An explicit documentation reference is also evidence for the referenced form. Neither proves that every overload, edge case, numeric boundary, or error behavior from the 3.0 base chapter was revalidated. Each function retains its original `base-manual-3.0` source origin; 3.5 evidence is recorded separately.

The English manual's front matter explicitly says that the book was originally printed for version 3.0 and that Chapter 13 is the addendum describing the extra features provided in version 3.5. Although the supplied software is GFA BASIC 3.5, Chapters 1–12 are therefore treated as 3.0 documentation, not as independent 3.5 confirmation. The German book's Chapter 18 introduction says that the following commands are new in 3.5 and that the old commands were fundamentally unchanged (with minor bug fixes and performance improvements). This is useful general historical context, but it does not restate individual function signatures or overloads; this audit promotes inherited functions only when a 3.5-specific example or explicit reference supplies function-level evidence.

The German book identifies the material in Chapter 18 as new in version 3.5. Its Chapter 18 examples therefore provide independent 3.5 evidence for inherited functions that are not used in the English addendum.

## Directly evidenced inherited functions

| Function | 3.5 source and page | What the source demonstrates | What remains unconfirmed |
|---|---|---|---|
| `RAND` | English interpreter addendum, p. 13-2 | `RAND(10)` populates elements of a floating-point matrix in the `MAT SET` example. | Distribution details, seed behavior, and other random functions are not restated. |
| `STR$` | English interpreter addendum, p. 13-3; German book, p. 457 | The `MAT PRINT` description explicitly refers to `STR$(x,g,n)` formatting; the German book independently corroborates the formatting reference. | The one- and two-argument base-manual forms are not independently restated. |
| `SUCC` | English interpreter addendum, p. 13-5 | A matrix-copy example assigns values using `SUCC(i%)`. | The string overload is not independently demonstrated. |
| `STRING$` | English interpreter addendum, p. 13-21 | Matrix examples use `STRING$(n, "-")` to print separator lines. | The numeric character-code overload is not independently demonstrated. |
| `HEX$` | English interpreter addendum, p. 13-36 | The GURU error-handler example prints `HEX$(GURU)`. | The optional width argument is not independently demonstrated. |
| `RND` | German book, Chapter 18, p. 458 | The matrix example uses `RND(1)` inside `INT(RND(1)*5)`. | Only the one-argument form is demonstrated; `RANDOM` and `RANDOMIZE` are not confirmed by this occurrence. |
| `INT` | German book, Chapter 18, p. 458 | The same matrix example uses `INT(RND(1)*5)`. | Only the one-argument integer-conversion form is demonstrated. |
| `SQR` | German book, Chapter 18, p. 473 | The vector-normalization example uses `SQR(l)` for vector length and `1/SQR(l)` for normalization. | Only the one-argument form is demonstrated. |

These eight identifiers remain recorded with their 3.0 base-manual origin and receive the separate `confirmed-by-3.5-source-evidence` evidence status. Their original base-manual page references remain intact; the 3.5 source identifiers and printed-page locators are stored in separate confirmation fields.

## Similar-looking occurrences deliberately excluded

The English addendum matrix examples contain `~INP(2)` (pp. 13-21 onward). The base-manual function candidate is documented as `INP(#n)`, which reads a byte from an opened data channel. The addendum spelling and context do not establish that it is the same call form. Therefore `INP` is not promoted and remains documented-in-3.0-only pending clarification.

Likewise, apparent matches for `ADD`, `SUB`, and `MUL` in the addendum are `MAT` statements, not function calls. Words such as `AND`, `OR`, `EVEN`, `POINT`, `SCREEN`, `TAB`, `TYPE`, and `WINDOW` occur as prose or labels, not built-in function calls.

The German book also contains `SIN(i%)` and `COS(i%)` in a compiler-performance example on printed p. 430. That example is in Chapter 17, before the book's explicitly version-3.5 Chapter 18 material, and is not labelled as a 3.5 addition or 3.5-specific example. It is therefore not used as independent 3.5 evidence for the trigonometric functions. This is an evidence-scope exclusion, not a claim that those functions are absent from or incompatible with 3.5.

## Result and boundary

The full 117-identifier scan found eight inherited functions with direct 3.5 use/reference evidence: four example uses in the English interpreter addendum (`RAND`, `SUCC`, `STRING$`, `HEX$`), one explicit documentation reference there (`STR$`), and three example uses in the German 3.5 book (`RND`, `INT`, `SQR`). The German book also independently corroborates the `STR$` formatting reference.

Together with the four functions explicitly introduced by the English addendum (`FACT`, `VARIAT`, `COMBIN`, and `GURU`), these are twelve function identifiers with direct 3.5 documentary evidence. The four explicit additions remain a distinct category; the eight inherited functions are not reclassified as 3.5 additions.

No other base-manual function identifier was promoted by this pass. Lack of an occurrence in the audited 3.5 material is not evidence of removal or incompatibility. No runtime-conformance or Rust-implementation claim is made.

See also `amiga-gfa-basic-functions.toml`, `amiga-gfa-basic-candidates.toml`, and the generated `amiga-gfa-basic-candidate-report.md`.
