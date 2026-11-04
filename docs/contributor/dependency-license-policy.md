# Dependency License Policy

This policy applies to third-party dependencies used by the Rust workspace and its
supporting development, test, fuzzing, and release tooling.

## Default allowlist

The following SPDX license identifiers are approved for new dependencies by default:

- `MIT`
- `Apache-2.0`
- `BSD-2-Clause`
- `BSD-3-Clause`
- `ISC`
- `Zlib`
- `Unicode-3.0`

A dependency is eligible for automatic policy acceptance only when its applicable
license expression is fully covered by this allowlist. For dual-licensed
dependencies, the project may select an allowed licensing option where the
upstream license expression permits that choice. Do not assume that an
unrecognized identifier, custom license, or additional license exception is
approved.

## Individual approval

Copyleft licenses and all licenses not listed in the default allowlist require
explicit maintainer approval before a dependency is added or enabled.

A proposed exception must be raised for review with:

- the dependency and version;
- the exact SPDX license expression and relevant license texts;
- why the dependency is needed and whether alternatives exist;
- the intended use (runtime, build, test, fuzz, or release tooling);
- any redistribution or notice obligations.

The repository maintainer must explicitly approve or reject the proposal.
Record the decision and its scope in the relevant GitHub issue or pull request.
Approval for one dependency/version/use does not automatically approve other
versions, dependencies, or uses. Until approval is recorded, treat the
dependency as not approved.

## Ongoing review

For every new or materially changed dependency, review its license together
with maintenance status, security advisories, platform support, native build
requirements, and reproducibility. Re-check license metadata when updating
dependencies; do not rely solely on package-name allowlists.

Project-owned source and documentation remain MIT-licensed. This policy does
not grant rights to third-party manuals, ROMs, disk images, or historical
sample programs; those materials remain excluded from redistribution by
default.

## SBOM

A Software Bill of Materials (SBOM) is required for every project release.
The generation process must be reproducible and tested, and the SBOM must
describe the dependencies actually included in the corresponding release
artifacts.

**SPDX 2.3 is the approved SBOM specification version.** Generate release
SBOMs using SPDX 2.3. The generation tooling and retention/publication
location remain to be decided and are tracked in issue #39.
