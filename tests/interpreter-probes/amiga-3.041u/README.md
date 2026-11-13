# GFA BASIC 3.041U interpreter probe

This profile targets the exact Amiga GFA BASIC **3.041 U** interpreter:

- Executable: `gfabasic`
- SHA-256: `f5185db2dfc99af11a15570120f0c40df9f8ad201c28b43cc0894bfaa9562759`
- Binary size and AmigaOS loadseg header are checked in CI before installation.

The native workflow extracts the single ADF from the corresponding private ZIP and copies it byte-for-byte as the working disk. It replaces the interpreter file in place with the hash-verified executable, replaces `S/Startup-Sequence` with our controlled startup sequence, and installs `gfatest.LST` on that same ADF. It does not format, synthesize, unpack or repack a replacement disk. CI verifies that the resulting ADF has the same size as the source and that its boot block is unchanged.

Amiberry boots and runs the customized original disk. The private ZIP, source and modified ADFs, Kickstart ROM and commercial executable are never committed or uploaded. CI artifacts contain only hashes, disk listings, startup text, screenshots and textual test output.

The initial regression batch covers arithmetic precedence, FOR/NEXT and string operations. Runtime observations are specific to this exact 3.041U binary and recorded Amiberry/Kickstart environment. They are a separate 3.0-era comparison profile, not 3.5U or 3.52D evidence.
