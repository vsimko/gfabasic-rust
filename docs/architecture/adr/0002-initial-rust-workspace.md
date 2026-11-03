# ADR 0002: Initial Rust workspace crate boundaries

- Status: Proposed
- Date: 2026-11-04
- Decision makers: Project maintainers

## Context

The project needs a Rust workspace for a GFA BASIC compiler and generated-program runtime. The planned compiler pipeline includes source management, diagnostics, frontend, semantic analysis, project-owned IR, LLVM code generation, and a CLI. Generated programs need a core runtime and may need platform-specific services. Linux SDL3 integration must remain isolated from language semantics.

Creating one crate for every compiler stage or API profile before interfaces exist would add build and maintenance overhead. Putting everything in one crate would make it difficult to isolate compiler-only dependencies from generated-program runtime dependencies and native platform libraries.

The existing architecture requires the frontend and project-owned IR to remain independent of LLVM and SDL3. It also requires API profiles to be selected per compilation rather than implicitly by Cargo features.

## Decision

This ADR proposes the following initial workspace split:

1. **`gfabasic-compiler`** — one compiler package containing the source manager, diagnostics, lexer/parser/AST, semantic analysis, project IR, LLVM backend, and CLI as modules. The CLI remains a binary target of this package until it has a separate ownership or distribution requirement.
2. **`gfabasic-runtime`** — one generated-program runtime package containing language-core runtime support, API-profile runtime modules, and project-owned platform-service contracts. It must not depend on the compiler, LLVM, or SDL3.
3. **`gfabasic-platform-linux`** — one Linux-specific package containing implementations of runtime platform services and the SDL3 adapter. It may depend on `gfabasic-runtime`; the reverse dependency is prohibited.

Do not create separate crates for the lexer, parser, semantic analyzer, IR, backend, CLI, or individual Amiga/Atari profiles at bootstrap. Revisit a split only when independent ownership, reuse, build isolation, or a concrete dependency constraint justifies it.

Cargo features are build-time capability switches. They may enable an optional LLVM backend or SDL3 adapter after those dependencies are selected, but they must not select a GFA BASIC dialect or API profile. Dialect and API-profile selection belong to compiler/project configuration.

Use Rust edition 2024 and Cargo workspace resolver 3. Pin an exact stable development toolchain and install `rustfmt` and `clippy`. Rust 1.99.1 is the current candidate for the initial bootstrap as of this proposal; the pin must be checked again when the workspace is created. Keep the declared MSRV separate and do not promise one until CI tests it.

Commit `Cargo.lock`, use explicit dependency versions, and review dependency maintenance, license, security, native build requirements, platform support, and reproducibility. Do not add LLVM or SDL3 bindings until their versions and clean-build strategies are evaluated. Keep unsafe FFI within narrow platform/backend adapters and document its invariants.

## Alternatives considered

### One workspace crate

A single package with compiler, runtime, and platform modules would minimize initial manifests. It would also make native SDL3 and future LLVM dependency isolation harder, and blur the generated-program runtime boundary. Rejected as the initial layout.

### A crate for every compiler stage and API profile

Separate crates for source management, lexer, parser, semantic analysis, IR, backend, CLI, and each API profile would create premature public interfaces and dependency coordination. The current architecture does not establish a need for that granularity. Rejected for bootstrap.

### Two crates: compiler and runtime, with SDL3 inside runtime

This would separate compiler dependencies from generated-program support, but the runtime package would then own a platform-specific native dependency. That would make headless builds and non-Linux profile reuse harder. A separate Linux platform crate gives SDL3 a clear boundary. Rejected in favor of the three-package proposal.

## Consequences

- Frontend and IR can be built and tested without LLVM or SDL3 installed.
- Runtime consumers do not need compiler implementation dependencies.
- Linux native dependencies are isolated to one package.
- API profile selection remains a per-compilation concern rather than a Cargo build feature.
- The workspace starts with three packages, not one package per logical compiler stage.
- The exact cross-crate runtime interfaces and generated-program link model still require separate design.
- This ADR is a proposal. It does not authorize implementation of unverified GFA BASIC behavior or claim that any compiler/runtime component exists.

## Evidence and references

- [Workspace and dependency policy](../WORKSPACE.md).
- [Compiler pipeline and IR](../PIPELINE-AND-IR.md).
- [Runtime architecture](../RUNTIME.md).
- [CLI and configuration](../CLI-AND-CONFIG.md).
- [Versioning and compatibility lifecycle](../VERSIONING.md).
- Rust 1.99.1 stable release announcement, 2026-10-15: https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/.

The crate split is an architectural proposal, not an implementation or compatibility claim. LLVM and SDL3 binding choices, runtime ABI, and linking behavior remain unverified and deferred.

## Supersedes / superseded by

None.
