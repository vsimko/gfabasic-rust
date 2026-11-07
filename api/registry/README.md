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
- Book-documented and primary-reference-cross-checked claims require printed-page locators.
- Unknown/unsupported target support and non-passing conformance require explicit pending reasons.
- Disputed evidence must describe the conflict and cannot claim implemented, supported, or passing behavior.
- Unknown/inferred evidence cannot claim supported target behavior.
- Do not create placeholder entries or infer implementation support from names, prose, issue titles, or the existence of a research fixture.

## AI-assisted evidence review

Generate a bounded review package for a declaration with `scripts/build_api_review_package.py`. Store completed advisory results under `api/reviews/`; CI validates their schema, source references, and input hashes with `scripts/validate_api_reviews.py`. See [AI-assisted API verification](../../docs/architecture/AI-VERIFICATION.md). Agent verdicts never promote implementation or conformance status.


The public site catalogue is generated from this registry by `scripts/generate_api_catalogue.py` into `site/data/api-catalogue.json`. The JSON is a deployment artifact, never an editable source. Production and PR-preview workflows regenerate it; generator tests cover deterministic ordering, independent status counts, empty registries, and stale-output detection.


## Version-scoped base-manual research

The supplied 3.5 interpreter manual contains both a 3.0 base manual and a 3.5 addendum. Do not treat Chapters 2–12 as 3.5-specific evidence merely because they occur in the same PDF. The initial 3.0 topic and page inventory is maintained separately in [the Amiga 3.0 base-manual inventory](../../docs/research/amiga-gfa-basic-3.0-base-inventory.md) and tracked by #298. Only add declarations to the 3.5 registry after their 3.5 applicability is independently established.


- Every Amiga 3.5 declaration must set evidence source-scope to `amiga-3.5-addendum` or `runtime-observation`. Base-manual 3.0 evidence alone cannot establish 3.5 availability. This is checked by the registry validator.
