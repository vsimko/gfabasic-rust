---
type: "Reference"
title: "Amiga GFA BASIC API reference — initial research inventory"
---

# Amiga GFA BASIC API reference — initial research inventory

- **Status:** Historical 3.0 research inventory with a source-verified 3.5 platform-profile crosswalk; the canonical structured declarations remain in `api/registry/amiga/`.
- **Source scope:** Historical Amiga GFA BASIC interpreter 3.0 evidence from the French 1989 edition identified as `AM-BOOK-FR-1989`.
- **Target warning:** The historical 3.0 inventory is not the 3.5 API contract. Section 7 identifies the four platform declarations explicitly confirmed by the 3.5 addendum and keeps inherited candidates unpromoted.
- **Tracking:** [issue #24](https://github.com/vsimko/gfabasic-rust/issues/24).
- **Related material:** [Amiga GFA BASIC 3.0 language and API research notes](amiga-gfa-basic-3.0.md), [source register](../reference-sources.md), [API profile architecture](../architecture/API-PROFILES.md), and [proposed API registry model](../architecture/API-REGISTRY.md).

This page preserves the limited command and platform behavior extracted from the historical Amiga 3.0 book and adds a separately version-scoped 3.5 crosswalk in §7. It does not claim complete API coverage, exact 3.52 compatibility, Linux runtime support, or runtime conformance. OCR-derived signatures and semantics in the historical inventory remain subject to page-image review; consult §7 and the canonical TOML registry before treating any entry as a 3.5 declaration.

## 1. Source and version boundaries

The source used for this inventory is *Le livre du GFA Basic Amiga*, by Bleek, Hecht, and Litzkendorf, French translation by Christian Frey, Data Becker / Micro Application, ©1989, ISBN 2-86899-207-2. Its preface states that it describes the GFA BASIC Amiga 3.0 interpreter. The source register records the PDF provenance and the copyright/digitalization notices.

Use printed page numbers from this edition, not PDF viewer page numbers. The German 1990 edition `AM-BOOK-DE-1990` is a separate publication whose preface describes interpreter 3.03 and whose pagination differs. Do not combine the two editions' page references or silently transfer their version claims. Neither edition establishes GFA BASIC Amiga 3.52 behavior.

### Evidence labels used here

| Label | Meaning |
|---|---|
| **Book-documented; OCR review pending** | The feature and summary are present in the searchable text extraction with a printed-page reference. OCR-sensitive punctuation, symbols, and detailed edge cases still require checking against the page image. |
| **Example-backed** | A form appears in a worked example, but the example does not establish a complete signature or all semantics. |
| **TOC-only** | The table of contents establishes that a topic is covered; individual commands and behavior have not yet been extracted. |
| **Secondary example only** | A published Amiga GFA BASIC example demonstrates usage, but it is not a primary reference for a complete command contract. |
| **Unknown / not extracted** | The current sources do not establish the required detail. |

The evidence labels describe source research only. They do not describe compiler implementation maturity or target support.

## 2. Amiga-specific I/O and device behavior

Chapter 5 covers input/output, disk operations, file management, direct-access files, port I/O, the AmigaDOS library, printing, and sound (printed pp. 61–124). The initial inventory below is based on the chapter extraction recorded in the [Amiga 3.0 research notes](amiga-gfa-basic-3.0.md).

Some constructs may belong to the language core rather than the Amiga API profile. The Amiga-specific part is the documented device namespace and platform behavior. The final ownership of each declaration between the language core and the Amiga profile remains open.

### 2.1 File channels and access modes

| Item | Extracted form or behavior | Source | Evidence |
|---|---|---|---|
| `OPEN` | `OPEN mode, #channel, filename[, record_length]`. The book lists modes `O` (output/create or truncate), `I` (input/read), `A` (append), `U` (update/read and write), and `R` (random/direct access). The `R` form describes a direct-access file held in memory, with a total data limit of 65,535 bytes. | pp. 5-91–5-93; 5-97–5-98 | Book-documented; OCR review pending |
| Channel range | Up to 100 simultaneously open files; channel numbers 0–99. The introductory notation says `#` is optional for most file operations, with `IN` and `OUT` exceptions. | pp. 5-92; 1-11–1-12 | Book-documented; OCR review pending |
| Record length | The default direct-access record length is 128 bytes when omitted. The book also states a maximum record length of 65,535 bytes. The combined FIELD layout must not exceed the selected record length. | pp. 5-92–5-93; 5-97–5-98 | Book-documented; OCR review pending |
| Amiga device names | `CON:`, `PRT:`, `PAR:`, `SER:`, `COM:`, `PIPE:`, `SPEAK:`, `RAW:`, `NEWCON:`, and `AUX:` are listed as file-like device interfaces. The full name including the colon is supplied as the filename; the book says the OPEN mode may be omitted for these interfaces. | pp. 5-92–5-93 | Book-documented; OCR review pending |

Device names are AmigaDOS endpoints, not ordinary host file paths. Their availability and behavior depend on the relevant Amiga device handlers. A Linux implementation must not silently reinterpret these names as local files or substitute a host terminal, printer, serial port, pipe, or speech service.

### 2.2 File and direct-access operations

| Construct | Extracted form or behavior | Source | Evidence |
|---|---|---|---|
| `BGET` | `BGET [#]channel, start, count`; reads bytes from the current file position into memory beginning at `start`. The OCR text appears to restrict the opened channel to mode `T` or `U`, but `T` conflicts with the chapter's listed OPEN modes and is not interpreted here pending page-image review. | p. 5-87 | Book-documented; mode glyph unresolved |
| `BPUT` | `BPUT [#]channel, start, count`; writes bytes from memory beginning at `start` to the file's current position. The book describes use with files opened in `O` or `U` mode. | p. 5-87 | Book-documented; OCR review pending |
| `CLOSE` | `CLOSE [#channel]`; the extraction says that bare `CLOSE` closes all channels. | p. 5-88 | Book-documented; OCR review pending |
| `EOF` | `EOF(#channel)`; the book's description reports −1 when the file pointer is at the final byte and 0 otherwise. Empty-file and past-end behavior are not established by the current extraction. | p. 5-88 | Book-documented; edge cases unknown |
| `FIELD` | Two extracted forms: `FIELD #channel, length AS string_var[, length AS string_var...]` and `FIELD #channel, length AT(address)[, length AT(address)...]`. `AS` and `AT` forms may be mixed and continued across multiple program lines for one file. | pp. 5-88–5-89 | Book-documented; OCR review pending |
| `GET` / `PUT` | Extracted forms: `GET #channel[, record_number]` and `PUT #channel[, record_number]`. They read/write a selected direct-access record using the current FIELD layout. If the record number is omitted, the book describes using the next record after the last GET or the record selected by RECORD. | pp. 5-89, 5-93 | Book-documented; OCR review pending |
| `LOC` | `LOC(#channel)`; returns the current file position in bytes from the beginning. The example reports 4 when the pointer is on byte 4; do not assume a host-style zero-based offset without a compatibility probe. | p. 5-90 | Book-documented; OCR review pending |
| `LOF` | `LOF(#channel)`; returns the file length in bytes. | p. 5-90 | Book-documented; OCR review pending |
| `RECORD` | `RECORD [#]channel, record_number`; selects the record for subsequent GET/PUT operations. | p. 5-94 | Book-documented; OCR review pending |
| `RELSEEK` | `RELSEEK #channel, [-] offset`; moves the file position relative to the current position. Negative offsets move toward the beginning. The book warns against selecting nonexistent positions. | pp. 5-94–5-95 | Book-documented; OCR review pending |
| `SEEK` | `SEEK #channel, [-] offset`; a positive value positions from the beginning and a negative value from the end. The book warns against nonexistent positions; boundary behavior needs a runtime probe. | p. 5-95 | Book-documented; OCR review pending |
| `STORE` | `STORE #channel, string_array()[, count]`; writes array elements in order and appends CR/LF for each line. The optional count limits the number of elements; a `from TO to` range form is also described. | p. 5-96 | Book-documented; OCR review pending |
| `RECALL` | `RECALL #channel, string_array(), count, bytes_read_var`; reads lines into the array. The book describes CR/LF control codes as line terminators, stops silently at EOF or when the array is full, and returns the number of bytes read through the numeric variable. A `from TO to` range form is also described. | pp. 5-94–5-95 | Book-documented; OCR review pending |
| `TOUCH` | `TOUCH #channel`; assigns the current system date to the file's internal date fields. | p. 5-95 | Book-documented; OCR review pending |

The distinction between sequential byte/line I/O and random/direct-access records is important. `FIELD`, `GET`, `PUT`, and `RECORD` must be described as one related record-layout contract after the original pages have been visually checked. Do not infer byte order, padding, numeric representation, or string layout from the command names.

### 2.3 Keyboard, text, and byte-channel I/O constructs

These are recorded because the Amiga book describes their interaction with the console and file channels. Their eventual classification as core language constructs or profile-specific behavior is unresolved.

| Construct | Extracted form or behavior | Source | Evidence |
|---|---|---|---|
| `FORM INPUT` | `FORM INPUT length, string_var`; bounded text entry with a documented maximum of 255 characters. An `AS` form edits an existing string from its beginning. | pp. 5-61–5-62 | Book-documented; OCR review pending |
| `INKEY$` | Returns the most recently pressed key as a string; the book discusses function keys, cursor keys, and modifier combinations. | p. 5-62 | Book-documented; edge cases unknown |
| `INPUT` | Reads values into a variable list; comma-separated fields are described, with quoted commas for string data. The book also describes input from an opened file/device channel. | pp. 5-64–5-66 | Book-documented; OCR review pending |
| `INPUT$` | `INPUT$(count[, #channel])`; returns a string of up to the requested character count. The channel form reads from an opened file. | pp. 5-66–5-67 | Book-documented; OCR review pending |
| `LINE INPUT` | Extracted forms: `LINE INPUT [prompt;] string_var` and `LINE INPUT #channel, string_var`; reads a line where commas are text rather than field separators. | p. 5-67 onward | Book-documented; OCR review pending |
| `INP` | `INP(#channel)`; reads one byte from the specified data channel. | p. 5-99 | Book-documented; OCR review pending |
| `OUT` | `OUT #channel, byte1[, byte2...]`; writes byte values to the specified data channel. | p. 5-99 | Book-documented; OCR review pending |

## 3. Initial graphics and window evidence

Chapter 9 covers graphics definitions, primitives, lines/points, graphics operations, PUT-string organization, graphics memory, and animation (printed pp. 199–246). The following limited facts are available from the French 3.0 book.

| Item | Extracted behavior | Source | Evidence |
|---|---|---|---|
| Hires display | The book describes a 640 × 256 pixel coordinate space. | pp. 4-59–4-60 | Book-documented; context-specific |
| Lowres display | The book describes 320 × 256 pixels and mentions 512 vertical points for interlaced mode. | pp. 4-59–4-60 | Book-documented; mode combinations not fully extracted |
| Coordinate origin | In the illustrated normal screen/window context, (0,0) is at the upper-left. The active drawing target and offsets still need to be described. | pp. 4-59–4-60 | Book-documented; context-specific |
| `ELLIPSE` | A four-value example uses x/y position followed by horizontal and vertical radii. The book explains these argument roles. Additional forms, defaults, and target selection are not established here. | pp. 4-59–4-60 | Example-backed |
| `GRAPHMODE` | The book demonstrates graphics-mode state; an example uses mode 1 for XOR drawing and contrasts it with REPLACE behavior. The complete mode enumeration and persistence rules have not been extracted. | pp. 4-59–4-60; 5-114–5-115 | Example-backed |
| `OPENW` / `CLOSEW` | A worked example uses `OPENW 5`, draws in the window, and closes it with `CLOSEW 5`. The book describes the opened object as a small centered BASIC window. This does not establish the full OPENW signature or all lifecycle rules. | pp. 5-114–5-115 | Example-backed |

The page map also identifies window commands (pp. 339–346), screen management (pp. 347–352), menus (pp. 353–358), and BASIC event control (pp. 359–368). These are **TOC-only** at present: do not infer individual command signatures from chapter titles.

### 3.1 Secondary example corroboration

The online Amiga edition of *Computers and Chaos* contains GFA BASIC listings that the author identifies as version 3.0 examples. Its examples demonstrate uses including `PLOT`, `COLOR`, `GRAPHMODE`, `BOX`, `MOUSEK`, `MOUSEX`, `MOUSEY`, `OPENS`, `OPENW`, `TITLEW`, `MENU`, `MENU(0)`, and `ON MENU GOSUB`.

This is a secondary example corpus, not a complete command reference. The book itself reports testing/advice for versions 3.0 and 3.5; it does not establish 3.52 behavior. Do not use these listings to fill missing signatures or to override the primary manual. See the [source register entry AM-EXAMPLES-BOOK](../reference-sources.md).

## 4. Other Amiga subsystems still awaiting extraction

The French 3.0 book's table of contents establishes that the following subjects exist, but this reference does not yet contain complete command declarations for them:

| Area | Printed pages | Current state |
|---|---:|---|
| Disk operations and AmigaDOS library | 76–112 | Chapter map known; command inventory and signatures incomplete |
| Port I/O and sound | 99–124 | Chapter map known; exact operations and safety implications not extracted |
| Arrays, pointers, memory, and user memory management | 259–283 | Chapter map known; address and ownership semantics incomplete |
| Amiga Exec library | 284–300 | Chapter map known; calls, structures, and lifecycle not extracted |
| Program control, date/time, errors, multitasking, and debugging | 301–330 | Chapter map known; profile/core ownership and exact behavior incomplete |
| Windows and screens | 339–352 | TOC-only |
| Menus | 353–358 | TOC-only |
| BASIC event control | 359–368 | TOC-only |
| Later commands and GFA BASIC 3.04 environment | 369–376 | Version-specific extraction required; do not treat 3.04 additions as 3.0 or 3.52 behavior |

Hardware, memory, Exec, port, and device operations require particular care. Their Amiga semantics must be specified independently of the Linux adapter. Do not expose raw Amiga addresses, structures, or side effects as portable Linux behavior.

## 5. Current implementation and Linux target status

This page remains a historical Amiga 3.0 research draft; the repository has since acquired a Rust workspace, an experimental compiler frontend, runtime and Linux platform crates, a CLI build path, and CI workflows. The earlier statement that no workspace/compiler/runtime/CLI/CI exists is obsolete.

The current implementation boundary is narrower than this inventory:

- The canonical API registry is maintained under `api/registry/`; its [README](../../api/registry/README.md) records 24 language-core and four Amiga-profile declarations in the interim 3.5 addendum slice.
- The four Amiga-profile declarations (`PCOLOR`, the negative-screen `OPENW` variant, `GURU`, and `GURU()`) are not implemented. Linux target support remains unknown and conformance remains untested.
- The current compiler/runtime prototype implements only a restricted combinatorics slice (`FACT`, `VARIAT`, `COMBIN`) from the language-core profile. This does not implement the Amiga APIs listed on this page.
- No host mapping has been accepted for Amiga devices, graphics, windows, menus, events, sound, ports, memory, or Exec operations. The existence of a Linux platform crate does not establish semantic equivalence with Amiga behavior or SDL3.
- Hardware-dependent or unavailable operations must eventually produce explicit diagnostics or require an explicit capability policy; they must not be silently ignored.
- This historical 3.0 inventory makes no runtime-conformance claim. Consult the [compatibility matrix](../architecture/COMPATIBILITY-MATRIX.md), [canonical API registry](../../api/registry/README.md), and [3.5 documentation audit](../research/amiga-gfa-basic-documentation-audit.md) for current status.

## 6. Next extraction and acceptance work

Before promoting this research draft into an accepted API reference:

1. Check OCR-sensitive signatures, symbols, and tables against the corresponding page images of `AM-BOOK-FR-1989`.
2. Extract exact graphics, bitmap/PUT-string, memory, screen/window, menu, and event declarations with printed-page references.
3. Keep the German `AM-BOOK-DE-1990` edition and its 3.03 scope separate; record cross-checks without merging pagination.
4. Obtain evidence for later releases before making any 3.04, 3.5, 3.51, or 3.52 availability claim.
5. Decide which declarations belong to the language core and which require the Amiga API profile.
6. Add or revise declarations only in the accepted canonical TOML registry under `api/registry/`, following its schema, profile ownership, provenance, and validation rules.
7. Define independently authored probes and conformance fixtures; preserve source provenance and rights information.
8. Classify Linux runtime support per declaration only after an explicit semantic mapping, implementation, and tests exist.

Do not close issue #24 merely because this initial inventory exists. Complete coverage, source-checked declarations, profile/registry consistency, and explicit runtime support classifications remain outstanding.


## 7. Amiga GFA BASIC 3.5 platform-profile crosswalk

This section reconciles the historical inventory above with the selected interpreter 3.5 target. The normative source is `AM-INTERPRETER-35`: its printed Chapter 13 is explicitly the GFA-BASIC 3.5 addendum. The 3.5 addendum was page-image reviewed; see the [3.5 addendum inventory](../research/amiga-gfa-basic-35-addendum-inventory.md), [source/topic index](../research/amiga-gfa-basic-source-topic-index.md), and [primary-reference decision](../research/amiga-gfa-basic-35-primary-reference.md). Use printed addendum pages, not PDF viewer page numbers.

### 7.1 Source-verified Amiga-profile declarations

The current Amiga registry contains four platform-profile declarations. These are the complete set of platform-specific constructs explicitly established by the reviewed 3.5 addendum; this is **not** a claim that the complete 3.5 language/API surface has been reconciled.

| Construct | 3.5 source contract | Printed source locator | Stable registry ID |
|---|---|---|---|
| `PCOLOR foreground[, background]` | Sets the foreground and optional background colours for screen output subsequently produced by `PRINT`; otherwise described as behaving like `COLOR`. Colour ranges and all screen/window interactions are not specified in this passage. | `AM-INTERPRETER-35`, p. 13-34 | [`gfa.amiga.graphics.pcolor`](../../api/registry/amiga/special-and-platform-commands.toml) |
| `OPENW` with a negative screen number | Opens the window on the current (top) screen. This is only the negative-screen variant, not a complete `OPENW` signature or lifecycle contract. | `AM-INTERPRETER-35`, p. 13-34 | [`gfa.amiga.windows.openw-negative-screen`](../../api/registry/amiga/special-and-platform-commands.toml) |
| `GURU numeric-expression` | Zero disables GURU checking; nonzero enables it. On a GURU Meditation, control transfers to `ONERROR GOSUB` or error #109 is generated. The addendum also records the compiled-program `RESUME NEXT` / `RESUME LABEL` restriction. | `AM-INTERPRETER-35`, p. 13-36 | [`gfa.amiga.system.guru-check`](../../api/registry/amiga/special-and-platform-commands.toml) |
| `GURU` (function) | Returns the GURU number associated with the current condition. The source does not specify the function's return type. | `AM-INTERPRETER-35`, p. 13-36 | [`gfa.amiga.system.guru-number`](../../api/registry/amiga/special-and-platform-commands.toml) |

The four records retain Amiga profile ownership and separate evidence, implementation, Linux support, and conformance. They are not implemented; Linux support is unknown and conformance is not tested. No platform conformance fixture or original-interpreter observation is currently linked to these declarations. Do not infer support from a Linux/SDL feature with a similar name.

### 7.2 3.5 material kept outside the Amiga platform profile

The 3.5 addendum also documents language-level additions. They belong in `language-core/`, not `api/registry/amiga/`: matrix/vector operations, `FACT`, `VARIAT`, `COMBIN`, `_DATA`, `_DATA=`, and `_0`…`_9`. The German book's explicitly 3.5-labelled Chapters 18–19 provide secondary corroboration for matrix/combinatorics and DATA-pointer material, but do not turn those constructs into Amiga platform APIs. See the [canonical registry inventory](../../api/registry/README.md) and [3.5 German-book cross-check](../research/amiga-gfa-basic-35-german-book-crosscheck.md).

The 3.5 editor update on printed p. 1-25 is editor behavior, not an interpreter platform API. The German book's compiler chapter (Chapter 17, printed pp. 405–450) is compiler documentation and is likewise excluded from the interpreter profile.

### 7.3 Historical platform candidates not promoted to 3.5

The following Amiga-specific families are documented in the German 1990 book's general command reference, whose foreword identifies its interpreter reference as version 3.0 and whose introduction also mentions V3.03. They are useful candidates for later compatibility research, but the book's cover wording “bis Version 3.5” is not evidence that each general-reference declaration applies to interpreter 3.5. The 3.5 addendum does not provide a blanket revalidation of base-manual Chapters 2–12.

| Candidate family | German 1990 printed pages | 3.5 disposition |
|---|---:|---|
| AmigaDOS devices, file/disk and channel integration | 75–119 | Historical 3.0/V3.03 candidates; exact 3.5 availability and semantics unresolved |
| AmigaDOS library calls, printer, sound and speech | 120–143 | Historical 3.0/V3.03 candidates; OS/device dependencies and 3.5 availability unresolved |
| Graphics primitives, drawing state and bitmap/PUT operations | 225–278 | Historical 3.0/V3.03 candidates; do not infer 3.5 availability from the 3.5 `PCOLOR` addition |
| Memory, typed access, allocation and pointers | 305–316 | Historical 3.0/V3.03 candidates; address, ownership, safety and 3.5 availability unresolved |
| Amiga Exec tasks, signals, ports, lists, libraries and devices | 317–329 | Historical 3.0/V3.03 candidates; ABI, structure layout, lifecycle and 3.5 availability unresolved |
| Requesters, mouse, joystick and input-device operations | 363–374 | Historical 3.0/V3.03 candidates; hardware/OS dependencies and 3.5 availability unresolved |
| Windows and screens (including the full `OPENW` family) | 375–388 | Historical 3.0/V3.03 candidates; only the negative-screen `OPENW` variant is separately confirmed by the 3.5 addendum |
| Menu construction and Intuition event dispatch | 389–404 | Historical 3.0/V3.03 candidates; event payloads, constants, lifecycle and 3.5 availability unresolved |

These are explicit unresolved version-applicability candidates, not 3.5 declarations. Do not add registry records for them until a 3.5 source passage or a controlled, version-identified interpreter observation establishes applicability. Where the evidence eventually establishes Amiga hardware/OS semantics without a safe Linux equivalent, record the original semantics and classify Linux support independently (including `unsupported` or `unsafe-disabled` where justified).

### 7.4 Platform-area coverage audit

The following audit closes the investigation loop for the areas named in issue #286. The candidate page ranges below are from `AM-BOOK-DE-1990`, whose general interpreter reference is explicitly 3.0/V3.03; they are discovery evidence, **not** 3.5 availability evidence. The normative 3.5 addendum (`AM-INTERPRETER-35`, printed pp. 13-32–13-36) adds only the platform constructs listed in §7.1. Absence from that short addendum is not proof that a historical API was removed; it leaves applicability unresolved.

| Investigation area | Historical candidate surface and source locator | 3.5 disposition | Amiga dependency / Linux boundary |
|---|---|---|---|
| File, disk and device I/O | `OPEN`, channel I/O, directory/file operations; `AM-BOOK-DE-1990`, pp. 75–119 | Candidate only; no declaration promoted | AmigaDOS device handlers, filesystem semantics and channel behavior are not host-file aliases. Linux support is unclassified until 3.5 applicability and semantics are established. |
| AmigaDOS library | DOS library routines for handles, locks, directories, file metadata and process execution; pp. 120–131 | Candidate only; no declaration promoted | Direct OS calls and AmigaDOS data structures; no implicit POSIX mapping. |
| Port I/O | `INP`, `OUT`; p. 119 | Candidate only; no declaration promoted | Hardware/port access is platform-specific. Do not emulate with arbitrary host I/O or mark safe on Linux. |
| Sound and speech | `SOUND`, `WAVE`, `SAY`, speech translation; pp. 139–143 | Candidate only; no declaration promoted | Amiga audio/speech facilities and device availability are unresolved; no SDL/audio equivalence is inferred. |
| Graphics primitives and drawing state | `PLOT`, `LINE`, `DRAW`, `BOX`, `CIRCLE`, `GRAPHMODE` and related drawing operations; pp. 225–256 | Candidate only; `PCOLOR` is the sole separately confirmed 3.5 addition in this area | Original RastPort, pen, drawing-mode and active-target semantics need declaration-level evidence; generic host drawing is not proof of compatibility. |
| Bitmaps, sprites and Blitter objects | `GET`, `PUT`, `SPRITE`, `VSYNC`, `OBJECT.*`, collision operations; pp. 257–278 | Candidate only; no declaration promoted | Bitplanes, hardware sprites, Blitter objects and display timing are Amiga-specific. Linux support may require explicit unsupported/unsafe-disabled classification if later confirmed. |
| Memory and pointers | `PEEK/POKE`, typed access, `MALLOC/MFREE`, `VARPTR`, `ABSOLUTE`, `INLINE`; pp. 305–316 | Candidate only; no declaration promoted | Address width, ownership, lifetime and executable-memory behavior are unresolved. Never expose raw Amiga addresses as portable host pointers. |
| Exec tasks, signals, ports, lists, libraries and devices | Direct Exec routine families; pp. 317–329 | Candidate only; no declaration promoted | Amiga Exec ABI, structures, scheduling and resource lifecycle require original headers/runtime evidence; no POSIX-thread or handle substitution is assumed. |
| Program control, errors, multitasking and debugging | `RUN/STOP`, error handling, time, multitasking and diagnostic operations; pp. 331–361 | Candidate only except the separately confirmed `GURU` command/function in §7.1 | GURU semantics are recorded independently; other control and task APIs remain unverified for 3.5. |
| Requesters and input devices | `ALERT`, `FILESELECT`, mouse, joystick, `STICK/STRIG`; pp. 363–374 | Candidate only; no declaration promoted | Intuition requester and input-device dependencies; no desktop-dialog or generic gamepad equivalence is inferred. |
| Windows and screens | `OPENW/CLOSEW`, window management, `OPENS/CLOSES`, screen modes; pp. 375–388 | Candidate only except the negative-screen-number `OPENW` variant in §7.1 | Window/screen lifecycle, IDCMP messages, flags, bitmaps and screen modes remain unverified as a complete 3.5 contract. |
| Menus and event dispatch | `MENU` family, `MENU()`, `ON MENU`, event handlers and `SLEEP`; pp. 389–404 | Candidate only; no declaration promoted | Intuition event payloads, message masks and handler lifecycle are Amiga-specific and lack 3.5 confirmation. |
| Compiler and editor facilities | Compiler/linker chapter pp. 405–450; separate editor material pp. 477–482 in the German book; explicit 3.5 editor update at `AM-INTERPRETER-35`, p. 1-25 | Explicitly outside the interpreter platform API profile | Compiler tools and editor UI are not runtime Amiga platform declarations. The editor update remains documented separately. |
| Matrix, combinatorics and DATA additions | Matrix/vector operations, `FACT`, `VARIAT`, `COMBIN`, `_DATA`, `_DATA=`, `_0`…`_9`; explicit 3.5 addendum pp. 13-1–13-36 | Confirmed 3.5 language-core scope; excluded from `api/registry/amiga/` | These are language-core constructs, not Amiga OS/platform APIs. Their registry entries and implementation state remain in the language-core profile. |

The candidate lists are intentionally family-level. They are not complete signatures and must not be copied into the Amiga 3.5 registry as declarations. Where a row is only a 3.0/V3.03 candidate, Linux support is **not applicable as a registry classification yet**: first establish the 3.5 contract, then classify its host mapping independently. For hardware/OS-dependent behavior, a later declaration must preserve the Amiga semantics and use `unsupported` or `unsafe-disabled` when no safe equivalent exists; it must never silently substitute a host operation.

### 7.5 Coverage and remaining gaps

- **Verified 3.5 Amiga-profile declarations:** four, all linked above.
- **Verified 3.5 language-core material:** stored separately under `language-core/`; not counted as Amiga APIs.
- **Unresolved inherited candidates:** the platform families in §7.3; absence from the addendum is not evidence of removal.
- **Implementation / Linux support / conformance:** none of the four Amiga declarations is implemented or conformance-tested; Linux support remains unknown.
- **Next evidence needed:** declaration-level 3.5 source or controlled 3.5 interpreter observations for inherited platform candidates, plus independently authored probes for platform behavior. The current 3.52 observations must remain labelled 3.52 and must not be promoted to 3.5.

The canonical structured source is [`api/registry/amiga/special-and-platform-commands.toml`](../../api/registry/amiga/special-and-platform-commands.toml). This prose crosswalk is navigation and rationale, not a second editable declaration inventory.
