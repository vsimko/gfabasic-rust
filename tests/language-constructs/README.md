# Version-scoped language-construct catalogue

This catalogue is deliberately separate from the existing API registry and regression corpus. It describes one semantic construct per directory and records version-specific expectations, probes, native execution observations, and Rust implementation status without conflating them.

## Layout

```text
tests/language-constructs/
  README.md
  fact-function/
    manifest.yaml
    probe-3.5.LST
    probe-3.52.LST
  mat-one-statement/
    manifest.yaml
    probe-3.5.LST
    probe-3.52.LST
  for-next/
    manifest.yaml
    probe-3.5.LST
  procedure/
    manifest.yaml
  user-function/
    manifest.yaml
```

A directory name is the stable construct ID. It must match `construct.id` and use lowercase kebab-case. A directory may contain only one semantic construct; use multiple `syntax_forms` only when the forms exercise the same construct. Split constructs with different semantics into separate directories.

## Manifest schema version 1

Every `manifest.yaml` is UTF-8 YAML with these required top-level fields. Schema version 1 is strict: unknown fields are rejected at every level so a typo cannot silently change selection.

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
- `probe`: null or a path relative to the construct directory. An applicable, testable version maps to its own `.LST` probe; version-specific syntax uses explicitly named files such as `probe-3.52.LST`.
- `execution.status`: exactly one of the ADR result values `PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`.
- `execution.observed`: null for `UNTESTED`; otherwise the concise observed output/behavior.
- `execution.run_url`: null for `UNTESTED`; otherwise a stable URL to the native run.
- `execution.probe_source`: null for `UNTESTED`; required for a native result. It is a repository-relative `.LST` path under `tests/` to the exact source file used by an existing observation. It can differ from the isolated `probe` when a previous shared regression suite already exercised the construct.
- `execution.evidence_refs`: evidence IDs supporting the observation.
- `implementation.status`: one of `not-targeted`, `not-started`, `syntax-only`, `prototype`, `production`, or `unknown`.
- `implementation.refs`: existing repository-relative file paths supporting the implementation status. They are required for `syntax-only`, `prototype`, and `production`, and empty for `not-targeted`, `not-started`, and `unknown`.

The `implementation` field describes the current Rust implementation, not the original interpreter. The `execution` field describes native interpreter evidence, not a Rust unit test. A Rust prototype test must never be recorded as a native interpreter PASS.

### Path and identity rules

1. Manifest filename is exactly `manifest.yaml`.
2. `construct.id` must equal the directory name and match `^[a-z0-9]+(?:-[a-z0-9]+)*$`.
3. Probe paths are relative to the construct directory, must stay inside it, must exist as regular files, and must use the `.LST` extension.
4. A version marked `applicable` must map to a probe and appear in at least one syntax form. Unknown or not-applicable versions must not be assigned a guessed probe; their expectation status must be `unknown` or `not-applicable` respectively. Probe files should be ASCII with LF line endings unless the construct explicitly tests encoding behavior.
5. `execution.probe_source` is provenance for an already observed run, not the selection path for future per-construct jobs. It must be repository-relative and point to an existing test source.
6. Every evidence reference must resolve to exactly one evidence ID in the same manifest.
7. Version identifiers are exact strings. Do not normalize `3.041U`, `3.5`, or `3.52`.
8. A native result other than `UNTESTED` requires observed output, a run URL, the source probe path, and evidence references. `UNTESTED` requires all four to be empty/null.
9. Expected behavior is not evidence. Evidence is not implementation status. A probe path is not proof that it ran.

## Examples

- `fact-function` is a function with a 3.5 manual-backed expectation and an observed 3.52 native result. The 3.52 observation came from the existing shared regression suite; the new isolated probe is mapped for future construct-level execution.
- `mat-one-statement` is a statement with a 3.5 page-image-backed expectation and a 3.52 shortcut form recorded in ADR-0017 and observed in the 3.52 native run. Its Rust implementation is currently syntax-only for 3.5.

Both examples explicitly leave 3.041U applicability unknown rather than assuming cross-version equivalence.

## Validator and selector CLI

Run the validator from the repository root:

```sh
python scripts/validate_language_constructs.py validate
```

The selector validates every manifest before emitting a deterministic matrix:

```sh
# Select only versions mapped to changed construct paths.
python scripts/validate_language_constructs.py select --base origin/main --head HEAD

# Explicit workflow_dispatch-style selection.
python scripts/validate_language_constructs.py select --construct fact-function
python scripts/validate_language_constructs.py select --construct fact-function --version 3.5
python scripts/validate_language_constructs.py select --all
python scripts/validate_language_constructs.py select --validate-only
```

A changed manifest selects only that construct's applicable versions; a changed probe selects only its mapped version. Deleted/renamed paths and multiple constructs are handled deterministically. Changes to this shared contract or validator perform full manifest validation but return an empty native matrix unless a construct directory also changed. All-native selection requires the explicit `--all` option. A manual workflow dispatch without a construct selection should use `--validate-only`; it validates the full catalogue and deliberately emits no native jobs.

Issue [#571](https://github.com/vsimko/gfabasic-rust/issues/571) implements validation and selection. The validator rejects duplicate IDs, unsupported schema versions, missing/extra version keys, invalid enums, dangling evidence references, path traversal, missing/non-`.LST` probes, and inconsistent expectation/execution states. It does not modify or migrate the existing corpus YAML or API registry TOML.

The new construct catalogue is metadata for selecting and interpreting tests; it does not add parser, IR, runtime, or API-registry behavior.
