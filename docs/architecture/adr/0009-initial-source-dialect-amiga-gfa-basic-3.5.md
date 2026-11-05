# ADR 0009: Initial source dialect — Amiga GFA BASIC interpreter 3.5

- Status: Accepted
- Date: 2026-11-04
- Decision makers: Project maintainer
- Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3)
- Supersedes: [ADR 0008](0008-initial-source-dialect-amiga-gfa-basic-3.0.md)

## Context

The project needs one bounded, normative source-language target before dialect-specific lexer, parser, semantic, and conformance work can be treated as implementation requirements. ADR 0008 initially selected Amiga GFA BASIC interpreter 3.0 using the 1989 Amiga reference book available at that time.

Subsequent source acquisition identified a separate Amiga GFA BASIC 3.5 Interpreter manual. Its introduction identifies the included software as GFA BASIC 3.5 and explains that the manual was originally printed for version 3.0 with a 3.5 addendum. The addendum and further-new-commands chapter provide version-specific 3.5 evidence. The repository maintainer explicitly selected Amiga GFA BASIC 3.5 as the normative target in the 2026-11-04 decision recorded on issue #65.

Historical Amiga 3.0 and 3.03 books remain useful, but they do not independently establish that every documented rule is unchanged in 3.5. Interpreter and compiler product versions must also remain distinct: contemporary evidence lists interpreter 3.51 and compiler 3.52 as separate products.

## Decision

1. The first normative source-language target is **Commodore Amiga GFA BASIC interpreter 3.5**.
2. Use the Amiga GFA BASIC 3.5 Interpreter manual, including its version-specific addenda, as the primary normative source for 3.5-specific syntax and behavior.
3. A rule documented only in a 3.0 or 3.03 source is an **inherited candidate**, not automatically a verified 3.5 rule. Confirm its applicability to 3.5 before making it an implementation or conformance requirement.
4. Keep Amiga and Atari ST dialects separate. Atari ST GFA BASIC V2.xx/V3.0 and Atari ST 3.5 material remain research-only for this initial target; do not use them to fill gaps in Amiga evidence.
5. Keep language dialect/version, platform API profile, build host, and output target as separate configuration dimensions. Selecting the Amiga source dialect does not implicitly select or implement an Amiga API profile.
6. Identify the selected source dialect and version explicitly. Do not infer it from the output target, file extension, or source contents.
7. Distinguish the Amiga 3.5 interpreter from later interpreter/compiler versions and from archive-catalogue labels. Do not treat an uninspected 3.51/3.52-labelled disk image as proof of a particular executable component version.
8. This decision selects the normative research and implementation target; it does **not** claim that the compiler, runtime, API profile, or any GFA BASIC feature is implemented or conformant.
9. Where 3.5 evidence is absent, ambiguous, OCR-sensitive, or contradicted by a 3.0/3.03 source, behavior remains unresolved until checked against the applicable 3.5 source or reproducible compatibility evidence.

## Alternatives considered

- **Amiga GFA BASIC interpreter 3.0 first:** superseded. It was the initial bounded target under ADR 0008, but the maintainer subsequently selected 3.5 after the version-specific interpreter manual became available.
- **Amiga GFA BASIC interpreter 3.51 or 3.52:** not selected. Contemporary commercial evidence distinguishes interpreter 3.51 from compiler 3.52, while archive catalogue labels do not establish the component version of the actual disk contents.
- **Amiga and Atari ST together:** rejected for the initial target to avoid silently conflating platform-specific syntax and semantics.
- **Defer all dialect selection:** rejected because downstream research and implementation need one normative source target.

## Consequences

- Dialect-specific research, lexer/parser requirements, and the first conformance inventory must prioritize Amiga GFA BASIC interpreter 3.5.
- The 1989 Amiga 3.0 book and 1990 Amiga 3.03 reference remain historical and supplementary sources. Each inherited rule needs explicit 3.5 confirmation.
- The 3.5 interpreter manual and its addenda must be indexed with exact printed/PDF page references; OCR-sensitive syntax must be checked against page images. The initial lexer-specific evidence inventory is [Amiga GFA BASIC 3.5 lexer evidence](../../research/amiga-gfa-basic-35-lexer-evidence.md); it separates the verified Chapter 14 special names from inherited 3.0/V3.03 candidates and unresolved source-format rules.
- Atari ST evidence remains non-normative for the initial compiler target.
- API profile selection, runtime behavior, and output target remain independent decisions.
- The compatibility matrix must distinguish the selected 3.5 target from historical 3.0/3.03 research, implementation status, and conformance status.
- Issue #3 remains open until feature-level evidence, the compatibility policy, explicit dialect selection, and diagnostic acceptance criteria are complete.

## Evidence and references

- The maintainer's explicit 3.5 target decision is recorded in [issue #65](https://github.com/vsimko/gfabasic-rust/issues/65) (2026-11-04).
- The supplied *GFA BASIC 3.5 Interpreter* manual identifies the included software as GFA BASIC 3.5 and contains the 3.5-specific addenda. Its source inventory and extracted feature notes are tracked in [PR #160](https://github.com/vsimko/gfabasic-rust/pull/160); the manual itself is not committed.
- The 1989 German Amiga book explicitly documents interpreter 3.0. It remains a historical baseline, not sole proof of 3.5 behavior; see [the Amiga 3.0 research notes](../../reference/amiga-gfa-basic-3.0.md).
- The 1990 German Amiga book's main command reference identifies interpreter V3.03 despite its title's “up to 3.5” wording. Keep its interpreter and compiler coverage separate; see [the source register](../../reference-sources.md).
- Contemporary *Amiga Magazine* evidence lists interpreter 3.51 and compiler 3.52 separately; it is historical commercial evidence, not a language specification.
- Atari ST research is separately recorded in [the Atari ST GFA BASIC V2.xx/V3.0 notes](../../reference/atari-gfa-basic-3.0.md) and must not be transferred to Amiga.
