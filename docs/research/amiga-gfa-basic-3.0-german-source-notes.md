---
type: "Research Note"
title: "German Amiga GFA BASIC 3.0 source — initial verification notes"
---

# German Amiga GFA BASIC 3.0 source — initial verification notes

- **Source ID:** `AM-BOOK-DE-1989`
- **Title page:** *Das große GFA-BASIC-Buch*
- **Authors:** Wolf-Gideon Bleek, Martin Hecht, Uwe Litzkendorf
- **Publisher:** DATA BECKER GmbH
- **Edition:** 1. Auflage, 1989
- **ISBN:** 3-89011-276-5
- **Dialect/version documented by this source:** Commodore Amiga GFA-BASIC interpreter 3.0
- **Project compatibility target (decision, 2026-11-04):** Commodore Amiga GFA-BASIC 3.5
- **Source scan:** [PDF on the Internet Archive](https://dn710209.ca.archive.org/0/items/das-grosse-buch-zu-gfa-basic-amiga/Das_grosse_Buch_zu_GFA_BASIC_AMIGA%20%28Data%20Becker%29.pdf)
- **Research copy:** supplied by the repository owner in the conversation/work environment on 2026-11-04. The PDF is not stored in this repository.
- **Scan properties:** 432 PDF pages; selectable OCR text is present. Printed page numbers are visible and correspond to the book's own pagination, not necessarily PDF viewer page numbers.

## Project target and compatibility policy

The project owner selected **Amiga GFA-BASIC 3.5** as the normative compatibility target on 2026-11-04.

This 1989 book is explicitly a source for Amiga GFA-BASIC 3.0, not 3.5. It is useful as a historical baseline and as a source for features that may be inherited, but it must not by itself be cited as proof of 3.5 behavior.

For implementation and conformance work, classify each rule as one of:

1. **Verified 3.5:** directly supported by a 3.5-specific primary source or by a reproducible observation against the 3.5 interpreter/compiler.
2. **Inherited candidate:** documented for 3.0 and believed relevant to 3.5, but not yet confirmed for 3.5.
3. **3.5 addition/change:** documented or observed as introduced or changed in 3.5.
4. **Unresolved:** source identity, exact syntax, semantics, or version applicability is not established.

Do not silently promote a 3.0 rule to verified 3.5. Keep language-core behavior separate from Amiga-specific runtime, OS/library, graphics, window, menu, and event APIs. Keep compiler and interpreter behavior distinct where the sources distinguish them.

The next research priority is to obtain and identify an Amiga-specific GFA-BASIC 3.5 manual, release notes, and/or original distribution media. Use those sources to establish the 3.5 command/function inventory and version-specific differences. Runtime probes should record the exact 3.5 build and environment. Atari ST 3.5 material may be used only as a lead until Amiga applicability is independently confirmed.

## Applicability and edition identity

The cover title is generic, but the first-edition foreword explicitly states that the book documents, tests, and explains all commands and functions of **Amiga GFA-BASIC version 3.0**. It is therefore a primary German-language source for the project's historical Amiga 3.0 baseline.

The title/copyright pages identify the first edition as 1989, ISBN 3-89011-276-5, published by DATA BECKER. The foreword is dated 6 December 1988. Do not confuse this edition with the separate 1990 Amiga 3.x/V3.03 book (ISBN 3-89011-399-0) already catalogued as `AM-BOOK-DE-1990`.

The scan's table of contents covers BASIC fundamentals, I/O, AmigaDOS and Exec libraries, program structure, strings, arithmetic, graphics, arrays/memory/pointers, program control, multitasking, debugging, windows/screens, menus, and event monitoring. The scan has 432 PDF pages; the book's printed pagination runs through the index and appendix.

## Access and rights handling

The scan includes a 2020 digitization foreword by Uwe Litzkendorf stating that he agreed to the digitization and free online availability. The following page says the digital edition may not be resold, may only be passed on unchanged, permits printing individual pages for private use, and permits public presentations/excerpts only when they are not for financial gain. The original 1989 copyright page also contains a DATA BECKER all-rights-reserved notice.

Treat the scan as a research source, not as project content. Do not commit the PDF, OCR dump, page images, or substantial verbatim passages. Commit only bibliographic metadata, concise original summaries, technical facts, and printed-page citations. If future redistribution or publication of excerpts is considered, obtain explicit rights clarification rather than inferring that all rights have expired.

## Initial visual verification

The PDF text layer was extracted locally and checked against rendered page images for the following pages. This is a targeted first pass, not a full manual audit.

| Printed page | PDF page in supplied scan | Verified subject | Initial result |
|---:|---:|---|---|
| front matter | 7 | Edition and copyright | 1st edition 1989; ISBN 3-89011-276-5; DATA BECKER; original rights notice |
| front matter | 9 | Foreword to first edition | Explicitly identifies Amiga GFA-BASIC version 3.0 and the three authors |
| 46–50 | 46–50 | Numeric, Boolean, and string variables | Visual check completed; key type and storage rules recorded below |
| 213–214 | 213–214 | Single-bit and bit-width functions | Visual check completed; signatures and bit numbering recorded below |

### Variable and value model (printed pp. 46–50)

- The book distinguishes numeric/value variables from alphanumeric/text/string variables.
- A real-valued numeric variable uses the bare variable name (no type suffix), occupies 8 bytes, and is described as providing up to 13 significant decimal digits. Assignments beyond that precision are rounded. The book also documents an exponent range on printed p. 47.
- Integer variables use the `%` suffix, occupy 4 bytes, and are documented with the signed range −2,147,483,648 through 2,147,483,647. The text also mentions 1-byte and 2-byte integer variants in GFA BASIC 3.0; their suffixes and exact ranges need a separate visual/source check.
- Boolean variables use the `!` suffix, occupy 2 bytes, and normalize assigned values to only 0 (false) or −1 (true); any nonzero assigned value becomes −1.
- String variables use the `$` suffix. The documented maximum length is 32,767 characters. Storage is proportional to the actual string length, plus a 6-byte descriptor per string variable. The descriptor stores the string address and length.
- The book explains −1 as the true value using 32-bit two's-complement representation. Do not substitute the more common BASIC convention of true = +1.
- The OCR text and rendered images agree for the points above. This verifies the cited rules as statements in this book; it does not yet prove that every interpreter build implements them identically.

### Bit operations (printed pp. 213–214)

The page images confirm these signatures and descriptions:

| Function | Signature | Source description |
|---|---|---|
| `BCHG` | `Var=BCHG(Wert,Bit)` | Toggle the selected bit |
| `BCLR` | `Var=BCLR(Wert,Bit)` | Clear the selected bit |
| `BSET` | `Var=BSET(Wert,Bit)` | Set the selected bit |
| `BTST` | `Var=BTST(Wert,Bit)` | Return −1 if selected bit is set, otherwise 0 |
| `BYTE` | `Var=BYTE(Wert)` | Return the low 8 bits unsigned |
| `CARD` | `Var=CARD(Wert)` | Return the low 16 bits unsigned |
| `EQV` | `Var=EQV(Wert1,Wert2)` | Logical equivalence operation |

Additional source details on printed p. 213: bit numbering starts at the right with bit 0; the text says values above bit 31 are not processed correctly, and values greater than the unsigned 32-bit maximum are not processed. The page describes the supported width as 32 bits. Keep these as documented limits; runtime probes are still needed to establish edge-case behavior precisely.

## Chapter/topic map

The table of contents and OCR text provide this initial map. Printed page numbers are the book's pagination.

| Topic | Printed pages |
|---|---:|
| Amiga and GFA-BASIC introduction | 17–24 |
| BASIC fundamentals, variables, arrays, loops, comparisons | 25–70 |
| Input/output, screen, disks, files, AmigaDOS, ports, printer, sound | 71–140 |
| Program structure, declarations, procedures, assembler/C/PRG calls | 141–188 |
| String operations | 189–196 |
| Arithmetic, math, numeric/trigonometric functions, bit operations, random | 197–220 |
| Graphics, drawing, display memory, animation | 221–272 |
| Data conversion and number systems | 273–284 |
| Arrays, memory, memory management, pointers, Exec library | 285–324 |
| Program control, errors, date/time, multitasking, debugging | 325–358 |
| Program/user interaction | 359–370 |
| Window and screen programming | 371–386 |
| Menus and event monitoring | 387–404 |
| Appendices and index | 405–end |

## Research implications and next steps

1. Keep `AM-BOOK-DE-1989` as a distinct 3.0 source identity. Keep it separate from the French translation (ISBN 2-86899-207-2) and the 1990 Amiga 3.x/V3.03 edition (ISBN 3-89011-399-0).
2. Find and identify an Amiga GFA-BASIC 3.5 primary source before claiming 3.5 compatibility for any feature.
3. Use the German 3.0 scan to cross-check OCR-sensitive syntax in the French source, especially punctuation, type suffixes, parameter order, and bit-operation examples; label the results as 3.0 until confirmed for 3.5.
4. Continue page-image review across declarations, control flow, I/O, memory, pointers, and API sections. The initial review above does not make the whole book visually verified.
5. Record source claims separately from executable observations. Build conformance probes only after a rule's exact syntax and expected result are clear, and record the exact 3.5 build used.
6. Keep AmigaDOS/Exec/graphics/window/menu/event APIs distinct from the portable language core.
7. Do not treat the 1989 interpreter 3.0 book as a complete specification for Amiga 3.5 or later compiler/runtime behavior.

## Citation convention

Use citations in this form:

`AM-BOOK-DE-1989, Amiga GFA-BASIC 3.0, printed p. N or §N`

For visual verification, record both the printed page and (in private research notes, if needed) the corresponding PDF viewer page.
