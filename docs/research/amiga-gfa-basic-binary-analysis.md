# Amiga GFA-BASIC binary-analysis workflow

The interpreter binary is maintained in the private repository [vsimko/gfabasic-private-assets](https://github.com/vsimko/gfabasic-private-assets), at root path `GFABasic`. Its private-assets README records it as a 136,548-byte AmigaOS LoadSeg executable from GFA BASIC 3.52 (1990), with SHA-256 `fd80f6093e8d0163856af5849121e945e37051a2ad2a9053aa1930021783d962`.

## Running it

1. In the `gfabasic-rust` repository, configure the Actions repository secret `GFABASIC_PRIVATE_ASSETS_TOKEN` with a fine-grained PAT that has read-only Contents access to `vsimko/gfabasic-private-assets`.
2. Open **Actions → Amiga GFA-BASIC binary analysis → Run workflow**.
3. The workflow checks out the private assets repository into a temporary workspace, verifies the pinned SHA-256, and stages the executable for analysis.
4. Download the `amiga-gfa-basic-analysis` artifact from the run.

The workflow extracts printable strings with file offsets, imports the executable into Ghidra using a Motorola 68000 big-endian processor, and exports a function inventory plus C-like decompilation. The executable is not committed to or uploaded from the public repository.

## Important limitations

- The private asset is identified as AmigaOS LoadSeg, not an ADF image. Ghidra's loader must recognize this executable format; the first run will validate the import configuration.
- This binary is labelled GFA BASIC **3.52**, while the project's normative language target is Amiga interpreter **3.5**. Keep every binary-derived observation tagged as 3.52 until independently confirmed for 3.5.
- Printable strings may be absent, compressed, tokenized, or encoded in a non-ASCII character set.
- A command name found in a string is only a candidate. The report does not infer command syntax from strings.
- The next reverse-engineering step is to locate token/keyword tables and parser/dispatch references in Ghidra, then compare each candidate with the Amiga 3.5 interpreter manual. Keep base-manual 3.0 evidence and explicit 3.5 addendum evidence version-scoped.
- Decompiled output is generated as a temporary artifact and is not committed. Review licensing and provenance before retaining or publishing derived material.

The analysis artifact is retained for 30 days.
