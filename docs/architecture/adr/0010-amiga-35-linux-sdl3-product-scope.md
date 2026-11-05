# ADR 0010: Product scope — Amiga GFA BASIC 3.5 to Linux x86-64 with SDL3

- Status: Accepted
- Date: 2026-11-05
- Decision makers: Project maintainer
- Tracking: [issues #3, #4, #52, and #53](https://github.com/vsimko/gfabasic-rust/issues)

## Context

The project already selected Amiga GFA BASIC interpreter 3.5 as its normative source dialect (ADR 0009) and Linux x86-64 as its initial output target (ADR 0003). The earlier wording described these as initial choices and left room for multiple dialects, API profiles, and output targets. That open-ended wording creates unnecessary parallel work and obscures the intended product.

The product goal is a compiler for Amiga GFA BASIC 3.5 programs that produces native Linux x86-64 executables. Linux graphics, windowing, and input integration will use SDL3.

## Decision

1. **The sole supported source dialect/version is Commodore Amiga GFA BASIC interpreter 3.5.** This is the product target, not merely the first dialect in a planned matrix.
2. **The sole supported output target is Linux x86-64**, using the GNU ABI and ELF 64-bit format (x86_64-unknown-linux-gnu).
3. **LLVM is the planned native code-generation backend.**
4. **SDL3 is the selected Linux platform integration library** for graphics, windows, and input. SDL3 calls must remain behind project-owned runtime interfaces in the Linux platform adapter.
5. The Amiga GFA BASIC language core, Amiga API compatibility layer, generic runtime contract, LLVM backend, and Linux/SDL3 implementation remain separate architectural responsibilities.
6. The compiler frontend and target-independent IR must not depend on LLVM or SDL3. The generated program must link or load SDL3 only when required by the runtime functionality it uses.
7. Amiga 3.0/3.03 references remain historical evidence. Rules found only there must be confirmed against 3.5 before implementation or conformance claims.
8. Atari ST dialects/API profiles, other Amiga interpreter/compiler versions, other BASIC dialects, other output operating systems/architectures, and cross-compilation are out of scope. Existing research artifacts may remain for historical context but do not create support commitments.
9. Do not implement undocumented or ambiguous behavior by guessing. Unsupported source constructs and unavailable Linux capabilities must produce explicit diagnostics.
10. This scope decision does not claim that the language, LLVM backend, runtime, SDL3 adapter, or end-to-end compiler is already implemented or conformant.

## Alternatives considered

- Keep multiple dialects and platform profiles as near-term goals: rejected because the immediate product is specifically Amiga 3.5 to Linux and parallel targets dilute evidence and implementation effort.
- Implement Atari ST compatibility alongside Amiga: rejected; Atari evidence is not normative for Amiga and Atari support is outside the product scope.
- Use SDL3 directly from the compiler or language core: rejected because it would couple source semantics and IR to one platform library.
- Require SDL3 for every generated executable: rejected; non-graphical programs should not acquire an unnecessary graphics/input dependency.
- Add other output targets or cross-compilation immediately: rejected; the Linux x86-64 path must be completed and validated first.

## Consequences

- Research and implementation issues must prioritize Amiga GFA BASIC 3.5 evidence only.
- The lexer, parser, semantic analysis, typed IR, LLVM backend, runtime ABI, and CLI should be designed to deliver the one defined product target.
- Linux/SDL3 work is the only platform integration work in scope.
- Atari ST research may remain archived, but Atari implementation tasks should be closed as not planned.
- The compatibility matrix and README must not imply a multi-dialect or multi-target product.
- New scope expansion requires a new explicit product decision.

## References

- [ADR 0009: Initial source dialect — Amiga GFA BASIC interpreter 3.5](0009-initial-source-dialect-amiga-gfa-basic-3.5.md)
- [ADR 0003: Host and compilation target model](0003-target-model.md)
- [Project goals](../GOALS.md)
- [Dialect and version policy](../DIALECTS.md)
- [Host and target model](../TARGETS.md)
- [Linux platform and SDL3 integration — issue #52](https://github.com/vsimko/gfabasic-rust/issues/52)
- [Amiga API compatibility runtime — issue #53](https://github.com/vsimko/gfabasic-rust/issues/53)
