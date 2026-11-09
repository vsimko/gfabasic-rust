---
type: "Research Note"
title: "Amiga GFA BASIC 3.5 — editor feature audit"
---

# Amiga GFA BASIC 3.5 — editor feature audit

Primary source: *GFA BASIC 3.5 Interpreter*, editor update printed p. 1-25. These are editor capabilities, not BASIC runtime language declarations.

## 1. Program-row storage and backward scrolling

- The 3.5 editor uses two additional bytes per program row compared with editor versions 3.0–3.041.
- The stated benefits are faster backward scrolling and support for folding functions.
- The addendum does not specify the row data structure, byte layout, or a measurable performance guarantee.

## 2. Function folding

- The 3.5 update identifies folding functions as a capability enabled by the editor's row-storage change.
- The source does not describe fold markers, persistence, nesting rules, or keyboard shortcuts.

## 3. Search of closed procedure/function headers

- Search can find procedure/function header rows even while those procedures/functions are closed (folded).
- The source does not define search matching rules, case sensitivity, wraparound, or whether search opens the matched body.

## 4. Label indentation in listings

- During listing, labels are shifted two characters to the left, as with `CASE`.
- This is a listing-format rule; it does not establish a change to label parsing or execution semantics.

## 5. Tab-key operations

| Key | Documented action |
|---|---|
| Tab | Move cursor to the next tabulator position |
| Ctrl+Tab | Move cursor to the previous tab position |
| LeftShift+Tab | Insert spaces to the next tab position |
| RightShift+Tab | Delete blank spaces in the current line “up to and from the cursor” |

The last description is ambiguous in the source; the exact deletion boundary should not be guessed. The source also does not specify tab-stop width or whether tab width is configurable.

## Compatibility boundary

All items above are explicitly identified as editor changes for the 3.5 update. They should be tracked separately from interpreter statements, functions, operators, and runtime semantics. No editor runtime observations are available, so exact interaction details beyond the stated actions remain source-limited.
