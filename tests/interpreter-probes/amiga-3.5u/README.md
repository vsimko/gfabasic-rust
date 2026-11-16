# GFA-BASIC 3.52 interpreter probe

This profile targets the exact Amiga GFA-BASIC **3.52** distribution ZIP:

- Asset: `GFA-BASIC v3.52 (1990)(GFA Systemtechnik)(DE)(Disk 1 of 2)[WB].zip`
- Private asset Git blob SHA: `3c14e037559c2c2e1c0ae96088a7a7689594e4f5`
- The workflow verifies the ZIP Git blob SHA and mounts the ZIP directly as DF1 in Amiberry. Amiberry treats the compressed floppy as read-only; CI does not extract or rewrite the commercial disk image.
 The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

This branch carries a focused diagnostic for issue #537. The staged `gfatest.LST` is copied from this fixture and its contents are preserved as an artifact. The native editor Merge path is captured before and after the double-click; inspect the post-Merge screenshot against the staged source before interpreting execution results.

Manual testing found that the editor in GFA-BASIC 3.5 and 3.52 expands the short form `M O a()` to `MAT ONE a()` after Merge and the program runs. GFA-BASIC 3.041 instead leaves `==> M O a()` visible and fails at runtime with a syntax error. These are observations for the tested reference interpreters; do not transfer them across versions.

The 3.52 probe now uses the full spelling `MAT ONE a()` directly. It deliberately does not use `M O a()` or `MAT PRINT`: the latter is replaced by scalar PRINT statements for the nine matrix elements. This isolates MAT ONE from abbreviation expansion and from the separate MAT PRINT command. The supplied *GFA-BASIC 3.5 Interpreter* addendum, printed p. 13-2, defines MAT ONE as a square identity matrix.

The harness verifies the staged listing against the copy extracted from DF0 byte-for-byte before launching Amiberry. After Merge, recognized `MAT SET` remains a hard stop because it contradicts the full `MAT ONE` source. OCR is only a contradiction detector and cannot prove editor-buffer identity.

The probe records a marker before and after MAT ONE and checks the nine scalar matrix values. Native results are observations specific to the exact 3.52 binary hash and emulator/Kickstart setup recorded in each workflow artifact. This is not a claim of complete language conformance.
