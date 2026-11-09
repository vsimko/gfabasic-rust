# Amiga GFA-BASIC 3.5 binary-analysis workflow

The interpreter manual remains the primary source for command spelling and documented syntax. Binary analysis is a discovery aid for finding token tables, command/function registries, parser dispatch, and implementation routines. Decompiled C-like output does not itself establish a command signature.

## Running it

1. Obtain an Amiga GFA-BASIC interpreter executable through a lawful source.
2. Publish it at a temporary HTTPS location accessible to GitHub Actions. Do **not** commit the executable, disk image, ROM, or manual to this public repository.
3. Open **Actions → Amiga GFA-BASIC binary analysis → Run workflow**.
4. Supply the HTTPS URL and the executable's SHA-256 digest.
5. Download the \`amiga-gfa-basic-analysis\` artifact from the run.

The workflow checks the supplied digest, extracts printable strings with file offsets, imports the executable into Ghidra using a Motorola 68000 big-endian processor, and exports a function inventory plus C-like decompilation. Ghidra is downloaded from the official NSA Ghidra GitHub release at run time.

## Important limitations

- The workflow expects a single executable file, not an ADF disk image. Extract the executable from a lawfully obtained disk image before running it.
- Ghidra's loader must recognize the executable format (for example, Amiga Hunk). If auto-detection fails, the import step needs a format-specific loader configuration.
- Printable strings may be absent, compressed, tokenized, or encoded in a non-ASCII character set.
- A command name found in a string is only a candidate. The report does not infer command syntax from strings.
- The next reverse-engineering step is to locate token/keyword tables and parser/dispatch references in Ghidra, then compare each candidate with the Amiga 3.5 interpreter manual. Keep 3.0 base-manual evidence and 3.5 addendum evidence explicitly version-scoped.
- Decompiled output is generated as an artifact and is not committed. Review licensing and provenance before retaining or publishing any derived material.

The analysis artifact is retained for 30 days. The original executable is not uploaded as an artifact.
