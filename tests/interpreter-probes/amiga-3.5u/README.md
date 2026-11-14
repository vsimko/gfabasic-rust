# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded version banner: `GFA-BASIC Version 3.5 U`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and prepares a working disk for Amiberry. It verifies the exact executable hash before execution. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The native suite prioritizes deterministic GFA BASIC features used by Devil's Queer House (DQH): ABS, INT, MIN, SGN, ASC, CHR$, CVI, MKI$, LEFT$, MID$, LEN, STRING$, UPPER$, WHILE/WEND, REPEAT/UNTIL, SELECT/CASE, arithmetic, FOR/NEXT, strings and DATA-pointer handling. It now adds an initial graphics/screen subset: SETCOLOR, COLOR, PLOT, LINE, BOX, PBOX, PCIRCLE, OPENS, OPENW, SCREEN(), WINDOW(), SETSPEN, SETWPEN, CLEARW, CLOSEW and CLOSES. These checks use fixed coordinates and palette values, with POINT() as a readback helper. This remains an initial prioritized subset, not complete DQH command coverage.

The graphics syntax and expectations are based on the supplied GFA BASIC 3.5 Interpreter manual's inherited Amiga graphics and screen/window chapters (printed pp. 8-2, 8-10, 8-15–8-18, 9-17–9-20, 9-23–9-27, 9-32, 9-34–9-35). The manual's base chapters identify themselves as GFA BASIC 3.0 Amiga; the separate 3.5 addendum does not restate these commands. Therefore the manual is source evidence, while the native run is the runtime observation for the exact 3.5U executable.

Results are runtime observations specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in each workflow artifact. Manual-backed expectations and observed results remain distinct. This is not a claim of complete language conformance.

The native workflow `.github/workflows/amiga-35u-native-merge-run.yml` runs this suite against the private 3.5U interpreter when this profile or its workflow changes on `main`. It verifies the source ZIP and extracted executable hashes before booting the emulator.
