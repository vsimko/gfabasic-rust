# API profiles

- Status: **Accepted by [ADR 0004](adr/0004-api-profile-model.md).** Individual declarations and behavior remain unverified.

## Separation

The **language core** contains constructs that do not depend on a platform API. It is part of the selected source dialect/version; it is not an API profile.

Platform APIs are selectable profiles. The output target is independent of the selected source API profile.

The initial profile identifiers are:

- \`amiga\` — Amiga-specific API declarations and behavior.
- \`atari-st\` — Atari ST-specific API declarations and behavior.

There is no separate \`core\` or \`common\` API profile in the initial model. A construct belongs to the language core only when evidence establishes that it is a language feature rather than a platform API.

Profile identifiers are stable conceptual names. Applicable platform, GFA BASIC version, OS/library version, and declaration availability are metadata on each profile/declaration; they are not inferred from the profile name.

## Selection and target independence

- The selected source dialect/version is explicit and separate from API profiles.
- Users explicitly select zero or more API profiles.
- The output target is selected independently. Selecting \`amiga\` does not request an Amiga executable; it requests Amiga API semantics, which may be implemented by a compatibility runtime on Linux.
- No profile is enabled implicitly from the output target.
- Explicit configuration takes precedence over any future advisory detection.
- Automatic detection, if added, is advisory unless the user explicitly selects an inference mode.

Illustrative configuration fields (not a frozen CLI or file schema):

\`\`\`toml
# Illustrative only; identifiers and schema are defined separately.
dialect = "<explicit source dialect and version>"
api_profiles = ["amiga"]
target = "x86_64-unknown-linux-gnu"
\`\`\`

## Conflicts and dependencies

- Profiles must be independently selectable.
- Profile dependencies must be declared and evidence-backed.
- Conflicting declarations across enabled profiles must produce deterministic diagnostics. Never choose silently by load order or precedence.
- A profile must not silently change language-core semantics.
- The initial model does not define an implicit fallback profile or automatic conflict resolution.

## Declaration lifecycle

An API declaration is not supported merely because it is present in a registry. Before exposure as a supported declaration, it must identify:

- source dialect/version and platform;
- applicable API profile and OS/library version where relevant;
- primary reference, edition, page/section, and evidence status;
- syntax/signature and semantic contract;
- runtime mapping and target support status;
- conformance/regression tests and documentation reference.

Research-only, disputed, unknown, and unimplemented declarations must remain visibly classified and must not be accepted as supported behavior.

## Future profiles

A future platform or library profile may be added without changing the meaning of the language core, existing profile identifiers, or output target. It requires a source-backed profile definition, conflict/dependency rules, runtime capability classification, and tests before it can be marked supported.

## Deferred details

The exact CLI/configuration serialization, version-range syntax, registry schema, duplicate-name diagnostics, runtime capability encoding, and individual Amiga/Atari declaration inventories are tracked under the CLI, registry, and language/API research issues. This ADR accepts the profile model only; it does not accept any specific API declaration as implemented or conformant.

Tracking: [issue #5](https://github.com/vsimko/gfabasic-rust/issues/5), [registry #12](https://github.com/vsimko/gfabasic-rust/issues/12), [CLI #9](https://github.com/vsimko/gfabasic-rust/issues/9).
