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
| 5 | 61–154 | Input/output, screen, disk, files, random access, ports, printer, sound | Detailed first pass reviewed; page-image checks and runtime tests pending |
| 6 | 155–220 | Program structure, loops, branches, declarations, subprograms, external code | Reviewed |
| 7 | 221–228 | String manipulation, analysis, formatting | Reviewed |
| 8 | 229–250 | Operators, arithmetic, numeric/trigonometric functions, comparisons, bit operations, random values | Reviewed |
| 9 | 251–328 | Graphics definitions, object/line/pixel graphics, Line-A, raster operations, screen memory | Reviewed (first pass) |
| 10 | 329–340 | Data conversion and number systems | Reviewed (first pass) |
| 11 | 341–392 | Arrays, memory operations/management, pointers | Reviewed (first pass) |
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
