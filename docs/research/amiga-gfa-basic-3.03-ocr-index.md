# German Amiga GFA BASIC book: OCR index and edition notes

- **Status:** Research index; OCR-derived details require page-image review.
- **Source ID:** `AM-BOOK-DE-1990`.
- **Tracking:** [issue #65](https://github.com/vsimko/gfabasic-rust/issues/65), [issue #63](https://github.com/vsimko/gfabasic-rust/issues/63).
- **OCR PDF:** [Das große GFA-BASIC-Buch zum Amiga (OCR)](https://retro-commodore.eu/files/downloads/amigamanuals-xiik.net/eBooks/Das%20Grosse%20Buch%20zu%20GFA-Basic%20Amiga%20%28OCR%29%20-%20eBook-DE.pdf).

## Bibliographic identity

The searchable PDF front matter identifies:

- Title: *Das große GFA-BASIC-Buch zum Amiga*.
- Authors: Wolf-Gideon Bleek, Martin Hecht, and Uwe Litzkendorf.
- Publisher: Data Becker GmbH, Düsseldorf.
- Copyright/publication year printed in the PDF: 1990.
- ISBN: 3-89011-399-0.
- The PDF contains an OCR text layer and preserves printed page numbers in its contents and page headers.

A contemporary Amiga-News announcement dated 2026-09-03 says Thomas Werner is offering the 1989 German book and its French translation as free PDFs with the authors' permission: [announcement](https://amiga-news.de/en/standard/?frm_perPage=21&frm_start=42). The PDF's own copyright page states that reproduction and distribution require written permission. Treat the announcement as evidence about the offered scan's access permission only; it does not grant this repository permission to redistribute the book, scans, OCR text, or substantial extracts.

A bibliographic listing distinguishes ISBN 3-89011-399-0 (508 pages) from ISBN 3-89011-276-5 (430 pages): [bibliographic comparison](https://diebuchsuche.at/buch-isbn-9783890113999.html). The PDF front matter matches ISBN 3-89011-399-0. Do not merge it with the separate French translation or the other German ISBN record.

## Contemporary evidence for separate Amiga interpreter/compiler versions

A March 1991 *Amiga Magazine* issue (anno 4, no. 21) contains an ALEX Computers price list on printed page 17. The list names **GFA BASIC Interpreter 3.51** and **GFA BASIC Compiler 3.52** as separate products: [issue PDF](https://www.amigamagazine.info/pdf/AM_21.pdf).

This is contemporaneous commercial-catalogue evidence, not a manual and not a statement about the compiler's language semantics. It establishes that the two product labels were distinguished in a March 1991 listing. It does **not** prove that the 1990 German book's compiler chapter documents compiler 3.52, nor that interpreter 3.51 and compiler 3.52 share every language/API behavior.

The Amiga version table on the GFA-Basic FAQ also lists Amiga editor/compiler/linker 3.52, but the site states that it focuses on Atari ST support and that its non-Atari information is informational only and may be questionable: [FAQ](https://gfabasic.net/htm/gfa_faq.htm). Keep this as secondary corroboration, not normative evidence.

The magazine PDF's copyright page reserves reproduction and translation rights. This repository records only the bibliographic identity and product-version distinction; it does not reproduce the catalogue page or magazine contents.

## Version applicability: unresolved discrepancy

The front matter does not support a single unambiguous version label:

- The foreword says the book documents all commands and functions of the Amiga GFA-BASIC **version 3.0** and describes the compiler as newly released.
- The introductory section 1 says the book arranges approximately 360 commands/functions provided by the GFA **V3.03 interpreter**.
- The title page itself says “Bis Version 3.5 und Compiler 3.5”, which may reflect later additions or a later state of the book; it must not be treated as proof that every chapter describes 3.5 semantics.

Until the page images and edition history are checked, use the conservative applicability label **Amiga GFA BASIC 3.x; 1990 German edition; version coverage disputed between 3.0 and 3.03 in the front matter**. Do not silently label the whole book as 3.52 evidence. Keep it separate from the French 1989 book whose preface identifies interpreter 3.0.

## Provisional printed-page map

The following map is transcribed from the PDF's OCR table of contents. It is a navigation aid, not a verified command index. Printed page numbers are shown; PDF viewer page numbers differ.

| Topic | Printed pages / sections |
|---|---|
| About the book and Amiga introduction | 13–18 |
| GFA BASIC introduction | 21–28 |
| BASIC foundations: variables, matrices, loops, comparisons | 29–65 |
| Input/output commands | 75–140 |
| - Data input/output and screen operations | 75, 83, 89 |
| - Disk and file handling; random-access files | 92, 104, 115 |
| - Port I/O, AmigaDOS library, printer, sound | 119, 120, 132, 140 |
| Program structure | 145–186 |
| - Loops, conditional branches, scope and variable declarations | 145, 149, 167, 170 |
| - Subprograms and assembler/C/PRG calls | 173, 186 |
| Text operations | 193–199 |
| Operators and numeric functions | 201–223 |
| Graphics | 225–268 |
| - Graphics definitions, object graphics, line/point graphics, operations | 225, 238, 246, 255 |
| - PUT-string and screen-memory organization; object animation | 262, 265, 268 |
| Number-system conversion | 279–280 |
| Arrays, memory, pointers, and Amiga Exec library | 291–317 |
| Program control, error handling, multitasking, debugging | 331–355 |
| Program/user interaction | 363 |
| Window and screen programming | 375–388 |
| Menu programming | 389 |
| Event handling | 395 |
| GFA compiler | 405–434 |
| Vector and matrix calculations | 451–464 |
| Additional editor convenience commands | 477–481 |
| Appendices and alphabetical GFA BASIC command index | 483–498 |

The OCR table of contents has visibly corrupted words and punctuation. Verify each topic heading and printed page against the PDF page image before citing an individual rule.

## Research workflow

1. Preserve the original PDF outside this repository; do not commit it or its OCR text.
2. Use the printed-page map to locate a command or topic.
3. Compare OCR against the corresponding page image before transcribing syntax, punctuation, argument markers, tables, or code.
4. Record exact printed page and section, edition, version applicability, and confidence for each claim.
5. Separate language-core constructs from Amiga API calls. Do not assume that every command in a platform chapter belongs to the Amiga profile.
6. Compare the German edition with the French 1989 edition only as a cross-edition check; never merge their page references or version claims.
7. Record unresolved OCR or edition conflicts as unknown/disputed rather than choosing a plausible reading.

## Current limitations

- This pass verifies front-matter metadata and provides a first OCR-derived chapter map.
- The actual page images have not yet been checked for command signatures or OCR-sensitive syntax.
- The book's internal 3.0/3.03/“up to 3.5” wording requires edition-history investigation.
- No claim is made about Amiga 3.52 conformance or Linux runtime implementation.
