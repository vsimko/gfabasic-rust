---
type: "Research Audit"
title: "Amiga GFA BASIC documentation coverage audit"
tags: [amiga, gfa-basic, documentation, audit]
---

# Amiga GFA BASIC documentation coverage audit

**Audit date:** 2026-11-10  
**Target:** Commodore Amiga GFA BASIC interpreter 3.5  
**Tracking:** [GitHub issue #446](https://github.com/vsimko/gfabasic-rust/issues/446)

This page records the current documentation evidence boundary and the remaining workstreams. It is an audit/navigation aid, not a replacement language specification and not a claim of implementation or runtime conformance.

## Source authority and version boundary

| Source ID | Scope | Audit treatment |
|---|---|---|
| `AM-INTERPRETER-35` | Amiga GFA BASIC 3.5 interpreter manual: 3.0 base material plus explicitly marked 3.5 updates | Normative for explicitly identified 3.5 material. The editor update on printed p. 1-25 and the addendum have been visually reviewed. Base chapters are not thereby revalidated as 3.5. |
| `AM-INTERPRETER-30-35-UPDATE-SCAN` | Separately supplied 3.0 manual with bound 3.5 update | Independent pagination/source witness for the same addendum; use printed addendum page numbers and the documented crosswalk. |
| `AM-BOOK-DE-1990` | German Amiga book, 1990; foreword describes interpreter 3.0, section 1 identifies V3.03 | Historical Amiga command/API and compiler evidence. Its cover wording “bis Version 3.5” is not proof that the main interpreter command reference is 3.5. |
| `AM-BOOK-DE-1989` / `AM-BOOK-FR` | Historical Amiga interpreter 3.0 references | Corroboration for 3.0 only; not normative evidence for 3.5. |
| Atari ST manuals and references | Atari dialects/releases | Separate dialect evidence; do not transfer Amiga semantics without corroboration. |

Use the source register and the [3.5 primary reference notes](amiga-gfa-basic-35-primary-reference.md) for bibliographic metadata, pagination and detailed source limitations. Cite printed pages, not PDF viewer page numbers.

## Coverage status

| Area | Current evidence/documentation | Remaining work | Workstream |
|---|---|---|---|
| Source/version provenance | Source register, topic index, primary reference notes and addendum inventory identify the Amiga 3.5 target and distinguish 3.0/V3.03 sources | Keep all feature-level claims version-scoped; reconcile stale status text when found | [#19](https://github.com/vsimko/gfabasic-rust/issues/19), [#446](https://github.com/vsimko/gfabasic-rust/issues/446) |
| Explicit 3.5 editor and language additions | Editor update (printed p. 1-25) and addendum page-image review; inventory covers matrix operations and further 3.5 commands | Keep parser/editor implementation and runtime conformance distinct from source evidence | [#19](https://github.com/vsimko/gfabasic-rust/issues/19), [#21](https://github.com/vsimko/gfabasic-rust/issues/21), [#284](https://github.com/vsimko/gfabasic-rust/issues/284), [#285](https://github.com/vsimko/gfabasic-rust/issues/285) |
| Lexical rules and source format | Existing 3.0 research candidates; 3.5 primary notes explicitly say encoding, BOM, line endings, identifiers, string escapes and numeric grammar are not established | Extract exact 3.5 evidence or record a named interpreter observation; do not infer UTF-8 or loader behavior | [#20](https://github.com/vsimko/gfabasic-rust/issues/20), [#120](https://github.com/vsimko/gfabasic-rust/issues/120) |
| Types, arrays, operators and expressions | Historical 3.0 inventory and topic extraction; incomplete 3.5 confirmation | Extract full signatures/semantics with printed-page references; distinguish inherited candidates from verified 3.5 rules | [#20](https://github.com/vsimko/gfabasic-rust/issues/20), [#284](https://github.com/vsimko/gfabasic-rust/issues/284) |
| Statements and control flow | Historical 3.0 chapter coverage; partial parser support documented separately | Complete grammar and 3.5 applicability; keep implemented parser subset explicit | [#21](https://github.com/vsimko/gfabasic-rust/issues/21), [#47](https://github.com/vsimko/gfabasic-rust/issues/47) |
| Built-in functions | Historical inventory; the 3.5 addendum explicitly adds FACT, VARIAT, COMBIN and GURU(). FACT/VARIAT/COMBIN are in the language-core registry; GURU() is currently declared in the Amiga profile alongside the GURU statement | Reconcile the 114 interpreter candidates from #307 with declaration-level 3.5 evidence; decide and document whether GURU() belongs in language-core or remains an Amiga-profile intrinsic, without duplicating its stable declaration | [#22](https://github.com/vsimko/gfabasic-rust/issues/22), [#285](https://github.com/vsimko/gfabasic-rust/issues/285), [#307](https://github.com/vsimko/gfabasic-rust/issues/307) |
| Files, disks, DATA and resources | Detailed German book extraction and selected 3.5 addendum evidence | Verify 3.5 applicability and complete command semantics; preserve printed page provenance | [#23](https://github.com/vsimko/gfabasic-rust/issues/23) |
| Amiga platform APIs | Broad historical Amiga API extraction | Separate language core from AmigaDOS/Exec/graphics/window/event APIs and distinguish interpreter from compiler/runtime support | [#24](https://github.com/vsimko/gfabasic-rust/issues/24) |
| Examples and conformance | Independently authored corpus and focused parser regressions exist | Map every example to dialect/version/evidence; distinguish documented examples from observed native behavior | [#42](https://github.com/vsimko/gfabasic-rust/issues/42) |
| API naming and declaration identity | Canonical API catalogue and language-core/built-in registry workstreams | Reconcile prose names, registry IDs, categories, signatures and declaration status | [#283](https://github.com/vsimko/gfabasic-rust/issues/283), [#284](https://github.com/vsimko/gfabasic-rust/issues/284), [#285](https://github.com/vsimko/gfabasic-rust/issues/285) |

## Registry taxonomy finding: GURU function\n\nThe 3.5 addendum documents both a `GURU` statement (enable/disable checking) and a distinct `GURU` function (returns the current GURU number), on printed pp. 13-36 (combined update scan pp. 14-6–14-7). The registry currently places both declarations in `api/registry/amiga/special-and-platform-commands.toml`; the function has stable ID `gfa.amiga.system.guru-number`. The built-in catalogue issue #285 and the function inventory in #307 should explicitly account for this existing declaration. Do not add a second `gfa.language.*` record until the project decides whether this Amiga-specific intrinsic belongs in the language-core namespace or remains in the Amiga profile.\n\n## Resolved source discrepancy: MAT INV

The 3.5 addendum inventory resolves the operand direction as:

`MAT INV destination()=source()`

The detailed definition and worked example on printed p. 13-30 say that the inverse of source matrix `a()` is written to destination `b()`. The reversed summary form on printed p. 13-16 is treated as a source typo. The German 1990 book's p. 475 is corroboration only, because that book's main interpreter reference is version-scoped to 3.0/V3.03.

This resolves the documented syntax direction only. Runtime edge cases, parser support and conformance remain separate. The older progress note in issue #47 has been explicitly corrected by a comment; do not treat its earlier “both directions rejected pending evidence” statement as current source status.

## Required status vocabulary

Use separate status dimensions rather than a single “supported” flag:

- **Documented:** the feature appears in a named source.
- **Source-verified:** the exact passage, version applicability and printed page have been checked.
- **Implemented:** the repository has an implementation for the stated subset.
- **Runtime-conformant:** behavior has been tested against a named interpreter/runtime build with reproducible evidence.

A feature can be documented but not source-verified for 3.5; implemented but not runtime-conformant; or source-verified while not implemented.

## Audit follow-up

1. Continue the issue-linked extraction work rather than promoting 3.0-only claims to 3.5.
2. Keep the source/topic index and candidate manifest synchronized with page-image findings.
3. Reconcile reference prose with the structured API registries and independently authored corpus.
4. Search for and correct stale “incomplete”, “disputed”, or version claims after evidence has been superseded.
5. Add follow-up issues only for gaps that are not already covered by the workstreams above.

The [research index](index.md), [reference index](../reference/index.md), and [command catalogue](amiga-gfa-basic-command-catalogue.md) are the navigation entry points. This page intentionally summarizes status and links to the evidence rather than duplicating detailed command definitions.
