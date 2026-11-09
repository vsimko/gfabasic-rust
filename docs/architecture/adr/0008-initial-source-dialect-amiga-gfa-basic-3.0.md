---
type: "Architecture Decision Record"
title: "ADR 0008: Initial source dialect — Amiga GFA BASIC interpreter 3.0"
---

# ADR 0008: Initial source dialect — Amiga GFA BASIC interpreter 3.0

- Status: Superseded
- Date: 2026-11-04
- Decision owner: Repository maintainer
- Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3)
- Superseded by: [ADR 0009](0009-initial-source-dialect-amiga-gfa-basic-3.5.md)

## Context

The project needs one bounded, normative source-language target before dialect-specific lexer, parser, semantic, and conformance work can be treated as implementation requirements. Research covers both Amiga and Atari ST GFA BASIC, but their evidence and platform APIs must not be conflated.

The Amiga research identifies GFA BASIC interpreter 3.0 in a French 1989 Amiga reference book. Atari ST GFA BASIC V2.xx/V3.0 has separate research evidence and a separately identified July 1988 primary manual. Similar constructs across the platforms do not establish identical syntax or semantics.

## Decision

1. The first normative source-language target is **Amiga GFA BASIC interpreter 3.0**.
2. The Atari ST GFA BASIC V2.xx/V3.0 material remains **research-only**. It is not part of the initial compiler compatibility contract and must not be used to fill gaps in the Amiga specification.
3. Amiga and Atari ST language dialects and platform API profiles remain separate dimensions. The selected source dialect does not implicitly select or implement an API profile.
4. The compiler must identify the selected source dialect and version explicitly. The exact CLI/configuration spelling and defaults remain governed by the CLI/configuration specification; no implicit dialect inference from the output target is permitted.
5. This decision selects the normative research and implementation target; it does **not** claim that the compiler, runtime, API profile, or any GFA BASIC feature is already implemented or conformant.
6. Where Amiga 3.0 evidence is absent, ambiguous, or OCR-sensitive, behavior remains unresolved until verified against the Amiga source or compatibility evidence. Atari-only evidence is not a substitute.

## Alternatives considered

- **Atari ST GFA BASIC V3.0 first:** rejected for the initial target. Its research is useful and remains available, but the maintainer selected the Amiga platform.
- **Both platforms from the beginning:** rejected to keep the first language contract bounded and avoid silently conflating dialect-specific behavior.
- **Defer all dialect selection:** rejected because it prevents downstream work from having a normative source target.

## Consequences

- Dialect-specific language research and the first conformance inventory should prioritize Amiga GFA BASIC interpreter 3.0.
- Amiga manual claims must retain exact source/page references and evidence status.
- Atari research can continue independently, but remains non-normative for the first compiler target.
- API profile selection, runtime behavior, and output target remain independent decisions.
- The compatibility matrix must distinguish selected normative dialect from implemented and conformant status.
- Issue #3 remains open until its remaining evidence, specification, explicit-selection, and diagnostic acceptance criteria are completed.

## Evidence

- Amiga GFA BASIC 3.0 research is recorded in [docs/reference/amiga-gfa-basic-3.0.md](../../reference/amiga-gfa-basic-3.0.md); the research identifies a French 1989 Amiga book and records page-image checks still outstanding.
- The reference source register is [docs/reference-sources.md](../../reference-sources.md).
- Atari ST primary-source research is separately recorded in [docs/reference/atari-gfa-basic-3.0.md](../../reference/atari-gfa-basic-3.0.md) and must not be transferred to Amiga.
- Maintainer decision recorded in issue #3 on 2026-11-04.
