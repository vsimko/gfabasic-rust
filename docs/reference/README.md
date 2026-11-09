---
type: "Reference"
title: "GFA BASIC language reference notes"
---

# GFA BASIC language reference notes

This directory contains original research notes and technical summaries. These documents are version- and platform-scoped. They are not interchangeable specifications.

| Document | Platform and version | Status |
|---|---|---|
| [Amiga GFA BASIC 3.5 formatter](amiga-gfa-basic-35-formatter.md) | Amiga, interpreter 3.5 | Prototype scope, canonical style, CLI behavior, and evidence-gated limitations |
| [Atari ST GFA BASIC V2.xx / V3.0](atari-gfa-basic-3.0.md) | Atari ST, V2.xx/V3.0 | Preliminary research draft; not an accepted compiler dialect |
| [Atari ST API reference](atari-gfa-api.md) | Atari ST, V2.xx/V3.0 | Initial API inventory; signatures and runtime behavior require validation |
| [Systematic Atari book review](../research/atari-gfa-book-systematic-review.md) | Atari ST, V2.xx/V3.0 | First pass complete for all 24 chapters; validation and page-image checks pending |
| [German Amiga GFA-BASIC book command/API extraction](../research/amiga-gfa-basic-german-book-command-api-extraction.md) | Amiga, DATA BECKER 1990 | Historical 3.0/3.03 command and OS API research, cross-checked against the English handbook's explicit 3.5 addendum; use the separate addendum inventory for detailed 3.5 matrix semantics |
| [Amiga GFA BASIC 3.5 primary reference](../research/amiga-gfa-basic-35-primary-reference.md) | Amiga, interpreter 3.5 | Primary manual identified; selected editor/addendum pages visually reviewed; full Chapter 13/14 review remains incomplete; base 3.0 language rules remain inherited candidates pending feature-level 3.5 confirmation |
| [Amiga GFA BASIC 3.5 addendum inventory](../research/amiga-gfa-basic-35-addendum-inventory.md) | Amiga, interpreter 3.5 | Selected Chapter 13/14 sections page-image reviewed; broader review remains incomplete; the `MAT INV` operand-direction conflict remains disputed and unresolved |
| [German Amiga GFA BASIC 3.0 source notes](../research/amiga-gfa-basic-3.0-german-source-notes.md) | Amiga, interpreter 3.0 | Primary source; front matter and selected pages visually checked; historical evidence, not proof of 3.5 behavior |
| [Amiga GFA BASIC 3.0](amiga-gfa-basic-3.0.md) | Amiga, interpreter 3.0 | Historical language/API research notes; not the selected 3.5 target |
| [German Amiga GFA BASIC book OCR index](../research/amiga-gfa-basic-3.03-ocr-index.md) | Amiga, German 1990 edition | OCR-derived chapter map; version applicability disputed between 3.0 and 3.03; not normative yet |
| [Amiga API reference](amiga-gfa-api.md) | Amiga, interpreter 3.0 | Historical 3.0 API evidence; not the selected 3.5 API contract; signatures and applicability require review |
| [Atari book diskette corpus](../research/atari-gfa-book-diskette.md) | Atari ST, examples accompanying the 1988 book | Static corpus analysis; examples are not normative specifications |
| [Amiga emulator experiment lab](../research/amiga-emulator-lab.md) | Amiga, emulator-based research procedure | Proposed procedure; no GFA BASIC image executed |

## Rules for using these notes

- Keep Atari ST and Amiga syntax/API evidence separate.
- Keep each fact tied to its source edition, version, platform, and printed page.
- Treat later releases as separate compatibility targets until checked.
- Do not infer compiler support from the presence of documentation.
- Use original explanations and independently authored tests. Do not add scans or substantial manual reproductions.
- See [the source register](../reference-sources.md) and [dialect policy](../architecture/DIALECTS.md).
