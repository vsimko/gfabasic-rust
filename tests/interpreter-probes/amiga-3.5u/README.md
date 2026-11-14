# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The native suite is an incremental smoke/regression batch. It currently covers FACT boundary values, integer arithmetic and precedence, FOR/NEXT, string concatenation/LEFT$, MAT DET, MAT INV, and the 3.5 addendum's DATA-pointer example: DATA, _DATA, DO WHILE DATA, READ, INC, DEC and FOR...DOWNTO. The DATA-pointer probe follows printed pages 13-34–13-35. The MAT DET probe uses the command form on pp. 13-26–13-27. The MAT INV probe follows the inverse command and same-type destination rule on printed page 13-30; it checks the inverse of a 2x2 matrix by multiplying the result by the source matrix and checking for the identity matrix.

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. Manual-backed expectations and observed results remain distinct. This is not a claim of complete language conformance.

The native workflow `.github/workflows/amiga-35u-native-merge-run.yml` runs this suite against the private 3.5U interpreter when this profile or its workflow changes on `main`. It verifies the source ZIP and extracted executable hashes before booting the emulator.
