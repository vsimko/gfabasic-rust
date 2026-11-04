# Contributor documentation

**Audience:** People who want to review, research, or contribute to this repository.

**Status:** Documentation-only CI is implemented. The repository does not yet have a Rust workspace or executable compiler/runtime build/test workflow.

## Before making a change

1. Read the [project README](../../README.md) and the [architecture overview](../architecture/README.md).
2. Check the open [GitHub issues](https://github.com/vsimko/gfabasic-rust/issues) and pull requests to avoid duplicate work.
3. Follow the [documentation policy](../DOCUMENTATION.md) for Markdown, evidence, citations, and copyright.
4. For language or API research, consult the [source register](../reference-sources.md) and keep Amiga and Atari ST evidence separate.
5. For test design, read [compatibility testing and release criteria](../architecture/TESTING.md) and the [conformance corpus instructions](../../tests/corpus/README.md).

## Current development constraints

There is no `Cargo.toml`, Rust workspace, compiler, runtime, or CLI on the default branch. Therefore, `cargo build` and `cargo test` are not available as project commands. The documentation-only GitHub Actions workflow is implemented in [docs.yml](../../.github/workflows/docs.yml); it runs Markdown linting and offline local-link/anchor validation.

Run the same documentation checks locally from the repository root:

```sh
npx --yes markdownlint-cli2@0.24.0 "**/*.md" "#node_modules"
lychee --offline --include-fragments anchor-only --root-dir "$PWD" "**/*.md"
```

Do not claim that a compiler/runtime build or test passes. The [workspace architecture](../architecture/WORKSPACE.md) describes the current proposal. The full CI workflow design remains proposed in [CI architecture](../architecture/CI.md); compiler/runtime, integration, fuzz, security, and release workflows are not implemented.

## Pull request expectations

- Keep changes small and scoped to an issue or a clearly described defect.
- Preserve accepted architecture decisions. Mark proposals as proposals.
- Cite the applicable dialect, platform, version, source edition, and page/section for language/API claims.
- Keep copyrighted manuals, ROMs, disk images, and unlicensed examples out of the repository.
- Distinguish research candidates from verified conformance tests.
- Report checks that were actually run and state when a check is unavailable.

## Future development setup

A reproducible development setup guide will be added after the workspace, toolchain, dependencies, and CI commands are accepted and can be tested from a clean Linux environment. See the [documentation index](../README.md#planned-user-and-contributor-pages) for the planned page location.
