# Atari ST GFA BASIC V2.xx / V3.0 — preliminary language specification

- Status: **Research draft; not an accepted compiler dialect**
- Platform: Atari ST
- Source coverage: GFA BASIC V2.xx and V3.0, as described by the sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (1988)
- Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3), [issue #19](https://github.com/vsimko/gfabasic-rust/issues/19), [issue #25](https://github.com/vsimko/gfabasic-rust/issues/25)
- Source register: [ATARI-GFA-BOOK-1988](../reference-sources.md)

## 1. Purpose and limits

This document records an initial, source-backed technical model of the Atari ST GFA BASIC dialect described by the 1988 book. It is a research specification. It does not claim that the compiler implements this dialect.

The book explicitly covers GFA BASIC V2.xx and V3.0. Its author states that the V3.0 interpreter provides approximately 450 commands and functions. The book also covers the Atari ST environment and its system interfaces. These platform interfaces must not be treated as portable language-core features.

Do not transfer these findings to Amiga GFA BASIC. Do not assume that later Atari ST releases (including 3.5 or 3.6TT) have identical syntax or semantics. Each rule must retain its source version and platform.

## 2. Evidence and terminology

Use the repository evidence vocabulary:

- **Verified** — directly supported by an identified source passage or repeatable reference-interpreter observation.
- **Inferred** — a plausible interpretation that is not directly established by the source.
- **Disputed** — sources or observed implementations disagree.
- **Unknown** — evidence is not available.
- **Not yet assessed** — the topic is in scope but has not been researched in sufficient detail.

A book statement is evidence of what the book documents. It is not automatically proof of behavior in every interpreter build. Runtime observations must identify the exact interpreter release.

Page references below are printed page numbers in the 1988 book, not PDF viewer page offsets.

## 3. Dialect identity

| Dimension | Current research value | Evidence |
|---|---|---|
| Platform | Atari ST | Book title/context and Atari ST chapters |
| Language family | GFA BASIC | Title and language chapters |
| Earlier coverage | V2.xx | Preface and command notes |
| Main coverage | V3.0 | Preface; book describes the V3.0 interpreter |
| Later versions | 3.5, 3.6TT | Out of scope for this source |
| Runtime environment | Atari ST / TOS with GEM-related interfaces | Chapters on BIOS, XBIOS, GEMDOS, VDI, AES and system variables |
| Compiler | V2.xx compiler is documented separately | Chapter 24 |
| Evidence status | Primary published technical reference inspected | Source register entry ATARI-GFA-BOOK-1988 |

The book is the first detailed source for this Atari ST profile. It is not sufficient by itself to decide the project's first supported dialect. That decision remains tracked by issue #3.

## 4. Initial language-core findings

### 4.1 Source layout and statements

| Rule | Finding | Status | Source |
|---|---|---|---|
| Statements per source line | The book states that GFA BASIC accepts one command/instruction per line. | Verified as documented V2.xx/V3.0 behavior | p. 23 |
| Editor formatting | The interpreter/editor automatically arranges lines into a structured visual layout and indents nested constructs. | Verified as documented editor behavior | pp. 23–24 |
| Internal line numbering | V3.0 editor has internal line numbering. This is an editor feature; it does not establish BASIC-style numbered source lines. | Verified; source-line semantics remain unknown | p. 24 |
| Comments | V3.0 accepts REM and apostrophe comments. The book also describes ! as a comment marker within a command line and says a leading ! is converted to REM. | Verified as documented V3.0 behavior | p. 186 |
| Statement separators | No general colon-separated multi-statement syntax is established by the inspected passages. | Unknown | Further lexical research required |
| Labels and branch targets | Labels appear in examples and are used by constructs such as RESTORE Labelname; full label grammar and scope are not yet specified. | Partially verified; grammar unknown | pp. 111, 189 and control-flow chapter |
| Maximum source-line length | The function definition discussion mentions a maximum input-line length of 256 characters. Its exact applicability to all source lines and editor/compiler modes needs confirmation. | Verified for the cited function-definition context only | p. 193 |

Do not implement line numbers, colon separators, label syntax, or line-length limits from assumptions. Extract the relevant sections and confirm edge cases first.

### 4.2 Version 3.0 language additions

The book explicitly identifies several V3.0 changes over earlier versions:

- Byte and word variable types.
- Structured control-flow additions, including SELECT CASE and ELSE IF.
- Multi-line FUNCTION constructs.
- A revised editor and improved program-structure support.

Evidence: printed p. 24. This is a version-difference summary, not a complete grammar or semantics definition. Exact syntax and edge cases must be extracted from the relevant chapters.

### 4.3 Variable types and declarations

The book describes these V3.0 declaration commands (printed pp. 189–192). The page-image check confirms the names, type categories, storage widths and abbreviations:

| Declaration | Abbreviation | Declared type | Width | Suffix |
|---|---|---|---:|---|
| `DEFBIT` | — | Boolean | 2 bytes for scalar variables; 1 bit per array element | `!` |
| `DEFBYT` | `DEFB` | Unsigned integer | 1 byte | `|` |
| `DEFWRD` | `DEFW` | Signed integer | 2 bytes | `&` |
| `DEFINT` | `DEFI` | Signed integer | 4 bytes | `%` |
| `DEFFLT` | `DEFFL` | Floating point | 8 bytes | `#` |
| `DEFSTR` | `DEFS` | String | Descriptor-based; see below | `$` |

All six declarations are marked Version 3.0 in the command reference. The definition argument is a string (constant, variable or expression) describing the name prefixes to match. Examples on pp. 190–191 show individual initial letters (`"a"`), comma-separated prefixes (`"b,c"`), ranges (`"g-l"`) and multi-character prefixes (`"word"`). Matching is against the beginning of the variable name.

The declarations are global. The book says a later declaration invalidates the previous one. A variable with an explicit type suffix is excluded from the global prefix declaration and the explicit suffix takes precedence. The example on p. 191 demonstrates this with `DEFINT "i"`, an explicitly suffixed string variable `I_string$`, and an unsuffixed integer variable `I_4byte`.

The editor accepts `DEFSNG` and `DEFDBL` as alternatives for `DEFFLT`, but converts those spellings to `DEFFLT` during syntax checking. They should therefore be modeled as editor input aliases, not automatically as separate runtime declarations.

These are V3.0 language declarations; do not assume their availability in V2.xx merely because the book's companion listings also contain V2.xx examples.

The book also documents these type suffixes in the declaration discussion:

- % — 4-byte integer.
- & — 2-byte integer.
- ! — Boolean.
- # — 8-byte floating point.
- $ — string.

The V3.0 1-byte integer suffix is **`|` (vertical bar)**, confirmed directly from the page image (printed p. 716); it is not an OCR substitute for another punctuation mark. It denotes an unsigned byte with values 0–255. The same page documents `&` as a signed 16-bit integer (-32768…32767), and `%` as a signed 32-bit integer (-2147483648…2147483647). Both are V3.0-only for `|` and `&`; the book explicitly labels those types as V3.0 additions.

Additional representation details checked against printed pp. 716–718:

- Unsuffixed real variables use 6 bytes in V2.xx and 8 bytes in V3.0; the book states up to 11 and 13 decimal digits of precision respectively, with maximum magnitudes documented as 1.0E+154 and 3.595386269725E+308.
- `!` Boolean scalar variables use 2 bytes and are documented as holding only 0 (False) or -1 (True). Boolean array elements use 1 bit each, not 2 bytes.
- A string variable uses a 6-byte descriptor: a 4-byte address followed by a 2-byte length. The book describes a 4-byte back-trailer pointing back to the descriptor and a possible padding byte to align that trailer.
- The page 718 image explicitly prints `CARD{}` and `LONG{}` as the V3.0 alternatives to `DPEEK` and `LPEEK` for reading words and longwords. The brace notation is therefore image-verified in this context.

These are source-documented storage details, not a portable host representation or an endorsement of exposing raw Atari memory in a Linux runtime. Numeric conversion, overflow handling in every execution mode, string garbage-collection behavior under stress, and complete array layouts remain **not yet assessed**.

### 4.4 Functions and procedures

The book documents:

- DEFFN for defining compact user functions.
- FN and the shorthand @ for calling such functions.
- Optional parameter lists.
- Parameters of differing types in a function parameter list.
- Parameter-count and parameter-type mismatch diagnostics.
- Parameter variables behaving locally during the function call, with the previous global value restored after return.
- Access to global variables not listed as parameters.
- V3.0 multi-line FUNCTION constructs.

Evidence: printed pp. 192–194.

This is not yet a complete procedure/function specification. The grammar, return typing, recursion limits, evaluation order, argument passing mechanism, VAR/by-reference behavior, local-variable declarations, and exact distinction between PROCEDURE, FUNCTION, and DEFFN require separate entries.

### 4.5 Control flow

The book's contents identify a dedicated program-structure chapter covering loops, conditional branches, declarations, and subprograms (chapter 6, printed pp. 155 onward). It explicitly names SELECT CASE and ELSE IF as V3.0 additions (p. 24).

The exact syntax and semantics for IF, ELSE, ENDIF, FOR/NEXT, WHILE/WEND, REPEAT/UNTIL, DO/LOOP, EXIT, CONT, GOTO, GOSUB, and RETURN have not yet been fully extracted. The book disk examples contain instances of several of these constructs, but examples alone are not normative proof of all edge cases.

### 4.6 Source composition and program files

The book distinguishes ASCII listings from tokenized program files. It describes .LST as an ASCII listing format and .BAS / .GFA as tokenized formats in the relevant interpreter versions. It also documents MERGE/editor composition and notes that the interpreter validates lines when ASCII files are merged.

Evidence: printed pp. 113–116.

The exact file-format byte layout, token assignments, escaping rules, and compatibility between interpreter versions are not specified here. Do not treat the book's example disk files as a complete or lossless corpus: its README says comments were removed to fit a single-sided disk and some programs require additional fragments to be merged as described in the book.



### 4.7 V2.xx compiler execution-mode differences

The book documents a **V2.xx compiler**, not a V3.0 compiler. It states that compiled programs are generally source-compatible with interpreter-developed programs, but lists important execution-mode differences (printed pp. 701–715):

| Area | Interpreter | V2.xx compiled program | Evidence |
|---|---|---|---|
| Break handling | Interpreter controls program execution | Configurable with OPTION U0–U3; checks occur at different program points | pp. 705–706 |
| Integer INC/DEC overflow | Interpreter checks signed 32-bit integer bounds | Direct machine instructions may wrap unless OPTION T+ adds checking | pp. 706–707 |
| Error text | Descriptive interpreter errors | OPTION E- gives code/PC; E+ adds descriptive text | pp. 707–708 |
| CPU exceptions | Interpreter handlers can translate selected exceptions | OPTION B- leaves bomb errors; B+ installs handlers | pp. 708–709 |
| Interpreter commands | LIST, LLIST, TRON, TROFF, DEFLIST, SAVE, PSAVE, LOAD, STOP, CONT are available in the interpreter context | Compiler cannot process them; it reports and can ignore or abort | p. 711 |
| RESUME / RESUME NEXT | Interpreter tracks the exact command location | Compiled mode requires an OPTION Ux program-counter checkpoint; U1/U2 may be approximate | pp. 712–713 |
| CHAIN | Loads another BAS program for interpreter execution | Launches PRG/TOS/TTP through AES SHEL_WRITE and terminates the caller | pp. 713–714 |
| FILESELECT | Editor screen buffer stores the background | Requires about 32.5 KB free memory for background storage | pp. 714–715 |
| Mouse cursor | Interpreter routinely restores visibility | Compiled program does not restore it after every command | p. 711 |
| 0/0 | Division-by-zero error | Book says result is 1 | p. 715 |

Compiler options U, T, E and B are compiler-profile behavior, not general language declarations. The book says the last matching in-source OPTION takes precedence over the compiler-dialog default. U0 omits break support; U1 checks at its point; U2 checks at loop boundaries and before GOTO; U3 checks after every BASIC statement. Additional checks and richer error/exception handling increase output size and/or reduce speed (pp. 705–709, 715).

The 0/0 result and the compiler's INC/DEC overflow behavior are particularly important compatibility quirks. Keep them scoped to the documented V2.xx compiler until tested against an identified compiler binary. Do not implement them as portable arithmetic semantics. The book's claims have not been runtime-verified by this project.



### 4.8 Interpreter operating contexts: editor, Run-Only and direct mode

The book distinguishes the full editor-interpreter, a Run-Only Interpreter, and the interpreter's direct mode (printed pp. 722–725). These are execution and interaction contexts, not dialects or separate syntax versions.

| Context | Documented capability | Restrictions / version differences | Evidence |
|---|---|---|---|
| Full editor-interpreter | Edit, load and run BASIC programs; auto-start is also supported | Direct mode has its own command restrictions | pp. 722–724 |
| Run-Only Interpreter | Load and start BAS programs; disk BAS files can auto-start when clicked | Editing the loaded program is not possible | p. 722 |
| Direct mode | Execute each entered command immediately after Return; call an editor-defined PROCEDURE using GOSUB | No command sequences; constructs requiring multiple commands, e.g. FOR…NEXT, cannot be entered as a direct sequence | pp. 723–724 |
| V3.0 direct-mode history | Recall, edit and rerun the last eight direct commands; Insert toggles insert/overwrite | V2.xx history is documented only as Undo for the last input line | p. 724 |

The book describes the Run-Only Interpreter as freely copyable and suitable for distribution with programs. This is a historical source statement, not a verified license grant for any binary currently available. No binary was redistributed or executed.

The book documents direct-mode recovery through Ed+Return, Esc then Return, or Control+Shift+Alternate. It also describes separate color-register behavior for editor and direct mode: V2.xx may require a color correction on return to the editor, while V3.0 restores the original editor colors. A rare incompletely handled system error may crash during the transition into direct mode (pp. 723–724).

The DSAVE, DLOAD and DLINE procedures printed on pp. 724–725 are user-authored extensions that manipulate the eight-entry history buffer. They are not built-in commands and do not establish a supported history-buffer ABI. Do not implement their BASEPAGE offsets as language or runtime contracts.

Page 725 only introduces the editor chapter's organization: common V2.xx/V3.0 functions are discussed separately from V3.0 additions, with small Replace and cursor-Tab differences noted. Detailed editor semantics require later pages and are not inferred here.



### 4.9 Editor source formats and version-specific file handling

The book distinguishes tokenized program files from ASCII listings (printed pp. 728–729):

| Editor operation | V2.xx | V3.0 | Behavior documented by the book |
|---|---|---|---|
| Program Save/Load | .BAS | .GFA | Saves/loads program code as one block without interpreter checking |
| Save,A / Merge | .LST | .LST | Saves/loads ASCII text; Merge inserts at the cursor and checks merged lines |

Program Save/Load bypasses interpreter checking. Program Load overwrites the current program and, once started, cannot be cancelled except by a system reset. Save,A writes ASCII; Merge inserts the text at the cursor position and marks lines that cannot be interpreted with ==> . The book says Merge rejects lines containing NUL bytes or longer than 255 characters, aborting the merge (pp. 728–729).

Do not treat .BAS and .GFA as interchangeable extensions or assume that either tokenized format is portable across GFA versions. The exact tokenized layout and compatibility rules remain unknown. The 255-character rejection is specifically documented for Merge; it is not established as a universal source-line limit. The editor's horizontal scrolling after 79 visible characters is a separate display behavior (p. 726).

V3.0 editor navigation also permits the right-Tab key to place the cursor beyond the current line end; vertical navigation and selected page/program navigation actions trigger syntax or structure checks (pp. 727–728). These are editor interactions, not lexical rules.



### 4.9 Editor source formats and version-specific file handling

The book distinguishes tokenized program files from ASCII listings (printed pp. 728–729):

| Editor operation | V2.xx | V3.0 | Documented behavior |
|---|---|---|---|
| Program Save/Load | `.BAS` | `.GFA` | Program block; interpreter checking is bypassed |
| Save,A / Merge | `.LST` | `.LST` | ASCII text; Merge inserts at cursor and checks imported lines |

Program Load replaces the current program. Merge inserts text at the cursor and marks invalid lines with the book's `==>` marker. The book says Merge rejects lines containing NUL bytes or longer than 255 characters. This is a Merge-specific constraint, not an established universal source-line limit. The editor's horizontal scrolling after 79 visible characters is separate (pp. 726, 728–729).

The exact tokenized `.BAS` and `.GFA` layouts and their cross-version compatibility remain unknown. Do not assume these formats are interchangeable. V3.0 right-Tab can position the cursor beyond the current line end; navigation can trigger syntax or structure checks (pp. 727–728). These are editor behaviors, not lexical rules.

## 5. Platform API boundary

The same book describes Atari ST-specific facilities in separate chapters:

- BIOS, XBIOS, GEMDOS and system variables.
- GEM/VDI/GDOS and AES libraries.
- Line-A graphics and screen memory.
- Keyboard, mouse, sound, disk and device operations.
- Windows, menus, events and resource files.

These are Atari ST API/runtime profile topics, not automatically language-core semantics. Track their declarations and behavior under [issue #25](https://github.com/vsimko/gfabasic-rust/issues/25) and the API registry. Do not map them directly to Linux/SDL3 without an explicit compatibility design.

### 5.1 Reviewed Atari API areas

| API area | Book coverage | Main concerns | Evidence state |
|---|---|---|---|
| Screen, keyboard, sound, printer, file/device I/O | Chapter 5, pp. 61–154 | Channel semantics versus device APIs; binary/random access; hardware-specific sound and port behavior | First-pass reviewed; detailed signature/image validation pending |
| Line-A, graphics and screen memory | Chapter 9, pp. 251–328 | Atari graphics structures, raster operations, screen addresses and target memory layout | First-pass reviewed; low-level details require page-image and TOS cross-checks |
| Numeric conversion, arrays and memory | Chapters 10–11, pp. 329–392 | Numeric representation, array descriptors, pointers, block moves and memory allocation | First-pass reviewed; storage details are not host ABI |
| Program control, errors, time and keyboard | Chapter 12, pp. 393–452 | CONT/RESUME behavior, error-handler lifecycle, timer units, packed keyboard codes and cooperative callbacks | First-pass reviewed; interpreter/compiler differences remain |
| Alerts, file selector, mouse and joystick | Chapter 13, pp. 453–470 | AES dialogs, selector return states, pointer coordinates and input mode restoration | First-pass reviewed; API behavior must be isolated from core |
| GEM windows | Chapter 14, pp. 471–486 | Four-window table, handle mapping, redraw responsibility and window attributes | First-pass reviewed; WINDTAB layout needs image verification |
| Menus and event monitoring | Chapters 15–16, pp. 487–498 | Menu arrays, AES message/event vectors, event polling and handler reentrancy | First-pass reviewed; button-event parameter interactions need runtime probes |
| GDOS/VDI and fonts | Chapter 17, pp. 499–530 | Optional driver availability, workstation lifecycle, parameter blocks, font metrics and device capabilities | First-pass reviewed; function signatures and font offsets need image/TOS verification |
| AES managers and resources | Chapter 18, pp. 531–584 | Object/resource, event, menu, form, window and shell manager calls | First-pass reviewed; full function/signature validation pending |
| GEM object structures and fields | Chapters 19–20, pp. 585–604 | OBJECT/TEDINFO/ICONBLK/BITBLK/PARMBLK layouts, field widths, flags and target pointers | First-pass reviewed; all offsets/bit masks need image verification |
| BIOS, GEMDOS, VDISYS and XBIOS | Chapter 21, pp. 605–656 | Trap/opcode signatures, parameter blocks, handles, privilege, disk geometry and hardware registers | First-pass reviewed; cross-check against TOS documentation and probes required |
| TOS/system variables | Chapter 22, pp. 657–664 | Absolute addresses, interrupt vectors, screen state and memory bounds | First-pass reviewed; machine/TOS-specific and unsafe by default |

### 5.2 API/runtime classification rules

- GFA BASIC statements that wrap GEM/AES/VDI calls remain part of the Atari API profile even when they use BASIC-like syntax.
- MENU, OPENW, ALERT, FILESELECT, ON MENU, V_OPNWK and related facilities must not be treated as portable core semantics.
- VDI workstation handles and AES window handles are different resource types. Keep their ownership and lifetimes separate.
- AES/VDI parameter arrays and event/message buffers are mutable shared state. Calls can overwrite them; preserve call ordering and buffer lifetime.
- Target 68k pointers, words, longwords, endianness and record offsets must be represented explicitly. Never cast host-native Rust structures to Atari layouts.
- BIOS/GEMDOS/XBIOS calls, supervisor mode, interrupt vectors, absolute system variables, raw disk formatting and direct hardware access are privileged or unsafe surfaces. They require explicit support classification and a virtualized target memory/device model.
- The V2.xx compiler and interpreter are separate execution profiles. Break polling, overflow checks, error support, RESUME, cursor handling and memory allocation can differ.

## 6. Book and diskette corpus observations

The accompanying disk archive contains 162 .LST files plus support files. Its README states that comments were removed to reduce disk usage and that some programs need additional source fragments inserted with the editor's MERGE operation. This means:

- A .LST file is a sample, not necessarily a complete standalone program.
- Missing comments are a property of the distributed sample disk, not a language rule.
- The sample set is useful for discovering command names and idioms.
- A construct seen in an example must be cross-checked against the book's command description before being marked verified.

Do not copy the book PDF or the complete sample disk into this repository. The source register records the access and rights notices. Publish original explanations, metadata, short citations, and independently authored tests only.

## 7. Feature inventory and research backlog

| Area | Current state | Next evidence to extract |
|---|---|---|
| Character encoding and byte decoding | Not yet assessed | Editor/source file and tokenized file sections |
| Identifiers and case sensitivity | Partially described in examples | Variable chapter and lexical notes |
| Keywords and abbreviations | Abbreviations are documented; complete list exists in appendices | Command abbreviation list and parser behavior |
| Numeric/string/Boolean literals | Not yet assessed | Literal and expression sections |
| Operators and precedence | First-pass reviewed | Chapter 8, pp. 229–249; precedence and edge cases need normalization and tests |
| Numeric semantics | First-pass reviewed | Chapters 8, 10 and 24; reconcile representation and compiler/interpreter differences |
| String semantics | First-pass reviewed | Chapter 7, pp. 221–228; runtime representation also described in chapter 24 |
| Arrays | First-pass reviewed | Chapter 11, pp. 341–353; descriptor/layout validation pending |
| Memory and pointers | First-pass reviewed | Chapter 11, pp. 354–392; target memory model validation pending |
| Control flow | Partially assessed | Chapter 6, pp. 155–220 |
| I/O and files | Partially assessed | Chapter 5, pp. 61–154 |
| Graphics | Platform-specific; not language core | Chapter 9, pp. 251–328 |
| Error handling | First-pass reviewed | Chapter 12, pp. 393–452; interpreter/compiler behavior differs |
| Editor and compiler behavior | Partially assessed | Chapter 24, pp. 701–793 |
| Built-in command/function catalogue | Indexed in first pass; signatures not yet normalized | Appendix and alphabetical command list, pp. 742–793 |

## 8. Required feature record

Each detailed feature entry must include:

1. Stable feature ID and canonical name.
2. Dialect, version, and API profile.
3. Syntax signature, including optional parts.
4. Argument and result types.
5. Semantics, side effects, and edge cases.
6. Source title, edition, platform, printed page and section.
7. Evidence status and any OCR/manual verification notes.
8. Version differences and known conflicts.
9. Implementation status.
10. Independently authored positive and negative conformance tests.

## 9. Open questions

1. Which exact V2.xx release(s) are represented by the book's V2.xx references?
2. Does the compiler's first dialect target Atari ST GFA BASIC 3.0, Amiga GFA BASIC, or a deliberately bounded common subset?
3. What is the exact source encoding and treatment of Atari character-set bytes?
4. Are source line numbers accepted by the interpreter, or are the editor's line numbers internal metadata only?
5. What are the exact identifier grammar, case rules, and name collision rules?
6. What is the complete precedence and associativity table?
7. What are the exact numeric representations, ranges, rounding, and overflow behavior?
8. What is the exact meaning of VAR parameters and variable aliasing?
9. What are the exact semantics of each loop form and loop exit?
10. How are errors trapped, resumed, and propagated?
11. Which editor/compiler differences affect source acceptance or runtime behavior?
12. Which commands are core language features and which are Atari ST library/API calls?

## 10. Change policy

This document is a research draft. Promote a rule to an accepted compiler requirement only after:

- the relevant source passage is inspected and its edition/version is recorded;
- OCR-sensitive names and signatures are checked against the page image;
- conflicting platform/version sources are compared;
- an independently written conformance test or a documented reason for deferring the test is added;
- the relevant issue and decision record are updated.

Related work: [#3 dialect/version policy](https://github.com/vsimko/gfabasic-rust/issues/3), [#19 feature inventory](https://github.com/vsimko/gfabasic-rust/issues/19), [#25 Atari ST API reference](https://github.com/vsimko/gfabasic-rust/issues/25), [#42 conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42).
