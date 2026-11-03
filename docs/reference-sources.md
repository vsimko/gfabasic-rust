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

Amiga-specific book and manual candidates have since been identified and are listed below. Their applicability is version-scoped, and none is currently accepted as a complete normative source for Amiga 3.52. The Atari ST manual must not be treated as sufficient evidence for Amiga-specific commands or behavior.

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
| AM-ADF-352-D1 | Archive catalogue label: GFA-BASIC v3.52 (1990), GFA Systemtechnik, German, disk 1 of 2 | Amiga GFA BASIC package; component/version identity not independently verified | Archive catalogue entry; binary not inspected | Planet Emulation catalogue lists a 256 KB ADF and checksum `be832a95`. Do not infer that the interpreter itself is 3.52 from the archive title: a contemporary 1991 Amiga Magazine product list separately advertises Interpreter 3.51 and Compiler 3.52. Verify the disk's actual executables/version strings before assigning component versions. Do not redistribute the image; use only where lawful and record provenance. [Catalogue entry](https://www.planetemu.net/en/rom/commodore-amiga-applications-adf/gfa-basic-v3-52-1990-gfa-systemtechnik-de-disk-1-of-2-wb). |
| AM-ADF-352-D2 | Archive catalogue label: GFA-BASIC v3.52 (1990), GFA Systemtechnik, German, disk 2 of 2 | Amiga GFA BASIC package; component/version identity not independently verified | Archive catalogue entry; binary not inspected | Planet Emulation catalogue lists a 234 KB ADF and checksum `5f75a9cc`. Apply the same component-version caution as disk 1. Do not redistribute the image; use only where lawful and record provenance. [Catalogue entry](https://www.planetemu.net/en/rom/commodore-amiga-applications-adf/gfa-basic-v3-52-1990-gfa-systemtechnik-de-disk-2-of-2). |
| AM-MAG-1991-03 | *Amiga Magazine*, Anno 4, no. 21, March 1991, p. 17 (Italian distributor product listing) | Contemporary Amiga commercial availability; identifies interpreter and compiler products separately | Contemporary secondary commercial listing; not a technical specification | The listing names “GFA BASIC INTERPRETER 3.51” and “GFA BASIC COMPILER 3.52” as separate purchasable products. This is direct evidence that these component version labels were distinct in this 1991 product listing, but it does not prove the exact version strings of later archive images or the internal compatibility relationship. [Scanned issue PDF](https://www.amigamagazine.info/pdf/AM_21.pdf), printed p. 17. |
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

| ATARI-GFA3-UM-1988 | GFA BASIC Version 3 User Manual, GFA Software Technologies, Inc., July 1988, ISBN 1-85181-186-9; copyright ©1988 GFA Systemtechnik GmbH | Atari ST GFA BASIC 3.0 (explicitly Atari ST; not an Amiga manual) | Primary Atari ST interpreter manual; searchable PDF inspected | The title page names GFA BASIC Version 3 User Manual, July 1988, ISBN, and publisher. The manual's contents document Atari-specific GEMDOS/BIOS/XBIOS, VDI, AES, Line-A, event, menu, and window facilities. It is a valid primary source for the Atari ST profile (#25), and may be considered for common language-core cross-checks only after an explicit feature-by-feature comparison with Amiga sources. Do not use its graphics/window/event signatures as Amiga API evidence. The PDF's copyright notice prohibits copying/reproduction without prior written permission. [PDF scan](https://www.atarimania.com/st/files/GFA_BASIC_Version_3_Interpreter_User_Manual.pdf). |

### Explicit platform exclusion: Atari ST GFA BASIC 3 manual

The July 1988 GFA BASIC Version 3 User Manual is now identified from its title page, publication data, and contents as an Atari ST manual, not an Amiga manual. It is a strong primary source for the separate Atari ST profile (#25). Its graphics chapter covers GRAPHMODE, PLOT, LINE, DRAW, BOX, and related commands; its event/menu/window chapter covers ON MENU, MENU(), OPENW, CLOSEW, and WINDTAB; later chapters explicitly describe GEMDOS, BIOS, XBIOS, VDI, AES, and Line-A interfaces.

This manual must not be used as direct evidence for Amiga screen/window/menu/event behavior. Shared-looking GFA BASIC language constructs may be cross-checked only one feature at a time against an Amiga primary source, with any differences recorded. Its copyright notice forbids reproduction without prior written permission; retain bibliographic metadata, concise original summaries, and citations only.

| AM-BOOK-DE-1989-CATALOG | Das Grosse Buch zu GFA BASIC AMIGA, attributed to Bleek, Hecht, Litzkendorf; Data Becker, 1989, ISBN 3-89011-276-5 | Amiga GFA BASIC; exact interpreter-version coverage not yet verified | Secondary bibliographic/catalogue lead only; physical title/copyright pages and content not inspected | Retrobug and used-book catalogue records list a 1989 German Amiga book by these authors, Data Becker, ISBN 3-89011-276-5, with a reported length around 430 pages. This is a distinct bibliographic candidate from the searchable 1990 Data Becker scan with ISBN 3-89011-399-0. Do not merge the two records or assign version coverage until the 1989 book's title/copyright pages and contents are inspected. [Catalogue record](https://www.retrobug.org/library/med0142). |

### Acquisition attempt: authorized 1989 German PDF (checked 2026-11-03)

The reported collection at [ebook.pixas.de](http://ebook.pixas.de/) was checked again. The site could not be retrieved from the available research environment (the web retrieval service reported the URL inaccessible; a direct HTTPS/HTTP request from the container failed DNS resolution). Search-indexed Amiga-News coverage still reports that Thomas Werner offers the German 1989 title and French Amiga translation with the authors' permission, but no PDF bytes, file name, checksum, title page, imprint page, or pagination were obtained in this session. This is an access limitation of the current research environment, not evidence that the site is offline for all users. Do not mark the German scan as inspected or treat its exact version scope as verified. Next acquisition route: access the site from a normal browser or contact the maintainer (Thomas Werner) for the direct PDF and the exact permission scope; then record the original file hash and inspect the front matter without committing the scan.

### Version-component cross-check: interpreter 3.51 vs compiler 3.52

A contemporary Italian Amiga software catalogue provides an important qualification to archive labels that say “GFA-BASIC 3.52”. *Amiga Magazine*, year 4, no. 21 (March 1991), printed p. 17, lists two separate products: “GFA BASIC INTERPRETER 3.51” and “GFA BASIC COMPILER 3.52”. This is contemporary evidence for distinct interpreter and compiler product-version labels. It does not establish that every two-disk archive catalogued as “GFA-BASIC v3.52” contains an interpreter version 3.52. Until the binaries are legally obtained and inspected, record the ADFs as a GFA-BASIC 3.52-labelled package, with interpreter/compiler component versions unknown. The GFA-Basic FAQ's later version table reports Amiga editor, compiler, and linker 3.52, but is secondary and does not remove the need to inspect the specific release. [Amiga Magazine no. 21 (March 1991), p. 17](https://www.amigamagazine.info/pdf/AM_21.pdf); [GFA-Basic FAQ](https://gfabasic.net/htm/gfa_faq.htm).

### Unresolved German Amiga book bibliography

A further catalogue lead identifies *Das Grosse Buch zu GFA BASIC AMIGA* by Bleek, Hecht, and Litzkendorf, Data Becker, 1989, ISBN 3-89011-276-5, reportedly about 430 pages. This record is not yet backed by inspection of the physical title/copyright pages or contents. Keep it distinct from the searchable 1990 Data Becker scan (ISBN 3-89011-399-0, copyright 1990) and from the French 1989 translation (ISBN 2-86899-207-2). The similar titles and author list are not sufficient to conclude that these are the same edition, a translation, or a revision. Version coverage and page mapping remain unknown until the 1989 German candidate is independently verified.

### Bibliographic cross-check: German Amiga book, 1989 (checked 2026-11-03)

The 1989 German title is corroborated by multiple independent catalogue and historical references. BoekenWebsite's individual used-book record gives the title *Das grosse Buch zu GFA BASIC Amiga*, authors as “Bleek ea.”, publisher Data Becker, 430 pages, ISBN-10 3890112765, ISBN-13 9783890112763, German language, and publication year 1989. The ISBN-10 check digit and ISBN-13 are consistent. Retrobug independently lists the title, authors Bleek/Hecht/Litzkendorf, Data Becker, 1989, and ISBN 3890112765. DieBuchSuche explicitly groups two similar but distinct editions: ISBN 9783890112763 (430 pages) and ISBN 9783890113999 (508 pages); it gives the latter as a 1990 Data Becker edition. Its marketplace records are internally inconsistent about the year of the 430-page ISBN (some offers label it 1989, others 1990-02), so the year must ultimately be read from the scan's imprint rather than inferred from resale metadata. An Amiga Future bibliography based on a 1992 market list includes *Das große Buch zu GFA-BASIC Amiga* and names Bleek and Hecht. A contemporary 1988 review independently establishes that GFA-Basic 3.0 Amiga shipped with a German manual of approximately 420 pages; it is not proof that this 430-page book is that manual or that both refer to the same edition.

Amiga-News reported on 2026-09-03 that Thomas Werner offers the German 1989 book and the French translation as free PDFs with the authors' permission. This is a secondary report of authorization and availability, not direct inspection of the German scan. The reported download collection is associated with [ebook.pixas.de](http://ebook.pixas.de/), but the site was not accessible during this check. The French scan at gods69.com is a separate translation with ISBN 2-86899-207-2 and explicitly documents GFA BASIC Amiga interpreter 3.0; do not transfer its exact ISBN, version claims, or pagination to the German scan.

**Current conclusion:** bibliographic identity for a distinct 1989 German Amiga book is strongly corroborated; the 430-page / ISBN 3-89011-276-5 metadata is supported by a bookshop catalogue and corroborating historical catalogue entries. The title page, imprint/copyright page, actual scan pagination, precise interpreter-version coverage, and correspondence (if any) with the 1990 ISBN 3-89011-399-0 edition remain unverified. Keep AM-BOOK-1989 as a candidate primary source, not yet a page-verified normative source.

Sources checked:
- BoekenWebsite individual catalogue record: https://www.boekenwebsite.nl/computers-software/das-grosse-buch-zu-gfa-basic-amiga
- DieBuchSuche comparison of ISBN 9783890112763 and 9783890113999 (records show distinct 430- and 508-page editions but inconsistent dates for the former): https://diebuchsuche.at/buch-isbn-9783890113999.html
- Retrobug catalogue record: https://www.retrobug.org/library/med0142
- Amiga Future historical bibliography (article dated 2008, reproducing a 1992 market list): https://www.amigafuture.de/app.php/kb/viewarticle?a=2584
- Contemporary review of GFA-Basic 3.0 Amiga, *Happy Computer* 12/1988: https://www.stcarchiv.de/happy-computer/1988/12/gfa-basic-amiga
- Amiga-News report on author-authorized PDF availability, 2026-09-03: https://www.amiga-news.de/en/standard/?frm_perPage=21&frm_start=42
- Reported PDF collection: http://ebook.pixas.de/ (not directly accessible during this check)


### Additional Amiga GFA BASIC 3.0 reference-book lead (2026-11-03)

| ID | Source | Applicability | Evidence status | Notes |
|---|---|---|---|---|
| AM-BOOK-KOFLER-1989 | *Amiga GFA-Basic 3.0 Referenzbuch*, attributed to Kofler; Markt & Technik (bibliography shorthand: [mut]), 1989 | Explicitly catalogued as an Amiga GFA-BASIC 3.0 reference book; exact edition, author full name, ISBN, page count, and interpreter build coverage not yet verified | Bibliographic lead corroborated by the author's own bibliography and a separate Amiga book-order catalogue; no scan inspected | Michael Kofler's bibliography lists “Amiga GFA-Basic 3.0 Referenzbuch” under 1989. A 1991 Amiga book-order listing separately lists “GFA-Basic 3.0 Referenzbuch” by Kofler as an Amiga title (order no. 3347), distinct from “GFA Basic” by Bleek (order no. 3336). This may be another valuable German-language reference for checking the 3.0 book, but do not assume it is the 420-page software manual reviewed in 1988 or the Bleek/Hecht/Litzkendorf title. The exact bibliographic identity and contents remain to be verified from the scan's front matter. [Kofler bibliography](https://kofler.info/ueber/bibliografie/); [1991 Amiga book-order listing](https://www.sharp-pc-1600.de/allesfuersharp/ASFC_1991_06.pdf). |

The 1988 *Happy Computer* review independently confirms that the GFA-BASIC 3.0 Amiga package was accompanied by a German handbook of approximately 420 pages and that its appendix contained an alphabetical command index with page references. This is a useful identification clue for newly acquired scans, but it does not prove that the Kofler reference book is that same handbook. [Contemporary review](https://www.stcarchiv.de/happy-computer/1988/12/gfa-basic-amiga).


### ATARI-GFA-BOOK-1988 — *Das große GFA-BASIC-Buch*

- **Author:** Uwe Litzkendorf.
- **Edition:** 6th revised and expanded edition, 1988.
- **Publisher:** DATA BECKER GmbH, Düsseldorf.
- **ISBN:** 3-89011-222-6.
- **Original copyright notice:** ©1986 DATA BECKER GmbH; the printed book reserves all rights and states that reproduction requires written permission.
- **Platform:** Atari ST.
- **Version coverage:** GFA BASIC V2.xx and V3.0, as stated in the book's preface; the book describes approximately 450 commands and functions available in the V3.0 interpreter.
- **Source form inspected:** User-provided ZIP containing a 300-dpi PDF scan and the accompanying gfabuch30-Buchdisk.zip. The PDF has 836 pages and a searchable text layer. The nested disk archive contains 162 .LST files and 10 other files.
- **Digitalization notice:** The PDF front matter contains a 2020 note by digitalizer Thomas Werner and a foreword by Uwe Litzkendorf. The foreword says the author agreed to digitization and free online availability. The same notice says the PDF may only be redistributed unchanged and may not be resold. The printed 1988 copyright notice remains present. Do not infer that the accompanying diskette files have the same permission terms.
- **Research use:** Primary Atari ST technical reference for the language and platform material within its stated V2.xx/V3.0 scope. Use printed page numbers and section headings. The source disk is useful as an example corpus, but its README says comments were removed and some examples require fragments to be merged; individual files are not necessarily standalone.
- **Repository handling:** Do not commit the PDF, full OCR transcript, or complete sample archive. Publish original technical summaries, bibliographic metadata, short citations, and independently authored tests. Confirm the rights status of individual sample files before redistributing any of them.
- **Important separation:** This is an Atari ST source. Do not use it as direct evidence for Amiga APIs or later Atari ST releases such as 3.5/3.6TT without cross-checking.
- **Initial page map:** Language overview and V3.0 changes, pp. 23–24; comments, p. 186; variable declarations/types, pp. 189–192; user functions and parameter behavior, pp. 192–194; program structure, chapter 6 beginning p. 155; strings, chapter 7 beginning p. 221; arithmetic/operators, chapter 8 beginning p. 229; graphics, chapter 9 beginning p. 251; arrays/memory/pointers, chapter 11 beginning p. 341; system APIs, chapter 21 beginning p. 605; editor/compiler, chapter 24 beginning p. 701.
- **Evidence status:** Bibliographic identity and stated coverage verified from the scanned title/copyright/front-matter pages. The initial language facts above are transcribed as source-backed research findings; detailed syntax tables and OCR-sensitive signatures still require page-image verification.


### Contemporary edition and pagination cross-check (2026-11-03)

A contemporary *ST-Magazin* book review/catalogue in the December 1988 issue explicitly identifies Uwe Litzkendorf's *Das große GFA Basic 3.0 Buch* as the **6th revised and expanded edition (1988)**, Data Becker, ISBN 3-89011-222-6, with **828 pages**. The same issue's publisher advertisement lists the hardcover at 828 pages with diskette. This independently corroborates the edition and printed extent recorded from the user-provided scan. The scan has 836 PDF pages; the eight-page difference is consistent with unnumbered digitization/front-matter or inserted material, but the PDF-to-printed-page mapping should be treated as its own pagination layer.

An October 1988 *ST-Magazin* literature listing gives the same ISBN but reports 574 pages without naming an edition. That earlier count is not sufficient to override the explicit December sixth-edition record; it may refer to an earlier printing/edition or a catalogue discrepancy. Retain the scan's title/copyright/front-matter evidence as the identity authority and record the October count as an unresolved historical catalogue variance, not as a second confirmed pagination for this exact sixth edition.

Sources:
- *ST-Magazin* 12/1988, printed p. 136 (book review and bibliographic record): https://www.homecomputerworld.at/magazine/st-magazin/st_magazin_1988-12.pdf
- *ST-Magazin* 12/1988, publisher advertisement (same issue; 828-page hardcover listing): https://www.homecomputerworld.at/magazine/st-magazin/st_magazin_1988-12.pdf
- *ST-Magazin* 10/1988, literature list: https://www.stcarchiv.de/st-magazin/1988/10/buecherliste


### Aminet `dev/basic` catalogue verification (2026-11-03)

The live Aminet mirror directory and the package README were checked on 2026-11-03:

| Item | Catalogue metadata | Classification | Evidence limit |
|---|---|---|---|
| `gfalist.lha` / `gfalist.readme` | Directory listing: 150 KB, dated 2021-02-18. README: version 0.01; authors Peter Backes and Markus Hoffmann; architecture `ppc-morphos`; short description: reads tokenized GFA BASIC version 3.x. | Developer tool for inspecting tokenized GFA BASIC files. | The README does not establish interpreter semantics, token assignments, or compatibility with a specific Amiga 68k release. The archive contents have not been inspected. |
| `GFA_Gadgets.lha` / `GFA_Gadgets.readme` | Present in the live `dev/basic` directory; archive listed as 28 KB, dated 1998-12-31. | Candidate GUI/gadget helper package. | Package README could not be decoded by the available text fetcher in this pass. Contents, author, dependencies, and license remain unverified. |
| `GFA_Menus.lha` / `GFA_Menus.readme` | Present in the live `dev/basic` directory; archive listed as 15 KB, dated 1998-12-31. | Candidate menu helper package. | Package README could not be decoded by the available text fetcher in this pass. Contents, author, dependencies, and license remain unverified. |
| `GuiGfxDevBas.lha` / `GuiGfxDevBas.readme` | Present in the live `dev/basic` directory; archive listed as 31 KB, dated 2003-07-21. | Candidate GUI/graphics developer package. | Package README could not be decoded by the available text fetcher in this pass. Contents, author, dependencies, and license remain unverified. |

Sources:
- Live directory listing: [Aminet `dev/basic` mirror](https://ftp.fau.de/aminet/dev/basic/).
- Exact `gfalist` metadata: [gfalist.readme](https://ftp.fau.de/aminet/dev/basic/gfalist.readme).
- Independent contemporary upload notice: [Amiga-News, Aminet uploads through 2021-02-20](https://www.amiga-news.de/en/standard/?frm_start=1883).

The directory and README confirm that `gfalist` is a MorphOS/PPC tool for tokenized GFA BASIC 3.x. It is not an Amiga GFA BASIC language reference and must not be used to infer built-in commands or runtime behavior. The other three packages are catalogue leads only. Their names are not evidence that their functionality is built into GFA BASIC.

This is a verified snapshot of selected catalogue entries, not a complete survey of `dev/basic`. Archive inspection, license checks, package dependency extraction, and classification of additional entries remain open work for issue #64.


## Amiga emulator and experiment environment

The proposed reproducible emulator procedure is documented in [the Amiga emulator experiment lab](research/amiga-emulator-lab.md). It uses FS-UAE as a Linux-hosted research tool, requires a legally usable Kickstart ROM and a separately authorized GFA BASIC copy, and does not claim that any interpreter or compiler image has been executed.

| ID | Source | Applicability | Evidence status | Notes |
|---|---|---|---|---|
| AM-EMU-FS-UAE | FS-UAE official documentation and Linux downloads | Host emulator for Amiga experiments | Tool documentation; no GFA BASIC behavior evidence | FS-UAE is GPL-licensed and provides Linux x86-64 and ARM64 builds. Its documentation recommends original Kickstart ROMs for compatibility and notes that the built-in AROS replacement is less compatible. Record the exact emulator build and configuration used for every observation. [Linux downloads](https://fs-uae.net/download/linux/); [Kickstart ROM guidance](https://fs-uae.net/docs/kickstart-roms/); [release notes](https://fs-uae.net/news/). |
| AM-EMU-ROM-LICENSE | Amiga Forever ROM/OS licensing FAQ | Legal acquisition/use of Amiga ROM and OS files | Vendor licensing guidance; not legal advice | The vendor describes the EULA scope for ROM/OS use and warns that third-party files have separate licensing conditions. A ROM/OS license does not establish rights to copy or redistribute GFA BASIC disks. Review the current EULA and the rights for each third-party component separately. [Use of ROM and OS files](https://www.amigaforever.com/kb/13-145). |
