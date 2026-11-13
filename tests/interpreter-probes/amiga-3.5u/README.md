# GFA BASIC 3.5U interpreter probe

This suite targets the exact Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded identity: `GFA-BASIC Version 3.5 U`

The native workflow takes the single ADF contained in the private 3.5U ZIP as its base disk. It unpacks that disk, replaces the interpreter at its original path with the hash-verified executable, replaces the disk's `S/Startup-Sequence` with our controlled startup sequence, and adds `gfatest.LST`. It then repacks the same volume, preserving the original disk's filesystem contents, volume metadata and boot block except for those deliberate changes.

The generated ADF is used directly by Amiberry. The private ZIP, original and customized ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The current suite is a first smoke/regression batch covering FACT boundary values, integer arithmetic, FOR/NEXT, and string operations. Results are specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in the workflow artifact. This is runtime observation, not a claim of complete language conformance.
