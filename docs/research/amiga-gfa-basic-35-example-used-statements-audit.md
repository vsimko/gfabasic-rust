---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — inherited statements used in examples"
---

# Amiga GFA BASIC 3.5 — inherited statements used in examples

## Scope

This pass records additional inherited statements that are directly used in the 3.5 companion book's Chapter 18–19 examples but were not yet represented as granular candidate entries. The source origin remains the 3.0 base interpreter manual; the companion book supplies a separate 3.5 example-use witness. Each confirmation is limited to the exact form shown.

Page-image checks were made for the Chapter 18 examples on printed pp. 458 and 471, the Chapter 19 DATA-pointer example on p. 480, and the English 3.5 addendum example on p. 13-24.

## Directly evidenced statements

| Candidate | 3.5 evidence | Demonstrated form | Boundary |
|---|---|---|---|
| `DIM` | pp. 458, 471, 480 | Numeric vector/matrix dimensions and a DATA-pointer index array | Does not confirm every type suffix or bounds form |
| `OPTION BASE` | p. 458 | `OPTION BASE 0` | `OPTION BASE 1` is not shown |
| `INC` | p. 480 | `INC i` in the DATA-pointer loop | Does not confirm DEC or every typed/overflow behavior |
| `DATA` | p. 471 | Numeric DATA items for vector coordinates | Does not confirm string DATA or numeric-prefix grammar |
| `READ` | p. 480 | `READ a` into one scalar variable | Variable lists and string reads are not shown |
| `RESTORE` | p. 480 | Bare `RESTORE` before reading from the beginning | Labelled RESTORE is not shown |
| `OPEN` | p. 458 | Output mode `"O"` and input mode `"I"` with numbered channel | Append, update, and random-access modes are not shown |
| `CLOSE` | p. 458 | `CLOSE #1` | The no-argument close-all form is not shown |
| `PAUSE` | p. 471 | `PAUSE 50` | Does not confirm DELAY or every timing edge case |
| `PROCEDURE / RETURN` | p. 471 | Parameterless `PROCEDURE drawp` and `RETURN` | GOSUB and parameter-passing forms are not shown |
| `@procedure-call` | p. 471 | `@drawp` invokes the procedure | Confirms the shown prefix-call form only |
| `END` | p. 471 | Top-level `END` | EDIT and STOP alternatives are not shown |
| `CLS` | p. 13-24 (3.5 addendum) | `CLS` before `MAT READ` | The optional `CLS #n` redirection form is not shown |

## Siblings deliberately left unconfirmed

| Candidate | 3.0 source page | Why it remains unconfirmed |
|---|---:|---|
| `DEC` | p. 4-10 | The 3.5 example uses INC, not DEC |
| `DELAY` | p. 7-36 | The 3.5 example uses PAUSE, not DELAY |
| `GOSUB` | pp. 7-21–7-22 | The example defines and invokes a procedure using the `@` procedure-call form; it does not use GOSUB |
| `EDIT / STOP` | p. 7-37 | The example uses END, not EDIT or STOP |

These non-occurrences do not imply removal or incompatibility. They only mean that this particular 3.5 example pass supplies no direct witness for those siblings.

## Result

Thirteen additional inherited statement/procedure-call records receive direct 3.5 example-use evidence. Their 3.0 source origin is retained and their 3.5 witness pages are stored separately in the candidate manifest. No registry declaration, implementation state, runtime conformance, or language-core count is changed. This is an evidence inventory increment, not a complete statement catalogue.
