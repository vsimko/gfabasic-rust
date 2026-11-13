# GFA BASIC 3.5U interpreter probe

This suite is intended for the extracted Amiga GFA BASIC **3.5 U** interpreter:

- Executable: `GFABasic`
- Size: 135,956 bytes
- SHA-256: `c5e9ffdb96b621307069d8f21a3282788386930bbc1ea679dfebe2caef5493ca`
- Embedded identity: `GFA-BASIC Version 3.5 U`

The native Amiberry workflow extracts the executable from the private 3.5U disk 1 ADF, verifies the hash and AmigaOS loadseg header, then copies it to a fresh bootable working floppy. The original ADF is not modified or published.

The current suite is a first smoke/regression batch covering FACT boundary values, integer arithmetic, FOR/NEXT, and string operations. Results are specific to the exact 3.5U binary hash and emulator/Kickstart setup recorded in the workflow artifact. This is runtime observation, not a claim of complete language conformance.
