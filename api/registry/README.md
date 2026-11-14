# API registry

Canonical declarations use TOML in the `language-core/`, `amiga/`, and `atari-st/` namespaces.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `language-core/math-and-matrix.toml`: `FACT`, `VARIAT`, `COMBIN`, and the initial matrix base/generation operations.
- `language-core/matrix-operations.toml`: matrix/vector DATA and file I/O, copy/transpose, arithmetic, determinant, rank, and inverse operations.
- `language-core/special-variables-and-data-pointers.toml`: `_DATA`, `_DATA=`, and `_0`…`_9`.
- `language-core/inherited-functions-3-5-evidence.toml`: eight inherited function call forms directly used or explicitly referenced in 3.5-labelled material (`RAND`, `STR# API registry

Canonical declarations use TOML in the `language-core/`, `amiga/`, and `atari-st/` namespaces.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `language-core/math-and-matrix.toml`: `FACT`, `VARIAT`, `COMBIN`, and the initial matrix base/generation operations.
- `language-core/matrix-operations.toml`: matrix/vector DATA and file I/O, copy/transpose, arithmetic, determinant, rank, and inverse operations.
, `SUCC`, `STRING# API registry

Canonical declarations use TOML in the `language-core/`, `amiga/`, and `atari-st/` namespaces.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `language-core/math-and-matrix.toml`: `FACT`, `VARIAT`, `COMBIN`, and the initial matrix base/generation operations.
- `language-core/matrix-operations.toml`: matrix/vector DATA and file I/O, copy/transpose, arithmetic, determinant, rank, and inverse operations.
, `HEX# API registry

Canonical declarations use TOML in the `language-core/`, `amiga/`, and `atari-st/` namespaces.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `language-core/math-and-matrix.toml`: `FACT`, `VARIAT`, `COMBIN`, and the initial matrix base/generation operations.
- `language-core/matrix-operations.toml`: matrix/vector DATA and file I/O, copy/transpose, arithmetic, determinant, rank, and inverse operations.
, `RND`, `INT`, `SQR`).
- `amiga/special-and-platform-commands.toml`: `PCOLOR`, the negative-screen-number `OPENW` variant, and the `GURU` command/function forms.

The language-core catalogue currently contains 32 declarations, and the Amiga platform profile contains four declarations. All are limited to features explicitly identified in the supplied 3.5 interpreter addendum. Neither catalogue claims completeness. The addendum's Chapter 13/14 page references and known source limitations are documented in [the extracted feature inventory](../../docs/research/amiga-gfa-basic-35-addendum-inventory.md).

## Evidence and implementation boundaries

- The normative Amiga source is `AM-INTERPRETER-35`. Base-manual 3.0 features and the 1990 German book are not silently promoted to 3.5 declarations.
- `book-documented` means the source documents the feature; per-declaration notes identify page-image checks and unresolved limits.
- Evidence, implementation, target support, and conformance are independent. The three combinatorics declarations are marked partial and have stable runtime handler IDs; all other declarations remain not implemented. Every declaration still has unknown Linux support and untested conformance.
- The runtime has partial typed handlers for FACT, VARIAT, and COMBIN, and the compiler's narrow literal IR lowering resolves them through the canonical registry. Their implementation status remains partial because the compiler subset and original-interpreter overflow/error behavior are not fully established.
- Existing 3.5 matrix fixtures are research fixtures in a `research_only` corpus. They are not executable conformance tests and are therefore not linked as passing tests.
- `MAT INV` uses the destination-first form documented by the detailed 3.5 definition/example and corroborated by the German companion book; the reversed command-summary notation is retained as a source discrepancy note. Runtime implementation and conformance remain untested.

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


## Static website payload format

The website generator writes two kinds of deployment artifacts; neither is edited manually:

- `site/data/api-catalogue.json` is schema version 2, a compact index containing category keys, declaration names, syntax/search summaries, independent status fields, counts, and each detail file's relative path.
- `site/data/api-catalogue-details/<declaration-id>.json` contains the full registry record for one declaration.

The browser downloads the index once. It creates category and declaration disclosure elements from index fields only; a declaration's detail JSON is fetched from the same static site origin only when its disclosure is opened. No runtime API, backend, database, or server-side catalogue logic is used.

Run `python3 scripts/generate_api_catalogue.py` to regenerate the payloads and `python3 scripts/generate_api_catalogue.py --check` to verify byte-for-byte reproducibility and detect missing or stale detail files. CI runs both commands and tests the generator with empty, partial, populated, and 1,000-entry synthetic registries.

## Dashboard implementation progress scope

The maintainer dashboard exposes an interim **Implemented 3.5 addendum API** metric. Its denominator is generated from declarations whose availability is the Amiga GFA BASIC interpreter 3.5 and whose evidence source-scope is exactly `amiga-3.5-addendum`. It currently covers 32 language-core declarations and four Amiga platform declarations (36 total). The numerator counts only declarations with implementation status exactly `implemented`; prototype, partial, planned, research, and blocked statuses do not count.

This is deliberately not labelled as the total Amiga GFA BASIC 3.5 API. It excludes inherited 3.0 base-manual declarations that have not been individually reconciled for 3.5, editor-only changes, compiler-only features, and Atari ST declarations. The complete language-wide denominator remains blocked on declaration-level 3.5 reconciliation or controlled interpreter observations tracked by issue #307. Evidence, implementation, Linux target support, and conformance remain separate dimensions.


## Runtime handler integration

`scripts/generate_runtime_handler_requirements.py` generates the Rust declaration
requirements from all TOML registry files. The runtime crate validates this
generated table against its explicit typed handler registrations. CI checks
that the generated file is current and Rust tests validate the complete
registry, rejecting missing, duplicate, stale, profile-mismatched, and
signature-mismatched registrations. The compiler's current combinatorics
lowering resolves the same stable IDs through the runtime API resolver.
