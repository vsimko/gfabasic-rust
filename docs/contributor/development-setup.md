# Development setup

## Scope

The current workspace is a compileable architectural skeleton. It does not yet implement a GFA BASIC compiler, runtime behavior, or command-line interface. These instructions validate the Rust workspace only; they do not compile GFA BASIC programs.

The initial CI host is Linux x86-64 using the GitHub-hosted Ubuntu runner. Other development hosts may work with Rust, but are not yet part of the project's validated environment matrix.

## Required tools

- Rust 1.99.1, pinned by `rust-toolchain.toml`.
- The `rustfmt` and `clippy` Rust components.
- Git.

The workspace currently has no third-party Cargo dependencies or required native system libraries. LLVM and SDL3 are deliberately deferred and are not required to build the skeleton. Do not install arbitrary LLVM or SDL3 versions for this stage.

The project license is not yet selected. See [issue #78](https://github.com/vsimko/gfabasic-rust/issues/78); repository visibility does not grant permission to reuse the code.

## Install the toolchain

Install rustup using the official Rust installation instructions for your operating system. Then run these commands from the repository root:

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup default 1.99.1
rustc --version
cargo --version
```

The toolchain file also selects Rust 1.99.1 when commands run inside this repository.

## Build and test

Run the same checks used by the Rust pull-request workflow:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --all-targets
```

The workspace contains no GFA BASIC conformance tests yet. Successful Cargo tests therefore validate the crate skeleton, not language compatibility.

For documentation changes, run the separate checks:

```sh
npx --yes markdownlint-cli2@0.24.0 "**/*.md" "#node_modules"
lychee --offline --include-fragments anchor-only --root-dir "$PWD" "**/*.md"
```

The documentation workflow also validates the API registry and its validator tests.

## Troubleshooting

### The pinned toolchain cannot be found

Run the `rustup toolchain install` command above. If the version is unavailable on a host mirror, do not silently substitute a different compiler; report the mirror/toolchain problem and keep CI on the repository pin.

### Formatting or Clippy fails

Run the failing command locally with the pinned toolchain. Do not add broad lint suppressions to make the check pass. Fix the source or document a narrowly justified exception.

### Cargo reports a lockfile or workspace error

Check that commands run from the repository root and that all workspace members listed in the root `Cargo.toml` exist. Do not delete `Cargo.lock` to hide a dependency-resolution difference.

### A GFA BASIC build command is unavailable

There is no implemented compiler or CLI yet. Do not invent or document a `gfabasic build` command until the CLI issue is implemented and tested.

### LLVM or SDL3 is missing

These dependencies are not part of the current workspace skeleton. They must be selected and added only in the relevant compiler backend or Linux adapter work, with pinned versions and a reproducible clean-build strategy.

## CI parity

- [Rust checks](../../.github/workflows/rust.yml) run formatting, Clippy, tests, and workspace build on Rust 1.99.1.
- [Documentation checks](../../.github/workflows/docs.yml) run Markdown linting, API registry validation, and local-link/anchor validation.

When adding a new check, update this guide and the corresponding workflow in the same change.
