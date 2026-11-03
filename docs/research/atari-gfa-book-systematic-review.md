# Systematic review of *Das große GFA-BASIC-Buch* (Atari ST)

- Source ID: `ATARI-GFA-BOOK-1988`
- Source: Uwe Litzkendorf, *Das große GFA-BASIC-Buch*, 6th revised and expanded edition, DATA BECKER, ISBN 3-89011-222-6.
- Scope stated by the book: Atari ST GFA BASIC V2.xx and V3.0.
- Inspection started: 2026-11-03.
- Evidence: searchable OCR extracted from the user-provided 300-dpi PDF; printed page numbers are retained below.
- Status: **systematic review in progress**. This is not a complete transcription or a validated implementation specification.

## 1. Method and evidence rules

The book is being reviewed in its printed order, using its table of contents as the chapter checklist. Findings are recorded in original wording and linked to printed page numbers. OCR is used to locate and summarize passages, but OCR-sensitive operator glyphs, command signatures, tables, and code must be checked against the page image before being treated as exact syntax.

Evidence labels:

- **Book-backed** — the statement is legible in the extracted passage and tied to a printed page.
- **OCR-sensitive** — the relevant passage exists, but one or more glyphs, symbols, or table cells may have been misrecognized.
- **Example-only** — a construct appears in an accompanying listing, but this does not establish complete syntax or semantics.
- **Unverified** — requires page-image checking, comparison with another edition, or a version-pinned interpreter test.

The book's own preface says it organizes roughly 450 commands and functions available in the V3.0 interpreter. It also says many procedures/examples on the accompanying diskette were written with V2.0/V2.02 in mind to preserve compatibility with the V2.xx compiler. Therefore, the edition is mixed-version evidence and each feature must retain its version label.

## 2. Full chapter/page map

Printed page ranges below follow the book's table of contents. This map is a review checklist, not a claim that every page has already been analyzed.

| Chapter | Printed pages | Subject | Review status |
|---|---:|---|---|
| 1 | 15–19 | How to read the reference; syntax notation; diskette instructions | Reviewed |
| 2 | 21–22 | Atari ST hardware background | Context reviewed |
| 3 | 23–25 | GFA BASIC overview and V3.0 changes | Reviewed |
| 4 | 27–60 | BASIC foundations, names, variables, loops, comparisons, precedence | Reviewed |
| 5 | 61–154 | Input/output, screen, disk, files, random access, ports, printer, sound | Initial pass; detailed command-by-command extraction pending |
| 6 | 155–220 | Program structure, loops, branches, declarations, subprograms, external code | Reviewed |
| 7 | 221–228 | String manipulation, analysis, formatting | Reviewed |
| 8 | 229–250 | Operators, arithmetic, numeric/trigonometric functions, comparisons, bit operations, random values | Reviewed |
| 9 | 251–328 | Graphics definitions, object/line/pixel graphics, Line-A, raster operations, screen memory | Pending |
| 10 | 329–340 | Data conversion and number systems | Pending |
| 11 | 341–392 | Arrays, memory operations/management, pointers | Pending |
| 12 | 393–452 | Program control, errors, time, keyboard, multitasking, debugging | Pending |
| 13 | 453–470 | Program/user interaction | Pending |
| 14 | 471–486 | BASIC window commands | Pending |
| 15 | 487–490 | Menu programming | Pending |
| 16 | 491–498 | Event monitoring | Pending |
| 17 | 499–530 | GDOS/VDI library and fonts | Pending |
| 18 | 531–584 | AES library and managers | Pending |
| 19 | 585–596 | GEM object variables and structures | Pending |
| 20 | 597–604 | GEM addresses and fields | Pending |
| 21 | 605–656 | BIOS, GEMDOS, GEMSYS, VDISYS, XBIOS and low-level examples | Pending |
| 22 | 657–664 | System variables | Pending |
| 23 | 665–700 | RAM card-index application example | Example review pending |
| 24 | 701–793 | Compiler, interpreter, variables/types, editor, quick lists, errors and command index | Pending |

## 3. General source and editor model

### 3.1 Source layout

The book describes GFA BASIC as a structured language in which each source line contains one command. The interpreter/editor automatically formats and indents nested structures, making matching block terminators visible. This is a language/editor behavior claim for the Atari ST releases discussed by the book, not a universal claim about every GFA BASIC port. [pp. 23–24]

The diskette instructions say examples are stored as `.LST` files and are loaded through the editor's `MERGE` command. Some examples require additional fragments to be merged at specified positions. Comments were removed from diskette listings to save space. This is packaging/editor workflow, not a lexical rule. [p. 19; diskette README]

### 3.2 Command names, identifiers, labels

The book states that from V2.02 command names can also be used as variable names, except for reserved variables such as `TIMER`, `GINTIN`, and `DATE$`. Procedures and functions are identified by their declarations; labels are standalone names followed by a colon. Procedure calls are shown with `GOSUB` or `@`, while function calls use `FN` or `@`. [p. 52]

The described identifier character set is letters, digits, underscore, and period. Variable and function names must start with a letter; procedure names may start with a digit. [p. 53]

The book describes suffixes for integer (`%`), Boolean (`!`), and string (`$`) variables; unsuffixed names are treated as real variables in the variable-organization appendix. The complete V3.0 type model, including BYTE and WORD, is described later in §6.4 and Appendix 24.2. Do not collapse the V2.xx and V3.0 type inventories. [pp. 52, 716–721]

**Unverified:** exact maximum identifier length, case sensitivity, Unicode/Atari character-set treatment, name collision rules, and complete reserved-variable list require extraction from the appendix and/or interpreter tests.

## 4. Control flow and declarations (chapter 6)

### 4.1 Loop families

The book explicitly identifies four loop forms. [pp. 54–57, 155–164]

| Construct | Entry/exit model described | Evidence |
|---|---|---|
| `REPEAT … UNTIL condition` | Condition is tested at the end; body executes at least once. | Book-backed, pp. 54–55 |
| `WHILE condition … WEND` | Condition is tested before the body; body may execute zero times. | Book-backed, p. 55 |
| `FOR variable = start TO end [STEP step] … NEXT variable` | Counter-controlled loop; full variants still need command-entry extraction. | Basic model book-backed, p. 55 |
| `DO … LOOP` | Unconditional loop; use an explicit exit such as `EXIT IF` or an external break mechanism. | Book-backed, pp. 56–57 |

Loops may be nested, but their closing constructs must match in reverse nesting order. The book explicitly warns against crossing terminators (for example, closing an outer `FOR` before an inner `FOR`). [p. 56]

The book says `GOTO` is not recognized as a structural loop element; structured loop forms should be preferred. [p. 57]

### 4.2 Conditional branches

The chapter covers `IF … THEN`, nested `IF`, `ELSE`, `ELSE IF`, `ENDIF`, and V3.0 `SELECT … CASE … ENDSELECT`. The latter is an expression-based selection construct, and the book compares it with C's switch/case. [pp. 159–175]

For string selection, the book limits the string specified in a `CASE` clause to at most four characters and describes conversion of the selected string into a value for comparison. This unusual rule must be checked against the page image before being used as a parser requirement. [p. 173; OCR-sensitive]

A `CONT` option immediately before a `CASE` can cause the following case branch to be executed in addition to the already selected branch. This is a V3.0-specific behavior described by the book and needs a focused conformance test. [p. 175]

### 4.3 Variable declarations

The declaration section describes prefix-based type declarations and says an explicitly suffixed variable overrides the general declaration. The V3.0 additions include declarations for Boolean, byte, word, integer, floating-point, and string variables; exact names and declaration syntax are being kept in the main language reference pending table-by-table OCR review. [pp. 189–192]

### 4.4 Procedures and functions

The book documents `DEFFN`/ `FN` user functions, with `@` as a shorthand call form. It distinguishes single-line `DEFFN` functions from V3.0 multiline `FUNCTION` blocks. [pp. 192–194, 198–201]

V3.0 permits `VAR` parameters in procedure/function headers. These provide direct variable passing: a formal parameter declared after `VAR` aliases the caller's variable, so writes inside the routine affect the caller's variable. The book says VAR parameters must be placed at the end of the parameter list. [pp. 201, 207]

Procedures use `PROCEDURE … RETURN`; V3.0 accepts `SUB` as an alternative to `PROCEDURE`, and `ENDPROC`/`ENDSUB` as alternatives to `RETURN`. [pp. 205–207]

The book also describes local variables, recursion, nested procedure calls, and function parameters. Exact scope/lifetime rules and all parameter coercion cases still need dedicated extraction and tests. [pp. 193–203]

## 5. String operations (chapter 7)

### 5.1 String mutation and extraction

- `MID$(target$, start [, count]) = text` replaces part of an existing string variable. The target string's original length remains unchanged; insertion is limited to the space available from the start position. [p. 221]
- `LSET` and `RSET` place text at the left or right of a fixed-length target string, padding or truncating as required. The book presents these mainly for fixed-width/random-access records. [p. 221]
- `LEFT$(s [, count])`, `RIGHT$(s [, count])`, and `MID$(s, start [, count])` return portions of a string. With no count, LEFT$/RIGHT$ return one character; MID$ returns the remainder from the start position. [pp. 223–224]
- `LEN(s)` returns the string length. [p. 223]

### 5.2 String search and transformation

- `INSTR(target$, search$ [, start])` returns the one-based position, or zero when no match exists. The book says matching is case-sensitive. For two empty strings, it reports 1. [p. 222]
- `RINSTR` searches from the end of the target string. [p. 226; identified as V3.0]
- `PRED` and `SUCC` return the preceding/following ASCII character relative to the first character of the input. [pp. 224, 226; V3.0]
- `SPACE$(count)` creates a string of spaces; `STRING$(count, text)` repeats a string, or repeats a character specified by its ASCII value. [p. 227]
- `TRIM$(s)` removes leading and trailing spaces; an empty or all-space input produces an empty string. [p. 227]
- `UPPER$(s)` converts lowercase letters to uppercase. Its exact Atari character-set behavior for umlauts and non-ASCII bytes needs verification. [pp. 227–228]

The book states a maximum string-variable length of 32,767 characters in its descriptions of `STRING$` and related operations. Treat this as a book-specific runtime limit pending confirmation against the exact interpreter build. [pp. 227–228]

## 6. Operators and numeric semantics (chapter 8)

### 6.1 Operator families

The operator section lists arithmetic, comparison, and logical operators. [p. 229]

- Arithmetic: exponentiation, unary negation, multiplication/division, integer division (`DIV`), modulo (`MOD`), addition/subtraction.
- Comparisons: equality (`=`), inequality (`<>` or `><`), greater/less than, and inclusive comparisons with both orderings shown (`>=`/`=>`, `<=`/`=<`).
- Approximate equality is also listed; the OCR text gives a bit-width note that is not reliable enough to publish as a fixed rule without checking the page image.
- Logical: `AND`, `OR`, `NOT`, `XOR`, `IMP`, and `EQV`.

The book states that comparison expressions yield 0 for false and −1 for true, and that string comparisons are lexicographic by character code. If one string is a prefix of another, the longer string sorts later. [pp. 57–59]

### 6.2 Precedence

The book's precedence list, from highest to lowest, is described as: parentheses; exponentiation; unary negation; multiplication/division; integer division/modulo; addition/subtraction; comparisons; logical operators. Operators at the same precedence level are evaluated left-to-right, and parentheses can override the default order. [pp. 59–60, 229]

The relative precedence among the individual logical operators is not sufficiently clear in the OCR table; do not encode an assumed order without page-image inspection or interpreter tests.

### 6.3 Numeric and bit operations

The chapter documents integer arithmetic statements/functions such as `ADD`, `SUB`, `DEC`, `DIV`, and `MOD`, numeric functions, trigonometric functions, comparison helpers, bitwise Boolean functions, individual bit operations, shifts/rotates, and random-number generation. [pp. 230–250]

V3.0-specific entries include bit operations such as `BCHG`, `BTST`, `SHL`, `SHR`, `ROL`, and `ROR`, with byte/word/long variants described in the chapter. The exact width suffixes and signedness rules need careful page-image checking. [pp. 241–249]

The book describes `RANDOM(n)` as returning a 32-bit integer in the half-open range from zero up to (but excluding) n, including support for negative n; `RANDOMIZE` initializes the Atari ST random generator and can accept a start value. [p. 250]

## 7. Initial I/O chapter findings (chapter 5)

This chapter is much larger than a single language feature group. The contents divide it into data input/output, screen operations, disk operations, file handling and random-access files, port I/O, printer statements, and sound-chip statements. [contents; chapter starts p. 61]

The first detailed pass confirms:
- `INPUT` and `LINE INPUT` are described separately; the latter has keyboard and file-channel forms. The book warns that file `LINE INPUT#` is reliable for ASCII text with CR/LF line endings and has a 254-character line limit. [pp. 61–81]
- `PRINT` supports screen positioning via `AT(x,y)`, semicolon concatenation without automatic CR/LF, comma-based tab zones, apostrophe spacing, and output to a file channel. [pp. 82–85]
- `PRINT USING` supports numeric and string format patterns; the detailed pattern grammar is on pp. 88–90 and needs a separate command-entry pass.
- VT52 escape-sequence examples and cursor/screen controls are catalogued on pp. 85–88. These are terminal-control strings, not core parser syntax.
- `WRITE` is described as a structured output form intended particularly for sequential-file storage. [p. 90]
- Disk and file commands begin later in the chapter; their signatures and semantics have not yet been exhaustively extracted.

## 8. Corrections and questions to preserve

1. The book mixes V2.xx and V3.0 descriptions. A feature marked V3.0 must not be assumed available in V2.xx.
2. The author says many procedures were deliberately written for V2.0/V2.02 compatibility; examples do not imply that the feature set is limited to those releases.
3. The book's page 229 operator table contains OCR-sensitive symbols and a questionable approximate-comparison bit-width note. Check the image before using these exact details.
4. The book's string SELECT rule (maximum four characters in CASE) is unusual and needs page-image and runtime verification.
5. The book describes a V3.0 `CONT` behavior in SELECT/CASE that should receive an isolated test.
6. Atari-specific APIs in chapters 9 and 12–22 remain outside the language-core specification. They belong in the Atari API profile and must not be mapped to Amiga or Linux APIs by name alone.

## 9. Next review batches

1. Finish chapter 5 command-by-command: keyboard/screen I/O, disk and file commands, random-access records, port I/O, printer, sound.
2. Review chapters 9–11: graphics, data conversion, arrays, memory, pointers.
3. Review chapters 12–16: runtime control, error handling, user interaction, windows, menus, events.
4. Review chapters 17–22: GDOS/VDI, AES, object structures, GEM memory layout, BIOS/GEMDOS/XBIOS, system variables.
5. Review appendices 24.2–24.14: variable representation, editor/compiler differences, error catalogue, syntax list, alphabetical command list.
6. For every promoted rule, add an independently authored positive/negative conformance test to issue #42 and retain source/page/version metadata.
