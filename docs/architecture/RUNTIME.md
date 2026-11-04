# Runtime architecture

- Status: Proposed baseline; language-specific contracts remain open under [issue #7](https://github.com/vsimko/gfabasic-rust/issues/7).
- Implementation status: no generated-program runtime behavior or runtime ABI is implemented.

This document defines boundaries and minimum interface properties. It does not define GFA BASIC semantics, source data layouts, or a stable ABI.

## Layers and dependency direction

1. **GFA BASIC core runtime** — language services that cannot be lowered directly to target instructions, such as strings, arrays, numeric conversions, file operations, and error/status handling.
2. **API compatibility runtime** — independently selected Amiga and Atari ST profile behavior. A profile must be based on its own source/version evidence.
3. **Platform-service contract** — project-owned interfaces for display, input, timing, files/devices, and process integration where the source profile requires them.
4. **Target adapter** — Linux-specific implementation of platform services. SDL3 is a candidate backend for window, input, and presentation services, not a definition of GFA BASIC behavior.

Dependency direction:

```text
generated program -> project runtime ABI -> core runtime
                                  |-------> selected API profile
                                  |-------> platform-service contract
                                                   ^
                                                   |
                                    Linux platform adapter (SDL3)
```

The runtime must not depend on the compiler or LLVM. The compiler frontend and generic IR must not depend on SDL3. The Linux adapter may depend on the runtime's platform-service contract; the runtime must not depend on the Linux adapter.

## Runtime ABI boundary

Generated code calls project-owned runtime symbols only. It must not call SDL3, GEM, AmigaOS, or other platform APIs directly.

Before implementation, each ABI operation must define:

- stable symbol identity and ABI version;
- calling convention and parameter/return representation;
- ownership and lifetime of strings, arrays, buffers, and handles;
- error/status propagation and which failures terminate an operation;
- initialization and shutdown requirements;
- thread-safety and reentrancy requirements where applicable;
- target/profile availability and support classification.

Do not use a Rust public API as an accidental generated-code ABI. The generated-program interface must be explicit and testable independently from Rust crate internals. The ABI version must be independent of the compiler product version and registry schema.

## Data representation and conversion

The runtime must preserve the source dialect's documented semantics at the compatibility boundary. It must not assume that a Rust type, host pointer width, SDL surface format, or Linux file API is equivalent to an historical GFA BASIC representation.

Keep these representations distinct until the relevant dialect research establishes the contract:

- source-language numeric and Boolean values;
- strings and arrays;
- original-platform addresses and memory layouts;
- profile-specific records and graphics buffers;
- target-native buffers and SDL presentation formats.

Conversions must be explicit operations with defined overflow, truncation, byte order, alignment, ownership, and failure behavior. Unknown representation details remain unresolved; do not infer them from OCR alone.

## Resource lifecycle and errors

Platform resources (screens, windows, input devices, files, audio devices, and similar objects) require explicit ownership and deterministic release.

- Creation returns either a valid opaque handle or a structured failure.
- A handle is valid only for the documented lifetime and selected profile/target.
- Closing/releasing a resource must define repeated-close and failure behavior.
- Initialization and shutdown must be ordered and safe on partial initialization failure.
- Runtime errors must be propagated through the project-owned ABI; they must not be silently swallowed or converted into success.
- Process termination must release resources owned by the runtime where the host permits it.

The exact resource semantics for GFA BASIC commands must be derived from the selected manuals and reference observations, not from this generic policy.

## Graphics, input, and headless testing

Graphics state and original data formats belong to the compatibility runtime. SDL3 is only a Linux presentation/input adapter.

The design must keep separate:
- logical screen/window dimensions and pixel formats;
- palette and drawing state;
- planar/bitplane or indexed data and host presentation buffers;
- drawing operations and presentation timing;
- input event collection and source-language polling/event semantics.

A software conversion/emulation layer may be required where an original API does not map directly to SDL3. The adapter must not silently change coordinate systems, palette indices, draw modes, or event ordering.

Core runtime tests must run without SDL3. Platform adapter tests should support a headless CI mode where practical. Tests that require a display/device must be explicitly marked and isolated; a skipped hardware test is not a passing conformance result.

## Capability reporting

Every registry/runtime operation must be classified per target and API profile as supported, partial, adapter-backed, emulated, unsupported, unsafe-disabled, or unknown. A successful link does not establish semantic compatibility. Unsupported operations must fail explicitly.

## Deferred decisions

The following require further source evidence or implementation evaluation:

- exact ABI symbols, calling convention, and version compatibility;
- source-specific strings, arrays, numeric formats, and ownership;
- Amiga/Atari screen, window, graphics, and event semantics;
- which operations can be faithfully emulated on Linux;
- SDL3 version/binding and clean-build strategy;
- static/dynamic runtime linking and deployment.

Tracking: [runtime architecture #7](https://github.com/vsimko/gfabasic-rust/issues/7), [memory model #8](https://github.com/vsimko/gfabasic-rust/issues/8), [Amiga API #24](https://github.com/vsimko/gfabasic-rust/issues/24), [Atari ST API #25](https://github.com/vsimko/gfabasic-rust/issues/25), and [Linux platform adapter #52](https://github.com/vsimko/gfabasic-rust/issues/52).
