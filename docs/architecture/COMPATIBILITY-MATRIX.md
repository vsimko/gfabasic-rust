# Current compatibility matrix

- Status: **Amiga GFA BASIC interpreter 3.5 is the accepted initial normative source dialect. A narrow FACT/PRINT prototype exists; no complete dialect is implemented or conformant.**
- Updated: 2026-11-05.
- Tracking: [dialect policy #3](https://github.com/vsimko/gfabasic-rust/issues/3), [compatibility testing #10](https://github.com/vsimko/gfabasic-rust/issues/10), [versioning policy #58](https://github.com/vsimko/gfabasic-rust/issues/58).

This matrix separates accepted normative source scope from research evidence, compiler implementation, API profile implementation, output target, and conformance status. Selecting a source dialect does not mean that it is implemented or supported.

## Current status

| Evidence target | Policy status | Research evidence | Language implementation | API profile implementation | Output target | Conformance status |
|---|---|---|---|---|---|---|
| Amiga GFA BASIC interpreter 3.5 | **Accepted initial normative source dialect** | Supplied 3.5 Interpreter manual identifies the included software as 3.5 and contains version-specific addenda; the FACT/PRINT example is page-image-verified, but no original interpreter observation exists | Experimental FACT/PRINT frontend subset and temporary C/`cc` native bootstrap only; general language semantics/IR/LLVM are not implemented | Not implemented | One native host executable can be produced by the temporary bootstrap bridge; production Linux x86-64 backend is not implemented | Prototype fixture path only; not full dialect conformance |
| Amiga GFA BASIC interpreter 3.0 / V3.03 | Historical research only; not the selected initial target | Separate 1989 3.0 and 1990 V3.03 book evidence; feature-by-feature applicability to 3.5 must be confirmed | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Historical/inherited candidates only; no 3.5 conformance claim |
| Atari ST GFA BASIC V2.xx/V3.0 | Research-only; not in initial compiler contract | Preliminary source-backed research from the 1988 Atari book and July 1988 user manual; detailed OCR-sensitive rules still need review | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Research only; no compiler acceptance suite |
| Amiga interpreter 3.51 / compiler 3.52 | Unselected; future research | Contemporary 1991 product listing distinguishes interpreter 3.51 and compiler 3.52; catalogue-labelled ADF images have not been inspected | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Not selected; component identity and runtime observations remain incomplete |

## Project-wide implementation status

- Rust workspace: bootstrapped with compiler, generated-program runtime, and Linux platform crates. Compiler source-map, strict UTF-8 transport loading, structured diagnostics, an evidence-gated FACT/PRINT frontend subset, and a temporary C/`cc` native bootstrap are present. General semantic analysis, typed IR, LLVM backend, project configuration, runtime behavior, and platform adapter are not implemented.
- A narrow `build` path exists for the FACT/PRINT prototype only. General GFA BASIC compilation, semantic analysis, project IR, and LLVM backend remain unimplemented; `check` remains unavailable.
- GFA BASIC core runtime: not implemented.
- Amiga and Atari ST compatibility runtimes: not implemented.
- Linux platform layer and SDL3 integration: not implemented.
- CI: documentation validation and fast Rust workspace checks are present; no GFA BASIC conformance runner or compiler/runtime integration workflows exist.
- No dialect/version or API profile is currently claimed as implemented, stable, supported, or conformant. Amiga GFA BASIC interpreter 3.5 is selected as the normative target only.

## Rules for updating the matrix

Add or change a row only when the evidence and implementation state are explicit.

1. Name the exact upstream dialect, version, and platform. Do not merge interpreter and compiler versions.
2. Identify the source documents and evidence status. Separate manual-backed claims from interpreter/compiler observations.
3. Name the selected API profile set independently from the source dialect.
4. Name the output target independently from the host machine and source platform.
5. Report implementation status separately for language core, API profile, runtime, and target.
6. Link conformance tests and state whether they are candidates, verified fixtures, or executed tests.
7. List known deviations and blocked prerequisites.
8. Do not use “supported” or “stable” until the exact scope is implemented and the relevant tests pass.

The planned Linux x86-64 target is not evidence of a working backend. An Amiga or Atari ST API profile implemented for Linux would not make the output binary a native Amiga or Atari executable.

The matrix must be reviewed with the release notes and updated whenever a compiler release changes the supported subset, runtime, API profiles, or target capabilities.
