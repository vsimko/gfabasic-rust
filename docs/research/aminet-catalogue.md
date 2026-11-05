# Aminet GFA BASIC package survey

- **Status:** Initial catalogue from the live directory listing; one package-page false positive has been classified. Most archive contents, package readmes, source languages, and redistribution terms remain uninspected.
- **Tracking:** [issue #64](https://github.com/vsimko/gfabasic-rust/issues/64).
- **Directory surveyed:** [Aminet `dev/basic`](https://ftp.fau.de/mirrors/aminet/dev/basic/), mirrored by [Aminet mirror index](https://aminet.net/dev/basic/).
- **Survey date:** 2026-11-04; targeted GFABasicFIX readme re-check: 2026-11-05.

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
| [`GTBoxConv.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GTBoxConv.lha) | 1996-05-05 | 44 KB | **Unrelated/false positive:** GadToolsBox-to-Blitz2 converter | Aminet package page identifies version 4.50 by Fabio Rotondo for m68k AmigaOS; its two listed members are a guide and converter executable. It generates Blitz2 code, not GFA BASIC. License remains unknown. |
| [`GetCDDB_src.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GetCDDB_src.lha) | 2000-01-08 | 20 KB | Source-archive candidate | The `_src` suffix is only a clue. Inspect file list, language, author, and rights. |
| [`HotShotsSRC.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/HotShotsSRC.lha) | 2000-01-09 | 403 KB | Source-archive candidate | Inspect for GFA BASIC source, dependencies, and redistribution terms before considering test-corpus use. |
| [`OutTakesSRC.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/OutTakesSRC.lha) | 2000-01-10 | 82 KB | Source-archive candidate | Inspect for GFA BASIC source, dependencies, and redistribution terms before considering test-corpus use. |
| [`ID3TagDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/ID3TagDevBas.lha) | 2011-08-27 | 37 KB | Developer-library candidate | Inspect API wrapper, required library version, examples, and license. |
| [`PointerDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/PointerDevBas.lha) | 2017-01-09 | 34 KB | Developer-library candidate | Inspect pointer-related declarations and examples; do not infer core pointer semantics. |
| [`ReqAttDevBas.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/ReqAttDevBas.lha) | 2002-06-23 | 27 KB | Requester/attribute library candidate | Inspect declarations and dependencies; likely relevant to platform API research only if contents confirm it. |
| [`HotShotsSRC.readme`](https://ftp.fau.de/mirrors/aminet/dev/basic/HotShotsSRC.readme) | 2005-11-19 | 966 B | Package metadata | Readme is listed; contents were not retrieved in this pass. |

## Verified false positive: GTBoxConv

The [Aminet package page](https://aminet.net/package/dev/basic/GTBoxConv) identifies GTBoxConv as version 4.50, author Fabio Rotondo, architecture m68k AmigaOS, dated 1996-05-05. Its description calls it a GadToolsBox-to-Blitz2 code generator. The package member listing contains only `GTBox.guide` and the `GTBoxConv` executable. It is therefore unrelated to GFA BASIC despite appearing in the `dev/basic` category. The page does not state a license; do not redistribute or use it as a GFA BASIC fixture.

This is metadata and the archive member listing exposed by Aminet's package page; the executable was not downloaded or run.

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

- The complete directory listing was surveyed. GTBoxConv's package metadata and member listing were inspected through its Aminet page, but its executable was not downloaded or run; other selected package readmes and archive members remain uninspected.
- The listing contains material for multiple BASIC dialects. The category name alone does not establish GFA BASIC applicability.
- No license is inferred from an Aminet listing or from the presence of a downloadable archive.
- No package is currently promoted to a conformance fixture or normative language/API evidence.

## Readme inspection update (2026-11-04)

A second pass retrieved selected readmes from the Aminet mirror. This provides package-level evidence for three additional classifications; it does not constitute archive extraction or execution.

| Archive | Readme evidence | Updated classification | Rights / corpus consequence |
|---|---|---|---|
| [`GFABasicFIX.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GFABasicFIX.lha) | Readme identifies Andre Pfeiffer as author, version 1.0, requires **GFA BASIC + Compiler 3.5**, and describes a fix for AmigaOS 3.x / 68060 compatibility. | **GFA BASIC 3.5 compatibility patch/tool**; useful as evidence of a release-specific environment/compatibility fix, not a language reference or source fixture. | Readme says “Freely Distributable”. Preserve the exact attribution and terms if later retaining any material; archive members remain uninspected. |
| [`GetCDDB_src.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/GetCDDB_src.lha) | Readme identifies Roger Light, version 0.08b, and explicitly calls the source **Blitz Basic II**. It names `newgetcddb.bb2` as the program/GUI entry point and `GetCDDB_BigFunc.bb2` as a function source. | **Unrelated to GFA BASIC: Blitz Basic II source archive.** The `.bb2` filenames and readme agree. | Author states the files are freeware except third-party material and retains copyright; users should credit the author. Do not add as GFA BASIC corpus material. |
| [`HotShotsSRC.lha`](https://ftp.fau.de/mirrors/aminet/dev/basic/HotShotsSRC.lha) | Readme identifies Chris Nash, version 1.2, and explicitly describes an unfinished **Blitz Basic II** game source. | **Unrelated to GFA BASIC: Blitz Basic II game source.** | Author permits inclusion on media and modification/reuse, but expressly prohibits selling the archive, parts, or converted results. This is not an unrestricted redistribution license; exclude from the GFA corpus. |

The mirror's current directory index confirms the associated archive names, listing dates, and sizes. The readme files are independently linked above: [GFABasicFIX readme](https://ftp.fau.de/mirrors/aminet/dev/basic/GFABasicFIX.readme), [GetCDDB source readme](https://ftpmirror2.infania.net/sites/aminet.net/dev/basic/GetCDDB_src.readme), and [HotShots source readme](https://ftpmirror2.infania.net/sites/aminet.net/dev/basic/HotShotsSRC.readme). Readme retrieval for some other legacy entries failed because of unsupported text encodings or content types; those entries remain unverified rather than being classified by guesswork.

## Current limitations (updated)

- No archive has been downloaded, unpacked, checksummed, or executed in this pass.
- `GFABasicFIX` is now confirmed as a patch for GFA BASIC/Compiler 3.5, but its patch contents and exact affected components are not yet inspected.
- `GetCDDB_src` and `HotShotsSRC` are excluded as Blitz Basic II material; neither is a GFA BASIC example.
- The paired readmes for `GFA_CLI`, `GFA_Gadgets`, `GFA_Menus`, and `GuiGfxDevBas` could not be decoded by the available reader. Their listing metadata is confirmed, but their contents and terms remain unverified.
- No package is promoted to a conformance fixture or normative language/API evidence.


## Tokenized-source utility follow-up: gfalist (2026-11-04)

The gfalist lead is now better identified through its package metadata and a public source repository:

- The Aminet/MorphOS package record describes gfalist as reading tokenized GFA BASIC version 3.x files, lists version 0.01, authors Peter Backes and Markus Hoffmann, uploader Stefan Haubenthal, MorphOS/PPC architecture, upload date 2021-02-18, size 149 KB, and MD5 37f5ea38e38ef4bb0c73805aef9d0b92. See the [MorphOS Storage package record](https://www.morphos-storage.net/?id=1613594).
- The public [mmuman/gfalist source repository](https://github.com/mmuman/gfalist) describes the utility as converting tokenized .GFA files into readable ASCII .LST listings. Its source repository includes a GPL-2.0 license file. This is useful corroboration for the tool's function and for locating source code, but the exact license and contents of the separate Aminet MorphOS archive still need to be verified independently.
- The tool is a **tokenized-source inspection/conversion utility**, not a GFA BASIC compiler, language specification, or conformance corpus. Its documented target is GFA BASIC 3.x; do not assume that it decodes every Amiga-specific binary variant or every release without testing.
- No source code or archive content from the external project was copied into this repository.

References: [mmuman/gfalist README](https://github.com/mmuman/gfalist/blob/master/README.md), [license](https://github.com/mmuman/gfalist/blob/master/COPYING), and [Aminet directory entry](https://ftp.fau.de/mirrors/aminet/dev/basic/gfalist.readme).

The remaining research step is to inspect the exact Aminet archive contents and determine whether its executable/build matches the public source repository, including its supported tokenized file variants.

## Additional false-positive classification: HB2Gads (2026-11-04)

The paired [HB2Gads readme](https://ftpmirror2.infania.net/sites/aminet.net/dev/basic/HB2Gads.readme) identifies the package as “GadTools Gadgets in HBasic2”, with Benito Lombardi as author/uploader and m68k AmigaOS as its architecture. The archive is therefore an HBasic2/Blitz Basic II ecosystem library candidate, not a GFA BASIC language or API reference. The five-line readme does not state redistribution terms, so its license remains unknown. The archive itself was not downloaded or inspected.

This is another example of why the Aminet directory category and BASIC-related filename are insufficient to establish GFA BASIC applicability.


## Targeted GFABasicFIX readme re-check (2026-11-05)

The live Aminet directory and paired readme were re-opened. The readme identifies the package as version 1.0 by Andre Pfeiffer, requires GFA BASIC + Compiler 3.5, targets AmigaOS 3.x and 68060 CPUs, and labels the distribution “Freely Distributable”. This is evidence of a compatibility fix for the 3.5 toolchain on newer AmigaOS/CPU combinations, not evidence about language syntax or interpreter semantics. The archive itself remains uninspected; no binary or source was downloaded or committed.
