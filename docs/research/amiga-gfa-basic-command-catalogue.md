---
type: "Research Guide"
title: "Amiga GFA BASIC command catalogue"
tags: [amiga, gfa-basic, catalogue, navigation]
---

# Amiga GFA BASIC command catalogue

This page is the entry point for discovering the command, function, statement, property, and API material already extracted from the Amiga GFA BASIC references. It is a navigation and scope guide. The linked inventories remain the evidence-bearing sources; this page does not create a new language specification or claim runtime support.

## Start here

- **Find a name in the book's alphabetical index:** use the [Appendix D OCR transcription](amiga-book-command-index-ocr-transcription.md) or the [first-pass command index](gfa-basic-command-index-ocr.md).
- **Understand the source pages and chapter coverage:** use the [515-page extraction audit](amiga-book-515-page-extraction-audit.md), [chapter heading harvest](amiga-book-chapters05-17-heading-harvest.md), and [source/topic index](amiga-gfa-basic-source-topic-index.md).
- **Read the detailed extraction:** use the [German book command and Amiga OS API extraction](amiga-gfa-basic-german-book-command-api-extraction.md).
- **Check built-in function names in structured form:** use the [function inventory](amiga-gfa-basic-functions.toml) and its [human-readable notes](amiga-gfa-basic-function-inventory.md).
- **Check 3.5 applicability:** use the [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md), [candidate manifest](amiga-gfa-basic-candidates.toml), and [generated candidate report](amiga-gfa-basic-candidate-report.md).
- **Check implementation status:** consult the language/API reference and implementation records separately. A manual entry is not evidence that the Rust compiler or runtime implements it.

## Existing inventories and what they mean

| Resource | What it contains | How to interpret it |
|---|---|---|
| [Appendix D OCR transcription](amiga-book-command-index-ocr-transcription.md) | Alphabetical transcription of the German book's printed command index, pp. 493–497 | Broad discovery list; includes more than callable commands and has OCR uncertainty |
| [First-pass command index](gfa-basic-command-index-ocr.md) | A second, richer OCR transcription of the same Appendix D index | Candidate inventory; retain compound forms and uncertain spellings pending classification |
| [Detailed book/API extraction](amiga-gfa-basic-german-book-command-api-extraction.md) | Chapter-based extraction through Chapter 19 and Appendices B–D, reconciled with the English 3.5 addendum | Main detailed research source for semantics, Amiga OS interfaces, and page references |
| [Source/topic index](amiga-gfa-basic-source-topic-index.md) | Consolidated printed-page navigation and cross-source disagreements | Best route from a topic to the relevant source pages |
| [Function inventory](amiga-gfa-basic-functions.toml) | Structured names found in the combined manual | 116 unique names; 114 interpreter candidates, with disputed/compiler-only distinctions documented alongside it |
| [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) | Features explicitly documented by the 3.5 addendum, especially Chapters 13–14 | Version-specific evidence; not a complete inventory of every feature in the interpreter |
| [Candidate manifest and report](amiga-gfa-basic-candidates.toml) / [report](amiga-gfa-basic-candidate-report.md) | Version-scoped candidate families and their evidence state | The report currently has 13 family entries confirmed in 3.5 and 43 documented in the 3.0 base only; these are family-level statuses, not individual command counts |

## Counts and version boundaries

Do not describe the material as a verified list of “400+ commands”:

- The German book's introduction describes approximately **360 commands and functions** for the interpreter version it covers.
- The book's Appendix D is an alphabetical index, not a normative declaration table. It mixes statements, functions, aliases, constants, variables, object properties, and other terms. OCR can also duplicate, damage, or merge entries.
- The book is identified in its introduction as describing **GFA V3.03**. Its cover/title reference to 3.5 does not make every Appendix D entry a 3.5 declaration.
- The structured function inventory has **116 unique names** (114 interpreter candidates under its documented counting rules). This is a function-name inventory, not a total command count.
- The 3.5 candidate report counts **families**, not individual declarations: 13 are currently confirmed in the 3.5 addendum and 43 are documented in the 3.0 base only, with 3.5 applicability not established.
- The detailed book extraction and the 3.5 addendum reconciliation provide broad coverage, but individual declarations still need evidence-level classification and, where relevant, runtime conformance tests.

These numbers answer different questions and must not be added together. Before quoting a total, define whether it counts index entries, normalized identifiers, statement forms, functions, properties, OS calls, aliases, or version-confirmed declarations.

## Recommended research workflow

1. Search the alphabetical OCR index for the spelling or alias.
2. Follow the index's printed page reference into the detailed extraction and source/topic index.
3. Record the source, platform, edition, interpreter version, printed page, and exact syntax form.
4. Classify the item: statement, function, operator, property, constant, variable, alias, editor feature, or OS/library call.
5. Reconcile the evidence against the 3.5 primary reference/addendum. Keep 3.0/3.03-only evidence separate from explicit 3.5 evidence.
6. Update the structured candidate/function inventory where applicable. Do not scrape this explanatory page to generate machine-readable declarations.
7. Track parser, semantic, runtime, and native-interpreter verification independently. Documentation evidence alone does not establish implementation.

## Source and verification limits

The [research overview](README.md) describes the research-only policy and the broader source set. The [documentation policy](../DOCUMENTATION.md) defines how to preserve provenance and distinguish documented behavior from observed execution. For current supported language behavior, consult the [reference index](../reference/index.md), not the raw OCR index.
