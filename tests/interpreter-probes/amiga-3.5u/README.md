# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

This branch temporarily replaces the broad suite with a diagnostic for issue #537. The staged `gfatest.LST` is copied from this fixture and its contents are preserved as an artifact. The native editor Merge path is captured before and after the double-click; inspect the post-Merge screenshot against the staged source before interpreting execution results.

The previous full-spelling probe staged the exact ASCII line `MAT ONE a()` (confirmed in the artifact's `gfatest.LST`), but the post-Merge editor screenshot appeared to display `MAT SET a()`. That mismatch is unresolved: it may be an editor/token display or Merge issue, and must not be mistaken for proof that the source file itself contained `MAT SET`. The current probe therefore tests the documented short form `M O a()` separately. The supplied *GFA-BASIC 3.5 Interpreter* addendum, printed p. 13-2, documents both `MAT ONE a()` and the abbreviation `m o a()`; it defines the result as a square identity matrix. The German GFA-BASIC book independently describes the square-matrix requirement on printed p. 455.

The probe writes a marker before and after the short-form operation and prints the resulting 3x3 matrix. Native results are observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. This is not a claim of complete language conformance.
