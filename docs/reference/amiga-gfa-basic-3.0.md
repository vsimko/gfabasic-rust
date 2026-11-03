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