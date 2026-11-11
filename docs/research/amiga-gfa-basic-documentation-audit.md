---
type: "Research Audit"
title: "Amiga GFA BASIC documentation coverage audit"
tags: [amiga, gfa-basic, documentation, audit]
---

# Amiga GFA BASIC documentation coverage audit

**Audit date:** 2026-11-11  
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
| Built-in functions | Historical inventory; the 3.5 addendum explicitly adds FACT, VARIAT, COMBIN and GURU(). FACT/VARIAT/COMBIN are in the language-core registry; GURU() is currently declared in the Amiga profile alongside the GURU statement | Reconcile the 115 interpreter candidates (113 unambiguous names plus two disputed name slots) from the 117-identifier inventory with declaration-level 3.5 evidence; preserve GURU() as an Amiga-profile intrinsic because it exposes Amiga GURU system-error context, without duplicating its stable declaration | [#22](https://github.com/vsimko/gfabasic-rust/issues/22), [#285](https://github.com/vsimko/gfabasic-rust/issues/285), [#307](https://github.com/vsimko/gfabasic-rust/issues/307) |
| Files, disks, DATA and resources | Detailed German book extraction and selected 3.5 addendum evidence | Verify 3.5 applicability and complete command semantics; preserve printed page provenance | [#23](https://github.com/vsimko/gfabasic-rust/issues/23) |
| Amiga platform APIs | Broad historical Amiga API extraction | Separate language core from AmigaDOS/Exec/graphics/window/event APIs and distinguish interpreter from compiler/runtime support | [#24](https://github.com/vsimko/gfabasic-rust/issues/24) |
| Examples and conformance | Independently authored corpus and focused parser regressions exist | Map every example to dialect/version/evidence; distinguish documented examples from observed native behavior | [#42](https://github.com/vsimko/gfabasic-rust/issues/42) |
| API naming and declaration identity | Canonical API catalogue and language-core/built-in registry workstreams | Reconcile prose names, registry IDs, categories, signatures and declaration status | [#283](https://github.com/vsimko/gfabasic-rust/issues/283), [#284](https://github.com/vsimko/gfabasic-rust/issues/284), [#285](https://github.com/vsimko/gfabasic-rust/issues/285) |

## Registry taxonomy finding: GURU function

The 3.5 addendum documents both a `GURU` statement (enable/disable checking) and a distinct `GURU` function (returns the current GURU number), on printed pp. 13-36 (combined update scan pp. 14-6–14-7). The registry places both declarations in `api/registry/amiga/special-and-platform-commands.toml`; the function has stable ID `gfa.amiga.system.guru-number`. This is intentionally Amiga-profile-scoped because it exposes the Amiga GURU system-error context, not a general language operation. The source does not specify the function's return type (see [the GURU declaration audit](amiga-gfa-basic-35-guru-declaration-audit.md)), so the registry signature records that uncertainty instead of inferring an integer type. The built-in catalogue issue #285 and function inventory in #307 account for this existing declaration. Do not add a second `gfa.language.*` record.

## Resolved source discrepancy: MAT INV

The 3.5 addendum inventory resolves the operand direction as:

`MAT INV destination()=source()`

The detailed definition and worked example on printed p. 13-30 say that the inverse of source matrix `a()` is written to destination `b()`. The reversed summary form on printed p. 13-16 is treated as a source typo. The German 1990 book's p. 475 is corroboration only, because that book's main interpreter reference is version-scoped to 3.0/V3.03.

This resolves the documented syntax direction only. Runtime edge cases, parser support and conformance remain separate. Issue #47's former disputed-status section has been replaced with the resolved destination-first syntax evidence; do not treat its earlier “both directions rejected pending evidence” statement as current source status.

## Registry population cross-check

The current registry README reports an interim 3.5 addendum catalogue of **24 language-core declarations plus four Amiga-profile declarations (28 total)**. The four Amiga-profile records are `PCOLOR`, the negative-screen-number `OPENW` variant, the `GURU` statement, and the `GURU()` function. The language-core subset includes `FACT`, `VARIAT`, and `COMBIN`; the GURU function remains Amiga-profile-scoped under `gfa.amiga.system.guru-number` and must not be duplicated under a second ID. This 28-entry addendum slice is not the complete language-wide 3.5 denominator requested by #307. The function inventory contains 117 unique identifiers: 115 interpreter candidates (113 unambiguous names plus two disputed name slots) and two compiler-only names (`CINT` and `CFLOAT`). The 115 interpreter candidates include 3.0 base-text candidates and are not 115 verified 3.5 declarations.

## Corpus cross-check

The corpus currently has 13 independently authored Amiga 3.5 fixtures marked `verified` on cited page-image evidence and three Amiga 3.0 fixtures marked `candidate` (16 total). The fixture summary in `tests/corpus/README.md` is generated from the manifest and sidecar metadata, and `scripts/validate_corpus.py` checks that the summary stays synchronized; `docs/architecture/TESTING.md` links to that canonical summary rather than maintaining a second numeric count (issue [#474](https://github.com/vsimko/gfabasic-rust/issues/474), completed 2026-11-11). The [corpus README declaration crosswalk](../../tests/corpus/README.md#registry-declaration-crosswalk) maps each fixture ID to its stable language-core declaration ID(s), including the multi-declaration combinatorics and MAT BASE/TRANS fixtures. These mappings are evidence links only: registry `support.tests` remains empty because the fixtures are not executable conformance tests. The corpus remains `research_only`: there is no general corpus runner, and none of these fixtures constitutes original-interpreter execution. The three Amiga 3.0 fixtures remain `candidate`; their presence does not establish 3.5 compatibility. Issue #42's stale four-fixture/partial-addendum-review summary has been corrected to reflect the current inventory and completed Chapter 13–14 page-image review recorded in #65. Issue #66 provides a native Amiberry execution workflow for the available 3.52 interpreter, but those observations must remain version-labelled 3.52; exact 3.5 interpreter media is still the blocker for 3.5 runtime observations.

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


## Declaration-level audit: four Amiga profile entries (2026-11-10)

Compared all four records in `api/registry/amiga/special-and-platform-commands.toml` with the page-image-reviewed primary manual (`AM-INTERPRETER-35`), the Chapter 14 addendum inventory, the primary-reference pagination crosswalk, and the source register. No signature or semantic correction was required.

| Registry ID / declaration | Primary 3.5 evidence | Inventory/source cross-check | Audit result |
|---|---|---|---|
| `gfa.amiga.graphics.pcolor` — `PCOLOR` | Chapter 13, printed p. 13-34 | Combined update scan, Chapter 14 printed p. 14-4; inventory records `PCOLOR foreground[,background]` | Match. PRINT screen output uses COLOR-like foreground/optional background; exact colour range and all mode interactions remain unspecified. |
| `gfa.amiga.windows.openw-negative-screen` — negative-screen `OPENW` variant | Chapter 13, printed p. 13-34 | Combined update scan, Chapter 14 printed p. 14-3; inventory records current/top screen behavior | Match. This is only the negative-screen variant, not a complete OPENW signature or lifecycle contract. |
| `gfa.amiga.system.guru-check` — `GURU x` statement | Chapter 13, printed p. 13-36 | Combined update scan, Chapter 14 printed pp. 14-6–14-7; inventory records zero/nonzero switch and #109/#110–#118 notes | Match. Compiled-program RESUME restrictions remain scoped to the manual's explicit statement; do not generalize them to interpreted mode. |
| `gfa.amiga.system.guru-number` — `GURU` function | Chapter 13, printed p. 13-36 | Combined update scan, Chapter 14 printed pp. 14-6–14-7; dedicated GURU audit records `PRINT HEX$(GURU)` | Match. The function's return type is unspecified by the source; registry correctly does not infer INTEGER. |

The registry's source ID `AM-INTERPRETER-35`, source scope `amiga-3.5-addendum`, and printed-page locators agree with the source register and addendum inventory. The German 1990 book is supplementary historical 3.0/V3.03 evidence (its “bis Version 3.5” cover wording is not normative interpreter evidence); it is not needed to promote these four declarations. The inventory's TOC/OCR spelling `PCOLOUR` is not a separate declaration: the primary addendum body and registry use `PCOLOR`.

These four entries are an **interim addendum-only platform slice**, not a complete Amiga 3.5 API. Their source documentation is verified for 3.5; implementation remains `not-implemented`, Linux support `unknown`, and conformance `not-tested`. The available native Amiberry observations are for interpreter **3.52** only (see #66), and must not be relabelled as 3.5 observations or used to claim 3.5 runtime conformance.


## Base-manual chapter evidence progress (2026-11-11)

The supplied interpreter manual's Chapters 7–12 now have separate page-cited research notes. These are **historical 3.0 base-manual evidence**, not declarations of 3.5 availability. The notes preserve printed-page locators and keep the 3.5 addendum and 3.52 runtime observations in separate sections.

| Chapter | Research note | Coverage |
|---|---|---|
| 7 | [Program-handling evidence](amiga-gfa-basic-chapter-7-program-handling-evidence.md) | Control flow, procedures/functions and error handling (pp. 7-2–7-55). |
| 8 | [Graphics evidence](amiga-gfa-basic-chapter-8-graphics-evidence.md) | Graphics statements and 3.5 PCOLOR reconciliation (pp. 8-2–8-24). |
| 9 | [Event/window evidence](amiga-gfa-basic-chapter-9-event-window-evidence.md) | Events, menus, screens, windows and requesters (pp. 9-2–9-39). |
| 10 | [Advanced-programming evidence](amiga-gfa-basic-chapter-10-advanced-programming-evidence.md) | CLI, platform variables, sprites, joystick, speech, FD files and BOB/OBJECT (pp. 10-2–10-16). |
| 11 | [System-routine evidence](amiga-gfa-basic-chapter-11-system-routines-evidence.md) | Seven Amiga OS library groups and their version/ABI boundary (pp. 11-2–11-92). |
| 12 | [Appendix and index evidence](amiga-gfa-basic-chapter-12-appendices-index-evidence.md) | Keyboard/ASCII references, fill patterns, AmigaDOS errors, command index and OS-call index (pp. 12-2–12-27). |

The structural validator `scripts/validate_amiga_chapter_inventories.py` now covers Chapters 7–12. Its CI checks validate required table structure, non-empty fields, duplicate labels, chapter-local printed-page citations and an explicit runtime-evidence section. They do **not** prove that a chapter inventory is exhaustive, that OCR transcriptions are correct, that aliases are semantically equivalent, or that a documented construct is implemented.

Remaining source-level gap: this chapter series does not close the 3.5 reconciliation for inherited 3.0 candidates. The 3.5 addendum inventory and the 13 verified page-image corpus fixtures remain distinct evidence sets. Exact 3.5 interpreter runtime validation remains tracked by [#66](https://github.com/vsimko/gfabasic-rust/issues/66); available 3.52 observations must not be relabelled as 3.5.
