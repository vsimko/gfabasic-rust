# Amiga GFA BASIC 3.0 reference research notes

- Status: Research baseline. This document is not a complete language specification.
- Dialect: GFA BASIC for Commodore Amiga.
- Version covered by the main book: interpreter 3.0, with some later-version notes.
- Primary source: *Le livre du GFA Basic Amiga*, Wolf-Gideon Bleek, Martin Hecht, Uwe Litzkendorf; French translation by Christian Frey; Data Becker / Micro Application, ©1989; ISBN 2-86899-207-2.
- Source access: [Searchable PDF scan](https://www.gods69.com/download/Le_livre_du_GFA_BASIC_30_Amiga.pdf).
- Source register: [reference-sources.md](../reference-sources.md).

## Evidence and use

The PDF front matter identifies the authors, publisher, translation, year, ISBN, and source copyright. The preface says that the book documents the commands and functions available in the GFA 3.0 interpreter. The book is therefore a primary technical reference for the Amiga 3.0 dialect, subject to OCR validation and edition checks.

The PDF's digitalization notice records permission from author Uwe Litzkendorf for the digital edition. It also states that the author's rights remain with the author, the PDF may only be transmitted unmodified, individual-page printing is for private use, and public presentations/excerpts are allowed only when they are not for financial gain. The original publisher copyright notice is reproduced in the scan. This repository must not mirror the PDF or reproduce substantial extracts. Use short source citations and original summaries.

Evidence labels used below:
- **Verified from source**: directly stated in the identified book section/page.
- **Needs cross-check**: OCR or translation could be ambiguous, or the claim needs comparison with an original German edition or executable.
- **Observed**: reserved for behavior measured on a named interpreter release. No runtime observations are recorded in this document yet.

## Version scope and historical notes

The book's preface distinguishes several releases:

| Release | What the book reports | Evidence status |
|---|---|---|
| 3.00 | Final release after a beta with serious memory-allocation and file-size problems; German and English commercial editions are mentioned. | Verified from source, printed pp. 3-18 |
| 3.01 | First release distributed in France; the book's French-manual preparation reportedly exposed problems that were corrected in this release. | Verified as the author's historical account, printed p. 3-18 |
| 3.02 | Improvements included serial-port behavior and other functions, including INLINE. | Verified as the author's historical account, printed p. 3-18 |
| 3.03 | Described as offering greater safety and new menu-bar options; the preface still mentions issues, including SOUND. | Verified as the author's historical account, printed p. 3-18 |
| 3.04 | A later section is titled “Environnement du GFA Basic 3.04” (printed p. 374). The full scope of 3.04 changes is not established by the table of contents alone. | Needs cross-check |

Do not treat this 3.0 book as a complete specification for 3.52. Later releases require separate evidence and compatibility tests.

## Source notation and language rules

The introduction (printed pp. 1-9–1-12) describes the book's syntax notation:

- Square brackets mark optional syntax elements.
- Curly braces can mark an accepted command abbreviation. The book gives SYS and RET as examples. Curly braces also occur in memory-command syntax, so context matters.
- An ellipsis marks a sequence of statements, such as the body of a FOR...NEXT structure.
- Command, statement, and function names are printed in uppercase in syntax descriptions. Variables, strings, and parameters are represented in lowercase.
- The interpreter expands recognized abbreviations. The book does not provide a complete abbreviation table in the introductory passage.
- Parameter categories include addresses, numbers, arguments, arrays, return variables, numeric/string expressions, indices, file channels, window numbers, text, variables, and screen coordinates.
- The book says addresses are specified essentially as 32-bit integers. The exact address and pointer semantics must be cross-checked against the memory chapter and runtime.
- Functions can be used in assignments, output expressions, tests, and as dummy arguments to VOID. The book illustrates this in the introduction.
- The book states that the # prefix for file-channel numbers is optional in file operations, except OUT and IN.
- For GFA BASIC 3.0, the book says GOSUB can be omitted from ON...GOSUB procedure forms and that the interpreter inserts it. Example category: ON BREAK name.

The source also describes a structural editing model in which each command, statement, or function occupies its own program line. It describes indentation of structured blocks such as IF/ENDIF and FOR/NEXT. These claims are documented in the introductory chapter and should be checked against actual parser behavior before being encoded as hard lexical restrictions.

## Language and API topic map

The table below follows the book's chapter structure. Printed page numbers refer to the book, not the PDF viewer page number.

| Topic | Printed pages | Planned specification area |
|---|---:|---|
| BASIC fundamentals, variables, arrays, loops, comparisons, precedence | 21–60 | Language core |
| Input/output, screen, disk, files, direct-access files | 61–98 | Core and Amiga I/O APIs |
| Port I/O and AmigaDOS library | 99–112 | Amiga API profile |
| Printing and sound | 113–124 | Amiga API profile |
| Program structures, loops, branches, declarations, subprograms | 125–166 | Language core |
| String operations and formatting | 167–174 | Language core |
| Operators, math, numeric/trigonometric functions, comparisons, random numbers | 175–198 | Language core |
| Graphics definitions, drawing, graphics memory, animation | 199–246 | Amiga graphics API |
| Data conversion and number systems | 247–258 | Language core |
| Arrays, memory, user memory, pointers, Exec library | 259–300 | Core memory model and Amiga API |
| Program control, date/time, errors, multitasking, debugging | 301–330 | Core and Amiga API |
| Program/user interaction | 331–338 | Amiga API |
| Window and screen programming | 339–352 | Amiga GUI/graphics API |
| Menu programming | 353–358 | Amiga GUI API |
| BASIC event control | 359–368 | Amiga event API |
| Advanced programming and newer commands | 369–376 | Version-specific API |

## Initial confirmed architecture implications

These are implications for the project architecture, not implementation claims:

1. Keep the Amiga language dialect/version explicit. The book covers GFA BASIC Amiga 3.0 and includes historical notes for 3.01–3.04.
2. Keep the language core separate from AmigaDOS, Exec, graphics, windows, menus, sound, serial, and event APIs.
3. Model low-level address and pointer operations explicitly. Do not map arbitrary Amiga addresses directly to Linux process addresses.
4. Keep interpreter/editor/runtime behavior separate from language syntax and generated-program runtime behavior.
5. Record each API command with its exact Amiga release, source page, signature, semantics, implementation status, and conformance test.
6. Treat third-party libraries and example packages as separate from built-in GFA BASIC commands.

## Next extraction tasks

- Extract the book's alphabetical command/function index, if present, and cross-reference every entry to its detailed section.
- Extract exact signatures and semantics from chapters 5–17 into feature records. Preserve printed page numbers.
- Manually inspect page images for OCR-sensitive syntax tables, symbols, and code examples.
- Compare the French translation with the German original where wording or command names are ambiguous.
- Compare the 3.0 reference with Amiga 3.52 documentation and executable behavior. Record version differences instead of silently updating the 3.0 claims.
- Build minimal conformance probes only after the exact command syntax and expected behavior are sufficiently established.

## Source citation format

Use citations in this form:

`AM-BOOK-FR-1989, GFA BASIC Amiga 3.0, printed p. N or §N`

For OCR-derived statements, manually check the relevant page image before marking the feature as verified.
## First OCR-pass inventory: numeric and bit operations

This is a discovery index from chapter 8, not a normative command registry. The OCR text available for this pass reaches printed page 197. Command spellings and punctuation below must be checked against the page images before use in a parser or API registry.

| Printed pages | OCR-discovered names | Status |
|---|---|---|
| 184–187 | ACOS, ASIN, ATN, COS, COSQ, DEG, PI, RAD, SIN, SINQ, TAN | Needs page-image check; OCR contains damaged punctuation and at least one apparent correction note for ACOS |
| 188–189 | MAX, MIN | Names and broad purpose visible; exact argument grammar needs page-image check |
| 189–195 | AND, BCHG, BCLR, BSET, BTST, BYTE, CARD, EQV, IMP, SHL, SHR, ROL, ROR, SWAP, WORD, XOR | Names visible; several overload/type-width spellings are OCR-sensitive |
| 196–197 | RAND, RANDOMIZE, RND | Names and broad purpose visible; exact signatures and edge cases need page-image check |

Examples of details that should be captured only after visual verification:

- The book describes BYTE and CARD as unsigned low-width extraction operations, and WORD as a signed extension operation. Confirm the exact widths and signedness from the page image.
- The shift/rotate family has long, word, and byte forms. OCR uses symbols such as ampersand and vertical bar in the spellings; these are particularly easy for OCR to corrupt.
- RAND, RANDOMIZE, and RND have different ranges and result types in the book. Do not collapse them into one generic random function.
- The OCR text reports a caveat for ACOS; preserve it as a version-specific historical note only after checking the printed page.

## OCR coverage

The searchable PDF extraction contains 12,360 lines and includes text through the end of the book, including printed pages 377–379. The initial research pass had only inspected chapter 8 through page 197; that was a limitation of the first extraction pass, not a limitation of the PDF. The final two appendices are an ASCII-code table (p. 377) and GFA BASIC error messages (p. 379).
## Verified language-core findings from the complete OCR text

**Correction to the earlier OCR coverage note:** the searchable PDF extraction contains 12,360 lines and includes text through the end of the book, including printed pages 377–379. The earlier statement that OCR ends around printed page 197 was incorrect; it appears to have confused the initial chapter-8 extraction with the complete document. The table of contents identifies Appendix A as an ASCII-code table (p. 377) and Appendix B as GFA BASIC error messages (p. 379).

### Names, keywords, labels, and type suffixes

Source: *Le livre du GFA Basic Amiga*, printed pp. 4-43–4-45.

- The book explicitly says Amiga GFA BASIC has no reserved words: variables may use names that are also instruction names, including key variables such as TIMER and DATES. This is a significant lexer/parser design constraint; token classification cannot simply reserve every command spelling in every syntactic context.
- A procedure is introduced by `PROCEDURE`; a user-defined function by `DEFFN`. The book gives examples of procedure calls by name or with `@`, and function calls using `FN` or `@`.
- Labels end in a colon and may optionally be followed by a `!` comment. The book warns that a label spelling that is also recognized as a command may be interpreted as that command (examples: `SAVE`, `FILESELECT`).
- The book describes type suffixes: `%` integer, `|` byte, `&` word, `!` Boolean, `$` string, and `#` real. It says `#` is optional for real variables.
- Variable and function names must start with a letter; the text permits letters, digits, dot, and underscore thereafter. The character-class notation in OCR is damaged, so confirm exact accepted character repertoire from the page image before implementing it.
- The editor displays keywords in uppercase and variables in lowercase. This is editor presentation behavior; do not confuse it with a case-sensitive language rule.

### Numeric representation and Boolean values

Source: printed pp. 4-38–4-40.

- Real variables are described as occupying 8 bytes, with approximately 13 significant decimal digits; excess fractional digits are rounded.
- Integer variables use a 4-byte signed representation with range −2,147,483,648 to 2,147,483,647. The book also mentions 1-byte and 2-byte integer variables.
- Boolean values are represented by 0 (false) and −1 (true). The book explains true as all bits set in a two's-complement long word.
- The book describes integer assignment from a real value as discarding the fractional digits. The exact rounding/truncation behavior should be verified with a runtime probe.

### Control-flow structures

Source: printed pp. 4-45–4-50.

- Four loop structures are documented: `FOR...NEXT`, `DO...LOOP`, `REPEAT...UNTIL`, and `WHILE...WEND`.
- `REPEAT...UNTIL` tests its termination condition at the end of the loop body.
- The book documents nested loops and states that their closing constructs must appear in reverse order to their opening constructs.
- `DO...LOOP` without a condition can be interrupted by the interpreter's interrupt key combination or by an `EXIT IF` condition.
- `GOTO` is available as a label jump but is not recognized as a structural block by the editor. Structured loops receive indentation in the editor.
- The book states that `EXIT IF` is the named conditional loop-exit construct; exact placement and allowed loop types require extraction from its own command entry.

### Arithmetic/bit operation inventory, checked against OCR near the end of the book

Source: printed pp. 8-189–8-197. Names and broad semantics are present in the searchable source. Operator suffix glyphs and a few OCR-corrupted signatures still need page-image verification.

| Name/family | Documented behavior | Evidence status |
|---|---|---|
| `BSET(value, bit)` | Sets the selected bit and returns the resulting integer value. | Verified from text, p. 8-191 |
| `BTST(value, bit)` | Tests a bit; returns −1 when set, 0 otherwise. | Verified from text, p. 8-191 |
| `BYTE(value)` | Returns the low 8 bits without sign. | Verified from text, p. 8-191 |
| `CARD(value)` | Returns the low 16 bits without sign. | Verified from text, p. 8-191 |
| `EQV(value1, value2)` | Bitwise equivalence operation. | Verified from text, p. 8-192 |
| `IMP(value1, value2)` | Bitwise implication operation. | Verified from text, p. 8-192; second parameter is OCR-corrupted in one occurrence |
| `SHL`, `SHR` | Bit shifts; long, word, and byte variants are documented. | Semantics verified, suffix spellings need page-image check, pp. 8-192–8-193 |
| `ROL`, `ROR` | Bit rotations; long, word, and byte variants are documented. | Semantics verified, suffix spellings need page-image check, pp. 8-193–8-194 |
| `SWAP(value)` | Swaps the low and high 16-bit words of a 32-bit value. | Verified from text, p. 8-195 |
| `WORD(value)` | Sign-extends the low 16-bit word to a signed 32-bit result. | Verified from text, p. 8-195 |
| `XOR(value1, value2)` | Bitwise exclusive OR. | Verified from text, p. 8-195 |
| `RAND(n)` | Returns an unsigned 16-bit integer in [0, n); n above 65535 is reduced modulo 65535 according to the book. | Verified from text, p. 8-196; edge behavior merits runtime test |
| `RANDOM(n)` | Returns a 32-bit integer in [0, n); negative n is permitted. | OCR spelling/signature needs page-image check, p. 8-197 |
| `RANDOMIZE([start])` | Initializes the random generator; reusing a start value reproduces the same sequence. | Verified from text, p. 8-197 |

Do not infer compiler implementation details from the prose alone. In particular, signed extension, integer conversion, Boolean representation, and width-specific bit operations should become conformance tests before code generation is committed.

## Research implications

1. The French book is not missing its final chapters: its OCR extraction includes the complete text and both appendices. Remove the earlier incomplete-OCR warning from planning.
2. The book says it describes approximately 360 commands, instructions, reserved variable names, and arrays (printed p. 1-9 and p. 4-44). The 1988 magazine review independently reported about 350 commands in the original software package. These are approximate counts with different scopes, not a contradiction to resolve by inventing a precise total.
3. The French book's appendices are ASCII codes and error messages, not an alphabetical command index. The contemporary 1988 review says the original German software manual had an alphabetical command list in its appendix. Locating that original manual remains a high-value task.
4. The no-reserved-words statement and type suffixes materially affect lexer/parser design. Raise these as evidence-backed dialect requirements, but keep them version-scoped to the Amiga 3.0 book until cross-checked against 3.52.
## First Amiga I/O command inventory

Source: chapter 5, printed pp. 5-61–5-98. This section describes interpreter I/O commands with Amiga-specific file and device behavior. The entries below are source-backed summaries; OCR-sensitive punctuation and all edge cases still require visual checking and conformance tests.

### Keyboard and text input

| Construct | Signature/behavior | Source |
|---|---|---|
| `FORM INPUT` | `FORM INPUT length, string_var`; bounded text entry, maximum 255 characters. The `AS` form edits an existing string from its beginning. | pp. 5-61–5-62 |
| `INKEY$` | Returns the most recently pressed key as a string; the book says function keys, cursor keys, and many modifier combinations can be detected. | p. 5-62 |
| `INPUT` | Reads values into a variable list; a comma separates fields, and a comma inside a string must be quoted. Supports input from an opened file/device channel. | pp. 5-64–5-66 |
| `INPUT$(count[, #channel])` | Returns a string of up to the specified number of characters; channel form reads from an opened file. Keyboard input is hidden and continues when the requested character count is reached. | pp. 5-66–5-67 |
| `LINE INPUT [prompt;] string_var` / `LINE INPUT #channel, string_var` | Reads a text line; commas are part of the text rather than field separators. | pp. 5-67 onward |

### File and device operations

| Construct | Signature/behavior | Source |
|---|---|---|
| `OPEN` | `OPEN mode, #channel, filename[, record_length]`. Modes documented: `O` output/create or truncate; `I` input/read; `A` append; `U` update/read and write; `R` random/direct-access records. | pp. 5-91–5-93 |
| Channel range | The book documents up to 100 simultaneously open files, channel numbers 0–99, normally referenced with `#`. | p. 5-92 |
| Amiga device names | The book lists `CON:`, `PRT:`, `PAR:`, `SER:`, `COM:`, `PIPE:`, `SPEAK:`, `RAW:`, `NEWCON:`, and `AUX:` as file-like device interfaces. | pp. 5-92–5-93 |
| `BGET [#]channel, start, count` | Reads a number of bytes from the current file position into memory at `start`. | p. 5-87 |
| `BPUT [#]channel, start, count` | Writes bytes from memory at `start` into the file at the current file position. | p. 5-87 |
| `CLOSE [#channel]` | Closes one channel; bare `CLOSE` closes all channels. | p. 5-88 |
| `EOF(#channel)` | Returns −1 when the file pointer is at the final byte, otherwise 0, according to the book's description. Verify empty-file and past-end behavior. | p. 5-88 |
| `FIELD` | Maps portions of a random-access record to string variables (`AS`) or memory addresses (`AT`). Multiple FIELD declarations may contribute to one record layout. | pp. 5-88–5-89 |
| `GET #channel[, record_number]` / `PUT #channel[, record_number]` | Reads/writes a selected direct-access record using the current FIELD layout. | pp. 5-89, 5-93 |
| `LOC(#channel)` | Returns the current file pointer position in bytes from the beginning. | p. 5-90 |
| `LOF(#channel)` | Returns the file length in bytes. | p. 5-90 |
| `RECORD [#]channel, record_number` | Selects the record for subsequent GET/PUT operations. | p. 5-94 |
| `RELSEEK #channel, offset` | Moves the file pointer relative to its current position; negative offsets move toward the beginning. | pp. 5-94–5-95 |
| `SEEK #channel, offset` | Positions the file pointer using an offset from the beginning or, for negative values, from the end as described in the book. | p. 5-95 |
| `STORE #channel, string_array[, count]` / `RECALL` | Writes/reads lines between a text file and a string array; STORE appends CR/LF per line. | pp. 5-95–5-96 |

### Amiga-specific implications

- `OPEN` is not just host filesystem I/O: the documented device names expose Amiga console, printer, parallel, serial, speech, pipe, and shell-console endpoints.
- The `R` mode and `FIELD`/`GET`/`PUT` family describe record-oriented direct access, distinct from sequential byte/line I/O.
- The book documents a maximum of 100 channels and a default direct-access record length of 128 bytes when omitted (pp. 5-92, 5-97).
- Host Rust file APIs cannot be treated as a drop-in semantic replacement for Amiga devices or direct-access record files. The compiler/runtime should expose these through a target API profile or explicit compatibility layer.
- File-channel syntax should be parsed with the documented optional `#` marker in mind; the introduction says `#` is optional for most file operations, with exceptions `IN` and `OUT` (printed pp. 1-11–1-12).

## Graphics, windows, screens, menus, and events: first verified extraction

This section records only claims tied to the Amiga 3.0 book. It is a discovery baseline for issue #24, not a complete API registry. Do not implement command signatures from the topic list alone.

### Graphics model and coordinate system

Source: AM-BOOK-FR-1989, chapter 9 (printed pp. 199–246); introductory example in chapter 4 (printed pp. 4-59–4-60); OPENW example in chapter 5 (printed pp. 5-114–5-115).

- The book documents two principal Amiga display modes: Hires at 640 × 256 pixels and Lowres at 320 × 256 pixels. It mentions 512 vertical points for interlaced mode; exact screen-mode selection and supported combinations need extraction from chapter 14.
- The example places coordinate origin (0,0) at the upper-left corner in the normal screen/window context. Treat coordinate origin and offsets as state associated with the active drawing target, not as a global assumption, until the window/screen chapter is fully extracted.
- ELLIPSE x,y,rx,ry is illustrated: the first two values specify the ellipse position/center, followed by horizontal and vertical radii. The source example is on printed pp. 4-59–4-60.
- GRAPHMODE is a documented graphics-state command; the book's examples use GRAPHMODE 1 and refer to an XOR drawing mode. The complete mode enumeration and persistence rules still need page-level extraction.
- OPENW 5 is shown opening a small centered BASIC window; the example then selects GRAPHMODE 1, draws ellipses, invokes HARDCOPY, and closes the window with CLOSEW 5 (printed pp. 5-114–5-115). This establishes a window lifecycle example, not the full OPENW signature or window resource semantics.
- The graphics chapter is explicitly divided into graphics definitions (pp. 199–209), graphical primitives (pp. 210–216), lines/points (pp. 217–224), graphics operations (pp. 225–236), PUT-string organization (p. 232), graphics-memory organization (p. 235), and animation (pp. 237–246).

### Window, screen, menu, and event API scope

The table of contents establishes separate chapters and versioned scope, but is not itself sufficient evidence for each command's syntax:

| Area | Printed pages | What can be asserted now | Status |
|---|---:|---|---|
| GFA BASIC window commands | 339–346 | Dedicated window-command section exists | Signatures/semantics not yet extracted |
| GFA BASIC screen management | 347–352 | Dedicated screen-management section exists | Signatures/semantics not yet extracted |
| Menus | 353–358 | Dedicated menu-programming chapter exists | Signatures/semantics not yet extracted |
| BASIC event control | 359–368 | Dedicated event-control chapter exists | Signatures/semantics not yet extracted |
| New graphics functions | 372–373 | Separate later-command section exists | Release applicability and command details not yet extracted |
| GFA BASIC 3.04 environment | 374 | Explicitly version-scoped section exists | Do not assume it applies to 3.0 or 3.52 |

### Implementation constraints inferred for architecture (not API semantics)

1. Keep graphics state (drawing mode, fill/line/text definitions, clipping, active Rastport/Viewport/Colormap, and coordinate transforms) separate from SDL3 calls.
2. Model screens and windows as explicit runtime resources with ownership and lifecycle. OPENW/CLOSEW examples demonstrate resource creation/destruction, but exact semantics remain to be extracted.
3. Preserve Amiga display-mode and pixel-coordinate semantics in the compatibility layer. Convert to the Linux presentation format only at the backend boundary.
4. Treat sprite/BOB animation, graphics memory, bitmap transfer, menus, and event handling as separate subsystems; do not collapse them into a single generic drawing API.
5. Keep the 3.0 book's commands distinct from later 3.04 additions and 3.52 observed behavior.

### Next source extraction checklist

- [ ] Manually inspect chapter 9 page images and extract exact syntax for every graphics-definition and drawing command.
- [ ] Extract GET/PUT/SGET/SPUT and graphics-memory semantics, including string-buffer layout and ownership.
- [ ] Extract exact OPENW/CLOSEW/screen-management signatures and resource rules from chapter 14.
- [ ] Extract menu construction, selection return values, and event-handler signatures from chapters 15–16.
- [ ] Identify which chapter-17 additions belong specifically to 3.04.
- [ ] Cross-check all command spellings and signatures against the German original and an Amiga 3.52 interpreter before treating them as implementation contracts.


## Cross-check from an Amiga GFA BASIC example corpus (1993)

Source: Conrad Bessant, *Computers and Chaos* (Amiga edition), 1993; online transcription at https://amigasourcecodepreservation.gitlab.io/computers-and-chaos-amiga-edition/. This is a published example corpus, not a normative GFA manual. The author explicitly says the GFA listings use version 3.0 syntax and should run on 3.0 and 3.5 without alteration (book §1, online lines 117–118). Use it as corroborating evidence and for future conformance fixtures, not as the sole source for command semantics.

### Graphics operations found in tested listings

| Construct | What the example demonstrates | Evidence / limitation |
|---|---|---|
| `PLOT x,y` | Plots a point at integer screen coordinates; used in nested loops to render a Julia set. | GFA listing in chapter 5; syntax and practical usage corroborated by examples, exact clipping/rounding behavior unspecified. |
| `COLOR n` | Selects the drawing colour before plotting. | Used before PLOT and PSET; palette-index range and invalid-index behavior unspecified. |
| `GRAPHMODE 3` | Selects XOR mode for drawing a temporary zoom rectangle; `GRAPHMODE 0` restores normal mode. | GFA listing in chapter 5; supports these two observed values only, not a complete mode enumeration. |
| `BOX x0,y0,x1,y1` | Draws a rectangle used as a reversible XOR zoom-selection outline. | Example demonstrates four-coordinate form; fill/outline semantics need primary-manual confirmation. |
| `MOUSEK`, `MOUSEX`, `MOUSEY` | Polls mouse-button state and current pointer coordinates during interactive selection. | Example usage only; bit mapping and coordinate frame need primary-manual confirmation. |
| `OPENS 1,0,0,640,256,4,32768` | Opens a 16-colour high-resolution non-interlaced screen in the example. | Literal example from chapter 5; parameter meanings require the primary manual. |
| `OPENW 1,0,0,640,256,4096,4096` | Opens a window associated with the screen in the example. | Literal example; parameter meanings and ownership remain unverified. |
| `TITLEW #1, text` | Sets the window title. | Used in example; channel marker and exact lifecycle semantics need manual confirmation. |
| `CLOSEW #3` and `CLOSEW #0` | Closes a temporary window and then selects/closes the main screen/window context in the example's helper routine. | Do not infer the exact meaning of channel 0 from this one example. |

The same book's conversion appendix explicitly maps Amiga BASIC `PSET(x,y)` to GFA BASIC `PLOT x,y`, and Amiga BASIC `SCREEN`/`WINDOW` to GFA BASIC `OPENS`/`OPENW` as approximate rather than exact replacements. This is useful corroboration that these are distinct GFA APIs, not syntax aliases to implement blindly.

### Menus and event dispatch in a complete GFA example

The 1993 GFA BASIC menu example demonstrates the following design:

- `MENU menu$()` creates/installs a menu from a string array. Empty strings delimit menus and a pair of empty strings terminate the menu structure.
- `MENU item,status` changes an item's status. The example uses values 80 (enabled, unticked), 336 (enabled, ticked), and 0 (disabled/light); treat these as observed example values until confirmed in the primary reference.
- `MENU(0)` returns the selected menu item index in the example.
- `ON MENU GOSUB branch` installs a menu-selection handler. The example's handler uses `ON MENU(0) GOSUB ...` to dispatch by item index.
- `MENU KILL` removes the menu before processing and the program reinstalls it afterward.
- The main loop uses `DO : SLEEP : LOOP` while the menu handler performs dispatch.

These observations provide a concrete source-code corpus for future menu/event tests. They do not establish callback reentrancy, event queue ordering, or whether SLEEP is required in every program.

### Version and evidence guardrails

- The corpus author identifies the examples as GFA BASIC 3.0 syntax and states compatibility with 3.5; this is not proof of compatibility with 3.52.
- The book's general discussion of Amiga BASIC SCREEN/WINDOW commands must not be mistaken for GFA BASIC syntax. Only listings explicitly identified as GFA BASIC were used above.
- Add these examples to the future conformance corpus only after checking source licensing and extracting the relevant GFA listing with provenance. The online book is CC BY-SA 4.0; preserve attribution and license obligations if any material is reused.
- No runtime feature is implemented on the basis of this secondary source. Exact signatures and semantics remain subject to the Amiga GFA BASIC manual and interpreter probes.
