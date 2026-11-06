# Formatter style and current support

- Status: **Prototype style for the implemented frontend subset**.
- Tracking: [issue #208](https://github.com/vsimko/gfabasic-rust/issues/208).
- Architecture boundary: [ADR 0007](adr/0007-compiler-pipeline-and-ir.md).

## Scope

The formatter is a library feature in `gfabasic-compiler::formatter`. It consumes source text and a source ID, validates the whole document through the lossless parser, and returns formatted text or the parser's diagnostics. It is independent of the CLI, LLVM, generated-program runtime, and SDL3.

The currently supported and formatted statements are the prototype assignment forms `FACT`, `VARIAT`, `COMBIN`, and `PRINT`, with the current prototype variable subset. The formatter rejects every construct the parser cannot validate. Comments, string and DATA payloads, broader identifier forms, and nested blocks are not accepted by the formatter yet.

## Evidence boundary

The supplied sources must be distinguished by the interpreter version they actually document:

- *GFA BASIC 3.5 Interpreter* identifies itself as software version 3.5, but its introduction says the manual was originally printed for version 3.0 and that chapter 13 is the 3.5 addendum. Its comment descriptions occur in the inherited base material (printed pp. 1-8 and 7-35), not in the 3.5 addendum. They are therefore not sufficient evidence to enable comment syntax in the 3.5 lexer.
- *Das große GFA-BASIC-Buch zum Amiga* has the cover wording “Bis Version 3.5”, but its foreword describes the V3.03 interpreter. Its comment descriptions (printed pp. 167 and 169) are useful Amiga V3.03 corroboration, not proof of 3.5 behavior.

Consequently, REM, apostrophe, and trailing `!` comment syntax remain research findings, not accepted production lexer behavior. The books describe `!` as unavailable on DATA lines because it is interpreted as DATA text; this is not yet a reason to implement DATA parsing. A specifically 3.5-marked grammar passage or reproducible observation from the 3.5 interpreter is required before broadening accepted syntax. The same evidence gate applies to string and DATA payload grammar.

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

Formatting is all-or-nothing: parsing/lexing must succeed before output is returned. The library API performs no file writes, so a failed format cannot modify the input. The CLI writes only after successful formatting; file output uses a sibling temporary file and atomic rename. `--check` never writes. The formatter normalizes line endings to LF and emits exactly one final newline.

## Regression coverage

Tests cover:

- Golden output for each currently supported statement: FACT, VARIAT, COMBIN, and PRINT.
- Canonical whitespace and LF output, including CRLF input, blank lines, and missing final newline.
- Idempotence across a small supported-source corpus.
- Parsed-program meaning before and after formatting for each supported statement and a combined program.
- Rejection of strings, REM comments, apostrophe comments, trailing `!` comments, and DATA statements while their grammar remains outside the supported subset.

This is focused regression coverage, not a corpus-wide canonical-format check or a full-language conformance suite.

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

## CLI

The `gfabasic fmt` command uses this library directly. It writes formatted text to stdout by default; `--check` returns 1 when formatting would change the file and never writes; `-o <path>` writes to a separate destination; and `--in-place` explicitly replaces the source. The two write modes create a sibling temporary file, flush it, and rename it over the destination. Formatting diagnostics and read failures return before any write. `--check` cannot be combined with either write option, and `-o` cannot be combined with `--in-place`.

## Next increments

1. Verify comment, string/DATA, identifier, and broader grammar rules against the Amiga 3.5 interpreter or specifically 3.5-marked evidence before accepting them.
2. Add delimiter-aware, lossless literal/DATA support only after that evidence exists.
3. Add corpus-wide formatting checks after fixture policy and canonical style are established.
4. Share the same formatter implementation with LSP document/range formatting; editor edits must be version-bound as specified by issue #208 and ADR 0012.
