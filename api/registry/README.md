# API registry

Canonical declarations use TOML. The planned registry namespaces are `language-core/`, `amiga/`, and `atari-st/`; this describes the intended layout, not the current set of populated directories.

## Current inventory

- `atari-st/time.toml`: one research-status Atari ST `TIMER` declaration for version 3.0.
- `amiga/math-and-matrix.toml`: an initial Amiga GFA BASIC 3.5 slice containing `FACT`, `VARIAT`, `COMBIN`, `MAT BASE`, `MAT CLR`, `MAT SET`, `MAT ONE`, `MAT TRANS`, and `MAT NORM`.
- `language-core/`: no declarations yet.

The Amiga slice is deliberately limited to features explicitly identified in the supplied 3.5 interpreter addendum. It does not claim completeness. The addendum's Chapter 13/14 page references and known source limitations are documented in [the extracted feature inventory](../../docs/research/amiga-gfa-basic-35-addendum-inventory.md).

## Evidence and implementation boundaries

- The normative Amiga source is `AM-INTERPRETER-35`. Base-manual 3.0 features and the 1990 German book are not silently promoted to 3.5 declarations.
- `book-documented` means the source documents the feature; per-declaration notes identify page-image checks and unresolved limits.
- Evidence, implementation, target support, and conformance are independent. The Amiga declarations currently have unknown Linux support and untested conformance.
- The experimental compiler prototype supports only a restricted literal subset of the three combinatorics intrinsics. That subset is recorded as an implementation note, not misrepresented as a complete runtime implementation or a registry-linked handler.
- Existing 3.5 matrix fixtures are research fixtures in a `research_only` corpus. They are not executable conformance tests and are therefore not linked as passing tests.
- The `MAT INV` operand direction remains disputed between the command summary and detailed example; it is intentionally not declared as settled syntax.

## Editing rules

- Registry `support.tests` references use each manifest-listed fixture sidecar's stable `id`; manifest path stems are storage locations, not test IDs.
- Do not mark a declaration supported without primary evidence, a real implementation, and passing conformance tests.
- Unknown support classifications require an explicit pending reason.
- Do not create placeholder entries or infer implementation support from names, prose, issue titles, or the existence of a research fixture.
