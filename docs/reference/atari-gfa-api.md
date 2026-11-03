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

### 4.1 GDOS availability, workstation lifecycle and font calls

The book's GDOS chapter is specific to the Atari ST environment and to the Digital Research GDOS releases named by the author. Except for `VQT_EXTENT()` and `VQT_NAME()`, the chapter's functions require GDOS to be resident. The author recommends GDOS 1.8 over 1.1 and notes that a valid `ASSIGN.SYS` must be found at boot. [pp. 499–502]

The `ASSIGN.SYS` driver identifiers documented in the example are 01 (screen virtual workstation only), 11 (plotter), 21 (printer), 31 (metafile), 41 (camera) and 51 (graphics tablet). The screen pseudo-drivers 02/03/04 represent low/mid/high resolution font lists. Driver IDs are configuration identifiers, not portable device classes. [pp. 500–502]

| GFA BASIC function | Signature | Documented contract |
|---|---|---|
| `GDOS?` | `GDOS?` | Returns TRUE (-1) if GDOS 1.1+ is resident, otherwise FALSE (0). No V2.xx equivalent is documented. [p. 502] |
| `V-H` | Read: `V-H`; write: `V-H = handle` | Reads/sets the active VDI handle. Nonnegative values select a handle; -1 restores the handle assigned internally to GFA BASIC. [p. 503] |
| `V_CLRWK()` | `V_CLRWK()` | Clears the selected workstation output buffer; on printer workstation also emits form feed. [p. 504] |
| `V_CLSVWK()` | `V_CLSVWK()` | Closes a virtual screen workstation and restores GFA's VDI handle. GFA's own screen virtual workstation normally makes this unnecessary. [pp. 504–505] |
| `V_CLSWK()` | `V_CLSWK()` | Flushes pending output, closes the selected physical workstation and restores GFA's handle. Close any separately opened virtual workstation first. [p. 505] |
| `V_OPNVWK(driverId[,lineType,lineColor,markerType,markerColor,textStyle,textColor,fillStyle,fillPattern,fillColor,coordinateFlag])` | Returns VDI handle | Opens/configures a virtual screen workstation. Output is direct/unbuffered; `V_UPDWK()` is not needed. GFA already opens its own screen workstation. [pp. 505–506] |
| `V_OPNWK(driverId[,lineType,lineColor,markerType,markerColor,textStyle,textColor,fillStyle,fillPattern,fillColor,coordinateFlag])` | Returns VDI handle | Opens the GDOS driver selected by `ASSIGN.SYS`; output is buffered until update/close. The ten optional values initialize VDI attributes; defaults are 1,1,1,1,1,1,1,1,1,2. Calling without resident GDOS or with an invalid ID is documented to crash the system. [pp. 507–509] |
| `V_UPDWK()` | `V_UPDWK()` | Sends the current workstation output buffer to the device. Not needed for direct virtual-screen output. [p. 512] |
| `VQT_EXTENT(text$[,xlo,ylo,xro,yro,xru,yru,xlu,ylu])` | Returns text extent; eight optional coordinate outputs | Measures the bounding quadrilateral using current GEM text attributes. Coordinates are also returned in PTSOUT(0–7); upper-left is (0,0), lower-right gives width/height. Available without GDOS according to the chapter. [pp. 512–513] |
| `VQT_NAME(index,name$)` | Returns VDI font face; fills name$ | Looks up the VDI face/name for a font in the most recently loaded GDOS font list. Without GDOS, returns the system-font face/name. [p. 514] |
| `VST_LOAD_FONTS(0)` | Returns number of fonts loaded | Loads fonts listed for the active resolution in `ASSIGN.SYS`; may allocate memory and require prior `RESERVE`. [p. 515] |
| `VST_UNLOAD_FONTS(0)` | Returns status (not specified precisely) | Unregisters fonts loaded by VST_LOAD_FONTS and attempts to release their memory. [p. 515] |

The V2.xx examples use VDISYS opcodes 3 (clear workstation), 101 (close virtual workstation), 2 (close workstation), 100 (open virtual workstation), 1 (open workstation), 4 (update workstation), 116 (text extent), 130 (font name), 119 (load fonts) and 120 (unload fonts). These are book-documented wrapper examples; they must not be assumed to be the full or universal VDI opcode contract. [pp. 504–515]

The V2.xx `V_OPNVWK`/ `V_OPNWK` procedures require all eleven integer setup parameters plus a pointer to an integer result variable. The V3.0 form makes the ten attribute values optional. The book's examples construct the VDI INTIN array explicitly and return the device handle from CONTRL(6). [pp. 506–507, 511]

### 4.2 Workstation and font lifecycle constraints

- Keep the GFA screen handle, virtual workstation handles and GDOS physical workstation handles distinct. `V-H=-1` switches back to GFA's own screen output even while another workstation remains open. [p. 503]
- For a physical workstation, reserve enough memory for its output buffer. The book estimates at least one bit per output pixel plus approximately 20% overhead; high-resolution printer pages can require over 1 MB. [pp. 509–510]
- Flush buffered output with `V_UPDWK()` before closing. The author reports RESERVE errors if a workstation is opened and closed without drawing, and internal GEM problems if buffered drawing is closed without update. This is a test observation, not a universal VDI guarantee. [p. 511]
- Load fonts only after confirming the relevant GDOS configuration. Query each font with `VQT_NAME`, select its face through `DEFTEXT`, and unload fonts before restoring BASIC memory/terminating. [pp. 510–515]
- The `VQT_NAME` V2.xx example reads GEM's original INTOUT buffer through an absolute address. Treat that implementation detail as version-/memory-layout-specific, not as a portable API. [p. 514]

### 4.3 Own-font material (pp. 516–530)

The remainder of the chapter describes loading custom 8×16 system fonts, changing TOS/GEM font pointers, converting Degas and STAD/OLIFONT font data, and a custom GFA-specific font format. It includes supervisor-/memory-address-dependent patches and direct writes to system font pointers (notably addresses around 0x607E and 0x2924). These examples are historical Atari-specific techniques, not safe general GFA BASIC APIs. They must be classified as unsafe/disabled unless a future Atari machine/TOS profile provides a fully virtualized memory model and explicit compatibility evidence.

The source describes these file layouts:
- Degas high-resolution font: 2,050 bytes (2-byte header plus 2,048 bytes for 128 glyphs × 16 scanlines).
- STAD/OLIFONT high-resolution font: 4,096 bytes (256 glyphs × 16 scanlines).
- 8×16 system font: 4,698 bytes (602-byte header plus 4,096 glyph bytes).
These are source-described file formats; do not infer that all Atari font loaders accept them interchangeably. [pp. 528–530]

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

### 5.1 Signature notes transcribed from pp. 548–580

The following signatures and contracts are transcribed from the book's AES chapter. This is a **first-pass OCR/text transcription**, not yet page-image-verified. Names and argument order are therefore useful research data, but must not be promoted into normative registry entries until checked against page images and, where necessary, a matching AES/TOS reference. The book explicitly says that its V3.0 call syntax follows C-style calls; the `<<` marks in printed signatures denote output parameters and are editorial notation, not BASIC syntax. [pp. 547–548]

**Application manager**

| Function | Signature / result | Documented behavior and constraints |
|---|---|---|
| `APPL_EXIT()` | Returns 0 on failure, nonzero on success | Deregisters the application; the book says this is unnecessary in GFA because startup/shutdown handling is automatic. [p. 548] |
| `APPL_FIND(programName)` | Returns application ID; -1 if not found | Name must be exactly eight characters, padded with spaces if needed; longer input may crash. [pp. 548–549] |
| `APPL_INIT()` | Returns the GFA application ID | Dummy wrapper: GFA already performed initialization at startup. [p. 549] |
| `APPL_READ(handle,length,address)` | Returns 0/nonzero status | Reads a byte count from another application's AES message buffer; destination must be at least `length` bytes. [p. 549] |
| `APPL_TPLAY(address,count,speed)` | Return value documented as always 1 | Replays recorded events; book restricts it to Blitter TOS and gives speed values 50 = half speed, 100 = normal, 200 = double speed. [pp. 549–550] |
| `APPL_TRECORD(address,count)` | Returns number of events actually recorded | Blitter-TOS-only; six-byte records; the author reports Digital Research implementation defects and poor reliability. [p. 550] |
| `APPL_WRITE(handle,length,address)` | Returns 0/nonzero status | Writes bytes to another application's AES message buffer. [p. 551] |

**Event manager**

| Function | Signature | Contract |
|---|---|---|
| `EVNT_BUTTON(clicks,button,status[,mouseX,mouseY,mouseButtons,shift])` | Returns actual click count | Waits for a mouse-button event. Button: 1 left, 2 right, 3 both; status selects which button state triggers. Optional outputs return pointer/button/modifier state. [p. 551] |
| `EVNT_DCLICK(doubleClick,mode)` | Returns previous click interval | Mode 0 reads current setting; mode 1 sets it. Values are documented as 0 (slow) through 4 (fast). [p. 552] |
| `EVNT_KEYBD()` | Returns packed 16-bit key value | Low byte is ASCII; high byte is scan code. [p. 552] |
| `EVNT_MESAG(address)` | Return value documented as always 1 | Waits for a window/menu message in a 16-byte (8-word) buffer. Can wait forever if no installed menu/window can generate a message. [p. 552] |
| `EVNT_MOUSE(flag,x,y,width,height[,mouseX,mouseY,mouseButtons,shift])` | Return value documented as always 1 | Waits for pointer to enter (0) or leave (1) a screen rectangle. Optional outputs match EVNT_BUTTON. [p. 553] |
| `EVNT_MULTI(mask,clicks,button,status,box1...,box2...,address,time[,outputs])` | Returns event bitmask | Combines keyboard, button, two rectangle, message and timer waits. The printed table itself duplicates bit 0 for the timer event (bit 0 is also assigned to keyboard). This is a source-level inconsistency confirmed on the page image; do not silently infer a corrected bit position. [pp. 553–554] |
| `EVNT_TIMER(milliseconds)` | Return value documented as always 1 | Blocks until the requested number of milliseconds elapses. [p. 554] |

**Form and file-selector managers**

| Function | Signature | Contract |
|---|---|---|
| `FORM_ALERT(defaultButton,text$)` | Returns selected button number | Alert format encodes icon, up to five text lines (30 chars each), and up to three buttons (10 chars each), separated by `|` inside bracketed components. [pp. 554–555] |
| `FORM_BUTTON(tree,object,clicks[,nextObject])` | Returns 0 when an EXIT/TOUCHEXIT object is selected, otherwise positive | Internal helper used by FORM_DO; optional output gives next object. [p. 555] |
| `FORM_CENTER(tree[,x,y,width,height])` | Return value documented as always 1 | Calculates centered form bounds; coordinates and dimensions are output parameters. [p. 555] |
| `FORM_DIAL(mode,littleRect...,bigRect...)` | Returns 0/nonzero status | Mode 0 saves background; 1 grow-box; 2 shrink-box; 3 restores background. Window contents still require application redraw. [pp. 555–556] |
| `FORM_DO(tree,firstObject)` | Returns selected EXIT object index | Runs the form interaction loop. A form without EXIT/TOUCHEXIT can trap control indefinitely; normal GFA abort/menu handling is unavailable while it owns input. [p. 556] |
| `FORM_ERROR(code)` | Return value documented as 0 | Displays a TOS error alert. [p. 557] |
| `FORM_KEYBD(tree,current,next,char$[,nextObject,nextChar$])` | Returns 0 on EXIT/TOUCHEXIT, otherwise positive | Internal form-editing helper; output values identify the next editable object/character. [p. 557] |
| `FSEL_INPUT(path$,file$,button)` | Returns 0/nonzero status | Creates a file selector. Outputs chosen path, file and exit object (0 cancel, 1 accept). The book warns that malformed string buffers can corrupt GEM state and recommends the safer GFA FILESELECT wrapper. [pp. 557–558] |

**Graphics, menu and object managers**

| Function | Signature | Contract |
|---|---|---|
| `GRAF_DRAGBOX(boxW,boxH,startX,startY,boundsX,boundsY,boundsW,boundsH[,lastX,lastY])` | Returns 0/nonzero status | Drag operation requires left mouse button held; final position is output. [pp. 558–559] |
| `GRAF_GROWBOX(smallRect,bigRect)`, `GRAF_SHRINKBOX(smallRect,bigRect)` | Returns 0/nonzero status | Visual transition only; does not alter screen contents. [pp. 559, 561–562] |
| `GRAF_HANDLE([charW,charH,boxW,boxH])` | Returns current VDI handle | Optional outputs give standard font character and cell dimensions. [p. 560] |
| `GRAF_MKSTATE([mouseX,mouseY,buttons,shift])` | Return value documented as 1 | Reads pointer and modifier state. [p. 560] |
| `GRAF_MOUSE(shape,address)` | Returns 0/nonzero status | Selects predefined/custom pointer or hides/shows it; custom data is 37 words. [p. 560] |
| `GRAF_MOVEBOX(width,height,startX,startY,endX,endY)` | Returns 0/nonzero status | Moves a rectangle without changing screen contents. [p. 561] |
| `GRAF_RUBBERBOX(x,y,minW,minH[,width,height])` | Returns 0/nonzero status | Draws rubber-band rectangle while left button is held; final dimensions are outputs. [p. 561] |
| `GRAF_SLIDEBOX(tree,frameObject,sliderObject,orientation)` | Returns 0–1000 slider position | Form-only helper; does not update object coordinates or redraw the object. Orientation 0 horizontal, 1 vertical. [pp. 562–563] |
| `GRAF_WATCHBOX(tree,object,insideState,outsideState)` | Returns 1 if pointer is inside on release, else 0 | Form-only helper; changes object state as pointer enters/leaves while button is held. [p. 563] |
| `MENU_BAR(tree,mode)` | Returns 0/nonzero status | Mode 0 removes menu bar; 1 draws it. [p. 563] |
| `MENU_ICHECK(tree,object,mode)` | Returns 0/nonzero status | Mode 0 clears checkmark; 1 sets it. [p. 564] |
| `MENU_IENABLE(tree,object,mode)` | Returns 0/nonzero status | Mode 0 disables; 1 enables item. [p. 564] |
| `MENU_TEXT(tree,object,text$)` | Returns 0/nonzero status | Replaces menu line; new text must not exceed original buffer length. [p. 564] |
| `MENU_TNORMAL(tree,object,mode)` | Returns 0/nonzero status | Mode 0 inverse title, 1 normal title. [p. 565] |
| `MENU_REGISTER(handle,text$)` | Returns menu index 0–5, or -1 | Registers accessory title; maximum six entries. [p. 565] |
| `OBJC_ADD(tree,parent,child)` | Returns 0/nonzero status | Inserts object and updates OB_NEXT, OB_HEAD and OB_TAIL; book notes this differs from raw GEMSYS behavior. [p. 565] |
| `OBJC_CHANGE(tree,object,dummy,x,y,w,h,state,mode)` | Returns 0/nonzero status | Changes OB_STATE; mode 0 no redraw, 1 redraw within clipping rectangle. Dummy is always 0. [p. 566] |
| `OBJC_DELETE(tree,object)` | Returns 0/nonzero status | Removes object from selectable tree by adjusting links; can be restored/reinserted. [p. 566] |
| `OBJC_DRAW(tree,firstLevel,children,x,y,w,h)` | Returns 0/nonzero status | Draws tree levels inside clipping rectangle; child count 0 means only first level. [pp. 566–567] |
| `OBJC_EDIT(tree,object,char$,position,mode[,nextPosition])` | Returns 0/nonzero status | Modes 1 initialize/formatted text and cursor, 2 insert character, 3 finish; optional output gives next character position. [p. 567] |
| `OBJC_FIND(tree,startObject,levels,x,y)` | Returns object index or -1 | Finds object at screen position. [pp. 567–568] |
| `OBJC_OFFSET(tree,object[,x,y])` | Returns 0/nonzero status | Outputs absolute screen coordinates of object's top-left corner. [p. 568] |
| `OBJC_ORDER(tree,object,level)` | Returns 0/nonzero status | Moves object to another tree level. [p. 568] |

**Resource, scrap, shell and window managers**

| Function | Signature | Contract |
|---|---|---|
| `RSRC_FREE()` | Returns 0/nonzero status | Releases memory allocated by RSRC_LOAD and clears corresponding object tree. [p. 569] |
| `RSRC_GADDR(type,object[,address])` | Returns 0/nonzero status | Reliable only for type 0 TREE and 15 AD_FRSTR according to the author; other documented type selectors are reported unreliable. [pp. 569–570] |
| `RSRC_LOAD(name$)` | Returns 0/nonzero status | Loads RSC resource, converts character-cell coordinates to pixels; requires sufficient reserved memory. [p. 570] |
| `RSRC_OBFIX(tree,object)` | Return value documented as always 1 | Converts object coordinates from character units to pixels. [p. 570] |
| `RSRC_SADDR(type,object,pointer)` | Returns 0/nonzero status | Replaces a resource structure pointer; author warns it is not always reliable. [p. 571] |
| `SCRP_READ([buffer$])` | Returns 0/nonzero status | Reads global Scrap buffer (about 160 bytes in this book's TOS context); caller prepares a sufficiently large string. [pp. 571–572] |
| `SCRP_WRITE(text$)` | Returns 0/nonzero status | Writes up to 160 bytes to global Scrap buffer. [p. 572] |
| `SHEL_ENVRN([address,string$])` | Returns offset; outputs environment address | Searches DOS environment string; source documents unusual offset/address semantics. [p. 572] |
| `SHEL_FIND([path$])` | Returns 0/nonzero status | Input/output path buffer; searches current location and then drive A root. Wildcards are not expanded; buffer must allow longer output. [p. 573] |
| `SHEL_GET(count[,buffer$])`, `SHEL_PUT(count,buffer$)` | Returns 0/nonzero status | Read/write up to 518 bytes of current desktop environment buffer. [pp. 573–574] |
| `SHEL_READ([name$,command$])` | Returns 0/nonzero status | Returns caller program name and command tail; strings are NUL-terminated. [p. 574] |
| `SHEL_WRITE(mode,graphics,gemFlag,command$,name$)` | Returns 0/nonzero status | Starts program or exits GEM; caller memory is released before launch. [p. 574] |
| `WIND_CALC(mode,attributes,inX,inY,inW,inH[,outX,outY,outW,outH])` | Returns 0/nonzero status | Converts between window work-area and outer-frame geometry. [p. 575] |
| `WIND_CLOSE(handle)` | Returns 0/nonzero status | Closes visible window but keeps handle allocated for later WIND_OPEN. [p. 575] |
| `WIND_CREATE(attributes,maxX,maxY,maxW,maxH)` | Returns handle (>0), or <1 on failure | Allocates and registers a window. [p. 576] |
| `WIND_DELETE(handle)` | Returns 0/nonzero status | Deregisters window and releases handle/resource. [p. 576] |
| `WIND_FIND(x,y)` | Returns window handle, or 0 | Finds window at screen coordinates. [p. 576] |
| `WIND_GET(handle,mode[,get1,get2,get3,get4])` | Returns 0/nonzero status | Mode 4 = work area; 5 = current outer rectangle; 6 = previous outer rectangle; 7 = maximum outer rectangle; 8/9 = horizontal/vertical slider position; 10 = current window handle; 11/12 = first/next visible rectangle; 15/16 = horizontal/vertical slider size. The printed mode-4 description labels the height output as Get1 again instead of Get4; this is a confirmed source-table error, not merely OCR noise. [p. 577] |
| `WIND_OPEN(handle,x,y,width,height)` | Returns 0/nonzero status | Opens previously created window. [p. 578] |
| `WIND_SET(handle,mode,set1,set2,set3,set4)` | Returns 0/nonzero status | Mode 1 attributes; 2 title pointer; 3 info-line pointer; 6 geometry; 8/9 slider positions; 10 top/activate; 14 desktop tree; 15/16 slider sizes. Pointer arguments are split into high/low words. [pp. 578–579] |
| `WIND_UPDATE(mode)` | Returns 0/nonzero status | Modes 0/1 unlock/lock redraw rectangle lists; 2/3 hand mouse control to AES/program. Locking must be balanced and must not strand AES menu/window interaction. [pp. 579–580] |

The book's page 577 transcription contains an apparent OCR/print-layout error in the mode-4 output description (the fourth output is described as height but is labelled Get1 again). Treat the four-value rectangle result as unresolved until the page image is checked. Likewise, do not infer missing AES calls from the manager family names: this chapter is intentionally a selected, incomplete catalogue, not the full AES API.


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


### 8.5 BIOS call inventory (printed pp. 605–608)

The generic call convention is `BIOS(opcode[,parameters])`; `L:` marks a 32-bit longword argument and unmarked arguments are words. The function result is returned in D0. The following is the chapter's complete BIOS list, normalized from its entries:

| Opcode | Name | Book-described contract |
|---:|---|---|
| 0 | GETMPB | Writes a 12-byte memory-parameter block: free-list, allocated-list and roving-pointer addresses. Each memory-list node is four longwords (link, start, length, owner/process descriptor). The author explicitly warns that this is unusable without detailed TOS knowledge. |
| 1 | BCONSTAT | Tests whether bytes are available on a device/port. |
| 2 | BCONIN | Waits for a byte from a device; the book describes a 4-byte return value, with keyboard device 2 corresponding to KEYGET-style packed input. |
| 3 | BCONOUT | Outputs one byte to a device and waits until accepted. |
| 4 | RWABS | Reads/writes logical sectors using a caller-owned buffer. Flags 0/1 read/write with disk-change errors; 2/3 read/write while ignoring disk-change. Buffer size is 512 × sector count in this book's context. |
| 5 | SETEXEC | Replaces an exception vector and returns its previous address; exception number -1 queries the old address only. |
| 6 | TICKCAL | Returns the millisecond duration between timer ticks (book says 20 ms). |
| 7 | GETBPB | Returns the address of an 18-byte per-drive BIOS parameter block. |
| 8 | BCOSTAT | Tests whether a device is ready to accept output. |
| 9 | MEDIACH | Returns disk-change status: 0 definitely unchanged, 1 possibly changed, 2 definitely changed. |
| 10 | DRVMAP | Returns drive-presence bit vector; the book notes the system assumes at least A and B and may return 3 even if only A is physically present. |
| 11 | KBSHIFT | With -1, reads modifier state; a positive value sets/simulates the state. Bits 0–6 cover right/left Shift, Control, Alternate, CapsLock, Alt+ClrHome and Alt+Insert. |

GETBPB's nine words are sector byte size, sectors/cluster, cluster byte size, directory sectors, FAT sectors, second FAT sector, first data-cluster sector, data-cluster count and flags (meaning not known to the author). [pp. 605–608]

### 8.6 GEMDOS call inventory (printed pp. 608–618)

GEMDOS uses the same word/longword marker convention. The book lists these calls; the table groups related entries while preserving the printed opcode numbers:

| Opcode(s) | Name(s) | Book-described contract / caution |
|---:|---|---|
| 0 | PTERM | Terminates the process; explicitly do not use from GFA BASIC—use QUIT or SYSTEM. |
| 1–8 | CONIN, CONOUT, AUXIN, AUXOUT, PRNOUT, RAWCONIO, RAWCONIN, RAWNECIN | Console, RS232, printer and raw keyboard I/O. CONIN/RAWCONIO may return packed 32-bit key data; Control-C can crash in some input paths. |
| 9–10 | CONWS, CONRS | NUL-terminated console string output; buffered console input using a caller-provided length byte and buffer. |
| 11 | CONSTAT | Keyboard-buffer status (0 or 65535). |
| 14 | SETDRV | Changes default drive and returns the previously active drive. |
| 16–19 | CONOUTSTAT, PRTOUTSTAT, AUXINSTAT, AUXOUTSTAT | Console/printer/RS232 readiness. The book explicitly says not to use CONOUTSTAT in GFA. |
| 25 | CURDRV | Reads current default drive. |
| 26 | SETDTA | Sets disk transfer address; described as equivalent to V3.0 FSETDTA. |
| 32 | SUPER | Switches user/supervisor stack/mode. Privileged, register-sensitive and explicitly dangerous; never expose as an ordinary portable operation. |
| 42–45 | GETDATE, SETDATE, GETTIME, SETTIME | Read/write packed date/time words. Prefer DATE$/TIME$/SETTIME GFA-level facilities where applicable. |
| 47–48 | GETDTA, GETVERS | Read DTA address and GEMDOS version. |
| 49 | KEEP PROCESS | Terminates while retaining a memory region; book says not to use in ordinary GFA programs. |
| 54 | GETDFREE | Writes four longwords (16 bytes total: free clusters, total clusters, sector bytes, sectors/cluster) to a caller buffer. The source's four fields are at offsets 0, 4, 8 and 12. |
| 57–59 | MKDIR, RMDIR, CHDIR | Directory creation/removal/current-directory change using NUL-terminated target strings. |
| 60–64 | CREATE, OPEN, CLOSE, READ, WRITE | Raw GEMDOS file handles and buffers; not equivalent to GFA channel numbers. |
| 65–67 | UNLINK, LSEEK, CHMODE | Delete file, seek within handle, and get/set file date/time or attributes as described in the entry. |
| 69–70 | DUP, FORCE | Duplicate a handle and force a standard handle to refer to another handle. |
| 71 | GETDIR | Query current directory for a drive. |
| 72–74 | MALLOC, MFREE, SETBLOCK | Allocate/free/resize GEMDOS memory blocks. Requires strict ownership and lifetime tracking. |
| 75–76 | PEXEC, PTERM | Execute a process / terminate with a status value. Process control and memory ownership are OS-specific. |
| 78–79 | SFIRST, SNEXT | Start/continue directory search using a DTA. |
| 86–87 | RENAME, GSDTOF | Rename and date/time conversion helpers. |

The book lists common negative error results: -32 invalid function, -33 file not found, -34 path not found, -35 too many open files, -36 access denied, -37 invalid handle, -39 insufficient memory, -40 invalid memory-block address, -46 invalid drive and -49 no more matching files. It states zero means success for the calls discussed; this is not a universal return convention for every Atari trap. [pp. 608–618]

The book's raw GEMDOS file operations use NUL-terminated path strings, target addresses and OS handles. Do not route these directly to host file descriptors without an Atari-compatible handle table, path semantics, DTA/search model, error mapping and memory validation.

### 8.7 VDISYS call shape and selected operations (printed pp. 618–623)

Page-image check of p. 618 confirms the V3.0 form as `VDISYS(opcode[,I_len,P_len[,id]])`; the V2.xx form is `VDISYS(opcode)`. I_len and P_len describe INTIN and PTSIN array lengths; an optional subfunction ID is used by escape operations. If the same opcode is repeated without changing the VDI arrays, the opcode may be omitted. The active workstation handle is in CONTRL word 6 (byte offset 12); the book says V2.xx obtains it from VDIBASE+40 and V3.0 from V-H.

The chapter gives these selected operations:
- 35: query current polyline attributes.
- 36: query current polymarker attributes.
- 37: query current fill-area attributes.
- 38: query current graphic-text attributes.
- 104: set fill-perimeter visibility.
- 118: exchange timer interrupt vector (explicitly marked V2.xx-only in the example).
- 5: VDI escape functions, with subfunction IDs for screen character grid, pixel/character text mode, cursor movement/position, cursor/screen/line clearing, cursor position query, mouse availability, hardcopy, mouse positioning and system-font selection.

These are examples of the book's generic VDI trap wrapper, not a complete VDI function catalogue. VDI arrays and CONTRL remain shared mutable target buffers. Interrupt-vector manipulation is unsafe and version-specific. [pp. 618–623]

### 8.8 XBIOS call inventory and hardware boundaries (printed pp. 622–656)

The book documents the following XBIOS calls. Names below follow its printed labels; spelling and availability must still be cross-checked against the selected TOS release.

| Opcode | Name | Function family / notes |
|---:|---|---|
| 0 | INITMOUSE | Installs custom mouse handler with a target parameter block and code address; author gives no usable parameter-layout contract. |
| 1 | SSBRK | ROM-module memory reservation during boot, not an ordinary application call. |
| 2–3 | PHYSBASE, LOGBASE | Read physical and logical screen base addresses. |
| 4–5 | GETREZ, SETSCREEN | Query/set screen resolution and screen base pointers; SETSCREEN's resolution parameter is not usable under GEM according to the book. |
| 6–7 | SETPALETTE, SETCOLOR | Palette/register color operations. |
| 8–10 | FLOPREAD, FLOPWRITE, FLOPFMT | Floppy sector read/write/format. Destructive disk operations; disabled by default. |
| 11 | — | Book says “no effect”. |
| 12–13 | MIDIWS, MFPINT | MIDI output and MFP interrupt-vector handling. |
| 14–16 | IOREC, RSCONF, KEYTABLE | Serial I/O ring-buffer state, RS232 configuration and keyboard translation tables. |
| 17–19 | RANDOM, PROTOBT, FLOPVER | Random value, floppy boot-sector operation and floppy verification/read-related operation. |
| 20 | SCRDMP | Screen hardcopy. |
| 21–25 | CURSCONF, SETTIME, GETTIME, BIOSKEYS, IKBDWS | Cursor configuration, system clock, BIOS keyboard table and IKBD commands. |
| 26–30 | JDISINT, JENABINT, GIACCES, OFFGIBIT, ONGIBIT | MFP interrupt enable/disable and sound-chip access/interrupt handling. |
| 31–32 | XBTIMER, DOSOUND | MFP timer installation and sound-chip register sequence execution. |
| 33–36 | SETPRT, KBDVBASE, KBRATE, PRTBLK | Printer configuration, keyboard vector base/repeat and printer block output. |
| 37–39 | VSYNC, SUPEXEC, PUNTAES | Vertical sync, supervisor callback and AES shutdown attempt. |

XBIOS error values listed on p. 622 include -1 general error, -2 device not ready, -3 unknown command, -5 invalid command, -6 track not found, -7 invalid boot sector, -8 sector not found, -10 write error, -12 general error, -13 write-protected disk, -14 disk changed, -15 unknown device, -16 bad sector and -17 no disk inserted. The book says zero indicates success for the operations discussed.

The source's detailed floppy format/boot-sector and virus discussion (pp. 630–645), sound-chip/interrupt routines (pp. 646–655), and supervisor/AES shutdown material (p. 656) are historical, privileged or destructive Atari-specific material. Do not implement these as host disk, audio, interrupt or process operations. Require a sandboxed machine profile and explicit safety policy before any emulation.

**Page-image checks completed:** p. 605 confirms BIOS generic signature, L: longword marker, D0 result and GETMPB layout; p. 612 confirms the SUPER warning and stack/mode caveats; p. 618 confirms the VDISYS V3.0 argument form and workstation-handle source; p. 623 confirms XBIOS INITMOUSE/SSBRK/PHYSBASE/LOGBASE declarations. Other detailed XBIOS disk/sound examples remain text/OCR-derived pending targeted image review.
 

### 9.1 System-variable inventory (printed pp. 657–664)

The book lists the following absolute Atari/TOS addresses. These are **source-documented target addresses**, not portable constants and not safe host memory locations.

| Address | Width | Name | Book-described meaning / caveat |
|---:|---:|---|---|
| 0x0424 | Word | membctrl | Copy of memory-controller configuration. |
| 0x0426 | Long | resvalid | Reset validation magic 0x31415926; author says internal mechanism is not sufficiently understood. |
| 0x042A | Long | resvector | Reset callback vector. |
| 0x042E | Long | phystop | End of physical RAM. |
| 0x0432 | Long | membot | Start of user memory above OS-reserved low memory. |
| 0x0436 | Long | memtop | End of user memory; normally beginning of screen memory. |
| 0x0440 | Word | seekrate | Floppy step rate; write changes drive mechanics timing. |
| 0x0442 | Word | timer_ms | Milliseconds between system timer ticks; source says normally 20. |
| 0x0444 | Word | fverify | Floppy write-verify enable; zero disables verification. |
| 0x0446 | Word | bootdev | Drive number used to boot. |
| 0x0448 | Word | palmode | PAL/NTSC mode; source says changing it after boot has no effect. |
| 0x044A | Word | defshiftmod | Desired color resolution when switching from monochrome; read-only without system knowledge. |
| 0x044C | Word | sshiftmd | Current screen resolution; same information as XBIOS(4). |
| 0x044E | Long | v_bas_ad | Logical screen base pointer; use XBIOS(5) rather than direct modification. |
| 0x0452 | Word | vblsem | VBL routine semaphore; zero disables VBL routines and can disable BASIC Break handling. |
| 0x0454 | Word | nvbls | Number of VBL routines. The text calls it a Longword once, but the entry heading and address layout say Word; retain this source inconsistency for verification. |
| 0x0456 | Long | vblqueue | Pointer to VBL routine address list; changing it requires coordinated table/count updates. |
| 0x045A | Long | colorptr | Pointer to 16-word palette loaded on next VBL; zero disables palette reload. |
| 0x045E | Long | screenptr | Pending logical screen base installed on next VBL; zero disables repeated install. |
| 0x0462 | Long | vbclock | VBL interrupt count since boot. |
| 0x0466 | Long | frclock | Interrupt routine count since boot. |
| 0x046A | Long | hdv_init | Hard-disk initialization jump-table pointer. |
| 0x046E | Long | swv_vec | Color-mode resolution-change vector. |
| 0x0472 | Long | hdv_bpb | Hard-disk BPB lookup vector. |
| 0x0476 | Long | hdv_rw | Hard-disk read/write routine pointer. |
| 0x047A | Long | hdv_boot | Hard-disk boot routine pointer. |
| 0x047E | Long | hdv_mediach | Hard-disk media-change routine pointer. |
| 0x0484 | Byte | conterm | Four-bit keyboard click/repeat/bell/BIOS(2) high-byte status configuration. |
| 0x0486 | Long | trpl4ret | Return address after TRAP #14 (XBIOS). |
| 0x048E | 4 Longs | themd | Memory descriptor block referenced by BIOS(0). |
| 0x04A2 | Long | savptr | Pointer to saved processor register state. |
| 0x04A6 | Word | nflops | Floppy count; source notes minimum A/B assumption. |
| 0x04AC | Word | save_row | Cursor-position buffer. |
| 0x04AE | Long | save_context | Exception-processing buffer pointer. |
| 0x04BA | Long | hz_200 | 200 Hz system timer used by BASIC TIMER. |
| 0x04C2 | Long | drvbits | Drive-presence bit vector, with the same A/B minimum assumption. |
| 0x04C6 | Long | dskbufp | Pointer to 1,024-byte disk buffer containing current boot sector. |
| 0x04CE | 8 Longs | vbl_list | Default VBL routine list; can be relocated/extended only with coordinated nvbls/vblqueue changes. |
| 0x04EE | Word | dumpflg | Set while hardcopy is in progress. |
| 0x04F0 | Word | prtabt | Printer timeout-abort flag; meaning uncertain to the author, default 252. |
| 0x04F2 | Long | sysbase | OS start address. |
| 0x04FA | Long | end_os | End of OS RAM area. |
| 0x04FE | Long | exec_os | AES start address within OS. |

The book's chapter 22 includes direct writes that change floppy timing/verification, VBL registration, screen/palette switching and GFA BASIC Break/direct-mode handling. The V2.xx interpreter patch example at p. 660 is particularly version- and build-specific. These are **unsafe/disabled** by default. Do not expose raw addresses through the host process address space; any future Atari emulation must use a virtual address map and machine/TOS profile.



### 8.9 Source consistency correction: GETDFREE buffer size

A line-by-line check of p. 614 confirms that GEMDOS(54) GETDFREE writes **four 32-bit longwords**, at buffer offsets +0, +4, +8 and +12. The required buffer is therefore **16 bytes**, not 12 bytes. The printed prose calls it a “12-Byte-Puffer”, but the fourth field is explicitly placed at +12 and occupies another longword. This is a source-internal contradiction; the field layout determines the minimum size. Treat the phrase “12-byte buffer” as a likely book typo and preserve the discrepancy in evidence notes. No host implementation should allocate only 12 bytes.


### 8.10 Cross-reference validation against Atari OS documentation

The book's trap wrappers were cross-checked against Atari's *A Hitchhiker's Guide to the BIOS*, Rev. 1.1 (5 March 1990), and *The Atari Compendium* (GEMDOS/XBIOS chapters). These are independent Atari OS references; they do not prove that every GFA wrapper exposes every OS version's full contract.

- BIOS uses TRAP #13; XBIOS uses TRAP #14; GEMDOS uses TRAP #1. Arguments are pushed on the 68k stack in reverse order, followed by the function opcode. The caller cleans the stack. The GFA wrapper hides this assembly-level calling sequence.
- Atari's BIOS guide says XBIOS uses the same calling convention as BIOS. The Atari Compendium documents D0–D2 and A0–A2 as scratch registers for GEMDOS/XBIOS and says the function opcode on the stack is modified. The generic D0 return description in the GFA book is therefore insufficient as a complete ABI contract.
- GEMDOS is not guaranteed re-entrant. BIOS and XBIOS have limited re-entrancy (the BIOS guide says three nested calls, without depth checking); this is not a promise that arbitrary callbacks or interrupt handlers may safely invoke them.
- The Compendium recommends GEMDOS/VDI abstractions over XBIOS hardware calls where possible, and warns that hardware-specific calls may not transfer to later Atari systems.
- The Atari BIOS Rev. 1.1 guide gives a concrete INITMOUS parameter block, whereas the GFA book explicitly does not provide a usable parameter layout. Keep the GFA-facing wrapper evidence and the OS-level structure as separate evidence records until the wrapper's marshaling is verified.
- XBIOS opcode 11 is described as “no effect” by the GFA book. Atari's 1990 BIOS guide labels it “used-by-BIOS”; later Atari Compendium material identifies DBMSG as a debugger extension. Treat this as version/context-dependent, not as a stable no-op across all TOS variants.
- The GFA book's BIOS(11) KBSHIFT description assigns bits 5–6 to Alt+ClrHome/Alt+Insert. Atari's BIOS Rev. 1.1 guide describes these bits as mouse-button states (and bit 7 as reserved). This is a concrete semantic discrepancy requiring an explicit source-version note; do not silently merge the tables.

Primary cross-reference URLs:
- Atari, *A Hitchhiker's Guide to the BIOS*, Rev. 1.1: https://manualzz.com/doc/26181874/atari-bios-rev-1.1-system-software-guide
- *The Atari Compendium*, GEMDOS: https://bus-error.nokturnal.pl/atari_compendium/html/chap2.htm
- *The Atari Compendium*, XBIOS: https://www.yardley.cc/atari/compendium/atari-compendium-chapter-4-XBIOS.htm

### 9.2 System-variable cross-source discrepancies

The Atari BIOS Rev. 1.1 guide does not match every low-memory field in the GFA book. Preserve the values as source-specific rather than normalizing them into one supposedly universal map:

| Field | GFA book | BIOS Rev. 1.1 guide | Treatment |
|---|---|---|---|
| memctrl/memcntlr at 0x0424 | Word | Byte | Width conflict; profile/version-specific until resolved |
| defshiftmod at 0x044A | Word | Byte | Width conflict |
| nvbls at 0x0454 | Word in heading/address map; prose calls it Long | Word | External guide supports Word; book's prose is internally inconsistent |
| hz_200 at 0x04BA | Long | 0x04BC | Address conflict |
| drvbits at 0x04C2 | Long | 0x04C4 | Address conflict |
| XBIOS opcode 11 | “No effect” | Used by BIOS | Version/context conflict |

The BIOS Rev. 1.1 guide also lists additional variables omitted from the GFA book's table, so absence from the book must not be interpreted as absence from TOS. Any future absolute-memory emulation must select a precise machine/TOS map and must not use this book's addresses as universal constants.
