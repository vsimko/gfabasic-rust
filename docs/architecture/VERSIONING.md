# Versioning and compatibility lifecycle

- Status: **Policy accepted by [ADR 0010](adr/0010-versioning-and-compatibility-lifecycle.md); CLI/schema/ABI mechanisms are not implemented yet.**
- Tracking: [issue #58](https://github.com/vsimko/gfabasic-rust/issues/58).
- Scope: compiler product releases, machine-readable schemas, language dialects, API profiles, runtime ABI, target support, and deprecation.

## 1. Keep version domains separate

A single version number must not describe all compatibility dimensions. Record these independently:

| Domain | What it versions | Rule |
|---|---|---|
| Compiler product | The compiler executable and its release artifacts | Proposed Semantic Versioning 2.0.0 |
| GFA BASIC dialect | The upstream language variant and release being compiled | Preserve the exact source/platform/version identity; do not derive it from the compiler version |
| API profile | The platform API declaration and behavior set selected for a compilation | Identify independently from the language dialect and output target |
| Project configuration schema | The structure and meaning of project configuration files | Give the format its own explicit schema version |
| API registry schema | The machine-readable declaration format | Give the registry format its own explicit schema version |
| Conformance corpus schema | The manifest and fixture metadata contract | Version independently; the current corpus manifest uses its own integer schema version |
| Runtime ABI | Calling convention, data layout, symbols, and runtime contracts used by generated programs | Version independently from the compiler and registry |
| Output target | OS, CPU, ABI, object/executable format, and runtime capabilities | Report as a target descriptor, not as a language or compiler version |

The compiler version does not prove that a dialect or API profile is supported. The presence of a registry declaration or test fixture does not prove implementation or conformance.

## 2. Compiler product version

Use Semantic Versioning 2.0.0 for public compiler releases.

- Before the first stable release, use `0.y.z`.
- During the `0.y` series, minor releases may contain breaking changes. Patch releases should be limited to compatible fixes where practical.
- Declare `1.0.0` only after the project has an implemented, bounded dialect subset, a usable toolchain, published compatibility matrix, and repeatable release validation.
- After `1.0.0`, increment the major version for incompatible public compiler interfaces or supported project-file behavior; increment the minor version for backward-compatible capabilities; increment the patch version for backward-compatible fixes.
- Pre-release identifiers may be used for test builds. Do not treat a pre-release as a stable compatibility promise.

The release number identifies the compiler product only. It does not imply compatibility with every release of GFA BASIC.

## 3. Dialect and API profile identity

A compilation must identify its source dialect and version explicitly. Preserve upstream names and version labels in documentation and diagnostics. Do not silently normalize distinct releases into one version range.

For example, Amiga GFA BASIC 3.0, an Amiga interpreter build labelled 3.51, an Amiga compiler labelled 3.52, and Atari ST GFA BASIC V2.xx/V3.0 are distinct evidence targets. Their relationship must be established by source and compatibility tests, not by the compiler product version.

API profiles are selected independently from the source dialect and output target. Profile metadata must state the applicable platform/version range and evidence status. A Linux executable implementing an Amiga API profile is still a Linux-targeted executable; it is not an Amiga binary.

The exact stable identifiers and selection syntax remain open under the dialect, API profile, and CLI issues.

## 4. Machine-readable schema versions

Configuration, API registry, corpus manifest, and other machine-readable formats must each declare their own schema version.

- Do not reuse the compiler release number as a schema version.
- A schema change must document whether old files remain readable.
- Reject unsupported future schema versions with an actionable diagnostic. Do not silently reinterpret fields.
- Provide an explicit migration path when a schema change requires file updates.
- Keep generated reference documentation tied to the schema version that produced it, where generation is introduced.

The current corpus manifest's `schema_version = 1` applies only to the test-corpus manifest contract. It does not select the future project configuration or API registry schema version.

Exact serialization formats and migration tooling remain open under the CLI/configuration and API registry issues.

## 5. Runtime ABI versioning

Generated programs may depend on a project-owned runtime. The runtime contract must be versioned separately from the compiler executable.

The runtime ABI definition must cover at least:

- symbol naming and availability;
- calling convention and parameter/return layout;
- integer, floating-point, string, array, pointer, and aggregate representation;
- ownership and lifetime rules;
- error propagation;
- initialization and shutdown;
- platform/API adapter boundaries.

Before a stable runtime ABI is declared, treat generated artifacts and runtime libraries as belonging to a matched compiler/runtime build. A successful link is not proof of semantic compatibility.

Any incompatible runtime ABI change must be visible in release metadata and documented migration notes. The exact ABI encoding and compatibility mechanism remain open until runtime interfaces are designed.

## 6. Compatibility and support reporting

Every release should publish the [compatibility matrix](COMPATIBILITY-MATRIX.md) that identifies:

- compiler product version and source revision;
- supported dialects and exact version subsets;
- selected API profiles and their version scope;
- output targets and required runtime components;
- conformance corpus revision and test status;
- known deviations, unsupported constructs, and unresolved behavior;
- project configuration, registry, corpus, and runtime ABI schema versions.

Use separate status values for feature maturity, evidence, implementation, and target support.

Feature-maturity labels:

- **Research**: evidence is being collected; no accepted specification or implementation is implied.
- **Proposed**: a specification or design is written but has not been accepted.
- **Experimental**: an implementation exists, but the behavior or interface may change and must not be presented as stable.
- **Stable**: the exact behavior is documented, implemented, and covered by relevant tests for the stated dialect/version/profile/target. The compatibility matrix identifies the tested scope and known deviations.
- **Deprecated**: a previously stable feature remains available but has a documented replacement and earliest removal release.
- **Removed**: the feature is no longer available in the stated compiler release.

Evidence status (for example, verified, inferred, disputed, or unknown) describes the quality of the source or observation. Implementation status describes whether code exists. Target support describes whether the implementation is native, emulated, partial, unsupported, or unknown. None of these dimensions can substitute for another. A source-backed feature may be documented but unimplemented; an implemented feature may be experimental; a stable feature may be supported on one target and unsupported on another.

The user-facing meaning of support outcomes and their relationship to diagnostics are defined in [DIAGNOSTICS.md](DIAGNOSTICS.md). Do not treat a support outcome as a replacement for feature maturity or evidence status.

Never use an unqualified “GFA BASIC compatible” claim. State the exact dialect/version/profile/target and the evidence level.

## 7. Deprecation and breaking changes

Before `1.0.0`, breaking changes may be necessary while the language subset, configuration, registry, and runtime contracts are still being established. Each such change must still include:

- a changelog entry;
- affected interfaces and versions;
- migration instructions where existing user data or source projects are affected;
- the reason for the change.

After `1.0.0`, public compiler/configuration interfaces should follow the normal Semantic Versioning compatibility rules. Deprecate an interface before removing it, identify its replacement, emit a clear diagnostic or warning where feasible, and document the earliest release in which removal may occur. Security, legal, or severe correctness issues may require an exceptional removal; explain the exception in release notes.

Do not deprecate a GFA BASIC language construct merely because it is old or inconvenient. Language availability is determined by the selected dialect/version and the compatibility evidence, not by the compiler's own release lifecycle.

## 8. Release notes and version reporting

Each public release should include a changelog entry covering user-visible changes, compatibility effects, new/removed support, known deviations, and required migration steps.

When the CLI exists, its version report should identify at least the compiler version and build revision. A diagnostic/support report should also expose the selected dialect/version, API profiles, output target, and relevant runtime/schema versions. Avoid machine-local paths and timestamps in reproducibility metadata unless explicitly requested.

Reference: [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

No release command, version-reporting CLI, schema migration, or runtime ABI mechanism exists yet. This document defines a proposed policy, not implemented behavior.
