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

| Topic | Scope | Tracking |
|---|---|---|
| Project goals and compatibility | Purpose, non-goals, conformance dimensions | [#2](https://github.com/vsimko/gfabasic-rust/issues/2) |
| Dialects and versions | Source dialect identity and version-specific rules | [#3](https://github.com/vsimko/gfabasic-rust/issues/3) |
| Target model | Host/target distinction and target capabilities | [#4](https://github.com/vsimko/gfabasic-rust/issues/4) |
| API profiles | Core, Amiga, and Atari ST profile boundaries | [#5](https://github.com/vsimko/gfabasic-rust/issues/5) |
| Compiler pipeline and IR | Frontend stages, semantic analysis, typed IR, LLVM boundary | [#6](https://github.com/vsimko/gfabasic-rust/issues/6) |
| Runtime | Runtime layers and SDL3 adapter boundary | [#7](https://github.com/vsimko/gfabasic-rust/issues/7) |
| Memory model | Storage, pointers, hardware access, safety policy | [#8](https://github.com/vsimko/gfabasic-rust/issues/8) |
| CLI and configuration | Inputs, profiles, outputs, reproducibility | [#9](https://github.com/vsimko/gfabasic-rust/issues/9) |
| Compatibility testing | Test layers and release criteria | [#10](https://github.com/vsimko/gfabasic-rust/issues/10) |
| Rust workspace | Crate/module boundaries and dependency policy | [#11](https://github.com/vsimko/gfabasic-rust/issues/11) |
| API registry | Canonical machine-readable declarations | [#12](https://github.com/vsimko/gfabasic-rust/issues/12) |

Detailed decisions remain open until the corresponding research and review are complete. This index does not claim that the linked architecture issues are resolved.

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

Use numbered Architecture Decision Records in [adr/](adr/). Each record states context, decision, status, alternatives, consequences, and evidence. An accepted ADR can be superseded by a later ADR; do not silently rewrite the historical rationale.

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
