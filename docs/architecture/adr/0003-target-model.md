# ADR 0003: Host and compilation target model

- Status: Accepted
- Date: 2026-11-04

## Context

The compiler runs on a build host and generates executables for a compilation target. These environments may differ. The initial output target is Linux x86-64 with the GNU ABI and ELF 64-bit format.

LLVM needs a target identity for code generation. Runtime capability reporting, ABI requirements, and package dependencies also need metadata that is not fully represented by a code-generation triple.

## Decision

1. Treat the build host and compilation target as separate concepts. Host discovery must not silently choose or override the requested target.
2. Use the LLVM target triple as the canonical code-generation target identity.
3. Define a project-owned target descriptor around the triple. The descriptor records OS, CPU architecture, ABI/calling convention, object format, pointer width, endianness, runtime ABI requirements, system dependencies, and supported runtime capabilities as applicable.
4. The initial target is \`x86_64-unknown-linux-gnu\`: Linux, x86-64, GNU ABI, ELF 64-bit.
5. Keep the frontend and project-owned IR independent of the target triple, host pointer width, endianness, OS APIs, and ABI. Target-specific lowering and code generation occur after the platform-independent IR boundary.
6. The target descriptor selects the matching runtime libraries and reports unsupported target capabilities. Unsupported operations must produce deterministic diagnostics.
7. Linux x86-64 is the sole product output target under [ADR 0011](0011-amiga-35-linux-sdl3-product-scope.md). Cross-compilation and additional target triples are out of scope unless the product decision is explicitly revised.

## Alternatives considered

- Use the LLVM target triple as the only target model: rejected because it does not fully describe project runtime capabilities, package dependencies, or support classifications.
- Use only a project-owned target name and synthesize LLVM triples later: rejected because it duplicates target identity and creates avoidable mapping ambiguity.
- Let the build host determine the target by default without an explicit target model: rejected because native and cross compilation must have unambiguous semantics.

## Consequences

- Target identity is interoperable with LLVM while runtime capability metadata remains project-owned.
- Build configuration must distinguish host and target.
- Runtime selection and capability diagnostics must consume the project-owned descriptor.
- The sole target is precisely named, but no compiler/backend/runtime implementation is implied.
- Any future proposal for another target requires a new product-scope decision; this ADR does not commit the project to additional targets.

## References

- [Target model specification](../TARGETS.md)
- [Project goals](../GOALS.md)
- [Compiler pipeline and IR](../PIPELINE-AND-IR.md)
- [Issue #4](https://github.com/vsimko/gfabasic-rust/issues/4)
