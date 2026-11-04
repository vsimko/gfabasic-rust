# Dialects, versions, and compatibility policy

- Status: Accepted initial source dialect; feature-level specification remains in progress.
- Normative source target: **Amiga GFA BASIC interpreter 3.0**.
- Research-only comparison target: **Atari ST GFA BASIC V2.xx/V3.0**. Atari findings are not normative for the Amiga grammar or semantics.

The decision and rationale are recorded in [ADR 0008](adr/0008-initial-source-dialect-amiga-gfa-basic-3.0.md).

## Required identity

Every source compilation must resolve an explicit dialect identity and version. The initial normative identity is Amiga GFA BASIC interpreter 3.0. API profile selection is separate. Dialect identity must not be inferred from the output target. Exact CLI/configuration spelling and defaults remain governed by the CLI/configuration specification.

## Evidence rules

- Use the applicable GFA BASIC manual as the normative source for documented syntax and behavior.
- Record exact title, edition, version, platform, page, and section.
- Do not transfer rules from Atari ST to Amiga merely because a port exists.
- Separate documented rules from implementation observations.
- Record compiler/interpreter differences when evidence is available.
- Mark undocumented constructs unresolved until evidence supports a decision.

## Amiga GFA BASIC interpreter 3.0

Amiga 3.0 is the first normative language target. Its source research is recorded in [amiga-gfa-basic-3.0.md](../reference/amiga-gfa-basic-3.0.md). The evidence register identifies the available Amiga reference and outstanding page-image/OCR verification. No feature should be promoted to confirmed solely from an OCR-sensitive transcription.

## Atari ST research-only profile

The source register includes the sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (Uwe Litzkendorf, 1988, ISBN 3-89011-222-6) and the *GFA BASIC Version 3 User Manual* (GFA Software Technologies, Inc., July 1988, ISBN 1-85181-186-9). Findings are recorded in [the preliminary Atari specification](../reference/atari-gfa-basic-3.0.md). This material is research-only for the current project baseline. It must not fill gaps in Amiga 3.0 language or API evidence.

## Topics requiring a specification

Encoding and line endings; identifiers and case rules; keywords; comments; statement boundaries; labels; literals; operators and precedence; type suffixes; arrays; procedures and functions; parameter passing; control flow; error behavior; built-ins; source composition; and version differences.

## Required feature record

Each feature should include a stable identifier, applicable dialect/version/profile, syntax, semantics and edge cases, source citation, evidence status, implementation status, conformance tests, and known deviations.

## Diagnostics

Unknown or unsupported syntax must not be skipped. Diagnostics must identify the source location and explain whether the construct is invalid, unsupported, ambiguous, or unavailable in the selected dialect/profile. The structured diagnostic and support-outcome contract is defined in [DIAGNOSTICS.md](DIAGNOSTICS.md).

Selecting Amiga 3.0 establishes the normative research target; it does not claim that any GFA BASIC feature is implemented or conformant. Future implementation milestones must state a bounded subset and cite sources.

Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3); [reference register](../reference-sources.md).
