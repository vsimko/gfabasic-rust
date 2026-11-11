---
type: "Research Report"
title: "Amiga GFA BASIC 3.5 candidate verification report"
tags: [amiga, gfa-basic, evidence, research]
---

# Amiga GFA BASIC 3.5 candidate verification report

> Generated from `amiga-gfa-basic-candidates.toml`. Status describes evidence of 3.5 applicability, not implementation or runtime support.

## Summary

| Status | Entries |
|---|---:|
| candidate | 0 |
| confirmed-in-3.5 | 13 |
| contradicted | 0 |
| documented-in-3.0-only | 100 |
| unresolved | 0 |

## Unresolved and pending candidates

| ID | Candidate | Source scope | Printed pages | Notes |
|---|---|---|---|---|
| — | None | — | — | — |

## Documented in the 3.0 base only (3.5 applicability not established)

| ID | Candidate | Printed pages | Evidence boundary |
|---|---|---|---|
| `amiga.base.advanced` | Advanced programming and hardware features | 10-2–10-14 | SPRITE, STICK and STRIG have selected notes; CLI, EXEC, SAY, FD-files and object commands need reconciliation. |
| `amiga.base.appendices` | Appendix tables and command index | 12-2–12-19 | Reference material is not a declaration candidate by itself; index entries must be checked against their cited chapter pages. |
| `amiga.base.events-windows` | Events, menus, screens and windows | 9-2–9-37 | FILESELECT and selected screen/window topics are inventoried; event/menu/window behavior needs systematic 3.5 verification. |
| `amiga.base.files` | Disk handling and file organization | 6-2–6-21 | File-channel I/O is partially extracted; other disk and random-access operations need separate 3.5 checks. |
| `amiga.base.graphics` | Graphics language commands | 8-2–8-24 | Selected drawing and fill operations have page-backed notes; full graphics surface is not individually reconciled. |
| `amiga.base.input-output` | Input and output | 5-2–5-22 | Selected command forms have page-backed notes; complete declaration-by-declaration 3.5 reconciliation remains outstanding. |
| `amiga.base.numeric-string` | Numeric and string functions | 4-2–4-36 | The base inventory extracts selected semantics only; the full Chapter 4 family is not individually verified for 3.5. |
| `amiga.base.operators` | Operators and precedence | 3-2–3-16 | OCR-sensitive operator glyphs and precedence spellings require page-image and 3.5 reconciliation. |
| `amiga.base.os-libraries` | Amiga OS library calls | 11-2–11-92 | Seven library families and many routines are inventoried; exact wrapper contracts, OCR details and 3.5 availability remain unverified. |
| `amiga.base.program-control` | Program handling and control flow | 7-2–7-55 | Selected conditionals, loops and branching forms are extracted; procedures, events, errors and editor commands remain to be reconciled. |
| `amiga.base.variables-memory` | Variables, arrays and memory | 2-2–2-44 | Chapter 2 contains multiple declarations; this family row is a tracking boundary, not a claim that every listed feature is absent from 3.5. |
| `amiga.base10.sprite` | SPRITE | 10-5–10-6 | Defines, frees, moves, enables and disables Amiga hardware sprites; source describes eight sprite numbers and shared registers. 3.5 availability is unresolved. |
| `amiga.base10.stick-strig` | STICK / STRIG | 10-7 | Queries joystick position and fire-button state; port 0 interaction with mouse/Intuition is noted. 3.5 availability is unresolved. |
| `amiga.base2.absolute` | ABSOLUTE | 2-39 | Address/variable binding feature in the base manual; classification and detailed signature remain source-scoped. |
| `amiga.base2.asc-chr` | ASC / CHR$ | 2-13 | ASC returns the first character code (zero for empty input); CHR$ uses the low byte. 3.0 base-manual evidence only. |
| `amiga.base2.bin-oct-hex` | BIN$ / OCT$ / HEX$ | 2-14 | Integer-to-base-2/base-8/base-16 string conversions; optional width parameters are documented. 3.0 base-manual evidence only. |
| `amiga.base2.cint-cfloat` | CINT / CFLOAT | 2-17 | CINT rounds float to integer; CFLOAT converts integer to float. The manual explicitly says these are meaningful only to the compiler, not interpreter built-ins. |
| `amiga.base2.cvx` | CVI / CVL / CVS (CVF spelling disputed) | 2-18–2-19 | Binary-string conversions; prose names CVS while the MKx counterpart list includes MKF$. Preserve the source naming mismatch; do not normalize a CVF alias. |
| `amiga.base2.date-time` | DATE$ / TIME$ | 2-26 | Read/write date and read time string forms are described; the source text's DATE$/DATES and TIME$/TIMES naming is retained for further checking. |
| `amiga.base2.dump` | DUMP | 2-31 | DUMP heading and page locator are documented; preserve uncertainty about callable form until detailed page transcription. |
| `amiga.base2.false-true-pi` | FALSE / TRUE / PI | 2-25 | FALSE=0, TRUE=-1, PI is the mathematical constant. 3.0 base-manual evidence only. |
| `amiga.base2.fre` | FRE | 2-29 | Memory-availability query documented in the base manual; argument/return variants need exact signature extraction. |
| `amiga.base2.malloc-mfree` | MALLOC / MFREE | 2-41–2-42 | Memory allocation/release forms are documented; allocator/error/ownership semantics are not promoted to 3.5. |
| `amiga.base2.mkx` | MKI$ / MKL$ / MKS$ / MKF$ / MKD$ | 2-18–2-19 | Numeric-to-binary-string conversion family; counterpart mapping inherits the CVx naming inconsistency. 3.0 base-manual evidence only. |
| `amiga.base2.peek` | PEEK | 2-36 | Memory-read forms are documented in the base manual; width/type variants need independent 3.5 evidence. |
| `amiga.base2.pointer-star` | * (array descriptor pointer form) | 2-33 | Indirect array-descriptor form is documented alongside ARRPTR; keep separate from multiplication operator. |
| `amiga.base2.str` | STR$ | 2-15 | Overloads STR$(x), STR$(x,y), STR$(x,y,z); width and selected-slice behavior are described. 3.0 base-manual evidence only. |
| `amiga.base2.timer` | TIMER | 2-28 | Timer function heading and page locator in the base manual; exact return/tick contract is not promoted. |
| `amiga.base2.type` | TYPE | 2-30 | Variable type query documented in the base manual; 3.5 applicability unverified. |
| `amiga.base2.val` | VAL / VAL? | 2-16 | VAL parses a numeric prefix; VAL? reports consumed characters. 3.0 base-manual evidence only. |
| `amiga.base2.varptr-arrptr` | VARPTR / ARRPTR | 2-33 | Distinguishes element/string-data address from array-descriptor address. 3.0 base-manual evidence only. |
| `amiga.base2.void` | VOID | 2-40 | VOID heading and page locator are documented; exact return semantics remain unverified. |
| `amiga.base3.comparison-equality` | = / == / <> | 3-2, 3-11–3-16 | = compares and also assigns in variable context; == is approximate numeric equality (8.5 decimal places); <> is inequality. |
| `amiga.base3.integer-divide-mod` | DIV / MOD | 3-2 | Whole-number division and modulo precedence documented; no 3.5 confirmation. |
| `amiga.base3.logical` | NOT / AND / OR / XOR / IMP / EQV | 3-2–3-9 | NOT is unary; remaining operators are binary; logical truth and 32-bit bitwise behavior are described. 3.0 base-manual evidence only. |
| `amiga.base3.multiply-divide` | * / | 3-2 | Multiplication and division precedence documented; no 3.5 confirmation. |
| `amiga.base3.numeric-add-sub` | Numeric + / - | 3-2, 3-10 | Arithmetic addition/subtraction documented separately from string concatenation. |
| `amiga.base3.power` | ^ | 3-2 | Power operator precedence is documented in the 3.0 base manual only. |
| `amiga.base3.precedence` | Operator precedence hierarchy | 3-2 | Page-image transcription gives 11 precedence levels; comparison rows contain OCR/source spelling ambiguity. No associativity inferred. |
| `amiga.base3.relational` | < / > / <= / >= and =< / => | 3-2, 3-13–3-16 | Text explicitly accepts <= and =<, and >= and =>. Preserve both alternate spellings; string ordering is separately described. |
| `amiga.base3.string-plus` | String concatenation + | 3-2–3-10 | Plus is documented for string concatenation as well as numeric addition; 3.0 base-manual evidence only. |
| `amiga.base3.unary-plus-minus` | Unary + / - | 3-2, 3-10 | Prefix plus/minus are separately ranked in the precedence table; exact parse/associativity not inferred. |
| `amiga.base4.abs-sgn` | ABS() / SGN() | 4-12 | Absolute value and sign (-1, 0, 1) functions documented. |
| `amiga.base4.bit-test` | BCLR() / BSET() / BCHG() / BTST() | 4-19–4-20 | Zero-based bit indices masked to 0–31; BTST is described with 1/0 result in this section. |
| `amiga.base4.bitwise-and-or-xor` | AND() / OR() / XOR() | 4-23 | Integer bitwise function forms; distinct from logical operators as expression operators pending exact parser treatment. |
| `amiga.base4.bitwise-imp-eqv` | IMP() / EQV() | 4-24 | Integer bitwise function forms documented separately from logical truth tables. |
| `amiga.base4.byte-card-word` | BYTE() / CARD() / WORD() | 4-26 | Byte/word conversion and extension operations documented. |
| `amiga.base4.deg-rad` | DEG() / RAD() | 4-7 | Degree/radian conversion functions documented in the 3.0 base manual. |
| `amiga.base4.exp-log` | EXP() / LOG() / LOG10() | 4-9 | Exponential and logarithm family; retain LOG10 as candidate pending precise page-level confirmation. |
| `amiga.base4.instr-rinstr` | INSTR() / RINSTR() | 4-30–4-31 | Forward/reverse substring search functions; preserve source-specific index semantics. |
| `amiga.base4.int-trunc-fix-frac` | INT() / TRUNC() / FIX() / FRAC() | 4-13 | Manual distinguishes truncation from floor for negative values and describes TRUNC/FRAC relation. |
| `amiga.base4.integer-arithmetic-functions` | ADD() / SUB() / MUL() / DIV() / MOD() | 4-3 | Integer-expression function forms are distinct from same-named mutating commands on pp. 4-2–4-3; decimal places are ignored. |
| `amiga.base4.left-right` | LEFT$() / RIGHT$() | 4-27 | Substring functions; omitted count returns one character. |
| `amiga.base4.len-trim` | LEN() / TRIM$() | 4-34 | String length and trim functions documented. |
| `amiga.base4.lset-rset` | LSET / RSET | 4-36 | Left/right justified assignment into existing string variable; padding/truncation details remain unspecified. |
| `amiga.base4.mid` | MID$() | 4-28 | Function returns substring; separate MID$ command is on p. 4-29 and is not merged here. |
| `amiga.base4.mid-command` | MID$ command | 4-29 | Mutates an existing string variable; not the MID$ value-returning function. |
| `amiga.base4.min-max` | MIN() / MAX() | 4-16 | Accept a list of numeric or string expressions and return the extremum. |
| `amiga.base4.odd-even` | ODD() / EVEN() | 4-15 | Boolean result is -1/0; zero is even. |
| `amiga.base4.pred-succ` | PRED() / SUCC() | 4-11, 4-33 | Numeric and string forms are separately discussed; do not collapse their argument/result contracts. |
| `amiga.base4.quadrant-trigonometric` | SINQ() / COSQ() | 4-6 | Degree-based interpolated lookup-table functions; interpolation described in 1/16-degree steps. |
| `amiga.base4.random` | RND() / RANDOM() / RAND() / RANDOMIZE | 4-17–4-18 | Distinct result ranges and seeding forms are described; preserve separate contracts for RND, RANDOM and RAND. |
| `amiga.base4.rotate` | ROL() / ROR() | 4-22 | Rotate within selected byte/word/longword width; distinct from shifts. |
| `amiga.base4.round` | ROUND() | 4-14 | ROUND(x) and ROUND(x,n) are documented; negative n rounds before decimal point. |
| `amiga.base4.shift` | SHL() / SHR() | 4-21–4-22 | Width-sensitive byte/word/longword shift forms are documented. |
| `amiga.base4.sqr` | SQR() | 4-8 | Square root; negative argument is documented to produce an error. |
| `amiga.base4.string-space` | STRING$() / SPACE$() | 4-32 | String construction functions; SPC is excluded as PRINT formatting syntax rather than a value function. |
| `amiga.base4.swap` | SWAP() | 4-25 | Function swaps upper/lower 16-bit words; explicitly distinct from Chapter 2 SWAP command. |
| `amiga.base4.trigonometric` | SIN() / COS() / TAN() | 4-5 | Trigonometric functions use radians in the base manual. |
| `amiga.base4.upper` | UPPER$() | 4-35 | Uppercase conversion function documented. |
| `amiga.base5.form-input` | FORM INPUT / FORM INPUT AS | 5-6 | Edits a string field of width 1–255; FORM INPUT AS starts from the current value. 3.5 availability is unresolved. |
| `amiga.base5.inkey` | INKEY$ | 5-2 | Reads keyboard input; special keys use a CSI sequence. 3.0 source fact only; 3.5 availability is unresolved. |
| `amiga.base5.input` | INPUT | 5-3–5-4 | Reads numeric/string variables with optional prompt; window behavior and numeric re-entry are described. 3.5 availability is unresolved. |
| `amiga.base5.line-input` | LINE INPUT | 5-5 | Reads string input while allowing commas in the entered text. 3.5 availability is unresolved. |
| `amiga.base5.print-using` | PRINT USING | 5-9 | Formatted screen output; exact placeholder grammar remains unreviewed. 3.5 availability is unresolved. |
| `amiga.base5.print-write` | PRINT, PRINT AT, WRITE, LOCATE | 5-7–5-8 | Screen output, cursor positioning and comma-separated WRITE format are distinguished. 3.5 availability is unresolved. |
| `amiga.base6.file-input` | INPUT # / LINE INPUT # | 6-15 | Reads channel data as values or lines; keep channel forms distinct from screen input. 3.5 availability is unresolved. |
| `amiga.base6.file-output` | PRINT # / WRITE # | 6-16 | Writes expressions through a data channel; PRINT # supports USING but not PRINT AT. 3.5 availability is unresolved. |
| `amiga.base6.inp-out` | INP / OUT | 6-13 | INP reads one byte and OUT writes byte values through an opened channel; documented channel range is 0–99. 3.5 availability is unresolved. |
| `amiga.base6.input-string` | INPUT$ | 6-14 | Reads a specified character count from keyboard or an optional file channel. 3.5 availability is unresolved. |
| `amiga.base7.do-loop` | DO / LOOP variants | 7-17–7-19 | Beginning- and end-tested WHILE/UNTIL forms are described; exact grammar remains page-sensitive. 3.5 availability is unresolved. |
| `amiga.base7.elseif` | ELSE IF / ELSEIF | 7-6–7-7 | Chained conditional branches; editor spelling varies and needs lexer/editor confirmation. 3.5 availability is unresolved. |
| `amiga.base7.exit-if` | EXIT IF | 7-20 | Exits the active loop when its Boolean condition is satisfied; allowed within IF and SELECT structures per the source. 3.5 availability is unresolved. |
| `amiga.base7.for-next` | FOR / NEXT | 7-13–7-14 | Loop forms include DOWNTO and editor replacement ENDFOR; OCR-sensitive termination details remain open. 3.5 availability is unresolved. |
| `amiga.base7.if` | IF / ELSE / ENDIF | 7-4–7-5 | Conditional execution; THEN is optional and any nonzero numeric expression is logically true. 3.5 availability is unresolved. |
| `amiga.base7.on-error` | ON ERROR / ON ERROR GOSUB | 7-29 | The 3.0 manual documents reset-to-default handling and procedure-based error handling. The 3.5 addendum confirms that GURU Meditation may transfer to ONERROR GOSUB, but does not restate the complete syntax or general 3.5 availability. |
| `amiga.base7.ongosub` | ON GOSUB | 7-8 | Selects a procedure by numeric index; out-of-range selection calls none. 3.5 availability is unresolved. |
| `amiga.base7.repeat-until` | REPEAT / UNTIL | 7-15 | Post-tested loop; body executes at least once. 3.5 availability is unresolved. |
| `amiga.base7.resume` | RESUME / RESUME NEXT / RESUME label | 7-29 | The 3.0 manual documents retry, continue-next and continue-at-label behavior, plus a FATAL-error restriction. The 3.5 addendum specifically disallows RESUME NEXT and RESUME LABEL after a GURU-triggered ONERROR GOSUB in compiled 3.5 programs; broader 3.5 behavior remains unverified. |
| `amiga.base7.select-case` | SELECT / CASE / DEFAULT / ENDSELECT | 7-9–7-10 | Numeric or short-string selector; CASE supports values, ranges and lists. 3.5 availability is unresolved. |
| `amiga.base7.while-wend` | WHILE / WEND | 7-16 | Pre-tested loop; condition is checked before each body execution. 3.5 availability is unresolved. |
| `amiga.base8.box` | BOX / PBOX | 8-15 | Draws outlined or filled rectangles. 3.5 availability is unresolved. |
| `amiga.base8.circle-ellipse` | CIRCLE / PCIRCLE / ELLIPSE / PELLIPSE | 8-16 | Draws outlined or filled circles and ellipses. 3.5 availability is unresolved. |
| `amiga.base8.clip` | CLIP | 8-9 | Restricts graphics output to a rectangle; source explicitly excludes GET and PUT. 3.5 availability is unresolved. |
| `amiga.base8.deffill` | DEFFILL | 8-3–8-4 | Configures fill color/style/pattern for filled graphics operations; pattern-string limits are described. 3.5 availability is unresolved. |
| `amiga.base8.plot-line-draw` | PLOT / LINE / DRAW | 8-10–8-14 | Point, line and path drawing; DRAW inquiry forms and OCR-sensitive command strings need exact grammar review. 3.5 availability is unresolved. |
| `amiga.base8.point` | POINT | 8-18 | Returns the color value at a screen coordinate. 3.5 availability is unresolved. |
| `amiga.base8.poly` | POLYLINE / POLYFILL | 8-17 | Uses coordinate arrays; polygon endpoints are connected automatically. 3.5 availability is unresolved. |
| `amiga.base8.setcolor` | SETCOLOR / COLOR | 8-2 | Sets palette RGB components and selects foreground/background/fill-outline colors. 3.5 availability is unresolved. |
| `amiga.base9.fileselect` | FILESELECT | 9-37–9-38 | Displays a file selector and returns the selected name or an empty string on Cancel. 3.5 availability is unresolved. |

## Explicitly confirmed by the 3.5 addendum

| ID | Declaration family | Printed pages | Qualification |
|---|---|---|---|
| `amiga.35.combinatorics` | FACT, VARIAT, COMBIN | 13-33–13-34 (469-page scan), 14-1–14-2 (combined update scan) | Factorial, permutations and combinations are explicitly enumerated as additional 3.5 functions. |
| `amiga.35.data-pointer` | _DATA and _DATA= | 13-34–13-35 (469-page scan), 14-5 (combined update scan) | The leading underscore is confirmed by page-image review; query and restore the DATA pointer. |
| `amiga.35.editor-update` | Editor row layout, search and listing changes | 1-25 | Explicit editor update; this is editor behavior, not a language/runtime declaration. |
| `amiga.35.guru` | GURU statement and function | 13-35–13-36 (469-page scan), 14-6–14-7 (combined update scan) | GURU checking control and GURU-number function are explicitly documented; crash-recovery tests require isolation. |
| `amiga.35.mat-analysis` | MAT NORM, MAT DET, MAT QDET, MAT RANG, MAT INV | 13-31–13-39 | The family is documented in Chapter 13. MAT INV destination-first order is established by the detailed definition and worked example on printed p. 13-30; the reversed summary form on p. 13-16 is treated as a source typo. Numerical edge cases remain unverified. |
| `amiga.35.mat-arithmetic` | MAT ADD, MAT SUB, MAT MUL | 13-22–13-30 | Element-wise, scalar, matrix-product and vector-product forms are documented. The 3.5 addendum explicitly defines MAT MUL array(),scalar as in-place multiplication of every field element by the scalar; this is source-verified syntax, not a runtime-conformance claim. |
| `amiga.35.mat-base` | MAT BASE | 13-1–13-2 | Explicitly documented in the version-specific Chapter 13 addendum; applicability is limited to the documented matrix operations. |
| `amiga.35.mat-copy` | MAT CPY, MAT XCPY, MAT TRANS | 13-8–13-20 | Detailed source-checked shape, offset, extent, transpose and vector-orientation rules are recorded in the addendum inventory. |
| `amiga.35.mat-generation` | MAT CLR, MAT SET, MAT ONE | 13-2–13-3 | Generation forms and floating-point field constraints are documented in Chapter 13. |
| `amiga.35.mat-io` | MAT READ, MAT PRINT, MAT INPUT | 13-3–13-6 | Chapter 13 explicitly documents vector/matrix data I/O forms. |
| `amiga.35.openw-negative-screen` | OPENW negative screen-number variant | 13-34 (469-page scan), 14-3 (combined update scan) | Explicit variant opens a window on the current top screen. |
| `amiga.35.pcolor` | PCOLOR | 13-34 (469-page scan), 14-4 (combined update scan) | Applies foreground and optional background colors to PRINT screen output. |
| `amiga.35.special-longwords` | _0 through _9 | 13-35 (469-page scan), 14-6 (combined update scan) | Ten special longword variables; persistence and clearing behavior are described in the addendum. |

## Contradicted candidates

No candidates are currently classified as contradicted. This is not evidence that all remaining candidates are compatible.

## Interpretation and limitations

- `documented-in-3.0-only` closes the source audit: the cited evidence is from the 3.0 base manual, and neither the 3.5 addendum nor a reproducible runtime observation establishes 3.5 applicability. This does not mean removed, incompatible, or confirmed in 3.5.
- `unresolved` is reserved for candidates whose source scope or declaration identity still needs research.
- A confirmed family does not settle every overload or edge case; qualifications in the manifest and detailed research notes remain authoritative.
- Chapter 12 indexes and reference tables are navigation aids, not declarations or proof of availability.
- No absence-based removal or compatibility inference is made.

Source: `AM-INTERPRETER-35` in `docs/reference-sources.md`; detailed page notes are in `amiga-gfa-basic-3.0-base-inventory.md` and `amiga-gfa-basic-35-addendum-inventory.md`.
