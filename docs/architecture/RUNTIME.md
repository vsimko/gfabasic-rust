# Runtime architecture

- Status: Approved architectural baseline (2026-11-04); language-specific contracts remain evidence-gated.
- Implementation status: source-backed Rust core primitives and typed handler resolution exist; no stable generated-program ABI is claimed.

## Layers and dependency direction

1. **GFA BASIC core runtime** — language services such as strings, arrays, numeric conversions, file operations, and error/status handling.
2. **API compatibility runtime** — independently selected Amiga and Atari ST profile behavior.
3. **Platform-service contract** — project-owned interfaces for display, input, timing, files/devices, and process integration.
4. **Target adapter** — Linux-specific implementation; SDL3 is an adapter, not a definition of GFA BASIC behavior.

The runtime must not depend on the compiler or LLVM. The compiler frontend and generic IR must not depend on SDL3.

## Runtime handler IDs

Registry `implementation.handler` values are stable identifiers, not Rust paths. Core handlers use `gfa.runtime.core.<area>.<operation>`. A Rust refactor does not rename the registry contract. Profile handlers must be owned by their profile and cannot be attached to language-core declarations.

Each implemented/partial declaration must resolve to exactly one registration. Resolution validates missing, duplicate, stale, namespace-incompatible, profile-incompatible, and signature-incompatible entries. Unsupported, planned, research-only, and not-implemented declarations receive no fallback handler.

The initial registrations are:

| Declaration | Handler ID | Signature |
| --- | --- | --- |
| `gfa.language.math.fact` | `gfa.runtime.core.math.fact` | integer → exact unsigned integer |
| `gfa.language.math.variat` | `gfa.runtime.core.math.variat` | integer × integer → exact unsigned integer |
| `gfa.language.math.combin` | `gfa.runtime.core.math.combin` | integer × integer → exact unsigned integer |

The Amiga GFA BASIC 3.5 addendum (printed pp. 13-33–13-34) defines FACT(0)=1, VARIAT(n,k)=n!/(n-k)!, and COMBIN(n,k)=n!/((n-k)!k!). Its example gives FACT(6)=720, VARIAT(6,2)=30, and COMBIN(6,2)=15. The companion German book, *Das große GFA-BASIC-Buch zum Amiga*, printed pp. 477–479, identifies these as 3.5 additions and documents n >= k for VARIAT/COMBIN.

The Rust boundary accepts signed 64-bit arguments, checks conversion to u32, and returns exact u128 results. Invalid domains and u128 overflow return a Rust error. These are explicit Rust API choices, not claims about the interpreter's numeric range, representation, overflow behavior, or error numbers. Reference-interpreter conformance remains untested.

## Runtime ABI boundary

Generated code must call project-owned runtime symbols only; it must not call SDL3, GEM, AmigaOS, or other platform APIs directly. Before publishing a stable generated-code ABI, define its version, calling convention, parameter/return representation, ownership, lifetime, error propagation, initialization, shutdown, and thread-safety rules. A Rust public API is not an accidental generated-code ABI.

## Data representation and conversion

Do not assume Rust types, host pointer width, SDL formats, or Linux file APIs match historical GFA BASIC representations. Keep source numeric values, strings, arrays, original-platform addresses/layouts, profile records, and target-native buffers distinct until evidence establishes conversions. Define overflow, truncation, byte order, alignment, ownership, and failure behavior explicitly.

## Resource lifecycle and errors

Resource creation returns a valid opaque handle or structured failure. Closing defines repeated-close behavior. Initialization and shutdown handle partial failure. Runtime errors are propagated, never silently converted to success. Exact GFA BASIC resource semantics must come from source evidence or reference observations.

## Graphics, input, and headless testing

Graphics state and original data formats belong to the compatibility runtime. SDL3 is only a Linux presentation/input adapter. Keep logical dimensions, palette/drawing state, original pixel data, host presentation buffers, drawing, timing, and event ordering separate. Core tests must run without SDL3; hardware-dependent tests must be isolated and a skip is not conformance.

## Capability reporting

Every operation is classified per target/profile as supported, partial, adapter-backed, emulated, unsupported, unsafe-disabled, or unknown. A successful link does not establish semantic compatibility. Unsupported operations fail explicitly.

Tracking: [runtime architecture #7](https://github.com/vsimko/gfabasic-rust/issues/7), [memory model #8](https://github.com/vsimko/gfabasic-rust/issues/8), [Amiga API #24](https://github.com/vsimko/gfabasic-rust/issues/24), [Atari ST API #25](https://github.com/vsimko/gfabasic-rust/issues/25), and [Linux platform adapter #52](https://github.com/vsimko/gfabasic-rust/issues/52).
