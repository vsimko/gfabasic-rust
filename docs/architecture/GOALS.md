# Project goals and compatibility objectives

- Status: Proposed baseline; review under issue #2.

## Purpose

Build a general-purpose compiler and runtime for documented GFA BASIC dialects. Implement the compiler in Rust. The compiler must provide deterministic behavior and useful diagnostics.

## Primary direction

- Produce native executables. Generated Rust source is not the primary output.
- Use LLVM as the planned native code-generation infrastructure.
- Start with Linux x86-64, GNU ABI, and ELF 64-bit output.
- Use SDL3 behind project-owned interfaces for initial Linux graphics/window/input integration where required.
- Keep language semantics, API profiles, output targets, and host configuration separate.
- Permit future targets and API profiles without redesigning the frontend.
- Keep compilation independent of LLM services.

These are planning assumptions. They do not establish that the compiler, backend, or runtime exists.

## Compatibility dimensions

Do not use an unqualified “GFA BASIC compatible” claim. Report compatibility separately for source syntax, static semantics, language runtime, platform API, program behavior, and binary/toolchain behavior. Every claim must name its dialect, version, profile, and evidence status.

## Initial non-goals

- Implementing every historic dialect or release.
- Silently mixing Amiga and Atari ST API behavior.
- Emulating every original operating-system or hardware feature.
- Making compatibility claims without primary-source evidence.
- Requiring an LLM or network access to compile a program.
- Special-casing a downstream application.
- Treating generated Rust as the required output format.

Unsupported or ambiguous constructs must produce explicit diagnostics. They must not be ignored or silently assigned semantics from another BASIC dialect.

## Open decisions

First supported dialect/version; compatibility baseline for compiler versus interpreter behavior; acceptable source/runtime deviations; scope of optional future transpilation; runtime distribution and ABI stability.

See [issue #2](https://github.com/vsimko/gfabasic-rust/issues/2) and [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3).
