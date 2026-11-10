---
type: "Research Note"
title: "Amiga GFA-BASIC 3.5 primary reference and target decision"
---

# Amiga GFA-BASIC 3.5 primary reference and target decision

Decision: target the **Amiga GFA-BASIC 3.5 interpreter and its documented 3.5 addendum**. Do not select 3.51 or 3.52 merely from secondary catalog listings; the available 3.5 interpreter manual is the governing reference in this project.

## Available source set

### GFA BASIC 3.5 Interpreter

File supplied in the project workspace: `GFA_Basic_3.5_Interpreter.pdf` (469 PDF pages). Its introductory page states: “The software included with this manual is GFA BASIC 3.5”; it also explains that the manual was originally printed for version 3.0 and that Chapter 13 is an addendum describing the extra features provided in version 3.5. Chapter 13 is headed “GFA-BASIC 3.5 Addendum”. The supplied 469-page manual presents the 3.5 addendum as Chapter 13. Its printed pp. 13-33–13-36 include the FACT/VARIAT/COMBIN, OPENW, PCOLOR, DATA-pointer, special-variable, and GURU material. The separately supplied 487-page combined update scan divides the same addendum into Chapter 13 (linear matrix operations) and Chapter 14, “Further new commands in Version 3.5” (printed pp. 14-1–14-7). For the special names, the supplied manual prints `_DATA`/`_DATA=` on p. 13-34 and `_0`…`_9` on p. 13-35; the combined scan prints the corresponding sections on pp. 14-5–14-6. These are different chapter/page layouts for the same 3.5 update material; the crosswalk is summarized in the [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md). This is the primary version-specific source for the 3.5 interpreter target.

The addendum documents editor changes: two extra bytes per program row compared with editor versions 3.0–3.041, faster backward scrolling, folding functions, searching procedure/function headers while closed, and labels shifted two characters left during listing. The explicit editor update is printed p. 1-25; its tab operations are also recorded in the addendum inventory. It adds tab navigation/editing operations. It introduces linear operations for one- and two-dimensional floating-point fields, including MAT BASE, MAT CLR, MAT SET, MAT ONE, MAT READ, MAT PRINT, MAT INPUT, MAT CPY, MAT XCPY, and MAT TRANS. The supplied 469-page interpreter manual's explicitly identified 3.5 material (editor update on printed p. 1-25 and Chapter 13, printed pp. 13-1–13-36; the separately supplied combined update scan is cross-referenced below) has been visually checked against page images, with the combined 487-page update scan used as an independent pagination witness. The page-image pass is recorded in issue #65 and the [addendum inventory](amiga-gfa-basic-35-addendum-inventory.md). The MAT INV operand direction is resolved by the detailed definition and worked example; see the addendum inventory for the exact page citation and evidence. This full 3.5-addendum review does not establish that every rule in the 3.0 base manual was revalidated for 3.5. The corpus currently contains 13 independently authored Amiga 3.5 fixtures marked `verified` from cited page-image evidence for selected matrix and combinatorics features. The corpus remains `research_only`: it has no general runner, and none of these fixtures establishes original-interpreter runtime conformance. Exact signatures, semantics, and the resolved `MAT INV` operand direction are recorded in the [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md).

### Das große GFA-BASIC-Buch zum Amiga

File supplied in the project workspace: `Das Grosse Buch zu GFA-Basic 3.5 Amiga.pdf` (515 PDF pages). Title/copyright pages identify DATA BECKER, 1990, ISBN 3-89011-399-0. Its foreword says the book documents, tests, and explains the commands/functions of Amiga GFA-BASIC version 3.0, and separately covers the then newly released GFA-BASIC compiler. The cover says “Bis Version 3.5 und Compiler 3.5”, but the foreword's explicit statement about the main command reference is version 3.0. Treat this book as the broad Amiga platform and compiler reference, not as sole proof of 3.5 interpreter additions.

### Das große GFA-BASIC-Buch (1989)

The separate 1989 first edition, ISBN 3-89011-276-5, explicitly documents Amiga GFA-BASIC 3.0. Retain it as historical corroboration only; do not label it a 3.5 source.

## Compatibility evidence rules

1. Use the 3.5 Interpreter manual and its Chapter 13 addendum as the normative reference for the selected 3.5 interpreter.
2. Use the 1990 German Amiga book for Amiga-specific environment, APIs, examples, and compiler coverage; preserve its 3.0/3.03 scope for the main command reference, but use its explicitly 3.5-labelled Chapters 18–19 as secondary corroboration for the additions listed in the [German-book 3.5 cross-check](amiga-gfa-basic-35-german-book-crosscheck.md).
3. Use the 1989 German book as an earlier 3.0 cross-check.
4. Mark any 3.0-documented behavior not explicitly reconfirmed in the 3.5 manual as inherited-from-3.0 / confirmation-needed, rather than assuming it changed or remained unchanged.
5. Keep interpreter and compiler behavior separate. The compiler is described in the German Amiga book; do not infer interpreter equivalence for compiler-only behavior.
6. Do not import Atari ST-only 3.5 claims into the Amiga target unless corroborated by the supplied Amiga materials.

## Unresolved source-language evidence

The current extraction identifies the 3.5 source and its version-specific additions, but it does **not** yet establish the complete source-language contract. In particular, no 3.5-specific rule is currently claimed here for:

- source character encoding/code page, BOM handling, or invalid-byte behavior;
- accepted line endings and physical-line/statement-boundary rules;
- identifier character repertoire and case handling;
- string literal escaping and complete numeric literal grammar;
- the complete operator set, precedence, and evaluation order;
- MERGE/source-composition behavior and relative-path resolution.

These areas remain open under [issue #20](https://github.com/vsimko/gfabasic-rust/issues/20) and [issue #120](https://github.com/vsimko/gfabasic-rust/issues/120). The generic strict UTF-8 source loader is a transport primitive, not evidence that GFA BASIC source files are UTF-8. Do not infer language encoding, BOM, line-ending, or composition semantics from the Rust loader or from OCR-only 3.0 notes. Promote a rule only after its 3.5 source passage or a named interpreter observation is recorded.

## Next work

- Continue declaration-level extraction of the 3.0 base manual's Chapter 4 and Chapter 6–7 command families, keeping each candidate tied to a printed page and explicitly historical pending 3.5 reconciliation.
- Reconcile base-manual candidates against explicit 3.5 addendum passages or controlled interpreter observations; do not treat absence from the addendum as evidence of removal.
- The Chapter 13–14 and editor-update page-image review is complete (tracked in #65). Continue declaration-level reconciliation of the 3.0 base-manual candidates; preserve the resolved MAT INV direction and other explicitly recorded source discrepancies.
- Map the German Amiga book's command reference and compiler chapter separately from interpreter behavior, preserving its stated 3.0 command-reference scope.
- Build conformance tests from exact source examples and separately record actual interpreter observations.


### Independently supplied 3.0 manual with bound 3.5 update (selected pages page-image checked)

A second scan, supplied as `GFA-Basic 3.0 + 3.5 Upd (Amiga).pdf` (487 PDF pages), independently confirms the structure and version boundary of the 3.5 material:

- PDF p. 3 title page: `GFA BASIC 3.0 Interpreter`, platform Amiga.
- PDF p. 4 imprint: ©1988 GFA Systemtechnik GmbH; program author Frank Ostrowski; second edition, December 1988. The same page restricts copying of the 3.0 interpreter except for the purchaser's personal use and permits redistribution of the Run-Only interpreter; this is not blanket permission to redistribute the manual.
- PDF p. 7, headed `Erweiterte Version GFA-BASIC 3.5 Amiga`: explicitly says to add Chapter 13 (linear vector/matrix operations, printed pp. 13-1–13-41), Chapter 14 (further commands in version 3.5, printed pp. 14-1–14-8), and the editor changes on printed p. 1-25 to the GFA-BASIC 3.0 Interpreter manual.
- PDF pp. 479–485 contain the printed Chapter 14 pages 14-1–14-7; the end of Chapter 13 is on PDF pp. 450–477; the explicit editor update printed p. 1-25 is PDF p. 41. These page mappings are scan-specific; cite printed addendum page numbers in specifications.

This scan is a separately supplied witness for the same 3.0-base-plus-3.5-addendum structure. It does **not** change the version boundary: the base manual is explicitly 3.0, and only the identified addendum/editor changes are explicitly marked 3.5. Its page 3 title and page 4 imprint were visually inspected. The complete Chapter 13 and Chapter 14 page-image review, together with the explicit editor update on printed p. 1-25, is recorded as complete in issue #65. The `MAT INV` operand direction is resolved as `MAT INV destination()=source()`: the detailed 3.5 definition and worked example (printed p. 13-30) take precedence over the reversed command-summary line on p. 13-16, which is treated as a source typo. The German 1990 book independently corroborates destination-first order on p. 475, but remains historical 3.0/V3.03 evidence. Treat it as a separate supplementary source, not as this manual.


## Base-manual versus addendum boundary

The base chapters (2–12) describe the manual's original 3.0 baseline. The 3.5-specific additions are identified separately in the addendum. A topic's presence in this PDF is therefore not, by itself, proof that its detailed behavior is unchanged or applicable in interpreter 3.5. See the [version-scoped 3.0 base-manual inventory](amiga-gfa-basic-3.0-base-inventory.md) and issue #298. The normative 3.5 catalogue must contain only rules explicitly confirmed for 3.5.
