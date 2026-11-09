---
type: User Guide
title: User documentation
tags: [user, usage]
---

# User documentation

**Audience:** People evaluating or using the planned GFA BASIC compiler.

**Status:** User documentation is limited because source compilation and runtime behavior are not implemented. The minimal CLI supports help and version output only; `build` and `check` do not compile source.

## Current status

The repository includes a minimal `gfabasic` executable, but it supports only help and version output. The `build` and `check` commands explicitly report that source compilation is unavailable. There is no installation package or supported GFA BASIC dialect/API profile. Do not attempt to follow build commands copied from another GFA BASIC implementation.

Start with the [project README](../../README.md) for the current project direction and limitations. Use the [compatibility matrix](../architecture/COMPATIBILITY-MATRIX.md) for the current support status.

The compiler library includes a source-map diagnostic model and human-readable renderer, but no GFA BASIC frontend or CLI command currently emits compiler diagnostics. See [Understanding compiler diagnostics](diagnostics.md) for the proposed user-facing vocabulary.

## Research is not a support claim

The [language and API reference notes](../reference/README.md) record source-backed research and unresolved questions. Their presence does not mean that a feature is implemented, tested, or supported by this project. Amiga and Atari ST material is kept separate.

## Planned user guides

The documentation index records where future guides belong and which implementation prerequisites they require: [planned user and contributor pages](../README.md#planned-user-and-contributor-pages).

Installation, quick-start, CLI, and troubleshooting instructions will be added only after they can be verified against a reproducible build. See [the project documentation index](../README.md).
