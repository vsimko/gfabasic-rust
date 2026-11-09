# Architecture overview

This directory contains the accepted architecture, implementation policies, and decision records for gfabasic-rust. Read [COMPATIBILITY-EVIDENCE.md](COMPATIBILITY-EVIDENCE.md) before changing language support, compatibility tests, or reference-interpreter research.

## Architecture documents

| Topic | Scope | Document | Tracking |
|---|---|---|---|
| Compatibility evidence and agent workflow | 3.52 reference behavior, evidence priority, debugging, feature records, test provenance, and issue-driven agent workflow | [COMPATIBILITY-EVIDENCE.md](COMPATIBILITY-EVIDENCE.md) | [#46](https://github.com/vsimko/gfabasic-rust/issues/46), [#66](https://github.com/vsimko/gfabasic-rust/issues/66) |
| Project goals and compatibility | Purpose, non-goals, compatibility dimensions | [GOALS.md](GOALS.md) | [#2](https://github.com/vsimko/gfabasic-rust/issues/2) |
| Dialects and versions | Source dialect identity and version-specific rules | [DIALECTS.md](DIALECTS.md); [Amiga 3.5 lexer evidence inventory](../research/amiga-gfa-basic-35-lexer-evidence.md); [ADR 0009](adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md) | [#3](https://github.com/vsimko/gfabasic-rust/issues/3) |
| Implementation dashboard | Static maintainer dashboard and canonical status sources | [IMPLEMENTATION-DASHBOARD.md](IMPLEMENTATION-DASHBOARD.md); [ADR 0014](adr/0014-implementation-dashboard.md) | [#304](https://github.com/vsimko/gfabasic-rust/issues/304)–[#310](https://github.com/vsimko/gfabasic-rust/issues/310) |
| Repository consistency validation | Deterministic checks for drift between repository sources | [VALIDATION.md](VALIDATION.md); [ADR 0013](adr/0013-repository-consistency-validation.md) | [#248](https://github.com/vsimko/gfabasic-rust/issues/248), [#249](https://github.com/vsimko/gfabasic-rust/issues/249) |
| Product scope | Output target, platform integration, and product boundaries | [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md) | [#3](https://github.com/vsimko/gfabasic-rust/issues/3), [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| Target model | Host/target distinction and target capabilities | [TARGETS.md](TARGETS.md); [ADR 0003](adr/0003-target-model.md) | [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| API profiles | Platform-specific declarations and behavior | [API-PROFILES.md](API-PROFILES.md); [ADR 0004](adr/0004-api-profile-model.md) | [#5](https://github.com/vsimko/gfabasic-rust/issues/5) |
| Compiler pipeline and IR | Frontend stages, semantic analysis, typed IR, LLVM boundary | [PIPELINE-AND-IR.md](PIPELINE-AND-IR.md); [ADR 0007](adr/0007-compiler-pipeline-and-ir.md) | [#6](https://github.com/vsimko/gfabasic-rust/issues/6) |
| Diagnostics | Stable diagnostic codes, source locations, and support outcomes | [DIAGNOSTICS.md](DIAGNOSTICS.md) | [#26](https://github.com/vsimko/gfabasic-rust/issues/26) |
| Compatibility testing | Test taxonomy, fixtures, conformance, deterministic behavior, release criteria | [TESTING.md](TESTING.md); [corpus conventions](../../tests/corpus/README.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#41](https://github.com/vsimko/gfabasic-rust/issues/41) |
| Compatibility matrix | Current dialect/version/profile/target support status | [COMPATIBILITY-MATRIX.md](COMPATIBILITY-MATRIX.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |
| Frontend fuzzing | Fuzz targets, corpus separation, resource budgets, and CI policy | [FUZZING.md](FUZZING.md) | [#43](https://github.com/vsimko/gfabasic-rust/issues/43), [#118](https://github.com/vsimko/gfabasic-rust/issues/118) |
| Rust workspace | Crate/module boundaries and dependency policy | [WORKSPACE.md](WORKSPACE.md) | [#11](https://github.com/vsimko/gfabasic-rust/issues/11) |
| API registry | Canonical TOML declarations, evidence/support states, schema validation | [API-REGISTRY.md](API-REGISTRY.md); [ADR 0006](adr/0006-api-registry.md) | [#12](https://github.com/vsimko/gfabasic-rust/issues/12), [#115](https://github.com/vsimko/gfabasic-rust/issues/115) |
| Versioning and compatibility lifecycle | Compiler releases, schema versions, runtime ABI, deprecation | [VERSIONING.md](VERSIONING.md); [ADR 0010](adr/0010-versioning-and-compatibility-lifecycle.md) | [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |

## Intended compiler pipeline

The proposed pipeline is:

1. Load source files and preserve source identity.
2. Tokenize source using the selected dialect and version.
3. Parse tokens into an AST with source spans.
4. Resolve names and declarations.
5. Perform type checking and semantic analysis.
6. Lower valid constructs into a typed, target-independent project IR.
7. Perform target- or API-specific lowering where required.
8. Generate and verify LLVM IR.
9. Generate native object code and link the selected runtime.
10. Report diagnostics and build artifacts.

The frontend and project-owned IR must remain independent of SDL3 and the output target. Semantic analysis belongs to the compiler, not LLVM. This pipeline is an architectural direction; exact interfaces remain open.

## Dependency boundaries

- Language parsing must not depend on platform API implementation details.
- The project-owned IR must not expose SDL3 or Linux-specific types.
- The compiler must depend on stable project-owned runtime interfaces, not call SDL3 directly from generated code.
- Frontend and IR work should be testable without LLVM or SDL3 installed.
- Platform API profiles must not silently alter language-core semantics.
- No language/API feature may be claimed as supported without version-scoped evidence and an implementation/test status.

## Decision records

Use numbered Architecture Decision Records in [adr/](adr/). Each record states context, decision, status, alternatives, consequences, and evidence. An accepted ADR can be superseded by a later ADR; do not silently rewrite historical rationale. The [ADR template](adr/TEMPLATE.md) defines the required structure.

## Glossary

- **Dialect**: a defined GFA BASIC language variant associated with a platform and/or release.
- **Version**: a specific release of a dialect.
- **Language core**: language syntax and semantics not tied to a platform API.
- **API profile**: a set of platform-specific declarations and behavior.
- **Host**: the machine and environment on which the compiler runs.
- **Target**: the machine and ABI for which the compiler generates output.
- **Runtime**: project-owned support code used by generated programs.
- **Backend**: the component that translates project IR into target code.
- **Compatibility**: a measured degree of agreement with a specified reference version, not a blanket claim.
- **Conformance test**: a test that compares compiler or runtime behavior with a documented rule or trusted reference implementation.

## Open questions

- Which 3.52 behaviors are also present in an independently identified 3.5 interpreter?
- Which manuals describe the exact behavior observed in the 3.52 reference interpreter, and where do they disagree?
- Which LLVM version and Rust binding are suitable for the chosen Rust toolchain?
- Which runtime operations require software emulation rather than direct platform adapters?

Record evidence and decisions in the relevant issue and canonical document before claiming closure.
