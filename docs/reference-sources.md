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
