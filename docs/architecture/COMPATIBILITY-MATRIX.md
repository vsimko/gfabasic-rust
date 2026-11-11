---
type: "Architecture"
title: "Current compatibility matrix"
---

# Current compatibility matrix

- Status: **The sole product target is Amiga GFA BASIC interpreter 3.5 to Linux x86-64. A narrow FACT/VARIAT/COMBIN/PRINT prototype exists; selected MAT forms are parsed/formatted but not executable. No complete dialect is implemented or conformant.**
- Updated: 2026-11-11. Reviewed against the current [CLI implementation](CLI-AND-CONFIG.md), [formatter scope](../reference/amiga-gfa-basic-35-formatter.md), [runtime boundary](RUNTIME.md), [API registry](../../api/registry/README.md), and [conformance-corpus status](TESTING.md).
- Tracking: [dialect policy #3](https://github.com/vsimko/gfabasic-rust/issues/3), [compatibility testing #10](https://github.com/vsimko/gfabasic-rust/issues/10), [versioning policy #58](https://github.com/vsimko/gfabasic-rust/issues/58), [product scope ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md).

This matrix separates the sole product target from historical research evidence, compiler implementation, Amiga API implementation, Linux/SDL3 implementation, and conformance status. A selected target does not mean that it is implemented or supported.

## Product target

| Dimension | Sole product target | Current implementation |
|---|---|---|
| Source dialect/version | Commodore Amiga GFA BASIC interpreter 3.5 | Experimental FACT/VARIAT/COMBIN assignments and PRINT; selected MAT syntax is parser/formatter-only |
| Historical source evidence | 3.5 Interpreter manual and version-specific addenda; 3.0/3.03 references are inherited candidates requiring 3.5 confirmation | Page-image-verified 3.5 combinatorics fixtures; no original-interpreter runtime observation |
| API profile | Amiga GFA BASIC 3.5 API only | Not implemented |
| Output OS/architecture | Linux x86-64 | Temporary host C/cc bootstrap only; production backend not implemented |
| ABI/object format | GNU ABI, ELF 64-bit; LLVM triple x86_64-unknown-linux-gnu | Not implemented by production backend |
| Native backend | LLVM | Not implemented |
| Graphics/window/input | SDL3 behind Linux platform adapter | Not implemented |
| Conformance | Amiga 3.5 source-backed fixtures and original-interpreter observations where legally available | Narrow prototype tests only; not full dialect conformance |

## Historical and excluded research

| Material | Status | Permitted use |
|---|---|---|
| Amiga GFA BASIC 3.0 / V3.03 | Historical research only; not a supported source version | Inherited candidates only, after confirmation against Amiga 3.5 |
| Atari ST GFA BASIC V2.xx/V3.0 and Atari ST 3.5 | Archival research only; outside product scope | Provenance/context only; never used to define Amiga behavior or product builds |
| Amiga interpreter 3.51 / compiler 3.52 | Other product versions; outside product scope | Historical catalogue/source notes only; no support or compatibility commitment |

## Project-wide implementation status

- Rust workspace: bootstrapped with compiler, generated-program runtime, and Linux platform crates. Compiler source maps, strict UTF-8 transport loading, structured diagnostics, a narrow FACT/VARIAT/COMBIN/PRINT frontend, selected MAT parsing/formatting, combinatorics literal lowering through typed runtime handlers, and a temporary C/cc native bootstrap are present.
- General lexer/parser, semantic analysis, typed IR, LLVM backend, project configuration, GFA BASIC core runtime, Amiga API compatibility runtime, and Linux/SDL3 platform adapter are not implemented.
- A narrow build path exists for the combinatorics/PRINT literal prototype only. Selected MAT statements are not lowered to executable IR. General Amiga 3.5 compilation is not available; check remains unavailable.
- CI: documentation validation and fast Rust workspace checks are present; no full GFA BASIC conformance runner or production compiler/runtime integration workflow exists.
- No complete Amiga 3.5 language subset or Amiga API profile is currently claimed as fully supported or conformant.

## Rules for updating the matrix

1. Name the exact Amiga interpreter version and applicable source evidence.
2. Distinguish historical 3.0/3.03 evidence from confirmed 3.5 behavior.
3. Report implementation status separately for language core, Amiga API, runtime, backend, and Linux/SDL3 adapter.
4. The only product target is Linux x86-64; do not add alternate dialects, profiles, or targets without a new product-scope decision.
5. Link conformance tests and state whether they are candidate, verified, or executed tests.
6. List known deviations and blocked prerequisites.
7. Do not use “supported” or “stable” until the exact feature scope is implemented and relevant tests pass.
