# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The native suite is an incremental smoke/regression batch. It currently covers FACT boundary values, the 3.5 addendum combinatorics example (`FACT(6)`, `VARIAT(6,2)`, `COMBIN(6,2)` => `720 30 15`), integer arithmetic and precedence, FOR/NEXT, string concatenation/LEFT$, and the 3.5 addendum's DATA-pointer example: DATA, _DATA, DO WHILE DATA, READ, INC, DEC and FOR...DOWNTO. The latter probe follows the example on printed pages 13-34–13-35: it saves each DATA pointer, restores them in reverse order, and checks the documented reverse sequence (0, 1, 3242, 328, 24, 13, 9, 8, 7, 6, 5, 4, 3, 2, 1).

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. Manual-backed expectations and observed results remain distinct. This is not a claim of complete language conformance.

The native workflow `.github/workflows/amiga-35u-native-merge-run.yml` runs this suite against the private 3.5U interpreter when this profile or its workflow changes on `main`. It verifies the source ZIP and extracted executable hashes before booting the emulator.
