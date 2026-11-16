---
type: "Architecture"
title: "Project goals and compatibility objectives"
---

# Project goals and compatibility objectives

- Status: **Accepted product scope**; see [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md).

## Purpose

Build a deterministic compiler and runtime in Rust for **Commodore Amiga GFA BASIC interpreter 3.5**, producing native **Linux x86-64** executables. The project is not a universal BASIC compiler and does not target multiple GFA BASIC dialects.

## Sole supported product target

- Source language: Amiga GFA BASIC interpreter 3.5 only.
- Output: Linux x86-64, GNU ABI, ELF 64-bit.
- Native code generation: LLVM.
- Linux graphics, windowing, and input: SDL3 behind project-owned runtime interfaces.
- The compiler must be deterministic and must not require an LLM or network access.

SDL3 is a platform adapter, not part of language semantics, the lexer, parser, or target-independent IR. Programs that do not use SDL-backed platform functionality should not acquire an unnecessary SDL3 runtime dependency.

## Compatibility dimensions

Do not use an unqualified “GFA BASIC compatible” claim. Report syntax, static semantics, language runtime, Amiga API behavior, observable program behavior, and Linux binary/toolchain behavior separately. Every claim must identify Amiga GFA BASIC 3.5, evidence status, implementation status, and known deviations.

The Amiga 3.5 interpreter manual is the primary normative source. Rules found only in Amiga 3.0/3.03 material are inherited candidates and must be confirmed for 3.5 before implementation or conformance claims.

## Explicit non-goals

- Atari ST GFA BASIC, including Atari ST 3.0 and 3.5.
- Amiga GFA BASIC versions other than interpreter 3.5 as supported source dialects.
- Other BASIC dialects.
- Windows, macOS, BSD, other Linux architectures, non-Linux output, and cross-compilation.
- Additional graphics or platform frameworks in place of SDL3.
- Reproducing Amiga hardware or operating-system behavior that cannot be represented or reasonably emulated on Linux.
- Implementing every historical API regardless of evidence or feasibility.
- Requiring an LLM or network service in the compilation path.
- Generated Rust as the primary output.

Historical manuals and Atari research may remain in the repository as archival context. They do not create support commitments or authorize implementation of another dialect/profile. Unsupported, ambiguous, or unavailable constructs must produce explicit diagnostics; they must never be silently ignored or assigned semantics from another BASIC dialect.

## Architecture constraints

- Keep Amiga GFA BASIC 3.5 language semantics independent of Linux and SDL3.
- Keep target-independent IR free of OS, ABI, and SDL3 details.
- Put Amiga API compatibility behavior behind project-owned runtime interfaces.
- Put SDL3 calls only in the Linux platform adapter.
- Select and link only runtime components required by a generated program.
- Keep LLVM behind the backend boundary.

Tracking: [issue #2](https://github.com/vsimko/gfabasic-rust/issues/2), [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3), and [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md).
