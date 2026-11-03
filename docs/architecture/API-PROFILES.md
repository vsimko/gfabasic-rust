# API profiles

- Status: Proposed baseline; exact declarations and behavior remain unverified.

## Separation

The language core contains constructs that do not depend on a platform API. Platform APIs are selectable profiles. The output target is independent of the selected source API profile.

Planned profile concepts are `core`, `amiga`, and `atari-st`. Exact identifiers remain provisional until CLI/configuration design is accepted.

## Selection and conflicts

- Explicit user configuration takes precedence.
- Profiles must be independently selectable.
- No profile is enabled implicitly from the output target.
- Conflicting declarations across enabled profiles must produce deterministic diagnostics. Never choose silently.
- Automatic detection, if added, is advisory unless the user selects an inference mode.
- A Linux target may implement documented Amiga or Atari ST API semantics through a compatibility runtime. It does not produce a native executable for the original platform.

## Declaration lifecycle

An API declaration is not supported merely because it is present in a registry. It must identify its source dialect/version, reference, semantic contract, runtime mapping, target support status, and tests.

Open decisions include profile versions/dependencies, common API versus language core, configuration representation, duplicate-name policy, signature compatibility, and capability reporting.

Tracking: [issue #5](https://github.com/vsimko/gfabasic-rust/issues/5).
