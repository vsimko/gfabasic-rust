# GFA BASIC 3.041U interpreter probe

This profile targets the separate Amiga GFA BASIC **3.041 U** interpreter:

- Executable: `gfabasic`
- Size: 125,092 bytes
- SHA-256: `f5185db2dfc99af11a15570120f0c40df9f8ad201c28b43cc0894bfaa9562759`
- Embedded identity: `GFA-BASIC Version 3.041 U`

The native workflow takes the single ADF contained in the private 3.041 ZIP as its base disk. It unpacks that disk, replaces the interpreter at its original path with the hash-verified executable, replaces the disk's `S/Startup-Sequence` with our controlled startup sequence, and adds `gfatest.LST`. It then repacks the same volume, preserving the original disk's filesystem contents, volume metadata and boot block except for those deliberate changes.

The generated ADF is used directly by Amiberry. The private ZIP, original and customized ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The initial regression batch covers arithmetic precedence, FOR/NEXT and string operations. Runtime observations are specific to this exact 3.041U binary and recorded Amiberry/Kickstart environment. They are a separate 3.0-era comparison profile, not 3.5U or 3.52D evidence.
