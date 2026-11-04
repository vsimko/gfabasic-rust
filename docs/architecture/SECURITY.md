# Dependency and supply-chain security

- Status: **RustSec advisory auditing is implemented.** License allowlist, exception approvals, and release SBOM format remain pending maintainer approval.
- Tracking: [issue #39](https://github.com/vsimko/gfabasic-rust/issues/39).

## Dependency advisory policy

The dedicated [security audit workflow](../../.github/workflows/security-audit.yml) runs when Cargo manifests/lockfiles or the audit workflow change, weekly against the default branch, and by manual dispatch.

- The workflow installs `cargo-audit 0.22.2` with `--locked` and audits the committed `Cargo.lock` against the current RustSec advisory database.
- Any reported vulnerability advisory fails the job, regardless of CVSS score. This is deliberately stricter than a high-severity-only threshold because the project currently has no third-party Cargo dependencies and no accepted exception list.
- Cargo Audit warnings about unmaintained, unsound, or yanked crates remain visible but are not fatal under the default command. A future maintainer-approved policy may promote selected warning classes to failures.
- The audit uses no write-capable token. Workflow permissions are limited to `contents: read`; the command's exit status and package/advisory details provide the actionable failure output.
- The audit is an advisory-database check, not a complete security review. It does not replace code review, static analysis, secret scanning, binary analysis, or OS-level dependency checks.

The current lockfile contains only the three local workspace packages and no third-party crates. The first successful audit on 2026-11-04 ([workflow run 41832754189](https://github.com/vsimko/gfabasic-rust/actions/runs/41832754189)) loaded the RustSec advisory database and passed against those three workspace package entries. This establishes the workflow baseline, not a claim that future dependencies are safe.

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

The repository itself is MIT-licensed, but that does not automatically approve every third-party dependency license. No dependency-license allowlist or exception policy is accepted yet. The maintainer must approve the allowed SPDX identifiers and how dual-licensed crates are handled before a license gate is enabled.

No release SBOM is generated yet. Before packaging/release work, the maintainer should select SPDX or CycloneDX and the required version/output format. The generation command and tool version must then be pinned and the output made reproducible from the release lockfile.

## Workflow supply-chain controls

- The audit workflow uses a full immutable commit SHA for checkout.
- The workflow grants only read access to repository contents.
- Rust and Cargo Audit versions are pinned.
- Cargo Audit's transitive tool dependencies are installed with the published lockfile via `--locked`.
- The workflow runs on dependency changes and a weekly schedule; manual dispatch is available for incident response.

These controls reduce workflow drift but do not establish provenance or signatures for every future dependency. Review action SHAs, tool versions, and workflow permissions when updating CI.
