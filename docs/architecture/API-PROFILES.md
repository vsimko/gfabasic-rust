# API profiles

- Status: **The sole product API profile is Amiga GFA BASIC interpreter 3.5.** Product scope is fixed by [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md). Individual declarations and runtime behavior remain evidence-gated.

## Separation

The **language core** contains constructs that do not depend on a platform API. It belongs to Amiga GFA BASIC interpreter 3.5 and is not an API profile.

The product includes one platform API compatibility layer:

- `amiga` — Amiga GFA BASIC 3.5 API declarations and behavior, implemented by a compatibility runtime on Linux where evidence and feasibility permit.

The Linux output target does not make the program an Amiga executable. It means the generated Linux program may reproduce verified Amiga API behavior through project-owned runtime interfaces.

Atari ST API declarations may remain in archival research or registry material for provenance, but Atari ST is not a product profile, must not be selectable for product builds, and must not be marked supported. Other API profiles are out of scope.

## Selection and target

- The source dialect is fixed to Amiga GFA BASIC interpreter 3.5.
- The product API profile is fixed to Amiga.
- The output target is fixed to Linux x86-64, GNU ABI, ELF 64-bit.
- Configuration must reject unsupported dialects, versions, profiles, and target triples with actionable diagnostics.
- Do not infer Amiga API semantics from Linux/SDL3 behavior; SDL3 is only the platform implementation mechanism.

Illustrative configuration:

```toml
[source]
dialect = "amiga-gfa-basic"
version = "3.5"

[api]
profiles = ["amiga"]

[target]
triple = "x86_64-unknown-linux-gnu"
```

The identifier spelling above is illustrative until the compiler configuration schema is implemented. It does not authorize alternate values.

## Declaration lifecycle

An API declaration is not supported merely because it is present in a registry. Before exposure as supported, it must identify:

- Amiga GFA BASIC interpreter 3.5 applicability;
- source reference, edition, page/section, and evidence status;
- syntax/signature and semantic contract;
- Linux runtime mapping and capability status;
- conformance/regression tests and documentation reference.

Research-only, disputed, unknown, and unimplemented declarations must remain visibly classified and must not be accepted as supported behavior.

## Runtime boundary

Amiga API semantics belong in project-owned runtime modules. Linux/SDL3 is an implementation adapter behind the runtime contract. The compiler frontend and target-independent IR must not depend on SDL3. Unsupported Amiga hardware/OS behavior must be classified and diagnosed rather than guessed.

## Deferred details

The exact CLI/configuration serialization, registry schema, runtime capability encoding, and individual Amiga declaration inventory remain tracked under the CLI, registry, and language/API research issues. This document does not accept any specific API declaration as implemented or conformant.

Tracking: [issue #5](https://github.com/vsimko/gfabasic-rust/issues/5), [registry #12](https://github.com/vsimko/gfabasic-rust/issues/12), [CLI #9](https://github.com/vsimko/gfabasic-rust/issues/9), and [Amiga runtime #53](https://github.com/vsimko/gfabasic-rust/issues/53).
