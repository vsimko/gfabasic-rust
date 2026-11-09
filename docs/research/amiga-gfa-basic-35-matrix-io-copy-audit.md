---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — matrix I/O and copy declaration audit"
---

# Amiga GFA BASIC 3.5 — matrix I/O and copy declaration audit

Primary source: *GFA BASIC 3.5 Interpreter*, Chapter 13 addendum, printed pp. 13-3–13-20. The statements below are explicitly part of the Amiga interpreter 3.5 addendum. The documented matrix operations are restricted to one- and/or two-dimensional fields with floating-point variables (p. 13-1). These source facts do not establish runtime conformance of the Rust implementation.

## 1. MAT READ

- **Context:** statement reading DATA values into a previously dimensioned vector or matrix.
- **Source form:** `MAT READ a()`.
- **Effect:** reads DATA values into the elements of `a()`; the destination must already be dimensioned.
- **Indexing:** affected by `MAT BASE` (printed pp. 13-1–13-2).
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-3–13-5.
- **Unspecified here:** exact behavior when DATA values run out partway through a matrix, conversion/rounding rules, interaction with `RESTORE` and the 3.5 `_DATA` pointer, and exact error codes.

## 2. MAT PRINT

- **Context:** formatted output statement for a vector or matrix.
- **Source form:** `MAT PRINT [#i,]a()[g,n]`.
- **Arguments:** optional output channel `#i`; matrix/vector `a()`; optional numeric formatting parameters `g,n`.
- **Effect:** prints a vector on one row with comma-separated elements; matrix rows are separated by rowfeed. The formatting is described as analogous to `STR$(x,g,n)`.
- **Indexing:** affected by `MAT BASE`.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-3–13-6.
- **Unspecified here:** exact formatting grammar, defaults for omitted `g,n`, behavior for empty dimensions, output-channel error behavior, and byte encoding.

## 3. MAT INPUT

- **Context:** statement reading a vector or matrix from an ASCII file.
- **Source form:** `MAT INPUT #i,a()`.
- **Arguments:** file channel `#i`; previously dimensioned vector/matrix `a()`.
- **Effect:** reads matrix/vector values from an ASCII file. The source describes the file representation as the reverse of `MAT PRINT`; commas and linefeeds may vary in the same manner as ordinary `INPUT #`.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-3–13-6.
- **Unspecified here:** whitespace grammar, decimal separator, malformed-input handling, partial-read behavior, channel state requirements, and exact error codes.

## 4. MAT CPY

- **Context:** region-copy statement for numeric vectors/matrices.
- **Source forms:** `MAT CPY a([i,j])=b([k,l])[,h,w]`; shorthand `MAT CPY a()=b()`.
- **Effect:** copies a rectangular region from source `b` to destination `a`; offsets `(i,j)` and `(k,l)` select destination and source origins, and `h,w` specify copied height and width.
- **Shorthand:** complete overlapping index region is copied when dimensions differ; only row/column indices present in both arrays are copied.
- **Vector rule:** row and column vectors are syntactically treated as column vectors for MAT CPY; the documented vector form is `MAT CPY a(n,1)=b(m,1)`.
- **Indexing:** affected by `MAT BASE`.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-8–13-15, including vector/extent rules on pp. 13-17–13-19.
- **Unspecified here:** overlap/aliasing when source and destination are the same array, invalid offsets, out-of-range extents, and exact error behavior.

## 5. MAT XCPY and MAT TRANS

- **Context:** transpose-copy and transpose statements.
- **Source forms:** `MAT XCPY a([i,j])=b([k,l])[,h,w]`; `MAT TRANS a()[=b()]`.
- **MAT XCPY effect:** copies the source into the destination while transposing rows and columns; source matrix remains unchanged. It follows the MAT CPY offset/extent model.
- **MAT XCPY vector rules:** one operand must be explicitly dimensioned as a row vector and the other as a column vector. The source documents `DIM a(1,n),b(m,1)` and `DIM a(n,1),b(1,m)`; it instructs that the dimensions in the MAT XCPY form be written syntactically as column vectors. Optional vector extents have asymmetric zero rules: for column-to-row, `w>=1` makes only `h` relevant and `w=0` copies nothing; for row-to-column, `h>=1` makes only `w` relevant and `h=0` copies nothing.
- **MAT TRANS effect:** `MAT TRANS a()=b()` copies transpose of `b` to `a`; if `b` is n×m, `a` must be m×n. `MAT TRANS a()` transposes a square matrix in place.
- **Indexing:** MAT XCPY is affected by `MAT BASE`; MAT TRANS is described separately.
- **Version:** both explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-15–13-20.
- **Unspecified here:** aliasing behavior for MAT XCPY, invalid dimensions/offsets, in-place rectangular transpose, and exact runtime errors.

## Compatibility boundary

The forms and behaviors above are addendum-backed 3.5 source facts. They are not claims that all corner cases are known or implemented. Preserve the unusual vector syntax and MAT XCPY extent-zero rules in future parser/semantic tests; do not replace them with a generic rectangular-array copy rule. Runtime edge cases require a version-identified Amiga GFA BASIC 3.5 interpreter.
