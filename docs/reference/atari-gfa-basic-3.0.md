# Atari ST GFA BASIC V2.xx / V3.0 — preliminary language specification

- Status: **Research draft; not an accepted compiler dialect**
- Platform: Atari ST
- Source coverage: GFA BASIC V2.xx and V3.0, as described by the sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (1988)
- Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3), [issue #19](https://github.com/vsimko/gfabasic-rust/issues/19), [issue #25](https://github.com/vsimko/gfabasic-rust/issues/25)
- Source register: [ATARI-GFA-BOOK-1988](../reference-sources.md#atari-gfa-book-1988)

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

The book describes these V3.0 declaration families (printed pp. 189–192):

| Declaration | Documented type category | Storage width stated by the book | Status |
|---|---|---:|---|
| DEFBIT | Boolean | Not stated in the inspected declaration passage | Verified name/category |
| DEFBYT | Integer | 1 byte | Verified |
| DEFWRD | Integer | 2 bytes | Verified |
| DEFINT | Integer | 4 bytes | Verified |
| DEFFLT | Floating point | 8 bytes | Verified |
| DEFSTR | String | Variable-sized/representation not specified here | Verified name/category |

The declaration commands assign types to variables by matching name prefixes against a definition string. The book gives examples of individual letters, ranges, and comma-separated prefixes. A later explicit type suffix on a variable takes precedence over the global prefix declaration.

The book also documents these type suffixes in the declaration discussion:

- % — 4-byte integer.
- & — 2-byte integer.
- ! — Boolean.
- # — 8-byte floating point.
- $ — string.

The 1-byte integer suffix is OCR-sensitive in the available text and must be checked against the page image before being recorded as a definitive lexical rule. Do not infer the suffix from a different BASIC dialect.

The exact numeric range, signedness, overflow behavior, conversion rules, string representation, default typing rules, and array element layout remain **not yet assessed**.

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

## 5. Platform API boundary

The same book describes Atari ST-specific facilities in separate chapters:

- BIOS, XBIOS, GEMDOS and system variables.
- GEM/VDI/GDOS and AES libraries.
- Line-A graphics and screen memory.
- Keyboard, mouse, sound, disk and device operations.
- Windows, menus, events and resource files.

These are Atari ST API/runtime profile topics, not automatically language-core semantics. Track their declarations and behavior under [issue #25](https://github.com/vsimko/gfabasic-rust/issues/25) and the API registry. Do not map them directly to Linux/SDL3 without an explicit compatibility design.

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
| Operators and precedence | Contents identify operator and comparison sections | Chapter 8, pp. 229–249 |
| Numeric semantics | Not yet assessed | Numeric functions, precision, overflow and conversion |
| String semantics | Not yet assessed | Chapter 7, pp. 221–228 |
| Arrays | Not yet assessed | Chapter 11, pp. 341–353 |
| Memory and pointers | Not yet assessed | Chapter 11, pp. 354–392 |
| Control flow | Partially assessed | Chapter 6, pp. 155–220 |
| I/O and files | Partially assessed | Chapter 5, pp. 61–154 |
| Graphics | Platform-specific; not language core | Chapter 9, pp. 251–328 |
| Error handling | Not yet assessed | Chapter 12, pp. 393–452 |
| Editor and compiler behavior | Partially assessed | Chapter 24, pp. 701–793 |
| Built-in command/function catalogue | Not yet assessed | Appendix and alphabetical command list, pp. 742–793 |

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
