# Amiga GFA-BASIC 3.5 interpreter addendum — extracted feature inventory

Primary source: `GFA_Basic_3.5_Interpreter.pdf`, supplied in the project workspace. The introduction states that the included software is GFA BASIC 3.5 and that Chapter 13 is an addendum to a manual originally printed for 3.0. Chapter 13 is titled “New features in GFA Basic 3.5”. The following page references are the manual's printed addendum pages.

## 1. Editor changes (printed addendum p. 13-1)

- Editor uses two more bytes per program row than versions 3.0–3.07; the source associates this with faster backward scrolling and folding functions.
- Search can search header rows of closed procedures/functions.
- Listing prints labels two characters to the left, as with CASE.

## 2. Tab-key operations (printed addendum p. 13-1)

| Key | Documented action |
|---|---|
| Tab | Move cursor to next tabulator position |
| Ctrl+Tab | Move cursor to previous tab position |
| LeftShift+Tab | Insert spaces to next tab position |
| RightShift+Tab | Delete blank spaces in one row up to/from cursor |

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
| `MAT READ a()` | Read a previously dimensioned vector/matrix from DATA rows |
| `MAT PRINT [#i,]a()[,g,n]` | Print vector/matrix; vectors on one row with comma-separated elements, matrix rows followed by rowfeed; optional output channel and numeric formatting like `STR$(x,g,n)` |
| `MAT INPUT #i,a()` | Read vector/matrix from ASCII file; source says format is reverse of MAT PRINT and commas/rowfeeds may vary as with INPUT # |

### Copy and transpose — printed pp. 13-4–13-15

| Command | Source-described effect |
|---|---|
| `MAT CPY a([i,j])=b([k,l])[,h,w]` | Copy a selected rectangular region from `b` to `a` |
| `MAT XCPY a([i,j])=b([k,l])[,h,w]` | Copy while transposing source rows/columns; source matrix remains unchanged |
| `MAT TRANS a()[=b()]` | Copy transposed matrix `b` to `a`; for square matrix, `MAT TRANS a()` transposes in place |

The addendum documents special forms of MAT CPY for whole matrices and offset copies. It says only elements with matching indices are copied. For vectors, it distinguishes row vectors (`DIM a(n)`) and column vectors (`DIM a(n,1)`); `MAT CPY` treats both vectors syntactically as column vectors. `MAT XCPY` requires one row vector and one column vector because it transposes the second vector before copying. Optional `h,w` behavior for vectors has additional restrictions (printed pp. 13-12–13-14); preserve these for detailed implementation rather than reducing them to a generic slice copy.

### Arithmetic and matrix analysis — printed pp. 13-15–13-31

| Command form | Source-described effect / constraints |
|---|---|
| `MAT ADD a()=b()+c()` | Element-wise sum into `a`; operands must have same order |
| `MAT ADD a(),b()` | Add `b` element-wise into `a` in place; same order; original `a` lost |
| `MAT ADD a(),x` | Add scalar `x` to every element of `a` in place |
| `MAT SUB a()=b()-c()` | Element-wise difference into `a`; operands must have same order |
| `MAT SUB a(),b()` | Subtract `b` element-wise from `a` in place; same order |
| `MAT SUB a(),x` | Subtract scalar `x` from every element of `a` in place |
| `MAT MUL a()=b()*c()` | Matrix multiplication into `a`, for appropriately dimensioned matrices |
| `MAT MUL x=a()*b()` | Scalar result for compatible vectors with equal element count, as described in the text |
| `MAT MUL x=a()*b()*c()` | Scalar product form for qualified vectors `a` and `c` with matrix `b` between them |
| `MAT MUL a(),x` | Multiply every element of `a` by scalar `x` in place |
| `MAT NORM a(),0` | Normalize a matrix using the maximum norm (the source's example transforms rows/columns as described on pp. 13-22–13-24) |
| `MAT NORM a(),1` | Normalize a matrix using the Euclidean norm (see source examples pp. 13-22–13-24) |
| `MAT DET x=a([i,j])[,n]` | Calculate determinant of a square matrix/selected square submatrix |
| `MAT QDET x=a([i,j])[,n]` | Same purpose as MAT DET; source presents it as a quicker determinant calculation and compares deviation (pp. 13-25–13-28) |
| `MAT RANG x=a([i,j])[,n]` | Calculate rank of a square matrix/selected square submatrix |
| `MAT INV a()=b()` | Calculate inverse of square matrix `b` and write it to `a` (the prose example uses destination/source order; confirm exact operand orientation against the printed syntax before implementation) |

Note: The scanned OCR has obvious ambiguities in some matrix multiplication and inverse examples. Verify exact punctuation, argument orientation, and numerical algorithms against page images before implementing. Do not use OCR artifacts as normative syntax.

## 4. Additional commands in version 3.5 — Chapter 14, printed pp. 13-32–13-36

The manual explicitly says that, in addition to Chapter 13, version 3.5 implements further commands/features (Chapter 14). These are part of the same 3.5 interpreter source and must be included in the compatibility inventory.

| Feature / syntax | Source-described behavior |
|---|---|
| `x=FACT(n)` | Factorial `n!`; defines `0! = 1` |
| `y=VARIAT(n,k)` | Number of k-permutations of n elements without repetition: `n!/(n-k)!` |
| `z=COMBIN(n,k)` | Number of k-combinations without repetition: `n!/((n-k)!*k!)` |
| `OPENW` with negative screen number | Opens window on current (top) screen |
| `PCOLOR foreground[,background]` | Like COLOR, but applies to actual screen output by PRINT |
| `DATA` | Returns/indicates DATA pointer position; zero if next READ would cause “out of data” |
| `_DATA=` | Sets DATA pointer to a previously determined value |
| `_0` … `_9` | Ten special longword variables, like `%`, not floating-point; cleared only by NEW, not by CLEAR, LOAD, CHAIN, or RUN |
| `GURU x` | Enables/disables GURU check: x=0 off, x<>0 on; on GURU Meditation it transfers to ONERROR GOSUB or generates error #109 |
| `GURU` | Function returning the GURU number, per example and explanatory text |

Chapter 14 further notes that GURU error messages #110–#118 correspond to normal GURUs $2–$A. In compiled 3.5 programs, RESUME NEXT and RESUME LABEL cannot be executed after an ONERROR GOSUB call caused by a GURU. The text warns that error #109 is a serious system condition and that GURU checking only works if the system can continue execution.

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
