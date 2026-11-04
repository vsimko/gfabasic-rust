# Aminet GFA BASIC package survey

- **Status:** Initial catalogue from the live directory listing; archive contents, package readmes, source languages, and redistribution terms have not yet been inspected.
- **Tracking:** [issue #64](https://github.com/vsimko/gfabasic-rust/issues/64).
- **Directory surveyed:** [Aminet `dev/basic`](https://ftp.fau.de/mirrors/aminet/dev/basic/), mirrored by [Aminet mirror index](https://aminet.net/dev/basic/).
- **Survey date:** 2026-11-04.

## Method

The complete current `dev/basic` directory index was reviewed. Search terms used for this first pass included `GFA`, `GFA BASIC`, `GFA_`, `DevBas`, `SRC`, `include`, `gadgets`, `menus`, and `tokenized`.

This pass records directory metadata and package-name clues only. A filename suffix such as `SRC`, `DevBas`, or `inc` is a discovery clue, not proof of the archive's contents, source dialect, or license. No archive has been downloaded or unpacked for this inventory. Do not treat any package as a normative language reference or as an implementation-ready conformance test.

## Selected candidates

The date and size below are the values displayed by the mirror index. Archive contents and licenses remain unverified.

| Archive | Listing date | Listed size | Provisional classification | Evidence / next step |
|---|---:|---:|---|---|
| [`gfalist.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/gfalist.lha) | 2021-02-18 | 150 KB | Tool candidate | The listing description says it reads tokenized GFA BASIC versions. Inspect readme and executable behavior; do not infer language semantics from it. |
| [`GFA_Gadgets.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GFA_Gadgets.lha) | 1998-12-31 | 28 KB | Gadget/library candidate | A paired readme exists. Inspect both before recording API names or dependencies. |
| [`GFA_Menus.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GFA_Menus.lha) | 1998-12-31 | 15 KB | Menu/library candidate | A paired readme exists. Determine whether it contains GFA source, includes, binaries, or examples. |
| [`GFA_CLI.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GFA_CLI.lha) | 2000-07-17 | 2.2 KB | CLI/tool candidate | A paired readme exists. Verify intended GFA BASIC version and actual contents. |
| [`GuiGfxDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GuiGfxDevBas.lha) | 2003-07-21 | 31 KB | GUI/graphics development-library candidate | A paired readme exists. Inspect declarations and distinguish wrappers from built-ins. |
| [`GFABasicFIX.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GFABasicFIX.lha) | 1998-01-23 | 32 KB | Fix/patch candidate | A paired readme exists. Identify affected interpreter/compiler release and patch terms. |
| [`GTBoxConv.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GTBoxConv.lha) | 1996-05-05 | 44 KB | Conversion-tool candidate | A paired readme exists. Determine input/output formats and whether any GFA source is included. |
| [`GetCDDB_src.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GetCDDB_src.lha) | 2000-01-08 | 20 KB | Source-archive candidate | The `_src` suffix is only a clue. Inspect file list, language, author, and rights. |
| [`HotShotsSRC.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/HotShotsSRC.lha) | 2000-01-09 | 403 KB | Source-archive candidate | Inspect for GFA BASIC source, dependencies, and redistribution terms before considering test-corpus use. |
| [`OutTakesSRC.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/OutTakesSRC.lha) | 2000-01-10 | 82 KB | Source-archive candidate | Inspect for GFA BASIC source, dependencies, and redistribution terms before considering test-corpus use. |
| [`ID3TagDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/ID3TagDevBas.lha) | 2011-08-27 | 37 KB | Developer-library candidate | Inspect API wrapper, required library version, examples, and license. |
| [`PointerDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/PointerDevBas.lha) | 2017-01-09 | 34 KB | Developer-library candidate | Inspect pointer-related declarations and examples; do not infer core pointer semantics. |
| [`ReqAttDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/ReqAttDevBas.lha) | 2002-06-23 | 27 KB | Requester/attribute library candidate | Inspect declarations and dependencies; likely relevant to platform API research only if contents confirm it. |
| [`HotShotsSRC.readme`](https://ftp.fau.de/mirrors/aminet/dev/basic/HotShotsSRC.readme) | 2005-11-19 | 966 B | Package metadata | Readme is listed; contents were not retrieved in this pass. |

## Classification rules

Use these categories only after inspecting the archive:

- **Documentation:** manuals, command references, or explanatory material.
- **Example/source program:** complete or partial source intended to be compiled or run.
- **Library/API wrapper:** includes, declarations, wrappers, or reusable routines that call platform libraries.
- **Tool:** editor, converter, tokenized-file reader, debugger, or other development utility.
- **Unrelated/uncertain:** a BASIC archive that is not GFA BASIC, or whose relationship cannot be established.

A third-party library or wrapper is not evidence that a name is a built-in GFA BASIC command. Keep its declarations separate from the language core and from the Amiga API reference until independently verified.

## Follow-up and rights checklist

For each selected archive:

1. Retrieve the paired readme and record its complete description, author, date, version, dependencies, and stated license.
2. Download only where access and terms permit; record the source URL and SHA-256 checksum.
3. List archive members without committing copyrighted binaries or source.
4. Identify source files, includes, examples, generated files, and required external libraries.
5. Confirm whether source is actually GFA BASIC and record any dialect/version clues.
6. Review copyright and redistribution terms before retaining source or derived fixtures.
7. Select small, redistributable examples for [issue #42](https://github.com/vsimko/gfabasic-rust/issues/42) only after provenance and expected behavior are established.
8. Link confirmed platform API wrapper findings to [issue #24](https://github.com/vsimko/gfabasic-rust/issues/24).

## Current limitations

- Only the directory listing was surveyed; package readmes and archive members remain uninspected.
- The listing contains material for multiple BASIC dialects. The category name alone does not establish GFA BASIC applicability.
- No license is inferred from an Aminet listing or from the presence of a downloadable archive.
- No package is currently promoted to a conformance fixture or normative language/API evidence.
