---
type: "Architecture"
title: "GitHub Actions workflow architecture"
---

# GitHub Actions workflow architecture

- Status: Proposed full CI architecture. Implemented workflows cover OKF documentation validation and generation, Markdown/link checks, website preview and production publication, fast Rust workspace checks, source-loader fuzzing, RustSec audits, source-loading benchmark validation/measurement, and SPDX 2.3 workspace SBOM generation/validation with a configured GitHub Release attachment. Lexer/parser fuzzing, compiler/runtime/conformance integration, verified branch protection, and compiler build/package/release workflows remain unimplemented. A stable `pr-gate` workflow is now implemented; it aggregates the existing path-filtered PR workflows but is not yet configured as a required ruleset check. The SBOM attachment has not yet been exercised on a published release.
- Tracking: [issue #27](https://github.com/vsimko/gfabasic-rust/issues/27).

## Goals

CI should give fast, predictable feedback while avoiding expensive work that is unrelated to a pull request. A skipped component job must be distinguishable from a missing or failed required check.

The workflow design must preserve the repository's current architecture:

- Frontend and project-owned IR checks must not require LLVM or SDL3.
- Compiler, generated-program runtime, and Linux platform checks remain separate.
- Documentation and research-only corpus changes must not imply language conformance.
- Long fuzzing, compatibility experiments, and benchmarks must not slow every pull request.

This document defines the proposed full CI structure. The implemented [documentation workflow](../../.github/workflows/docs.yml) runs Markdown linting, OKF v0.2 bundle validation, deterministic documentation-site generation, API registry and corpus-metadata validation, validator tests, and offline local-link/anchor checks. The [PR preview workflow](../../.github/workflows/pr-preview.yml) builds and smoke-tests generated documentation; the [Pages workflow](../../.github/workflows/pages.yml) runs the same source validation/generation on `main`, deploys docs-only changes, and smoke-tests the production documentation. The [Rust workflow](../../.github/workflows/rust.yml) runs formatting, Clippy, workspace tests, and a workspace build on the pinned toolchain. The [fuzz workflow](../../.github/workflows/fuzz.yml) runs bounded AFL++ campaigns against the generic source-loader transport boundary only (30 seconds on relevant pull requests; 10 minutes on weekly/manual runs), not a GFA BASIC lexer or parser. The [security audit workflow](../../.github/workflows/security-audit.yml) checks the root, fuzz, and benchmark Cargo lockfiles against RustSec. The [benchmark workflow](../../.github/workflows/benchmarks.yml) compiles the benchmark harness on pull requests and runs synthetic source-loading measurements on main/scheduled/manual runs. The [release-SBOM workflow](../../.github/workflows/release-sbom.yml) generates and validates SPDX 2.3 JSON for the complete workspace on relevant pull requests and is configured to attach it to a published GitHub Release. The [stable PR gate](../../.github/workflows/pr-gate.yml) runs for every pull request and uses the changed-file list plus a tested relevance map to decide which validation workflows must succeed. All implemented validation workflows now start on every pull request, so an unclassified path can safely require all of them. The gate fails for missing, failed, cancelled, skipped, or timed-out expected runs. These workflows validate the current infrastructure only; they do not test GFA BASIC compilation, runtime/platform behavior, or language conformance. The gate exists, but repository ruleset/branch-protection enforcement still requires independent verification.

The broader compiler release and artifact policy in [RELEASES.md](RELEASES.md) remains proposed because no compiler build, package, or installation artifact exists. The narrow release-SBOM workflow is implemented; its published-release event still needs an actual release to verify the upload.

## Implemented workflow set

| Workflow | Events | Purpose and actual coverage |
|---|---|---|
| `docs.yml` | Every `pull_request`; `push` to `main` for documentation/registry paths | Markdown lint, OKF v0.2 validation and generation, API registry/corpus validation/tests, offline local links and anchors |
| `rust.yml` | Every `pull_request`; `push` to `main` for Cargo/toolchain/crate paths | Formatting, Clippy, workspace tests, and workspace build |
| `fuzz.yml` | Every `pull_request`; weekly `schedule`; `workflow_dispatch` | AFL++ source-loader transport-boundary campaign only; lexer/parser targets do not exist yet |
| `security-audit.yml` | Every `pull_request`; relevant dependency `push` to `main`; weekly `schedule`; `workflow_dispatch` | RustSec audit of the root, fuzz, and benchmark Cargo lockfiles |
| `benchmarks.yml` | Every `pull_request`; relevant `push` to `main`; weekly `schedule`; `workflow_dispatch` | Pull requests format/check the benchmark harness; main/scheduled/manual runs measure synthetic source loading |
| `release-sbom.yml` | Every `pull_request`; published `release` | Pull requests generate and validate the workspace SPDX 2.3 JSON SBOM; published releases are configured to receive the SBOM asset. A real release upload has not yet been exercised. |\n| `pr-gate.yml` | Every `pull_request` (opened, reopened, synchronize, ready for review) | Stable `pr-gate` job aggregates the path-relevant PR workflow runs, rejects missing or unsuccessful expected runs, and selects all workflows for unclassified paths. |

These workflows do not constitute a compiler build, compiler/runtime conformance suite, or full release pipeline. The stable `pr-gate` job is implemented; requiring it through a repository ruleset or branch-protection rule remains unverified.

## Planned workflow components

A stable pull-request gate, component-specific compiler/runtime/platform jobs, and a compiler build/package/publish workflow remain future work. Add them only when their checks have executable commands and clean-environment validation paths.

Do not add empty placeholder workflows. Introduce each workflow when its checks have an executable command and a clean-environment validation path.

## Event and duplicate-run policy

- Use `pull_request` for validation of proposed changes. Cover opened, synchronize, reopened, and ready-for-review transitions as needed.
- Use `push` only for the default branch and release tags. Do not add a general feature-branch `push` workflow that duplicates the pull-request run.
- Use `workflow_dispatch` for reproducible manual investigations and longer jobs.
- Use `schedule` only for work that benefits from periodic execution, such as longer fuzzing, dependency auditing, or repeatable benchmark measurement. Scheduled jobs should operate on the default branch.
- Use release events only for a specific approved publication artifact after its policy is defined. The current release event is limited to the workspace SBOM; it does not build or publish compiler artifacts.
- Do not use `pull_request_target` to check out or execute untrusted pull-request code.
- Apply workflow-appropriate concurrency groups. For pull requests, group by PR number and cancel superseded runs. Do not let a new PR run cancel a release or scheduled campaign.
- Keep deterministic test retries disabled by default. If a retry is permitted for a known infrastructure failure, preserve the original failure and report the retry explicitly.

## Change categories and path policy

The pull-request workflow should detect these categories once a workspace and implementation exist:

| Category | Representative paths | Checks to select |
|---|---|---|
| Documentation | `README.md`, `docs/**`, documentation configuration | Markdown style, local links/anchors, generated-document consistency where available |
| Compiler/frontend | compiler crate source, source manager, lexer, parser, semantic analysis, IR | Formatting, linting, unit/property tests, relevant regression tests |
| Runtime core | runtime crate source and runtime tests | Formatting, linting, runtime unit/integration tests |
| Linux platform | Linux platform crate, SDL3 adapter, platform integration tests | Platform build and relevant integration tests on the declared Linux environment |
| Language corpus and fixtures | `tests/corpus/**`, regression fixtures | Metadata/schema validation and relevant tests; do not treat corpus presence as conformance |
| Fuzz infrastructure | fuzz targets, dictionaries, seed corpora, fuzz configuration | Build targets and bounded smoke fuzzing once implemented |
| Cargo/toolchain | workspace/package manifests, `Cargo.lock`, `rust-toolchain.toml`, Cargo configuration | Broad workspace checks and dependency-policy validation |
| CI configuration | `.github/workflows/**`, CI scripts and configuration | Workflow/configuration validation plus the checks affected by the change |

The exact path patterns must be maintained alongside the repository layout. A new or unclassified path must select a safe broad check rather than silently skipping validation. Changes to manifests, lockfiles, toolchain pins, workflow definitions, or shared test infrastructure should trigger the broadest relevant check set.

Documentation-only changes currently run `docs.yml` and do not run compiler/runtime builds because those components and their workflows do not exist. Once the compiler workflow is implemented, path-aware change detection may skip compiler/runtime jobs only when it proves that no code, manifest, toolchain, test harness, or workflow input changed.

## Required-check contract

Use the stable, non-matrix final job name `pr-gate` as the branch-protection requirement. The gate runs for every pull request and independently determines which existing path-filtered workflows are relevant. Individual workflows may have no run when their path filters prove them irrelevant; the final gate must still run.

The gate must:

1. Depend on change detection and every conditional check job.
2. Run even when one of its dependencies fails or is skipped (for example, with an always-run condition).
3. Fail if change detection fails, a required relevant job fails/cancels/times out, or a required job was unexpectedly skipped.
4. Accept a skipped component job only when its category is explicitly irrelevant according to successful change detection.
5. Report which categories ran, were skipped as irrelevant, or failed.

Do not mark each matrix entry or optional component job as a required branch-protection check. A stable final gate avoids a pull request being blocked merely because a path-aware job was correctly skipped. The gate implementation itself must be tested with representative documentation-only, frontend, runtime, platform, manifest, and CI-only changes.

A branch-protection rule or repository ruleset must require `pr-gate` after its check name and behavior have been verified. The GitHub integration used for this change cannot verify or configure repository rulesets; until a maintainer confirms that configuration, do not claim that pull requests are protected.

## Permissions and supply-chain policy

- Set workflow-level permissions to `contents: read` by default. Grant additional permissions only to the specific job that needs them.
- Pull-request validation must not receive deployment credentials, publishing tokens, or write access to repository contents.
- Keep secrets out of fork and untrusted pull-request workflows. Do not print environment variables or credentials in logs.
- Pin third-party actions to immutable commit SHAs where practical, and include a human-readable version comment. Review action updates like dependency changes.
- Prefer maintained actions and avoid downloading unpinned executables in build scripts.
- Keep release publishing in a separate workflow with explicit environment approval and narrowly scoped write permissions.
- Record native toolchain and system-library versions used by CI so failures can be reproduced locally.
- The [security audit workflow](../../.github/workflows/security-audit.yml) uses a read-only token, immutable checkout SHA, Rust 1.99.1, and pinned `cargo-audit` 0.22.2 to audit the root, fuzz, and benchmark Cargo lockfiles against RustSec. The approved dependency-license policy and SPDX 2.3 SBOM process are documented in [SECURITY.md](SECURITY.md); PR SBOM generation/validation is handled by [release-sbom.yml](../../.github/workflows/release-sbom.yml).

## Timeouts, artifacts, and diagnostics

- Give every job an explicit timeout appropriate to its work. Fast PR jobs should have bounded execution.
- Upload sanitized logs, test reports, and minimized failure reproducers when they materially help diagnosis. Do not upload secrets, copyrighted source collections, ROMs, disk images, or untriaged large fuzz corpora.
- Preserve the failing command, toolchain versions, target, and relevant configuration in test output.
- Keep scheduled fuzzing and benchmark artifacts separate from required PR artifacts.
- Avoid automatic retries for deterministic failures. A retry must not turn a reproducible failure into an unexplained green result.

## Initial rollout

1. Documentation checks and fast Rust workspace checks are implemented for the current repository surface.
2. Source-loader fuzzing, RustSec audits, source-loading benchmark validation/measurement, and workspace SBOM generation/validation are implemented with bounded triggers and scoped permissions.
3. Extend fuzzing only when lexer/parser entry points and their source contracts are stable.
4. Add compiler, runtime, and platform jobs as those components and deterministic tests become available.
5. Add and test a stable final pull-request gate before configuring branch protection or rulesets.
6. Exercise the release-SBOM upload on an actual published GitHub Release. Add compiler build/package/publish automation only after those artifacts and their validation commands exist.

No workflow should claim to validate a component that does not yet exist. A green documentation-only gate is not evidence that the compiler, runtime, or GFA BASIC conformance is correct.

## Cache policy and measured baseline

Do not add a shared build-artifact cache until it is likely to reduce total workflow time. The current workspace has one pinned third-party Cargo dependency (`unicode-width` 0.2.2), so there is a small registry download to reuse. Given the single small dependency and small build outputs, a shared cache is still unlikely to reduce total workflow time; keep clean builds until measurements justify a cache.

A baseline review on 2026-11-04 measured two successful Rust workflow runs at approximately 20 seconds each (GitHub Actions runs [41831742392](https://github.com/vsimko/gfabasic-rust/actions/runs/41831742392) and [41831868187](https://github.com/vsimko/gfabasic-rust/actions/runs/41831868187)). This is shorter than a typical cache restore/save cycle can reliably improve, and does not justify mutable shared build artifacts at this stage.

**Current decision: no Cargo registry, Git, LLVM, SDL3, or `target/` cache is configured.** This avoids stale native artifacts, cross-toolchain reuse, cache-key complexity, and concurrent writers while there is no measurable benefit. Clean builds remain the source of truth.

Re-evaluate caching when one or more of these triggers occurs:

- The workspace adds external crates and registry/git downloads become material.
- LLVM or SDL3 must be downloaded or built in CI.
- Repeated clean Rust workflow runs exceed 60 seconds and build/restore measurements identify a reusable expensive step.
- Multiple compatible jobs need the same immutable toolchain/dependency artifacts.

When a cache is introduced, keys must include OS/architecture, the exact Rust toolchain, `Cargo.lock`, relevant native dependency versions, target triple, and build profile. Prefer immutable dependency/toolchain caches first. Cache build artifacts only after measuring restore/save cost against a clean build. Never cache credentials, final release artifacts, or outputs whose provenance cannot be reproduced. A cache miss or corruption must fall back to a clean build.

## Selective job execution: deferred until component suites exist

The current executable CI surface includes documentation checks, Rust workspace checks, source-loader fuzzing, RustSec audits, source-loading benchmark validation/measurement, and release-SBOM generation/validation with a configured release-asset upload. All validation workflows now run on pull requests; the stable gate requires only the workflows relevant to the changed paths. There are not yet separate frontend, semantic/IR, LLVM, runtime, Amiga, Atari, or SDL3 integration test suites to select between.

**Do not add a job-level change detector or empty matrix now.** It would add a second source of path truth without reducing any existing test work, and there are no component jobs against which to validate dependency fan-out. The path-to-test matrix above remains the design reference, not a claim that selective component execution exists.

Reopen the selective-execution implementation when at least two independent component test suites exist (for example, frontend and platform integration). At that point add a tested change map, conservative handling for unknown paths, explicit dependency fan-out, and a stable final PR gate. Until then the workflow-level path filters are the proportionate implementation.
