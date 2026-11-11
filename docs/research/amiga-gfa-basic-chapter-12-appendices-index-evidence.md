---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 12 — appendix and index evidence"
---

# Amiga GFA BASIC Chapter 12 — appendix and index evidence

## Scope and version boundary

Primary source: supplied *GFA-BASIC 3.5 Interpreter* PDF, Chapter 12, “Appendices”, printed pages 12-2–12-27 (PDF pages 402–427). The manual identifies Chapters 1–12 as original 3.0 material and Chapter 13 as the 3.5 addendum. Chapter 12 is therefore **3.0 base-manual reference evidence**, not an independent list of 3.5 additions.

Printed page labels are the primary locators. Chapter 12 is chiefly a set of reference tables and indexes; its entries point back to descriptions elsewhere in the manual.

## Appendix inventory

| Section | Source content | Purpose / evidence boundary | Printed pages |
|---|---|---|---:|
| Keyboard scan codes | Amiga keyboard scan-code table | Reference values for keyboard-oriented programming; the table itself does not define a BASIC statement or guarantee host keyboard compatibility. | 12-2 |
| ASCII table | ASCII character/code table | Character-code lookup reference. Preserve the source's numeric values; do not substitute a modern Unicode interpretation. | 12-3 |
| Special ASCII codes | Additional/special character-code table | Supplementary character encodings, including displayed accented/special characters. OCR of glyphs is unreliable; verify exact glyph/code pairs against the page image before machine transcription. | 12-4–12-5 |
| Fill patterns | Numbered fill-pattern illustrations and explanation | The source says these patterns are used with `DEFFILL`; the first number selects style and the second selects pattern. | 12-6 |
| AmigaDOS error messages | Numeric error-code/message list | OS-level error references, including codes 103, 104, 120–122, 202–226 and 232. These are not interchangeable with GFA BASIC's own error numbers. Retain source spelling when transcribing messages. | 12-7 |
| Table of commands | Alphabetical command/function/operator index, with abbreviations and description-page references | Navigation index across the manual (printed pp. 12-8–12-18); entries include language constructs, built-ins, operators, special variables and platform facilities. It is not a version applicability matrix or a proof of implementation. | 12-8–12-18 |
| Table of Libraries | Alphabetical Amiga OS system-call index with chapter/page references | Navigation index into Chapter 11 library descriptions (printed pp. 12-19–12-27). It includes calls from Exec, Graphics, Workbench, Intuition, DiskFont, Layers and DOS; it is not a complete ABI specification. | 12-19–12-27 |

## Index interpretation

The command table records three distinct kinds of information in adjacent columns: the source spelling, an abbreviation where supplied, and the page on which the manual describes the item. A listed item may be an operator, variable, statement, function, editor facility or Amiga-specific feature. Do not count every row as a distinct BASIC command without applying the project's normalization and classification policy.

The system-call table is likewise a locator, not an independent API definition. Its entries should resolve to the Chapter 11 routine description for argument and return-value details. Keep OS calls in the Amiga OS API namespace rather than merging them into the BASIC keyword inventory.

## Version and runtime evidence

- Chapter 12 belongs to the manual's 3.0 base chapters; it does not independently establish 3.5 support.
- Chapter 13 is the separately identified 3.5 addendum. Its explicit additions must be recorded from that chapter, not inferred from Chapter 12 index rows.
- The tracked native 3.52 regression probes (`fact-zero`, `arithmetic`, `for-next`, `strings`) do not validate the keyboard/ASCII tables, fill-pattern rendering, AmigaDOS error catalogue, or completeness of either index.
- No N/M compatibility metric is derived from appendix rows. Index coverage and runtime conformance are different measures.

## Extraction cautions

- Keep printed page labels 12-2 through 12-27; PDF viewer page numbers are secondary.
- The ASCII and special-character pages are image-heavy. OCR substitutions can change both glyphs and numeric codes; use page images for any exact transcription.
- The fill-pattern page is graphical. Preserve its visual mapping rather than trusting OCR-rendered pattern labels.
- The command and system-call tables span multiple pages and contain OCR-sensitive spacing and punctuation. The current inventory records their scope and page boundaries, not a verified row-by-row transcription.
- AmigaDOS messages are OS diagnostics. Do not map their numeric values directly onto GFA BASIC runtime error codes.

## Related repository evidence

- [Chapter 11 system-routine evidence](amiga-gfa-basic-chapter-11-system-routines-evidence.md)
- [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md)
- [Language population policy](amiga-gfa-basic-language-population-policy.md)
- [Appendix D command index OCR transcription](amiga-book-command-index-ocr-transcription.md)
- [Native 3.52 regression batch](../../tests/interpreter-probes/amiga-3.52/regression-suite.LST)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
