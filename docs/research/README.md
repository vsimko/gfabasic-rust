# Research materials

This directory contains research procedures and analysis of historical material. It is not a normative language specification and does not define implementation support.

**Research-only policy:** Historical manuals, books, ROMs, disk images, and historical sample programs are external references, not project deliverables. Do not add these artifacts to the repository or bundle them with releases. Keep only source metadata, concise citations, original analysis, and independently authored tests.

| Document | Scope | Status |
|---|---|---|
| [Aminet GFA BASIC package survey](aminet-catalogue.md) | Amiga Aminet `dev/basic` archive listing | Initial package inventory; readmes, archive contents, and rights remain unverified |
| [German Amiga book OCR index](amiga-gfa-basic-3.03-ocr-index.md) | Amiga GFA BASIC, German 1990 edition | OCR table-of-contents map; version coverage and page-image validation remain open |
| [German Amiga GFA BASIC 3.0 source notes](amiga-gfa-basic-3.0-german-source-notes.md) | Amiga interpreter 3.0; German first edition, 1989 | Front matter and selected pages visually checked; remaining chapter/page review is incomplete |
| [Amiga GFA BASIC 3.5 primary reference](amiga-gfa-basic-35-primary-reference.md) | Amiga interpreter 3.5 manual | Version-specific primary source identified; full manual feature inventory remains in progress |
| [Amiga GFA BASIC 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) | Amiga interpreter 3.5, Chapters 13–14 | Original feature summaries with printed-page citations; OCR-sensitive syntax still needs page-image verification |
| [Amiga emulator experiment lab](amiga-emulator-lab.md) | Proposed FS-UAE setup and procedure for future Amiga interpreter experiments | Procedure documented; no GFA BASIC image has been executed |
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
