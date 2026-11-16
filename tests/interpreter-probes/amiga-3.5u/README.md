# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

This branch temporarily replaces the broad suite with a launch-control probe for issue #537. The listing is merged through the native Editor and Run path. It writes a marker before `MAT ONE`, then checks the documented 3x3 identity matrix. Its setup follows the supplied *GFA-BASIC 3.5 Interpreter* addendum example on printed p. 13-2: `DIM a(3,3)`, `MAT ONE a()`, `MAT PRINT a()`. It intentionally does not insert `MAT SET` or `MAT BASE`; neither is part of that example, and the addendum says the default MAT BASE is 1.

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. This is not a claim of complete language conformance.
