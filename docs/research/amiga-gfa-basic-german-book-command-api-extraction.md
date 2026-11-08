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



## 7. Interpreter language constructs: Chapter 6

The following entries are extracted from Chapter 6, printed pp. 145–191. They are language constructs, not Amiga library calls. The OCR is sufficient to identify the constructs and most syntax, but not to settle every parser edge case.

### 7.1 Loop constructs (printed pp. 145–150)

| Construct | Syntax recorded in the book | Semantics / aliases |
|---|---|---|
| FOR … NEXT | FOR Zaehl=Start TO [DOWNTO]Ende [STEP Schritt]; body; NEXT Zaehl | Counted loop. Without STEP, TO advances by +1. DOWNTO advances by -1 and does not permit STEP. Negative STEP values are possible with TO. |
| ENDFOR | ENDFOR Zaehl | Alternative to NEXT Zaehl; interpreter converts it to NEXT. |
| DO … LOOP | DO; body; LOOP | Unconditional loop if no conditions are supplied. Both endpoints can optionally carry WHILE condition or UNTIL condition, allowing entry/exit conditions. |
| ENDDO | ENDDO | Alternative to LOOP; interpreter converts it to LOOP. |
| REPEAT … UNTIL | REPEAT; body; UNTIL condition | Post-tested loop; body executes at least once. |
| ENDREPEAT | ENDREPEAT condition (as an alternate terminator) | Interpreter converts this terminator to UNTIL. The OCR around the exact operand placement is imperfect; verify against the page image before implementing syntax. |
| WHILE … WEND | WHILE condition; body; WEND | Pre-tested loop; body is skipped if the condition is false on entry. |
| ENDWHILE | ENDWHILE | Alternative to WEND; interpreter converts it to WEND. |
| EXIT IF | EXIT IF condition | Exits the current enclosing loop when condition is true. The book says it applies inside FOR/NEXT, DO/LOOP, REPEAT/UNTIL and WHILE/WEND loops. Execution continues after the loop's terminator; statements between the EXIT IF and terminator are not run again. |

The book states that loop types may be nested, and that terminators must close nested loops in reverse order. It also documents Boolean operators AND, OR, NOT, XOR, IMP and EQV for combining conditions.

### 7.2 Conditional branching and selection (printed pp. 149–167)

| Construct | Syntax recorded in the book | Semantics / aliases |
|---|---|---|
| IF | IF condition [THEN]; body; optional ELSE; ENDIF | THEN is optional. IF blocks may be nested. |
| ELSE IF | ELSE IF condition; body; repeated as needed | Additional branch conditions; only reached if preceding conditions were false. |
| SELECT | SELECT expression; one or more CASE clauses; optional DEFAULT; ENDSELECT | Selects a matching case. The book says statements before the first CASE are not registered/executed as part of the selection. |
| CASE | CASE value[, value...] or CASE lower TO upper | Supports single values, comma-separated alternatives and inclusive ranges. Either range bound may be omitted to represent the minimum/maximum bound. |
| DEFAULT | DEFAULT; fallback body; ENDSELECT | Fallback when no CASE matched. OTHERWISE is an alias converted by the interpreter to DEFAULT. |
| CONT in SELECT | CONT immediately before a CASE or DEFAULT | Allows the next CASE/default sequence to execute in addition to the already matched branch. Outside this position, CONT has its ordinary program-continuation meaning. |

### 7.3 Procedures and variable passing (printed pp. 170–185)

The book defines PROCEDURE as a named subprogram that may contain arbitrary program text and returns to the statement after its caller when RETURN executes.

```basic
PROCEDURE Name [(Variablenliste)]
  ... statements ...
RETURN
```

- Procedure names may contain letters, digits, underscore and period; unlike variable names, the first character may be a digit.
- A procedure may declare a list of local receiving variables. The book says these receive values passed through GOSUB; parameter types must match the corresponding argument types.
- Procedures may call other procedures and may recursively call themselves.
- ENDPROC and ENDSUB are documented as alternatives to RETURN; the interpreter normalizes them to RETURN.
- The book documents a VAR marker in PROCEDURE/FUNCTION parameter lists for direct variable passing, as an alternative to indirect pointer variables. The OCR examples show the shape PROCEDURE Name([ordinary parameters] VAR variable[, ...]) and similarly for FUNCTION; exact punctuation and mixed-parameter grammar need page-image verification.
- The chapter cross-references LOCAL, VAR, GOSUB and FUNCTION. Do not infer their complete grammar from the cross-reference alone.

### 7.4 Computed procedure dispatch and Break handling

The book also describes computed procedure dispatch:

```basic
ON Wert GOSUB Proc1 [, Proc2, Proc3, ...]
```

The value selects a procedure by one-based position: 1 calls the first procedure, 2 the second, and so on. Zero or a value greater than the number of supplied procedures causes execution to continue at the next statement without branching. The described targets must be procedures that do not expect a parameter list.

Break handling is documented with these forms:

- ON BREAK GOSUB Procedure: redirects the break key sequence to a procedure.
- ON BREAK: restores normal break behavior.
- ON BREAK CONT: disables normal break termination until a later ON BREAK/ON BREAK GOSUB, according to the chapter's description.

The exact semantics and state transitions should be checked against the interpreter manual before implementation.

### 7.5 Evidence boundary

This chapter provides strong evidence for the existence and broad syntax of the constructs above in the book's Amiga interpreter reference. It does not establish every whitespace, abbreviation, nesting, error-recovery or expression-precedence rule. Those details should be taken from the primary 3.5 interpreter manual or verified with the interpreter, not guessed from OCR.
