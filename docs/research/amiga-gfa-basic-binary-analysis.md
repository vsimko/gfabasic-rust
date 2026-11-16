---
type: "Research Note"
title: "Amiga GFA-BASIC binary analysis"
tags: [amiga, gfa-basic, binary-analysis, research]
---

# Amiga GFA-BASIC binary analysis

The existing **Prepare Amiga reference interpreter** workflow is the single entry point for both preparing the emulator input and analyzing the interpreter. Run it through **Actions → Prepare and analyze Amiga GFA-BASIC reference interpreter → Run workflow**.

The binary is maintained in the private repository [vsimko/gfabasic-private-assets](https://github.com/vsimko/gfabasic-private-assets), at root path `GFABasic`. Its private-assets README identifies it as a 136,548-byte AmigaOS LoadSeg executable from GFA BASIC 3.52 (1990), SHA-256 `fd80f6093e8d0163856af5849121e945e37051a2ad2a9053aa1930021783d962`.

## Access and outputs

The workflow reuses the existing `GFA_ASSETS_TOKEN` Actions secret. It must be a token with read-only Contents access to `vsimko/gfabasic-private-assets`; no second secret or second workflow is needed.

The run produces two artifacts and publishes a reviewed repository update:

- `gfa-basic-3.52-amiga`: verified interpreter executable plus a provenance note, retained for 7 days.
- `amiga-gfa-basic-analysis`: complete output including decompilation, retained for 30 days.

The workflow also updates the dedicated `amiga-analysis/3.52` branch with the durable report and machine-readable indexes under `analysis/amiga-3.52/`, then opens or updates a pull request against `main`. The public branch excludes the original executable and C-like decompilation files; those remain in the temporary Actions artifact. Each update is reviewable before it reaches `main`.

The executable is staged under the exact same path used by the Ghidra import step and is never committed to the public repository. Durable analysis indexes are published separately; C-like decompilation remains only in the Actions artifact. The analysis runs with a Motorola 68000 big-endian processor definition.

## Interpreting the results

This is a discovery pipeline, not yet a complete command-reference generator. Printable strings and function names are candidate evidence only. The next step is to identify keyword/token tables and parser/dispatch code, then map each command to its token and grammar. The resulting command/syntax table must be checked against the Amiga GFA-BASIC 3.5 interpreter manual.

Keep version provenance explicit:

- The binary is labelled **3.52**, not 3.5. Binary-derived observations remain tagged 3.52 until corroborated.
- Base-manual 3.0 material is not automatically normative for 3.5; distinguish it from explicit 3.5 addendum material.
- A string that resembles a command is not proof of a supported command or its syntax.

The executable is an AmigaOS LoadSeg file, not an ADF. Ghidra's loader must recognize its Hunk format; the first run validates that import. Kickstart ROMs and disk images are not bundled. Decompiled output is retained only as a workflow artifact; review licensing and provenance before publishing derived material.
