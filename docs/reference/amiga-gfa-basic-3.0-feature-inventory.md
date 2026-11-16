---
type: "Reference"
title: "Amiga GFA BASIC interpreter 3.0 — feature inventory (working draft)"
---

# Amiga GFA BASIC interpreter 3.0 — feature inventory (working draft)

- Status: Initial inventory; incomplete and not an implementation contract.
- Source scope: Amiga GFA BASIC interpreter 3.0; this is a historical source-specific inventory, not the selected project target.
- Current normative target: Amiga GFA BASIC interpreter 3.5 (see [ADR 0009](../architecture/adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md)).
- Primary source: `AM-BOOK-FR-1989`, *Le livre du GFA Basic Amiga*, Bleek, Hecht, and Litzkendorf, French translation by Christian Frey, Data Becker / Micro Application, ©1989, ISBN 2-86899-207-2.
- Companion source: `AM-BOOK-DE-1990`, *Das große GFA-BASIC-Buch zum Amiga*, Data Becker, 1990, ISBN 3-89011-399-0. This edition describes the V3.03 interpreter and compiler material; do not merge its version-specific claims into the 3.0 baseline.
- Related policy: [DIALECTS.md](../architecture/DIALECTS.md), [ADR 0009](../architecture/adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md), superseded [ADR 0008](../architecture/adr/0008-initial-source-dialect-amiga-gfa-basic-3.0.md).
- 3.5 source comparison: [primary reference notes](../research/amiga-gfa-basic-35-primary-reference.md) and [3.5 addendum inventory](../research/amiga-gfa-basic-35-addendum-inventory.md).
- Detailed research notes: [amiga-gfa-basic-3.0.md](amiga-gfa-basic-3.0.md).

This inventory records what the 3.0 sources say. It must not be used as a 3.5 language specification: every 3.0-only rule remains an inherited candidate until confirmed against the selected 3.5 interpreter manual.

## Evidence conventions

- **Text-backed**: the identified Amiga source contains the statement at the cited printed page. OCR-derived punctuation/signatures may still need page-image inspection.
- **Needs visual check**: OCR, symbols, exact signature, or translation may be ambiguous.
- **Topic identified**: the chapter or example establishes that an area exists, but not its complete syntax or semantics.
- **Unresolved**: available material does not establish the rule.
- **Observed** is reserved for reproducible tests against a named interpreter build. No interpreter observations are claimed in this inventory.

Do not transfer Atari ST rules to Amiga. The Atari ST V2.xx/V3.0 books and manual remain a separate research-only comparison profile. Do not treat the 1990 Amiga V3.03 book, 3.04 environment section, or 3.52 catalogue labels as evidence that a feature belongs to interpreter 3.0.

## Feature inventory

| Feature group | Current Amiga 3.0 findings | Evidence / status | Remaining work |
|---|---|---|---|
| Source organization and editor model | The book describes a structural editor in which each command, statement, or function occupies its own program line; structured blocks are indented. | Text-backed, introductory chapter; editor behavior must not automatically become a compiler lexical restriction. | Establish whether physical line layout is a language requirement or editor convention; inspect line continuation and separators. |
| Syntax notation and abbreviations | Square brackets denote optional elements; braces can denote accepted abbreviations; ellipses represent statement sequences. The interpreter expands recognized abbreviations. | Text-backed, introductory chapter. Complete abbreviation table not identified. | Extract the full abbreviation rules and distinguish parser aliases from editor completion. |
| Keywords and identifiers | The book explicitly says Amiga GFA BASIC has no reserved words; variables may share spellings with instruction names. Labels end in a colon. | Text-backed, printed pp. 4-43–4-45. Label/command ambiguity is described. | Visually verify identifier character classes, case behavior, label grammar, and ambiguity resolution. |
| Variable naming and type suffixes | Suffixes listed: `%` integer, `|` byte, `&` word, `!` Boolean, `$` string, `#` real; `#` may be omitted for real variables. Names start with a letter; OCR text for subsequent characters is damaged. | Text-backed, printed pp. 4-43–4-45; character repertoire needs visual check. | Verify exact allowed characters, suffix binding, default types, and declaration precedence. |
| Numeric representation | Real variables: 8 bytes and approximately 13 significant decimal digits. Integer variables: signed 32-bit range −2,147,483,648 to 2,147,483,647; 1-byte and 2-byte integer types are also mentioned. | Text-backed, printed pp. 4-38–4-40. | Verify rounding, overflow, conversion, and byte/word signedness with page images and interpreter probes. |
| Boolean values | False is 0; true is −1, described as all bits set in a two's-complement long word. | Text-backed, printed pp. 4-38–4-40. | Verify Boolean conversion and interaction with relational/logical operators. |
| Arrays | Arrays are a major language topic in the book (fundamentals and a later array chapter); introductory text mentions arrays among the language constructs. | Topic identified; detailed bounds, dimensions, indexing, allocation, and string-array semantics not yet consolidated. | Extract array declarations, dimensions, lower bounds, allocation, indexing, and errors with printed-page citations. |
| Literals and expressions | The book covers comparisons, precedence, numeric/string expressions, and conversion. Functions may appear in assignments, output expressions, tests, and as dummy arguments to `VOID`. | Topic identified in introductory material and chapter map. | Extract complete literal grammar, precedence table, evaluation order, and coercion rules. |
| Arithmetic and comparison operators | Arithmetic, comparisons, numeric/trigonometric functions, and precedence are covered in the language-core chapters. The operator hierarchy is now transcribed from the visually checked printed p. 3-2, including separate string-comparison and numeric-comparison rows. | Text-backed, printed p. 3-2; token spellings are preserved exactly as printed and remain source-ambiguous pending 3.5 reconciliation. | Verify token spellings and associativity against 3.5 evidence; extract integer/real behavior and edge cases. |
| Bit operations | `BSET`, `BTST`, `BYTE`, `CARD`, `EQV`, `IMP`, shift/rotate families, `SWAP`, `WORD`, and `XOR` appear in the OCR inventory. | Text-backed summaries, printed pp. 8-189–8-195; shift/rotate suffix glyphs and some signatures need visual check. | Verify exact spellings, widths, signedness, parameter order, and edge cases. |
| Control flow and loops | Four loop families are documented: `FOR...NEXT`, `DO...LOOP`, `REPEAT...UNTIL`, and `WHILE...WEND`. `REPEAT...UNTIL` tests after the body; nested loop closers must reverse opening order; `EXIT IF` is identified as a conditional loop exit. | Text-backed, printed pp. 4-45–4-50. | Extract exact loop grammar, step/default rules, condition semantics, exit placement, and error behavior. |
| Branching and labels | `GOTO` jumps to a label; labels end with a colon. The book describes `ON...GOSUB` procedure forms and says `GOSUB` can be omitted in a documented form. | Text-backed introductory/structure notes; exact ON-family grammar remains incomplete. | Extract `IF/ELSE/ENDIF`, `ON`, `GOTO`, `GOSUB`, `RETURN`, `EXIT`, and label edge cases. |
| Procedures and user functions | Procedures use `PROCEDURE`; user functions use `DEFFN`. The book illustrates procedure calls by name or with `@`, and function calls using `FN` or `@`. | Text-backed, printed pp. 4-43–4-45. | Extract parameter modes, typing, scope, recursion, return values, and lifetime. |
| String operations | A dedicated string operations/formatting chapter exists. | Topic identified, printed pp. 167–174. | Extract string operators, functions, indexing/slicing, encoding, length, and formatting semantics. |
| Keyboard and text input | `FORM INPUT`, `INKEY$`, `INPUT`, `INPUT$(count[, #channel])`, and `LINE INPUT` are described. | Text-backed summaries, printed pp. 5-61–5-67; exact optional punctuation should be visually checked. | Verify prompt, delimiter, EOF, channel, and maximum-length behavior. |
| File and device I/O | `OPEN` modes O/I/A/U/R; channel numbers 0–99; up to 100 open files; `BGET`, `BPUT`, `CLOSE`, `EOF`, `FIELD`, `GET`, `PUT`, `LOC`, `LOF`, `RECORD`, `RELSEEK`, `SEEK`, `STORE`, and `RECALL` are identified. | Text-backed summaries, printed pp. 5-87–5-97. | Verify record layout, EOF edge cases, file-pointer semantics, error codes, and host mapping. |
| Amiga device interfaces | The book lists `CON:`, `PRT:`, `PAR:`, `SER:`, `COM:`, `PIPE:`, `SPEAK:`, `RAW:`, `NEWCON:`, and `AUX:`. | Text-backed, printed pp. 5-92–5-93; Amiga-specific API/device behavior. | Identify which are interpreter-supported, OS-provided, or optional; document capability and failure behavior. |
| Random functions | `RAND`, `RANDOM`, `RANDOMIZE`, and `RND` are present in the OCR inventory. The book distinguishes ranges/result types and deterministic reseeding. | Text-backed, printed pp. 8-196–8-197; `RANDOM` spelling/signature and edge cases need visual check. | Verify signatures, ranges, seed behavior, and generator reproducibility. |
| Date/time and system functions | Date/time, errors, multitasking, debugging, and program control are separate topics in the book. | Topic identified, printed pp. 301–330. | Extract each built-in and separate language core from Amiga OS/editor/runtime facilities. |
| Memory and address operations | Addresses are described essentially as 32-bit integers; memory, user memory, pointers, and Exec library have dedicated coverage. | Topic identified; exact address/pointer semantics unresolved. | Extract PEEK/POKE-like operations and layouts; do not map Amiga addresses to Linux pointers. |
| Data and number conversion | A dedicated data-conversion and number-systems chapter exists. | Topic identified, printed pp. 247–258. | Extract conversion functions, base notation, overflow, and representation. |
| Graphics and drawing | The book covers graphics definitions, primitives, lines/points, operations, graphics memory, and animation. Examples show `ELLIPSE`, `GRAPHMODE`, `OPENW`, `CLOSEW`; a separate 1990 Amiga edition corroborates `BOX`, `LINE`, `FILL`, `PLOT`, `DRAW TO` and GRAPHMODE values 1/3. | Primary 3.0 topic map and examples; 1990 V3.03 source is a separate corroborating edition, not silently merged. | Extract complete 3.0 command signatures, coordinate/state rules, resource ownership, and version applicability. |
| Windows and screens | Dedicated window and screen-management chapters exist; an example opens/closes a BASIC window. | Topic identified, printed pp. 339–352; full signatures not extracted. | Extract creation, selection, close, dimensions, display modes, ownership, and error semantics. |
| Menus and events | Dedicated menu and BASIC event-control chapters exist. A separately published 1993 GFA 3.0 example demonstrates MENU string arrays and ON MENU GOSUB. | Manual topic identified, printed pp. 353–368; secondary example is corroborating only. | Extract exact menu/event signatures, return values, callback ordering, and lifecycle from the Amiga 3.0 manual. |
| Errors and diagnostics | Appendix B lists GFA BASIC error messages; program-control/error chapter exists. | Topic identified; full error catalogue and trigger semantics not inventoried. | Extract error names/codes, trapping behavior, source/runtime context, and reproducibility. |
| Compiler-specific behavior | The 1990 German edition has a compiler chapter and distinguishes interpreter/compiler versions. | Separate V3.03/compiler material; not the selected interpreter 3.0 language baseline. | Keep compiler extensions and limitations in a separate versioned specification. |

## Cross-platform and version guardrails

- Atari ST V2.xx/V3.0 is research-only. Its additional loop forms (such as DO-WHILE/DO-UNTIL and LOOP-WHILE/LOOP-UNTIL) are not established for Amiga 3.0 and must not be imported.
- The French 1989 book documents Amiga interpreter 3.0 and historical notes for later releases; its 3.04 environment section is not evidence that every 3.04 feature belongs to 3.0.
- The German 1990 book identifies V3.03 interpreter/compiler content. Cite it separately as `AM-BOOK-DE-1990`.
- Catalogue labels for Amiga 3.52 do not prove the interpreter component version or any 3.52 language feature.

## Next extraction priorities

1. Extract the complete alphabetical command/function index if an applicable Amiga 3.0 source with that appendix can be located.
2. Complete the core syntax, literals, precedence, arrays, and type-conversion tables from the Amiga 3.0 chapters.
3. Finish statement and procedure/function signatures with printed-page citations.
4. Extract strings, data conversion, date/time, and error catalogue entries.
5. Continue I/O and memory semantics with exact edge cases.
6. Extract graphics, windows, screens, menus, and event signatures separately as Amiga APIs.
7. Manually compare OCR-sensitive syntax tables and symbols against page images before promoting them to verified registry entries.
8. Build original, minimal candidate fixtures only when their syntax and expected behavior are tied to the exact source edition.

## Relationship to issues

- [#19](https://github.com/vsimko/gfabasic-rust/issues/19): feature inventory and compatibility matrix.
- [#20](https://github.com/vsimko/gfabasic-rust/issues/20): lexical/syntax/type/expression reference.
- [#21](https://github.com/vsimko/gfabasic-rust/issues/21): statements and control flow.
- [#22](https://github.com/vsimko/gfabasic-rust/issues/22): built-in and mathematical functions.
- [#23](https://github.com/vsimko/gfabasic-rust/issues/23): file, disk, data, and resource operations.
- [#24](https://github.com/vsimko/gfabasic-rust/issues/24): Amiga platform API reference.
- [#42](https://github.com/vsimko/gfabasic-rust/issues/42): independently authored conformance corpus.

This file is a living research index. It intentionally records missing evidence rather than guessing. Acceptance requires further extraction, visual review where needed, and tests; the inventory is not a claim of compiler support.
