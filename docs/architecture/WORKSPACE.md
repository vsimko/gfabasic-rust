# Rust workspace and dependency policy

- Status: Proposed; exact crate boundaries remain open under issue #11.

## Logical components

Source manager/diagnostics; lexer/parser/AST; semantic analysis/symbol tables; project-owned typed IR; LLVM backend; CLI/configuration; core generated-program runtime; Amiga compatibility runtime; Atari ST compatibility runtime; Linux platform layer/SDL3 adapter.

These responsibilities do not require one Cargo crate each. Split crates only when ownership, reuse, build isolation, independent testing, or dependency constraints justify it.

## Dependency direction

- Source management and frontend must not depend on LLVM or SDL3.
- Semantic analysis owns language typing and profile resolution.
- Project IR must not depend on platform integration.
- LLVM backend depends on project IR, not the reverse.
- Runtime code is separate from compiler implementation dependencies.
- SDL3 is restricted to the Linux platform adapter.
- CLI orchestrates stages and configuration; it does not own semantics.

Do not use Cargo features as a substitute for per-compilation API profile selection. Cargo features select compiled capabilities; compiler configuration selects source dialect/profile. Keep LLVM/SDL3 out of builds that do not need them where practical.

Pin Rust toolchain and components when the workspace is bootstrapped. Decide LLVM and SDL3 dependency strategies in an ADR after compatibility testing. Frontend and IR must build/test without LLVM/SDL3 installed.

Tracking: [issue #11](https://github.com/vsimko/gfabasic-rust/issues/11).
