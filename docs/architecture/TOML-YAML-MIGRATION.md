---
type: "Architecture"
title: "TOML inventory and YAML migration policy"
---

# TOML inventory and YAML migration policy

## Scope and snapshot

This inventory records every tracked `.toml` file on `main` at the start of issue [#574](https://github.com/vsimko/gfabasic-rust/issues/574). It is a path-level inventory, not a claim that every TOML file is a project manifest.

The snapshot contains **41 files**. The decision is based on each file's role, not its extension alone.

## Inventory and decisions

| Area | Count | Files / pattern | Decision |
|---|---:|---|---|
| Rust package manifests | 7 | `Cargo.toml`, `bench/Cargo.toml`, `crates/*/Cargo.toml`, `fuzz/Cargo.toml` | Keep TOML: Cargo requires this format. |
| Rust toolchain | 1 | `rust-toolchain.toml` | Keep TOML: rustup's supported toolchain file format. |
| Link checker configuration | 1 | `.lychee.toml` | Keep for now: tool-native configuration; migrate only if the tool supports YAML with equivalent semantics. |
| API registry | 6 | `api/registry/{amiga,atari-st,language-core}/*.toml` | Keep TOML in this migration: established registry schema and consumers are TOML-based; changing it is a separate compatibility migration, not a filename conversion. |
| Research inventories | 8 | `docs/research/*.toml` | Keep for now: structured evidence datasets, not fixture manifests. Inventory their readers before considering a format change. |
| Corpus manifest and fixture sidecars | 17 | `tests/corpus/manifest.toml` and all `tests/corpus/**/*.toml` | **Migrate to YAML** together with the validator, summary generation, docs, skills, and CI. |
| Interpreter probe sidecar | 1 | `tests/interpreter-probes/amiga-3.5/fact-zero.toml` | **Migrate to YAML** with its probe tooling and documentation. |

### Exact research inventory

- `docs/research/amiga-gfa-basic-3.0-event-window-statements.toml`
- `docs/research/amiga-gfa-basic-3.0-file-data-statements.toml`
- `docs/research/amiga-gfa-basic-3.0-operators.toml`
- `docs/research/amiga-gfa-basic-3.0-predefined-names.toml`
- `docs/research/amiga-gfa-basic-3.0-statements.toml`
- `docs/research/amiga-gfa-basic-candidates.toml`
- `docs/research/amiga-gfa-basic-functions.toml`
- `docs/research/amiga-gfa-basic-language-population-policy.toml`

### Corpus migration inventory

The corpus group consists of `tests/corpus/manifest.toml` and these 16 sidecars:

- `tests/corpus/amiga/3.0/bit-operations/bset-btst-basic.toml`
- `tests/corpus/amiga/3.0/control-flow/nested-loop-close-order.toml`
- `tests/corpus/amiga/3.0/control-flow/repeat-until-postcondition.toml`
- `tests/corpus/amiga/3.5/combinatorics/fact-zero.toml`
- `tests/corpus/amiga/3.5/combinatorics/three-intrinsics.toml`
- `tests/corpus/amiga/3.5/data/mat-mul-triple.toml`
- `tests/corpus/amiga/3.5/data/mat-read-data.toml`
- `tests/corpus/amiga/3.5/matrix/mat-add.toml`
- `tests/corpus/amiga/3.5/matrix/mat-base-transpose.toml`
- `tests/corpus/amiga/3.5/matrix/mat-clear-zero.toml`
- `tests/corpus/amiga/3.5/matrix/mat-mul-dot.toml`
- `tests/corpus/amiga/3.5/matrix/mat-norm-row.toml`
- `tests/corpus/amiga/3.5/matrix/mat-one-identity.toml`
- `tests/corpus/amiga/3.5/matrix/mat-set-scalar.toml`
- `tests/corpus/amiga/3.5/matrix/mat-sub.toml`
- `tests/corpus/amiga/3.5/research/mat-cpy-submatrix.toml`

The separate interpreter probe sidecar is not part of the corpus manifest and needs its own consumer audit.

## Migration contract

For the corpus and interpreter probe sidecars:

1. YAML becomes the sole accepted on-disk format after migration; do not retain silent TOML fallback parsing.
2. Preserve field names and values unless a separately documented schema change is required.
3. Define and validate required fields, scalar types, enumerations, uniqueness, path correspondence, and source-register references.
4. Reject duplicate YAML mapping keys and unsafe paths/symlinks. Keep validation deterministic and offline.
5. Test semantic equivalence between the pre-migration TOML data and migrated YAML data during the transition; remove temporary conversion fixtures once equivalence is established.
6. Update all readers, generators, tests, documentation, agent skills, and workflow path filters in the same change.
7. Keep metadata validation separate from BASIC parsing or interpreter conformance claims.

## Sequencing

- This document completes the inventory and format decision for issue #574.
- The corpus manifest, 16 corpus sidecars, and the Amiga 3.5 interpreter-probe sidecar have been migrated to YAML in this PR. Remaining work in #574 is to verify downstream consumers, CI, and documentation references before closing the issue.
- Construct-scoped YAML manifests and change selection are tracked separately in [#570](https://github.com/vsimko/gfabasic-rust/issues/570) and [#571](https://github.com/vsimko/gfabasic-rust/issues/571); parallel construct/version CI is [#572](https://github.com/vsimko/gfabasic-rust/issues/572).
- Do not migrate tool-owned formats (Cargo, rustup) or convert research/API data merely to make all extensions uniform.
