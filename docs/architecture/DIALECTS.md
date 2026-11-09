# Dialects, versions, and compatibility policy

- Status: **Current practical reference implementation: Commodore Amiga GFA BASIC interpreter 3.52.** The product's historical 3.5 scope and the current 3.52 working baseline are distinct. See [COMPATIBILITY-EVIDENCE.md](COMPATIBILITY-EVIDENCE.md).

## Current compatibility baseline

The compiler's immediate compatibility goal is to accept and implement the largest practical set of programs and language features that the available Amiga GFA BASIC 3.52 interpreter can interpret. Use the existing Amiberry workflow for repeatable reference tests.

This is a project working assumption. It does not claim that interpreter 3.5 and 3.52 are historically identical. A direct comparison with an independently identified 3.5 interpreter is a later validation phase and must not block compiler development.

## Source and version boundaries

Keep interpreter and compiler products/version numbers distinct. Record the exact executable identity and environment for every runtime observation. Do not infer the interpreter version from an archive label alone.

Books and manuals are supporting evidence. They can describe older releases, omit details, or contain errors. Record exact title, edition, platform, printed page, and section. Rules found only in earlier 3.0/3.03 sources remain historical candidates unless confirmed by runtime evidence or a version-specific source.

Atari ST GFA BASIC V2.xx/V3.0 and Atari ST 3.5 are not supported targets. Do not use Atari evidence to fill gaps in Amiga behavior.

## Required feature record

Each feature should include a stable identifier, platform and interpreter version, syntax, observed semantics and edge cases, source citation, evidence status, implementation status, conformance tests, and known deviations. See [COMPATIBILITY-EVIDENCE.md](COMPATIBILITY-EVIDENCE.md).

## Evidence rules

- Observed behavior in the 3.52 interpreter is the primary compatibility oracle for current development.
- Record the exact executable identity, environment, input program, and observable result for runtime tests.
- Historical programs that run in the reference interpreter are valuable fixtures; record provenance and rights.
- Books and manuals help discover features and investigate failures. They do not override a contrary runtime observation.
- Disassembly/decompilation is a separate diagnostic evidence class. Record binary identity and relevant address, symbol, or context.
- Mark claims as observed, documented, inferred, disputed, or unknown.
- Preserve disagreements between sources. Do not silently reconcile them.
- Do not transfer rules from Atari ST or another BASIC dialect to Amiga.

## Diagnostics and tests

Unknown or unsupported syntax must not be skipped. Diagnostics must identify the source location and explain whether the construct is invalid, unsupported, ambiguous, or unavailable in the selected runtime contract.

When a compiler test fails or crashes, reduce it to a minimal reproducer, run it in the reference interpreter, compare observable behavior, consult relevant documentation, and inspect binary-analysis evidence if useful. Fix the implementation or document a justified limitation. Add a regression test for each reproducible defect.

Do not claim compiler support because a manual describes a feature or because a registry entry exists. Evidence, implementation, and test status are separate.

Tracking: [issue #46](https://github.com/vsimko/gfabasic-rust/issues/46), [reference validation issue #66](https://github.com/vsimko/gfabasic-rust/issues/66).
