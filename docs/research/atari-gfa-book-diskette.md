# Atari GFA BASIC book diskette — source-corpus research notes

- Source ID: `ATARI-GFA-BOOK-1988`
- Scope: accompanying diskette archive for Uwe Litzkendorf, *Das große GFA-BASIC-Buch*, 6th revised and expanded edition (1988), Atari ST, GFA BASIC V2.xx/V3.0.
- Inspection date: 2026-11-03
- Evidence status: **sample-corpus inspection**; examples demonstrate usage but are not a normative language reference.
- Related work: [#18 sources/provenance/licensing](https://github.com/vsimko/gfabasic-rust/issues/18), [#19 language feature inventory](https://github.com/vsimko/gfabasic-rust/issues/19), [#25 Atari ST API reference](https://github.com/vsimko/gfabasic-rust/issues/25), [#3 dialect/version policy](https://github.com/vsimko/gfabasic-rust/issues/3).

## 1. Archive inventory

The supplied nested archive contains **172 files**:

| Kind | Count | Research value |
|---|---:|---|
| GFA BASIC `.LST` listings | 162 | Language idioms, command discovery, version comparisons, platform API usage |
| Other support files | 10 | Binary program, assembler, data/definition files, GEM resources and fonts |

The ten non-`.LST` files are:

- `ADRESSEN.DAT`, `ADRESSEN.DEF`: data and definition pair; likely application data format, not language specification.
- `ALERT.RSC`, `FORMULAR.RSC`, `MENU.RSC`: GEM resource files; useful as platform/resource-format artifacts.
- `GFA.PRG`: binary executable; its exact role/version has not been established by this inventory.
- `SUPER.FON`, `SYSTEM.FNT`: font data.
- `SYSTEM.S`: Motorola 68000 assembler source.
- `SYSTEM.LST`: GFA BASIC loader that constructs `SYSTEM.PRG` from hexadecimal `DATA` values.

The diskette README is important provenance evidence. It says comments were removed from the listings to fit the programs on a single-sided disk, that some programs need extra source fragments merged into them using the interpreter's `MERGE` operation, and that users should make a write-protected backup. It specifically warns that the disk-formatting example in book section 21.5.2 formats the disk currently in the drive.

**Consequences:** a listing may be incomplete by design; absent comments are not language syntax; and source fragments must not be treated as standalone compilable tests until reconstructed from the book's instructions.

## 2. What the listings add to the language research

The corpus is especially useful as corroborating evidence for constructs and idioms that should be indexed under issue #19. It is not sufficient by itself to settle full grammar, precedence, type rules, or runtime edge cases.

| Topic | Example files | What the examples demonstrate | How to use |
|---|---|---|---|
| Procedures and local scope | `DEFFN.LST`, `GETDIR.LST`, `MENUE.LST`, `WINDOWS.LST` | `PROCEDURE`, `LOCAL`, parameters, calls using `@`, `RETURN` | Cross-check syntax and parameter semantics against the book's program-structure chapter |
| User functions | `DEFFN.LST`, `FN_ARITH.LST`, `FN_FSFIR.LST`, `FN_FSNEX.LST`, `FN_KEYGE.LST`, `FN_KEYTE.LST`, `FN_MALLO.LST`, `FN_MFREE.LST`, `FN_MSHRI.LST`, `FN_STICK.LST`, `VT52_LIB.LST`, `WORD.LST` | `DEFFN`, function calls and compact function definitions | Build a syntax inventory; confirm typing, evaluation and return rules in the manual |
| Conditional flow | `IF_ENDIF.LST`, `DEFFN.LST`, `DIGIT.LST`, `LOWER.LST`, `EXTEND.LST` | Nested `IF/ELSE/ENDIF`; V3-style `SELECT/CASE/ENDSELECT` appears in multiple samples | Confirm branch grammar, nesting and fall-through behavior from the manual |
| Loops and exits | `FOR_NEXT.LST`, `INPAUX.LST`, `GETDIR.LST`, `LUPE.LST`, `TIMER.LST`, `DEBBIE.LST`, `KOMPRESS.LST` | `FOR/NEXT`, `WHILE/WEND`, `REPEAT/UNTIL`, `DO/LOOP`, `EXIT`, `EXIT IF` | Add construct-specific conformance cases only after exact manual rules are extracted |
| V2.xx versus V3.0 | `PCODE_2X.LST`, `PCODE_30.LST`, `DUMP_V2X.LST`, `V2X_COMP.LST`, `GETDIR.LST`, `RC_COPY.LST` | Paired examples show changes in typed memory access, `VAR` parameter use, array/type inventories, and availability of `SELECT/CASE` | High-value version-difference evidence; compare line-by-line and verify against printed version notes |
| Variables, arrays and descriptors | `ARRPTR.LST`, `POINTER.LST`, `BOUND_V2.LST`, `MENU_ARR.LST`, `SIZE.LST` | Arrays, dimensions, `VARPTR`, array descriptors and string storage observations | Useful for memory-model research; do not infer a stable ABI from one example |
| Memory and typed access | `ABSOLUTE.LST`, `MALLOC.LST`, `OUT_4_XX.LST`, `SLOW.LST`, `SHOWBOOT.LST`, `WORD.LST`, `PCODE_30.LST` | `PEEK/POKE`, `DPEEK/DPOKE`, `LPEEK/LPOKE`, `BYTE{}`, `CARD{}`, typed pointer dereferencing and memory reservation | Separate language operators from Atari memory/runtime interfaces; document side effects and unsafe operations |
| DATA and serialization | `PCODE_2X.LST`, `PCODE_30.LST`, `DRAW.LST`, `EPS_INIT.LST`, `RECALL.LST`, `WRITE.LST`, `SCROLL.LST` | `DATA/READ/RESTORE`, `STORE/RECALL`, serialization to files and memory | Check the manual for data-pointer, array and EOF semantics |
| File and directory I/O | `DISK_OPS.LST`, `FILES.LST`, `GETDIR.LST`, `FIND.LST`, `RECALL.LST`, `WRITE.LST`, `REMKILL3.LST` | File open modes, handles, directory traversal, random access, bulk string/array operations | Good source for command/API discovery; verify each operation and error behavior |
| String processing | `LOWER.LST`, `RPLC.LST`, `REMKILL2.LST`, `REMKILL3.LST`, `GETDIR.LST`, `FN_ARITH.LST` | `LEFT$/RIGHT$/MID$`, `INSTR/RINSTR`, `TRIM$`, string concatenation and parsing | Discover functions and idioms; verify one-based indexing and edge cases in the manual |
| Graphics primitives and raster ops | `DRAW.LST`, `GPLANE.LST`, `DISTORT.LST`, `DSPRITE.LST`, `RASTER.LST`, `RC_COPY.LST`, `RC_COP_2.LST`, `RC_INTER.LST`, `STAR.LST`, `STAR_ANI.LST` | Drawing, screen buffers, sprites, raster copying and clipping/intersection helpers | Feed Atari graphics API research (#25), not language-core grammar |
| GEM, AES, VDI and windows | `AES_ALRT.LST`, `AES_FORM.LST`, `AES_MENU.LST`, `GDOS_V2.LST`, `V_OPNWK.LST`, `WINDOWS.LST`, `WINDTAB.LST`, `GETFILL.LST`, `GETLINE.LST`, `GETMARK.LST`, `GETTEXT.LST` | AES alerts/forms/menus, VDI/GDOS calls, workstation and window operations | Strong Atari API index; validate signatures against the API chapters and registry |
| Input, mouse and keyboard | `DMOUSE.LST`, `MSTOP.LST`, `KEYPRESS.LST`, `KEY_XXX.LST`, `SETMOUSE.LST`, `SETMOUS2.LST`, `STICK.LST`, `ON_BUTTN.LST` | Mouse state/coordinates, keyboard scanning, joystick and event-driven interaction | Platform API evidence; distinguish built-ins from BIOS/XBIOS calls |
| Sound and timing | `DOSOUND.LST`, `WAVE.LST`, `TIMER.LST`, `VDI_TIMR.LST`, `XBIOS_16.LST`, `XBIOS_17.LST` | Sound-related calls, timing loops and hardware/system timer access | Atari API research; exact hardware behavior requires manual and runtime validation |
| Algorithms and utilities | `QSORT.LST`, `SORT.LST`, `KOMPRESS.LST`, `RANDOM_A.LST`, `DIGIT.LST`, `LOWER.LST`, `VIRUSKIL.LST` | Sorting, compression, random generation, conversion and utility-program patterns | Useful for realistic parser coverage, but not authoritative command documentation |

The examples also expose less common constructs and syntax worth locating in the book: `ABSOLUTE`, `VAR` parameters, `CHAR{}`, `BYTE{}`, `CARD{}`, `KEYPAD`, `ON BREAK`, `ON MENU`, `ON MENU MESSAGE`, `ON MENU KEY`, `ON MENU IBOX/OBOX`, `TRON/TROFF`, `RESERVE`, `CHAIN`, `OPTION`, `BGET/BPUT`, `BSAVE/BLOAD`, `FILESELECT`, `RC_COPY`, and `RC_INTERSECT`. Their presence in the diskette is a discovery lead, not a complete specification.

## 3. Particularly valuable V2.xx/V3.0 comparisons

### `PCODE_2X.LST` versus `PCODE_30.LST`

This is the strongest paired example in the diskette for version research.

- The V2.xx version uses `PEEK`, `DPEEK`, `LPEEK`, `POKE`, `DPOKE`, and `LPOKE` to read/write memory by width.
- The V3.0 version uses typed dereferencing forms such as `BYTE{address}`, `CARD{address}`, and `{address}`.
- The V3.0 version uses `SELECT/CASE/ENDSELECT` where the V2.xx version uses separate `IF/ENDIF` branches.
- The V3.0 procedure uses `VAR p.frm%` as an output parameter in `pread`; the V2.xx version writes through an address parameter.
- The program's purpose is to serialize memory data into `DATA` statements and restore it.

This pair is suitable for a carefully annotated version-difference note. It is not safe to infer every width, signedness, or pointer rule from the example alone.

### `DUMP_V2X.LST`

The listing explicitly comments on changes needed for V3.0: different file extensions, a larger variable/type table, and additional variable categories for byte/word variables and arrays. It is a useful lead for the tokenized program-file format and variable-table changes, but its hard-coded offsets are version-specific reverse-engineering observations, not a supported public file-format specification.

### `GETDIR.LST`

The listing contains paired V2.xx/V3.0 notes around a `VAR` output parameter versus writing through an address. It also demonstrates recursive procedures, `LOCAL`, `ABSOLUTE`, Atari GEMDOS file-search calls, and typed memory access. The sample is especially useful for separating language-core procedure semantics from Atari filesystem API behavior.

### `RC_COPY.LST`

The file shows a native-looking `RC_COPY ... TO ...` statement in V3.0 and comments with a user-procedure replacement for V2.xx. This is valuable evidence that the command's availability differs by version and that the earlier release used a library/procedure workaround. Verify the exact command signature and version boundary in the printed reference.

## 4. Atari platform API findings for issue #25

The corpus offers a practical index of subsystems and example files:

- **BIOS/XBIOS/GEMDOS:** `ATARI.LST`, `GEMDOS32.LST`, `XBIOS_16.LST`, `XBIOS_17.LST`, `XBIOS_18.LST`, `GETDIR.LST`, `DISK_OPS.LST`.
- **VDI/GDOS:** `GDOS_V2.LST`, `V_OPNWK.LST`, `GETFILL.LST`, `GETLINE.LST`, `GETMARK.LST`, `GETTEXT.LST`, `VDI_TIMR.LST`.
- **AES/GEM:** `AES_ALRT.LST`, `AES_FORM.LST`, `AES_MENU.LST`, `WINDOWS.LST`, `WINDTAB.LST`.
- **Graphics and screen memory:** `GPLANE.LST`, `RASTER.LST`, `SCREEN.LST`, `RC_COPY.LST`, `RC_INTER.LST`, `FONTSET.LST`, `SYSFONT.LST`.
- **Input and events:** `DMOUSE.LST`, `SETMOUSE.LST`, `SETMOUS2.LST`, `STICK.LST`, `WINDOWS.LST`.
- **Sound/timing:** `DOSOUND.LST`, `WAVE.LST`, `TIMER.LST`.

Treat these as an index of candidate API topics. Do not copy signatures into the registry until they are checked against the relevant manual pages. The examples often write directly to operating-system structures and hardware-related memory.

## 5. Safety and execution cautions

Do not execute these historical programs on a modern host or in an emulator without first understanding their side effects.

- `XBIOS_18.LST` explicitly formats the floppy disk in the drive and writes boot/FAT-related structures.
- `V2_PATCH.LST` patches the active interpreter or a `GFABASIC.PRG` executable at hard-coded offsets.
- `SLOW.LST` manipulates interrupt/VBL vectors and memory-resident system state.
- `SYSTEM.S` and `SYSTEM.LST` install/replace font data and alter system vectors.
- `VIRUSKIL.LST` is a system-level utility whose behavior should be reviewed before execution; the filename alone is not proof of its safety or purpose.
- Several disk, boot-sector, memory and direct-I/O examples can overwrite data or destabilize the machine.

For the compiler project, these files should initially be used only for static inspection and command discovery. Any future runtime probe must be minimal, isolated, version-pinned, and run against a disposable emulator disk/image.

## 6. Rights and publication policy

The book PDF's front matter includes an author-approved digitalization notice with conditions for redistribution of the unchanged PDF. **That notice does not establish that the accompanying diskette source files have the same permission.** The book's printed copyright notice reserves rights, and the diskette README does not grant a source-code redistribution licence.

Therefore:

- Keep the original ZIP, extracted listings, binary files and full OCR corpus in the user's local research archive.
- Do not commit the original diskette archive, its complete source files, or bulk verbatim code to this repository.
- Publish concise original summaries, file-name/topic indexes, provenance, printed-page citations and independently authored minimal tests.
- If a specific sample is ever to be redistributed, first establish permission/licence for that sample and retain its provenance.
- Do not present the user's possession of the physical book or archive as a redistribution licence.

## 7. Recommended follow-up work

1. Use the file/topic index above to extract precise page references from the book for each language-core construct and Atari API family.
2. Compare the paired V2.xx/V3.0 examples line by line and add a version-difference matrix to the language inventory (#19).
3. Create independently authored minimal syntax probes for the constructs only after their documented rules have been extracted; link them to the conformance corpus (#42).
4. Use the Atari API families as a discovery checklist for the Atari API reference (#25).
5. Reconstruct incomplete examples only where the book identifies the missing fragments; do not treat reconstruction as permission to republish the original listing.

## 8. Limits of this inspection

This is a static inspection of the supplied archive. It does not claim that the programs were executed, that they compile on every V2.xx/V3.0 build, or that their comments/omitted fragments have been restored. OCR and character decoding can also obscure non-ASCII text. Exact syntax, signatures and semantics must be checked against printed pages and, where necessary, a version-identified interpreter.
