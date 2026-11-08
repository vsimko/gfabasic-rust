# GFA-BASIC 3.5 additions documented in the Amiga book

Source: `Das Grosse Buch zu GFA-BASIC 3.5 Amiga.pdf`, chapter 18 (PDF pp. 451–475; printed pp. 451–475), plus chapter 19 (PDF pp. 477–482). This is an extraction of what the book explicitly says, not a claim that the list is exhaustive for every GFA-BASIC 3.5 target/platform.

## Explicit version statement

PDF p. 451 introduces chapter 18 as the commands newly added in version 3.5 and says the old commands were not fundamentally changed. The chapter is titled “Vektor- und Matrizenberechnungen”.

## Chapter 18 — matrix/vector command inventory

The following commands are individually described in chapter 18. Page numbers are PDF/printed page numbers (the pages coincide here).

| Command | First description page | Syntax or behavior visible in OCR |
|---|---:|---|
| MAT BASE | 454 | `MAT BASE 0` / `MAT BASE 1`; sets the base index used for matrix/vector indices |
| MAT CLR | 454 | `MAT CLR m()`; clears all elements to zero |
| MAT SET | 455 | `MAT SET m()=x`; sets all elements to a value |
| MAT ONE | 455 | `MAT ONE m()`; creates an identity matrix |
| MAT READ | 456 | `MAT READ m()`; reads enough DATA values to fill the matrix |
| MAT PRINT | 457 | Matrix/vector output; optional formatting parameters and channel output are discussed |
| MAT INPUT | 457–458 | Reads matrix data from a file/channel; partial matrix reads are disallowed |
| MAT CPY | 459 | `MAT CPY a([i,j])=b([k,l])[,h,w]`; copies a matrix or a selected region |
| MAT XCPY | 462 | `MAT XCPY a([i,j])=b([k,l])[,h,w]`; copy variant that transposes the copied region |
| MAT TRANS | 463 | `MAT TRANS a()[=b()]`; copies/transposes a matrix; compatibility of dimensions is discussed |
| MAT ADD | 465 | `MAT ADD a()=b()+c()`; also in-place and scalar-addition forms |
| MAT SUB | 467 | `MAT SUB a()=b()-c()`; also in-place and scalar-subtraction forms |
| MAT MUL | 469 | `MAT MUL a()=b()*c()`; matrix multiplication, plus assignment/scalar forms |
| MAT NORM | 472 | `MAT NORM v(),0` / `MAT NORM v(),1` (OCR); normalizes a vector |
| MAT DET | 474 | `MAT DET y=m([i,j])[,d]` (OCR); determinant |
| MAT QDET | 474 | `MAT QDET y=m(n,n)[,d]` (OCR); faster determinant routine, possible precision differences |
| MAT RANG | 474 | `MAT RANG y=m(...)` (OCR); calculates matrix rank |
| MAT INV | 475 | `MAT INV i()=m()`; inverse of a square matrix |

**Count in this chapter inventory: 18 MAT command families.** This count is based on individually described command headings, not a count of every syntax form or alias.

## Chapter 19 — additional features, attribution needs care

PDF pp. 477–480 explicitly discuss:
- `FACT(n)`: factorial function (PDF p. 477).
- `VARIAT(m,k)`: number of variations without repetition (PDF p. 478).
- `COMBIN(m,k)`: number of combinations (PDF p. 479).
- `_DATA`: DATA-pointer position can be queried/stored/restored (PDF p. 480; OCR text uses “DATA” in prose and the example heading shows `_DATA`).

PDF pp. 481–482 describe editor keyboard additions (Tab / Shift / Ctrl combinations). The book says the last two editor functions were not implemented in the test version available to the authors (PDF p. 482). These editor additions are not language statements/functions and should be tracked separately.

The book's chapter 19 heading calls these “Mehr Bedienkomfort”; PDF p. 477 says they are additions alongside the vector/matrix commands. Unlike chapter 18's explicit opening statement, chapter 19 does not provide an equally clear version-by-version list in the extracted text. Keep these as 3.5 candidates pending reconciliation with the interpreter manual/addendum.

## OCR and verification caveats

- Several command headings and syntax lines are damaged by OCR. In particular, MAT NORM argument syntax, MAT DET/RANG optional parameters, and MAT TRANS assignment syntax should be checked against page images.
- MAT INPUT is split across pages 457–458; the command heading is partly lost in OCR.
- The examples on pp. 458–473 provide additional usage evidence but are not fully transcribed here.
- Do not merge these 18 matrix commands with the 28-declaration denominator currently used by PR #321 without an explicit reconciliation step. They are a separate source-derived inventory pending that reconciliation.
