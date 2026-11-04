# Dialects, versions, and compatibility policy

- Status: **Accepted initial target: Amiga GFA BASIC interpreter 3.5.** Atari ST GFA BASIC V2.xx/V3.0 remains research-only for the initial compiler target. See [ADR 0009](adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md); [ADR 0008](adr/0008-initial-source-dialect-amiga-gfa-basic-3.0.md) preserves the superseded 3.0 decision.

## Initial normative target

The first normative source-language target is **Commodore Amiga GFA BASIC interpreter 3.5**. The supplied 3.5 Interpreter manual identifies the included software as GFA BASIC 3.5 and includes version-specific addenda. This decision selects the target for specification and implementation; it does not claim that any language feature is implemented or conformant.

The 1989 Amiga book documents interpreter 3.0, while the 1990 German Amiga book's main command reference identifies interpreter V3.03 despite its title's “up to 3.5” wording. Treat rules found only in those earlier references as inherited candidates until their applicability to 3.5 is confirmed. The 3.5 manual's own version-specific material is the primary source for 3.5 additions and changes.

Atari ST GFA BASIC V2.xx/V3.0 and Atari ST 3.5 material remain research-only. Atari-specific syntax or semantics must not be imported into the Amiga 3.5 contract. Constructs documented only by an Atari source remain unsupported or unresolved for Amiga until independently evidenced.

## Required identity

Every source compilation must resolve an explicit dialect identity and version. The selected initial identity is Amiga GFA BASIC interpreter 3.5. The exact CLI/configuration spelling remains governed by the CLI/configuration specification. API profile selection is separate. Dialect identity must not be inferred from the output target.

## Version boundaries

- Keep interpreter and compiler versions separate. Contemporary Amiga commercial evidence lists interpreter 3.51 and compiler 3.52 as distinct products.
- Do not infer executable component versions from archive catalogue labels alone; the corresponding disk images have not been inspected.
- Do not assume that 3.0 or 3.03 language rules remain unchanged in 3.5. Record each rule's source version and its 3.5 confirmation status.
- Keep the 3.5 interpreter manual's addenda distinct from the manual's originally printed 3.0 material.

## Evidence rules

- Use the applicable Amiga GFA BASIC 3.5 interpreter manual as the primary source for 3.5-specific behavior.
- Record exact title, edition, version, platform, page, and section.
- Mark rules supported only by 3.0/3.03 sources as inherited candidates until confirmed for 3.5.
- Do not transfer rules from Atari ST to Amiga merely because a port exists.
- Separate documented rules from implementation observations.
- Record compiler/interpreter differences when evidence is available.
- Mark undocumented or ambiguous constructs unresolved until evidence supports a decision.

## Separate Atari ST research

The source register includes the sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (Uwe Litzkendorf, 1988, ISBN 3-89011-222-6), which explicitly covers Atari ST V2.xx/V3.0. Initial findings are recorded in [the preliminary Atari specification](../reference/atari-gfa-basic-3.0.md). Do not apply these findings to Amiga GFA BASIC or later Atari releases without separate evidence.

## Topics requiring a specification

Encoding and line endings; identifiers and case rules; keywords; comments; statement boundaries; labels; literals; operators and precedence; type suffixes; arrays; procedures and functions; parameter passing; control flow; error behavior; built-ins; source composition; and version differences.

## Required feature record

Each feature should include a stable identifier, applicable dialect/version/profile, syntax, semantics and edge cases, source citation, evidence status, implementation status, conformance tests, and known deviations.

## Diagnostics

Unknown or unsupported syntax must not be skipped. Diagnostics must identify the source location and explain whether the construct is invalid, unsupported, ambiguous, or unavailable in the selected dialect/profile. The structured diagnostic and support-outcome contract is defined in [DIAGNOSTICS.md](DIAGNOSTICS.md).

Selecting Amiga GFA BASIC interpreter 3.5 establishes the normative target, not a supported language subset. Future milestones must state a bounded implemented subset and cite sources.

Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3); [reference register](../reference-sources.md).
