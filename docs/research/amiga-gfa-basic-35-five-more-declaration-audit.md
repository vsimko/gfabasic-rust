# Amiga GFA BASIC 3.5 — five more declaration-level audit blocks

Primary source: supplied *GFA BASIC 3.5 Interpreter*, Chapter 13 addendum, printed pp. 13-1–13-3 and 13-34–13-36. The independently supplied combined update scan numbers the latter material Chapter 14, printed pp. 14-5–14-7. These are source-confirmed 3.5 forms; runtime conformance is not implied.

## 1. _DATA — query the DATA pointer

- **Context:** special variable / expression value.
- **Source spelling:** `_DATA` (leading underscore confirmed visually on printed p. 13-34).
- **Effect:** reports the current DATA pointer position. The addendum says it is zero when the next `READ` would produce “out of data”.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed p. 13-34; combined update p. 14-5.
- **Unspecified:** numeric representation/type, whether the value is an ordinal, byte/record offset or opaque token, and behavior after program edits or source composition.

## 2. _DATA= — restore the DATA pointer

- **Context:** special assignment form.
- **Source spelling:** `_DATA=value`; the right-hand value is a position previously obtained from `_DATA`.
- **Effect:** sets/restores the DATA pointer to a previously determined position, allowing subsequent `READ` operations to resume there.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed p. 13-34; combined update p. 14-5.
- **Unspecified:** accepted expression type, validation of arbitrary/stale values, behavior across procedures/program changes, and exact error conditions. Do not model it as ordinary assignment to a scalar variable without preserving the special pointer side effect.

## 3. _0 through _9 — special longword variables

- **Context:** ten predefined special variables.
- **Source spellings:** `_0`, `_1`, …, `_9` (leading underscores confirmed on printed p. 13-35).
- **Representation:** longword variables; the source compares them to integer variables with the `%` suffix and distinguishes them from floating-point variables.
- **Lifetime:** cleared by `NEW`; not cleared by `CLEAR`, `LOAD`, `CHAIN`, or `RUN`, according to the addendum.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed p. 13-35; combined update p. 14-6.
- **Unspecified:** exact signedness/range wording, initial values, persistence across interpreter reset/process exit, and aliasing/interaction with memory operations.

## 4. MAT BASE

- **Context:** matrix-operation mode statement.
- **Source forms:** `MAT BASE 0` and `MAT BASE 1`.
- **Effect:** sets the row/column index offset used by the documented matrix operations. The stated default is `MAT BASE 1`; `MAT BASE 0` resets the offset to zero.
- **Affected operations named by the source:** `MAT READ`, `MAT PRINT`, `MAT CPY`, `MAT XCPY`, `MAT ADD`, `MAT SUB`, and `MAT MUL`.
- **Qualification:** the addendum says it is sensibly used with `OPTION BASE 0`. The Chapter 13 matrix operations are limited to one- and/or two-dimensional fields with floating-point variables.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-1–13-2.
- **Unspecified:** invalid argument handling, exact interaction with arrays dimensioned under each OPTION BASE setting, and whether changing MAT BASE affects already dimensioned arrays or only operation indexing.

## 5. MAT CLR, MAT SET, MAT ONE — matrix/vector generation

- **Context:** matrix/vector mutation statements for floating-point fields.
- **Source forms:** `MAT CLR a()`, `MAT SET a()=x`, `MAT ONE a()`.
- **Effects:** `MAT CLR` sets every element to zero; `MAT SET` sets every element to scalar `x`; `MAT ONE` creates a square identity matrix with ones on the diagonal and zeros elsewhere.
- **Source equivalence:** `MAT CLR` is described as equivalent to `ARRAYFILL a(),0`; `MAT SET` is described as equivalent to `ARRAYFILL a(),x`.
- **Qualification:** the Chapter 13 matrix operations apply to one- and/or two-dimensional fields with floating-point variables; do not generalize to integer or string arrays.
- **Version:** explicitly documented in the 3.5 addendum.
- **Source:** printed pp. 13-1–13-3.
- **Unspecified:** exact error behavior for non-square MAT ONE targets, zero-sized dimensions, non-floating arrays, and scalar conversion/precision.

## Compatibility and evidence boundary

These five blocks are confirmed as documented Amiga interpreter 3.5 features by the version-specific addendum. Confirmation is limited to the source-described forms and behavior above. The unresolved points are intentionally not filled using another GFA dialect or generic BASIC assumptions. No Rust implementation or runtime-conformance claim is made here.
