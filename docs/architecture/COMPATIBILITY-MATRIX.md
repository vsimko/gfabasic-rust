# Current compatibility matrix

- Status: **Current project snapshot; no GFA BASIC dialect/version is accepted or implemented.**
- Updated: 2026-11-04.
- Tracking: [dialect policy #3](https://github.com/vsimko/gfabasic-rust/issues/3), [compatibility testing #10](https://github.com/vsimko/gfabasic-rust/issues/10), [versioning policy #58](https://github.com/vsimko/gfabasic-rust/issues/58).

This matrix separates research evidence from compiler implementation, API profile implementation, output target, and conformance status. A source listed here is not a supported dialect.

## Current status

| Evidence target | Research evidence | Language implementation | API profile implementation | Output target | Conformance status |
|---|---|---|---|---|---|
| Atari ST GFA BASIC V2.xx/V3.0 | Preliminary source-backed research from the 1988 Atari book and July 1988 user manual; detailed OCR-sensitive rules still need review | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Research only; no compiler acceptance suite |
| Amiga GFA BASIC interpreter 3.0 | French 1989 Amiga book identifies interpreter 3.0; research notes contain source-backed findings with remaining page-image checks | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Research only; two candidate corpus fixtures are not verified |
| Amiga interpreter/compiler 3.51/3.52 | Contemporary 1991 product listing distinguishes interpreter 3.51 and compiler 3.52; catalogue-labelled ADF images have not been inspected | Not implemented | Not implemented | Linux x86-64 is planned, not implemented | Blocked; component identity and runtime observations are unavailable |

## Project-wide implementation status

- Rust workspace: bootstrapped with compiler, runtime, and Linux platform crates; compiler source-map and strict UTF-8 file-loading primitives are implemented, while the language frontend, semantic analysis, IR, backend, CLI, runtime behavior, and platform adapter are not implemented.
- Compiler frontend, semantic analysis, IR, LLVM backend, and CLI: not implemented.
- GFA BASIC core runtime: not implemented.
- Amiga and Atari ST compatibility runtimes: not implemented.
- Linux platform layer and SDL3 integration: not implemented.
- CI: documentation validation and fast Rust workspace checks are present; no GFA BASIC conformance runner or compiler/runtime integration workflows exist.
- No dialect/version or API profile is currently claimed as accepted, stable, supported, or conformant.

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
