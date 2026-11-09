# Amiga GFA BASIC 3.0 — source composition and editor evidence

Source: *GFA BASIC 3.5 Interpreter*, Chapter 1, “Introduction and editor”, printed pp. 1-9–1-23, plus the explicit 3.5 editor update on printed p. 1-25. The manual's introductory note identifies Chapters 2–12 as the original 3.0 material and the later addendum as 3.5-specific. The observations below are therefore split by version and are not promoted to 3.5 lexical rules unless the addendum explicitly says so.

## Program-file formats and composition (3.0 base manual)

- Editor **Load/Save** uses a tokenized GFA program format, normally with the `.GFA` extension. The manual says this format loads and saves quickly. (1-14)
- **Merge** imports ASCII-format data into the current program beginning at the line before the editor cursor. The normal extension is `.LST`. The editor displays `=>` on lines it cannot interpret. (1-15)
- **Save,A** exports the current program in ASCII format; the result can later be loaded through Merge and normally uses `.LST`. If a destination already exists, the editor uses `.BAK` as a backup extension. (1-15)
- The block editor can write a selected block as an ASCII data file. (1-17)
- The manual distinguishes the editor's physical program rows and editor operations from direct mode. It describes direct mode as immediate command execution after Return/Enter and notes that some commands, including loop commands, are not usable there. (1-20)

These are editor/file-workflow facts. They do not define source encoding, byte-order marks, accepted line endings, statement separators, line continuation, or a complete grammar for `.LST` files.

## Editor structure and procedures (3.0 base manual)

- The editor is structure-aware: procedure bodies can be hidden/unhidden with Help, and the editor can check whether loops and subroutines are properly closed using Test (F10). (1-9–1-10, 1-21)
- Editor marks set with Control+number and returned to with Alt+number are explicitly described as editor-only marks, distinct from program labels used by GOTO/RESTORE. (1-13)
- Tab stops in this base editor are eight characters apart. Return/Enter moves to the beginning of the next line. Escape switches to direct mode. (1-13)
- Search and replace operate on editor text. Text in folded/hidden procedures is not found by the described search operation. (1-18–1-19)

These behaviors describe the historical editor, not necessarily source-language tokenization or parser restrictions.

## Explicit 3.5 editor delta

The 3.5 addendum explicitly states (printed p. 1-25):

- Two additional bytes are used per program row compared with editor versions 3.0–3.041; the stated effect is faster backward scrolling and the ability to fold functions.
- Search can find procedure/function header rows even when those procedures/functions are closed.
- Listing shifts labels two characters to the left, like CASE.
- Tab moves to the next tab position; Ctrl+Tab moves to the previous one; LeftShift+Tab inserts spaces to the next tab position; RightShift+Tab deletes blank spaces in the current line up to/from the cursor (the wording does not define a more exact deletion boundary).

This explicit 3.5 delta is editor behavior. It does not establish new source grammar or program semantics.

## Version and evidence classification

| Topic | Source evidence | Status for selected Amiga 3.5 target |
|---|---|---|
| Tokenized `.GFA` program file | 3.0 editor, p. 1-14 | Inherited candidate; exact 3.5 file compatibility not independently stated here |
| ASCII `.LST` Merge/Save,A | 3.0 editor, p. 1-15 | Inherited candidate; Merge grammar and path behavior need 3.5 confirmation |
| Block-to-ASCII export | 3.0 editor, p. 1-17 | Inherited editor candidate |
| Direct mode restrictions | 3.0 editor, p. 1-20 | Inherited editor/runtime candidate |
| Procedure folding and structural Test | 3.0 editor, pp. 1-9–1-10, 1-21 | 3.0 baseline; 3.5 folding/search changes are explicit, other behavior not independently reconfirmed |
| Editor tab behavior | 3.0 editor, p. 1-13; 3.5 update, p. 1-25 | 3.5 tab actions explicitly documented; tab-stop width and editing details beyond the addendum remain inherited/uncertain |

## Explicitly unresolved lexical/source questions

The supplied source material reviewed so far does not establish a complete 3.5 contract for:

- source character encoding/code page, BOM handling, or invalid-byte behavior;
- accepted line endings and physical-line/statement-boundary rules;
- whether editor rows correspond one-to-one with parser statements in all input modes;
- identifier character repertoire, case sensitivity, and normalization;
- string literal escaping and complete numeric literal grammar;
- continuation characters, statement separators, and whitespace significance;
- exact Merge handling of duplicate labels/procedures, ordering, and relative paths.

Do not infer these rules from the tokenized `.GFA` format, the ASCII `.LST` extension, the Rust source loader, or the editor's visual layout. They remain unresolved for the Amiga 3.5 language inventory until an explicit 3.5 source passage or a reproducible observation from the named interpreter build is recorded.

## Relationship to issue #19

This closes the source-composition/editor topic discovery for the supplied manual and records the explicit 3.5 editor delta. It does not close the lexical/syntax inventory: source encoding, grammar, literals, identifiers and Merge semantics remain unresolved. Detailed parser requirements belong in #20 and #120; the inventory in #19 must retain these unknown states.
