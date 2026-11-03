# Atari ST GFA BASIC API reference — research draft

- Status: **Initial API inventory; not an accepted implementation contract**
- Platform: Atari ST / TOS / GEM
- Language coverage: GFA BASIC V2.xx and V3.0 as described by the 1988 sixth revised and expanded edition of *Das große GFA-BASIC-Buch*
- Tracking: [issue #25](https://github.com/vsimko/gfabasic-rust/issues/25), [API registry design #12](https://github.com/vsimko/gfabasic-rust/issues/12), [dialect policy #3](https://github.com/vsimko/gfabasic-rust/issues/3), [conformance corpus #42](https://github.com/vsimko/gfabasic-rust/issues/42)
- Source register: [ATARI-GFA-BOOK-1988](../reference-sources.md)
- Companion research: [systematic book review](../research/atari-gfa-book-systematic-review.md), [preliminary language specification](atari-gfa-basic-3.0.md)

## 1. Scope and evidence policy

This document inventories Atari-specific GFA BASIC interfaces. It does not define portable GFA BASIC core semantics and does not claim that the project implements these interfaces.

The source describes Atari ST GFA BASIC V2.xx/V3.0. Do not transfer these declarations to Amiga GFA BASIC or later Atari releases without separate evidence.

Evidence labels:

- **Book-documented** — the named interface or behavior appears in the identified book passage.
- **OCR/page check pending** — exact spelling, signature, bit field, or offset needs comparison with the printed page image.
- **Runtime-unverified** — no matching interpreter/TOS observation has been recorded.
- **Unsafe/OS-specific** — direct memory, privilege, interrupt, disk, or hardware behavior must be isolated and disabled unless a machine/TOS profile explicitly supports it.

A book description is evidence of documented behavior, not proof that every TOS/GEM release behaves identically.

## 2. API profile boundary

The API profile includes interfaces that depend on Atari GEM, AES, VDI/GDOS, TOS, BIOS, GEMDOS, XBIOS or machine hardware. The source-language dialect/version, API profile, host and output target remain separate configuration dimensions.

Do not map these interfaces directly to Linux system calls or SDL3. A Linux implementation must emulate the documented Atari semantics through project-owned interfaces and an explicit target memory/device model.

## 3. GFA-level GEM/AES wrappers

These commands are documented in the book as GFA BASIC facilities layered over GEM/AES. Exact behavior remains version-scoped.

| Name | Documented purpose | Version / source | Notes |
|---|---|---|---|
| `ALERT icon,boxText$,defaultButton,buttonText$,backVar` | Display a GEM alert with up to three buttons | V2.xx/V3.0, pp. 453–455 | Text/button limits and separators are documented; validate exact return values before implementation |
| `FORM_ALERT(...)` | Call the GEM form-alert facility | V3.0, pp. 455–456 | V2.xx wrapper example uses raw GEMSYS/AES structures; do not assume wrapper and V3.0 call have identical contracts |
| `FILESELECT "path","selection",backVar$` | Open the GEM file selector | V2.xx/V3.0, pp. 456–458 | Distinguish selection, empty field and cancel outcomes; exact output semantics need a dedicated test |
| `MOUSE x,y,buttons` | Read pointer coordinates and button mask | V2.xx/V3.0, pp. 458–460 | Left/right buttons are bit-coded; V3.0 permits negative coordinates outside the relevant window/clip origin |
| `MOUSEX`, `MOUSEY`, `MOUSEK` | Read current pointer coordinates and button state | V2.xx/V3.0, pp. 458–460 | Keep coordinate space and button semantics explicit |
| `SETMOUSE x,y[,button]` | Move pointer and optionally simulate a button | V3.0, pp. 460–461 | AES limitations apply |
| `STICK mode` | Select mouse or joystick mode | V3.0, pp. 461–463 | Restore mouse mode before GEM/AES calls and before program exit |
| `STICK(port)` | Read joystick direction bits | V3.0, pp. 462–463 | V2.xx example uses direct hardware access; fire-button handling differs |
| `STRIG(port)` | Read joystick fire-button state | V3.0, p. 463 | Hardware/profile-specific |
| `CLEARW [#]number` | Clear a GEM window after movement/resize | V3.0, pp. 471–473 | Application is responsible for redraw/restoration |
| `CLOSEW [#]number` | Close a GEM window | V3.0, pp. 473–474 | Window resource lifecycle |
| `FULLW [#]number` | Enlarge window to usable desktop bounds | V3.0, pp. 474–475 | Excludes menu-bar area |
| `INFOW [#]number,"text"` | Set a window information line | V3.0, pp. 475–476 | Must be configured before OPENW; changing it requires close/reopen |
| `OPENW number[,xcenter,ycenter]` | Open/activate a predefined window | V3.0, pp. 476–478 | Center parameters control intersection of predefined windows |
| `OPENW #number,x,y,width,height,attributes` | Define and open a GEM window | V3.0, pp. 477–479 | Attribute mask needs page-image verification |
| `TITLEW [#]number,"text"` | Set a window title | V3.0, pp. 479–481 | Special title strings affect title/resize behavior |
| `TOPW #number` | Activate an existing window | V3.0, pp. 481–482 | Does not re-register the window |
| `W_HAND(number)`, `W_INDEX(handle)` | Convert between GFA window number and GEM handle | V3.0, pp. 482–484 | Keep GFA and AES handle namespaces distinct |
| `WINDTAB`, `WINDTAB(number,index)` | Access the GFA window control table | V3.0, pp. 484–486 | Table layout and indexed accessor need page-image verification |
| `MENU item,attribute` | Change checkmark/enablement state of a menu item | V2.xx/V3.0, pp. 487–489 | OCR suggests four attribute actions; exact numeric values require page-image check |
| `MENU array$()` | Install a pull-down menu from a string array | V2.xx/V3.0, pp. 487–490 | Array grammar includes menu/title/item separators and terminators |
| `MENU KILL`, `MENU OFF` | Disable menu or restore title display | V2.xx/V3.0, pp. 489–490 | Distinguish disabled menu from removed menu data |
| `MENU(index)` | Read current AES event/message data | V2.xx/V3.0, pp. 491–494 | Indexed event vector; full layout below |
| `ON MENU [time]` | Poll AES multi-event queue | V2.02/V3.0, pp. 494–496 | Must be called frequently; message buffer can be overwritten |
| `ON MENU GOSUB procedure` | Install menu-selection handler | V2.xx/V3.0, pp. 494–497 | Event-driven control flow |
| `ON MENU BUTTON ... GOSUB procedure` | Install mouse-button handler | V2.xx/V3.0, pp. 496–497 | Parameter interactions are explicitly described as difficult to predict; runtime probe required |
| `ON MENU IBOX ... GOSUB procedure`, `ON MENU OBOX ... GOSUB procedure` | Detect pointer entering/leaving screen rectangles | V2.xx/V3.0, pp. 497–498 | Rectangles are absolute screen coordinates |
| `ON MENU KEY GOSUB procedure`, `ON MENU MESSAGE GOSUB procedure` | Handle keyboard and AES message events | V2.xx/V3.0, pp. 497–498 | Preserve event ordering and buffer lifetime |

### 3.1 Menu event vector

The book documents `MENU(index)` as an event/message vector:

| Index | Meaning |
|---:|---|
| -2 | Address of AES message buffer |
| -1 | Address of current object tree |
| 0 | Menu item index |
| 1 | Event/message type |
| 2 | VDI handle |
| 3 | Message length |
| 4 | Window handle/title index |
| 5–8 | Coordinates, dimensions or slider values depending on event |
| 9 | Event flags (keyboard, mouse button, IBOX/OBOX, message, timer) |
| 10–11 | Mouse coordinates |
| 12 | Mouse button state |
| 13 | Modifier-key state |
| 14 | Packed keyboard code |
| 15 | Click count |

The book lists event types including WM_SELECTED, WM_REDRAW, WM_TOPPED, WM_CLOSED, WM_FULLED, WM_ARROWED, WM_HSLID, WM_VSLID, WM_SIZED, WM_MOVED, WM_NEWTOP, AC_OPEN and AC_CLOSE. Exact numeric values and event payload interpretation must be checked against the page image and TOS/AES reference before being made normative.

## 4. GDOS and VDI

GDOS is an optional resident Atari service, normally installed at boot and configured through ASSIGN.SYS. Most functions in the chapter require GDOS; the book identifies VQT_EXTENT and VQT_NAME as exceptions. Device drivers can represent screen, plotter, printer, metafile, camera and graphics-tablet devices. [pp. 499–502]

| Name | Documented purpose | Source | Notes |
|---|---|---|---|
| `V-H` | Read/set current VDI handle | V3.0, p. 503 | Special values distinguish GFA-managed and explicitly selected handles |
| `V_CLRWK()` | Clear output buffer/current workstation | p. 504 | Printer use also emits a form feed |
| `V_CLSWK()` | Close workstation opened by V_OPNWK | p. 505 | Flushes pending output and restores GFA handle |
| `V_OPNWK(driverId,...attributes...)` | Open physical output workstation | pp. 507–511 | Driver ID comes from ASSIGN.SYS; exact full parameter list needs page-image transcription |
| `V_OPNVWK(...)` | Open virtual screen workstation | pp. 505–507 | Returns handle; V2.xx procedure form requires explicit parameter setup |
| `WORK_OUT(index)` | Read workstation output capabilities | pp. 505–507 | Documented output indices 0–56 |
| `VST_LOAD_FONTS(0)` | Load additional GDOS fonts | p. 515 | Returns count loaded; memory reservation may be needed |
| `VQT_NAME(index,name$)` | Query VDI font ID and name | p. 514 | Book describes fallback behavior without GDOS |
| `VQT_EXTENT(text$[,coordinates...])` | Measure text bounding quadrilateral | pp. 512–513 | Results are returned through PTSOUT and depend on current text attributes |

Workstation handles, capabilities and lifecycle must be explicit. Do not assume a printer/plotter workstation behaves like the screen. The book reports a RESERVE error in one tested environment when closing a newly opened workstation without first issuing graphics output; treat this as a compatibility warning, not a universal VDI rule. [p. 511]

The font-header and direct TOS/GEM font-pointer material on pp. 516–530 is low-level and OS-specific. It includes resident-memory and supervisor-level patches, including one patch limited to a specific 1986 ROM-TOS build. Do not treat these as safe general APIs.

## 5. AES function families

The book describes approximately 70 AES functions. V3.0 exposes AES calls as callable functions; V2.xx equivalents are shown where available. Calls exchange integer and address arrays through AES parameter blocks. Address parameters require 4-byte values; coordinate/value parameters require at least 2-byte integers. [pp. 547–549]

| AES manager | Book-documented function names | Source pages |
|---|---|---:|
| Application | APPL_INIT, APPL_EXIT, APPL_READ, APPL_WRITE; application/event recording calls where supported | 548–551 |
| Event | EVNT_BUTTON, EVNT_DCLICK, EVNT_KEYBD, EVNT_MESAG, EVNT_MOUSE, EVNT_MULTI, EVNT_TIMER | 551–554 |
| Form | FORM_ALERT, FORM_BUTTON, FORM_CENTER, FORM_DIAL, FORM_DO, FORM_ERROR, FORM_KEYBD, FORM_MOUSE | 554–559 |
| Graphics | GRAF_HANDLE, GRAF_MKSTATE, GRAF_MOUSE, GRAF_MOVEBOX, GRAF_GROWBOX, GRAF_SHRINKBOX, GRAF_RUBBERBOX, GRAF_SLIDEBOX, GRAF_WATCHBOX | 559–564 |
| Menu | MENU_ICHECK, MENU_IENABLE, MENU_REGISTER, MENU_TEXT, MENU_TNORMAL, MENU_BAR | 564–566 |
| Object | OBJC_ADD, OBJC_DELETE, OBJC_DRAW, OBJC_EDIT, OBJC_FIND, OBJC_OFFSET, OBJC_ORDER, OBJC_CHANGE and related calls | 566–569 |
| Resource | RSRC_FREE, RSRC_GADDR, RSRC_LOAD, RSRC_OBFIX, RSRC_SADDR | 569–571 |
| Scrap | SCRP_READ, SCRP_WRITE | 571–572 |
| Shell | SHEL_FIND, SHEL_GET, SHEL_PUT, SHEL_READ, SHEL_WRITE and related calls | 573–574 |
| Window | WIND_CALC, WIND_CREATE, WIND_DELETE, WIND_FIND, WIND_GET, WIND_OPEN, WIND_CLOSE, WIND_SET, WIND_UPDATE | 575–580 |

This is a family-level index, not a complete signature catalogue. Each AES function needs its own normalized signature, parameter-block layout, return semantics, version availability and tests before registry inclusion.

Important constraints:

- Event manager calls can block while waiting for user input. Integrate them with the runtime event pump and timer model.
- Form and menu calls require valid resource/object trees and correct object indices/flags.
- Resource memory is allocated by GEM and must be released by the matching resource operation.
- WIND_UPDATE locking must be paired correctly; incorrect pairing can block redraws by other applications/accessories.
- The book reports unreliable behavior for some AES calls, including event recording and resource pointer adjustment. Record these as source-specific warnings pending TOS-version tests.

## 6. GEM object and resource structures

The book documents a fixed-size OBJECT record linked by object indices. The fields include OB_NEXT, OB_HEAD, OB_TAIL, type, flags, state, specification and relative geometry. [pp. 585–591]

GFA field accessors documented in the book:

- OB_ADR(address,object): byte offset/address of an object in its tree.
- OB_NEXT, OB_HEAD, OB_TAIL: sibling/child links.
- OB_TYPE: object type.
- OB_FLAGS: 9-bit attribute mask.
- OB_STATE: 6-bit visual/interaction state mask.
- OB_SPEC: type-dependent value or pointer.
- OB_X, OB_Y, OB_W, OB_H: relative geometry.

Nested structures include BOXINFO, TEDINFO, ICONBLK, BITBLK, G_IMAGE, G_ICON and PARMBLK/application-object data blocks. [pp. 592–596]

These are 68k ABI structures with 16-bit words and 32-bit pointers. Use explicit target-width fields and documented offsets. Do not use host pointers or native Rust struct layout. Exact flags, field offsets, structure sizes and type-dependent OB_SPEC interpretations remain page-image validation tasks.

## 7. AES/VDI parameter blocks

The book documents these shared arrays and control blocks: [pp. 597–604]

| Name | Subsystem | Purpose |
|---|---|---|
| ADDRIN, ADDROUT | AES | Address input/output arrays; 4-byte pointer entries |
| GINTIN, GINTOUT | AES | Integer input/output arrays; 2-byte entries |
| GCONTRL | AES | Control block: opcode and integer/address array counts |
| INTIN, INTOUT | VDI | Integer input/output arrays |
| PTSIN, PTSOUT | VDI | Point input/output arrays |
| CONTRL | VDI | Control block: opcode, array lengths, subfunction and workstation handle |
| VDIBASE | VDI | Copy of workstation state, including text/line/fill attributes |

V3.0 exposes indexed array accessors as well as raw address forms. These are transient shared call buffers and can be overwritten by later AES/VDI calls. Exact lengths are call-specific. VDIBASE is reported unreliable on some Mega/Blitter TOS configurations and in GFA V3.0; the book also gives a ROM-TOS-specific fallback address. Do not use that hard-coded address without an exact machine/OS profile.

## 8. BIOS, GEMDOS, VDISYS and XBIOS

The book documents generic `BIOS(opcode,...)`, `GEMDOS(opcode,...)`, `VDISYS(opcode,...)` and `XBIOS(opcode,...)` entry points. Longword arguments require the `L:` marker; unmarked arguments are passed as words. Results are returned through the 68k D0 register. [pp. 605, 608, 618]

### 8.1 BIOS examples

- BIOS(2): console input.
- BIOS(3): console output.
- BIOS(4): sector read/write.
- BIOS(7): drive parameter block lookup.

The disk parameter block contains sector size, cluster geometry, directory/FAT layout and data-area start. [pp. 605–608]

### 8.2 GEMDOS examples

| Opcode | Book-described operation | Source |
|---:|---|---:|
| 0 | Process termination; book recommends GFA QUIT/SYSTEM instead | p. 608 |
| 1, 6–11 | Console/key input, output and status | pp. 608–610 |
| 14 | Set default drive | p. 611 |
| 32 | Enter/leave supervisor mode | pp. 611–612 |
| 42–45 | Date/time get/set in packed bit formats | pp. 612–614 |
| 54 | Drive free-space information | p. 614 |
| 61–64 | File open, close, read and write | pp. 614–616 |
| 67–70 | File date/time, attributes and handle operations | pp. 616–617 |
| 75–79 | Process execution/termination and directory search | pp. 617–618 |

Raw GEMDOS handles and buffers are not the same as GFA BASIC channel abstractions. Supervisor mode and direct process termination are privileged operations. Raw disk access and formatting must not be mapped to host filesystem operations without an explicit compatibility and safety model.

### 8.3 VDISYS

VDISYS invokes VDI through the shared parameter blocks. The book demonstrates queries for polyline/text attributes, escape functions (cursor position, mouse availability and hardcopy) and manipulation of interrupt-related VDI state. [pp. 618–623]

### 8.4 XBIOS

The chapter covers mouse initialization, screen resolution/base switching, floppy sector I/O and formatting, MIDI, RS232 buffers/configuration, keyboard translation tables, random values, sound, MFP interrupts/timers and hardcopy. Examples include XBIOS(4) GETREZ, (5) SETSCREEN, (8) FLOPREAD, (12) MIDIWS, (15) RSCONF, (17) RANDOM and (36) PRTBLK. [pp. 623–656]

The diskette structure/formatting example and boot-sector virus discussion on pp. 630–645 are historical low-level material, not safe generic disk guidance. MFP, sound-chip, MIDI, interrupt and printer operations on pp. 646–656 depend on Atari hardware and TOS versions.

## 9. System variables and absolute addresses

Chapter 22 lists absolute TOS/Atari addresses for memory controller/reset vectors, physical/user memory bounds, floppy timing, system timer/keyboard configuration, screen/video pointers, interrupt vectors and saved processor state. [pp. 657–664]

Direct POKE changes to floppy timing, interrupt handlers and BASIC Break behavior are highly version-specific and can crash the system. Classify these as unsafe/unsupported by default. Any emulation requires an explicitly selected Atari machine/TOS profile and safe virtualized memory model.

## 10. Runtime support classification

Use these categories per individual API entry. Do not assign one blanket support status to an entire subsystem.

| Classification | Meaning |
|---|---|
| Native-compatible | Same semantics can be implemented directly by a documented host capability without exposing host-specific behavior |
| Emulated | Atari behavior is reproduced by project-owned runtime code |
| Adapter-backed | A host service is used behind an Atari-compatible semantic adapter |
| Partial | Only a documented subset is supported |
| Unsupported | Not implemented for the selected target/profile |
| Unsafe/disabled | Requires privileged, raw hardware, arbitrary address or destructive disk behavior and is disabled by default |
| Unknown | Evidence is insufficient to classify |

No API entry in this document is currently claimed as implemented or runtime-conformant.

## 11. Validation backlog

1. Inspect page images for VDI signatures, font header offsets, WINDTAB layout, menu attribute values and AES event fields.
2. Transcribe complete AES signatures and V2.xx/V3.0 availability from pp. 547–580.
3. Verify OBJECT, TEDINFO, ICONBLK, BITBLK and PARMBLK sizes/offsets and all flag/state bits from pp. 585–596.
4. Verify AES/VDI parameter-block lengths and call-specific array layouts from pp. 597–604.
5. Cross-check BIOS/GEMDOS/XBIOS opcode signatures, register conventions and error results against authoritative TOS documentation.
6. Record machine/TOS applicability for absolute addresses, VDIBASE and hardware registers.
7. Create original minimal probes only where a lawful, version-pinned interpreter/TOS environment is available.
8. Promote individual entries into the canonical API registry only after signature, evidence, runtime mapping and test metadata are complete.

## 12. Source and rights

The source is the user-provided sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (1988). This reference contains original summaries and short API identifiers, not a reproduction of the book. Keep the PDF/OCR and diskette source files outside the public repository unless redistribution rights are independently established.
