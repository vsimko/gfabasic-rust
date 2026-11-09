---
type: Compatibility Policy
title: GFA BASIC compatibility baseline
description: Defines the current reference interpreter and the later comparison with interpreter 3.5.
tags: [compatibility, amiga, gfa-basic, versioning]
sources:
  - resource: ../../docs/architecture/COMPATIBILITY-EVIDENCE.md
    title: Compatibility evidence and agent workflow
  - resource: ../../docs/architecture/DIALECTS.md
    title: Dialect and version policy
---

# Current baseline

Use the Commodore Amiga GFA BASIC 3.52 interpreter as the practical behavioral reference for current compiler development. The immediate goal is to accept and implement the largest practical set of programs and language features that this interpreter can interpret.

Use the repository's existing Amiberry workflow for repeatable reference tests. Do not create a parallel emulator workflow without a specific technical need.

# Version boundary

The 3.52 baseline is a working project decision. It does not claim that interpreter 3.5 and 3.52 are historically identical. A comparison with an independently identified 3.5 interpreter is a later validation phase and must not block current compiler work.

Record the exact executable identity and environment for every observation. Do not infer the interpreter version from an archive filename or catalogue label.

# Further reading

- [Full compatibility evidence policy](../../docs/architecture/COMPATIBILITY-EVIDENCE.md)
- [Dialect policy](../../docs/architecture/DIALECTS.md)
- [Reference source register](../../docs/reference-sources.md)
