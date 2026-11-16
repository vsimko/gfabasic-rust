---
type: "User Guide"
title: "User documentation"
---

# User documentation

**Audience:** People evaluating or using the planned GFA BASIC compiler.

**Status:** User documentation is limited because only a narrow experimental source subset can be formatted or built. The CLI supports help/version, selected-subset `fmt`, and a combinatorics/PRINT prototype `build`; `check` and general compilation are unavailable.

## Current status

The `gfabasic` executable supports help/version, `fmt` for the evidence-gated combinatorics/PRINT and selected MAT syntax, and `build` for literal FACT/VARIAT/COMBIN assignments followed by PRINT. The build uses a temporary C/`cc` bootstrap and is not a general compiler. `check` remains unavailable. There is no installation package or generally supported GFA BASIC dialect/API profile. Do not treat the prototype as full interpreter compatibility or follow build commands copied from another GFA BASIC implementation.

Start with the [project README](../../README.md) for the current project direction and limitations. Use the [compatibility matrix](../architecture/COMPATIBILITY-MATRIX.md) for the current support status.

The compiler frontend and formatter produce source-located diagnostics for unsupported or malformed syntax; the LSP publishes parser diagnostics for open documents. The `check` CLI command and a general machine-readable diagnostic/report interface remain unavailable. See [Understanding compiler diagnostics](diagnostics.md) for the current boundary.

## Research is not a support claim

The [language and API reference notes](../reference/README.md) record source-backed research and unresolved questions. Their presence does not mean that a feature is implemented, tested, or supported by this project. Amiga and Atari ST material is kept separate.

## Planned user guides

The documentation index records where future guides belong and which implementation prerequisites they require: [planned user and contributor pages](../index.md).

Installation, quick-start, CLI, and troubleshooting instructions will be added only after they can be verified against a reproducible build. See [the project documentation index](../README.md).
