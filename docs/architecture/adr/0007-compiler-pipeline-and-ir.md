# ADR 0007: Compiler pipeline and typed IR boundaries

- Status: Accepted
- Date: 2026-11-04
- Decision makers: Project maintainers

## Context

The compiler needs a stable boundary between dialect-specific frontend semantics and target-specific code generation. The initial output target is Linux x86-64 and LLVM is the planned backend, but the frontend and intermediate representation must not depend on Linux, SDL3, or LLVM implementation types.

The dialect/version research is not complete. The compiler therefore needs an architecture that can preserve unresolved source distinctions and prevent an unverified construct from acquiring guessed semantics. The runtime ABI and detailed memory model are separate decisions.

## Decision

1. Use explicit compiler stages: source loading, lexing, parsing, name resolution, semantic analysis, typed project IR lowering, target/API lowering, backend code generation, object generation/linking, and artifact reporting.
2. Keep source syntax and source spans in the AST, and retain a lossless token/trivia stream or CST alongside semantic syntax. Preserve original spelling and trivia through source spans; resolve dialect-specific meaning during semantic analysis. Do not delegate language semantics to LLVM.
3. Make the project-owned IR typed, target-independent, and explicitly structured around basic blocks and terminators. Represent conversions, calls, memory operations, and source provenance explicitly.
4. Require IR verification before backend code generation. Invalid semantic or IR states must produce diagnostics and must not produce an executable.
5. Keep LLVM IR and LLVM-specific types inside the backend. The frontend and project IR must build and test without LLVM or SDL3.
6. Preserve source provenance through lowering and optimization so backend and runtime diagnostics can be mapped to original source locations.
7. Keep backend selection and target-specific lowering separate from source dialect and API-profile selection.
8. Do not select an LLVM version or Rust binding yet. Before adding one, evaluate LLVM/Rust compatibility, maintenance cadence, unsafe FFI surface, native build reproducibility, platform support, license, and security-update policy. Record the selected versions and binding in a follow-up ADR after a clean CI build and a verified IR-to-object prototype.

The exact GFA BASIC type inventory, source control-flow mapping, pointer semantics, and runtime-call ABI remain governed by the dialect, memory, and runtime decisions. This ADR defines the compiler boundary; it does not guess those semantics.

## Alternatives considered

### Lower source AST directly to LLVM IR

This would couple language semantics to LLVM types and APIs, make frontend tests depend on LLVM, and make another backend difficult to add. Rejected.

### Use LLVM IR as the canonical project IR

LLVM IR is target/code-generation oriented and does not express the project's source provenance, compatibility classifications, or profile-level runtime operations as a stable project-owned contract. Rejected.

### Create a separate IR for every target

This would duplicate semantic lowering and make it harder to compare behavior across targets. Use one target-independent project IR and explicit target/API lowering instead. Rejected.

### Select an LLVM binding during workspace bootstrap

No verified LLVM version, native package source, or clean-build strategy has been evaluated. Selecting a binding now would add an unvalidated native dependency to the workspace. Deferred.

## Consequences

- The frontend and IR can be developed and tested without LLVM or SDL3.
- Lossless token/trivia data remains available to formatting and editor tooling; the semantic AST is not required to carry formatting trivia itself.
- The backend consumes a verified semantic model rather than defining language behavior.
- Additional backends can be added without redesigning source parsing and semantic analysis.
- IR verification and source-provenance preservation become required compiler contracts.
- The LLVM binding and exact release remain open until dependency and CI evaluation is complete.
- Dialect-specific types, GOTO/GOSUB behavior, pointer semantics, and runtime ABI must be resolved by their respective evidence-backed decisions before implementation.

## Evidence and references

- [Compiler pipeline and IR specification](../PIPELINE-AND-IR.md).
- [Dialect/version policy](../DIALECTS.md), [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3).
- [Memory model](../MEMORY.md), [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8).
- [Runtime architecture](../RUNTIME.md), [issue #7](https://github.com/vsimko/gfabasic-rust/issues/7).
- [Workspace boundaries](../WORKSPACE.md), [ADR 0002](0002-initial-rust-workspace.md).
- [LLVM backend implementation](https://github.com/vsimko/gfabasic-rust/issues/50).

## Supersedes / superseded by

None.
