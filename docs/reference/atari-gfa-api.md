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

The book's printed tables on pp. 585–596 were checked against page images at high resolution. The following sizes and field offsets are directly visible in those tables. They describe the Atari 68k GEM resource ABI, not host-native Rust structures.

### 6.1 OBJECT record

The OBJECT record contains 11 fields and occupies **24 bytes**:

| Offset | Field | Width | Meaning |
|---:|---|---:|---|
| 0 | OB_NEXT | Word (16-bit) | Next object at the same level; root/sibling termination uses the documented special index |
| 2 | OB_HEAD | Word | First child object, or -1 when no child level exists |
| 4 | OB_TAIL | Word | Last child object, or -1 when no child level exists |
| 6 | OB_TYPE | Word | Object type index |
| 8 | OB_FLAGS | Word | Object attributes |
| 10 | OB_STATE | Word | Object visual/interaction state |
| 12 | OB_SPEC | Long (32-bit) | Type-dependent value or target pointer |
| 16 | OB_X | Word | Relative X coordinate |
| 18 | OB_Y | Word | Relative Y coordinate |
| 20 | OB_W | Word | Width |
| 22 | OB_H | Word | Height |

All object fields are word-aligned. OB_ADR(address,object) returns the byte offset of the selected object relative to the object-tree base; it is not an OBJECT field. The top-level object's coordinates are relative to screen origin; child coordinates are relative to the parent object's origin. [pp. 585–591]

Object types listed in the book include:

| Index | Type | OB_SPEC interpretation |
|---:|---|---|
| 20 | G_BOX | BOXINFO packed 32-bit value |
| 21 | G_TEXT | TEDINFO pointer |
| 22 | G_BOXTEXT | TEDINFO pointer |
| 23 | G_IMAGE | BITBLK pointer |
| 24 | G_PROGDEF | APPLBLK/USERBLK pointer |
| 25 | G_IBOX | BOXINFO packed 32-bit value |
| 26 | G_BUTTON | C-string pointer |
| 27 | G_BOXCHAR | BOXINFO packed 32-bit value |
| 28 | G_STRING | C-string pointer |
| 29 | G_FTEXT | TEDINFO pointer |
| 30 | G_FBOXTEXT | TEDINFO pointer |
| 31 | G_ICON | ICONBLK pointer |
| 32 | G_TITLE | C-string pointer |

The exact spelling and semantics of object-type constants should still be cross-checked against the matching Atari GEM version before implementation. [p. 587]

### 6.2 OB_FLAGS and OB_STATE masks

OB_FLAGS is described as a 9-bit mask. The page tables list:

| Bit | Name | Meaning |
|---:|---|---|
| 0 | SELECTABLE | Object can be selected |
| 1 | DEFAULT | Activated by Return; normally an EXIT object |
| 2 | EXIT | Selection returns control from FORM_DO |
| 3 | EDITABLE | Object can be edited |
| 4 | RBUTTON | Radio-button grouping behavior |
| 5 | LASTOB | Last object in the current tree |
| 6 | TOUCHEXIT | EXIT behavior without waiting for mouse-button release |
| 7 | HIDETREE | Hide this object and descendants from drawing/hit-testing |
| 8 | INDIRECT | OB_SPEC contains a pointer to the actual specification rather than the value itself |

OB_STATE is a 6-bit mask:

| Bit | Name | Meaning |
|---:|---|---|
| 0 | SELECTED | Draw selected/inverted |
| 1 | CROSSED | Draw diagonal cross |
| 2 | CHECKED | Draw check mark |
| 3 | DISABLED | Draw disabled/lightened |
| 4 | OUTLINED | Draw additional outline |
| 5 | SHADOWED | Draw drop shadow |

These masks are page-image checked against pp. 588–589. The descriptions above are concise summaries, not replacements for the GEM object manager's complete drawing rules.

### 6.3 RSC file header

The book labels the resource-file header as **38 bytes**, consisting of 19 16-bit words:

| Offset | Field | Width |
|---:|---|---:|
| 0 | Reserved; value 1 | Word |
| 2 | Object table position | Word |
| 4 | TEDINFO table position | Word |
| 6 | ICONBLK table position | Word |
| 8 | BITBLK table position | Word |
| 10 | Free-string area position | Word |
| 12 | Bound-string area position | Word |
| 14 | Image-data area position | Word |
| 16 | Free-image area position | Word |
| 18 | Object-tree table position | Word |
| 20 | Object count | Word |
| 22 | Tree count | Word |
| 24 | TEDINFO count | Word |
| 26 | ICONBLK count | Word |
| 28 | BITBLK count | Word |
| 30 | String count | Word |
| 32 | Image count | Word |
| 34 | RSC file length | Word |
| 36 | End marker; value 0 | Word |

The names above are normalized English descriptions of the German table. Keep the original word offsets and widths as the normative evidence. [pp. 591–592]

### 6.4 Nested structures

**BOXINFO** is a packed 32-bit value stored directly in OB_SPEC for G_BOX, G_IBOX and G_BOXCHAR. Its fields total 32 bits: fill-pattern color at bits 31–28 (4), fill pattern at 27–25 (3), drawing mode at bit 24 (1), text color at 23–20 (4; G_BOXCHAR only), border color at 19–16 (4), border thickness at 15–8 (8, signed interpretation described by the book), and ASCII character at 7–0 (8; G_BOXCHAR only). The bit diagram is on p. 592; do not replace it with host bitfields.

**TEDINFO** is 28 bytes:

| Offset | Field | Width |
|---:|---|---:|
| 0 | te_ptext | Long |
| 4 | te_ptmplt | Long |
| 8 | te_pvalid | Long |
| 12 | te_font | Word |
| 14 | te_resvd | Word |
| 16 | te_just | Word |
| 18 | te_color | Word |
| 20 | te_resvd2 | Word |
| 22 | te_thickness | Word |
| 24 | te_txtlen | Word |
| 26 | te_tmplen | Word |

The first three fields are pointers to C strings. The book documents font, justification, color, border thickness and string-length fields; some validation characters are explicitly marked unreliable and potentially crash-prone. [pp. 592–593]

**ICONBLK** is 36 bytes:

| Offset | Field | Width |
|---:|---|---:|
| 0 | ib_pmask | Long |
| 4 | ib_pdata | Long |
| 8 | ib_ptext | Long |
| 12 | ib_col | Byte |
| 13 | ib_char | Byte |
| 14 | ib_xchar | Word |
| 16 | ib_ychar | Word |
| 18 | ib_xicon | Word |
| 20 | ib_yicon | Word |
| 22 | ib_wicon | Word |
| 24 | ib_hicon | Word |
| 26 | ib_xtext | Word |
| 28 | ib_ytext | Word |
| 30 | ib_wtext | Word |
| 32 | ib_htext | Word |
| 34 | ib_resvd | Word |

**BITBLK** is 14 bytes:

| Offset | Field | Width |
|---:|---|---:|
| 0 | bi_pdata | Long |
| 4 | bi_wb (bitmap width in bytes) | Word |
| 6 | bi_hl (bitmap height in pixels) | Word |
| 8 | bi_x | Word |
| 10 | bi_y | Word |
| 12 | bi_color | Word |

**APPLBLK/USERBLK** is 8 bytes: a 32-bit code pointer at offset 0 and a 32-bit parameter pointer at offset 4. The book describes it as the callback structure for G_PROGDEF. [p. 595]

**PARMBLK** is 30 bytes:

| Offset | Field | Width |
|---:|---|---:|
| 0 | pb_tree | Long |
| 4 | pb_obj | Word |
| 6 | pb_prevstate | Word |
| 8 | pb_currstate | Word |
| 10 | pb_x | Word |
| 12 | pb_y | Word |
| 14 | pb_w | Word |
| 16 | pb_h | Word |
| 18 | pb_xc | Word |
| 20 | pb_yc | Word |
| 22 | pb_wc | Word |
| 24 | pb_hc | Word |
| 26 | pb_parm | Long |

The final field is a 32-bit application-defined parameter passed to the user drawing routine. [p. 596]

These structures use 16-bit words and 32-bit target pointers. Rust representations must use explicit target-width integer fields and explicit endian conversion. Never store host pointers in these structures or rely on host-native struct layout. Resource-file offsets are offsets within the RSC image and must be relocated/validated separately from runtime addresses.

## 7. AES/VDI parameter blocks

The book's parameter-block tables on pp. 597–601 were checked against page images. These are mutable shared buffers; calls overwrite input/output arrays and control words.

### 7.1 AES arrays and control block

| Name | Element width | Accessor/index semantics |
|---|---:|---|
| ADDRIN, ADDROUT | 4 bytes | V3.0 indexed arrays; index = byte offset DIV 4 |
| GINTIN, GINTOUT | 2 bytes | V3.0 indexed arrays; index = byte offset DIV 2 |
| GCONTRL | 2 bytes | V3.0 indexed array; index = byte offset DIV 2 |
| GB | 4-byte pointers | Pointer block for AES arrays; raw address-based access |

GCONTRL is a 10-byte control block (five 16-bit words):

| Offset | Field | Direction / meaning |
|---:|---|---|
| 0 | Function opcode | Input |
| 2 | GINTIN array length in bytes | Input |
| 4 | GINTOUT array length in bytes | Output |
| 6 | ADDRIN array length, in 4-byte elements | Input |
| 8 | ADDROUT array length, in 4-byte elements | Output |

The GB pointer block contains six 32-bit pointers:

| Offset | Points to |
|---:|---|
| 0 | GCONTRL array |
| 4 | Global array |
| 8 | GINTIN array |
| 12 | GINTOUT array |
| 16 | ADDRIN array |
| 20 | ADDROUT array |

[pp. 597–600]

### 7.2 VDI arrays and control block

| Name | Element width | Accessor/index semantics |
|---|---:|---|
| INTIN, INTOUT | 2 bytes | V3.0 indexed arrays; index = byte offset DIV 2 |
| PTSIN, PTSOUT | 2 bytes | V3.0 indexed arrays; index = byte offset DIV 2 |
| CONTRL | 2 bytes | V3.0 indexed array; index = byte offset DIV 2 |

The VDI CONTRL block is documented with these fields:

| Offset | Field |
|---:|---|
| 0 | Function opcode |
| 2 | PTSIN array length divided by 2 |
| 4 | PTSOUT array length divided by 2 |
| 6 | INTIN array length |
| 8 | INTOUT array length |
| 10 | Optional subfunction identifier |
| 12 | Device/workstation handle |
| 14–28 | Function-dependent information |

The source explicitly describes these as 2-byte fields. Do not confuse this VDI CONTRL block with AES GCONTRL; they are different layouts and use different arrays. [pp. 598–601]

### 7.3 VDIBASE and font header

VDIBASE is a 308-byte copy of VDI workstation state. The book says it is invalid for Mega/Blitter TOS and may contain incorrect values in GFA BASIC V3.0. For standard ROM-TOS it gives a virtual-workstation parameter-block address of 0x578C, also 308 bytes. That absolute address is a ROM/TOS-specific workaround and must not be used as a generic constant. [pp. 602–604]

The block includes current text rotation and clipping state, font-header pointer, fill-pattern state, text style/color, line and marker attributes, VDI handle, available/current font indices, an 88-byte copy of the current font header, text color, user line/fill patterns, vertical text alignment, graphics mode, coordinate mode and clipping rectangle.

The font header includes identifiers, pixel font size, a 32-byte font name, character range, line metrics, character widths, style flags and pointers to horizontal/character offset tables and font data. The book describes the horizontal-offset table as 512 bytes after the 88-byte header. Do not treat any of these pointers as host pointers.

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
3. **Page-image checked:** OBJECT (24 bytes), RSC header (38 bytes), TEDINFO (28 bytes), ICONBLK (36 bytes), BITBLK (14 bytes), USERBLK/APPLBLK (8 bytes), PARMBLK (30 bytes), and OB_FLAGS/OB_STATE masks, pp. 585–596. Runtime ABI and RSC parser validation remain pending.
4. Verify AES/VDI parameter-block lengths and call-specific array layouts from pp. 597–604.
5. Cross-check BIOS/GEMDOS/XBIOS opcode signatures, register conventions and error results against authoritative TOS documentation.
6. Record machine/TOS applicability for absolute addresses, VDIBASE and hardware registers.
7. Create original minimal probes only where a lawful, version-pinned interpreter/TOS environment is available.
8. Promote individual entries into the canonical API registry only after signature, evidence, runtime mapping and test metadata are complete.

## 12. Source and rights

The source is the user-provided sixth revised and expanded edition of *Das große GFA-BASIC-Buch* (1988). This reference contains original summaries and short API identifiers, not a reproduction of the book. Keep the PDF/OCR and diskette source files outside the public repository unless redistribution rights are independently established.
