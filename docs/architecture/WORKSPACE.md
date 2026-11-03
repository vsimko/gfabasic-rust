# Rust workspace and dependency policy

- Status: Proposed; the initial crate split is described in [ADR 0001](adr/0001-initial-rust-workspace.md). Maintainer review remains open under [issue #11](https://github.com/vsimko/gfabasic-rust/issues/11).

## Logical components

The compiler responsibilities are source management/diagnostics, lexing/parsing/AST, semantic analysis/symbol tables, project-owned typed IR, LLVM code generation, and CLI/configuration. Generated-program responsibilities are the language-core runtime and API-profile support. Linux-specific services and the SDL3 adapter form a separate platform boundary.

Keep small components as modules. Create a Cargo crate only when it has a distinct ownership, reuse, build-isolation, testing, or dependency-boundary reason.

## Proposed initial crate layout

```text
crates/
  gfabasic-compiler/
    src/
      source/
      diagnostics/
      frontend/
      semantic/
      ir/
      backend/
      cli/
  gfabasic-runtime/
    src/
      core/
      profiles/
      platform/
  gfabasic-platform-linux/
    src/
      sdl3/
```

The compiler crate owns compiler stages and the CLI. The runtime crate owns generated-program core behavior, profile runtime support, and the platform-service contract. The Linux platform crate owns Linux-specific implementations of that contract, including SDL3 integration when selected.

Do not create separate crates for the lexer, parser, semantic analyzer, IR, LLVM backend, CLI, Amiga profile, or Atari ST profile at bootstrap. Keep those as modules until independent ownership or dependency isolation is demonstrated.

## Dependency direction

- Compiler modules may depend on other compiler modules along the documented pipeline; the frontend and project-owned IR must not depend on LLVM or SDL3.
- The compiler must not call SDL3 directly. The backend may emit references to the documented runtime ABI; it does not need a Rust dependency on the runtime implementation merely to describe those references.
- The runtime crate must not depend on the compiler, LLVM, or SDL3.
- The Linux platform crate may depend on the runtime's platform-service contract. The runtime must not depend on the Linux platform crate.
- Platform-specific runtime code must not leak into parsing or language-core semantics.
- API profiles are selected by compiler/project configuration for each compilation. Cargo features are build-time capability switches only; they must not select the source dialect or API profile.
- Keep the default compiler/frontend and runtime builds free of LLVM and SDL3 system dependencies. Add those dependencies only to the backend or platform boundary that needs them.

The proposed dependency direction is:

```text
gfabasic-compiler  -- emits runtime ABI references --> generated program
gfabasic-platform-linux  ------>  gfabasic-runtime
gfabasic-runtime  ------>  Rust standard library
```

The generated program links the runtime components required by its selected target and API profile. The exact link model and runtime ABI remain open under the runtime and CLI design issues; this diagram is a crate-dependency boundary, not a final linker specification.

## Cargo and toolchain policy

- Use a Cargo workspace with resolver version 3 and Rust edition 2024 for new workspace packages.
- Pin the development toolchain to an exact stable Rust release in `rust-toolchain.toml`, including `rustfmt` and `clippy`. The current candidate for the initial bootstrap is Rust 1.99.1; refresh this candidate if a newer stable release is available when issue #44 is implemented.
- Keep the package `rust-version` (MSRV) separate from the pinned developer toolchain. Do not publish an MSRV guarantee until the declared minimum is tested in CI.
- Commit the workspace `Cargo.lock`. Use explicit dependency versions and workspace dependency declarations when multiple packages share a dependency.
- Keep LLVM bindings and SDL3 bindings out of the initial manifests until the selected versions and clean-build strategy have been evaluated. Add them as optional dependencies at their owning boundary, not as workspace-wide defaults.
- Review dependency maintenance, platform support, license, security advisories, native build requirements, and reproducibility before adding a dependency. Avoid wildcard versions and build scripts that download unpinned binaries.
- Keep unsafe FFI in the narrowest possible adapter modules. Document each unsafe boundary and its invariants; do not spread platform FFI into compiler/frontend crates.

## Stable and internal interfaces

The crate split is an implementation boundary, not a promise that every Rust item is a stable public API. Keep internal modules private by default. Expose only the interfaces required between compiler/runtime/platform crates. Version the generated-program runtime ABI independently from the compiler product, as described in [VERSIONING.md](VERSIONING.md).

## Deferred decisions

This proposal does not select an LLVM version or Rust binding, an SDL3 binding, a runtime ABI, a dynamic/static linking policy, or stable API-profile identifiers. Record each decision after compatibility and clean-build evaluation. Do not add a placeholder dependency or claim an integration exists merely to reserve a future interface.

Tracking: [issue #11](https://github.com/vsimko/gfabasic-rust/issues/11); workspace bootstrap [issue #44](https://github.com/vsimko/gfabasic-rust/issues/44).
