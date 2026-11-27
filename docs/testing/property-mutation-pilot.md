---
type: "Testing Guide"
title: "Generated property and mutation-testing pilot"
---

# Generated property and mutation-testing pilot

## Inventory and target selection

The property-test inventory for issue [#579](https://github.com/vsimko/gfabasic-rust/issues/579) found:

- `crates/gfabasic-compiler/src/source.rs` has focused unit tests for source IDs, UTF-8 spans, CRLF/LF/CR line endings, empty input, and UTF-8 loading. The generated property test lives in `crates/gfabasic-compiler/tests/source_properties.rs` so it exercises the public source-map API as an integration test.
- `fuzz/fuzz_targets/source_loader.rs` already exercises arbitrary byte input, strict UTF-8 rejection, source-map immutability on decode failure, and selected offsets (start, end, midpoint). The AFL++ campaign and its budgets remain governed by [FUZZING.md](../architecture/FUZZING.md).
- `scripts/tests/test_validate_api_registry.py` already has broad table-driven validation cases for schema errors, evidence scope, path traversal/symlinks, duplicate IDs/names, profile mismatches, and support claims.
- `scripts/tests/test_generate_api_catalogue.py` already covers deterministic output, empty/partial registries, a 1,000-entry synthetic registry, detail resolution, and stale output detection.

The selected property target is the stable `SourceFile` line-index interface. The integration test checks every byte offset for each generated text, rather than only the three offsets sampled by the existing fuzz target. It also cross-checks every reported line and line text against a separate test oracle.

## Generator contract

- Target: `SourceMap::add_source` and `SourceFile::{line_count,line_text,line_column}`.
- Seed: `0x5EED_35A1_2026` (fixed xorshift64 state; no external property-testing dependency).
- Bounds: 128 generated strings, at most 48 Unicode scalar values each.
- Alphabet: ASCII letters, two- and four-byte Unicode characters, CR, and LF.
- Properties: CR, LF, and CRLF line splitting; final empty lines; one-based line/column coordinates; Unicode-scalar columns; invalid UTF-8 byte offsets return no coordinate.
- Reproduction: run `cargo test -p gfabasic-compiler generated_source_text_has_consistent_lines_and_columns`. Failure messages include the seed, generated case, text, and byte offset.

This is a bounded deterministic property test, not a language-conformance test. It generates transport/source-map text only and makes no claim about GFA BASIC source encoding or syntax.

## Mutation testing

Mutation testing is intentionally excluded from pull-request and push CI because each mutant rebuilds and retests the Rust package. It runs in the dedicated **Rust mutation testing** workflow once per day at **02:00 UTC** (03:00 in Germany during CET and 04:00 during CEST). The workflow can also be started manually with `workflow_dispatch`.

The workflow accepts the exact scheduled-event payload `0 2 * * *`; it does not depend on the runner's local clock or timezone. Its only trigger events are `schedule` and `workflow_dispatch`, so mutation campaigns do not run on PR or push events.

The workflow uses the pinned `cargo-mutants` release and restricts mutation generation to the Rust implementation file `crates/gfabasic-compiler/src/source.rs`, testing only the `gfabasic-compiler` Rust package. The generated integration test remains part of that package's test suite. Python scripts, generated documentation, and other non-Rust implementations are out of scope. Results are uploaded as a workflow artifact for review.

The initial bounded pilot is the Unicode-column behavior in `SourceFile::line_column`: a byte-counting mutant (`prefix.len()`) must be detected by the generated deterministic test, while the committed implementation counts Unicode scalar values (`prefix.chars().count()`). The earlier one-line probe was applied only temporarily and has been reverted; its CI attempt stopped at rustfmt before tests ran, so it is **not** counted as a killed mutant.

This workflow establishes a narrow, scheduled Rust-only mutation baseline. It does not establish a repository-wide mutation score.
