# Release, artifact, and reproducible-build policy

- Status: **Proposed; no release workflow, compiler artifact, or runtime package exists.**
- Tracking: [issue #40](https://github.com/vsimko/gfabasic-rust/issues/40).
- Related work: [versioning #58](https://github.com/vsimko/gfabasic-rust/issues/58), [packaging #62](https://github.com/vsimko/gfabasic-rust/issues/62), [security and supply chain #39](https://github.com/vsimko/gfabasic-rust/issues/39).

This document defines a release workflow design. It does not claim that the project can currently build or publish a compiler.

## 1. Release identity

- Use a Git tag of the form `vMAJOR.MINOR.PATCH` for compiler product releases, subject to the versioning policy.
- A release tag must point to a commit on the protected default branch.
- Prefer signed annotated tags. The release process must verify the tag target and signature policy before building.
- Do not publish a release from a pull-request merge ref or an unreviewed branch.
- A pre-release must be explicitly marked as such and must not imply stable language/API compatibility.
- The release record must identify the compiler product version separately from GFA BASIC dialect/version, API profile, configuration/registry schema, runtime ABI, and output target.

## 2. Trigger and workflow boundaries

Proposed triggers:

| Trigger | Purpose | Publication permission |
|---|---|---|
| Pull request | Validate release scripts and packaging metadata without publishing | Read-only |
| Push to `main` | Run integration validation and preserve build diagnostics | Read-only |
| Published GitHub release for a `v*` tag | Build, validate, package, attest, and publish release artifacts | Write permission only in the final publishing job |
| Manual dispatch | Reproduce a release build or run a dry-run against an explicit tag | Read-only by default; publishing requires an explicit protected environment |

Do not publish artifacts from a pull-request workflow. Do not use `pull_request_target` to execute untrusted code. Keep build and test jobs separate from the job that creates or uploads a GitHub release.

The publishing job should depend on successful completion of all required build and validation jobs. Use a protected GitHub Environment with reviewer approval for public releases. Grant `contents: write` only to the publishing job; all other jobs use `contents: read`. Do not expose release credentials to fork pull requests or build scripts.

## 3. Release gates

Before publication, the workflow must verify:

1. The tag resolves to the intended reviewed commit and satisfies the tag policy.
2. The pinned Rust toolchain, LLVM version, system dependencies, and SDL3 integration versions match the declared build environment.
3. Formatting, linting, unit tests, integration tests, and relevant documentation checks pass.
4. The exact supported dialect/version, API profiles, target, and known deviations are represented in the compatibility matrix.
5. Conformance tests run only for fixtures whose source, version, expected behavior, and redistribution rights have been reviewed.
6. Unsupported, unverified, and target-limited behavior is not silently accepted.
7. Packaging includes the runtime components and system-library requirements needed by generated programs.
8. The final artifacts and metadata pass integrity checks before publication.

A skipped test may be accepted only when the workflow has positively established that it is irrelevant to the selected artifact. A missing, cancelled, or unexpectedly skipped required job must fail the release gate.

Do not publish a stable release while the compiler/runtime are absent or while the declared release scope cannot be validated. A research-only or documentation-only state is not a releasable compiler.

## 4. Build environment and target

The initial planned binary target is Linux x86-64, GNU ABI, ELF 64-bit. The build host and output target must be recorded separately.

The release build must use:

- the repository-pinned Rust toolchain;
- a declared LLVM version and Rust binding;
- declared SDL3/system-library versions where the selected runtime needs them;
- locked Cargo dependencies and a clean build directory;
- documented compiler flags and build profile;
- no network access from compiler tests unless a test is explicitly classified as external;
- a controlled environment with recorded OS image and architecture.

Use `cargo build --locked` and the release validation commands defined by the implementation. Do not invent a command for a component that does not yet exist. The exact toolchain and native dependency versions remain open until workspace/bootstrap work is accepted.

## 5. Artifact set and naming

The intended release set is:

- compiler executable for each supported host/target combination;
- runtime libraries or runtime package required by generated executables;
- optional debug-symbol artifacts where the platform toolchain supports separate symbols;
- release notes and compatibility matrix;
- machine-readable build metadata;
- SHA-256 checksums for every published artifact;
- software bill of materials (SBOM) and provenance/attestation when the selected tooling and policy are implemented.

A package name should identify at least the product version, operating system, architecture, and ABI, for example:

`gfabasic-<version>-linux-x86_64-gnu.tar.gz`

This is an example naming convention, not a currently available artifact.

Keep compiler installation separate from generated application assets. Do not bundle copyrighted manuals, ROMs, disk images, or research archives. The package layout, install/uninstall procedure, and runtime deployment model are defined under [issue #59](https://github.com/vsimko/gfabasic-rust/issues/62).

## 6. Build metadata

Each artifact set should include a machine-readable manifest with:

- compiler product version;
- source commit and tag;
- build target triple and build-host identity;
- Rust toolchain version;
- LLVM version and binding;
- SDL3 and other native dependency versions where relevant;
- build profile and feature/configuration flags;
- runtime ABI version, once defined;
- configuration, API registry, and corpus schema versions where relevant;
- compatibility matrix revision;
- reproducibility status and build-environment identifier.

Do not include user-specific paths, secrets, or environment values. A build timestamp may be recorded for operational traceability, but it must not affect the binary when reproducible-build verification is claimed.

## 7. Checksums, SBOM, and provenance

- Generate a SHA-256 checksum for each final artifact after packaging.
- Publish checksums alongside artifacts and verify them in a clean download test.
- Generate an SBOM from the resolved dependency graph and native dependency inventory when tooling is selected.
- Record provenance linking the artifact to the source commit, workflow run, build environment, and toolchain.
- Prefer a GitHub-supported artifact attestation mechanism with narrowly scoped permissions once the repository security policy is accepted.
- Do not claim signed provenance or reproducible builds until the corresponding verification is automated and reviewed.

The exact SBOM format, signing key model, attestation tool, and retention period remain open under issue #39.

## 8. Reproducible-build policy

A build is reproducible only when independent builds from the same source and declared inputs produce byte-identical artifacts, or when every documented non-deterministic field is excluded by a clearly specified comparison procedure.

Before making that claim:

1. Pin all build inputs, including toolchain, LLVM, native libraries, and dependencies.
2. Normalize timestamps and filesystem ordering where supported.
3. Avoid embedding absolute build paths and machine-local data.
4. Record environment variables and compiler flags that affect output.
5. Build the same tag in two clean environments.
6. Compare artifact hashes and investigate every difference.
7. Publish the verification method and result.

Until this test passes, describe the process as a controlled or repeatable build, not a reproducible build.

## 9. Release notes and compatibility claims

Each release must include:

- user-visible changes and migration instructions;
- compiler and runtime versions;
- supported source dialects and exact version subsets;
- API profile status and tested scope;
- output targets and system runtime dependencies;
- known unsupported, partial, unverified, and target-limited features;
- conformance corpus revision and validation result;
- configuration/registry/runtime ABI schema changes;
- artifact checksums and provenance links.

Never use an unqualified “GFA BASIC compatible” claim. A successful build or passing compiler unit tests do not establish source-language or runtime compatibility.

## 10. Rollback and publication failure

If artifact validation or publication fails, do not leave a partially published release presented as complete. Keep failed artifacts private to the workflow run, preserve sanitized logs, and retry only after identifying the cause.

If a published artifact is defective, withdraw or mark the release as superseded, publish corrected checksums and notes, and preserve the original tag/history. Do not silently replace an artifact under the same immutable release identity.

## 11. Implementation sequence

1. Establish the Rust workspace and verified build commands.
2. Implement compiler/runtime packaging and installation layout under #59.
3. Establish dependency and provenance policy under #39.
4. Add release validation to CI only after each check has an executable command.
5. Test artifact naming, metadata, checksums, and clean-install behavior on a release candidate.
6. Configure protected publishing permissions and environment approval.
7. Verify reproducibility independently before making any reproducibility claim.

Until these prerequisites exist, this document is a release design only.
