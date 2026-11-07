# GFA BASIC conformance corpus

## Purpose

This directory contains small, independently authored GFA BASIC programs and metadata for future compiler/runtime conformance tests. It is a source corpus, not a general executable conformance suite yet.

All fixtures must be independently authored. Do not copy historical program listings or require historical manuals, ROMs, disk images, or other third-party artifacts to be present in the repository or a release. Cite external evidence by bibliographic source and page/section; do not embed the source material itself.

The repository contains an experimental, evidence-gated Amiga GFA BASIC 3.5 parser/formatter for the documented `FACT`, `VARIAT`, `COMBIN`, and `PRINT` subset plus selected matrix statements. The executable compiler remains limited to the combinatorics subset and uses a temporary C/`cc` native bootstrap; there is no general compiler or corpus test runner. Six independently authored Amiga 3.5 addendum fixtures are marked `verified` on page-image evidence (`MAT ONE`, `MAT CLR`, `MAT SET`, `MAT BASE`/`MAT TRANS`, `MAT NORM`, and `FACT(0)`). This verifies their documented syntax and expected behavior against the cited page images; it is not original-interpreter execution or proof of full dialect conformance. The Amiga 3.0 fixtures remain `candidate` and must not be promoted without applicable evidence. The metadata validator does not compile or execute fixtures.

The verified combinatorics and matrix fixtures accepted by the formatter are canonicalized and checked by Rust CI with `gfabasic fmt --check`. This is a formatter-style gate for the implemented subset only, not a corpus runner or conformance claim for the remaining fixtures.

## Layout

- `manifest.toml`: corpus version and fixture index.
- `<dialect>/<version>/<area>/<case>.bas`: standalone BASIC source.
- `<dialect>/<version>/<area>/<case>.toml`: metadata and expected behavior.

Use lowercase, stable fixture IDs. Each source file must be original and self-contained. Do not require external assets, host paths, locale settings, or network access.

## Metadata contract

Each sidecar TOML file must define:

- `id`: stable unique identifier.
- `dialect` and `version`: exact source dialect and release.
- `api_profiles`: required platform API profiles; use an empty array for language-core tests.
- `kind`: `positive`, `negative`, or `semantic`.
- `status`: `candidate`, `verified`, `blocked`, or `retired`.
- `source_key`, `source_pages`, and `evidence_status`: source identity, printed page/section, and evidence quality.
- `expectation`: expected parse/compile/runtime outcome. State `expected_stdout`, `expected_stderr`, and `expected_exit` when applicable.

The manifest stores fixture paths relative to the corpus root, without the `.bas` or `.toml` suffix. The corresponding source and metadata files use the same path stem. Fixture files and every directory component below the corpus root must be regular, non-symlink paths; symlinks are rejected to prevent external targets and alternate fixture identities.
- `notes`: unresolved syntax, OCR, edition, or runtime questions.

Validate the manifest, companion paths, stable IDs, evidence-source keys, and required metadata without compiling or executing any fixture:

```sh
python scripts/validate_corpus.py
```

This validation only checks corpus metadata. It does not make a candidate fixture a passing conformance test.

A fixture may use `verified` only after its syntax and expected behavior are checked against the cited page image or a named trusted interpreter. OCR text alone is not sufficient for OCR-sensitive punctuation or signatures. A candidate fixture must not be used as a compiler acceptance rule.

## Evidence policy

- Keep Amiga and Atari ST fixtures in separate trees.
- Do not promote a construct from one platform/version into another dialect.
- Every expected result must cite an applicable source or a reproducible observation.
- Distinguish documented interpreter behavior from editor behavior and compiler-specific behavior.
- Do not mark a construct invalid merely because it is absent from the current inventory. Negative tests require an explicit source rule or a documented compatibility observation.
- Preserve the source edition and printed page numbers. Do not cite PDF viewer page numbers as printed pages.

## Adding fixtures

1. Check the relevant issue and architecture decisions.
2. Add one focused `.bas` file and its TOML sidecar.
3. Add the fixture ID to `manifest.toml`.
4. Record the source claim and expected outcome.
5. Keep the status at `candidate` until evidence review is complete.
6. When a runner exists, record the runner version and actual result separately from the expected result.

Tracking: [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42).
