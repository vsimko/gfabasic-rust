# German Amiga GFA-BASIC book: command and OS API extraction

- **Status:** Source-derived research notes; not a normative 3.5 specification.
- **Source:** Wolf-Gideon Bleek, Martin Hecht, Uwe Litzkendorf, *Das große GFA-BASIC-Buch zum Amiga*, DATA BECKER, 1990, ISBN 3-89011-399-0.
- **Source scope:** The foreword explicitly says the main command reference documents, tests and explains the Amiga GFA-BASIC **version 3.0** interpreter. The book's introduction describes the supplied interpreter as V3.03 and approximately 360 commands/functions. The cover mentions version 3.5 and compiler 3.5, but this does not make the main language reference a 3.5 specification.
- **Version policy:** Label the book's language/API evidence as Amiga 3.0/3.03 unless a section explicitly identifies a later version. Do not promote it to confirmed 3.5 behavior without separate 3.5 manual or runtime evidence.

## 1. High-value command families

The book's contents and command index provide a broad map of the language. Relevant sections include:

| Topic | Chapter/printed pages | Extraction focus |
|---|---:|---|
| Input/output | Ch. 5, pp. 75–91 | INPUT, PRINT, WRITE, cursor/screen I/O |
| Files and disks | Ch. 5, pp. 92–118 | OPEN/CLOSE, directory and file operations, random access |
| Port I/O | Ch. 5, p. 119 | INP, OUT |
| AmigaDOS library | Ch. 5, pp. 120–131 | Direct DOS library calls |
| Program structure | Ch. 6, pp. 145–191 | loops, branches, procedures, functions, calls |
| Graphics | Ch. 9, pp. 225–278 | graphics primitives, object/Bob/sprite operations |
| Memory and pointers | Ch. 11, pp. 291–317 | field, memory and pointer operations |
| Exec library | Ch. 11, pp. 315–329 | Amiga Exec routines |
| Program control | Ch. 12, pp. 331–361 | run/stop, errors, time, multitasking |
| User interaction | Ch. 13, pp. 363–374 | alerts, mouse, joystick, requesters |
| Window and screen programming | Ch. 14, pp. 375–388 | Amiga-specific GFA window/screen commands |
| Menus | Ch. 15, p. 389 onward | BASIC menu programming |
| Event handling | Ch. 16, pp. 395–404 | menu/window event dispatch |
| Compiler | Ch. 17, pp. 405–450 | compiler-only material; keep separate from interpreter |
| Vectors/matrices | Ch. 18, pp. 451–476 | matrix statements |
| More editor comfort | Ch. 19, pp. 477–482 | math additions and editor commands |

The book's Appendix D, printed pp. 493–497, is an alphabetical command/function index. It includes GFA commands, functions, graphics/object operations and compiler/editor entries. The OCR is noisy; individual spellings and page numbers must be checked against the relevant command description before being used as parser facts.

## 2. Amiga GFA window commands

Chapter 14.1, printed pp. 375–382, describes a separate family of Window commands. The text explains the naming convention: Window commands end in `W`, while corresponding Screen commands end in `S`.

| Command | Syntax recorded in the book | Meaning / notes | Printed page |
|---|---|---|---:|
| `CLEARW` | `CLEARW Nummer` (also `CLEARW [#]Nummer`, marked V3.0) | Clears the contents of the numbered window. The book warns that `CLS` only clears the current window and may not clear graphics at the right/bottom edges. | 375 |
| `CLOSEW` | `CLOSEW Nummer` (also `CLOSEW [#]Nummer`, marked V3.0) | Closes the numbered window; the text gives window numbers 0–15 for this command. | 376 |
| `FULLW` | `FULLW Nummer` (also `FULLW [#]Nummer`, marked V3.0) | Enlarges the numbered window to maximum/full-screen size. | 376 |
| `OPENW` | `OPENW Nummer`; `OPENW Nummer, Xpos, Ypos, Breite, Höhe, IDCMP, Flaggen[, Screen[, BitMap]]` | Opens or activates a window. The extended form specifies position/size, event-message mask, window flags, optional screen index and optional bitmap address. See the interpreter manual for its more detailed syntax and flag descriptions. | 377–379 |
| `TITLEW` | `TITLEW Nummer, "Window-Titel"[, "Screen-Titel"]` (also `TITLEW [#]Nummer,...`) | Sets the window title and optionally the screen title shown when the window is active. | 379 |
| `BACKW` | `BACKW Nummer` | Sends the numbered window to the background. | 380 |
| `FRONTW` | `FRONTW Nummer` | Brings the numbered window to the foreground. | 380 |
| `MOVEW` | `MOVEW Nummer, Xpos, Ypos` | Moves the window to an absolute position relative to the upper-left of the screen. | 381 |
| `SIZEW` | `SIZEW Nummer, Breite, Höhe` | Sets a new width and height. The book warns that invalid dimensions/positions can crash the Amiga rather than produce a safe error. | 381 |
| `LIMITW` | `LIMITW Nummer, Xmin, Ymin, Xmax, Ymax` | Sets minimum and maximum window dimensions, applying to both user sizing and `SIZEW`. | 382 |
| `SETWPEN` | `SETWPEN LinienStift, FüllStift` (OCR indicates two pen parameters) | Sets standard window frame/fill pens before opening a window; the book says this cannot be changed afterwards with this command. | 382 |

### OPENW details

The book's examples show windows numbered 0–15 in the initial default arrangement: window 0 fills the screen below the title bar; windows 1–4 occupy quadrants; windows 5–15 are centered. The book's chapter-14 prose says the Amiga can manage many windows across screens, but its particular default-window description is limited to 0–15.

The `IDCMP` parameter selects which Intuition messages are sent to the program. The `flags` parameter selects window features and gadgets. The book lists flag names including `SMART_REFRESH`, `WINDOW_SIZING`, `WINDOW_DRAG`, `WINDOW_DEPTH`, `WINDOW_CLOSE`, `SIZE_RIGHT`, `SIMPLE_REFRESH`, `SUPER_BITMAP`, `BACKDROP`, `REPORT_MOUSE`, `GIMME_ZERO_ZERO`, `BORDERLESS`, `ACTIVATE`, `RMB_TRAP`, and `NOCAREREFRESH`. OCR and historical spelling differences exist; validate the exact constants against the page image or the interpreter manual before encoding them.

## 3. Amiga GFA screen commands

Chapter 14.2, printed pp. 383–388, describes these commands/functions:

| Command/function | Syntax recorded in the book | Meaning / notes | Printed page |
|---|---|---|---:|
| `OPENS` | `OPENS Nummer[, Xpos, Ypos, Breite, Höhe, BitPlanes, Modus]` | Opens a screen. The book says indices 0–15; index 0 is the Workbench screen. The X coordinate is included for compatibility, but the text says the current hardware does not use horizontal screen movement at open time. | 383–384 |
| `WINDOW()` | `WINDOW(Nummer)` and `WINDOW(Adresse)` | Converts a GFA window number to the address of its Window data, or converts an address back to the GFA window number. | 383 |
| `FRONTS` | `FRONTS Nummer` | Brings the numbered screen to the foreground. | 385–386 |
| `CLOSES` | `CLOSES Nummer` | Closes the screen and its windows. | 386 |
| `BACKS` | `BACKS Nummer` | Sends the numbered screen to the background. | 386–387 |
| `TITLES` | `TITLES Nummer, TitelText` | Sets the default title text in the screen's title bar. | 387 |
| `MOVES` | `MOVES Nummer, Xpos, Ypos` | Moves a screen to absolute X/Y coordinates. | 387 |
| `SCREEN()` | `SCREEN(Nummer)` and `SCREEN(Adresse)` | Converts a screen number to its address, or an address to the screen number. | 388 |
| `SETSPEN` | `SETSPEN DetailPen, BlockPen` | Sets screen title/detail and title-block pen colors. | 388 |

The book's Screen chapter documents modes/flags including `GENLOCK VIDEO`, `INTERLACE`, `EXTRA HALFBRITE`, `HOLD AND MODIFY`, `SPRITES`, and `HIRES`; the numeric values are given in its printed pp. 384–385. Do not conflate these screen mode bits with `OPENW` window flags.

## 4. Direct Amiga library calls

The book explicitly distinguishes GFA BASIC statements from operating-system library routines. It says the Exec routines can be called from GFA BASIC as functions without a separate library-opening step. This statement is specifically about the Exec routines described there; do not generalize it to every Amiga library.

### 4.1 Graphics library examples

The book's System routines appendix (printed pp. 11-46–11-49) documents calls including:

| Routine | Signature in source | Purpose |
|---|---|---|
| `SetAPen` | `SetAPen(rastport, nr)` | Sets foreground drawing pen/color. |
| `SetBPen` | `SetBPen(rastport, nr)` | Sets background drawing pen/color. |
| `SetDrMd` | `SetDrMd(rastport, mode)` | Sets drawing mode; mode corresponds to `GRAPHMODE`. |
| `SetDrPt` | `SetDrPt(rastport, data)` | Sets a 16-bit line pattern. |
| `SetFont` | `SetFont(rastport, textfont)` | Replaces the font parameters in a RastPort with those in a TextFont structure; documented return value is 0 if no error. |
| `SetOPen` | `SetOPen(rastport, register)` | Sets outline pen color for text and area outlines. |
| `SetRast` | `SetRast(rastport, register)` | Fills the specified raster with a pen color. |
| `SetRGB4` | `SetRGB4(viewport, register, red, green, blue)` | Sets a viewport color register; register 0–31 and RGB intensities 0–15. |
| `SetRGB4CM` | `SetRGB4CM(colormap, register, red, green, blue)` | Sets a color-map entry. |
| `SetSoftStyle` | `SetSoftStyle(rastport, new, mask)` | Changes selected font style bits. |

The book identifies RastPort, ViewPort, TextFont and ColorMap arguments as structure addresses (passed as integer expressions). These are not GFA BASIC screen commands; they are direct system-library calls.

### 4.2 Intuition library examples

The System routines appendix (printed pp. 11-66–11-75) includes:

- `OpenScreen(newscreen)`: opens/displays a screen from a NewScreen structure; returns a Screen structure address or 0.
- `OpenWindow(newwindow)`: opens a window from a NewWindow structure; returns a Window structure address.
- `MoveScreen(screen, deltax, deltay)`: moves a screen by relative offsets.
- `MoveWindow(window, deltax, deltay)`: moves a window by relative offsets.
- `SizeWindow(window, deltax, deltay)`: changes window size by relative offsets.
- `WindowLimits(window, minx, miny, maxx, maxy)`: sets window size limits.
- `WindowToBack(window)` and `WindowToFront(window)`: change window stacking order.
- `SetWindowTitles(window, wtitle, stitle)`: sets window and screen title strings.
- `ReportMouse(window, true)`: enables/disables mouse-movement reports.
- `ShowTitle(screen, mode)`: controls screen title display relative to backdrop windows.

### 4.3 Exec library examples

The book's Exec chapter (printed pp. 11-5–11-19) describes a large set of low-level system routines. Extracted examples include:

- Memory: `AllocMem(size, attributes)`, `FreeMem(address, size)`, `AvailMem(attributes)`, `AllocEntry(memlist)`, `FreeEntry(memlist)`.
- Task/scheduling: `Forbid()`, `Permit()`, `Disable()`, `Enable()`, `FindTask(name)`, `SetTaskPri(task, priority)`, `RemTask(task)`.
- Libraries/resources: `OpenLibrary(name, version)`, `CloseLibrary(library)`, `AddLibrary(library)`, `RemLibrary(library)`, `SetFunction(library, offset, function_address)`, `SumLibrary(library)`, `OpenResource(name)`, `RemResource(resource)`.
- Message ports: `AddPort(msgport)`, `FindPort(name)`, `WaitPort(msgport)`, `PutMsg(msgport, message)`, `GetMsg(msgport)`, `ReplyMsg(message)`, `RemPort(msgport)`.
- Devices/I/O: `OpenDevice(name, unit, IORequest, flags)`, `CloseDevice(IORequest)`, `DoIO(IORequest)`, `SendIO(IORequest)`, `WaitIO(IORequest)`, `DeviceProc(name)`.
- Lists/nodes: `FindName(start, name)`, `Enqueue(list, node)`, `Insert(list, node1, node2)`, `Remove(node)`, `RemHead(list)`, `RemTail(list)`.

These signatures are the book's GFA-facing descriptions, not a substitute for the original Amiga NDK headers/ABI. The book warns that incorrect OS arguments may produce a DOS error or a Guru Meditation. Preserve pointer types, zero-terminated strings and structure layout in any future runtime/API model.

### 4.4 AmigaDOS library examples

Chapter 5.7 (printed pp. 120–131) documents 31 routines and gives examples. Extracted routines include:

- File handles and byte I/O: `Open(name, mode)`, `Close(filehandle)`, `Read(filehandle, buffer, length)`, `Write(filehandle, buffer, length)`, `Seek(filehandle, offset, mode)`, `WaitForChar(filehandle, timeout)`, `Input()`, `Output()`, `IsInteractive(filehandle)`, `IoErr()`.
- Lock/directory traversal: `Lock(name, mode)`, `UnLock(lock)`, `DupLock(lock)`, `Info(lock, buffer)`, `Examine(lock, buffer)`, `ExNext(lock, buffer)`, `ParentDir(lock)`, `CurrentDir(lock)`, `CreateDir(name)`.
- File metadata/management: `SetProtection(name, mode)`, `Rename(oldname, newname)`, `DeleteFile(name)`, `SetComment(name, comment)`, `LoadSeg(name)`, `UnLoadSeg(segment)`, `DateStamp(address)`, `Execute(command, input, output)`, `Delay(ticks)`, `Exit(code)`, `DeviceProc(name)`, `CreateProc(name, priority, segment, stack)`.

The book states that C-style OS strings must be NUL-terminated. It describes `Open` modes 1005 (existing file for writing/deletion), 1006 (create/truncate), and 1004 (exclusive access). Confirm all such numeric constants and structure offsets against primary system documentation before implementation.

## 5. Event and menu constructs identified by the book

The contents and index show the Amiga-specific event/menu surface, including:
`MENU`, `MENUKEY`, `MENUKILL`, `ON MENU`, `ON MENUBUTTON GOSUB`, `ON MENU GOSUB`, `ON MENUKEY GOSUB`, `SLEEP`, `MOUSE`, `MOUSEX`, `MOUSEY`, `MOUSEK`, `STICK`, `STRIG`, `FILESELECT`, `ALERT`, `EVERY`, `AFTER`.

The book's Chapter 16 explains that `MENU(1)` carries the Intuition message class, `MENU(2)` carries message-specific data, and `MENU(10)` carries the Window structure address in the described event cases. It describes decoding menu, window, and mouse events from these values. Exact masks/bit meanings must be transcribed and verified from the printed pages before being used as parser or runtime contracts.

## 6. Extraction cautions and next reconciliation

1. The German book is broad historical Amiga reference material, but its foreword explicitly scopes its main command/function documentation to interpreter 3.0; the introduction mentions V3.03. The compiler chapter is separate.
2. Its window/screen chapters explicitly label the described GFA commands as version 3.0.
3. The command index is OCR-derived and has clear OCR damage (for example, confusable letters, punctuation and page digits). Treat it as a discovery index only.
4. Amiga library routine names and signatures are not BASIC keywords. Keep them in a separate Amiga API namespace/catalogue.
5. The book itself warns about system crashes from invalid library arguments. Do not infer ABI safety or Linux emulation semantics from these descriptions.
6. Compare each relevant command with the supplied *GFA BASIC 3.5 Interpreter* manual and record 3.5 confirmation separately.

