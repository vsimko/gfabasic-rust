# GFA BASIC reference sources

This register identifies candidate sources for language and API research. A listed source is not automatically authoritative for every dialect or version. No feature should be promoted to “verified” until the relevant edition, platform, and page/section have been checked.

## Evidence status

- **Verified**: the exact source passage and applicability have been checked.
- **Inferred**: the conclusion is plausible but not explicit in the source.
- **Disputed**: sources or observed implementations conflict.
- **Unknown**: evidence has not been checked.

## Candidate Atari ST sources

| ID | Source | Applicability | Evidence status | Notes |
|---|---|---|---|---|
| ST-MANUAL-36 | *GFA BASIC 3 - User Manual*, Version 3 interpreter user manual | Atari ST GFA BASIC 3; exact edition and publication metadata still to be confirmed | Candidate primary manual; not yet page-verified | A publicly accessible scan is indexed by AtariMania: [PDF record](https://www.atarimania.com/st/files/GFA_BASIC_Version_3_Interpreter_User_Manual.pdf). Check title page, edition, version, and relevant pages before citing normative rules. |
| ST-REF-35 | *GFA-BASIC 3.5 command reference* | Atari ST GFA-BASIC 3.5 | Secondary/primary status to be confirmed | [Available PDF](https://info-coach.fr/atari/documents/general/gfabasic.pdf). The indexed introduction identifies it as a command reference for GFA-BASIC 3.5. Verify source provenance and page references. |
| ST-COMPENDIUM | GFA-Basic Compendium / hypertext documentation | Atari ST; exact versions per topic | Secondary reference | [GFA-Basic FAQ and resource site](https://www.gfabasic.net/htm/gfa_faq.htm). Useful for finding references and historical context; do not substitute it for the relevant manual when establishing syntax or semantics. |

## Candidate Amiga sources

No Amiga manual has yet been identified and verified in this initial register. The Atari ST manual must not be treated as sufficient evidence for Amiga-specific commands or behavior.

## Historical/version context

The GFA-Basic FAQ reports separate last-known releases for Atari ST/TT and Amiga, including Atari ST/TT 3.6 and Amiga 3.52. This is secondary historical context, not a language specification: [GFA-Basic FAQ](https://www.gfabasic.net/htm/gfa_faq.htm).

The FAQ also discusses differences among the Atari ST, Amiga, DOS, and Windows versions. These differences reinforce the need to keep dialect/version identity explicit. Confirm every detail against the relevant original manual.

## Citation convention

For every language or API rule, record:

`[source ID], [platform/dialect], [version/edition], p. [page] or §[section]`

Also record a short original explanation, evidence status, and any uncertainty. Do not reproduce substantial copyrighted source text.

## Research tasks

- Confirm the title-page metadata and pagination of each Atari ST manual.
- Identify the exact Amiga GFA BASIC 3.52 manual and its provenance.
- Determine which sources apply to interpreter behavior, compiler behavior, and runtime/API behavior.
- Record discrepancies between manual editions and observed interpreter/compiler behavior.
- Review copyright and redistribution status before adding source files to this repository.


## Amiga-specific candidate sources (research started 2026-11-03)

Initial Amiga 3.0 extraction notes: [Amiga GFA BASIC 3.0 reference research](reference/amiga-gfa-basic-3.0.md).

| ID | Source | Applicability | Evidence status | Notes |
|---|---|---|---|---|
| AM-BOOK-1989 | *Das große Buch zu GFA BASIC Amiga* / *Das große GFA-BASIC zum Amiga*, attributed to Wolf-Gideon Bleek, Martin Hecht, and Uwe Litzkendorf | Amiga GFA BASIC; likely 3.x, exact edition/version coverage to verify | Strong bibliographic candidate; primary text not yet inspected | Amiga-News reported on 2026-09-03 that Thomas Werner offers the 1989 book as a free PDF with the authors' permission. Bibliographic listings are inconsistent: ISBN 3-89011-276-5 is listed as 430 pages (1990/1989 variants), while ISBN 3-89011-399-0 is listed as a 508-page edition (1990). Verify title page, copyright page, and table of contents in the authorized scan before choosing canonical metadata. [Amiga-News announcement](https://www.amiga-news.de/en/standard/?frm_perPage=21&frm_start=42); [bibliographic comparison](https://diebuchsuche.at/buch-isbn-9783890113999.html); [catalog record](https://www.retrobug.org/library/med0142). |
| AM-BOOK-FR | *Le livre du GFA-BASIC 3.0 AMIGA* (French translation) | Amiga GFA BASIC 3.0; exact edition to verify | Candidate; reported as authorized free PDF | Amiga-News reports an authorized PDF offer alongside the German book. Useful for cross-checking 3.0 terminology, but not a substitute for later-version references. [Amiga-News announcement](https://www.amiga-news.de/en/standard/?frm_perPage=21&frm_start=42). |
| AM-FAQ-HIST | GFA-Basic FAQ, historical version table and platform notes | Secondary historical context for Amiga 3.52 and cross-platform lineage | Secondary; not normative | Lists Amiga editor/compiler/linker as 3.52 and describes Amiga as a port of Atari ST with file formats nearly identical. Treat this as a research lead only; verify every language claim against Amiga manuals and observed releases. [FAQ](https://gfabasic.net/htm/gfa_faq.htm). |
| AM-ADF-352-D1 | GFA-BASIC v3.52 (1990), GFA Systemtechnik, German, disk 1 of 2 | Amiga GFA BASIC 3.52 | Archive catalogue entry; binary not inspected | Planet Emulation catalogue lists a 256 KB ADF and checksum `be832a95`. Do not redistribute the image; use only where lawful and record provenance. [Catalogue entry](https://www.planetemu.net/en/rom/commodore-amiga-applications-adf/gfa-basic-v3-52-1990-gfa-systemtechnik-de-disk-1-of-2-wb). |
| AM-ADF-352-D2 | GFA-BASIC v3.52 (1990), GFA Systemtechnik, German, disk 2 of 2 | Amiga GFA BASIC 3.52 | Archive catalogue entry; binary not inspected | Planet Emulation catalogue lists a 234 KB ADF and checksum `5f75a9cc`. Do not redistribute the image; use only where lawful and record provenance. [Catalogue entry](https://www.planetemu.net/en/rom/commodore-amiga-applications-adf/gfa-basic-v3-52-1990-gfa-systemtechnik-de-disk-2-of-2). |
| AM-ADF-CATALOG | Planet Emulation Amiga application archive index | Amiga GFA BASIC releases 3.00, 3.02, 3.041, 3.51, 3.52 and 3.5U | Secondary archive catalogue | The index exposes several release labels and dates. Verify version strings from the software itself before treating archive labels as definitive. [Amiga application index](https://www.planetemu.net/en/roms/commodore-amiga-applications-adf?page=G). |

### Aminet survey leads

The Aminet `dev/basic` category is a relevant discovery point: [Aminet dev/basic](https://aminet.net/dev/basic/). Initial candidate filenames mentioned for follow-up include `GFA_Enforcer.txt`, `GFA_Gadgets.lha`, `GFA_Menus.lha`, and `GuiGfxDevBas.lha`. These entries have not yet been independently re-checked in the live index or their archive contents; classify them as unverified leads, not normative language documentation. Third-party libraries and wrappers must not be mistaken for built-in GFA BASIC commands.

### Initial findings and cautions

- The Amiga target has a promising Amiga-specific book and a reported authorized PDF distribution; this is a stronger lead than the generic Atari ST books previously considered.
- The book's bibliographic metadata is inconsistent across catalogues. Do not collapse the 430-page ISBN 3-89011-276-5 record and 508-page ISBN 3-89011-399-0 record until the actual scan is inspected.
- A secondary FAQ lists Amiga 3.52 for editor, compiler, and linker, but explicitly says it primarily supports Atari ST and that non-ST information is informational. It is historical context only.
- Archive catalogues list multiple Amiga releases, including 3.52 on two disks. Their contents and actual version strings remain to be verified.
- No copyrighted scans, disk images, ROMs, or substantial verbatim extracts should be committed. Record lawful access, metadata, checksums, and concise source-backed findings instead.


| AM-REVIEW-1988 | Happy Computer 12/1988, “GFA-Basic Amiga im Test: Programmierter Wahnsinn” | Early Amiga GFA BASIC 3.x product and manual review | Contemporary secondary source; strong historical evidence, not a specification | Reports an Amiga package with a German manual of about 420 pages and about 350 commands; notes one command per program line and named procedures called with CALL. These are leads to verify against the manual, not normative rules. [Article](https://www.stcarchiv.de/happy-computer/1988/12/gfa-basic-amiga). |
| AM-EXAMPLES-BOOK | *Computers and Chaos (Amiga edition)*, GFA BASIC listings | Amiga GFA BASIC example programs | Published example corpus; exact GFA release used by the book should be checked | The online edition identifies GFA BASIC listings saved as ASCII and includes examples such as `OPENS`, `TITLES`, `COLOR`, `PCIRCLE`, `SETCOLOR`, and a `PROCEDURE` using `MOUSEK`. Useful as a source/example corpus, not a complete command reference. [Online edition](https://amigasourcecodepreservation.gitlab.io/computers-and-chaos-amiga-edition/). |
| AM-PDK-1993-94 | Amiga Future PDK 10 (5/1993) and PDK 11 (5/1994) contents listings | Amiga GFA BASIC source examples and reusable routines | Contemporary archive catalogue; package contents still to be acquired/checked | PDK 10 lists GFA BASIC programs and source, including CrissCross, Druck, IconTyp, Kalender, Othello, Requester (req.library), and Wortzähler. It states that parts may be incorporated into own programs. Verify each listing and its terms against the actual archive. [PDK contents](https://www.amigafuture.de/app.php/kb/viewarticle?a=2972). |
| AM-AMINET-GFALIST | `gfalist.lha`, Aminet `dev/basic` | Tool for reading tokenized GFA BASIC programs | Archive index entry; binary and behavior not inspected | An Aminet upload listing describes it as “Reads tokenized GFA BASIC ver...”. This is a potentially useful reverse-engineering aid for tokenized files, not evidence of language semantics. [Aminet upload listing](https://www.amiga-news.de/en/standard/?frm_pageType=news&frm_perPage=21&frm_start=7644). |

### Research note: what is now known

The contemporary December 1988 review is a useful early snapshot: it reports a roughly 420-page German Amiga manual, about 350 commands, one command per source line, and named procedures. A later Amiga-specific book is also reported as available in an authorized PDF distribution (see AM-BOOK-1989). These are distinct sources and likely cover different releases; do not merge their command inventories without version tagging.

The online *Computers and Chaos* Amiga edition and the PDK listings provide promising source-code corpus leads. In particular, the PDK contents show examples using GFA BASIC and some third-party libraries. These must be classified individually so that library calls are not mistaken for built-in language statements.

Aminet discovery is currently partial: the `gfalist.lha` entry is confirmed by an Aminet upload listing, while the earlier candidate filenames (`GFA_Enforcer.txt`, `GFA_Gadgets.lha`, `GFA_Menus.lha`, `GuiGfxDevBas.lha`) remain unverified leads until their live package records and contents are checked.


| AM-BOOK-FR-1989 | *Le livre du GFA Basic Amiga*, Bleek, Hecht, Litzkendorf; French translation by Christian Frey, ©1989 | Explicitly GFA BASIC Amiga interpreter 3.0 | Primary published technical book; OCR/PDF inspected | A full searchable PDF is available at [PDF](https://www.gods69.com/download/Le_livre_du_GFA_BASIC_30_Amiga.pdf). The scan's front matter identifies the authors, Data Becker, 1989, ISBN 2-86899-207-2, and French translator Christian Frey. Its introduction explicitly says the book describes the commands/functions available in the GFA 3.0 interpreter. The author-approved digitalization notice and distribution conditions appear in the front matter; the original publisher copyright notice is also reproduced. Use for research and page/section citations; do not mirror or commit the PDF. |

### Verified contents of AM-BOOK-FR-1989

The searchable PDF has 12,360 extracted text lines. Its table of contents gives a detailed Amiga-specific map (printed page numbers shown in the book):

- Ch. 5, pp. 61–124: input/output, screen, disk operations, file management, direct-access files, port I/O, AmigaDOS library, printing, sound.
- Ch. 6, pp. 125–166: program structure, loops, conditional branches, ZONE and variable declarations, subprograms, calls to assembler/C/PRG programs.
- Ch. 7, pp. 167–174: string operations and formatting.
- Ch. 8, pp. 175–198: operators, mathematical and numeric functions, trigonometry, comparisons, random numbers.
- Ch. 9, pp. 199–246: graphics definitions, graphics primitives, drawing, graphics memory, animation.
- Ch. 10, pp. 247–258: data conversion and number systems.
- Ch. 11, pp. 259–300: arrays, pointers, memory operations, user memory management, Amiga Exec library.
- Ch. 12, pp. 301–330: program control, clearing, date/time, error handling, information, multitasking, debugging.
- Ch. 13, pp. 331–338: program/user interaction.
- Ch. 14, pp. 339–352: window and screen programming.
- Ch. 15, pp. 353–358: menu programming.
- Ch. 16, pp. 359–368: BASIC event control.
- Ch. 17, pp. 369–376: advanced programming and newer commands, including GFA BASIC 3.04 environment.

The introduction (printed pp. 1-9 to 1-12) documents the book's syntax notation, optional parameters, command abbreviations, parameter categories, expressions, and a GFA BASIC 3.0-specific rule for omission of GOSUB in ON...GOSUB forms. These are promising specification topics; every rule must be extracted with its exact printed page and checked against the German original or interpreter where possible.

### Aminet and source-code leads verified beyond the initial directory

- Aminet upload listing includes `gfalist.lha` under `dev/basic`, described as a tool that reads tokenized GFA BASIC versions. This may help inspect tokenized source files, but it is not itself a language specification. [Aminet upload listing](https://www.amiga-news.de/en/standard/?frm_pageType=news&frm_perPage=21&frm_start=7644).
- Amiga Future's PDK 10 (5/1993) and PDK 11 (5/1994) contents identify GFA BASIC example programs with source, including graphics, mouse, file requester, and utility examples. [PDK contents](https://www.amigafuture.de/app.php/kb/viewarticle?a=2972).
- The online *Computers and Chaos* Amiga edition contains ASCII GFA BASIC examples. [Example book](https://amigasourcecodepreservation.gitlab.io/computers-and-chaos-amiga-edition/).

Do not confuse these independent source categories: the 1989 book is a GFA BASIC 3.0 reference; 3.52 disk images are later executable releases; PDK and book listings are examples; Aminet tools may inspect tokenized files. Keep version and evidence provenance attached to every extracted feature.

### Additional verification from the complete French scan

- The book's preface says it presents approximately **360 commands and functions** available in the Amiga GFA BASIC 3.0 interpreter (printed p. 1-9). This is a broader count than the contemporary 1988 review's approximate 350 commands; retain both as approximate historical figures with different scope.
- The complete searchable extraction has 12,360 lines and reaches the final appendices, printed pp. 377–379. Appendix A is an ASCII-code table; Appendix B is a GFA BASIC error-message list. The French book does **not** contain the alphabetical command index described by the 1988 review for the original German software manual.
- Initial source-backed language-core findings now include the Amiga 3.0 book's no-reserved-words statement, type suffixes, Boolean representation (0/−1), four loop families, and bit/word/byte operations. See [Amiga GFA BASIC 3.0 research notes](reference/amiga-gfa-basic-3.0.md).
- Important correction: the earlier note that searchable OCR stopped around printed p. 197 was wrong. Only the first extraction pass stopped there; the full PDF OCR is searchable through the end of the book.

| AM-BOOK-DE-1990 | *Das große GFA-BASIC-Buch zum Amiga: Bis Version 3.5 und Compiler 3.5*, Bleek, Hecht, Litzkendorf, Data Becker, 1990, ISBN 3-89011-399-0 | Amiga GFA BASIC V3.03 interpreter plus compiler material through the 3.5-era title scope | Primary published technical book; searchable OCR inspected | The title page and preface identify the 1990 German Data Becker edition and say the book documents commands/functions of the GFA V3.03 interpreter. It includes separate chapters for graphics, interaction, windows/screens, menus, and event control. The source copyright notice prohibits reproduction/distribution without written permission; do not mirror the scan or reproduce substantial text/code. [German OCR PDF](https://retro-commodore.eu/files/downloads/amigamanuals-xiik.net/eBooks/Das%20Grosse%20Buch%20zu%20GFA-Basic%20Amiga%20%28OCR%29%20-%20eBook-DE.pdf). |

### Additional primary source located: German 1990 Amiga edition

The German edition is distinct from the French 1989 *Le livre du GFA Basic Amiga* entry above. The 1990 title is *Das große GFA-BASIC-Buch zum Amiga: Bis Version 3.5 und Compiler 3.5*, by Wolf-Gideon Bleek, Martin Hecht, and Uwe Litzkendorf, Data Becker, ISBN 3-89011-399-0. Its preface says the command/function descriptions cover the GFA V3.03 interpreter and reports approximately 360 commands/functions. The printed page map includes graphics (pp. 225–278), interaction (pp. 363–374), windows/screens (pp. 375–388), menus (pp. 389–394), and BASIC event control (pp. 395–404).

The German scan's copyright notice prohibits reproduction or distribution without written permission. Its presence online is not permission to copy it into this repository. Use concise summaries and edition-specific printed-page citations only. Keep its pagination and V3.03 scope separate from the French 1989 V3.0 book. The reference notes record the initial graphics corroboration and remaining uncertainty in [amiga-gfa-basic-3.0.md](reference/amiga-gfa-basic-3.0.md).
