# ADR 0002: Versioning and compatibility lifecycle

- Status: Accepted
- Date: 2026-11-04
- Decision makers: Project maintainers

## Context

The compiler will expose several independently evolving compatibility surfaces: the compiler executable, upstream GFA BASIC dialects, API profiles, project configuration, API registry, conformance corpus, generated-program runtime ABI, and output targets. A single version number cannot safely describe all of these surfaces.

The project also needs a release lifecycle that communicates experimental behavior, stable support scope, breaking changes, and deprecation without implying compatibility with every historical GFA BASIC release.

## Decision

1. Use Semantic Versioning 2.0.0 for public compiler product releases.
   - Before 1.0.0, use \`0.y.z\`; breaking changes may occur during development and must still be documented.
   - After 1.0.0, follow SemVer compatibility rules for public compiler interfaces.
   - Do not declare 1.0.0 until the supported language subset, toolchain, compatibility matrix, and release validation are implemented and repeatable.
2. Version GFA BASIC dialects and releases independently from the compiler product. Preserve upstream platform/version identity; do not collapse Amiga and Atari ST releases into one assumed dialect.
3. Version project configuration, API registry, conformance corpus, and other machine-readable schemas independently. A compiler release number must not stand in for a schema version.
4. Version the generated-program runtime ABI independently from the compiler executable. Before a stable ABI exists, treat generated artifacts and runtime libraries as a matched build.
5. Report feature maturity, source evidence, implementation state, and target support as separate dimensions. A source reference or registry entry does not prove implementation or compatibility.
6. Permit a **Stable** support claim only for the exact documented and tested dialect/version, API profile, and target scope.
7. Require release notes and migration guidance for breaking changes. After 1.0.0, deprecate public interfaces before removal where practical; do not deprecate upstream language constructs merely because they are old.
8. Publish the compatibility matrix with releases and never use an unqualified “GFA BASIC compatible” claim.

The normative detail and examples are maintained in [VERSIONING.md](../VERSIONING.md). Implementation of the CLI version report, schema validators/migrations, runtime ABI metadata, and release workflows remains separate work.

## Alternatives considered

- One version number for compiler, schemas, runtime, and language support: rejected because independent changes would make compatibility claims ambiguous.
- Treating compiler version as an upstream GFA BASIC version: rejected because the compiler product and source dialect have different release histories.
- Claiming stable compatibility from documentation or registry presence alone: rejected because implementation and conformance evidence are separate requirements.
- Allowing silent interpretation of unknown future schema versions: rejected because it can change build behavior without an actionable error.

## Consequences

- Release metadata must report the compiler version and the relevant dialect/profile/target/schema/runtime identities separately.
- The compatibility matrix becomes part of the release contract.
- The project must maintain migration notes when public interfaces change.
- The runtime ABI and machine-readable schema mechanisms still require implementation before stable compatibility guarantees can be made.
- This ADR accepts the policy, not the existence or maturity of any compiler/runtime feature.

## Evidence and references

- [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html)
- [Versioning and compatibility lifecycle](../VERSIONING.md)
- [Compatibility matrix](../COMPATIBILITY-MATRIX.md)
- [Issue #58](https://github.com/vsimko/gfabasic-rust/issues/58)
