# Version-scoped language-construct catalogue

This catalogue is deliberately separate from the existing API registry and regression corpus. It describes one semantic construct per directory and records version-specific expectations, probes, native execution observations, and Rust implementation status without conflating them.

## Layout

```text
tests/language-constructs/
  README.md
  manifest.schema.json
  fact-function/
    manifest.yaml
    probe-3.5.LST
    probe-3.52.LST
  mat-one-statement/
    manifest.yaml
    probe-3.5.LST
    probe-3.52.LST
```

A directory name is the stable construct ID. It must match `construct.id` and use lowercase kebab-case. A directory may contain only one semantic construct; use multiple `syntax_forms` only when the forms exercise the same construct. Split constructs with different semantics into separate directories.

## Manifest schema version 1

Every `manifest.yaml` is UTF-8 YAML validated against [manifest.schema.json](manifest.schema.json). The JSON Schema defines the structure; the repository validator adds cross-field, evidence-reference, filesystem-safety, and probe-integrity checks.

Every manifest has these required top-level fields:

| Field | Meaning |
|---|---|
| `manifest_schema_version` | Integer, exactly `1` for this contract. |
| `construct` | Stable ID, kind, display name, summary, and syntax forms. |
| `evidence` | Source records. Each record has a unique ID, source ID, location, kind, status, and a concise claim. |
| `versions` | A mapping containing the exact string keys `"3.041U"`, `"3.5"`, and `"3.52"`. Quote these keys; do not let YAML coerce version identifiers. |

### Construct fields

- `construct.id`: required, unique lowercase kebab-case identifier; must equal the containing directory name.
- `construct.kind`: one of `statement`, `function`, `operator`, `declaration`, `special-form`, or `command`.
- `construct.name` and `construct.summary`: non-empty strings.
- `construct.syntax_forms`: non-empty list of objects with `text` and a non-empty list of exact version strings in `versions`.

### Evidence records

Each evidence item has:

- `id`: unique kebab-case ID within the manifest.
- `source_id`: stable source identifier or repository record ID.
- `location`: printed page/section, repository path, workflow job, or other precise locator.
- `kind`: `primary-manual`, `repository-adr`, `native-observation`, `repository-code`, or `secondary-reference`.
- `status`: `page-image-verified`, `text-verified`, `direct-observation`, `repository-record`, `secondary`, `unknown`, or `disputed`.
- `claim`: the narrow statement supported by this record.
- `url`: optional stable repository/workflow URL.

Evidence status describes the source record only. It does not establish that a construct is implemented or that a native interpreter was executed.

### Per-version fields

Every exact version key has:

- `applicability`: `applicable`, `not-applicable`, or `unknown`.
- `expectation.status`: `documented`, `observed`, `unknown`, or `not-applicable`.
- `expectation.summary`: non-empty statement, including an explicit explanation when unknown.
- `expectation.evidence_refs`: IDs from the manifest's `evidence` list. Documented/observed expectations require at least one reference; unknown/not-applicable expectations use an empty list.
- `probe`: null or a basename relative to the construct directory. An applicable version must map to its own `.LST` probe.
- `execution.status`: exactly one of the ADR result values `PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`.
- `execution.observed`: null for `UNTESTED`; otherwise the concise observed output/behavior.
- `execution.run_url`: null for `UNTESTED`; otherwise a stable URL to the native run.
- `execution.probe_source`: optional repository-relative path to the exact source file used by an existing observation. It can differ from the isolated `probe` when a previous shared regression suite already exercised the construct.
- `execution.evidence_refs`: evidence IDs supporting the observation.
- `implementation.status`: one of `not-targeted`, `not-started`, `syntax-only`, `prototype`, `production`, or `unknown`.
- `implementation.refs`: repository paths supporting the implementation status.

The `implementation` field describes the current Rust implementation, not the original interpreter. The `execution` field describes native interpreter evidence, not a Rust unit test. A Rust prototype test must never be recorded as a native interpreter PASS.

### Path and identity rules

1. Manifest filename is exactly `manifest.yaml`.
2. `construct.id` must equal the directory name and match `^[a-z0-9]+(?:-[a-z0-9]+)*$`.
3. Probe paths are basenames relative to the construct directory, must exist as regular files, and must use the `.LST` extension.
4. A version marked `applicable` must map to a probe. Unknown or not-applicable versions must not be assigned a guessed probe.
5. `execution.probe_source` and implementation references are repository-relative paths and must point to existing files.
6. Every evidence reference must resolve to exactly one evidence ID in the same manifest.
7. Version identifiers are exact strings. Do not normalize `3.041U`, `3.5`, or `3.52`.
8. Expected behavior is not evidence. Evidence is not implementation status. A probe path is not proof that it ran.
9. Duplicate YAML keys, symlink traversal, unreferenced probe files, and path traversal are rejected.

## Validator and change selection

Run from the repository root:

```sh
python scripts/select_language_constructs.py --validate-only
python scripts/select_language_constructs.py --changed-path tests/language-constructs/fact-function/manifest.yaml
python scripts/select_language_constructs.py --all-constructs
python -m unittest discover -s tests/tooling -p "test_language_constructs.py"
```

The CLI emits deterministic JSON with `validate_all` and an `include` array of exact `construct`, `version`, and `probe` tuples. Changed files inside one construct select only that construct's applicable versions. A deleted construct selects no native jobs. A schema, validator, or catalogue README change requests full metadata validation but deliberately selects no native jobs. `--all-constructs` is an explicit full-selection mode for manual dispatch; it does not run implicitly.

Issue [#571](https://github.com/vsimko/gfabasic-rust/issues/571) implements validation and change selection. Issue [#572](https://github.com/vsimko/gfabasic-rust/issues/572) integrates the selected pairs into native CI. The new construct catalogue is metadata for selecting and interpreting tests; it does not add parser, IR, runtime, or API-registry behavior.
