---
type: "Research Note"
title: "Research materials"
---

# Research materials

This directory contains research procedures and analysis of historical material. It is not a normative language specification and does not define implementation support.

**Research-only policy:** Historical manuals, books, ROMs, disk images, and historical sample programs are external references, not project deliverables. Do not add these artifacts to the repository or bundle them with releases. Keep only source metadata, concise citations, original analysis, and independently authored tests.

## Amiga GFA BASIC command catalogue

For command/function discovery, start at the dedicated [Amiga GFA BASIC command catalogue](amiga-gfa-basic-command-catalogue.md). It explains which existing source to consult for alphabetical lookup, chapter-level semantics, structured function names, and interpreter-version evidence.

Quick routes:

- [Appendix D alphabetical OCR index](amiga-book-command-index-ocr-transcription.md) — broad name discovery; includes non-callable index terms and OCR uncertainty.
- [Detailed German book and Amiga OS API extraction](amiga-gfa-basic-german-book-command-api-extraction.md) — chapter-level extraction and page-backed semantics.
- [Source/topic index](amiga-gfa-basic-source-topic-index.md) — printed-page navigation and cross-source disagreements.
- [Machine-readable function inventory](amiga-gfa-basic-functions.toml) — 116 unique names under the documented function-inventory counting rules.
- [3.5 candidate manifest](amiga-gfa-basic-candidates.toml) and [report](amiga-gfa-basic-candidate-report.md) — version-applicability status by feature family, not by individual command.

**Counting warning:** the German book describes approximately 360 commands and functions, but its alphabetical index mixes statements, functions, aliases, constants, variables, object properties, and other terms. It describes GFA V3.03; it is not a verified 3.5 command list. The function inventory's 116 names and the candidate report's family counts measure different things. See the catalogue for definitions and workflow. Do not add these counts together or report them as a single command total.

| Document | Scope | Status |
|---|---|---|
| [Aminet GFA BASIC package survey](aminet-catalogue.md) | Amiga Aminet `dev/basic` archive listing | Initial package inventory; readmes, archive contents, and rights remain unverified |
| [German Amiga book OCR index](amiga-gfa-basic-3.03-ocr-index.md) | Amiga GFA BASIC, German 1990 edition | OCR table-of-contents map; version coverage and page-image validation remain open |
| [German Amiga GFA BASIC 3.0 source notes](amiga-gfa-basic-3.0-german-source-notes.md) | Amiga interpreter 3.0; German first edition, 1989 | Front matter and selected pages visually checked; remaining chapter/page review is incomplete |
| [Amiga GFA BASIC 3.5 primary reference](amiga-gfa-basic-35-primary-reference.md) | Amiga interpreter 3.5 manual | Version-specific primary source identified; full manual feature inventory remains in progress |
| [Amiga GFA BASIC 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) | Amiga interpreter 3.5, explicitly identified editor update and Chapters 13–14 | Page-image review of the complete explicitly identified 3.5 addendum/editor update is complete; OCR-sensitive details and recorded source discrepancies remain. This does not revalidate base-manual Chapters 2–12 for 3.5 |
| [Amiga GFA BASIC source and topic index](amiga-gfa-basic-source-topic-index.md) | Amiga interpreter 3.5 addendum and German 1990 Amiga book | Consolidated printed-page navigation, source/version boundaries, cross-source disagreements, and page-image validation register; research index only, not runtime conformance |
| [German Amiga GFA-BASIC book: command and OS API extraction](amiga-gfa-basic-german-book-command-api-extraction.md) | DATA BECKER, 1990, ISBN 3-89011-399-0 | German book extraction through Chapter 19 and Appendices B–D plus English 3.5 addendum reconciliation: added PCOLOR, GURU, _0…_9, negative-screen OPENW, MAT COPY and precise floating-point/vector semantics; German book and English 3.5 addendum fully indexed; page-image verified IDCMP flags (p. 378), Window flags (p. 379), and MENU attributes (p. 389, source duplicate value 64 retained); page-image verified SCROLL syntax/sign behavior (pp. 245–246), DRAW$ turtle grammar and DRAW() indices (pp. 248–250), GRAPHMODE examples 1=REPLACE/3=XOR (pp. 240, 250), and OBJECT.VY/OBJECT.PLANES (p. 273); page-image verified GET/PUT header, drawing modes and sprite bitmap/color-register layout (pp. 257, 261–267); DUMP selectors/output and MODE locale bit table (pp. 352–354, 358); ALERT, MOUSE and STICK/STICK() definitions (pp. 363, 371–372); FILESELECT outcomes and sample-only extension/backup routines (pp. 365–368), MOUSE button values (p. 368), STRIG() (p. 373), MENU event values/payloads (pp. 398–402, including OCR correction 264144→262144 and printed timer discrepancy 4194303 vs IDCMP 4194304), and ON MENU/SLEEP dispatch semantics (pp. 403–404) page-image verified; consolidated navigation and cross-source index is in `amiga-gfa-basic-source-topic-index.md`; runtime delivery, pointer lifetime and event-queue behavior remain untested |
| [Amiga GFA BASIC commands observed in dqh-amiga](amiga-gfa-basic-dqh-source-inventory.md) | Amiga game source `vsimko/dqh-amiga` (`dqh.lst`) | Source-derived GFA BASIC constructs and direct Amiga OS/library calls; not a complete or version-conformance specification |
| [Amiga GFA BASIC 3.5 built-in function inventory](amiga-gfa-basic-function-inventory.md) and [machine-readable list](amiga-gfa-basic-functions.toml) | Amiga interpreter 3.5 | 116 unique names across the combined manual; 114 interpreter candidates, including one disputed CVx name; two compiler-only entries are excluded from the interpreter count |
| [Amiga GFA BASIC candidate manifest](amiga-gfa-basic-candidates.toml) and [generated verification report](amiga-gfa-basic-candidate-report.md) | Amiga interpreter 3.5 candidate status | Structured, version-scoped tracking; unresolved 3.0 families are kept separate from declarations explicitly documented in the 3.5 addendum |
| [Amiga emulator experiment lab](amiga-emulator-lab.md) | Amiberry-based native interpreter experiment workflow | Native Merge + Run workflow is integrated on `main`; a 3.52 `FACT(0..2)` probe passed. Exact 3.5 interpreter media remains unavailable; see the [3.52 runtime baseline](amiga-gfa-basic-352-runtime-baseline.md) |
| [Atari GFA BASIC book diskette](atari-gfa-book-diskette.md) | Static analysis of examples accompanying an Atari ST reference book | Research corpus; file rights and standalone behavior require per-example review |
| [Systematic Atari book review](atari-gfa-book-systematic-review.md) | Chapter-by-chapter analysis of the Atari ST GFA BASIC reference book | Initial review completed; page-image and semantic validation remain |

## Research rules

- Identify source provenance, platform, dialect/version, edition, and page/section.
- Separate direct source statements, observed behavior, interpretation, and implementation decisions.
- Do not infer Amiga behavior from Atari ST sources or the reverse.
- Treat archive names and catalogue descriptions as leads until the underlying material is inspected.
- Record provenance and applicability, but do not distribute historical manuals/books, ROMs, disk images, or historical sample programs. Do not commit scans, OCR dumps, archive images, or copied source listings. Keep only metadata, concise citations, original analysis, and independently authored fixtures.
- Promote a research claim into a language/API reference only after its evidence has been reviewed.

See the [reference source register](../reference-sources.md), [language reference notes](../reference/README.md), and [documentation policy](../DOCUMENTATION.md).
