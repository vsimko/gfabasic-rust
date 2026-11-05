# Formatter style and current support

- Status: **Prototype style for the implemented frontend subset**.
- Tracking: [issue #208](https://github.com/vsimko/gfabasic-rust/issues/208).
- Architecture boundary: [ADR 0007](adr/0007-compiler-pipeline-and-ir.md).

## Scope

The formatter is a library feature in `gfabasic-compiler::formatter`. It consumes source text and a source ID, validates the whole document through the lossless parser, and returns formatted text or the parser's diagnostics. It does not depend on the CLI, LLVM, generated-program runtime, or SDL3.

At this stage, the parser accepts only the evidence-backed assignment forms `FACT`, `VARIAT`, `COMBIN`, and `PRINT` with the current prototype variable subset. The formatter rejects every construct the parser cannot validate. In particular, comments, string and DATA payloads, broader identifier forms, and nested blocks are not formatted until their Amiga GFA BASIC 3.5 lexical/grammar rules are verified and represented losslessly.

## Canonical prototype layout

For the supported subset only:

- One space on each side of assignment `=`.
- No spaces between an intrinsic name and its opening parenthesis, or just inside parentheses.
- No space before a comma and one space after a comma.
- One statement per line; empty lines are collapsed.
- LF line endings and exactly one final newline.
- Preserve the exact source spelling of every token, including identifiers, intrinsic names, and numeric literal text. No case conversion, transliteration, Unicode normalization, or identifier rewriting is performed.

These are project formatting choices, not claims about how the historical interpreter displayed or stored source. The formatter does not infer canonical keyword or builtin casing.

## Safety and determinism

Formatting is all-or-nothing: parsing/lexing must succeed before output is returned. The library API performs no file writes, so a failed format cannot modify the input. The same accepted source always produces byte-identical output. Callers that write files must use an atomic replacement strategy and must not write when formatting returns diagnostics.

The formatter currently normalizes line endings to LF. Preserving original line endings and adding explicit output policy options are deferred until the CLI contract is implemented.

## API

```rust
use gfabasic_compiler::formatter::format_source;
use gfabasic_compiler::source::SourceMap;

let source = "x=FACT(1)\\nPRINT   x\\n";
let mut sources = SourceMap::new();
let source_id = sources.add_source("example.bas", source);
let formatted = format_source(source_id, source)?;
# Ok::<(), Vec<gfabasic_compiler::diagnostics::Diagnostic>>(())
```

## Next increments

1. Add the CLI `fmt`, `fmt --check`, explicit output, and explicit in-place modes with atomic writes.
2. Extend lossless lexing and formatting only alongside verified comment, string/DATA, identifier, and grammar support.
3. Add corpus-wide formatting checks only after the supported fixture policy and canonical style are established.
4. Share the same formatter implementation with LSP document/range formatting; editor edits must be version-bound as specified by issue #208 and ADR 0012.
