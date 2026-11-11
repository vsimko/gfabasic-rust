# Research and evidence

## Documentation audit

- [Amiga GFA BASIC documentation coverage audit](amiga-gfa-basic-documentation-audit.md) - Current source boundary, verified 3.5 coverage, remaining base-language gaps, and linked GitHub workstreams.

## Amiga GFA BASIC command and API research

- [Command catalogue — start here](amiga-gfa-basic-command-catalogue.md) - Explains the existing alphabetical index, detailed extraction, function inventory, 3.5 evidence manifest, and how their counts differ.
- [German book command and Amiga OS API extraction](amiga-gfa-basic-german-book-command-api-extraction.md) - Main detailed chapter-based extraction through Chapter 19 and Appendices B–D, with 3.5 addendum reconciliation.
- [Appendix D command index OCR transcription](amiga-book-command-index-ocr-transcription.md) - Page-oriented transcription of the book's alphabetical index, printed pp. 493–497.
- [First-pass command index OCR](gfa-basic-command-index-ocr.md) - Richer first-pass transcription of Appendix D with compound forms and OCR caveats.
- [Source and topic index](amiga-gfa-basic-source-topic-index.md) - Printed-page navigation, source/version boundaries, disagreements, and page-image review status.
- [515-page extraction audit](amiga-book-515-page-extraction-audit.md) - PDF/page coverage audit for the German book.
- [Chapter 5–17 heading harvest](amiga-book-chapters05-17-heading-harvest.md) - Chapter structure and heading extraction.
- [Function inventory (117 unique names)](amiga-gfa-basic-functions.toml) - Machine-readable built-in function names and classification.
- [Function inventory notes](amiga-gfa-basic-function-inventory.md) - Human-readable scope and counting rules for the function inventory.
- [Base-manual I/O, graphics, events, and advanced-programming extraction](amiga-base-manual-io-graphics-events-advanced.md) - Chapter 5, 6, 8, 9, and 10 heading inventory with printed-page locators and explicit 3.0/3.5 source boundary.
- [Base-manual I/O, graphics, event, and advanced-programming details](amiga-base-manual-io-graphics-events-advanced-details.md) - Expanded signatures and semantics for Chapters 5, 6, 8, 9, and 10, with command/function classification and version caveats.
- [Base-manual numeric and string function extraction](amiga-base-manual-numeric-string-functions.md) - Source forms, argument categories, semantics, overloads, and printed-page locators for the Chapter 4 function families; explicitly separated from 3.5 availability evidence.
- [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) - Features explicitly described by the 3.5 addendum.
- [3.5 candidate manifest](amiga-gfa-basic-candidates.toml) - Structured version-applicability evidence by candidate family.
- [3.5 candidate report](amiga-gfa-basic-candidate-report.md) - Generated family-level status report; not a count of individual commands.
- [Language population policy](amiga-gfa-basic-language-population-policy.md) - Maintainer-approved rules for the issue #307 denominator, normalization, exclusions, evidence and implementation numerator.
- [3.03 book OCR index](amiga-gfa-basic-3.03-ocr-index.md) - Book introduction and contents evidence identifying the described interpreter version.

## Other Amiga research

- [Research overview](README.md) - Research scope, source policy, and remaining work.
- [Amiga GFA BASIC test output](amiga-gfa-test-output.md) - Native file-output observation and proposed batch result protocol.
- [Amiga GFA BASIC commands observed in dqh-amiga](amiga-gfa-basic-dqh-source-inventory.md) - Constructs and direct OS/library calls observed in an external game source.
- [Amiga emulator experiment lab](amiga-emulator-lab.md) - Proposed native interpreter experiment procedure.
- [Amiga GFA BASIC 3.5 interpreter probes](amiga-gfa-basic-35-lst-probes.md) - Manual-driven textual listing probe procedure.
- [Amiga GFA BASIC 3.52 runtime baseline](amiga-gfa-basic-352-runtime-baseline.md) - Version-scoped runtime observations.
- [Amiga GFA-BASIC binary analysis](amiga-gfa-basic-binary-analysis.md) - Reproducible binary analysis workflow.

## Base-manual topic research
- [GFA BASIC 3.0 variable, array, and conversion functions](amiga-base-manual-variable-array-conversion-functions.md) - Chapter 2 function forms, signatures, semantics, and unresolved evidence boundaries.
- [GFA BASIC Chapter 7 program-handling evidence](amiga-gfa-basic-chapter-7-program-handling-evidence.md) - Page-cited control-flow, procedures, functions, and error-handling inventory (printed pp. 7-2–7-55); keeps 3.0 documentation, 3.52 probes, and 3.5 claims separate.
- [GFA BASIC Chapter 8 graphics compatibility evidence](amiga-gfa-basic-chapter-8-graphics-evidence.md) - Page-cited graphics inventory (printed pp. 8-2–8-24), 3.5 PCOLOR reconciliation, and explicit absence of tracked 3.52 graphics probes.
- [GFA BASIC Chapter 9 event, menu, screen and window evidence](amiga-gfa-basic-chapter-9-event-window-evidence.md) - Page-cited event/menu/screen/window inventory (printed pp. 9-2–9-39), with the narrow 3.5 negative-screen OPENW variant separated.
- [GFA BASIC Chapter 10 advanced programming evidence](amiga-gfa-basic-chapter-10-advanced-programming-evidence.md) - Page-cited CLI, platform, sprite, joystick, speech, FD-file and BOB/OBJECT inventory (printed pp. 10-2–10-16), with 3.0/3.5/3.52 evidence kept distinct.
- [GFA BASIC Chapter 11 system-routine evidence](amiga-gfa-basic-chapter-11-system-routines-evidence.md) - Page-cited inventory of seven Amiga OS library groups (printed pp. 11-2–11-92); separates 3.0 base-manual APIs from 3.5 GURU additions and 3.52 probe coverage.
- [GFA BASIC Chapter 12 appendix and index evidence](amiga-gfa-basic-chapter-12-appendices-index-evidence.md) - Page-cited inventory of appendix reference tables, AmigaDOS errors, command index and OS-call index (printed pp. 12-2–12-27); documents OCR and version boundaries.
- [Chapter inventory validator](../../scripts/validate_amiga_chapter_inventories.py) - Structural checks for Chapter 7–12 page-cited inventories; does not assert completeness or semantic alias equivalence.
- [GFA BASIC 3.0 command index](amiga-gfa-basic-3.0-command-index.md) - Thematic command index derived from the base manual.
- [GFA BASIC 3.0 statement candidates](amiga-gfa-basic-3.0-statement-command-candidates.md) - Statement-family candidates requiring version reconciliation.
- [GFA BASIC 3.0 base inventory](amiga-gfa-basic-3.0-base-inventory.md) - Broad chapter-based base-manual inventory.
- [GFA BASIC 3.0 expressions, built-ins, and control flow](amiga-gfa-basic-3.0-expressions-builtins-control-flow.md) - Expression and built-in extraction.
- [GFA BASIC 3.0 control, file, and DATA I/O](amiga-gfa-basic-3.0-control-file-data-io.md) - Control-flow and I/O evidence.
- [GFA BASIC 3.0 graphics evidence](amiga-gfa-basic-3.0-graphics-evidence.md) - Graphics command evidence.
- [GFA BASIC 3.0 events and windows](amiga-gfa-basic-3.0-events-windows-evidence.md) - Event and window evidence.
- [GFA BASIC 3.0 system routines](amiga-gfa-basic-3.0-system-routines-evidence.md) - OS and system routine evidence.
- [GFA BASIC 3.0 advanced programming](amiga-gfa-basic-3.0-advanced-programming-evidence.md) - Advanced programming features.
- [GFA BASIC 3.0 appendix index evidence](amiga-gfa-basic-3.0-appendix-index-evidence.md) - Appendix and index evidence.
- [GFA BASIC 3.0 source notes](amiga-gfa-basic-3.0-german-source-notes.md) - Source edition and selected page notes.
- [GFA BASIC 3.0 procedure and error details](amiga-gfa-basic-3.0-procedure-error-details.md) - Procedures and error behavior.
- [GFA BASIC 3.0 screen and window details](amiga-gfa-basic-3.0-screen-window-details.md) - Screen/window behavior.
- [GFA BASIC 3.0 output device details](amiga-gfa-basic-3.0-output-device-details.md) - Output-device behavior.
- [GFA BASIC 3.0 menu event details](amiga-gfa-basic-3.0-menu-event-details.md) - Menu event payloads.
- [GFA BASIC 3.0 BOB object details](amiga-gfa-basic-3.0-bob-object-details.md) - BOB/object behavior.
- [GFA BASIC 3.0 source format and editor evidence](amiga-gfa-basic-3.0-source-format-editor-evidence.md) - Source and editor behavior.

## 3.5 focused audits

- [3.5 primary reference](amiga-gfa-basic-35-primary-reference.md) - Primary manual and version scope.
- [3.5 German-book cross-check](amiga-gfa-basic-35-german-book-crosscheck.md) - Independent evidence from explicitly 3.5-labelled Chapters 18–19 of the 1990 German Amiga book.
- [3.5 Chapter 18–19 extraction](amiga-book-gfa35-chapter18-19-extraction.md) - Dedicated 3.5 chapter extraction.
- [3.5 editor feature audit](amiga-gfa-basic-35-editor-feature-audit.md) - Editor-specific additions.
- [3.5 lexer evidence](amiga-gfa-basic-35-lexer-evidence.md) - Lexical evidence.
- [3.5 MAT arithmetic audit](amiga-gfa-basic-35-matrix-arithmetic-analysis-audit.md) - Matrix arithmetic details.
- [3.5 MAT I/O and copy audit](amiga-gfa-basic-35-matrix-io-copy-audit.md) - Matrix input/output and copy details.
- [3.5 declaration audits: FIVE](amiga-gfa-basic-35-five-declaration-audit.md) - FIVE declaration evidence.
- [3.5 declaration audits: FIVE MORE](amiga-gfa-basic-35-five-more-declaration-audit.md) - Additional declaration evidence.
- [3.5 GURU declaration audit](amiga-gfa-basic-35-guru-declaration-audit.md) - GURU declaration evidence.

## Atari and cross-platform research

- [Atari GFA book diskette](atari-gfa-book-diskette.md) - Static review of accompanying examples.
- [Systematic Atari book review](atari-gfa-book-systematic-review.md) - Chapter-by-chapter Atari reference review.
- [Aminet catalogue](aminet-catalogue.md) - Amiga BASIC package survey.
- [GFAList build and fixture validation](gfalist-build-and-fixture-validation.md) - GFAList build and fixture evidence.
- [GFAList format versus Rust frontend](gfalist-format-vs-rust-frontend.md) - Format comparison notes.
