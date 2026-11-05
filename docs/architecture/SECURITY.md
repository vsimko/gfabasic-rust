# Dependency and supply-chain security

- Status: **RustSec advisory auditing and the approved dependency-license policy are implemented.** Pull-request CI generates and validates the workspace SPDX 2.3 SBOM; the published-release attachment workflow is configured but has not yet been exercised against a real release.
- Tracking: [issue #39](https://github.com/vsimko/gfabasic-rust/issues/39).

## Dependency advisory policy

The dedicated [security audit workflow](../../.github/workflows/security-audit.yml) runs when Cargo manifests/lockfiles or the audit workflow change, weekly against the default branch, and by manual dispatch.

- The workflow installs `cargo-audit 0.22.2` with `--locked` and audits the committed `Cargo.lock` against the current RustSec advisory database.
- Any reported vulnerability advisory fails the job, regardless of CVSS score. This is deliberately stricter than a high-severity-only threshold because no vulnerability-advisory exceptions are currently approved.
- Cargo Audit warnings about unmaintained, unsound, or yanked crates remain visible but are not fatal under the default command. A future maintainer-approved policy may promote selected warning classes to failures.
- The audit uses no write-capable token. Workflow permissions are limited to `contents: read`; the command's exit status and package/advisory details provide the actionable failure output.
- The audit is an advisory-database check, not a complete security review. It does not replace code review, static analysis, secret scanning, binary analysis, or OS-level dependency checks.

The first successful audit on 2026-11-04 ([workflow run 41832754189](https://github.com/vsimko/gfabasic-rust/actions/runs/41832754189)) loaded the RustSec advisory database and passed against the then-current three local workspace package entries. The current root lockfile also contains the approved `unicode-width` 0.2.2 dependency; the workflow audits the complete lockfile, and subsequent dependency additions are subject to the same check. This establishes the workflow baseline, not a claim that future dependencies are safe.

## Exceptions

There is no advisory ignore list. Do not add an exception silently or use broad wildcard ignores.

If a specific advisory cannot be fixed immediately, open or update an issue containing:

1. The full RustSec advisory ID and affected package/version.
2. The dependency path and whether the affected code/feature is reachable.
3. The reason an upgrade or removal is not currently possible.
4. The compensating mitigation and residual risk.
5. An owner-approved expiry/review date and removal plan.

Only add the exact advisory ID to a checked-in `audit.toml` after the maintainer explicitly approves the exception. Revisit it on every dependency update and at the recorded expiry. Do not treat a passing audit with an ignore entry as proof that the advisory is irrelevant.

## License policy and SBOM

The default dependency-license allowlist and exception process are approved and documented in [the dependency-license policy](../contributor/dependency-license-policy.md). Copyleft licenses and identifiers outside the allowlist still require explicit maintainer approval. The root workspace has one direct third-party dependency, `unicode-width` 0.2.2, licensed MIT OR Apache-2.0 and used only for diagnostic terminal-cell measurement; it is within the approved allowlist. Separate fuzz and benchmark workspaces have tooling dependencies, and their lockfiles are included in the audit workflow.

The approved release SBOM is one workspace-level SPDX 2.3 JSON document covering all three packages (`gfabasic-compiler`, `gfabasic-runtime`, and `gfabasic-platform-linux`). The pinned generator is `cargo-sbom` 0.10.0, invoked with `spdx_json_2_3`. The pull-request path generates the document and validates the SPDX version and workspace package coverage. The published-release path is configured to attach the JSON to the corresponding GitHub Release. No real published release has yet exercised that upload, so the final acceptance criterion in [issue #39](https://github.com/vsimko/gfabasic-rust/issues/39) remains open and is labelled `needs-human` pending a release.

## Workflow supply-chain controls

- The audit workflow uses a full immutable commit SHA for checkout.
- The workflow grants only read access to repository contents.
- Rust and Cargo Audit versions are pinned.
- Cargo Audit's transitive tool dependencies are installed with the published lockfile via `--locked`.
- The audit covers the root workspace, separate fuzz workspace, and benchmark workspace lockfiles. It runs on relevant dependency changes, weekly, and by manual dispatch.

These controls reduce workflow drift but do not establish provenance or signatures for every future dependency. Review action SHAs, tool versions, and workflow permissions when updating CI.
