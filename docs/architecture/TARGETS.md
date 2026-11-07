# Host and target model

- Status: **Sole supported output target: Linux x86-64, GNU ABI, ELF 64-bit.** See [ADR 0003](adr/0003-target-model.md) and [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md).

The **host** is the environment that runs the compiler. The **target** is the environment for which the compiler generates an executable. The product target is fixed to Linux x86-64; cross-compilation is out of scope.

## Sole output target

- Operating system: Linux
- Architecture: x86-64
- LLVM target triple: x86_64-unknown-linux-gnu
- ABI: GNU
- Executable format: ELF 64-bit

This is the only supported output target. It is a scope decision, not a claim that code generation or runtime support is complete.

## Target descriptor

Represent at minimum the operating system, CPU architecture, ABI/calling convention, object format, pointer width and endianness where required, runtime availability, and supported capabilities. Use the LLVM target triple as the canonical code-generation target identity. Wrap it in a project-owned target descriptor that records runtime capabilities and packaging requirements not represented by the triple.

## Platform integration

SDL3 is the selected Linux platform library for graphics, windowing, and input. SDL3 calls must remain behind project-owned runtime interfaces in the Linux platform adapter. The compiler frontend and target-independent IR must not depend on SDL3. Generated programs should link or load SDL3 only when they use runtime functionality that requires it.

## Boundary requirements

- Shared frontend and IR must not assume Linux, x86-64, pointer width, endianness, or a specific ABI.
- Target-specific lowering belongs after the platform-independent IR boundary.
- Unsupported target or runtime capabilities must produce deterministic diagnostics.
- Runtime libraries and their ABI must be selected for the Linux target.
- Do not add another output OS, architecture, ABI, or cross-compilation path without an explicit product-scope decision.

Tracking: [issue #4](https://github.com/vsimko/gfabasic-rust/issues/4).
