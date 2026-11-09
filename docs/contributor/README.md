---
type: "Contributor Guide"
title: "Contributor documentation"
---

# Contributor documentation

**Audience:** People who want to review, research, or contribute to this repository.

**Status:** The Rust 1.99.1 workspace and fast Rust CI checks are implemented. The compiler library includes source-map/file-loading primitives and a structured diagnostic renderer; the minimal CLI supports help/version only. GFA BASIC frontend, project configuration, compilation, runtime behavior, and platform integration are not implemented.

## Before making a change

1. Read the [project README](../../README.md) and the [architecture overview](../architecture/README.md).
2. Check the open [GitHub issues](https://github.com/vsimko/gfabasic-rust/issues) and pull requests to avoid duplicate work.
3. Follow the [documentation policy](../DOCUMENTATION.md) for Markdown, evidence, citations, and copyright.
4. For language or API research, consult the [source register](../reference-sources.md) and keep Amiga and Atari ST evidence separate.
5. For test design, read [compatibility testing and release criteria](../architecture/TESTING.md) and the [conformance corpus instructions](../../tests/corpus/README.md).

## Development setup

Follow the [development setup guide](development-setup.md) for the pinned Rust toolchain, local build/test commands, documentation checks, and troubleshooting.

## Current development constraints

The default branch contains a Cargo workspace with compiler, runtime, and Linux platform crates. The compiler crate implements source-map and strict UTF-8 file-loading primitives plus a structured diagnostic model and human renderer. Its minimal CLI supports help/version; `build` and `check` do not compile. The lexer, parser, semantic analysis, IR, backend, project configuration, runtime behavior, and Linux platform adapter remain unimplemented. The pinned toolchain is Rust 1.99.1; LLVM and SDL3 are not required for the current workspace and remain deferred. The project source code and original project documentation are licensed under MIT; see the root [LICENSE](../../LICENSE) file and the [license section in the README](../../README.md#contributing-and-licensing).

The MIT license does not grant redistribution rights to third-party manuals, ROMs, disk images, sample programs, or other externally sourced materials. Check their individual rights and the documentation policy before adding or redistributing them.

## Documentation and workspace checks

Run the same documentation checks locally from the repository root:

```sh
npx --yes markdownlint-cli2@0.24.0 "**/*.md" "#node_modules"
lychee --offline --include-fragments anchor-only --root-dir "$PWD" "**/*.md"
python -m pip install --disable-pip-version-check PyYAML==6.0.2 markdown-it-py==3.0.0
python scripts/validate_okf.py
python scripts/generate_docs_site.py
python -m unittest discover -s scripts/tests -p "test_*.py"
```

Run the workspace checks from the repository root after installing the pinned toolchain:

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup default 1.99.1
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --all-targets
```

The same Rust checks run in [rust.yml](../../.github/workflows/rust.yml). The documentation workflow in [docs.yml](../../.github/workflows/docs.yml) runs Markdown linting, OKF v0.2 bundle validation and website generation, API registry/corpus validation, validator tests, and offline local-link/anchor validation. Separate workflows provide source-loader fuzzing, RustSec audits, source-loading benchmark checks/measurements, and workspace SBOM generation/validation. These checks validate repository tooling and the current source-loading primitives only; they do not validate a GFA BASIC compiler or runtime implementation. Lexer/parser fuzz targets, compiler/runtime conformance, a stable final PR gate, branch protection, and compiler build/package/release workflows remain unimplemented. The release-SBOM upload has not yet been exercised against a real published release.

## Human blockers and AI agent workflow

Follow the repository-wide [AI agent and human-blocker policy](../../AGENTS.md). Any progress-blocking human decision, approval, permission, credential, or manual action must be recorded as an open issue with the `needs-human` label. The issue must specify the requested action and affected work. Do not make owner-only decisions on a maintainer's behalf. Continue independent work while a blocker is waiting for human input.

## Pull request expectations

- Keep changes small and scoped to an issue or a clearly described defect.
- Preserve accepted architecture decisions. Mark proposals as proposals.
- Cite the applicable dialect, platform, version, source edition, and page/section for language/API claims.
- Do not add copyrighted manuals, ROMs, disk images, or unlicensed examples without a separate documented rights basis.
- Distinguish research candidates from verified conformance tests.
- Report checks that were actually run and state when a check is unavailable.
- By submitting a contribution, you agree that the contribution may be distributed as part of this project under the MIT License. You retain copyright in your contribution; no separate copyright assignment is implied.

## Future development setup

The initial development setup is documented above. Add LLVM and SDL3 prerequisites only after their versions and clean-build strategies are selected.
