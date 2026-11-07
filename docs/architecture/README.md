## Architecture documents

| Topic | Scope | Document | Tracking |
|---|---|---|---|
| Repository consistency validation | Deterministic Python/JSON checks for drift between repository sources | [VALIDATION.md](VALIDATION.md); [ADR 0013](adr/0013-repository-consistency-validation.md) | architecture [#248](https://github.com/vsimko/gfabasic-rust/issues/248), validator [#249](https://github.com/vsimko/gfabasic-rust/issues/249), website generator [#250](https://github.com/vsimko/gfabasic-rust/issues/250) |
| Website previews | Per-PR preview URLs and automatic cleanup after PR closure | [WEBSITE_PREVIEWS.md](WEBSITE_PREVIEWS.md) |
| Topic | Scope | Document | Tracking |
|---|---|---|---|
| Project goals and compatibility | Purpose, non-goals, compatibility dimensions | [GOALS.md](GOALS.md) | [#2](https://github.com/vsimko/gfabasic-rust/issues/2) |
| Dialects and versions | Source dialect identity and version-specific rules | [DIALECTS.md](DIALECTS.md); [Amiga 3.5 lexer evidence inventory](../research/amiga-gfa-basic-35-lexer-evidence.md); [ADR 0009](adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md); [superseded ADR 0008](adr/0008-initial-source-dialect-amiga-gfa-basic-3.0.md); [Amiga GFA BASIC 3.0 historical research](../reference/amiga-gfa-basic-3.0.md); [Atari ST GFA BASIC V2.xx/V3.0 research draft](../reference/atari-gfa-basic-3.0.md) | [#3](https://github.com/vsimko/gfabasic-rust/issues/3) |
| Product scope | Sole dialect, API profile, Linux x86-64 target, and SDL3 boundary | [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md) | [#3](https://github.com/vsimko/gfabasic-rust/issues/3), [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| Target model | Host/target distinction and target capabilities | [TARGETS.md](TARGETS.md); [ADR 0003](adr/0003-target-model.md) | [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| API profiles | Amiga GFA BASIC 3.5 API compatibility only; Atari ST research is archival and unsupported | [API-PROFILES.md](API-PROFILES.md); [ADR 0004](adr/0004-api-profile-model.md); [Amiga API research draft](../reference/amiga-gfa-api.md); [Atari ST API research draft](../reference/atari-gfa-api.md) | [#5](https://github.com/vsimko/gfabasic-rust/issues/5), [#24](https://github.com/vsimko/gfabasic-rust/issues/24), [#25](https://github.com/vsimko/gfabasic-rust/issues/25) |
| Language tooling and editor integration | [LANGUAGE-TOOLING.md](LANGUAGE-TOOLING.md); [ADR 0012](adr/0012-language-server-and-editor-integration.md) | Shared compiler frontend, LSP server, editor client boundaries, and refactoring safety | [#209](https://github.com/vsimko/gfabasic-rust/issues/209), [#210](https://github.com/vsimko/gfabasic-rust/issues/210), [#211](https://github.com/vsimko/gfabasic-rust/issues/211) |
| Compiler pipeline and IR | Frontend stages, semantic analysis, typed IR, LLVM boundary | [PIPELINE-AND-IR.md](PIPELINE-AND-IR.md); [ADR 0007](adr/0007-compiler-pipeline-and-ir.md) | [#6](https://github.com/vsimko/gfabasic-rust/issues/6) |
| Source formatting | Lossless, evidence-gated formatter library and canonical style for the implemented subset | [FORMATTER.md](FORMATTER.md) | [#208](https://github.com/vsimko/gfabasic-rust/issues/208) |
| Language server and editor integration | Shared compiler frontend, LSP lifecycle, UTF-16 positions, versioned edits, capability gates | [ADR 0012](adr/0012-language-server-and-editor-integration.md) (proposed) | [#209](https://github.com/vsimko/gfabasic-rust/issues/209) |
| Diagnostics and compatibility reporting | Stable diagnostic codes, source locations, support outcomes, migration guidance | [DIAGNOSTICS.md](DIAGNOSTICS.md) | [#26](https://github.com/vsimko/gfabasic-rust/issues/26) |
| Runtime | Runtime layers and SDL3 adapter boundary | [RUNTIME.md](RUNTIME.md) | [#7](https://github.com/vsimko/gfabasic-rust/issues/7) |
| Memory model | Storage, pointers, hardware access, safety policy | [MEMORY.md](MEMORY.md) | [#8](https://github.com/vsimko/gfabasic-rust/issues/8) |
| CLI and configuration | Commands, TOML project schema, precedence, diagnostics, output artifacts | [CLI-AND-CONFIG.md](CLI-AND-CONFIG.md); [ADR 0005](adr/0005-cli-and-project-configuration.md) | [#9](https://github.com/vsimko/gfabasic-rust/issues/9) |
| Compatibility testing | Test taxonomy, fixtures, conformance, deterministic behavior, release criteria | [TESTING.md](TESTING.md); [corpus conventions](../../tests/corpus/README.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#41](https://github.com/vsimko/gfabasic-rust/issues/41), [#42](https://github.com/vsimko/gfabasic-rust/issues/42), [#43](https://github.com/vsimko/gfabasic-rust/issues/43) |
| Compatibility matrix | Current dialect/version/profile/target support status | [COMPATIBILITY-MATRIX.md](COMPATIBILITY-MATRIX.md) | [#10](https://github.com/vsimko/gfabasic-rust/issues/10), [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |
| Frontend fuzzing | Fuzz targets, corpus separation, resource budgets, and CI policy | [FUZZING.md](FUZZING.md) | strategy [#43](https://github.com/vsimko/gfabasic-rust/issues/43), implementation [#118](https://github.com/vsimko/gfabasic-rust/issues/118) |
| Rust workspace | Crate/module boundaries and dependency policy | [WORKSPACE.md](WORKSPACE.md) | [#11](https://github.com/vsimko/gfabasic-rust/issues/11) |
| API registry | Canonical TOML declarations, evidence/support states, schema validation | [API-REGISTRY.md](API-REGISTRY.md); [ADR 0006](adr/0006-api-registry.md) | [#12](https://github.com/vsimko/gfabasic-rust/issues/12), metadata validation [#55](https://github.com/vsimko/gfabasic-rust/issues/55) (complete), runtime handler resolution [#115](https://github.com/vsimko/gfabasic-rust/issues/115) |
| Versioning and compatibility lifecycle | Compiler releases, schema versions, runtime ABI, deprecation | [VERSIONING.md](VERSIONING.md); [ADR 0010](adr/0010-versioning-and-compatibility-lifecycle.md) | [#58](https://github.com/vsimko/gfabasic-rust/issues/58) |
| Performance benchmarks | Compiler, generated-program, runtime, and build measurements | [BENCHMARKING.md](BENCHMARKING.md) | [#60](https://github.com/vsimko/gfabasic-rust/issues/60) |
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

- Which 3.0/3.03 language rules are inherited by Amiga GFA BASIC 3.5, and which require explicit 3.5 confirmation or page-image verification?
- Which Amiga and Atari ST manual editions are authoritative for each profile?
- What is the exact documentation format requirement referred to as “OKF” in issue #14?
- Which LLVM version and Rust binding are suitable for the chosen Rust toolchain?
- Which compiler components justify separate Cargo crates?
- Which runtime operations require software emulation rather than direct platform adapters?

Record evidence and decisions in the relevant issue and ADR before claiming closure.
