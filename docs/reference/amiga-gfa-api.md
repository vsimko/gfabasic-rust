# Amiga GFA BASIC API reference — initial research inventory

- **Status:** Research draft; not an accepted API profile or implementation contract.
- **Source scope:** Amiga GFA BASIC interpreter 3.0 as described by the French 1989 edition identified as `AM-BOOK-FR-1989`.
- **Tracking:** [issue #24](https://github.com/vsimko/gfabasic-rust/issues/24).
- **Related material:** [Amiga GFA BASIC 3.0 language and API research notes](amiga-gfa-basic-3.0.md), [source register](../reference-sources.md), [API profile architecture](../architecture/API-PROFILES.md), and [proposed API registry model](../architecture/API-REGISTRY.md).

This page begins a separate Amiga-specific reference. It does not claim complete API coverage, exact 3.52 compatibility, Linux runtime support, or conformance. It records only the limited command and platform behavior extracted so far from the identified Amiga 3.0 book. OCR-derived signatures and semantics remain subject to page-image review; entries below must not be treated as implementation-ready declarations.

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

The distinction between sequential byte/line I/O and random/direct-access records is important. `FIELD`, `GET`, `PUT`, and `RECORD` must be described as one related record-layout contract after the original pages have been visually checked. Do not infer byte order, padding, numeric representation, or string layout from the command names.

### 2.3 Keyboard and text input constructs

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
| `TOUCH` | `TOUCH #channel`; assigns the current system date to the file's internal date fields. | p. 5-95 | Book-documented; OCR review pending |

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

## 5. Linux runtime status

The repository currently has no Rust workspace, compiler, runtime, CLI, or CI workflow. Therefore:

- No Amiga API entry is implemented for Linux.
- No host mapping has been accepted for Amiga devices, graphics, windows, menus, events, sound, ports, memory, or Exec operations.
- The planned Linux/SDL3 platform layer is an implementation boundary, not evidence that SDL3 behavior is equivalent to Amiga behavior.
- Unsupported or hardware-dependent operations must eventually produce explicit diagnostics or require an explicit unsafe/capability policy; they must not be silently ignored.
- No conformance result or runtime observation is claimed by this document.

The [current compatibility matrix](../architecture/COMPATIBILITY-MATRIX.md) remains authoritative for project implementation status.

## 6. Next extraction and acceptance work

Before promoting this research draft into an accepted API reference:

1. Check OCR-sensitive signatures, symbols, and tables against the corresponding page images of `AM-BOOK-FR-1989`.
2. Extract exact graphics, bitmap/PUT-string, memory, screen/window, menu, and event declarations with printed-page references.
3. Keep the German `AM-BOOK-DE-1990` edition and its 3.03 scope separate; record cross-checks without merging pagination.
4. Obtain evidence for later releases before making any 3.04, 3.5, 3.51, or 3.52 availability claim.
5. Decide which declarations belong to the language core and which require the Amiga API profile.
6. Add declarations to the proposed API registry only after its schema and ownership decisions are accepted under issue #12.
7. Define independently authored probes and conformance fixtures; preserve source provenance and rights information.
8. Classify Linux runtime support per declaration only after an explicit semantic mapping, implementation, and tests exist.

Do not close issue #24 merely because this initial inventory exists. Complete coverage, source-checked declarations, profile/registry consistency, and explicit runtime support classifications remain outstanding.
