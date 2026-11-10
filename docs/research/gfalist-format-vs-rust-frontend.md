---
type: "Research Note"
title: "gfalist format notes vs. the Rust frontend"
---

# gfalist format notes vs. the Rust frontend

## Purpose and evidence

This note compares the archived `gfalist` format description
(`examples/amiga/gfalist/doc/gfa_frmt.txt`) with the current
evidence-gated source-text frontend
(`crates/gfabasic-compiler/src/frontend.rs`).

The two components operate at different layers:

- The Rust frontend currently lexes textual source input (`.bas`-style text).
- `gfalist` reads tokenized binary GFA files (`.GFA`) and renders listings.

The existence of a token in the binary format does not by itself establish
the exact textual spelling, grammar, or runtime semantics of a source
construct. Conversely, accepting a textual spelling does not implement
binary `.GFA` decoding.

## What the archived format note documents

The archived German note describes a header offset-pointer table whose
entries point to variable-name groups, the first program line, editor
positions, address tables for 16 variable groups, and variable-table
boundaries.

It lists typed variable forms:

| Type family | Scalar spelling | Array spelling |
|---|---|---|
| 1 | `x#` | `x#()` |
| 2 | `x$` | `x$()` |
| 3 | `x%` | `x%()` |
| 4 | `x!` | `x!()` |
| 9 | `x&` | `x&()` |
| 10 | `x|` | `x|()` |

The note also lists function-token values 224–255 for typed variables,
labels, procedure names, function names, and corresponding second forms.
These are decoder-level token identifiers, not a complete source grammar.

The note's wording and structure are preserved as an archival format
reference. It is not an official GFA BASIC 3.5 language specification.

## Current Rust frontend coverage

The current lexer recognizes:

- ASCII alphabetic identifier candidates, followed by ASCII letters/digits;
- decimal integer literals limited to `i64`;
- `=`, parentheses, comma, `+`, `-`, `*`, and `#`;
- CR, LF, CRLF line boundaries and horizontal spaces/tabs;
- the explicitly documented Amiga 3.5 special names `_DATA` and `_0`–`_9`.

The current parser is deliberately narrower still: the first supported
vertical slice is literal `FACT`, `VARIAT`, `COMBIN` assignments and
selected matrix statements. Variables in the combinatorics slice are
restricted to `x`, `y`, and `z`.

The current identifier scanner does not include GFA type suffixes as part
of identifiers. For example, the archived format note's `x#` and `x$`
forms are not represented as typed identifier tokens: `#` is currently a
separate file-channel token and `$` is not recognized by the lexer.
Likewise, array suffix syntax and the binary variable-table representation
are not implemented by this lexer.

## Concrete gaps (not yet implementation requirements)

| Area | Archived note | Current frontend | Status |
|---|---|---|---|
| Scalar type suffixes | `# $ % ! & |` | Not a typed-name model; `#` is a separate token, other suffixes unsupported | Gap |
| Array type suffixes | `#() $() %() !() &() |()` | No general array-name syntax | Gap |
| Identifier grammar | Type-specific variable groups | ASCII identifier candidates plus exact 3.5 underscore forms | Incomplete comparison; do not generalize from the note |
| Binary header | Offset-pointer entries and variable-table boundaries | Text source spans only | Separate future decoder layer |
| Function-token IDs | 224–255 | No binary token-ID representation | Separate future decoder layer |
| General statements/expressions | Not fully specified by this short format note | Narrow evidence-gated subset | Not comparable from this note alone |

## Recommended engineering sequence

1. Keep binary `.GFA` decoding separate from textual lexing/parsing.
2. Preserve the archived `gfalist` note and its provenance; do not copy its
   token numbers into the source lexer.
3. Obtain and inspect the complete `gfalist` C source and its test fixtures
   before designing a binary decoder. Its README describes GFA BASIC 3.x
   input, but its implementation explicitly rejects some old file versions;
   this needs to be verified against actual files.
4. Add independently authored fixtures only after recording their exact
   source provenance and expected decoder output.
5. Treat suffix-bearing identifiers and array syntax as separate language
   frontend work, gated by the Amiga 3.5 manual or interpreter evidence.

## Non-goals of this comparison

This note does not claim that the format note is exhaustive, that the
listed token IDs are stable across all GFA versions, or that the Rust
frontend should immediately implement all listed types. It does not
establish runtime semantics for any command or function.
