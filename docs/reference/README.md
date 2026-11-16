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
| [German Amiga GFA-BASIC book command/API extraction](../research/amiga-gfa-basic-german-book-command-api-extraction.md) | Amiga, DATA BECKER 1990 | Historical 3.0/3.03 command and OS API research, cross-checked against the English 3.5 addendum; preserve the book's stated version scope |
| [Amiga GFA BASIC 3.5 primary reference](../research/amiga-gfa-basic-35-primary-reference.md) | Amiga, interpreter 3.5 | Primary manual identified; explicitly marked 3.5 editor update and addendum page images reviewed. The base 3.0 chapters remain historical evidence unless separately confirmed for 3.5 |
| [Amiga GFA BASIC 3.5 addendum inventory](../research/amiga-gfa-basic-35-addendum-inventory.md) | Amiga, interpreter 3.5 | Explicit 3.5 addendum extraction with page-image review; `MAT INV destination()=source()` is resolved from the detailed definition/example (printed p. 13-30), overriding the reversed summary typo on p. 13-16. Runtime edge cases remain untested |
| [German Amiga GFA BASIC 3.0 source notes](../research/amiga-gfa-basic-3.0-german-source-notes.md) | Amiga, interpreter 3.0 | Primary source; front matter and selected pages visually checked; historical evidence, not proof of 3.5 behavior |
| [Amiga GFA BASIC 3.0](amiga-gfa-basic-3.0.md) | Amiga, interpreter 3.0 | Historical language/API research notes; not the selected 3.5 target |
| [Amiga GFA BASIC 3.0 feature inventory](amiga-gfa-basic-3.0-feature-inventory.md) | Amiga, interpreter 3.0 | Historical feature inventory; incomplete and not a 3.5 implementation contract |
| [German Amiga GFA BASIC book OCR index](../research/amiga-gfa-basic-3.03-ocr-index.md) | Amiga, German 1990 edition | OCR-derived chapter map; version applicability disputed between 3.0 and 3.03; not normative yet |
| [Amiga GFA API reference](amiga-gfa-api.md) | Amiga, interpreter 3.0 | Historical 3.0 API evidence; not the selected 3.5 API contract; signatures and applicability require review |
| [Atari book diskette corpus](../research/atari-gfa-book-diskette.md) | Atari ST, examples accompanying the 1988 book | Static corpus analysis; examples are not normative specifications |
| [Amiga emulator experiment lab](../research/amiga-emulator-lab.md) | Amiga, emulator-based research procedure | Native Amiberry Merge + Run workflow integrated on `main`; one 3.52 FACT probe passed. Exact 3.5 validation remains blocked; see the [3.52 runtime baseline](../research/amiga-gfa-basic-352-runtime-baseline.md) |

## Rules for using these notes

- Keep Atari ST and Amiga syntax/API evidence separate.
- Keep each fact tied to its source edition, version, platform, and printed page.
- Treat later releases as separate compatibility targets until checked.
- Do not infer compiler support from the presence of documentation.
- Use original explanations and independently authored tests. Do not add scans or substantial manual reproductions.
- See [the source register](../reference-sources.md), [dialect policy](../architecture/DIALECTS.md), and the [documentation audit](../research/amiga-gfa-basic-documentation-audit.md).
