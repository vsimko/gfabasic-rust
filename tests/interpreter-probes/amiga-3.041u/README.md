# GFA BASIC 3.041U interpreter probe

This profile targets the separate Amiga GFA BASIC **3.041 U** interpreter:

- Executable: `gfabasic`
- Size: 125,092 bytes
- SHA-256: `f5185db2dfc99af11a15570120f0c40df9f8ad201c28b43cc0894bfaa9562759`
- Embedded identity: `GFA-BASIC Version 3.041 U`

The workflow extracts the executable from the private 3.041 ADF ZIP, verifies its size, SHA-256 and AmigaOS loadseg header, then places it on a fresh bootable working floppy. The private ADF, ROM and commercial executable are never committed or uploaded.

The initial regression batch covers arithmetic precedence, FOR/NEXT and string operations. Runtime observations are specific to this exact 3.041U binary and recorded Amiberry/Kickstart environment. They are a separate 3.0-era comparison profile, not 3.5U or 3.52D evidence.
