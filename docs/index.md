---
okf_version: "0.2"
---

# Project documentation

This is the canonical OKF v0.2 documentation bundle for gfabasic-rust. Markdown files in this tree are the editable source; the project website is a generated publication.

# Documentation policy and overview

- [Documentation policy](DOCUMENTATION.md) - OKF v0.2 conventions, writing rules, and contributor workflow.
- [Documentation overview](README.md) - Overview of audiences and documentation areas.
- [Reference sources](reference-sources.md) - Bibliographic sources and provenance.

# Architecture and implementation

- [Architecture](architecture/) - System design, decisions, validation, and implementation status.
- [Runtime handler registration](runtime-handler-registration.md) - Canonical API handler mapping and runtime dispatch validation.

# Language and API reference

- [Reference](reference/) - Dialect, API, and feature reference material.

# Research and evidence

- [Research](research/) - Source reviews, inventories, and evidence notes.
- [Amiga GFA BASIC command catalogue](research/amiga-gfa-basic-command-catalogue.md) - Entry point for the extracted Amiga command/function lists, detailed book and OS API extraction, structured inventories, version evidence, and count definitions.
- [Amiga GFA BASIC language population policy](research/amiga-gfa-basic-language-population-policy.md) - Maintainer-approved denominator, normalization, exclusions, evidence and implementation counting rules for issue #307.
- [Amiga GFA BASIC Chapter 7 program-handling evidence](research/amiga-gfa-basic-chapter-7-program-handling-evidence.md) - Page-cited base-manual inventory for program control, procedures, and error handling; 3.52 probes are a separate evidence dimension.
- [Amiga GFA BASIC Chapter 8 graphics compatibility evidence](research/amiga-gfa-basic-chapter-8-graphics-evidence.md) - Page-cited graphics inventory, 3.5 PCOLOR reconciliation, and 3.52 probe coverage boundary.
- [Amiga GFA BASIC Chapter 9 event/window evidence](research/amiga-gfa-basic-chapter-9-event-window-evidence.md) - Page-cited event, menu, screen, window and requester inventory; isolates the documented 3.5 OPENW variant.
- [Amiga GFA BASIC Chapter 10 advanced programming evidence](research/amiga-gfa-basic-chapter-10-advanced-programming-evidence.md) - Page-cited advanced programming inventory; keeps Amiga-specific facilities and 3.0/3.5/3.52 evidence separate.
- [Amiga GFA BASIC Chapter 11 system-routine evidence](research/amiga-gfa-basic-chapter-11-system-routines-evidence.md) - Page-cited inventory of seven Amiga OS library groups; distinguishes 3.0 base-manual APIs, 3.5 addendum evidence, and 3.52 runtime probes.
- [Amiga GFA BASIC Chapter 12 appendix and index evidence](research/amiga-gfa-basic-chapter-12-appendices-index-evidence.md) - Page-cited reference-table inventory, AmigaDOS errors, and command/system-call indexes; preserves 3.0/3.5/3.52 evidence boundaries.
- [Amiga GFA BASIC test output](research/amiga-gfa-test-output.md) - Native file-output observation and proposed batch result protocol.

# Project use and contribution

- [Contributor guides](contributor/) - Development setup and contribution policies.
- [User guides](user/) - User-facing guidance.
- [Testing guides](testing/) - Reference interpreter and testing material.
- [Amiga GFA BASIC probe-writing guide](testing/amiga-gfa-basic-probes.md) - Rules for version-specific native interpreter probes.
- [Agent Skills](../.agents/skills/README.md) - Task-specific procedures for AI coding agents.
- [Templates](templates/) - Reusable documentation structures.
