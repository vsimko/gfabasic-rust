# Compatibility testing and release criteria

- Status: Proposed; release gates remain open under issue #10.

## Conformance corpus

The initial source corpus is maintained in [tests/corpus/](../../tests/corpus/README.md). It currently contains research-only Amiga GFA BASIC 3.0 candidate fixtures. Candidate status means that page-image and/or interpreter validation is still required; these files are not compiler acceptance tests. See [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42).

See the [current compatibility matrix](COMPATIBILITY-MATRIX.md) for the explicit statement of which dialects, profiles, and targets are not yet supported.

## Test layers

1. Source loading and diagnostics.
2. Lexer and parser unit tests.
3. AST and semantic analysis tests.
4. Typed IR verification and lowering tests.
5. LLVM code-generation tests.
6. Runtime core unit tests.
7. API profile and platform integration tests.
8. End-to-end source-to-executable tests.
9. Manual-derived conformance tests for verified rules.
10. Regression tests for every fixed defect.

Each conformance test must identify dialect/version, API profile, expected behavior, and source page/section or trusted reference implementation. A fixture does not become authoritative merely because it passes.

Fix locale, timezone, random seeds, environment variables, temporary paths, and external resources where they affect test behavior. Mark nondeterministic tests and isolate them.

A release must state compiler version, supported dialect/version subset, target, API profile statuses, known deviations, and required runtime dependencies. Do not claim full compatibility based only on unit-test coverage or successful compilation.

Tracking: [issue #10](https://github.com/vsimko/gfabasic-rust/issues/10).
