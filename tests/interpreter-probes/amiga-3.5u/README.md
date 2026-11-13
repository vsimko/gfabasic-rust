# GFA BASIC 3.5U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and copies it byte-for-byte as the working disk. It replaces the interpreter file in place with the hash-verified executable, replaces `S/Startup-Sequence` with our controlled startup sequence, and installs `gfatest.LST` on that same ADF. It does not format, synthesize, unpack or repack a replacement disk. The source 3.5U ADF has an invalid/empty boot block, so the workflow installs the standard AmigaDOS 1.x boot block required to execute our startup sequence; this deliberate boot-block change is recorded in the artifact. CI verifies that the resulting ADF has the same size as the source and that the boot block is now valid.

Amiberry boots and runs the customized original disk. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The current suite is a first smoke/regression batch covering FACT boundary values, integer arithmetic, FOR/NEXT, and string operations. Results are specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in the workflow artifact. This is runtime observation, not a claim of complete language conformance.

The native workflow `.github/workflows/amiga-35u-native-merge-run.yml` runs this suite against the private 3.5U interpreter when this profile or its workflow changes on `main`. It verifies the source ZIP and extracted executable hashes before booting the emulator.
