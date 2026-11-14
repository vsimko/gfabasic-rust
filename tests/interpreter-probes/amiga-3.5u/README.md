# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

This branch temporarily replaces the broad suite with a minimal native diagnostic for issue #537. It opens the two output files, writes a marker before `MAT ONE`, runs the manual's 3x3 `a()` example, and records all nine resulting elements. The marker distinguishes a failure before program execution from a hold at or after `MAT ONE`.

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. Manual-backed expectations and observed results remain distinct. This is not a claim of complete language conformance.

The native workflow `.github/workflows/amiga-35u-native-merge-run.yml` runs this suite against the private 3.5U interpreter when this profile or its workflow changes on `main`. It verifies the source ZIP and extracted executable hashes before booting the emulator.

## MAT ONE isolation experiment (#537)

A branch-only follow-up probe uses the addendum's own `a()` example with a square 3x3 array and explicitly executes `MAT BASE 1` immediately before `MAT ONE a()`. The existing scalar/DATA variable `a` was renamed within this probe so it does not share a name with the matrix. This experiment is not part of the passing `main` regression suite unless the exact native 3.5U run completes successfully.
