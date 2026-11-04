# ADR 0004: API profile model

- Status: Accepted
- Date: 2026-11-04

## Context

GFA BASIC language semantics and platform APIs must not be conflated. Amiga and Atari ST implementations share historical lineage, but their platform APIs and version-specific behavior require separate evidence. The output target is also independent: a Linux executable may implement a documented legacy platform API through a compatibility runtime without becoming a native executable for that platform.

## Decision

1. Treat the selected source dialect/version and the language core as distinct from platform API profiles.
2. The initial platform API profile identifiers are \`amiga\` and \`atari-st\`.
3. Do not define a separate \`core\` or \`common\` API profile in the initial model. Platform-independent language constructs belong to the selected dialect's language core.
4. Select zero or more API profiles explicitly. Do not infer or enable a profile from the output target.
5. Keep profile selection independent of the output target. The initial Linux target may implement selected Amiga or Atari ST API semantics through project-owned runtime interfaces.
6. Store platform/version/OS-library applicability on profile and declaration metadata rather than encoding an assumed version range into the profile identifier.
7. Conflicting declarations across enabled profiles must produce deterministic diagnostics. Never resolve conflicts silently by load order or implicit precedence.
8. A registry entry is not an implementation or support claim. Supported declarations require source evidence, semantic contracts, runtime mappings, target status, and conformance tests.
9. Automatic profile detection, if added, is advisory unless a user explicitly selects an inference mode.

The detailed policy is maintained in [API-PROFILES.md](../API-PROFILES.md). CLI/configuration syntax and registry serialization remain separate decisions.

## Alternatives considered

- Treat language core as a \`core\` API profile: rejected because it conflates language semantics with selectable platform APIs.
- Add an implicit \`common\` API profile: rejected until a source-backed set of platform-independent API declarations is identified.
- Infer Amiga/Atari API profile from Linux/Amiga output target: rejected because source API semantics and output platform are independent.
- Resolve duplicate declarations by profile load order: rejected because it makes compilation order-dependent and hides ambiguity.
- Encode all version information in profile identifiers: rejected because OS/library versions and GFA interpreter/compiler versions may evolve independently.

## Consequences

- The language core remains independent of platform API declarations.
- Amiga and Atari ST profiles can evolve independently and can be selected together only when conflicts are resolved explicitly.
- CLI and registry work must preserve explicit dialect/profile/target separation.
- The accepted model does not claim that any Amiga or Atari API declaration is complete, implemented, or conformant.
- Additional profiles require evidence, dependency/conflict rules, runtime capability status, and tests.

## References

- [API profile specification](../API-PROFILES.md)
- [Target model](../TARGETS.md)
- [Versioning policy](../VERSIONING.md)
- [Issue #5](https://github.com/vsimko/gfabasic-rust/issues/5)
