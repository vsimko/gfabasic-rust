---
type: "Architecture"
title: "Memory model and hardware access"
---

# Memory model and hardware access

- Status: **Approved safety baseline (2026-11-04); dialect-specific memory semantics remain subject to evidence-based research.**
- Tracking: [issue #8](https://github.com/vsimko/gfabasic-rust/issues/8), dependent on [dialect/version selection #3](https://github.com/vsimko/gfabasic-rust/issues/3) and the language inventory #19–#23.

This document sets a conservative implementation boundary. It does not claim that any particular PEEK/POKE, BYTE/WORD, address, or memory command exists in a supported dialect.

## 1. Separate source values from host pointers

A GFA BASIC integer, address value, pointer, and Rust pointer are different concepts unless a verified dialect rule explicitly relates them.

- Never infer pointer width from a BASIC integer width.
- Never reinterpret an arbitrary source integer as a valid Linux process pointer.
- Never expose a Rust reference or raw pointer as a source-language value.
- Represent unresolved source addresses as explicit opaque/address values in the semantic model until their meaning is known.
- Define target pointer width and address-space identity independently from source-language numeric types.

A 16-bit or 32-bit address from an Atari ST or Amiga program does not identify the same memory in a Linux process.

## 2. Ownership and lifetime

Runtime-managed strings, arrays, buffers, and resources require explicit ownership and lifetime rules.

- Every allocation must have one documented owner or a defined shared-ownership model.
- Borrowed views must not outlive their backing allocation.
- Resizing or releasing an allocation must define what happens to existing views.
- Array indexing and slicing must define bounds behavior.
- Resource handles must not be treated as memory addresses.
- Generated code must not retain pointers into movable or replaceable storage unless the runtime contract guarantees stability.

The exact source-language lifetime/aliasing rules remain unknown until the selected manual and interpreter behavior are reviewed.

## 3. Raw memory and hardware access policy

Any verified raw-memory operation must be classified before implementation:

1. **Equivalent implementation** — the target has a documented, semantically equivalent operation.
2. **Bounded emulation** — a project-owned memory region or data structure can reproduce the required observable behavior without exposing arbitrary host memory.
3. **Explicit unsafe/privileged extension** — only if separately designed, opt-in, isolated, and documented with a clear threat model. This is not part of the default compatibility baseline.
4. **Unsupported** — the operation depends on original hardware, OS memory maps, or privileged state that cannot be reproduced safely.

The approved Linux policy is **unsupported by default** for direct access to hardware registers and arbitrary memory, including unbounded process-memory reads/writes. Physical addresses from Amiga or Atari programs must never be interpreted as pointers into the Linux process. Never silently map a source address to a host pointer.

## 4. Bounds, alignment, and representation

Emulation is permitted only for an explicitly defined, bounded region and only when its observable behavior is supported by evidence. Before implementation, define all of the following:

- address-space size and valid regions;
- byte order and word/longword representation;
- alignment requirements;
- access width and signedness;
- out-of-range behavior;
- overlap and aliasing rules;
- initialization and lifetime of memory regions;
- synchronization requirements if multiple execution contexts can access the region.

Do not select little-endian or host-native representation by default. Original platform byte order and data layouts must be established from the relevant source/version and target evidence.

## 5. Compiler and runtime enforcement

- Semantic analysis must distinguish an address value from an ordinary integer when the dialect does.
- IR must represent address calculation, loads, stores, and allocation explicitly.
- The IR verifier must check types and address-space boundaries.
- The backend must not synthesize raw pointer operations for unresolved semantics.
- Privileged/unsafe memory-access modes are out of scope for the approved baseline.
- Unsupported or unsafe memory/hardware operations must produce a source-located diagnostic before executable-program creation/emission.
- No runtime check may convert an invalid access into an apparently successful result.

## 6. Required research before language support

For every memory-related construct, record exact dialect/version, syntax, operand types, width, signedness, address space, side effects, alignment, error behavior, and source page/section. Separate language-core memory operations from Amiga/Atari OS and hardware APIs. OCR-sensitive symbols require page-image review.

## 7. Migration and compatibility reporting

A future compatibility report may identify memory operations that are unsupported on Linux and show their source locations. It must not rewrite the program or guess a safe replacement. Any migration suggestion is advisory and separate from deterministic compilation.

## Open questions

- Which address and pointer constructs are actually part of each selected dialect?
- What storage and lifetime rules apply to strings and arrays?
- Which original memory regions are observable to ordinary GFA BASIC programs?
- Which operations can be bounded and emulated without changing behavior?
- Hardware-dependent access remains unsupported unless a separately approved, bounded emulation can reproduce its behavior.

These questions cannot be closed until a normative dialect/version is selected and its source evidence is reviewed.
