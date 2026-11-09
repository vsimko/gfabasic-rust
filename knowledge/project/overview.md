---
type: Project Overview
title: gfabasic-rust project overview
description: Purpose, target platform, current maturity, and compatibility direction of the Rust GFA BASIC compiler.
tags: [project, overview, compiler, status]
sources:
  - resource: ../../README.md
    title: Repository README
  - resource: ../../ROADMAP.md
    title: Project roadmap
---

# Purpose

Build a deterministic compiler and runtime in Rust for programs accepted by the Commodore Amiga GFA BASIC interpreter. The practical development baseline is interpreter 3.52. The planned output is native Linux x86-64 (ELF, GNU ABI).

# Current maturity

The repository contains an experimental compiler frontend and a narrow tested subset. It is not a complete GFA BASIC compiler and does not claim full interpreter compatibility. Read the [project roadmap](../../ROADMAP.md) for the current implementation state and limitations.

# Architecture

LLVM is the planned native code-generation backend. SDL3 is the planned Linux platform integration layer for graphics, windows, and input. Compilation must remain deterministic and must not require an LLM or network access.

# Key links

- [Repository README](../../README.md)
- [Project roadmap](../../ROADMAP.md)
- [Architecture overview](../../docs/architecture/README.md)
- [Project goals](../../docs/architecture/GOALS.md)
