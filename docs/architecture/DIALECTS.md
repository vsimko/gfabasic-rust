# Dialects, versions, and compatibility policy

- Status: **Accepted initial target: Amiga GFA BASIC interpreter 3.0.** Atari ST GFA BASIC V2.xx/V3.0 remains research-only for the initial compiler target. See [ADR 0008](adr/0008-initial-source-dialect-amiga-gfa-basic-3.0.md).

## Initial normative target

The first normative source-language target is **Amiga GFA BASIC interpreter 3.0**. The Amiga research identifies interpreter 3.0 in a French 1989 reference book; page-image verification and completion of the feature-by-feature evidence inventory remain outstanding. This decision selects the target for specification and implementation; it does not claim that any language feature is implemented or conformant.

Atari ST GFA BASIC V2.xx/V3.0 remains research-only. Its manual and book evidence may be developed as a separate profile, but Atari-specific syntax or semantics must not be imported into the Amiga 3.0 contract. Constructs documented only by the Atari source remain unsupported or unresolved for Amiga until independently evidenced.

## Required identity

Every source compilation must resolve an explicit dialect identity and version. Defaults may be added only after an accepted decision. API profile selection is separate. Dialect identity must not be inferred from the output target.

## Evidence rules

- Use the applicable GFA BASIC manual as the normative source for documented syntax and behavior.
- Record exact title, edition, version, platform, page, and section.
- Do not transfer rules from Atari ST to Amiga merely because a port exists.
- Separate documented rules from implementation observations.
- Record compiler/interpreter differences when evidence is available.
- Mark undocumented constructs unresolved until evidence supports a decision.

## Separate Atari ST V2.xx/V3.0 research

The source register now includes the user-provided sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (Uwe Litzkendorf, 1988, ISBN 3-89011-222-6). It explicitly covers V2.xx/V3.0 on Atari ST. Initial findings and their printed-page references are recorded in [the preliminary Atari specification](../reference/atari-gfa-basic-3.0.md). Do not apply these findings to Amiga GFA BASIC or later Atari releases without separate evidence.

## Topics requiring a specification

Encoding and line endings; identifiers and case rules; keywords; comments; statement boundaries; labels; literals; operators and precedence; type suffixes; arrays; procedures and functions; parameter passing; control flow; error behavior; built-ins; source composition; and version differences.

## Required feature record

Each feature should include a stable identifier, applicable dialect/version/profile, syntax, semantics and edge cases, source citation, evidence status, implementation status, conformance tests, and known deviations.

## Diagnostics

Unknown or unsupported syntax must not be skipped. Diagnostics must identify the source location and explain whether the construct is invalid, unsupported, ambiguous, or unavailable in the selected dialect/profile. The proposed structured diagnostic and support-outcome contract is defined in [DIAGNOSTICS.md](DIAGNOSTICS.md).

Selecting Amiga GFA BASIC interpreter 3.0 establishes the normative target, not a supported language subset. Future milestones must state a bounded implemented subset and cite sources.

Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3); [reference register](../reference-sources.md).
