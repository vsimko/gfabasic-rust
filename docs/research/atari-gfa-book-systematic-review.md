# Systematic review of *Das große GFA-BASIC-Buch* (Atari ST)

- Source ID: `ATARI-GFA-BOOK-1988`
- Source: Uwe Litzkendorf, *Das große GFA-BASIC-Buch*, 6th revised and expanded edition, DATA BECKER, ISBN 3-89011-222-6.
- Scope stated by the book: Atari ST GFA BASIC V2.xx and V3.0.
- Inspection started: 2026-11-03.
- Evidence: searchable OCR extracted from the user-provided 300-dpi PDF; printed page numbers are retained below.
- Status: **first-pass review of all 24 chapters completed**. This is not a complete transcription or a validated implementation specification; targeted page-image, cross-source and runtime validation remain open.

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
| 5 | 61–154 | Input/output, screen, disk, files, random access, ports, printer, sound | Detailed first pass reviewed; page-image checks and runtime tests pending |
| 6 | 155–220 | Program structure, loops, branches, declarations, subprograms, external code | Reviewed |
| 7 | 221–228 | String manipulation, analysis, formatting | Reviewed |
| 8 | 229–250 | Operators, arithmetic, numeric/trigonometric functions, comparisons, bit operations, random values | Reviewed |
| 9 | 251–328 | Graphics definitions, object/line/pixel graphics, Line-A, raster operations, screen memory | Reviewed (first pass) |
| 10 | 329–340 | Data conversion and number systems | Reviewed (first pass) |
| 11 | 341–392 | Arrays, memory operations/management, pointers | Reviewed (first pass) |
| 12 | 393–452 | Program control, errors, time, keyboard, multitasking, debugging | First pass reviewed; command and compiler-version validation pending |
| 13 | 453–470 | Program/user interaction | First pass reviewed; AES details and examples need validation |
| 14 | 471–486 | BASIC window commands | First pass reviewed; WINDTAB offsets and GEM behavior need validation |
| 15 | 487–490 | Menu programming | First pass reviewed; menu-array edge cases need validation |
| 16 | 491–498 | Event monitoring | First pass reviewed; event IDs and buffer lifecycle need validation |
| 17 | 499–530 | GDOS/VDI library and fonts | First pass reviewed; GDOS availability, workstation lifecycle and custom-font hazards need validation |
| 18 | 531–584 | AES library and managers | First pass reviewed; function signatures and GEM/TOS variants need validation |
| 19 | 585–596 | GEM object variables and structures | First pass reviewed; structure offsets and flag bits need page-image checks |
| 20 | 597–604 | GEM addresses and fields | First pass reviewed; shared-buffer layouts and OCR need validation |
| 21 | 605–656 | BIOS, GEMDOS, VDISYS, XBIOS and low-level examples | First pass reviewed; OS-call signatures and hardware-specific behavior need validation |
| 22 | 657–664 | System variables | First pass reviewed; absolute addresses are TOS/version-specific and unsafe |
| 23 | 665–700 | RAM card-index application example | First pass reviewed; example-only implementation and known sort defect recorded |
| 24 | 701–793 | Compiler, interpreter, variables/types, editor, quick lists, errors and command index | First pass reviewed; compiler differences and OCR-sensitive appendices need validation |

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

1. Complete chapter 5 page-image checks and target-interpreter validation for OCR-sensitive signatures, file-pointer semantics, and hardware-specific APIs.
2. Review chapters 12–16: runtime control, error handling, user interaction, windows, menus, and events.
3. Review chapters 17–22: GDOS/VDI, AES, object structures, GEM memory layout, BIOS/GEMDOS/XBIOS, and system variables.
4. Review appendices 24.2–24.14: variable representation, editor/compiler differences, error catalogue, syntax list, and alphabetical command list.
5. Promote verified rules into independently authored positive/negative conformance cases tracked by issue #42, retaining source/page/version metadata.
6. Keep chapters 9–11 findings in this research note; promote exact signatures only after page-image checks.

## 10. Graphics, conversion, arrays and memory (chapters 9–11)

This review batch covers the book's Atari-specific graphics chapter, data-conversion chapter, and array/memory chapter. These facilities are not all language-core features: many depend directly on Atari ST screen modes, VDI, Line-A, TOS memory layout, or hardware.

### 10.1 Graphics definitions and state (chapter 9, pp. 251–270)

The book separates graphics state/configuration from drawing primitives. The first group includes:

- `BOUNDARY`: enables/disables the outline around P-graphics objects. [p. 251; V3.0]
- `DEFFILL`: configures fill color, style, and pattern, including user-defined patterns. [pp. 254–256]
- `DEFLINE`: configures line style, width, and line-end form for line-drawing operations. [pp. 257–258]
- `DEFMARK`: configures marker type and size. [p. 258]
- `DEFMOUSE`: selects built-in or user-defined mouse forms; the manual describes the data layout of a 16×16 mouse cursor. [pp. 259–262]
- `DEFTEXT`: configures text color, style, rotation, size, and face. [pp. 262–264]
- `GRAPHMODE`: selects raster drawing mode; the book explains replace, transparent, XOR and related logical drawing modes. [pp. 265–266]
- `SETCOLOR` and V3.0 `VSETCOLOR`: set Atari color registers; VSETCOLOR addresses VDI-indexed color registers. [pp. 267–270]

The color-register descriptions are resolution-dependent (Hires/Midres/Lowres) and must remain part of the Atari graphics profile. They must not be represented as a platform-neutral color model. [pp. 253, 267–270]

### 10.2 Drawing primitives and raster operations (chapter 9, pp. 271–328)

The chapter's object-graphics group includes rectangle, circle/ellipse, arc, polygon, marker, and text operations. Examples and command entries include `BOX`/`PBOX`, `PCIRCLE`, `POLYLINE`, `POLYFILL`, `POLYMARK`, `DRAW`, and `LINE`. Polygon operations use point-count and coordinate arrays; some forms support an offset. [pp. 271–287]

The later part of the chapter documents Line-A operations such as `ALINE` and `PSET`, clipping, graphics-origin offsets, and raster-copy/combination operations. It also discusses screen-plane layout, screen-memory organization, image strings, `GET`/`PUT`, `RC_COPY`, `RC_INTERSECT`, sprites and vertical synchronization. [pp. 288–328]

Important architectural distinction: VDI raster operations, Line-A operations, P-graphics objects, screen-memory access, and helper functions such as RC_COPY/RC_INTERSECT have different dependencies and semantics. They should be separate API families in the Atari profile, not one generic graphics abstraction. Exact signatures and supported-version labels still need command-by-command extraction.

### 10.3 Data conversion (chapter 10, pp. 329–340)

The chapter covers conversion between numeric representations and binary/string encodings. The command families visible in the chapter include:

- ASCII/character conversion.
- Binary, hexadecimal, decimal and octal representations.
- Integer/long and floating-point serialization to strings, including `MKI$`, `MKL$`, `MKS$`, `MKF$`, and `MKD$`.
- Reverse conversion functions, including `CVI$`, `CVL$`, `CVS$`, `CVF$`, and `CVD$`.

The book notes the `&O` prefix for octal numeric literals and documents an optional width/number-of-digits argument for V3.0 formatting. [p. 337] The exact byte order, storage widths, floating-point encoding, overflow, and malformed-input behavior must be recorded per function after the syntax tables and examples are checked. Do not infer a portable serialization format from these names.

### 10.4 Arrays (chapter 11.1, pp. 341–353)

The chapter covers array initialization and manipulation. Entries include:

- `ARRAYFILL`: fills an array with a value matching the array's element type. [p. 341]
- `DELETE`: removes one element and shifts subsequent elements. [p. 341–348]
- `ERASE`: removes an array's dimensions and releases its allocated storage; V3.0 supports deleting multiple arrays in one statement. [p. 348]
- `INSERT`: inserts an element at a specified index. [p. 348]
- `QSORT`: V3.0 quicksort for numeric or string arrays, with ascending/descending selection, optional element count and optional parallel index array. String sorting can use a supplied collation/order table. [pp. 350–352]
- `SSORT`: V3.0 Shellsort counterpart; the book describes it as using less memory but generally being slower than QSORT. [p. 353]

The array examples and sorting descriptions make `OPTION BASE` important: index ranges in examples and optional counts depend on whether the lower bound is zero or one. The exact default and scope of OPTION BASE must be checked in the relevant command entry before compiler implementation.

### 10.5 Absolute variables, raster blocks and memory access (chapter 11.2, pp. 354–380)

The chapter describes `ABSOLUTE` as binding a BASIC variable to a specified memory address. The address-bound variable can be used to inspect or modify system-managed memory, but writes to incorrect or protected locations can crash the system. The book warns that addresses in the supervisor area below 2048 cannot be accessed as ordinary user data. [pp. 355–357]

`BITBLT` is documented in more than one form: VDI raster-copy parameters and Line-A forms available only in V3.0. The VDI form uses source and destination raster descriptors plus a parameter block; the Line-A form uses a memory block/parameter array whose fields are documented in a table. [pp. 358–361] These are distinct low-level APIs.

`BMOVE` copies a specified number of bytes from source to destination and permits overlapping source/destination ranges. The manual warns that a zero byte count can cause a crash and that addresses must not be in the supervisor area; even alignment affects speed. [p. 362]

The book also documents direct memory reads/writes and typed access. In V3.0, `BYTE{address}`, `CARD{address}`, and `LONG{address}` read/write 1, 2, and 4 bytes respectively; CARD/LONG accesses require even addresses. The manual says these typed accessors use user mode and cannot access supervisor memory. A four-byte access may omit LONG in the syntax. [p. 373; V3.0]

### 10.6 Memory management and pointers (chapter 11.3–11.4, pp. 381–392)

- `INLINE address%, bytes`: reserves an embedded memory region within the BASIC listing; the editor reserves it when the line is entered, and the reserved content is saved/loaded with the program. The book gives a maximum region size of 32,700 bytes. [pp. 381–382; V3.0]
- `MALLOC(bytes)`: allocates system memory above HIMEM and returns its start address, or zero on failure; `MALLOC(-1)` reports remaining allocatable space. [pp. 383–387; V3.0]
- `MSHRINK` and `MFREE`: reduce or release allocated memory. Correct release order matters; a leaked allocation can make later memory unavailable to BASIC. [pp. 383–387]
- `RESERVE`: adjusts the BASIC-managed memory region. V2.xx and V3.0 differ; V3.0 permits a negative size to reduce the upper end and allows a no-argument form to restore the original configuration where possible. [pp. 388–389]
- `VARPTR(variable)`: returns the address of a numeric variable or the address of the first character of a string; V3.0 permits the `V:` shorthand. [p. 392]

These operations expose a version-specific memory model and should not be emulated as ordinary Rust references. They require an explicit virtual-memory/address-space design if the compiler/runtime is to support them safely.

## 11. Updated review priorities

1. Finish chapter 5 command-by-command, especially file handles, random-access records, disk functions, ports, printer and sound.
2. Review chapter 12 runtime control and error handling before attempting runtime semantics.
3. Review chapters 13–16 as a coherent GEM interaction/event subsystem.
4. Review chapters 17–22 as Atari platform API and data-structure documentation.
5. Review appendix 24, especially variable representation, editor/compiler differences, errors, syntax list and alphabetical command list.
6. Cross-check OCR-sensitive syntax and tables against page images; add focused tests under #42 only after the rule is sufficiently precise.


## 11. Chapter 5 detailed command inventory (pp. 61–154)

This section records the command families and the semantics visible in the book's first command-by-command pass. It is an inventory, not yet a normative parser/runtime specification. OCR-sensitive spellings and low-level Atari details remain subject to page-image verification.

### 11.1 Keyboard and text input (pp. 69–81)

| Construct | Syntax / behavior recorded | Version and evidence |
|---|---|---|
| `FORM INPUT` | `FORM INPUT length, stringVar`; formatted text input with a maximum length of 255 characters. | V3.0 notation appears in the entry; book-backed, pp. 69–70 |
| `FORM INPUT … AS` | `FORM INPUT length AS stringVar`; edit an existing string value, replacing the variable's previous content after confirmation. | Book-backed, p. 70 |
| `INKEY$` | Returns an empty string if no key is pending; ordinary keys return a one-byte string, special keys a two-byte string whose first byte is NUL and second is a pseudo scan code. | Book-backed, pp. 70–72 |
| `INPUT` | `INPUT ["prompt";|,] var1 [,var2…]`; supports multiple variables and mixed string/numeric input. Semicolon form displays a space and question mark; comma form starts input immediately after the prompt. | Book-backed, pp. 75–78 |
| `INPUT #` | `INPUT #channel, var1 [,var2…]`; reads values from an opened file or virtual device channel. | Book-backed, p. 78 |
| `INPUT$(n)` | Returns up to the requested number of characters without echoing them during entry; the channel form `INPUT$(n,#channel)` reads a fixed byte count. | Book-backed, p. 79 |
| `LINE INPUT` | `LINE INPUT ["prompt";|,] stringVar [,stringVar…]`; treats commas as text and ends each listed variable's input with Return. | Book-backed, pp. 79–80 |
| `LINE INPUT #` | Reads a line from a channel. The book warns of a 254-character limit and reliable operation only with ASCII text and CR/LF line endings. | Book-backed, p. 80; test exact CR/LF pointer behavior |

The book describes a 255-character maximum for ordinary string input and for the FORM INPUT field. It describes insertion mode as an additional V3.0 feature. INKEY$ scan-code interpretation is affected by V3.0 KEYPAD settings. [pp. 70–76]

The keyboard buffer may retain repeated keypresses. The book shows clearing its count through low-level buffer access and describes BIOS(11,-1) for modifier-key state: right shift, left shift, Control, Alternate, and CapsLock occupy bits 0–4. It also gives GEMDOS(6,255) as a way to obtain a packed keyboard value containing ASCII, hardware scan code, and modifier status. These are Atari BIOS/GEMDOS integrations, not portable keyboard semantics. [pp. 71–75]

### 11.2 Screen output and terminal control (pp. 82–94)

- `PRINT`: screen output, with `AT(x,y)` positioning; file-channel form `PRINT #channel,`; semicolon joins adjacent expressions and suppresses CR/LF when trailing; comma advances to the next of five tab stops (x = 1, 17, 33, 49, 65); apostrophe inserts a space. The book gives screen dimensions of 80×25 for Hires/Midres and 40×25 for Lowres. [pp. 82–85]
- `PRINT USING`: `PRINT USING format, expr [,expr…]` and `PRINT #channel, USING format, expr…`. The format language includes digit placeholders, decimal placement, sign handling, fill characters, currency prefix, thousands separators, exponential notation, first-character/full-string output, fixed-width substring output, and escaping of a following format character. V3.0 adds decimal-point/decimal-comma selection through MODE. [pp. 88–90]
- `WRITE`: `WRITE [#channel,] ["text" [,var,expr…]]`. Unlike PRINT, it writes quotes around strings and commas between values; the book presents it particularly for sequential-file storage and subsequent INPUT# parsing. [pp. 90–91]
- `LOCATE`: `LOCATE column,row`; described as available from V2.02 as an alternative to PRINT AT. V3.0 catches out-of-range low coordinates to avoid a Blitter-TOS fatal error. [p. 92]
- `SPC(n)`: inserts the requested number of spaces in PRINT output. [p. 93]
- VT52 escape strings: the book defines functions for cursor movement, screen/line clearing, insert/delete line, cursor position, foreground/background color, cursor visibility, save/restore cursor, reverse video, and line wrapping. These are Atari TOS terminal-control sequences rather than language-core syntax. [pp. 85–88]

### 11.3 Disk and directory operations (pp. 94–120)

The book documents these operations and related directory helpers:

| Operation | Syntax / behavior | Evidence |
|---|---|---|
| `BLOAD` | `BLOAD "filename" [,startAddress]`; loads a binary file into memory. | pp. 97–98 |
| `BSAVE` | `BSAVE "filename", startAddress, byteCount`; saves a memory region. | pp. 98–99 |
| `CHDIR` | `CHDIR "directory"`; changes the current directory. A single backslash selects the root directory. | pp. 102–103 |
| `CHDRIVE` | Drive/path selection helper; the book labels the compact path form as V3.0-specific. | p. 103; verify exact syntax from page image |
| `DFREE(drive)` | Returns free disk space; drive 0 is current, 1 is A:, and subsequent values identify further drives. | p. 103 |
| `DIR` | `DIR ["path"] [TO "file"]`; lists directory entries and can redirect output to a file or device. | pp. 103–104 |
| `DIR$(drive)` | Returns the currently selected directory name for a drive, or an empty string when at its root. | p. 104 |
| `EXIST(path)` | Tests whether a file/path exists; used as a Boolean guard before OPEN. | p. 105 |
| `FGETDTA()` / `FSETDTA(address)` | Read/set the GEMDOS Disk Transfer Address buffer. The DTA contains file attributes, date/time and size fields at documented offsets. | pp. 105–108; Atari GEMDOS-specific |
| `FILES` | `FILES ["path"] [TO "file"]`; DIR-like listing with file size, date and time metadata. | pp. 106–107 |
| `FSFIRST(path,attributes)` / `FSNEXT()` | Find first/next matching directory entry, with results placed in the DTA buffer. | pp. 108–110; Atari GEMDOS-specific |
| `KILL` | `KILL "filename"`; deletes a file. | p. 112 |
| `LIST` | `LIST ["filename"]`; displays the current program or saves an ASCII listing; default extension is .LST. | pp. 112–114 |
| `LOAD` | `LOAD "program"`; loads a BASIC program; default extension described as .BAS. | pp. 117–118 |
| `MKDIR` | `MKDIR "directory"`; creates a directory. | p. 118 |
| `NAME` / `RENAME` | `NAME "old" AS "new"`; renames or moves a file within the supported directory rules. RENAME is documented as an alias in V3.0. | pp. 119–120 |
| `PSAVE` | `PSAVE "program"`; stores a protected/tokenized program that auto-starts on load and is not editable/listable through the normal interpreter workflow. | pp. 119–120 |
| `RMDIR` | `RMDIR "directory"`; removes an empty directory. | p. 120 |
| `SAVE` | `SAVE "program"`; saves the current program in token format; the book distinguishes V2.xx .BAS and V3.0 .GFA defaults. | p. 120 |
| `CHAIN` | Example uses a temporary data file or the Atari Basepage command area to pass values to another program; the command entry and exact semantics need a focused reread. | pp. 100–102; example-led |

The book says Atari disk-operation paths are generally normalized to uppercase by the system. The DTA layout and FSFIRST/FSNEXT behavior depend on GEMDOS and must remain in the Atari API profile. [pp. 105–110, 119]

### 11.4 File channels and random-access records (pp. 121–134)

| Operation | Syntax / behavior | Evidence |
|---|---|---|
| `BGET` | `BGET [#]channel, address, count`; reads bytes from a sequential file into memory at an address. | p. 121 |
| `BPUT` | `BPUT [#]channel, address, count`; writes bytes from memory to a sequential file. | pp. 121–122 |
| `CLOSE` | `CLOSE [#channel]`; closes one channel or all open channels. | p. 122 |
| `EOF(channel)` | Tests end-of-file; the book shows it to prevent file-end errors during sequential reads. | p. 123 |
| `FIELD` | `FIELD #channel, length AS stringVar [, ...]`; partitions a random-access record into fixed-width string fields. V3.0 adds an address-based form using `AT(address)`. | pp. 123–124 |
| `GET #` | `GET #channel [,recordNumber]`; reads a record into FIELD variables. | p. 124 |
| `LOC(#channel)` | Returns the current file-pointer byte position. | p. 124 |
| `LOF(#channel)` | Returns the file length in bytes. | p. 125 |
| `OPEN` | `OPEN "mode", #channel, "filename" [,recordLength]`; modes described include Output ("O"), Input ("I"), and Random ("R"). Up to 100 channels, numbered 0–99, are described. | pp. 125–127 |
| `PUT #` | `PUT #channel [,recordNumber]`; writes FIELD variables to a random-access record. | p. 127 |
| `RECALL` | `RECALL #channel, stringArray(), count, resultVar`; reads CR/LF-terminated text records into a string array and reports the actual count read. | pp. 127–128 |
| `RELSEEK` | `RELSEEK #channel, [+|-]offset`; moves the file pointer relative to its current position. | p. 129 |
| `SEEK` | `SEEK #channel, [+|-]offset`; sets the file pointer to a byte offset. | p. 129 |
| `TOUCH` | `TOUCH #channel`; updates the file's directory timestamp from system time. | p. 130; V3.0 |
| `STORE` | `STORE #channel, stringArray() [,count]`; writes array strings as CR/LF-terminated lines. | p. 130 |

The random-access model is record-oriented: OPEN "R" establishes a record length, FIELD divides each record into byte-sized fields, and GET#/PUT# select records. The book describes a maximum record length of 65,535 bytes and up to 65,535 records. It warns that reopening an existing file with an incompatible record length or FIELD layout can make data inaccessible or corrupt record boundaries. [pp. 126–134]

V2.xx permits only one FIELD declaration per random-access channel; V3.0 relaxes this and adds an address-based FIELD form. The default record length is described as 128 bytes when omitted. These version-sensitive rules require conformance tests before implementation. [pp. 123–124, 127, 131–134]

### 11.5 Port input/output (pp. 135–145)

- `INP(port)` and `INP(#channel)` read one byte from a hardware/device port or file channel. The port identifiers listed are 0=LST/Centronics, 1=AUX/RS232, 2=CON/keyboard, 3=MIDI, 4=IKB/keyboard processor, 5=VID/monitor. The book warns that INP waits for data and may not be interruptible through the normal Break mechanism. [pp. 135–138]
- `INP?(port)` tests whether input is available; `OUT?(port)` tests whether a port is ready to accept output. [p. 138]
- `INPAUX$` and `INPMID$` are V3.0 reserved string variables containing buffered bytes received since the previous read; reading empties the corresponding buffer. [p. 138]
- `OUT`: `OUT #channel, byte` or `OUT port, byte`; V3.0 allows multiple byte values in one statement. [p. 139]
- `OUT 4,n` sends commands to the Atari intelligent keyboard processor, changing mouse, joystick, and keyboard behavior. The book documents command values 7–31, several requiring trailing parameter bytes. It explicitly warns that incorrect values can cause unpredictable or persistent hardware/input failures. [pp. 139–145]

The OUT 4 command table is hardware-specific, partly uncertain even to the book's author, and must not be implemented as a portable input API. The safe initial implementation stance is to classify it as an Atari-specific operation with explicit support status, not to map it directly to SDL input calls.

### 11.6 Printer output (pp. 145–150)

- `HARDCOPY`: prints the current screen dump; the book describes a timeout if the printer is unavailable and version-specific behavior in V3.0. [pp. 145–146]
- `LLIST`: sends the current BASIC listing to the printer; interruptibility differs across V2.0, V2.02 and V3.0. [p. 146]
- `LPOS(dummy)`: returns the virtual printer-buffer head position, not necessarily the physical print-head position. [p. 147]
- `LPRINT`: printer analogue of PRINT, with no PRINT AT variant; printer control codes are device-specific. [pp. 147–150]

The book's custom-character examples assume Epson FX-compatible printers and define 11-column glyph bitmaps. These are printer-specific examples, not a general printer abstraction. [pp. 148–150]

### 11.7 Sound chip (pp. 150–153)

- `SOUND channel, volume, note, octave, duration` or `SOUND channel, volume, #period, duration`. The period form is derived from frequency using approximately `TRUNC(125000/frequency + 0.5)`; values above 4100 are reduced modulo 4100. [pp. 150–151]
- `WAVE channelMask, envelopeMask, shape, length, duration` configures the Yamaha YM-2149 sound generator. Channel/envelope masks select tone/noise generators; shape values 8–15 select envelope forms. Duration is described in 1/50-second units. [pp. 151–153]

These commands are tightly coupled to the Atari ST YM-2149 sound chip. A Linux runtime may emulate their observable behavior, but must not assume SDL audio is equivalent without a defined YM-2149 model and timing tests.

### 11.8 Implementation and specification implications

1. Keep sequential-file byte operations (`BGET`, `BPUT`, `INP(#)`, `OUT(#)`) distinct from record-oriented random access (`FIELD`, `GET#`, `PUT#`).
2. Define the file-pointer convention explicitly; the book's examples describe byte positions starting at 1, which may differ from a host runtime's zero-based offsets. [p. 124]
3. Do not assume that Atari device names (`CON:`, `AUX:`, `MID:`, `VID:`, `LST:`, `PRN:`) are ordinary Linux files. They are virtual device channels in the Atari interpreter.
4. Keep DTA, FSFIRST/FSNEXT, BIOS/GEMDOS keyboard access, IKB commands, printer ports, and YM-2149 sound in the Atari ST API profile.
5. Preserve V2.xx/V3.0 differences for INKEY$/KEYPAD, OPEN/FIELD, TOUCH, OUT multiple-byte syntax, and directory/path conveniences.
6. Do not infer exact on-disk numeric encoding, record padding, endianness, or error behavior solely from the book's examples. Verify them against the target interpreter and binary fixtures.

### 11.9 Issue traceability and remaining validation

This review batch belongs to existing issue **#23 — Document GFA BASIC file, disk, data, and resource operations**. Atari device, BIOS/GEMDOS, printer, and sound details also feed existing issue **#25 — Create Atari ST GFA BASIC API reference**. Cross-cutting feature inventory and version labels belong to **#19**, and executable compatibility checks belong to **#42**. No duplicate issue was opened.

Remaining work for chapter 5:
- Check page images for OCR-sensitive signatures and the full CHDRIVE/CHAIN entries.
- Extract the full PRINT USING grammar into a small syntax table and verify ambiguous placeholders.
- Verify file-pointer base, OPEN modes, random record limits, and record-length behavior against an exact Atari interpreter.
- Verify the byte layouts and calling conventions for BLOAD/BSAVE, DTA, keyboard packed values, and YM-2149 timing.
- Convert only verified language/core behavior into conformance cases; keep hardware-specific operations in Atari-profile tests.


## 12. Program control, errors, time, keyboard and debugging (chapter 12, pp. 393–452)

Chapter 12 mixes language/runtime control with Atari process, memory, keyboard and GEM/TOS integration. The inventory below deliberately keeps these classes separate.

### 12.1 Program lifecycle and clearing (pp. 393–397)

| Construct | Behavior recorded | Version / evidence |
|---|---|---|
| `CONT` | Resumes after `STOP` in the line following STOP. It is unavailable after CLEAR, program-listing edits, or introducing new variables. | Book-backed, p. 393 |
| `EDIT` | Ends execution and returns directly to the editor in interpreter mode, or to the desktop for a compiled program. | Book-backed, pp. 393–394 |
| `END` | Ends the current program. In the interpreter, variables and open files remain available in direct mode until program modification or CLEAR. | Book-backed, pp. 393–394 |
| `QUIT` / `SYSTEM` | Exits to desktop or caller without a confirmation prompt. V3.0 accepts an optional 16-bit return code. | V3.0 return-code form; book-backed, pp. 394, 396 |
| `RUN` | Restarts the current program and clears variables and screen. V3.0 accepts a program name to load and run. | Named form V3.0; book-backed, pp. 394–395 |
| `STOP` | Suspends execution and presents a choice to continue or enter direct mode. Variables and open files are retained; CONT may resume if the program has not been altered. | Book-backed, p. 396 |
| `CLEAR` | Resets numeric variables to zero, strings to empty, and erases arrays/dimensions. Prohibited inside procedures and FOR/NEXT loops; executed automatically at program start. | Book-backed, p. 397 |
| `CLR var[,var…]` | Clears listed scalar variables; array variables are excluded. | Book-backed, p. 397 |
| `CLS [#channel]` | Clears the active screen/window and homes the cursor; the file-channel form writes the clear-screen control sequence into the file. | Book-backed, p. 397 |
| `NEW` | Clears the BASIC workspace, program and variables. | Book-backed, p. 397 |

`QUIT x` return-code conventions are described as 0 for normal completion, positive for BASIC/bomb errors, and negative for GEMDOS/BIOS/XBIOS errors. These are Atari process conventions, not a portable exit-code ABI. [p. 394]

### 12.2 Date, time and delay (pp. 398–404)

- `DATE$`: reserved string containing `DD.MM.YYYY`; V3.0 allows assignment to set the system date. [p. 398]
- `TIME$`: reserved string containing `hh:mm:ss`; V3.0 allows assignment to set system time. [p. 403]
- `SETTIME time$,date$`: sets date and/or time. European format is documented for V2.xx and V3.0; US date/time format is V3.0 and controlled by MODE. Invalid formats are ignored. Seconds are represented in two-second steps. [pp. 399–400]
- `DELAY seconds`: pauses for the specified number of seconds. [p. 398]
- `PAUSE ticks`: pauses in 1/50-second units; during the pause, only the Break function remains active. ON ERROR, ON MENU and AFTER/EVERY callbacks are not serviced during this wait according to the book. [pp. 398–399]
- `TIMER`: reserved counter of elapsed 1/200-second ticks since system start. The book notes it can be modified through the Atari system timer address, so it is not an immutable monotonic clock if low-level memory writes are allowed. [pp. 403–404]

The timer frequency, wraparound, and relationship to the host monotonic clock must be explicitly specified for any compatibility runtime. PAUSE/DELAY must not be treated as interchangeable with an event-pumping wait.

### 12.3 Error handling (pp. 405–409)

| Construct | Behavior recorded | Evidence |
|---|---|---|
| `ERR` | Reserved numeric variable containing the last error identifier. | p. 405 |
| `ERR$(index)` | V3.0 returns the message text for an error index, formatted for use with FORM_ALERT. | V3.0, p. 405 |
| `ERROR number` | Raises/simulates the specified error; if ON ERROR GOSUB is active, transfers control to its handler. | V3.0 spelling/short form; p. 405 |
| `FATAL` | Reserved variable distinguishes ordinary errors (0) from fatal/bomb errors (-1), according to the book's runtime description. | p. 406; reliability explicitly questioned by author |
| `ON ERROR GOSUB procedure` | Installs an error handler; normal error display is suppressed and handler must decide what to do. Bare ON ERROR restores normal error handling. | V3.0 permits omitting GOSUB; pp. 407–408 |
| `RESUME` | Continues by retrying the failing statement. | p. 408 |
| `RESUME NEXT` | Continues at the statement after the failing statement. | p. 408 |
| `RESUME label` | Continues at the specified label; after fatal errors, the book says to use this form only. | p. 408 |

The handler is automatically disabled after its invocation; it must be reinstalled before RESUME if further errors should be caught. The book also describes RESUME label as a way to jump out of a procedure, and warns that this may clear the GOSUB stack and restore globals. This unusual control transfer requires exact interpreter tests before being represented in the compiler's control-flow model. [pp. 408–409]

### 12.4 Runtime and system information (pp. 409–417)

- `BASEPAGE`: address of the Atari TOS 256-byte process basepage. The book documents offsets for TPA, text/data/BSS segments, DTA, parent process, environment and command tail. [pp. 409–410]
- `CRSCOL` / `CRSLIN`: reserved variables for the current TOS text cursor column and row. Their ranges depend on screen resolution and font. [pp. 410–411]
- `FRE(dummy)` / V3.0 `FRE()`: runs garbage collection and reports free BASIC memory; V3.0 empty parentheses avoid a collection before measurement. The book recommends forcing collection before querying variable addresses. [pp. 411–412]
- `HIMEM`: address of the first byte after memory occupied by BASIC or the compiled program. [p. 412]
- `L-A`: base address of Atari Line-A/VDI system variables. The book lists many offsets into Line-A, VDI, mouse, font and workstation state. V2.xx is said to use fixed base address `&H293A`, with TOS-version caveats. [pp. 412–416]
- `TYPE(pointer)`: returns a numeric type identifier for the variable represented by a pointer. V3.0 adds identifiers for 2-byte and 1-byte integer variables and arrays. [pp. 415–416]
- `WORK_OUT(index)`: V3.0 exposes the VDI workstation output arrays captured during initial workstation opening; the book maps indices 0–56 to screen, font, color and device capabilities. [pp. 416–417]

BASEPAGE, L-A, raw offsets and WORK_OUT are Atari/TOS/VDI ABI surfaces. Their addresses and layouts must not leak into the platform-independent language core. TYPE is language/runtime introspection, but its numeric type IDs must be pinned to the documented dialect and variable model.

### 12.5 Keyboard control (pp. 417–425)

| Construct | Behavior recorded | Version / evidence |
|---|---|---|
| `KEYDEF key,text` | Assigns up to 31 characters to F1–F10 or Shift+F1–F10; strings can be inserted in programs/editor and dialog/file-selection input. | V3.0, pp. 417–420 |
| `KEYGET var` | Waits for a key and returns a packed 32-bit value: ASCII byte, scan-code byte, and modifier-status byte. | V3.0, pp. 420–421 |
| `KEYLOOK var` | Reads the first pending key from the keyboard buffer without removing it. | V3.0, p. 421 |
| `KEYPAD vector` | Configures NumLock, cursor control, Alt-character input and KEYDEF function-key activation through six bits. | V3.0, p. 421 |
| `KEYPRESS code` | Injects a simulated key event; packed long values can include scan code and modifier state. | V3.0, pp. 422–424 |
| `KEYTEST var` | Non-blocking keyboard check; returns the same packed representation as KEYGET or zero if no key is pressed. | V3.0, p. 425 |

The book provides V2.xx DEFFN equivalents for KEYGET and KEYTEST via GEMDOS calls. The packed key representation is distinct from the one-/two-byte string returned by INKEY$. Do not collapse these interfaces into one key type without preserving scan code, ASCII, modifier and buffer-consumption semantics.

### 12.6 Timer callbacks and multitasking (pp. 425–430)

- `AFTER ticks [GOSUB] procedure`: invokes a procedure once after the specified number of ticks; one second is 200 ticks. Execution is deferred until the current BASIC statement completes. [pp. 425–426]
- `AFTER STOP` / `AFTER CONT`: suspend/resume the pending single-shot callback. [p. 427]
- `EVERY ticks [GOSUB] procedure`: invokes a procedure repeatedly at the specified tick interval; it is also deferred until the current BASIC statement completes. [pp. 428–429]
- `EVERY STOP` / `EVERY CONT`: suspend/resume recurring callbacks. [p. 429]

The book states that AFTER and EVERY share a single GEM event-timer vector, so a later AFTER/EVERY registration replaces the earlier one; they are not independent concurrent timer queues. It also demonstrates re-entrancy hazards: a recurring callback that invokes a routine which itself can be called by the same EVERY timer must suspend EVERY around that call. These are cooperative/deferred callbacks, not arbitrary preemptive multithreading. [pp. 426–429]

### 12.7 Debugging and miscellaneous runtime commands (pp. 430–452)

- `DUMP [filter$] [TO file$]`: V3.0 outputs variable contents, array dimensions, labels, procedures and functions. Filters select names by prefix or select labels/routines; output can be redirected to a file/device. [pp. 430–432]
- `TRACE$`: V3.0 reserved string containing the next source line in a TRON Proc handler; empty outside that handler. [p. 434]
- `TROFF`: disables TRON and TRON Proc. [p. 434]
- `TRON [#channel]`: traces executed source lines to screen or an opened channel. [p. 434]
- `TRON procedure`: V3.0 redirects tracing to a procedure, allowing custom inspection using TRACE$ and DUMP. Interrupt callbacks and the trace handler itself are excluded from the trace stream. [pp. 435–446]
- `DEFLIST format`: interpreter direct-mode listing format control; not a normal runtime statement. [p. 447]
- `DEFNUM digits`: controls displayed numeric precision/rounding for subsequent output without changing internal numeric precision. [pp. 447–448]
- `FALSE` and `TRUE`: reserved constants 0 and -1. [pp. 448, 450]
- `LET variable=value`: assignment form retained for compatibility; largely unnecessary from V2.02. [p. 448]
- `MODE flags`: V3.0 locale settings for date and numeric formatting. [pp. 448–449]
- `OPTION "directive"`: V2.xx compiler directives affecting Break checks, integer overflow checking, error messages and bomb-error handling. [pp. 449–450]
- `SWAP a,b`: exchanges compatible scalar, array, array-element or pointer/array descriptors; V3.0 adds SWAP() as a function and permits VAR array parameters in procedures. [pp. 451–452]
- `VOID functionCall`: invokes a function while discarding its result, avoiding a dummy assignment. [p. 452]

The large TRON Proc example is an interpreter-debugger utility built from screen-memory access, keyboard scan codes, memory copies and V3.0 tracing. It is an example, not a specification of a debugger API. OPTION is compiler-specific and the documented options are explicitly for V2.xx; do not treat them as V3.0 language statements.

## 13. Program/user interaction (chapter 13, pp. 453–470)

### 13.1 Dialogs and file selection

- `ALERT icon,text$,defaultButton,buttonText$,result`: displays a GEM alert with icon 0–3, up to four text lines of 30 characters, up to three buttons, and button labels up to eight characters. The pipe character separates lines/buttons. [pp. 453–454]
- V3.0 `FORM_ALERT(...)`: exposes the underlying GEM alert routine with fewer interpreter-side safety restrictions. The book warns that malformed or overlong strings can crash the system; its V2.xx workaround directly calls GEMSYS and is likewise unsafe. [pp. 454–455]
- `FILESELECT path$,selection$,result$`: opens the GEM file selector and returns the selected name/path. The documented return cases distinguish selection, OK with/without a name, and cancel. [pp. 459–460]
- The author demonstrates changing GEM alert icons and the FILESELECT caption by locating and modifying internal GEM RAM structures. This is undocumented, version-/TOS-sensitive memory patching and must not be considered a supported API. [pp. 455–459]

### 13.2 Mouse and joystick interaction

- `MOUSE x,y,buttons`: returns mouse coordinates and button mask (left=bit 0, right=bit 1). In V3.0, coordinates may be negative relative to a GEM window or CLIP OFFSET origin. [pp. 464–466]
- `MOUSEX`, `MOUSEY`, `MOUSEK`: separate accessors for pointer coordinates and button state. [p. 466]
- V3.0 `SETMOUSE x,y[,button]`: moves the pointer and optionally simulates a button state; AES behavior is described as limited. [p. 466]
- `STICK mode`: switches the mouse/joystick port between mouse mode (0) and joystick mode (1). The book advises restoring mouse mode before AES calls and program exit. [pp. 467–468]
- V3.0 `STICK(port)`: returns joystick direction bits (up/down/left/right). The V2.xx DEFFN equivalent reads a hardware address and also includes fire-button state in bit 7. [pp. 468–469]
- V3.0 `STRIG(port)`: returns the joystick fire-button state. [p. 470]

The Rubberbox routine is an author-supplied example that draws a resizable rectangle by saving/restoring screen fragments; it is not a built-in command. The joystick port mapping and OUT/PEEK forms are Atari hardware-specific.

## 14. Window programming (chapter 14, pp. 471–486)

The window API is primarily V3.0 in the forms documented here. It manages four application windows plus a special background/menu window 0.

| Command/function | Syntax / behavior | Version / evidence |
|---|---|---|
| `CLEARW [#]number` | Clears the specified window (0–4); V3.0. | p. 471 |
| `CLOSEW [#]number` | Closes the specified window; V3.0 form. | p. 471 |
| `FULLW [#]number` | Expands a window to the maximum area below the menu bar. | p. 472 |
| `INFOW [#]number,text$` | Defines the information line below the title bar; must be configured before OPENW. | p. 472 |
| `OPENW number[,xCenter,yCenter]` | Opens/activates one of four tiled windows around a configurable intersection point. | pp. 472–473 |
| `OPENW #number,x,y,width,height,attributes` | Defines geometry and attributes and opens a window in one statement. | V3.0, pp. 472–474 |
| `TITLEW [#]number,text$` | Sets title-bar text; an empty string makes the window immovable, while two spaces suppress the title. | V3.0, pp. 474–475 |
| `TOPW #number` | Brings an already-open window to the top without closing/reopening it. | V3.0, p. 475 |
| `W_HAND(number)` | Maps GFA window number to original GEM handle. | V3.0, p. 475 |
| `W_INDEX(handle)` | Maps GEM handle back to GFA window number. | V3.0, p. 475 |
| `WINDTAB` / `WINDTAB(number,index)` | Exposes the window-management table; V3.0 array-like accessor permits reading/writing fields. | pp. 476–478 |

The book documents 12-bit window attributes: title/name, close, full, move, info line, size, vertical arrows/slider and horizontal arrows/slider. WINDTAB stores per-window handles, attributes and geometry, plus screen bounds and graphics origin. The manual's text has an OCR-sensitive index table (the Y-coordinate/width/height index labels are misrecognized in places); use the underlying six-word layout and page image before freezing the exact accessor indices. [pp. 476–478]

Window 0 is a special non-visible GEM work area that reserves the menu bar. Opening it changes the effective output origin; closing it restores full-screen TOS/GEM output. The book warns that window contents are the application's responsibility: after move/resize/redraw, text and graphics must be restored by the program. The large example demonstrates handling WM_TOPPED, WM_CLOSED, WM_FULLED, WM_ARROWED, WM_HSLID, WM_VSLID, WM_SIZED and WM_MOVED messages, and closing all windows on Break to avoid leaving GEM in an inconsistent state. [pp. 473–486]

## 15. Menu programming (chapter 15, pp. 487–490)

- `MENU array$()`: installs a pull-down menu from a one-dimensional string array. The first menu contains the desktop/accessory title, one information item, a dash separator sized for accessory names, accessory placeholders, and a null-string terminator. Further menus consist of a title and item strings, each terminated by a null string; the complete structure also needs a final terminator. [pp. 487–489]
- A menu item beginning with a dash is initially disabled/grey. Two leading spaces reserve room for a checkmark. [p. 488]
- `MENU itemIndex,attribute`: changes checkmark/disabled/enabled state. The OCR shows a duplicated attribute value for checkmark clear/set; this must be checked against the page image and a real interpreter before exact numeric constants are published. [p. 487]
- `MENU KILL`: disables the menu without discarding the menu text array. [p. 490]
- `MENU OFF`: restores the menu title's normal appearance after a selection. [p. 490]

The menu is a GEM/AES menu tree, not merely a textual menu. The book's accessory example temporarily creates an accessory menu, services it through ON MENU, and restores the screen background. Menu array sizing includes all strings and empty terminators; malformed layouts can corrupt menu construction.

## 16. Event monitoring (chapter 16, pp. 491–498)

### 16.1 Event buffer

`MENU(index)` exposes the current GEM multi-event buffer. The book maps MENU(0)–MENU(8) to the GEM message buffer and MENU(9)–MENU(15) to the VDI INTOUT array. Important fields include event ID, window handle, geometry, event flags, pointer coordinates, button state, modifier state, key code and click count. [pp. 491–493]

Documented message identifiers include:
- 10: menu item selected.
- 20: window redraw.
- 21: window topped/activated.
- 22: window closed.
- 23: window full-size requested.
- 24: window arrow/scroll action, with subcodes for page/line movement.
- 25 / 26: horizontal / vertical slider moved.
- 27: window resized.
- 28: window moved.
- 29: next window exposed after closing the top window.
- 30 / 31: accessory opened / closed. [pp. 491–492]

The event flag at MENU(9) uses bits 0–5 for keyboard, mouse button, IBOX1, IBOX2, message and timer events. MENU(10)–MENU(15) hold coordinates, button state, modifiers, packed key code and click count. [pp. 492–493]

### 16.2 Event registration and polling

| Construct | Behavior recorded | Evidence |
|---|---|---|
| `ON MENU [time]` | Polls/dispatches GEM multi-events; optional timeout in milliseconds from V2.02. Must be called repeatedly, including inside wait loops, or event messages can be overwritten/not serviced. | pp. 493–494 |
| `ON MENU GOSUB procedure` | Registers a handler for pull-down menu selections; MENU(0) identifies selected item. | p. 494 |
| `ON MENU BUTTON count,button,status GOSUB procedure` | Registers mouse-button/click-count conditions. The author explicitly says the parameter interactions are difficult to predict and should be tested. | pp. 494–495 |
| `ON MENU IBOX id,x,y,w,h GOSUB procedure` | Watches entry into one of two absolute screen rectangles. | p. 496 |
| `ON MENU OBOX id,x,y,w,h GOSUB procedure` | Watches exit from one of two absolute screen rectangles. | p. 496 |
| `ON MENU KEY GOSUB procedure` | Registers keyboard event handler; MENU(14) returns packed key code. | p. 497 |
| `ON MENU MESSAGE GOSUB procedure` | Registers general GEM message/multi-event handler; inspect MENU(index) for details. | p. 497 |

The book warns that the event buffer is continuously updated. A handler must inspect the current buffer promptly; delayed reads can observe a later event instead. The ON MENU polling call is required for ON MENU ... GOSUB callbacks to run. The ON MENU BUTTON description is explicitly empirical/uncertain in the source, so do not turn its parameter semantics into a firm rule without runtime tests.

## 17. Architecture implications and issue traceability (chapters 12–16)

1. Keep program lifecycle and error semantics in the GFA BASIC runtime core, but model TOS process exit codes and basepage behavior as Atari-specific.
2. Keep AFTER/EVERY as a single Atari interpreter timer facility with deferred callbacks and documented replacement/re-entrancy behavior; do not implement them as unrestricted preemptive threads.
3. Treat keyboard packed codes, GEM mouse state, joystick ports and function-key definitions as separate input surfaces with explicit versioned semantics.
4. Keep ALERT/FORM_ALERT/FILESELECT, window management, menus and ON MENU events in the Atari GEM/AES API profile.
5. Do not emulate GEM window/message/menu behavior by merely forwarding names to SDL3; the compatibility layer must reproduce observable GEM state and message ordering.
6. Raw Line-A, Basepage, DTA and internal GEM RAM modifications are low-level/unsafe operations and need explicit support classification.
7. The large code examples in chapters 13–16 are demonstrations and should not be copied as normative API definitions; the author's own warning about uncertain ON MENU BUTTON behavior is especially important.

This batch is covered by existing issue **#25 — Create Atari ST GFA BASIC API reference**, issue **#19 — Build complete GFA BASIC language feature inventory and compatibility matrix**, issue **#3 — Specify supported dialects and versions**, and issue **#42 — Build reference and conformance test corpus**. The menu/window/event behavior belongs primarily to the Atari API profile; no duplicate issue was opened.

Remaining validation:
- Page-image inspection of MENU attribute values and WINDTAB(index) element mapping.
- Confirm whether every window command is V3.0-only and identify any V2.xx equivalents.
- Test STOP/CONT restrictions, RESUME label stack effects and fatal-error recovery on exact interpreter builds.
- Test AFTER/EVERY replacement, callback timing, suspension and re-entrancy.
- Test packed KEYGET/KEYTEST codes, KEYLOOK non-consuming behavior and KEYPRESS injection.
- Verify MENU event IDs, MENU buffer field mapping and ON MENU BUTTON parameter behavior on real GEM/TOS.
- Separate safe public GEM/AES calls from undocumented RAM patching examples.


## 18. GDOS/VDI library (chapter 17, pp. 499–530)

### 18.1 GDOS availability and device configuration

The book states that most functions in this chapter require resident GDOS, installed during boot from `GDOS.PRG` and configured by `ASSIGN.SYS`. The two font-query helpers `VQT_EXTENT()` and `VQT_NAME()` are identified as exceptions. The ASSIGN.SYS examples distinguish screen, plotter, printer, metafile, camera and graphics-tablet drivers, with device IDs and font lists. [pp. 499–502]

This is an optional Atari system service, not an intrinsic BASIC facility. Availability must be represented as a runtime capability; failure to load GDOS or a device driver must not be silently treated as a successful workstation open.

### 18.2 Workstations and VDI state

- V3.0 `V-H` reads or sets the current VDI handle. The documented special values distinguish the GFA-managed handle from a freely selected handle; VDI calls use the handle in the control block. [p. 503]
- `V_CLRWK()` clears the output buffer/current workstation; for a printer it also emits a form feed. [p. 504]
- `V_CLSWK()` closes a workstation opened with `V_OPNWK()`, flushes pending output and restores the GFA handle. [p. 505]
- `V_OPNWK(driverId,...attributes...)` opens a physical output workstation (for example printer/plotter) and returns its VDI device handle. The driver ID comes from ASSIGN.SYS. [pp. 507–511]
- `V_OPNVWK(...)` opens a virtual workstation for the screen; it returns a handle and output capabilities are exposed through `WORK_OUT(0..56)`. The V2.xx procedure form requires more explicit parameter setup. [pp. 505–507]
- The book warns that closing a workstation after opening it but without issuing graphics output could produce a “RESERVE” error in its tested environment. Treat this as an observed compatibility warning, not a universal VDI rule. [p. 511]

A workstation has a lifecycle (open, select/use, flush/close); the runtime must track handles and device capabilities separately from the default screen. Printer output is buffered and may have side effects such as form feed.

### 18.3 Fonts and text metrics

- `VST_LOAD_FONTS(0)` loads additional fonts listed in ASSIGN.SYS and reports the number actually loaded; memory may need to be reserved first. [p. 515]
- `VQT_NAME(index,name$)` returns the VDI font ID and its name from the loaded font list; when GDOS is absent, the book describes fallback information for the current system font. [p. 514]
- `VQT_EXTENT(text$[,coordinates...])` computes the bounding quadrilateral using current VDI text attributes; output coordinates are returned in PTSOUT. [pp. 512–513]
- The examples use VDI text attributes, font selection and coordinate scaling for output on printer and screen workstations. [pp. 508–513]

The chapter's remaining pages (pp. 516–530) describe custom font formats, font headers, conversion and direct manipulation of TOS/GEM font pointers. These examples include resident memory, supervisor-level patches and a ROM-TOS-specific assembler routine. They are historical low-level techniques, not safe portable font APIs. The book explicitly limits one patch to a particular 1986 ROM-TOS build. Do not reproduce those patches as general runtime behavior.

## 19. AES library (chapter 18, pp. 531–584)

### 19.1 Resource Construction Set and object-tree model

The chapter introduces the Resource Construction Set (RCS) as the visual editor for GEM resource trees, including menus, alerts, dialogs, panels and other object trees. Resource files contain typed GEM objects and associated structures; object order, parent/child relationships and object flags determine interaction and drawing behavior. [pp. 531–547]

A GEM resource tree is not equivalent to a generic UI widget tree. Preserve object indices, sibling/child links, flags, state, geometry and resource-owned strings/bitmaps where compatibility with RSC files is required. The RCS discussion is an editor/workflow description, not BASIC syntax.

### 19.2 AES calling convention and managers

The book describes approximately 70 AES functions and explains that V3.0 exposes AES calls as callable functions; V2.xx equivalents are shown where available. Calls exchange typed integer and address arrays through AES parameter blocks. Address parameters require 4-byte values; coordinate/value parameters require at least 2-byte integers. [pp. 547–549]

The documented API families are:

- **Application manager:** `APPL_INIT`, `APPL_EXIT`, `APPL_READ`, `APPL_WRITE`, and (where supported) application/event recording calls. Application IDs/handles identify other AES applications and message buffers. [pp. 548–551]
- **Event manager:** `EVNT_BUTTON`, `EVNT_DCLICK`, `EVNT_KEYBD`, `EVNT_MESAG`, `EVNT_MOUSE`, `EVNT_MULTI`, and `EVNT_TIMER`. These are blocking/waiting AES event operations; message fields correspond to the event data exposed by `MENU()` in chapter 16. [pp. 551–554]
- **Form manager:** `FORM_ALERT`, `FORM_BUTTON`, `FORM_CENTER`, `FORM_DIAL`, `FORM_DO`, `FORM_ERROR`, `FORM_KEYBD` and `FORM_MOUSE`, plus file-selector/form routines. Forms operate on GEM object trees and return the selected exit object or edited text/state. [pp. 554–559]
- **Graphics manager:** `GRAF_HANDLE`, `GRAF_MKSTATE`, `GRAF_MOUSE`, `GRAF_MOVEBOX`, `GRAF_GROWBOX`, `GRAF_SHRINKBOX`, `GRAF_RUBBERBOX`, `GRAF_SLIDEBOX` and `GRAF_WATCHBOX`. These query pointer/workstation state or produce AES interaction effects. [pp. 559–564]
- **Menu manager:** `MENU_ICHECK`, `MENU_IENABLE`, `MENU_REGISTER`, `MENU_TEXT`, `MENU_TNORMAL` and `MENU_BAR`. These modify GEM menu-tree state, checkmarks, enablement and title highlighting. [pp. 564–566]
- **Object manager:** `OBJC_ADD`, `OBJC_DELETE`, `OBJC_DRAW`, `OBJC_EDIT`, `OBJC_FIND`, `OBJC_OFFSET`, `OBJC_ORDER`, `OBJC_CHANGE` and related calls. These navigate, draw and mutate typed object trees. [pp. 566–569]
- **Resource manager:** `RSRC_FREE`, `RSRC_GADDR`, `RSRC_LOAD`, `RSRC_OBFIX` and `RSRC_SADDR`. Loading reserves GEM memory; freeing releases it. Pointers to nested structures may need adjustment when structures are moved. The author warns that `RSRC_SADDR` is not always reliable. [pp. 569–571]
- **Scrap manager:** `SCRP_READ` and `SCRP_WRITE` access the shared GEM scrap/clipboard text buffer, with a documented maximum buffer size of 160 bytes in the described interface. [pp. 571–572]
- **Shell manager:** `SHEL_FIND`, `SHEL_GET`, `SHEL_PUT`, `SHEL_READ`, `SHEL_WRITE` and related calls locate applications, exchange launch information and read/write command-line or environment data. [pp. 573–574]
- **Window manager:** `WIND_CALC`, `WIND_CREATE`, `WIND_DELETE`, `WIND_FIND`, `WIND_GET`, `WIND_OPEN`, `WIND_CLOSE`, `WIND_SET` and `WIND_UPDATE`. The API separates window creation/handle allocation, opening/closing, geometry/state queries and update locking. [pp. 575–580]

The chapter includes examples of form/resource/menu use on pp. 580–584. These examples demonstrate sequencing and resource ownership, but should not be copied verbatim into the project's tests or treated as complete conformance coverage.

### 19.3 AES lifecycle and compatibility constraints

1. Application registration and exit are explicit AES lifecycle operations; the book notes that GFA may manage some application-ID details internally.
2. Event calls may block the BASIC program while waiting for user input. They must cooperate with the runtime's event pump and timer model.
3. Form and menu operations depend on a valid resource/object tree and correct object indices/flags.
4. Resource memory is allocated by GEM and must be released with the matching resource call. BASIC memory restoration/reservation may also be necessary.
5. `WIND_UPDATE` locks/unlocks AES window update behavior while an application draws; incorrect pairing can leave other applications/accessories unable to redraw.
6. The author reports bugs or unreliable behavior for some AES functions (including application event recording and resource pointer adjustment). Preserve these as source-specific warnings; do not assume every documented call works identically across TOS/GEM variants.

AES is a platform API profile, not language-core behavior. It requires an explicit GEM/AES compatibility layer with event ordering, resource ownership, message formats and window state.

## 20. GEM object variables and structures (chapter 19, pp. 585–596)

### 20.1 Object-tree record

The basic GEM OBJECT record is 24 bytes: six 16-bit link/type/flag/state words at offsets 0–10, a 32-bit OB_SPEC at offset 12, and four 16-bit geometry fields at offsets 16–22. Page images confirm this layout. [pp. 585–591]

GFA exposes object fields through getter/setter functions:
- `OB_ADR(address,object)`: byte offset/address of an object in its tree (not itself a field in the OBJECT record).
- `OB_NEXT`, `OB_HEAD`, `OB_TAIL`: sibling/child navigation links.
- `OB_TYPE`: object type.
- `OB_FLAGS`: 9-bit mask; bits 0–8 are SELECTABLE, DEFAULT, EXIT, EDITABLE, RBUTTON, LASTOB, TOUCHEXIT, HIDETREE and INDIRECT.
- `OB_STATE`: 6-bit visual/interaction state mask, including selected, crossed, checked, disabled, outlined and shadowed state.
- `OB_SPEC`: type-dependent value or pointer to a subordinate structure.
- `OB_X`, `OB_Y`, `OB_W`, `OB_H`: relative object geometry. [pp. 586–591]

Page-image checks confirm the nine OB_FLAGS bits (including INDIRECT at bit 8) and six OB_STATE bits. BOXINFO is a packed 32-bit value for G_BOX/G_IBOX/G_BOXCHAR; its fields are fill color, pattern, drawing mode, optional text color, border color/thickness and optional character. Exact normalized constants still need registry-level review. [pp. 588–592]

### 20.2 Nested GEM structures

The chapter documents the resource-file header and several subordinate structures:
- `BOXINFO`: packed 32-bit color/pattern/border specification for box-like object types. [p. 592]
- `TEDINFO`: text-edit structure containing pointers to initial text, input template and validation/format string, plus text/font/color/justification/length fields. [pp. 592–593]
- `ICONBLK`: icon mask/data pointers, dimensions, character and label information. [p. 594]
- `BITBLK`: bitmap pointer, dimensions, row/plane information and color. [pp. 594–595]
- `G_IMAGE` and `G_ICON` data blocks: raster data and optional mask/label fields. [pp. 594–595]
- `PARMBLK` / application-object block: tree/object identity, previous/current state, pixel geometry and redraw information passed to custom object drawing. [p. 596]

Page-image validation confirms RSC-HEADER 38 bytes, TEDINFO 28 bytes, ICONBLK 36 bytes, BITBLK 14 bytes, APPLBLK/USERBLK 8 bytes and PARMBLK 30 bytes. These 68k-era ABI structures use 16-bit words and 32-bit pointers. They must be modeled explicitly with target-width fields; host pointer size and host struct layout must never be used directly.

## 21. GEM addresses and parameter blocks (chapter 20, pp. 597–604)

The book documents direct access to the shared AES/VDI parameter arrays:

- `ADDRIN`, `ADDROUT`: AES address input/output arrays; 4-byte pointer entries.
- `GINTIN`, `GINTOUT`: AES integer input/output arrays; 2-byte entries.
- `GCONTRL`: AES control block with opcode, integer-array byte lengths and address-array element counts.
- `INTIN`, `INTOUT`: VDI integer input/output arrays.
- `PTSIN`, `PTSOUT`: VDI point input/output arrays.
- `CONTRL`: VDI control block with opcode, point/int array lengths, subfunction and workstation handle.
- V3.0 array accessors expose these blocks as indexed arrays; the corresponding raw address forms remain available. [pp. 597–601]

`VDIBASE` exposes a copy of VDI workstation state. The book warns that it is invalid on some Mega/Blitter TOS configurations and may contain incorrect values in GFA V3.0; it gives a ROM-TOS-specific fallback block address. The block contains text rotation, clipping, font count/current font, a copied font header, text color, line style and fill-pattern planes. [pp. 602–604]

Page-image validation confirms ADDRIN/ADDROUT use 4-byte elements, AES GINTIN/GINTOUT and GCONTRL use 2-byte words, and VDI INTIN/INTOUT/PTSIN/PTSOUT/CONTRL use 2-byte words. GCONTRL is five words (10 bytes); the GB pointer block contains six 32-bit pointers. VDI CONTRL has documented fields through offset 28. Treat these as transient shared call buffers, not independent application-owned data. VDIBASE is 308 bytes; its ROM-TOS fallback address remains profile- and OS-version-gated.

## 22. BIOS, GEMDOS, VDISYS, AES and XBIOS system calls (chapter 21, pp. 605–656)

### 22.1 Call interface

The book defines generic `BIOS(opcode,...)`, `GEMDOS(opcode,...)`, `VDISYS(opcode,...)` and `XBIOS(opcode,...)` entry points. Longword parameters must be explicitly marked with `L:`; unmarked values are passed as words. Results are returned through the 68k D0 register and can be used as expressions. V3.0 exposes AES calls as functions, while VDISYS remains the direct VDI escape hatch. [pp. 605, 608, 618]

These interfaces are raw Atari OS ABI calls, not portable BASIC functions. Parameter widths, signedness, register conventions, pointer validity, privilege level and error codes must be modeled explicitly.

### 22.2 BIOS

The chapter covers BIOS console input/output, raw disk I/O, device status and keyboard shift state. Notable calls include `BIOS(2)` console input, `BIOS(3)` console output, `BIOS(4)` sector read/write and `BIOS(7)` drive parameter block lookup. The parameter block describes sector size, cluster geometry, directory/FAT layout and data-area start. [pp. 605–608]

### 22.3 GEMDOS

The documented GEMDOS surface includes process termination/return, console input/output and status, default-drive selection, supervisor-mode switching, date/time, disk free-space, file open/close/read/write, file attributes, duplicate handles, process execution and directory search. Examples include:
- `GEMDOS(0)` process termination (the book explicitly says GFA programs should use QUIT/SYSTEM instead).
- `GEMDOS(1)`, `(6)`, `(7)`, `(8)`, `(9)`, `(10)`, `(11)`: console/key input/output and status.
- `GEMDOS(14)`: set default drive.
- `GEMDOS(32)`: enter/leave supervisor mode; dangerous privileged memory access.
- `GEMDOS(42–45)`: date/time get/set in packed bit formats.
- `GEMDOS(54)`: drive free-space information.
- `GEMDOS(61–64)`: open, close, read and write.
- `GEMDOS(67–70)`: file date/time/attribute and handle operations.
- `GEMDOS(75–79)`: process execution/termination and directory search. [pp. 608–618]

The book itself labels several low-level operations as dangerous, unsuitable or too complex to explain. Raw file handles and memory buffers must not be conflated with GFA's higher-level channel abstraction. Supervisor-mode and direct process termination calls require strict sandbox/privilege boundaries.

### 22.4 VDISYS

`VDISYS` invokes VDI functions using the shared parameter blocks. The book demonstrates querying current polyline and text attributes, using VDI escape functions (including cursor position, mouse availability and hardcopy), and manipulating interrupt-related VDI state. [pp. 618–623]

This is the same VDI subsystem as chapter 17 but through raw opcode/array access. Keep it in the Atari VDI profile and validate opcode-specific array layouts against authoritative VDI documentation.

### 22.5 XBIOS and hardware interfaces

The XBIOS section covers mouse initialization, screen resolution and screen-base switching, floppy sector I/O/formatting, MIDI, RS232 buffers/configuration, keyboard translation tables, random values, sound, MFP interrupt/timer control, and hardcopy. Examples include `XBIOS(4)` GETREZ, `XBIOS(5)` SETSCREEN, `XBIOS(8)` FLOPREAD, `XBIOS(12)` MIDIWS, `XBIOS(15)` RSCONF, `XBIOS(17)` RANDOM and `XBIOS(36)` PRTBLK. [pp. 623–656]

Pages 630–645 give a long diskette-structure and formatting example, including boot sector, FAT, directory and cluster layout, followed by a boot-sector virus discussion. Treat this as historical FAT12/Atari floppy material, not as a safe generic disk API. Do not reproduce disk-virus modification/cleaning procedures as implementation guidance. Pages 646–656 describe cursor timing, MFP bit/timer registers, sound-chip command bytes, MIDI vectors and printer blocks; these depend on specific Atari hardware and TOS versions.

## 23. System variables (chapter 22, pp. 657–664)

The final chapter lists absolute TOS/Atari memory addresses and hardware/system variables, including:
- Memory controller and reset vectors (`membctrl`, `resvalid`, `resvector`).
- Physical/user memory bounds (`phystop`, `membot`, `memtop`).
- Floppy seek rate and system timer/keyboard-related configuration.
- Screen/video state and pointers (`v_bas_ad`, `colorptr`, `screenptr`, VBL-related state).
- Interrupt vectors/lists and saved processor-register state (`savptr`, `vbl_list`). [pp. 657–664]

The book includes direct POKE-based changes to floppy timing, interrupt handlers and BASIC Break behavior. These are highly version-specific and potentially system-crashing. Record them as unsafe/unsupported low-level surfaces; do not implement them as ordinary language semantics. Any future emulation should use an explicitly selected Atari machine/TOS profile and safe virtualized memory model.

## 24. Architecture implications and issue traceability (chapters 17–22)

1. GDOS is optional and driver-configured; model it as a runtime capability and workstation lifecycle, not as always-present screen output.
2. VDI workstation handles, output buffers, font state and device capabilities need a dedicated VDI API profile. Physical printer/plotter output differs from virtual screen workstations.
3. AES requires a GEM object/resource model, application identity, event queue, form/menu/window managers and explicit resource ownership.
4. GEM OBJECT/TEDINFO/ICONBLK/BITBLK/PARMBLK records must use target 68k field widths and target addresses; host-native pointers/layout are incompatible.
5. AES/VDI parameter arrays are shared transient buffers. Calls can overwrite them; buffer lifetime and call ordering are observable.
6. BIOS/GEMDOS/XBIOS and absolute system variables are privileged Atari OS/hardware interfaces. Keep them out of the portable language core and gate by machine/TOS profile.
7. Disk formatting, supervisor-mode access, interrupt/vector changes and memory patching are unsafe operations. They need explicit support classification and must not be silently mapped to host OS operations.
8. Do not assume that the 1989 Atari V2.xx/V3.0 book documents Amiga behavior or later Atari releases.

This batch is covered by existing issues **#25 — Atari ST GFA BASIC API reference**, **#19 — language feature inventory**, **#3 — dialect/version compatibility policy**, **#18 — source provenance and licensing**, and **#42 — conformance test corpus**. No duplicate issue was opened.

Remaining validation for chapters 17–22:
- Check OCR-sensitive VDI function signatures, handle rules and font-header offsets against page images.
- Verify the exact AES function list/signatures and V2.xx versus V3.0 availability.
- Check all OBJECT flag/state bit positions and nested structure sizes/offsets against the printed tables.
- Verify AES/VDI parameter-block lengths and the V3.0 array accessors.
- Cross-check BIOS/GEMDOS/XBIOS opcode signatures and return/error conventions against Atari TOS documentation and real hardware/emulator behavior.
- Resolve book-specific TOS/ROM addresses and identify which are invalid on Mega/Blitter TOS or later machines.
- Do not mark runtime behavior as tested: this batch is a manual-backed first pass, not interpreter conformance testing.


## 25. Vario-RAM-Kartei application example (chapter 23, pp. 665–700)

The chapter presents RAMKART, a RAM-resident card-index/file-management application. It is explicitly a teaching example and a design study, not a language/API reference. The author says it replaces the simpler program from earlier editions and intentionally leaves a small sorting defect in the example: each sort pass shifts one card, and the final card is filled with ASCII 255. [pp. 665–666]

### 25.1 Storage model and file format

- The application reserves most of the BASIC memory with `RESERVE` and places fixed-size card records consecutively above `HIMEM`; V2.02 uses an additional 20,000-byte offset because of the interpreter's directory placement, while V2.0 and V3.0 do not need that offset. [p. 667]
- `BMOVE` is the primary in-memory block-transfer operation. `BGET#` and `BPUT#` transfer blocks to/from disk when disk updating is enabled. Individual cards are copied into string buffers for string-level search/edit operations. [p. 667]
- A companion `.DEF` initialization file stores field labels, field lengths, number of cards, data-file name and display title. A separate data file stores the fixed-size card area. [pp. 667–668]
- The whole card file is loaded into RAM at startup; the disk copy is updated only when the user requests it or when the Updating mode is enabled. [pp. 668–669]

### 25.2 User operations and implementation patterns

The example implements search (including two-term AND/OR search), browse, sort, updating toggle, jump-to-card, import, quit, alphabetic/numeric index tabs, row-based overview, in-place editing, cancel/restore, and commit-to-memory behavior. [pp. 668–670]

The listing demonstrates:
- UI/event loops combining `ON MENU`, mouse/key state, GEM alerts, file selection and custom drawing.
- Fixed-size record arithmetic and direct memory access.
- Search and sort over RAM buffers.
- Explicit dirty/update state and delayed disk persistence.
- Color palette access and restoration through XBIOS in the later helper routines. [pp. 671–700]

The program contains version-specific memory assumptions, direct screen/GEM access and low-level palette handling. Treat it as a source of architectural patterns and edge cases only. Do not import its implementation wholesale, and do not use its intentionally flawed sorting routine as an expected result. No new language feature is established by this example.

## 26. Compiler, variable organization, interpreter/editor and reference appendices (chapter 24, pp. 701–793)

### 26.1 V2.xx compiler model and options

The book describes the V2.xx compiler as producing standalone Atari PRG machine-code programs. The recommended workflow is to develop/debug in the interpreter, then compile from the desktop. The author claims interpreter/compiler compatibility, but also documents behavioral and runtime differences that must be preserved. [pp. 701–705]

Compiler options include:
- `OPTION "U1"`, `"U2"`, `"U3"`: select where/how Break checks are emitted. These affect whether Break is checked at explicit program points, loops or other runtime boundaries as documented in the option section. [pp. 706–707]
- Integer overflow checking can be enabled/disabled; the book notes compiled `INC`/`DEC` may use native machine instructions and wrap at signed 32-bit boundaries when unchecked. [p. 707]
- `OPTION "E+"` / `"E-"`: include or omit compiled error-message/handling support. [pp. 708–709]
- `OPTION "B+"` / `"B-"`: control bomb-error handling and recovery behavior. [pp. 709–710]
- The compiler's dialog options and source-level OPTION directives interact; the source directives take effect at their program position. [pp. 704–710]

The book further documents compiled-program differences for screen clearing, mouse cursor visibility, RESUME address recovery and process memory reservation. For example, the interpreter knows source-statement addresses directly, while a compiled program may need a label-based recovery target. [pp. 710–715]

Compiler semantics are a distinct execution profile. Do not assume that interpreter-only debugging, error recovery, cursor visibility, Break polling or memory allocation behavior transfers unchanged to compiled programs.

### 26.2 Variable representation and memory layout

The book documents target-specific variable storage:
- Unsuffixed numeric variables (and V3.0 `#` variables under the relevant DEFNUM/DEFLIST settings) are floating-point values: 6 bytes in V2.xx and 8 bytes in V3.0, with different decimal precision. [pp. 716–717]
- Integer suffixes select integer storage; integer array elements have a more compact representation than standalone variables in some cases. [pp. 717, 722]
- String variables use a 6-byte descriptor containing a 32-bit data pointer and 16-bit length. String contents are stored separately and managed through descriptor/trailer structures. [pp. 717–719]
- Array descriptors contain dimension and storage metadata; the book walks through how to locate string and numeric array elements through pointers and descriptors. [pp. 719–722]

These layouts are useful for understanding the Atari interpreter's memory model, but are not portable ABI guarantees. The compiler/runtime must not expose raw host pointers or reuse the 68k memory representation without a deliberate compatibility layer. Garbage collection and descriptor relocation are observable constraints for pointer-based code.

### 26.3 Direct mode and editor

The direct mode executes each submitted command immediately rather than waiting for RUN. It does not support multi-statement block constructs that require several separately entered lines; procedures defined in the editor can be invoked from direct mode. V3.0 includes a history of the last eight direct-mode commands. [pp. 723–725]

The editor section documents version-dependent keyboard shortcuts, syntax/structure checks, block operations, search/replace, load/save, listing and printer formatting, menus and editor configuration. V2.xx and V3.0 differ in several shortcut and menu details. [pp. 725–742]

This is an editor UX specification, not necessarily part of the language grammar. The compiler project should only model editor behavior if an editor compatibility target is explicitly in scope.

### 26.4 Quick lists, syntax tables, errors and indexes

The appendix contains:
- A procedure index for the accompanying book examples. [p. 743]
- Quick lists of IKB commands, system functions and API families, including BIOS/GEMDOS/XBIOS and AES/VDI calls. [pp. 744–759]
- An ASCII table. [p. 760]
- Editor and runtime error lists, including version-specific entries and bomb/68000 exception messages. [pp. 761–763]
- Syntax summaries grouped by I/O, sound, control flow, procedures/functions, operators, graphics, memory, time, events and system calls. [pp. 764–780]
- Alphabetical command/function index with page references. [pp. 780–793]

These indexes are useful as a completeness checklist and cross-reference aid, but OCR and typography make them unsuitable as the sole authority for exact signatures. Cross-check every indexed entry against its main chapter before adding it to the feature/API registry.

## 27. Overall conclusions after first-pass review of all 24 chapters

The complete printed chapter sequence has now received a first-pass review. This does **not** mean the book has been fully validated: OCR-sensitive tables and signatures, page images, version differences, conflicting sources and actual interpreter/compiler behavior still require separate verification.

Key architecture conclusions:

1. The book is evidence for Atari ST GFA BASIC V2.xx/V3.0 only. It must not be used as an Amiga specification or as proof of later Atari dialect behavior.
2. Separate language-core semantics, interpreter/editor behavior, compiler behavior, GEM/AES, VDI/GDOS, TOS system calls and hardware-specific interfaces.
3. Model target 68k words, longwords, pointers, endianness, alignment and memory descriptors explicitly; host-native data layouts are not compatible.
4. Preserve the distinction between BASIC channels and raw GEMDOS handles, and between normal screen output and VDI workstation output.
5. Event handling, timer callbacks, AES blocking waits, menu/window messages and redraw responsibilities are cooperative and stateful; they cannot be reduced to stateless command wrappers.
6. Low-level memory writes, supervisor mode, interrupt vectors, system variables, disk formatting and direct font-pointer patches are unsafe Atari-specific operations. They require explicit support policy and a virtualized target memory model, not direct host OS access.
7. The compiler and interpreter have documented differences in Break handling, overflow checks, error support, RESUME, cursor visibility and memory behavior.
8. The book's own example program contains a known sort defect; examples must not be treated as normative tests without independent expected results.

Issue traceability remains covered by existing **#3**, **#18**, **#19**, **#25** and **#42**. No new feature was implemented and no duplicate issue was opened.

### Remaining work after the first-pass book review

- Manually inspect page images for OCR-sensitive syntax, bit masks, structure layouts and command signatures, prioritizing language-core rules and frequently used APIs.
- Compare the Atari book against the project's other Atari references and the separately documented Amiga sources; resolve conflicts by dialect/version rather than merging them.
- Verify selected behaviors against version-pinned interpreter/compiler builds or emulators. Keep observed results distinct from manual claims.
- Convert the reviewed inventory into normalized reference/API registry entries and conformance-test candidates, retaining printed page, version, evidence status and unresolved questions.
- Continue respecting source rights: retain the source PDF/OCR locally; publish only concise original summaries and citations unless redistribution rights are confirmed.


## Chapter 18 follow-up: AES function catalogue (printed pp. 547–580)

**Status: first-pass OCR transcription completed; page-image verification and runtime validation pending.** The companion API reference now contains a per-function signature/contract inventory for the application, event, form, file-selector, graphics, menu, object, resource, scrap, shell and window managers.

Important source-specific findings:

- The author explicitly describes the chapter as a selected subset of roughly 70 AES functions, not a complete V2.xx library. V3.0 uses C-like function-call syntax; printed `<<` markers identify output parameters and are editorial notation, not BASIC syntax. Address arguments/outputs use 4-byte integer variables; coordinate/value arguments use at least 2-byte integers. [pp. 547–548]
- `APPL_INIT()` returns the application ID already assigned by GFA startup; `APPL_EXIT()` is described as unnecessary in GFA. `APPL_FIND` requires an exactly eight-character application name. [pp. 548–549]
- Event calls are blocking. `EVNT_MESAG` can wait indefinitely if no installed menu/window can produce a message. `EVNT_MULTI` combines keyboard, mouse-button, two rectangle, message and timer events; the printed table itself assigns the timer event to bit 0, duplicating the keyboard event bit; this is a confirmed source-level inconsistency, not OCR ambiguity. [pp. 551–554]
- `FORM_DO` owns the form interaction loop; without an EXIT/TOUCHEXIT object the program may not return to BASIC through normal interaction. `FSEL_INPUT` is explicitly described as more fragile than GFA's `FILESELECT` wrapper because invalid string buffers can corrupt GEM state. [pp. 555–558]
- Several graphics helpers are modal and require the left mouse button to be held; `GRAF_SLIDEBOX` does not update the object tree or redraw the final object. [pp. 558–563]
- The author records implementation defects or limitations for `APPL_TPLAY`, `APPL_TRECORD`, `RSRC_GADDR`, and `RSRC_SADDR`; these must be represented as source warnings, not generalized AES guarantees. [pp. 549–550, 569–571]
- `WIND_UPDATE` modes 0/1 control redraw-list locking; modes 2/3 transfer mouse control between AES and the application. Calls need balanced lifecycle semantics. [pp. 579–580]
- The OCR of the `WIND_GET` mode table contains a duplicated output-variable label, and the `EVNT_MULTI` timer bit is duplicated as bit 0 in the printed table. Page-image checks on pp. 549, 553–554 and 577 confirm the source wording/layout; do not silently infer a corrected bit. [pp. 553–554, 577]

No implementation or API-registry declarations were added. Existing issues #5, #12, #19, #25 and #42 already cover the API profile, registry, feature inventory, Atari reference and conformance evidence. The current work remains documentation/research under those issues.


### Page-image verification addendum (pp. 549, 553–554, 577)

The 180-dpi page images were checked against the OCR for selected high-risk details:

- **p. 549 — APPL_TPLAY:** speed values are exactly 50 (half), 100 (normal), 200 (double); Blitter-TOS-only restriction and always-1 return are visible.
- **p. 553 — EVNT_MULTI:** the printed six-bit event vector assigns keyboard to bit 0 and timer also to bit 0. This is a confirmed source defect in the book's table. No replacement bit is inferred.
- **p. 554 — EVNT_TIMER / FORM_ALERT:** timer units are 1/1000 second; FORM_ALERT syntax is `FORM_ALERT(Button,Text$)`.
- **p. 577 — WIND_GET:** modes are 4, 5, 6, 7 (work area/current outer/previous outer/maximum outer), 8/9 (horizontal/vertical slider position), 10 (current handle), 11/12 (first/next visible rectangle), and 15/16 (slider size). Mode 4's fourth output is printed as Get1 a second time, where the surrounding four-output pattern strongly suggests Get4, but the book is retained verbatim as evidence and the correction is not assumed.

These checks upgrade those exact source facts from OCR-only to page-image checked. They do not validate the runtime behavior against Atari hardware or GEM.


## Chapter 17 follow-up: GDOS/VDI and font handling (printed pp. 499–530)

**Status: first-pass transcription complete; selected function headings/contracts checked against page images. Runtime behavior remains unverified.**

The Atari API reference now records GDOS availability/configuration, VDI workstation lifecycle, font loading/querying and the book's own-font material.

Verified source details:

- GDOS functions in this chapter generally require resident GDOS and a valid boot-time `ASSIGN.SYS`; the source exempts `VQT_EXTENT()` and `VQT_NAME()`. The author recommends GDOS 1.8 over 1.1. [pp. 499–502]
- ASSIGN.SYS driver IDs documented: 01 screen virtual workstation only, 11 plotter, 21 printer, 31 metafile, 41 camera, 51 graphics tablet; 02/03/04 are screen-resolution pseudo-drivers for low/mid/high resolution. [pp. 500–502]
- `GDOS?` returns TRUE (-1)/FALSE (0). `V-H` reads/writes the active VDI handle; -1 restores GFA's own handle. [pp. 502–503]
- The page-image check confirms the V3.0 workstation calls and lifecycle: `V_CLRWK()`, `V_CLSVWK()`, `V_CLSWK()`, `V_OPNVWK(...)`, `V_OPNWK(...)`. A physical `V_OPNWK` requires resident GDOS and a valid non-screen driver ID; the book explicitly warns that missing GDOS or an invalid ID crashes the system. [pp. 504–507]
- `V_OPNWK` accepts ten optional VDI attribute defaults; the printed default call is `V_OPNWK(id,1,1,1,1,1,1,1,1,1,2)`. `V_OPNVWK` is for the screen and outputs directly without an update call. [pp. 505–507]
- The chapter documents `V_UPDWK()`, `VQT_EXTENT()`, `VQT_NAME()`, `VST_LOAD_FONTS(0)` and `VST_UNLOAD_FONTS(0)`. Font loading can allocate memory; the author recommends reserving memory first and unloading fonts before restoring BASIC memory. [pp. 510–515]
- The book reports that opening and closing a physical workstation without drawing may produce a RESERVE error; closing with pending buffered graphics but without `V_UPDWK()` may leave GEM in a faulty state. Keep this as an author-reported test observation. [p. 511]
- The own-font section (pp. 516–530) contains direct system pointer modification and low-level memory patches. These are unsafe/Atari-specific examples, not candidates for a general Linux runtime API.
- File layouts documented for high-resolution fonts: Degas 2,050 bytes; STAD/OLIFONT 4,096 bytes; 8×16 system font 4,698 bytes including a 602-byte header. [pp. 528–530]

Selected page-image checks: p. 503 (V-H read/write semantics), pp. 504–505 (clear/close/open virtual workstation signatures and lifecycle), p. 507 (physical V_OPNWK signature, ten optional parameters and crash warning). Other detailed font-layout fields and code examples remain OCR-derived and require additional image inspection before being treated as exact.

No compiler/runtime implementation was changed. Existing issues #5, #12, #19, #25 and #42 continue to cover the profile, registry, language/API inventory, Atari reference and conformance corpus work.


## Chapter 21 follow-up: BIOS, GEMDOS, VDISYS and XBIOS (printed pp. 605–656)

**Status: detailed first-pass inventory added to the Atari API reference; selected page-image checks completed; runtime behavior unverified.** Existing issue #25 covers the reference, with #3, #18, #19 and #42 covering dialect scope, source provenance, inventory and conformance. No duplicate issue was opened.

### BIOS (pp. 605–608)
The book documents BIOS opcodes 0–11: GETMPB, BCONSTAT, BCONIN, BCONOUT, RWABS, SETEXEC, TICKCAL, GETBPB, BCOSTAT, MEDIACH, DRVMAP and KBSHIFT. The generic signature marks longword arguments with `L:`; unmarked parameters are words and the function result is returned in D0. GETMPB returns a 12-byte structure containing three pointers; GETBPB returns an 18-byte per-drive parameter block. RWABS uses a 512-byte sector assumption in this source. BIOS vector replacement and raw sector I/O are unsafe OS-level surfaces, not portable BASIC semantics.

### GEMDOS (pp. 608–618)
The book documents console/serial/printer I/O, keyboard input/status, drive and directory state, DTA access, supervisor mode, packed date/time, process termination/residency, free-space query, raw file handles, memory allocation, process execution and directory search. The complete opcode grouping and source-described error values are now listed in §8.6 of the Atari API reference.

Important safety/compatibility distinctions:
- GEMDOS(0) process termination is explicitly discouraged in GFA; use QUIT/SYSTEM.
- GEMDOS(32) SUPER changes processor mode/stack and may alter registers; the book warns against casual use.
- GEMDOS(60–64) raw file handles are not GFA BASIC channels.
- GEMDOS(72–74) memory calls require exact block ownership/lifetime.
- GEMDOS(75–76) process control and GEMDOS(49) resident termination are not ordinary program operations.
- NUL-terminated target strings, DTA state, target pointers and negative error codes require an Atari compatibility layer; they must not be passed directly to host APIs.

### VDISYS (pp. 618–623)
A page-image check on p. 618 confirms the V3.0 call shape `VDISYS(opcode[,I_len,P_len[,id]])`, versus the V2.xx opcode-only wrapper. The active workstation handle is passed through CONTRL word 6 / byte offset 12; V2.xx obtains it from VDIBASE+40, V3.0 from V-H. The chapter demonstrates VDI opcodes 35–38, 104, 118 and escape opcode 5 with subfunction IDs. The VDI arrays are shared mutable target buffers. Timer-vector exchange is explicitly shown as V2.xx-only and must be treated as unsafe.

### XBIOS (pp. 622–656)
The chapter covers mouse initialization; screen base/resolution and palette; floppy sector I/O/format/verification; MIDI; MFP interrupts and timers; serial and keyboard configuration; sound-chip access; printer output; supervisor callbacks and AES shutdown. The complete grouped opcode inventory and error list are in §8.8 of the Atari API reference.

The detailed floppy/boot-sector/virus material (pp. 630–645) is destructive and security-sensitive historical Atari material. Sound-chip, MFP, timer and interrupt examples (pp. 646–655) are machine/TOS-specific. None should be implemented as direct host filesystem, audio, interrupt or process calls.

### Selected page-image verification

- **p. 605:** BIOS generic signature, `L:` longword marker, D0 result, GETMPB 12-byte block and its three pointers confirmed.
- **p. 612:** GEMDOS(32) SUPER description confirms supervisor-stack exchange, register caveat and explicit danger warning.
- **p. 618:** VDISYS V3.0 signature and active-handle source confirmed.
- **p. 623:** XBIOS INITMOUSE, SSBRK, PHYSBASE and LOGBASE declarations confirmed.

The rest of the chapter remains a searchable-text/OCR first pass; no TOS build, emulator or hardware probe was run. Do not mark any behavior as runtime-tested.


## Chapter 22 follow-up: system variables (printed pp. 657–664)

**Status: address inventory transcribed; selected page-image checks completed; no runtime validation.** The Atari API reference now contains a grouped inventory of the absolute addresses, widths and meanings listed in this chapter.

The chapter covers memory-controller/reset state; physical/user RAM bounds; floppy timing and write verification; boot drive and video mode; logical screen pointer; VBL semaphore/count/queue; palette and screen pointers; hard-disk vectors; keyboard terminal flags; trap return and memory descriptors; saved processor state; drive bits and disk buffer; hardcopy/printer flags; OS and AES base pointers. [pp. 657–664]

Important cautions:
- The source's nvbls entry is headed as a Word at 0x0454 but one sentence calls it a Longword. Preserve the discrepancy; do not normalize silently.
- vblsem can suppress VBL routines and BASIC Break handling. The examples that patch V2.xx interpreter bytes at p. 660 can also disable direct mode and make later ON BREAK behavior inconsistent.
- vblqueue, colorptr, screenptr and hard-disk vectors are pointers into target memory; host pointers are never valid substitutes.
- The author explicitly marks some fields read-only or says their internal behavior is not understood. Do not infer writable semantics from an address listing.

Page-image checks on pp. 657, 660 and 663 confirm the listed addresses and key descriptions, including the 0x045A palette pointer, 0x045E screen pointer, 0x0462/0x0466 counters, 0x04CE VBL list and 0x04F0 printer timeout flag. No runtime probe was performed.

## Chapters 23–24 completion status

The chapter 23 RAMKART example and chapter 24 appendices already have detailed first-pass sections above. They are considered **reviewed for source coverage**, not validated implementations:

- Chapter 23 records the fixed-size RAM card model, .DEF metadata, RAM/disk synchronization, UI/search/sort/edit workflows, memory assumptions and the intentionally defective sort pass. The example is not normative test data.
- Chapter 24 records V2.xx compiler workflow/options, interpreter/compiler divergences, numeric/string/array memory representations, direct mode, editor workflows, quick lists, syntax/error appendices and version caveats. Editor UI is separate from language semantics; raw target memory layouts are not host ABI contracts.

The systematic first-pass review now covers all 24 chapters. Remaining work is targeted verification and registry/test promotion, not an unreviewed chapter. OCR-sensitive claims and runtime behavior remain explicitly pending.


## Selected page-image checks: chapters 23–24 (printed pp. 665–793)

The following checks were made after the first-pass transcription:

- **p. 667 — RAMKART storage model:** confirms RESERVE-based memory management, fixed-size records above HIMEM, the additional 20,000-byte offset specifically for V2.02, BMOVE as the main transfer operation, BGET#/BPUT# for disk updates, and the companion .DEF initialization file. This validates the source summary; the application remains example-only.
- **p. 701 — compiler appendix:** confirms the beginning of the V2.xx compiler section and the author's framing of the compiler workflow as a separate execution environment. This is not evidence that the current Rust project implements compiler output.
- **p. 707 — integer overflow options:** confirms the distinction between unchecked native INC/DEC behavior and the T-/T+ overflow-checking option, plus E+/E- error-support options. Keep these as compiler-profile semantics, not interpreter defaults.
- **p. 717 — string representation:** confirms the six-byte string descriptor (32-bit pointer plus 16-bit length), a four-byte back-trailer, and the book's garbage-collection rationale. The memory representation is target-specific and must not be mapped to host pointers.
- **p. 761 — editor/runtime error list:** confirms that the appendix distinguishes editor structural errors from runtime error numbers and explicitly labels selected entries as V3.0-only. Do not flatten this into a dialect-neutral error enum.

These checks increase confidence in the cited source details only. No runtime/compiler probe was performed, and the full appendix remains a reference checklist rather than a complete transcription of every indexed command.


### Chapter 21 page-image/text consistency check: GETDFREE (p. 614)

A closer reading of the source reveals an internal contradiction in the GETDFREE description. It calls the destination a 12-byte buffer, but then lists four Longword fields at offsets +0, +4, +8 and +12. The fourth field makes the required extent 16 bytes. The API reference now records the 16-byte layout and preserves the printed “12-byte” wording as a source typo candidate. This is an evidence correction, not runtime validation.


## Chapter 21 cross-source validation: Atari BIOS guide and Atari Compendium

The BIOS/GEMDOS/XBIOS notes were compared with Atari's *A Hitchhiker's Guide to the BIOS*, Rev. 1.1 (5 March 1990), and *The Atari Compendium*.

Confirmed ABI details:
- BIOS uses TRAP #13, XBIOS TRAP #14, GEMDOS TRAP #1.
- The trap ABI pushes arguments in reverse order followed by the opcode; the caller removes the arguments from the stack.
- GEMDOS is not guaranteed re-entrant. BIOS/XBIOS have bounded re-entrancy and must not be treated as generally callback-safe.
- The Atari Compendium documents D0–D2 and A0–A2 as scratch registers for GEMDOS/XBIOS; the function opcode on the stack is modified. The GFA book's statement that the result is returned in D0 does not define the complete machine ABI.

Cross-source differences to retain:
- BIOS(11) KBSHIFT bits 5–6: GFA book says Alt+ClrHome/Alt+Insert; Atari BIOS Rev. 1.1 says mouse-button states, with bit 7 reserved.
- XBIOS opcode 11: GFA book says no effect; Atari BIOS Rev. 1.1 says used by BIOS; later Compendium material identifies DBMSG as a debugger extension.
- INITMOUS: the GFA book does not provide a usable parameter layout; Atari BIOS Rev. 1.1 provides a parameter block. Do not assume GFA's generic wrapper exposes that raw layout unchanged.

Chapter 22 low-memory discrepancies:
- 0x0424 memctrl width: Word in the GFA book, Byte in BIOS Rev. 1.1.
- 0x044A defshiftmod width: Word in the GFA book, Byte in BIOS Rev. 1.1.
- 0x0454 nvbls: Word in the book's heading/address layout and in BIOS Rev. 1.1, but one sentence in the book calls it a Longword.
- 0x04BA hz_200 and 0x04C2 drvbits in the GFA book differ from 0x04BC and 0x04C4 in BIOS Rev. 1.1.
- BIOS Rev. 1.1 lists other low-memory variables omitted by the GFA book.

The API reference now preserves these as source conflicts. The project must not collapse them into one universal Atari low-memory layout. No runtime probe or emulator comparison was run.

Sources:
- Atari, *A Hitchhiker's Guide to the BIOS*, Rev. 1.1 (1990): https://manualzz.com/doc/26181874/atari-bios-rev-1.1-system-software-guide
- *The Atari Compendium*, GEMDOS: https://bus-error.nokturnal.pl/atari_compendium/html/chap2.htm
- *The Atari Compendium*, XBIOS: https://www.yardley.cc/atari/compendium/atari-compendium-chapter-4-XBIOS.htm


## Chapter 17 cross-source validation: VDI/GDOS (printed pp. 499–530)

The chapter 17 notes were compared with *The Atari Compendium*, chapter 7 (VDI). The independent source confirms the central architectural split and adds important lifecycle/capability constraints:

- TOS includes a ROM-resident VDI screen driver. GDOS is a separate, optional boot-loaded extension for disk-loaded device drivers and additional fonts. Screen output with the system font does not inherently require GDOS.
- A physical workstation initializes one device; only one physical workstation may be open for a given device at a time. Multiple virtual workstations may share it, each with independent drawing attributes. Every VDI call requires the appropriate workstation handle.
- AES initializes the physical screen workstation. graf_handle() supplies its handle; opening a screen virtual workstation uses the physical handle and device ID Getrez()+2. This differs from opening a printer/plotter physical workstation via GDOS.
- The book's ASSIGN.SYS example IDs (01, 11, 21, 31, 41, 51) align with the Compendium's device-ID bands for screen, plotter, printer, metafile, camera and tablet. These IDs remain configuration-dependent, not guaranteed installed devices.
- Not every VDI function is supported by every device driver. The runtime must expose per-workstation capabilities rather than assume that a printer/plotter supports screen operations.
- The Compendium's vq_gdos() can distinguish original GDOS 1.x, FONTGDOS and FSM/Speedo variants. The book's GDOS? Boolean must not be promoted to a GDOS version query.
- The Compendium confirms that additional bitmap fonts become available to applications after vst_load_fonts(). It does not independently verify GFA's exact wrapper return values or unload behavior.

Implementation consequence: represent VDI availability, GDOS presence/version, physical devices, virtual workstation handles, and per-driver capabilities as separate runtime state. Preserve the book's exact GFA wrapper contracts and warnings as edition-specific until runtime probes are available. No interpreter/TOS probe was performed.

Source: [The Atari Compendium, Chapter 7 — VDI](https://www.yardley.cc/atari/compendium/atari-compendium-chapter-7-Atari-VDI.htm).


## Chapter 18 cross-source validation: AES event masks and window lifecycle (printed pp. 547–580)

The GFA AES chapter was compared with *The Atari Compendium*, chapter 6 (AES), and Atari's *GEM Programmer's Guide*, Volume 2.

- **EVNT_MULTI timer-bit discrepancy:** the GFA book's printed table assigns bit 0 to both keyboard and timer events; page-image inspection confirms this is in the source, not an OCR error. The Compendium documents the standard AES event masks as MU_KEYBD=0x01, MU_BUTTON=0x02, MU_M1=0x04, MU_M2=0x08, MU_MESAG=0x10 and MU_TIMER=0x20. Therefore the standard AES timer bit is 5 (0x20). Retain the book's transcription as a source defect and record the standard OS value separately; do not silently alter the source note or assume the GFA wrapper passes the mask unchanged without a runtime probe.
- **Blocking event model:** the Compendium describes EVNT_BUTTON, EVNT_KEYBD, EVNT_MESAG, EVNT_MOUSE and EVNT_TIMER as waiting calls and EVNT_MULTI as the combined wait operation. This corroborates treating GFA EVNT_* functions as potentially blocking control-flow operations.
- **AES ABI:** the Compendium confirms TRAP #2, D0 magic value 0xC8, D1 pointer to the AES parameter block and the six pointer fields CONTRL/GLOBAL/INTIN/INTOUT/ADDRIN/ADDROUT. This independently corroborates the parameter-block layout recorded in the API reference.
- **Window lifecycle:** the GEM Programmer's Guide distinguishes WIND_CREATE (allocate/obtain handle), WIND_OPEN, WIND_CLOSE and WIND_DELETE (release allocation). WIND_GET/WIND_SET operate on window state; WIND_UPDATE brackets redraw/update operations. This supports separate lifecycle states and paired update-lock handling.
- **Redraw ownership:** AES sends window-manager messages; applications process redraw requests and redraw the affected work area. A host windowing system must not be allowed to bypass this event/message contract.

The API reference records the source discrepancy and the standard AES contract separately. Exact GFA wrapper event-mask conversion, message-buffer behavior and return values remain runtime-unverified. No implementation or registry entries were changed.

Sources:
- The Atari Compendium, AES overview: https://bus-error.nokturnal.pl/atari_compendium/html/chap6.htm
- The Atari Compendium, AES function reference: https://bus-error.nokturnal.pl/atari_compendium/html/aes.htm
- Atari GEM Programmer's Guide, Volume 2: https://www.bitsavers.org/pdf/atari/ST/Atari_ST_GEM_Programming_1986/GEM_0464.pdf


## RSC fixture validation from the companion diskette

The supplied book companion-disk archive contains three RSC files: ALERT.RSC (1,154 bytes), FORMULAR.RSC (730 bytes) and MENU.RSC (2,630 bytes). Their first 36 bytes decode as the classic 18-word RSC header described by the Atari Compendium: `rsh_vrsn` is zero at offset 0, and the first resource data/string area begins at offset 36. The `rsh_rssize` field at offset 34 is 730 for FORMULAR.RSC and 2,630 for MENU.RSC, matching physical file sizes. ALERT.RSC declares 1,130 bytes while the archive entry is 1,154 bytes, leaving a 24-byte trailer that remains unexplained.

The companion BASIC listings AES_ALRT.LST, AES_FORM.LST and AES_MENU.LST load these exact resources through RSRC_LOAD. This ties the files to the book's example programs, but does not establish which resource-editor version created them. The verified fixtures therefore settle the format for these examples, not every possible GFA resource-editor output.

The book's 38-byte / 19-word header table remains a documented source discrepancy. No files from the companion diskette were copied into the public repository.

Source: user-provided *Das große GFA-BASIC-Buch* companion-disk archive; Atari Compendium Appendix C: https://bus-error.nokturnal.pl/atari_compendium/html/appendc.htm


## Chapter 24 companion-listing check: V2.xx versus V3.0 memory access

The companion diskette includes paired listings `TEIL1/PCODE_2X.LST` and `TEIL1/PCODE_30.LST`. Their own comments explicitly identify the target versions and provide a concrete cross-check of the book's V2.xx/V3.0 language differences.

- The V2.xx example declares `PROCEDURE pread_v2(p.frm%,p.anz%,p.adr%)` and writes through pointer arithmetic using `POKE`, `DPOKE` and `LPOKE`. It reads values with `PEEK`, `DPEEK` and `LPEEK`.
- The V3.0 example declares `PROCEDURE pread(p.adr%,VAR p.frm%,p.anz%)`, demonstrating an explicit `VAR` parameter, and uses typed memory references `BYTE{address}`, `CARD{address}` and `{address}` for byte, word and longword access.
- The V3.0 sample uses `MUL(b%,a%)` where the V2.xx sample uses ordinary multiplication, another source-backed indication that the examples are not intended to be syntax-identical across versions.
- The procedures named PCODE/PREAD in these listings are user-defined routines that serialize memory contents into DATA statements and restore them. They are **not** evidence of the compiler's internal p-code or bytecode format; the shared filename stem must not be confused with interpreter/compiler internals.

This companion material corroborates the book's version split and provides candidate syntax/semantic tests for issue #19 and the test corpus in issue #42. It is source evidence only; the routines were not executed in a GFA interpreter.


## Chapter 24 page-image validation: variable representation and suffixes (printed pp. 716–718)

The appendix's variable organization section was checked against page images, resolving an OCR-sensitive V3.0 byte suffix and adding storage details to the preliminary language reference.

- The 1-byte unsigned integer suffix is the literal vertical bar `|`, not an OCR placeholder. It is V3.0-only and has the documented range 0–255. (p. 716)
- The `&` suffix is a signed 2-byte integer, range -32768…32767, V3.0-only. The `%` suffix is a signed 4-byte integer, range -2147483648…2147483647. (p. 716)
- Unsuffixed real variables occupy 6 bytes in V2.xx and 8 bytes in V3.0. The book gives decimal precision limits of 11 and 13 digits, and maximum values 1.0E+154 and 3.595386269725E+308 respectively. These are book claims; they are not independently runtime-validated. (p. 716)
- Boolean scalar variables (`!`) occupy 2 bytes and are documented as 0/−1; Boolean array elements occupy one bit each. (pp. 716–717)
- String variables use a 6-byte descriptor containing a 4-byte character address and 2-byte length. The described string allocation also has a 4-byte back-trailer pointing to the descriptor and may include one alignment byte. (p. 717)
- The text describes `ARRPTR` as returning the string descriptor address. It describes reading descriptor fields with `LPEEK`/`DPEEK` and names V3.0 `CARD{}`/`LONG{}` forms on p. 718; exact lexical punctuation for those forms still needs page-image validation.

The vertical-bar suffix is now image-verified. These memory layouts remain Atari interpreter implementation details and must not be projected onto the project's portable IR or Linux runtime. No interpreter execution was performed.
