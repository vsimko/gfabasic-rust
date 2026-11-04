# GitHub Actions workflow architecture

- Status: Proposed full CI architecture. The documentation workflow, fast Rust workspace-check workflow, and RustSec advisory-audit workflow are implemented. Selective test matrices, compiler/runtime integration, fuzzing, license/SBOM policy, release, and branch-protection configuration remain open.
- Tracking: [issue #27](https://github.com/vsimko/gfabasic-rust/issues/27).

## Goals

CI should give fast, predictable feedback while avoiding expensive work that is unrelated to a pull request. A skipped component job must be distinguishable from a missing or failed required check.

The workflow design must preserve the repository's current architecture:

- Frontend and project-owned IR checks must not require LLVM or SDL3.
- Compiler, generated-program runtime, and Linux platform checks remain separate.
- Documentation and research-only corpus changes must not imply language conformance.
- Long fuzzing, compatibility experiments, and benchmarks must not slow every pull request.

This document defines the proposed full CI structure. The documentation workflow in [`.github/workflows/docs.yml`](../../.github/workflows/docs.yml) runs Markdown linting, API registry validation, and offline local-link/anchor checks. The fast Rust workflow in [`.github/workflows/rust.yml`](../../.github/workflows/rust.yml) runs formatting, Clippy, workspace tests, and a workspace build on the pinned Rust toolchain. These checks validate the current skeleton only; no compiler/runtime behavior or conformance is tested, and no branch-protection requirement is claimed.

Release and artifact requirements are specified in [RELEASES.md](RELEASES.md). That policy remains proposed until the workspace, compiler/runtime, and packaging commands exist.

## Proposed workflow set

| Workflow | Events | Purpose | Required on pull requests |
|---|---|---|---|
| `docs.yml` | `pull_request`, `push` to `main` (documentation paths) | Markdown structural lint and offline local links/anchors | Runs for matching documentation changes; branch protection is not configured |
| `pull-request.yml` | `pull_request` | Change detection, fast relevant checks, and one stable gate result | Proposed; not implemented |
| `main.yml` | `push` to `main` | Broader integration checks after merge | No separate PR requirement |
| `fuzz.yml` | `schedule`, `workflow_dispatch` | Longer frontend fuzz campaigns after harnesses exist | No |
| `release.yml` | `release` and controlled `workflow_dispatch` | Reproducible release validation and artifact publication after packaging exists | Release-specific |
| `security-audit.yml` | `pull_request`, `push` to `main` (Cargo/toolchain paths), weekly `schedule`, `workflow_dispatch` | RustSec advisory audit of the committed `Cargo.lock` | Runs for matching dependency changes; branch protection is not configured. License and SBOM checks remain open under #39 |

Do not add empty placeholder workflows. Introduce each workflow when its checks have an executable command and a clean-environment validation path.

## Event and duplicate-run policy

- Use `pull_request` for validation of proposed changes. Cover opened, synchronize, reopened, and ready-for-review transitions as needed.
- Use `push` only for the default branch and release tags. Do not add a general feature-branch `push` workflow that duplicates the pull-request run.
- Use `workflow_dispatch` for reproducible manual investigations and longer jobs.
- Use `schedule` only for work that benefits from periodic execution, such as longer fuzzing or dependency review. Scheduled jobs should operate on the default branch.
- Use release events only after the release artifact and signing/provenance policy is defined.
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

Use one stable, non-matrix final job name (proposed: `pr-gate`) as the branch-protection requirement. Individual component jobs may be skipped when the change detector proves they are irrelevant; the final gate must still run.

The gate must:

1. Depend on change detection and every conditional check job.
2. Run even when one of its dependencies fails or is skipped (for example, with an always-run condition).
3. Fail if change detection fails, a required relevant job fails/cancels/times out, or a required job was unexpectedly skipped.
4. Accept a skipped component job only when its category is explicitly irrelevant according to successful change detection.
5. Report which categories ran, were skipped as irrelevant, or failed.

Do not mark each matrix entry or optional component job as a required branch-protection check. A stable final gate avoids a pull request being blocked merely because a path-aware job was correctly skipped. The gate implementation itself must be tested with representative documentation-only, frontend, runtime, platform, manifest, and CI-only changes.

A branch-protection rule or repository ruleset must require the final gate after it has been introduced and verified. Until that configuration is confirmed, do not assume that an empty status list means a passing check or that a pull request is protected.

## Permissions and supply-chain policy

- Set workflow-level permissions to `contents: read` by default. Grant additional permissions only to the specific job that needs them.
- Pull-request validation must not receive deployment credentials, publishing tokens, or write access to repository contents.
- Keep secrets out of fork and untrusted pull-request workflows. Do not print environment variables or credentials in logs.
- Pin third-party actions to immutable commit SHAs where practical, and include a human-readable version comment. Review action updates like dependency changes.
- Prefer maintained actions and avoid downloading unpinned executables in build scripts.
- Keep release publishing in a separate workflow with explicit environment approval and narrowly scoped write permissions.
- Record native toolchain and system-library versions used by CI so failures can be reproduced locally.\n- The [security audit workflow](../../.github/workflows/security-audit.yml) uses a read-only token, immutable checkout SHA, Rust 1.99.1, and pinned `cargo-audit` 0.22.2. It audits all Cargo dependencies against the current RustSec database and fails on vulnerability advisories. License allowlists and SBOM output are intentionally not guessed; see [SECURITY.md](SECURITY.md).

## Timeouts, artifacts, and diagnostics

- Give every job an explicit timeout appropriate to its work. Fast PR jobs should have bounded execution.
- Upload sanitized logs, test reports, and minimized failure reproducers when they materially help diagnosis. Do not upload secrets, copyrighted source collections, ROMs, disk images, or untriaged large fuzz corpora.
- Preserve the failing command, toolchain versions, target, and relevant configuration in test output.
- Keep scheduled fuzzing and benchmark artifacts separate from required PR artifacts.
- Avoid automatic retries for deterministic failures. A retry must not turn a reproducible failure into an unexplained green result.

## Initial rollout

1. Implement the documentation checks under #33 once the documentation checker and its limitations are agreed.
2. Add the minimal pull-request workflow and stable gate when the first executable workspace checks exist.
3. Test path selection and gate behavior with representative synthetic changes before making the gate required.
4. Add compiler, runtime, and platform jobs as those components become buildable.
5. Add fuzz, license/SBOM, benchmark, and release workflows only when their respective tools and policies are ready. The RustSec advisory workflow is already implemented separately.
6. Configure and verify branch protection/rulesets after the required gate has run successfully on a real pull request.

No workflow should claim to validate a component that does not yet exist. A green documentation-only gate is not evidence that the compiler, runtime, or GFA BASIC conformance is correct.

## Cache policy and measured baseline

Do not add a shared build-artifact cache until it is likely to reduce total workflow time. The current workspace has no third-party Cargo dependencies: the committed `Cargo.lock` contains only the three local workspace packages. There are therefore no meaningful registry/git dependency downloads to reuse, and the current build outputs are small.

A baseline review on 2026-11-04 measured two successful Rust workflow runs at approximately 20 seconds each (GitHub Actions runs [41831742392](https://github.com/vsimko/gfabasic-rust/actions/runs/41831742392) and [41831868187](https://github.com/vsimko/gfabasic-rust/actions/runs/41831868187)). This is shorter than a typical cache restore/save cycle can reliably improve, and does not justify mutable shared build artifacts at this stage.

**Current decision: no Cargo registry, Git, LLVM, SDL3, or `target/` cache is configured.** This avoids stale native artifacts, cross-toolchain reuse, cache-key complexity, and concurrent writers while there is no measurable benefit. Clean builds remain the source of truth.

Re-evaluate caching when one or more of these triggers occurs:

- The workspace adds external crates and registry/git downloads become material.
- LLVM or SDL3 must be downloaded or built in CI.
- Repeated clean Rust workflow runs exceed 60 seconds and build/restore measurements identify a reusable expensive step.
- Multiple compatible jobs need the same immutable toolchain/dependency artifacts.

When a cache is introduced, keys must include OS/architecture, the exact Rust toolchain, `Cargo.lock`, relevant native dependency versions, target triple, and build profile. Prefer immutable dependency/toolchain caches first. Cache build artifacts only after measuring restore/save cost against a clean build. Never cache credentials, final release artifacts, or outputs whose provenance cannot be reproduced. A cache miss or corruption must fall back to a clean build.
