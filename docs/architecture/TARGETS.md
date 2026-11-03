# Host and target model

- Status: Proposed baseline; decisions remain open under issue #4.

The **host** is the environment that runs the compiler. The **target** is the environment for which the compiler generates an executable. They can differ.

## Initial target proposal

- Operating system: Linux
- Architecture: x86-64
- ABI: GNU
- Executable format: ELF 64-bit

This is the planned initial output target, not the compiler host and not a claim that code generation exists.

## Target descriptor

Represent at minimum the operating system, CPU architecture, ABI/calling convention, object format, pointer width and endianness where required, runtime availability, and supported capabilities. Evaluate LLVM target triples and whether a project-owned descriptor is also needed for runtime capabilities.

## Boundary requirements

- Shared frontend and IR must not assume Linux, x86-64, pointer width, endianness, or a specific ABI.
- Target-specific lowering belongs after the platform-independent IR boundary.
- Unsupported target features must produce deterministic diagnostics.
- Runtime libraries and their ABI must be selected for the target.
- Cross-compilation is a future capability, not an initial implementation requirement.

Tracking: [issue #4](https://github.com/vsimko/gfabasic-rust/issues/4).
