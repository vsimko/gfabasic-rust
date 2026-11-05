# Amiga GFA-BASIC 3.5 interpreter addendum — extracted feature inventory

Primary source: the supplied `GFA_Basic_3.5_Interpreter.pdf`; independently cross-checked against `GFA-Basic 3.0 + 3.5 Upd (Amiga).pdf` (487 PDF pages). The second scan's PDF p. 3 identifies the base as `GFA BASIC 3.0 Interpreter`; p. 4 gives ©1988 GFA Systemtechnik GmbH, program author Frank Ostrowski, second edition December 1988; p. 7 explicitly instructs readers to add Chapter 13, Chapter 14, and the 3.5 editor changes to the 3.0 manual. The following references are printed addendum page numbers.

## 1. Editor changes (printed base-manual p. 1-25; explicit 3.5 update)

- Editor uses two more bytes per program row than versions 3.0–3.041; the source says this makes backward scrolling faster and enables folding functions.
- Search can also find closed procedure/function header rows.
- During listing, labels are shifted two characters to the left, as with `CASE`.

## 2. Tab-key operations (printed base-manual p. 1-25; explicit 3.5 update)

| Key | Documented action |
|---|---|
| Tab | Move cursor to next tabulator position |
| Ctrl+Tab | Move cursor to previous tab position |
| LeftShift+Tab | Insert spaces to next tab position |
| RightShift+Tab | Delete blank spaces in the current line “up to and from the cursor” (the manual does not define a more precise selection boundary) |

## 3. Linear operations on numeric fields

The addendum explicitly limits all operations in Chapter 13 to one- and/or two-dimensional fields with floating-point variables (printed p. 13-1). Do not generalize them to integer/string arrays without evidence.

### Index base — printed pp. 13-1–13-2

- `MAT BASE 0` and `MAT BASE 1` set/reset the row/column offset used by the listed matrix operations.
- The text says `MAT BASE` is sensibly used when `OPTION BASE 0` is active; `MAT BASE 1` sets the matrix-operation offset to 1, and `MAT BASE 0` resets it to 0.
- The stated default is `MAT BASE 1`.
- The setting affects `MAT READ`, `MAT PRINT`, `MAT CPY`, `MAT XCPY`, `MAT ADD`, `MAT SUB`, and `MAT MUL`.

### Generation — printed pp. 13-2–13-3

| Command | Source-described effect |
|---|---|
| `MAT CLR a()` | Set every element of vector/matrix `a()` to zero; described as equivalent to `ARRAYFILL a(),0` |
| `MAT SET a()=x` | Set every element to scalar `x`; described as equivalent to `ARRAYFILL a(),x` |
| `MAT ONE a()` | Generate a square identity/unity matrix: diagonal elements 1, all others 0 |

### Read, print, and input — printed pp. 13-3–13-4

| Command | Source-described effect |
|---|---|
| `MAT READ a()` | Read a previously dimensioned vector/matrix from DATA rows (printed p. 13-5) |
| `MAT PRINT [#i,]a()[g,n]` | Print vector/matrix; vectors on one row with comma-separated elements, matrix rows followed by rowfeed; optional output channel and numeric formatting like `STR$(x,g,n)` (printed pp. 13-5–13-6) |
| `MAT INPUT #i,a()` | Read vector/matrix from an ASCII file; the format is the reverse of MAT PRINT, and commas/linefeeds may vary as with `INPUT #` (printed p. 13-6) |

### Copy and transpose — printed pp. 13-4–13-15

| Command | Source-described effect |
|---|---|
| `MAT CPY a([i,j])=b([k,l])[,h,w]` | Copy a selected rectangular region from `b` to `a` |
| `MAT XCPY a([i,j])=b([k,l])[,h,w]` | Copy while transposing source rows/columns; source matrix remains unchanged |
| `MAT TRANS a()[=b()]` | Copy transposed matrix `b` to `a`; for square matrix, `MAT TRANS a()` transposes in place |

The page-image review establishes the following exact rules (printed pp. 13-8–13-19):

- `MAT CPY a([i,j])=b([k,l])[,h,w]` copies a rectangle beginning at destination offset `(i,j)` from source offset `(k,l)`; `h,w` specify the copied height and width. The shorthand `MAT CPY a()=b()` copies the complete overlapping index region when the matrices have different orders: only elements whose row and column indices exist in both source and destination are copied (pp. 13-8–13-10, 13-14–13-15).
- `MAT XCPY a([i,j])=b([k,l])[,h,w]` follows the same offset/extent model, but transposes `b` while copying it into `a`; `b` remains unchanged. As with `MAT CPY`, only elements with matching source/destination indices are copied (p. 13-15).
- Vector representation is significant. `DIM a(n)` creates a row vector; `DIM a(n,1)` creates a column vector. `MAT CPY` treats both row and column vectors syntactically as column vectors, so its vector copy form is `MAT CPY a(n,1)=b(m,1)` regardless of the actual vector orientation (p. 13-17).
- `MAT XCPY` requires one operand explicitly dimensioned as a row vector and the other as a column vector; it transposes the second vector before copying. The documented supported cases are `DIM a(1,n),b(m,1)` (row-vector `a`, column-vector `b`) and `DIM a(n,1),b(1,m)` (column-vector `a`, row-vector `b`). For a correct `MAT XCPY` form, the manual says to write the destination/source dimensions as column vectors syntactically (pp. 13-17–13-18).
- Optional vector extents have special zero rules (p. 13-19): for `MAT CPY`, when `w>=1`, only `h` is considered; `w=0` performs no copy. For `MAT XCPY`, when transposing a column vector `b` into a row vector, `w>=1` makes only `h` relevant and `w=0` performs no copy. When transposing a row vector `b` into a column vector, `h>=1` makes only `w` relevant and `h=0` performs no copy.
- `MAT TRANS a()=b()` copies the transpose of `b` into `a`; the matrices must have compatible reversed dimensions (if `b` is `n×m`, `a` must be `m×n`). The example uses `DIM a(3,4),b(4,3)`. For a square matrix, `MAT TRANS a()` transposes in place (pp. 13-19–13-20).

These rules are source-verified for the 3.5 addendum and should be preserved explicitly in parser/semantic tests; do not reduce them to a generic slice-copy abstraction.

### Arithmetic and matrix analysis — printed pp. 13-15–13-31

| Command form | Source-described effect / constraints |
|---|---|
| `MAT ADD a()=b()+c()` | Element-wise sum into `a`; operands must have same order |
| `MAT ADD a(),b()` | Add `b` element-wise into `a` in place; same order; original `a` lost |
| `MAT ADD a(),x` | Add scalar `x` to every element of `a` in place |
| `MAT SUB a()=b()-c()` | Element-wise difference into `a`; operands must have same order |
| `MAT SUB a(),b()` | Subtract `b` element-wise from `a` in place; same order |
| `MAT SUB a(),x` | Subtract scalar `x` from every element of `a` in place |
| `MAT MUL a()=b()*c()` | If `b` and `c` are matrices, multiply rows of `b` by columns of `c`; `b`'s column count must equal `c`'s row count, and result `a` has `b`'s row count and `c`'s column count (printed p. 13-27). If `b` and `c` are vectors, this form produces their dyadic/outer product (printed p. 13-28). |
| `MAT MUL x=a()*b()` | For vectors with equal element counts, calculate scalar/dot (inner) product: sum of pairwise element products (printed p. 13-29). |
| `MAT MUL x=a()*b()*c()` | Scalar result for a row vector `a`, compatible matrix `b`, and column vector `c`; `a` has as many elements as `b` has rows, and `c` as many elements as `b` has columns (printed p. 13-30). |
| `MAT MUL a(),x` | Multiply every element of `a` by scalar `x` in place |
| `MAT NORM a(),0` | Normalize row-wise: each row is scaled so the sum of squares of its elements is 1 (printed pp. 13-31–13-32). |
| `MAT NORM a(),1` | Normalize column-wise: each column is scaled so the sum of squares of its elements is 1 (printed pp. 13-31–13-33). |
| `MAT DET x=a([i,j])[,n]` | Calculate determinant of a square matrix/selected square submatrix |
| `MAT QDET x=a([i,j])[,n]` | Same determinant purpose and submatrix selection as `MAT DET`; optimized for speed, not accuracy. The manual recommends `MAT DET` for critical matrices whose determinant is near zero (printed pp. 13-36–13-37) |
| `MAT RANG x=a([i,j])[,n]` | Calculate rank of a square matrix; like `MAT DET`/`MAT QDET`, accepts row and column offsets, with `n` selecting an n×n submatrix from that position (printed p. 13-38) |
| `MAT INV b()=a()` | Calculate the inverse of square matrix `a` and write it to destination matrix `b`; `b()` must have the same type/dimensions as `a()` (printed p. 13-39) |

Page-image checks of printed pp. 13-27–13-33 establish matrix multiplication dimensions and the row/column normalization meanings above. Printed pp. 13-36–13-39 confirm the `MAT QDET` performance/accuracy warning, the `MAT RANG` square-submatrix selection rule, and the detailed `MAT INV b()=a()` example. **Important source inconsistency:** the command summary on printed p. 13-22 instead lists `MAT INV a()=b()`. Until this discrepancy is resolved by another authoritative edition or interpreter observation, record the inverse operand direction as disputed. The earlier OCR-only warning for the matrix multiplication examples is superseded by the page-image checks recorded above. Detailed numerical algorithm derivations and other unreviewed sections remain to be checked. Do not use OCR artifacts as normative syntax.

## 4. Additional commands in version 3.5 — combined-scan Chapter 14, printed pp. 14-1–14-7

**Pagination crosswalk:** the supplied 469-page `GFA_Basic_3.5_Interpreter.pdf` presents the version 3.5 material as Chapter 13 only; its `_DATA`/`_DATA=` definitions are on printed p. 13-34 and `_0`…`_9` on printed p. 13-35. The separately catalogued 487-page `GFA-Basic 3.0 + 3.5 Upd (Amiga).pdf` binds further 3.5 commands as Chapter 14, printed pp. 14-1–14-7, with `_DATA`/`_DATA=` on p. 14-5 and `_0`…`_9` on p. 14-6. References in this section therefore use the combined scan's Chapter 14 pagination; references to the supplied interpreter PDF must use Chapter 13 pagination. These are two pagination witnesses for the same 3.5 feature material, not evidence for importing Atari or 3.0-only rules.

The combined update scan explicitly says that, in addition to Chapter 13, version 3.5 implements further commands/features (Chapter 14). These are part of the same 3.5 interpreter source and must be included in the compatibility inventory.

| Feature / syntax | Source-described behavior |
|---|---|
| `x=FACT(n)` | Factorial `n!`; `n` is an integer expression, result `x` is an arithmetic expression; defines `0! = 1` (printed pp. 14-1–14-2) |
| `y=VARIAT(n,k)` | Number of k-permutations of n elements without repetition: `n!/(n-k)!`; `n,k` are integer expressions (printed pp. 14-1–14-2) |
| `z=COMBIN(n,k)` | Number of k-combinations without repetition: `n!/((n-k)!*k!)`; `n,k` are integer expressions (printed p. 14-2) |
| `OPENW` with negative screen number | Opens window on the current (top) screen (printed p. 14-3) |
| `PCOLOR foreground[,background]` | Like `COLOR`, but applies to screen output produced by `PRINT`; background is optional in the examples (printed p. 14-4) |
| `_DATA` | Returns/indicates DATA pointer position; zero if the next `READ` would cause “out of data” |
| `_DATA=` | Sets the DATA pointer to a previously determined value obtained from `_DATA` |
| `_0` … `_9` | Ten special longword variables, like `%`, not floating-point; cleared only by `NEW`, not by `CLEAR`, `LOAD`, `CHAIN`, or `RUN` (printed p. 14-6) |
| `GURU x` | Enables/disables GURU checking: `x=0` disables it; `x<>0` enables it. On a GURU Meditation it transfers to `ONERROR GOSUB` or generates error #109 (printed pp. 14-6–14-7) |
| `GURU` | Function returning the GURU number (printed pp. 14-6–14-7) |

Chapter 14's opening page enumerates these as the additional 3.5 features (printed p. 14-1). The examples and definitions are on pp. 14-1–14-6. The closing notes (p. 14-7) state that error messages #110–#118 correspond to normal GURUs $2–$A; in compiled 3.5 programs, `RESUME NEXT` and `RESUME LABEL` cannot be executed after an `ONERROR GOSUB` call caused by a GURU. The manual treats error #109 (GURU Meditation) as a serious system condition and warns that GURU checking only works if the system can continue execution.

Page-image review of the combined update scan's printed p. 14-5 confirms the leading underscore in both DATA-pointer operations: `_DATA` and `_DATA=`. Ordinary `DATA` remains the data declaration statement. Printed p. 14-6 confirms `_0`…`_9` are longword special variables and distinguishes `GURU x` from the `GURU` function.

## 5. Implementation and conformance work breakdown

1. Add parser/AST coverage for every MAT statement and Chapter 14 function/command above.
2. Implement floating-point-only array constraints and dimension checks from the source; do not silently accept integer arrays.
3. Implement and test MAT BASE interaction with OPTION BASE and each affected command.
4. Add reference tests for generation, DATA/file I/O, region copy, transpose, arithmetic, norms, determinant, quick determinant, rank, and inverse.
5. Add tests for DATA pointer query/restore, special-variable persistence, PCOLOR, negative-screen OPENW, and GURU handling. Tests involving OS crash recovery must be isolated and carefully sandboxed.
6. Keep editor behavior separate from language/runtime semantics; it may be documented even if the Rust project does not implement the historical editor.
7. Mark all 3.0 manual features not explicitly reconfirmed by this 3.5 manual as inherited candidates, not verified 3.5 facts.

## Source and rights

This file contains concise original summaries and syntax inventories, not a reproduction of the manual. Cite the source as `GFA BASIC 3.5 Interpreter`, Chapter 13/14, printed addendum page N. The supplied PDF is a research source and is not committed to the repository.
