---
type: "Architecture"
title: "Dialects, versions, and compatibility policy"
---

# Dialects, versions, and compatibility policy

- Status: **Sole supported source target: Commodore Amiga GFA BASIC interpreter 3.5.** See [ADR 0009](adr/0009-initial-source-dialect-amiga-gfa-basic-3.5.md) and product-scope clarification [ADR 0011](adr/0011-amiga-35-linux-sdl3-product-scope.md).

## Sole normative target

The compiler targets **Amiga GFA BASIC interpreter 3.5 only**. This is the sole supported source dialect/version, not merely the first item in a multi-dialect roadmap. The supplied 3.5 Interpreter manual and its version-specific addenda are the primary normative sources.

The 1989 Amiga book documents interpreter 3.0, while the 1990 German Amiga book's main command reference identifies interpreter V3.03. Treat rules found only in those earlier references as inherited candidates until their applicability to 3.5 is confirmed. The 3.5 manual's version-specific material is primary for 3.5 additions and changes.

Atari ST GFA BASIC V2.xx/V3.0 and Atari ST 3.5 are not supported targets and must not be implemented as alternate dialects or API profiles. Existing Atari material may remain as historical research, but it must not be used to fill gaps in Amiga evidence or expand product scope. Other Amiga interpreter/compiler releases, including 3.51/3.52, are also outside the supported source-version contract.

## Required identity

Every source compilation must identify Amiga GFA BASIC interpreter 3.5 explicitly, either through a fixed product default or an explicit configuration field. Do not infer dialect/version from the output target, file extension, or source contents. There is no dialect-selection matrix because only one source dialect is in scope.

## Evidence rules

- Use the applicable Amiga GFA BASIC 3.5 interpreter manual as the primary source for 3.5 behavior.
- Record exact title, edition, version, platform, page, and section.
- Mark rules supported only by 3.0/3.03 sources as inherited candidates until confirmed for 3.5.
- Do not transfer rules from Atari ST or another BASIC dialect to Amiga.
- Separate documented rules from implementation observations.
- Mark undocumented or ambiguous constructs unresolved until evidence supports a decision.
- Keep interpreter and compiler products/version numbers distinct.

## Topics requiring a specification

Encoding and line endings; identifiers and case rules; keywords; comments; statement boundaries; labels; literals; operators and precedence; type suffixes; arrays; procedures and functions; parameter passing; control flow; error behavior; built-ins; source composition; and version-specific behavior.

## Required feature record

Each feature should include a stable identifier, Amiga 3.5 applicability, syntax, semantics and edge cases, source citation, evidence status, implementation status, conformance tests, and known deviations.

## Diagnostics

Unknown or unsupported syntax must not be skipped. Diagnostics must identify the source location and explain whether the construct is invalid, unsupported, ambiguous, or unavailable in the selected Amiga 3.5/Linux runtime contract. The structured diagnostic and support-outcome contract is defined in [DIAGNOSTICS.md](DIAGNOSTICS.md).

Selecting Amiga GFA BASIC interpreter 3.5 establishes the normative target, not a supported language subset. Each implementation milestone must state a bounded subset and cite its evidence.

Tracking: [issue #3](https://github.com/vsimko/gfabasic-rust/issues/3); [reference register](../reference-sources.md).
