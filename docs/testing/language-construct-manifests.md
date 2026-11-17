---
type: "Testing Guide"
title: "Language-construct manifest contract"
---

# Language-construct manifest contract

## Purpose and boundaries

A language-construct manifest is a version-scoped description of one independently testable GFA BASIC construct. It supports targeted native interpreter selection and keeps documentary expectations separate from observed interpreter behavior.

This catalogue is distinct from:

- `tests/corpus/manifest.yaml` and its fixture sidecars, which describe the research/conformance corpus;
- `api/registry/` TOML declarations, which remain the canonical API/language registry;
- `tests/interpreter-probes/`, which contains existing native-probe metadata and harness inputs.

Do not migrate or duplicate those formats here. A manifest is metadata, not proof that a construct is implemented, supported, or conformant.

## Layout and identity

Use one directory per construct:

```text
tests/language-constructs/
  schema-v1.json
  <construct-id>/
    manifest.yaml
    probe.LST                 # optional shared probe
    probe-<version>.LST       # optional version-specific probe
```

The construct ID and directory name must match. IDs use lowercase ASCII letters and digits separated by single hyphens (for example, `mat-one-statement`). Do not encode version numbers in an ID unless the construct itself is version-specific. Keep probe files inside their construct directory; manifest paths are basenames, not paths into other directories.

## Manifest schema v1

Every `manifest.yaml` is a YAML mapping validated against [schema-v1.json](../../tests/language-constructs/schema-v1.json). Unknown fields are rejected. The schema requires:

- `schema_version: 1`;
- stable `id`, human-readable `name`, construct `kind`, and broad `classification`;
- one or more exact syntax `form` entries with their contexts;
- `evidence.status` and source references with source ID, edition, printed page locator, and locator description;
- an explicit entry for each exact interpreter version: `3.041U`, `3.5`, and `3.52`.

Each version entry contains three independent fields:

1. **expectation** — documentary expectation only: `supported`, `unsupported`, `different`, or `unknown`, with an explanation. This is not an execution result.
2. **probe** — optional basename of the `.LST` input intended for that exact version. Use `null` when no probe is defined. A shared probe may be referenced by more than one version only when the source is genuinely equivalent.
3. **observed** — actual run outcome: `PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`. New/unrun entries must use `UNTESTED`; include a run ID/artifact only when it exists.

All three versions must be present. Use `unknown` and `UNTESTED` when evidence or execution is unavailable; do not omit a version or infer behavior from an archive label, a sibling version, a compiler release, or a different platform. `PASS` means the observed behavior matched the version-specific expectation in a named native run, not that a Rust implementation passed a unit test.

## Evidence and version rules

- Keep evidence status independent from per-version expectations and observed results.
- Cite primary references with the source-register ID and printed page/section, not PDF viewer page numbers.
- A 3.0/3.03 source does not establish 3.5 behavior without an explicit 3.5 applicability decision.
- Keep Amiga 3.5, 3.041U, and 3.52 results separate. The 3.52 product label does not by itself identify the interpreter component version.
- Listing occurrences, successful parsing, formatter acceptance, and a passing Rust unit test are not native interpreter observations.
- User-defined symbols, external library routines, and Amiga OS calls must not be classified as built-in language constructs without primary evidence.

## Examples and validation

The examples in [tests/language-constructs/](../../tests/language-constructs/README.md) show a function expression and a statement. Their 3.5 expectations are grounded in the cited 3.5 addendum; all runtime outcomes remain `UNTESTED` until a matching native run is recorded.

Issue [#570](https://github.com/vsimko/gfabasic-rust/issues/570) defines this schema and directory contract. The change-selection validator and GitHub Actions matrix integration are separate work in [#571](https://github.com/vsimko/gfabasic-rust/issues/571) and [#572](https://github.com/vsimko/gfabasic-rust/issues/572).
