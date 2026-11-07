# API registry

Canonical declarations use TOML. The planned registry namespaces are `language-core/`, `amiga/`, and `atari-st/`; this describes the intended layout, not the current set of populated directories.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `language-core/math-and-matrix.toml`: `FACT`, `VARIAT`, `COMBIN`, and the initial matrix base/generation operations.
- `language-core/matrix-operations.toml`: matrix/vector DATA and file I/O, copy/transpose, arithmetic, determinant, rank, and inverse operations.
- `language-core/special-variables-and-data-pointers.toml`: `_DATA`, `_DATA=`, and `_0`…`_9`.
- `amiga/special-and-platform-commands.toml`: `PCOLOR`, the negative-screen-number `OPENW` variant, and the `GURU` command/function forms.
- `language-core/`: no declarations yet.

The language-core catalogue currently contains 28 declarations, and the Amiga platform profile contains three declarations. All are limited to features explicitly identified in the supplied 3.5 interpreter addendum. Neither catalogue claims completeness. The addendum's Chapter 13/14 page references and known source limitations are documented in [the extracted feature inventory](../../docs/research/amiga-gfa-basic-35-addendum-inventory.md).

## Evidence and implementation boundaries

- The normative Amiga source is `AM-INTERPRETER-35`. Base-manual 3.0 features and the 1990 German book are not silently promoted to 3.5 declarations.
- `book-documented` means the source documents the feature; per-declaration notes identify page-image checks and unresolved limits.
- Evidence, implementation, target support, and conformance are independent. The Amiga declarations currently have unknown Linux support and untested conformance.
- The experimental compiler prototype supports only a restricted literal subset of the three combinatorics intrinsics. That subset is recorded as an implementation note, not misrepresented as a complete runtime implementation or a registry-linked handler.
- Existing 3.5 matrix fixtures are research fixtures in a `research_only` corpus. They are not executable conformance tests and are therefore not linked as passing tests.
- The `MAT INV` operand direction is represented as a disputed declaration with both source variants; it is not a settled syntax or implementation contract.

## Editing rules

- Registry `support.tests` references use each manifest-listed fixture sidecar's stable `id`; manifest path stems are storage locations, not test IDs.
- Do not mark a declaration supported without primary evidence, a real implementation, and passing conformance tests.
- Unknown support classifications require an explicit pending reason.
- Do not create placeholder entries or infer implementation support from names, prose, issue titles, or the existence of a research fixture.

## AI-assisted evidence review

Generate a bounded review package for a declaration with `scripts/build_api_review_package.py`. Store completed advisory results under `api/reviews/`; CI validates their schema, source references, and input hashes with `scripts/validate_api_reviews.py`. See [AI-assisted API verification](../../docs/architecture/AI-VERIFICATION.md). Agent verdicts never promote implementation or conformance status.
