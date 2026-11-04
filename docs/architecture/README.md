# Architecture

This directory contains the normative architecture baseline for `gfabasic-rust`.

## Status vocabulary

- **Accepted**: a project decision. Implementation must follow it unless a later decision supersedes it.
- **Proposed**: a candidate direction that still requires review or evidence.
- **Open**: unresolved. Do not implement behavior by guessing.
- **Deferred**: intentionally outside the current implementation scope.
- **Verified**: supported by an identified primary reference or reproducible compatibility evidence.
- **Inferred**: a reasoned interpretation that is not directly established by a source.
- **Disputed**: reliable sources or observed implementations disagree.
- **Unknown**: evidence is not currently available.

A planned technology or target is not an implementation claim. A language feature is not supported merely because it appears in a source inventory.

## Project baseline

The project intends to provide a reusable GFA BASIC compiler and runtime in Rust. Native executable generation is the primary planned output. LLVM is the planned code-generation infrastructure. The first planned output target is Linux x86-64 with the GNU ABI and ELF executable format. SDL3 is a planned adapter for Linux graphics, window, and input integration.

These are planning assumptions, not evidence that these components are implemented or that a particular GFA BASIC dialect is fully supported.

The language core is separate from platform API profiles. Amiga and Atari ST profiles are planned as independent choices. The selected source dialect/version, API profile set, build host, and output target must be represented as separate configuration concepts.

## Architecture documents

| Topic | Scope | Document | Tracking |
|---|---|---|---|
| Project goals and compatibility | Purpose, non-goals, compatibility dimensions | [GOALS.md](GOALS.md) | [#2](https://github.com/vsimko/gfabasic-rust/issues/2) |
| Dialects and versions | Source dialect identity and version-specific rules | [DIALECTS.md](DIALECTS.md); [Atari ST GFA BASIC V2.xx/V3.0 research draft](../reference/atari-gfa-basic-3.0.md) | [#3](https://github.com/vsimko/gfabasic-rust/issues/3) |
| Target model | Host/target distinction and target capabilities | [TARGETS.md](TARGETS.md); [ADR 0003](adr/0003-target-model.md) | [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| API profiles | Core, Amiga, and Atari ST profile boundaries | [API-PROFILES.md](API-PROFILES.md); [Amiga API research draft](../reference/amiga-gfa-api.md); [Atari ST API research draft](../reference/atari-gfa-api.md) | [#5](https://github.com/vsimko/gfabasic-rust/issues/5), [#24](https://github.com/vsimko/gfabasic-rust/issues/24), [#25](https://github.com/vsimko/gfabasic-rust/issues/25) |
| Compiler pipeline and IR | Frontend stages, semantic analysis, typed IR, LLVM boundary | [PIPELINE-AND-IR.md](PIPELINE-AND-IR.md) | [#6](https://github.com/vsimko/gfabasic-rust/issues/6) |
| Diagnostics and compatibility reporting | Stable diagnostic codes, source locations, support outcomes, migration guidance | [DIAGNOSTICS.md](DIAGNOSTICS.md) | [#26](https://github.com/vsimko/gfabasic-rust/issues/26) |
| Runtime | Runtime layers and SDL3 adapter boundary | [RUNTIME.md](RUNTIME.md) | [#7](https://github.com/vsimko/gfabasic-rust/issues/7) |
| Memory model | Storage, pointers, hardware access, safety policy | [MEMORY.md](MEMORY.md) | [#8](https://github.com/vsimko/gfabasic-rust/issues/8) |
| CLI and configuration | Inputs, profiles, outputs, reproducibility | [CLI-AND-CONFIG.md](CLI-AND-CONFIG.md) | [#9](https://github.com/vsimko/gfabasic-rust/issues/9) |
| Compatibility testing | Test taxonomy, fixtures, conformance, deterministic behavior, release criteria | [TESTING.md](TESTING.md); [corpus conventions](../../tests/corpus/README.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#41](https://github.com/vsimko/gfabasic-rust/issues/41), [#42](https://github.com/vsimko/gfabasic-rust/issues/42), [#43](https://github.com/vsimko/gfabasic-rust/issues/43) |
| Compatibility matrix | Current dialect/version/profile/target support status | [COMPATIBILITY-MATRIX.md](COMPATIBILITY-MATRIX.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |
| Rust workspace | Crate/module boundaries and dependency policy | [WORKSPACE.md](WORKSPACE.md) | [#11](https://github.com/vsimko/gfabasic-rust/issues/11) |
| API registry | Canonical machine-readable declarations | [API-REGISTRY.md](API-REGISTRY.md) | [#12](https://github.com/vsimko/gfabasic-rust/issues/12) |
| Versioning and compatibility lifecycle | Compiler releases, schema versions, runtime ABI, deprecation | [VERSIONING.md](VERSIONING.md) | [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |
| Performance benchmarks | Compiler, generated-program, runtime, and build measurements | [BENCHMARKING.md](BENCHMARKING.md) | [#57](https://github.com/vsimko/gfabasic-rust/issues/57) |
| Release and artifacts | Tag triggers, validation gates, artifact metadata, checksums, provenance, reproducibility | [RELEASES.md](RELEASES.md) | [#40](https://github.com/vsimko/gfabasic-rust/issues/40) |

Detailed decisions remain open until the corresponding research and review are complete. The existence of a document does not mean that its issue is resolved.

## Intended compiler pipeline

The proposed pipeline is:

1. Load source files and preserve source identity.
2. Tokenize source using the selected dialect and version.
3. Parse tokens into an AST with source spans.
4. Resolve names and declarations.
5. Perform type checking and semantic analysis.
6. Lower valid constructs into a typed, target-independent project IR.
7. Perform target- or API-specific lowering where required.
8. Generate LLVM IR and verify it.
9. Generate native object code and link the selected runtime.
10. Report diagnostics and build artifacts.

The frontend and project-owned IR must remain independent of SDL3 and of the initial output target. Semantic analysis belongs to the compiler, not LLVM. This pipeline is an architectural direction; interfaces and exact representations remain open.

## Dependency boundaries

- Language parsing must not depend on platform API implementation details.
- The project-owned IR must not expose SDL3 or Linux-specific types.
- The compiler must depend on stable project-owned runtime interfaces, not call SDL3 directly from generated code.
- Frontend and IR work should be testable without LLVM or SDL3 installed.
- Platform API profiles must not silently alter language-core semantics.
- No language/API feature may be claimed as supported without a dialect/version-specific reference and an implementation/test status.

## Decision records

Use numbered Architecture Decision Records in [adr/](adr/). Each record states context, decision, status, alternatives, consequences, and evidence. An accepted ADR can be superseded by a later ADR; do not silently rewrite the historical rationale. The [ADR template](adr/TEMPLATE.md) defines the required structure.

## Glossary

- **Dialect**: a defined GFA BASIC language variant associated with a platform and/or release.
- **Version**: a specific release of a dialect.
- **Language core**: language syntax and semantics not tied to a platform API.
- **API profile**: a selectable set of platform-specific declarations and behavior.
- **Host**: the machine and environment on which the compiler runs.
- **Target**: the machine and ABI for which the compiler generates output.
- **Runtime**: project-owned support code used by generated programs.
- **Backend**: the component that translates project IR into target code.
- **Compatibility**: a measured degree of agreement with a specified source dialect/version and API profile, not a blanket claim.
- **Conformance test**: a test that compares compiler or runtime behavior with a documented rule or trusted reference implementation.

## Open questions

- Which exact dialect/version is the first normative source-language target?
- Which Amiga and Atari ST manual editions are authoritative for each profile?
- What is the exact documentation format requirement referred to as “OKF” in issue #14?
- Which LLVM version and Rust binding are suitable for the chosen Rust toolchain?
- Which compiler components justify separate Cargo crates?
- Which runtime operations require software emulation rather than direct platform adapters?

Record evidence and decisions in the relevant issue and ADR before claiming closure.
