# Language-construct manifest contract

Each independently testable GFA-BASIC construct has one directory named by a stable lowercase kebab-case ID:

```text
tests/language-constructs/<construct-id>/
  manifest.yaml
  probe.LST                 # or a version-specific probe path
  README.md                 # optional rationale
```

## Schema version 1

`manifest.yaml` is a YAML mapping validated against [manifest.schema.json](manifest.schema.json). The schema is JSON Schema Draft 2020-12. The repository validator additionally rejects duplicate YAML keys, symlinks, non-canonical paths, unlisted files, and IDs that do not match their directory.

Required top-level fields:

- `schema_version`: integer `1`.
- `id`: exact directory name, lowercase kebab-case.
- `name`: concise human-readable construct name.
- `kind`: `statement`, `function`, `operator`, `declaration`, or `special_form`.
- `classification`: one of `language_core`, `runtime`, `graphics`, `io`, or `system_interface`.
- `evidence`: documentary evidence status and zero or more source references. Each reference identifies a registered `source_key`, locator, claim, and exact version scope. An empty list means no documentary evidence is currently recorded; it does not mean unsupported.
- `versions`: exactly the keys `3.041U`, `3.5`, and `3.52`. Each entry states applicability, expectation, probe path (or explicit null), and observed result independently.
- `implementation`: Rust implementation status, separate from language evidence and native interpreter results.

### Version entry

`applicability` is `applicable`, `not_applicable`, or `unknown`. If applicable, provide a repository-relative `.LST` path and an expectation with status `documented`, `inferred`, or `unknown`. For unavailable or unverified cases, use explicit `unknown` values rather than omitting the version.

`observed` uses `PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`. New manifests should normally start at `UNTESTED`. An observed result may include an artifact URL/path and run identifier; never invent either.

### Path and evidence rules

- All probe paths are relative to the repository root, use forward slashes, remain inside the construct directory, and end in `.LST`.
- A probe path must exist and must not traverse a symlink.
- Every probe file in a construct directory must be referenced by at least one version entry; every referenced probe must exist.
- Evidence `source_key` values must exist in `docs/reference-sources.md`. Record printed page/section locators and exact version scope; do not cite an OCR hit as primary evidence.
- Manifest expectations are not evidence. A successful manifest validation or host-side CI job is not interpreter conformance.
- Do not add historical listings, interpreter binaries, ROMs, disk images, or other copyrighted assets.

## Examples

- [`expression-function`](expression-function/manifest.yaml): simple function construct.
- [`if-then-else`](if-then-else/manifest.yaml): compound statement construct.

These examples are structural templates. Their expectations and native results remain explicitly unknown/untested; they do not assert interpreter support.

## Validation

From the repository root:

```sh
python -m pip install PyYAML==6.0.2
python scripts/validate_language_constructs.py
python -m unittest scripts.tests.test_validate_language_constructs
```

The validator checks manifest shape, exact version keys, enums, source-register references, directory/ID agreement, safe probe paths, and complete probe-file references. It does not parse or execute BASIC source. Change-scoped selection and native parallel execution belong to [#571](https://github.com/vsimko/gfabasic-rust/issues/571) and [#572](https://github.com/vsimko/gfabasic-rust/issues/572).
