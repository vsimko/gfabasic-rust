# Development setup

## Scope

The current workspace includes a source map, strict UTF-8 file-loading primitive, structured diagnostics, and an experimental, narrowly scoped Amiga GFA BASIC 3.5 frontend for the addendum's FACT assignment/PRINT example. The `gfabasic build` command uses a temporary C/`cc` bootstrap to produce a native host executable for that subset only. It is not general GFA BASIC support or original-interpreter conformance. General lexical/grammar coverage, semantic analysis, typed IR, LLVM code generation, project configuration, runtime behavior, and platform integration remain unimplemented. These instructions validate the Rust workspace and prototype tests; they do not establish general language compatibility.

The initial CI host is Linux x86-64 using the GitHub-hosted Ubuntu runner. Other development hosts may work with Rust, but are not yet part of the project's validated environment matrix.

## Required tools

- Rust 1.99.1, pinned by `rust-toolchain.toml`.
- The `rustfmt` and `clippy` Rust components.
- Git.
- Node.js 22 and npm/npx for Markdown linting.
- Python 3.12 and `jsonschema==4.25.1` for API registry validation.
- lychee v0.24.2 for local Markdown link and anchor checks.

The compiler crate has one third-party Cargo dependency, `unicode-width` 0.2.2, for terminal display-cell measurement; its MIT OR Apache-2.0 license is within the approved allowlist. No required native system libraries are needed. LLVM and SDL3 are deliberately deferred and are not required to build the current workspace. Do not install arbitrary LLVM or SDL3 versions for this stage.

The project source code and original documentation are licensed under MIT. See the root [LICENSE](../../LICENSE) and [project README](../../README.md#license). This license does not grant redistribution rights to third-party manuals, ROMs, disk images, or sample programs.

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
python -m pip install --disable-pip-version-check jsonschema==4.25.1
python scripts/validate_api_registry.py
python scripts/validate_corpus.py
python -m unittest discover -s scripts/tests -p "test_*.py"
```

The documentation workflow also validates the API registry and its validator tests. Run the same checks locally after installing the pinned Python dependency and lychee version:

## Troubleshooting

### The pinned toolchain cannot be found

Run the `rustup toolchain install` command above. If the version is unavailable on a host mirror, do not silently substitute a different compiler; report the mirror/toolchain problem and keep CI on the repository pin.

### Formatting or Clippy fails

Run the failing command locally with the pinned toolchain. Do not add broad lint suppressions to make the check pass. Fix the source or document a narrowly justified exception.

### Cargo reports a lockfile or workspace error

Check that commands run from the repository root and that all workspace members listed in the root `Cargo.toml` exist. Do not delete `Cargo.lock` to hide a dependency-resolution difference.

### A GFA BASIC build command is unavailable

The minimal CLI supports help and version output. The `build` and `check` commands are present only as explicit unimplemented responses; they do not compile source. Do not document them as usable compilation commands.

### LLVM or SDL3 is missing

These dependencies are not part of the current workspace skeleton. They must be selected and added only in the relevant compiler backend or Linux adapter work, with pinned versions and a reproducible clean-build strategy.

## CI parity

- [Rust checks](../../.github/workflows/rust.yml) run formatting, Clippy, tests, and workspace build on Rust 1.99.1.
- [Documentation checks](../../.github/workflows/docs.yml) run Markdown linting, API registry/corpus validation, validator tests, and local-link/anchor validation.
- [Fuzzing](../../.github/workflows/fuzz.yml) runs bounded campaigns against the generic source-loader transport boundary only; it does not fuzz a GFA BASIC lexer or parser.
- [Security audit](../../.github/workflows/security-audit.yml) checks the root, fuzz, and benchmark Cargo lockfiles against RustSec.
- [Benchmarks](../../.github/workflows/benchmarks.yml) validate the benchmark harness on pull requests and measure synthetic source loading on main/scheduled/manual runs.
- [Release SBOM](../../.github/workflows/release-sbom.yml) generates and validates the workspace SPDX 2.3 SBOM on pull requests and is configured to attach it to published GitHub Releases; an actual release upload has not yet been exercised.

These workflows validate repository tooling and the current source-loading primitives. They do not establish GFA BASIC language conformance or compiler/runtime support.

When adding a new check, update this guide and the corresponding workflow in the same change.
