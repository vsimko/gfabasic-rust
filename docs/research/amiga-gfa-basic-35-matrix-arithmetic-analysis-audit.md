---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — matrix arithmetic and analysis audit"
---

# Amiga GFA BASIC 3.5 — matrix arithmetic and analysis audit

Primary source: *GFA BASIC 3.5 Interpreter*, Chapter 13 addendum, printed pp. 13-15–13-39. All these operations are explicitly documented in the 3.5 addendum and apply to the documented one- and/or two-dimensional floating-point fields (p. 13-1). The descriptions below are source facts, not runtime-conformance claims.

## 1. MAT ADD

- **Forms:** `MAT ADD a()=b()+c()`; `MAT ADD a(),b()`; `MAT ADD a(),x`.
- **Effects:** first form stores element-wise sum of `b` and `c` in `a`; second adds `b` element-wise to `a` in place; third adds scalar `x` to every element of `a` in place.
- **Constraints:** array operands in the element-wise forms must have the same order.
- **Version/source:** 3.5 addendum, printed pp. 13-22–13-24.
- **Unspecified:** scalar coercion/precision, aliasing between destination and operands, mismatch errors and exact error codes.

## 2. MAT SUB

- **Forms:** `MAT SUB a()=b()-c()`; `MAT SUB a(),b()`; `MAT SUB a(),x`.
- **Effects:** first form stores element-wise difference of `b` minus `c` in `a`; second subtracts `b` element-wise from `a` in place; third subtracts scalar `x` from every element of `a` in place.
- **Constraints:** array operands in the element-wise forms must have the same order.
- **Version/source:** 3.5 addendum, printed pp. 13-24–13-26.
- **Unspecified:** scalar coercion/precision, aliasing, mismatch errors and exact error codes.

## 3. MAT MUL

- **Forms:** `MAT MUL a()=b()*c()`; `MAT MUL x=a()*b()`; `MAT MUL x=a()*b()*c()`; `MAT MUL a(),x`.
- **Matrix product:** for matrices `b` and `c`, rows of `b` are multiplied by columns of `c`; the column count of `b` must equal the row count of `c`; result `a` has the row count of `b` and column count of `c`.
- **Vector products:** two vectors in the destination-array form produce their dyadic/outer product. The scalar two-vector form produces the inner/dot product, summing pairwise element products. The three-factor scalar form accepts a row vector, compatible matrix, and column vector, with the vector lengths matching the matrix row and column counts respectively.
- **Scalar form:** `MAT MUL a(),x` multiplies every element of `a` by scalar `x` in place.
- **Version/source:** 3.5 addendum, printed pp. 13-26–13-30.
- **Unspecified:** empty-dimension behavior, scalar conversion/precision, aliasing, exact vector orientation diagnostics and error codes. The addendum's brief command summary and detailed examples should be read together; preserve each form separately.

## 4. MAT NORM

- **Forms:** `MAT NORM a(),0`; `MAT NORM a(),1`.
- **Effect with mode 0:** normalize row-wise; each row is scaled so the sum of squares of its elements is 1.
- **Effect with mode 1:** normalize column-wise; each column is scaled so the sum of squares of its elements is 1.
- **Version/source:** 3.5 addendum, printed pp. 13-31–13-33.
- **Unspecified:** behavior for zero rows/columns, zero-length dimensions, floating-point underflow/overflow, and invalid mode values.

## 5. MAT DET, MAT QDET, MAT RANG, MAT INV

- **MAT DET form:** `MAT DET x=a([i,j])[,n]`; calculates determinant of a square matrix or selected square submatrix (printed p. 13-26).
- **MAT QDET form:** `MAT QDET x=a([i,j])[,n]`; same determinant purpose and submatrix selection, optimized for speed rather than accuracy. The manual recommends MAT DET for critical matrices whose determinant is near zero (printed pp. 13-27–13-28).
- **MAT RANG form:** `MAT RANG x=a([i,j])[,n]`; calculates rank of a square matrix; optional offsets select the starting row/column and `n` selects an n×n submatrix (printed p. 13-29).
- **MAT INV form:** `MAT INV destination()=source()`; writes the inverse of square source matrix to destination, which must have the same type/dimensions. The detailed definition and worked example (printed p. 13-30) establish destination-first order; the command summary (printed p. 13-16) reverses the operands and is treated as a source typo.
- **Version/source:** 3.5 addendum, printed pp. 13-26–13-30. MAT INV direction also independently corroborated by the 1990 German Amiga book, p. 475.
- **Unspecified:** singular-matrix behavior, numerical tolerances, rank threshold, invalid offsets/submatrix sizes, destination initialization, and exact runtime errors.

## Compatibility boundary

The forms and semantics above are documented by the Amiga 3.5 addendum. No unsupported assumptions have been made about numerical edge cases or runtime errors. In particular, MAT QDET's accuracy warning and MAT INV's destination/source order are part of the documented contract and should be represented explicitly in future tests. Runtime numerical behavior still requires a version-identified original interpreter.
