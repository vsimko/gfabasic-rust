# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

This branch temporarily replaces the broad suite with a launch-control probe for issue #537. It opens the two output files, writes a marker to each, closes them and exits. It contains no array or MAT command. A successful result proves that the editor's Run path can execute a trivial listing and write to the test disk; failure leaves the MAT ONE investigation unresolved.

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. This is not a claim of complete language conformance.
