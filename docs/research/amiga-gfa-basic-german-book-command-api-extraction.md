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


## 8. Program control and error handling: Chapter 12

Extracted from printed pp. 331–361.

### 8.1 Lifecycle commands

| Command | Behavior described by the book |
|---|---|
| RUN | Restarts the current program and clears variables/screen. RUN "program-name" is also shown for loading and starting another program, with version/context caveats. |
| STOP | Suspends execution and displays an ALERT box, allowing continuation or entry to direct mode. |
| CONT | Continues after STOP at the following statement. Not available after CLEAR, listing edits, or introduction of new variables. |
| END | Stops the current program but retains variable contents and open files in the interpreter; direct mode can inspect them and CONT may resume. |
| SYSTEM / QUIT | Exits to Workbench or the caller (CLI/EXEC). QUIT is described as identical to SYSTEM; QUIT [x] is marked Version 3.0. |
| CLEAR | Sets numeric variables to zero, strings to empty, erases arrays and removes dimensions. The book says not to use it inside procedures or FOR/NEXT loops. |
| NEW | Clears the BASIC program and its variables from memory. |
| EDIT | Exits execution to the editor. |

### 8.2 Error handling

- ON ERROR GOSUB Procedure redirects BASIC/system errors to a procedure; the programmer owns error handling.
- ON ERROR restores normal error reporting/termination. The editor may insert GOSUB automatically in the described 3.0 environment.
- ERR is a reserved variable containing the error identifier.
- ERR$(index) returns the text for an error number; the book labels this function Version 3.0.
- FATAL distinguishes fatal system errors (-1) from ordinary BASIC errors (0), according to the book.
- ERROR number deliberately raises/simulates the selected error.
- RESUME is allowed only inside a procedure. RESUME retries the failed statement; RESUME NEXT continues with the following statement; RESUME Label continues at a label. After the error procedure returns, the interpreter resets error handling to normal, so ON ERROR GOSUB must be installed again if required.

Do not assume the book's special fatal-system-error recovery is reproducible in a modern host runtime.

### 8.3 Timer callbacks

The book states that the timer unit is 1/200 second (200 ticks = 1 second).

- AFTER Ticks [GOSUB] Procedure schedules a one-shot callback. It is dispatched only after the current BASIC statement completes; blocking operations can delay it.
- A later AFTER GOSUB replaces the previous pending callback. The described implementation cannot process multiple AFTER callbacks simultaneously.
- EVERY GOSUB is described as the periodic counterpart. The book suggests re-arming an AFTER handler before RETURN as an approximation; the extracted text does not establish complete EVERY edge-case semantics.

## 9. Intuition event handling: Chapters 15–16

Extracted from printed pp. 389–404. These are interpreter event facilities layered over Amiga Intuition messages.

### 9.1 Dispatch commands

| Command | Syntax / role described in the book |
|---|---|
| ON MENU [Zeit] | Waits for an Intuition message and dispatches the configured handler. The book recommends calling it repeatedly in a loop; without it, ON MENU…GOSUB handlers are not dispatched. |
| ON MENU GOSUB Procedure | Registers a menu-selection handler; MENU(0) identifies the selected entry. |
| ON MENU BUTTON GOSUB Procedure | Registers a mouse-button handler; inspect MENU() to determine which button changed and how. |
| ON MENUKEY GOSUB Procedure | Registers a keyboard handler; MENU(14) provides the key code in the described interface. |
| MENU KILL | Prevents further menu selection (shown in the menu-handler example). |
| SLEEP | Waits for an Intuition message; described as a simplified ON MENU wait using time parameter 1. |

The book warns that registering ON MENU GOSUB before defining a window with a menu can crash the program. If a named handler does not exist, the corresponding monitoring is disabled.

### 9.2 MENU() event record

| Field | Meaning described in the book |
|---|---|
| MENU(0) | Menu index / selected menu entry in the simple menu interface. |
| MENU(1) | Intuition message class; values correspond to IDCMP flags requested when the window was defined. |
| MENU(2) | Message-specific data. For menu-pick messages: menu number = MENU(2) AND 31; item = SHR(MENU(2), 5) AND 63; subitem = SHR(MENU(2), 11) AND 31. |
| MENU(3) | Qualifier/modifier-key state (e.g. Shift and Control). |
| MENU(4) | Address of the Intuition object responsible for the message (e.g. Gadget or Requester). |
| MENU(5), MENU(6) | Mouse X/Y coordinates relative to the window, or movement deltas for relevant mouse-move messages. |
| MENU(7), MENU(8) | The German book describes seconds and microseconds respectively. The English handbook says MENU(7) through MENU(10) are always populated independently of message type; reconcile the complete field layout against page images before implementation. |
| MENU(10) | Address of the Window structure that generated the message; WINDOW(address) maps it to a GFA window number. |
| MENU(14) | The German book's OCR assigns a keyboard code here, but the English interpreter handbook assigns the key code to MENU(2). This is conflicting source evidence; do not implement MENU(14) as the keyboard-code field without runtime/page-image verification. |

The book lists message classes for menu-pick, keyboard, gadget, window activation/refresh/resize, mouse movement/button, and requester events. Some numeric values in the OCR are damaged or ambiguous (digit grouping/leading zeros); do not encode an IDCMP constant table from OCR alone. Check the printed page images or primary interpreter manual first.

### 9.3 Event-loop implication

The sample pattern repeatedly waits for/dispatches Intuition messages (ON MENU or SLEEP) and processes MENU() values. The book warns that internal polling loops which do not yield to event handling can prevent event processing. Do not assume this Amiga event queue/scheduling behavior matches a non-Amiga runtime.


## 10. Cross-check against the English interpreter handbook and explicit 3.5 addendum

### 10.1 What the uploaded English PDF actually establishes

The title/introductory pages identify the publication as a handbook originally printed for Version 3.0. Its opening statement says that the supplied software is GFA BASIC 3.5 and that Chapter 13 is an addendum describing the extra features in Version 3.5. The contents distinguish the original Chapters 1–12 from Chapter 13, “GFA BASIC 3.5 Addendum”.

Therefore the evidence model for this PDF is:
- Chapters 1–12: the 3.0 reference baseline (with later 3.5 software supplied alongside the book).
- Chapter 13: explicit 3.5 additions.
- A construct described only in Chapters 1–12 is a documented 3.0 feature and may be inherited by the 3.5 interpreter, but this handbook does not individually re-certify every old construct as changed/unchanged in 3.5.
- Chapter 13 entries are direct 3.5 evidence.

### 10.2 Direct 3.5 additions (English handbook, Chapter 13)

The following are explicitly placed under “New features in GFA Basic 3.5” / “GFA-BASIC 3.5 Addendum”.

| Feature / command | Evidence and recorded behavior | Addendum pages |
|---|---|---:|
| Editor changes | Editor uses two extra bytes per program row versus versions 3.0–3.07; backward scrolling is accelerated; functions can be folded; search includes headers of closed procedures/functions; listing indents labels two characters to the left. | 13-1 |
| Tab editing | TAB, CTRL+TAB, LEFTSHIFT+TAB, RIGHTSHIFT+TAB provide next/previous tab stop, insert spaces to next tab stop, and remove spaces in a row. | 13-1 |
| MAT BASE 0/1 | Controls the row/column indexing offset for the matrix operations. Intended for use with OPTION BASE 0; the addendum says the default is MAT BASE 1. | 13-1 |
| MAT CLR a() | Sets every element of a numeric vector/matrix to zero; equivalent to ARRAYFILL a(),0. | 13-2 |
| MAT SET a()=x | Sets every element of a numeric vector/matrix to scalar x; equivalent to ARRAYFILL a(),x. | 13-2 |
| MAT ONE a() | Generates the identity/unit matrix for a square matrix. | 13-2 |
| MAT READ a() | Reads a previously dimensioned matrix/vector from DATA rows. | 13-3 |
| MAT PRINT [#i,]a()[,g,n] | Outputs a vector or matrix; vectors are one row with comma-separated elements, matrix rows are line-separated; optional #i redirects output. | 13-3 |
| MAT INPUT #i,a() | Listed as a matrix input operation. Its detailed behavior should be transcribed from the page text before implementing. | 13-3 |
| MAT CPY, MAT XCPY, MAT TRANS | Listed as matrix/vector copy and transpose operations. MAT TRANS a() transposes a square matrix in place; the original is replaced, and another transpose restores it. | 13-5–13-15 |
| MAT ADD, MAT SUB, MAT MUL | Element-wise addition/subtraction, matrix/vector multiplication, and scalar/dyadic products. Detailed forms include MAT ADD a()=b()+c(), MAT ADD a(),b(), MAT ADD a(),x; MAT SUB a()=b()-c(), MAT SUB a(),b(), MAT SUB a(),x; MAT MUL a()=b()*c(), MAT MUL x=a()*b(), MAT MUL x=a()*b()*c(), MAT MUL a(),x. | 13-16–13-22 |
| MAT NORM a(),0/1 | Normalizes rows (0) or columns (1) so the sum of squares in each normalized row/column is 1. | 13-22 onward |
| MAT DET, MAT QDET, MAT RANG, MAT INV | Determinant, quick determinant, rank and inverse operations are listed in the addendum. Extracted forms: MAT DET x=a([i,j])[,n], MAT QDET x=a([i,j])[,n], MAT RANG x=a([i,j])[,n], MAT INV a()=b(). Verify exact semantics and bounds in the corresponding pages before implementation. | 13-23–13-32 |
| FACT(n) | Calculates factorial; explicitly defined with 0! = 1. | 13-33 |
| VARIAT(n,k) | Calculates variations without repetition: n!/(n-k)!. | 13-33 |
| COMBIN(n,k) | Calculates combinations without repetition: n!/((n-k)!*k!). | 13-33 |
| OPENW negative screen index | New variant: if the screen number is negative, the window opens on the current/top screen. | 13-34 |
| PCOLOR foreground,background | Does the same as COLOR, but applies to actual screen output through PRINT. | 13-34 |
| DATA / _DATA= | DATA returns the current DATA-pointer position (zero if the next READ would be out of DATA); _DATA= restores/sets the pointer to a previously captured position. | 13-34 |
| _0 … _9 | Ten special longword variables, not floating-point. They survive CLEAR, LOAD, CHAIN and RUN; only NEW erases them. Suggested use includes retaining AllocMem addresses and library bases for cleanup across RUN/CHAIN. | 13-35 |
| GURU / GURU x | Enables/disables GURU checking and exposes the GURU number. With checking enabled, a GURU meditation can be redirected to ON ERROR GOSUB or reported as error #109. The addendum warns that compiled 3.5 programs cannot use RESUME NEXT or RESUME LABEL after a GURU-triggered ON ERROR call. | 13-36 |

This list is the high-confidence 3.5 delta from the uploaded handbook. Matrix operation details beyond the signatures and semantics above remain only partially extracted; do not fill them from generic linear algebra assumptions.

### 10.3 Corrections and clarifications from the English 3.0 baseline

The English handbook is more explicit than the German book in several places. Update the research catalogue accordingly:

- Window indices are explicitly documented as 0–31 in the English handbook (printed p. 9-26), not 0–15. The 0–15 range describes the built-in default window layouts (windows 1–15), not the full index range.
- The English syntax for OPENW is shown as OPENW no[,x,y,w,h,IDCMP,flags], optionally followed by scrno and bitmap. Chapter 13 adds the negative-screen-number behavior noted above.
- ON MENU KEY GOSUB proc is the spelling shown in the English handbook. The German OCR's ON MENUKEY spelling should not be treated as normative.
- The English handbook also documents ON MESSAGE GOSUB proc, absent from the earlier German extraction. Add it to the event command inventory.
- In the English handbook, keyboard input is described as the key code in MENU(2) for ON MENU KEY; the German extraction's MENU(14) claim is not supported by the English reference and should be treated as unverified/likely erroneous.
- The English handbook says MENU(7) through MENU(10) are always set independently of message type. MENU(10) is the window address. The earlier German extraction only listed MENU(7), MENU(8) as seconds/microseconds; keep the fuller field interpretation anchored to the English manual.
- SLEEP is explicitly equivalent to ON MENU 1; the ON MENU parameter is an Intui-tick timeout (tenths of a second). The German book's shorthand agrees, but the English manual supplies the exact unit and relationship.
- The English handbook gives explicit EVERY ticks GOSUB proc, EVERY STOP, EVERY CONT, AFTER ticks GOSUB proc, AFTER STOP, and AFTER CONT. Add these control forms; the German extraction did not capture the STOP/CONT variants.
- The English RESUME description says RESUME NEXT continues after the failing command, RESUME label jumps to a label, and plain RESUME retries the failing command. For fatal errors only RESUME label is permitted. This is more precise than the German summary.
- Loop documentation confirms ENDREPEAT condition as a replacement for UNTIL condition, and writes the WHILE terminator as END WHILE (two words) in the English manual. Preserve exact spelling variants separately rather than normalizing them without parser evidence.
- FOR documentation says the loop body is executed at least once and the counter remains at the first value that exceeds the end criterion. This is the handbook's documented behavior; test it against the actual interpreter before relying on it.

### 10.4 Version classification policy for the catalogue

Use these labels in future command inventories:
- 3.5-addendum-confirmed: explicitly described in Chapter 13.
- 3.0-reference: described in Chapters 1–12, without a separate 3.5 change statement.
- German-book-only: found only in the German book; retain as a lead, not a confirmed parser/runtime fact.
- conflicting-or-OCR-uncertain: sources differ or OCR is ambiguous; do not implement until checked against a page image or runtime.
- OS-library-call: Amiga API routine, not a GFA BASIC language keyword.

This distinction avoids treating the entire 3.0 reference as a fully verified 3.5 specification while still preserving the inherited command baseline.


## 11. AmigaDOS library: detailed Chapter 5.7 extraction

This expands §4.4 from the German book's printed pp. 120–132. The chapter says there are 31 routines. The entries below are OS-library calls exposed to GFA BASIC, not BASIC statements. Signatures retain the book's parameter names and level of detail; pointer arguments are addresses, not BASIC strings passed by value.

### 11.1 File I/O routines (printed pp. 120–123)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `Open` | `Filehandle = Open(Name, Modus)` | Opens the NUL-terminated name and returns a pointer to the AmigaDOS FileHandle structure; zero means failure. Modes described: 1005 opens an existing file for reading/writing; 1006 creates a file for writing and overwrites an existing same-name file; 1004 is like 1005 but exclusive while open. The book's wording for 1005 says writing or reading, and should not be silently narrowed. |
| `Seek` | `Position = Seek(Filehandle, Abstand, Modus)` | Repositions the next-byte pointer. Mode -1 is relative to file start, 0 relative to current position, and 1 relative to file end. Positive and negative offsets are allowed. The returned Position is the pointer position after the operation; `Seek(handle,0,0)` is shown to query the current position. |
| `Write` | `Anzahl = Write(Filehandle, Puffer, Länge)` | Writes Länge bytes from memory address Puffer. Returns actual bytes written; -1 indicates an error. |
| `Read` | `Anzahl = Read(Filehandle, Puffer, Länge)` | Reads up to Länge bytes into memory at Puffer. Returns actual bytes read; -1 indicates an error. |
| `Close` | `VOID Close(Filehandle)` | Closes the file opened by Open. The argument is the FileHandle pointer returned by Open. |
| `WaitForChar` | `Status = WaitForChar(Filehandle, Timeout)` | Waits Timeout microseconds for a character on the channel. Returns 0 if none arrives in the interval, -1 if a character arrives; the character can then be read with Read. |
| `Output` | `Filehandle = Output()` | Returns the standard output channel's FileHandle. |
| `Input` | `Filehandle = Input()` | Returns the standard input channel's FileHandle. |
| `IsInteractive` | `Status = IsInteractive(Filehandle)` | Returns -1 if the channel is interactive (input/output possible), otherwise 0. |
| `IoErr` | `Fehler = IoErr()` | Returns the error number from the most recent failed operation. The book says many other routines indicate failure by returning zero; IoErr then supplies the reason. |

For the later file/directory routines, the book uses a FileHandle-like integer to identify the channel where applicable, but lock-based routines instead take a Lock structure address. Do not conflate these two handle types.

### 11.2 Lock and directory routines (printed pp. 123–129)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `Lock` | `Lockstruktur = Lock(Name, Modus)` | Looks up a file or subdirectory and creates a Lock structure; zero indicates failure. The book describes mode -2 as read access that permits shared readers and mode -1 as write access restricted to the current program. |
| `UnLock` | `VOID UnLock(Lockstruktur)` | Removes a Lock structure created by Lock or DupLock. |
| `DupLock` | `Lockstruktur2 = DupLock(Lockstruktur1)` | Copies a Lock structure. The book says only read-enabled locks (mode -2) may be duplicated. |
| `Info` | `Status = Info(Lockstruktur, Infoadresse)` | Writes disk/volume information to a memory block. The lock may refer to a volume name, file, or directory; a drive name such as `DF0:` may be used. The buffer address must be 4-byte aligned. The example allocates 36 bytes with MALLOC, calls Info, reads status at offset +11, then frees the buffer. |
| `Examine` | `Status = Examine(Lockstruktur, Infoadresse)` | Writes information about the file/directory referred to by the lock into a caller-supplied buffer. The example reserves 260 bytes with MALLOC and reads fields by offsets. |
| `ExNext` | `Status = ExNext(Lockstruktur, Infoadresse)` | Retrieves the next entry in the directory represented by the lock and fills the Examine-style information buffer. Returns zero when no further entry exists; the appendix identifies DOS error 232 as “no more entries in directory”. |
| `ParentDir` | `neue_Lockstruktur = ParentDir(Lockstruktur)` | Returns a lock for the parent directory of the directory identified by the input lock. |
| `CurrentDir` | `alte_Lockstruktur = CurrentDir(Lockstruktur)` | Makes the supplied locked directory current and returns a lock for the directory that was current before the call. |
| `CreateDir` | `Lockstruktur = CreateDir(Name)` | Creates a subdirectory in the current directory and returns a lock structure for it. |

The book's example Examine buffer layout is recorded as follows (byte offsets, as printed pp. 128–129):

| Offset | Meaning |
|---:|---|
| 0–3 | Disk number |
| 4–7 | Entry type (>0 directory; otherwise file) |
| 8–115 | Entry name storage (the text says only 30 bytes are usable) |
| 116–119 | Protection/access status |
| 120–123 | Entry type, repeated |
| 124–127 | File length in bytes |
| 128–131 | Number of allocated blocks |
| 132–143 | Creation date |
| 144–259 | Comment storage (the text says up to 80 characters are usable) |

The four low protection bits are described as: bit 0 = not deletable; bit 1 = not executable; bit 2 = not overwritable; bit 3 = not readable. The Info and Examine structures are different: the former is 36 bytes in the example and requires 4-byte alignment; the latter is 260 bytes in the example. Do not reuse one layout for the other.

### 11.3 File metadata and process routines (printed pp. 129–132)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `SetProtection` | `Status = SetProtection(Name, Modus)` | Changes protection bits for a file or subdirectory. Only the lowest four mode bits are meaningful, with each set bit enabling the corresponding restriction listed above. |
| `Rename` | `Status = Rename(alter_Name, neuer_Name)` | Renames a file or directory. |
| `DeleteFile` | `Status = DeleteFile(Name)` | Deletes a file or directory. A directory must be empty; files within it must first be deleted individually. |
| `SetComment` | `Status = SetComment(Name, Kommentar)` | Attaches a comment of up to 80 characters to a file or directory. Both arguments are passed as pointers to NUL-terminated strings in the example. |
| `LoadSeg` | `Segment = LoadSeg(Name)` | Loads a program file into memory and returns the address of its first program segment. |
| `UnLoadSeg` | `VOID UnLoadSeg(Segment)` | Unloads a segment previously loaded by LoadSeg and releases its memory. |
| `DateStamp` | `VOID DateStamp(Adresse)` | Writes current date/time in AmigaDOS format into three longwords at the supplied address. |
| `Execute` | `Status = Execute(Kommando, Input, Output)` | Executes a CLI command. Kommando is a pointer to the command text; Input and Output are FileHandles for the chosen channels. |
| `Delay` | `VOID Delay(Zeit)` | Suspends the current process for Zeit units of 1/50 second. |
| `Exit` | `VOID Exit(Parameter)` | Terminates the current process/program and frees process-allocated memory. |
| `DeviceProc` | `Prozess = DeviceProc(Name)` | Returns the process identifier currently using the named I/O channel. |
| `CreateProc` | `Prozess = CreateProc(Name, Prior, Segment, Stack)` | Creates a new process under Name using the supplied priority, program segment and stack size. The chapter gives no further ABI-level detail here. |

The chapter notes that DOS functions `GetPacket` and `QueuePacket` are not directly callable from GFA BASIC at the time described. They are therefore not counted as callable members of the chapter's 31-routine list.

### 11.4 Error and safety notes from the source

- GFA BASIC's editor checks call syntax, but the book warns that semantically invalid OS arguments may escape BASIC error handling and produce an AmigaDOS error or a Guru Meditation.
- The examples consistently append `CHR$(0)` to names/comments and pass `VARPTR(string)` addresses to OS calls. Treat this as a calling-convention requirement for these examples, not as ordinary GFA BASIC string semantics.
- The chapter's sample code uses `MALLOC`/`MFREE` for the Info and Examine buffers. Preserve the stated size/alignment requirements; the book's 36-byte Info example and 260-byte Examine example are not interchangeable.
- The German text's numeric Open modes and Lock modes are source claims. Confirm them against the target AmigaDOS/NDK documentation before treating them as ABI constants in the Rust implementation.
- This is 3.0/3.03-era reference material. The chapter does not separately label these DOS calls as new or modified in GFA BASIC 3.5.


## 12. Printer output and hardcopy: Chapter 5.8

Chapter 5.8 (printed pp. 131–139) adds printer-specific commands and explains the Amiga printer devices. These entries belong to the I/O command inventory; they are not DOS library calls.

| Command/function | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `HARDCOPY` | `HARDCOPY`; extended form `HARDCOPY Rastport, Colormap, Modi, Abst_l, Abst_r, Breite, Höhe, Spalten, Zeilen, Flags` | Prints a graphical copy of the current screen or a selected raster area. The extended form accepts RastPort and ColorMap addresses, display mode, margins, dimensions, grid and flags. The chapter says at least one window must be open on the relevant screen for HARDCOPY to work. |
| `LPOS` | `Var = LPOS(dummy)` | Returns the current virtual print-head position in the printer's character buffer (maximum 255). The dummy argument is arbitrary and has no semantic meaning. After carriage return from LPRINT, the count restarts at zero. |
| `LLIST` | The prose describes printing the current program listing; the OCR renders the command heading as `LUST` and its abbreviation as `LL`. | The command sends the current program listing to the printer. It can be interrupted using the GFA Break key combination; the interpreter returns to readiness and the printer buffer is still output. Exact spelling should be checked against the printed heading before parser use. |
| `LPRINT` | `LPRINT [,'] "Text" [[;,'] Var1 [;,']]` | Works like PRINT but sends text/strings/values to the printer. The syntax is described as matching PRINT except for the AT(Xpos,Ypos) form. On logical device PRT:, standard printer control sequences are translated according to Preferences. |

### 12.1 HARDCOPY flags

The chapter describes the following flag bits in the extended HARDCOPY form (printed pp. 132–134). The OCR/table alignment is damaged, so retain the descriptions as source evidence and verify bit positions against the page image before encoding constants:

- Selects the horizontal and vertical scaling interpretation (1/1000-inch units, maximum width/height, or divisors of maximum width/height).
- Centers the printout.
- Preserves the screen image's aspect ratio on paper.
- Selects printer resolution; the text describes a three-bit field from lowest (`001`) to highest (`111`) resolution.
- Suppresses the final page feed, allowing multiple hardcopies to be joined.

### 12.2 Printer devices

The chapter distinguishes:
- `PRT:`: logical printer device with standard control-sequence translation based on Preferences.
- `PAR:`: parallel printer interface; sends control characters unfiltered, bypassing Preferences translation. The example opens it with `OPEN "O",#1,"PAR:"`, writes via `PRINT #1,...`, then closes it.
- `SER:`: serial interface, described as using the same file-open/write/close approach as PAR:.

The printer chapter includes extensive ESC/P-style control-sequence tables and examples for font, pitch, margins, tabs and character sets. Those strings are device-control payloads, not additional GFA BASIC keywords; they should be catalogued separately if printer emulation becomes an implementation target.


## 13. Sound and speech output: Chapter 5.9–5.10

This completes the remaining sound/speech material in Chapter 5 (printed pp. 139–144). It is source-derived from the German 1990 book and therefore belongs to the 3.0/V3.03 reference layer, not to the explicit 3.5 addendum.

### 13.1 SOUND and WAVE (printed pp. 139–141)

| Command | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `SOUND` | `SOUND Frequenz, Dauer [,Lautstärke] [,Kanal]` | Plays a tone on one of four Amiga sound channels. Frequency range: 20–15,000 Hz. Duration is a 16-bit value described as the number of repetitions of the WAVE envelope. Volume range: 0–255, default 127, linear. Channel range: 0–3, default 0. |
| `WAVE` | `WAVE Kanal, Hüllkurve()` | Sets the waveform/envelope for the selected channel. The envelope is a 256-element array; each element is in the range −128 to +127. The book recommends an integer-byte array for this data. |

Additional details stated by the book:

- Four independent channels are available, numbered 0–3.
- Stereo routing is fixed in the described setup: channels 0 and 3 are sent left; channels 1 and 2 right.
- If SOUND is used before a WAVE has been set, the interpreter generates a sine waveform for that channel.
- The sine waveform can also be selected explicitly with `WAVE Kanal,SIN()`.
- The note table gives note frequencies by octave; each octave doubles the frequency. Its examples identify A as 440 Hz in octave 3, 880 Hz in octave 4 and 1,760 Hz in octave 5.

The OCR extraction identifies the array as a byte-integer array but its displayed type-prefix character is not sufficiently reliable to transcribe as normative syntax. Confirm the exact declaration spelling from the page image or runtime before using it in parser tests.

### 13.2 SAY and voice parameters (printed pp. 142–143)

| Command | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `SAY` | `SAY Sprech$ [,Modi%()]` | Speaks the supplied phoneme-code string. The optional seven-element integer array controls voice parameters. |
| `TRANSLATE$` | `Sprech$=TRANSLATE$("Text")` | Converts natural-language text into phoneme code suitable for SAY. The book warns that the translator applies English pronunciation rules; German text may therefore sound unintelligible. |

The seven voice-control array elements are described as follows:

| Element | Meaning | Range / choices | Default |
|---:|---|---|---:|
| `Modi%(0)` | Sampling frequency; mainly affects voice pitch | 5,000–28,000 | 22,000 |
| `Modi%(1)` | Voice gender | 0 = male; 1 = female | 0 |
| `Modi%(2)` | Speaking speed, words per minute | 40–400 | 150 |
| `Modi%(3)` | Voice modulation | 0 = emphatic; 1 = monotone | 0 |
| `Modi%(4)` | Voice frequency/pitch | 65–320 | 110 |
| `Modi%(5)` | Voice volume | 0–64 | 64 |
| `Modi%(6)` | Output channel selection | 0–11, mapping below | 10 |

The channel-selection values for `Modi%(6)` are printed as:

| Value | Selected output |
|---:|---|
| 0 | Channel 0 |
| 1 | Channel 1 |
| 2 | Channel 2 |
| 3 | Channel 3 |
| 4 | Channels 0 and 1 |
| 5 | Channels 0 and 2 |
| 6 | Channels 3 and 1 |
| 7 | Channels 3 and 2 |
| 8 | Channel 0 and/or 3 |
| 9 | Channel 1 and/or 2 |
| 10 | Any free channel pair (default) |
| 11 | Any free channel |

The book says SAY requires the Amiga Narrator device in the boot disk's DEVS directory. If it is unavailable, the system may request the boot disk. It also gives an example of changing the voice from male to female by setting the second element (`Modi%(1)`) to 1.

### 13.3 Combining SAY and TRANSLATE$

The book explicitly permits the direct form:

```basic
SAY TRANSLATE$("HALLO")
```

It also notes that text can be rewritten phonetically to improve German output because TRANSLATE$ uses English pronunciation rules. The examples are suggestions for experimentation, not general conversion rules.

### 13.4 Evidence boundary

These commands and parameter descriptions are documented in the German book's 3.0/V3.03-era reference. The book does not identify them as 3.5 additions. The source's exact byte-array type prefix remains OCR-uncertain; the Narrator device requirement and channel map are retained as printed rather than replaced with modern Amiga audio assumptions.


## 14. Program text, DATA records and variable declarations: Chapter 6.3–6.4

This section captures the Chapter 6 material not yet detailed in §7: comments, DATA-pointer operations and global variable-type declarations. Source: German 1990 book, printed pp. 166–172 (3.0/V3.03-era reference).

### 14.1 Comments

| Form | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `REM` | `REM [Kommentar]` | Introduces a comment line; the text is ignored during program execution. |
| Apostrophe | `' Kommentar` | Alternative comment marker for REM. |
| Exclamation mark | `Befehlszeile ! Kommentartext` | Introduces an inline comment after a statement. At the beginning of a line, the editor converts it to the REM apostrophe form. |

The book warns that the inline comment marker is not available in the same way inside DATA lines, where the text is treated as DATA content.

### 14.2 DATA, READ and RESTORE

| Statement | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `DATA` | `DATA [Wertedatas [, ["Textdatas"] [, ...]]]` | Stores comma-separated numeric and/or text data for later READ statements. Text DATA does not generally need quotation marks unless the text contains commas. Without READ statements, DATA text is ignored like REM content. |
| `READ` | `READ Var [,Var2,Var3,Var2$, ...]` | Assigns successive DATA entries to the listed variables. Numeric READ requires numeric DATA; text READ treats a numeric-looking entry as text. Variables cannot be supplied as DATA values. |
| `RESTORE` | `RESTORE [Labelname]` | Resets the DATA pointer to the first DATA entry, or to the first DATA entry following the named label. The book also says a label may be used both as a GOTO target and as a RESTORE target. |

Additional constraints and observations:

- DATA entries are consumed in sequence. If more READ operations execute than there are DATA entries before program end, the interpreter reports an error.
- Numeric DATA may be written in decimal, binary, hexadecimal or octal notation.
- Text DATA entries are separated by commas. The book explains that quotation marks are needed when a text entry contains a comma, otherwise that comma is interpreted as a DATA separator.
- If a text DATA item is unquoted, the characters between its delimiting commas—including spaces—are read as text.
- RESTORE without a label points to the first DATA entry in the program.

### 14.3 Global variable-type declarations

The book describes DEF declarations as global naming rules that assign a variable type based on its leading character(s). A declaration string may contain individual letters and ranges. Examples in the book include `"a-c,x-z"` and `"i1,i2,i3"`.

| Declaration | Syntax | Type assigned by the book |
|---|---|---|
| `DEFBIT` | `DEFBIT Defstring$` | Boolean variables (the book's example assigns names beginning with `a`; Boolean suffix shown as `!`). |
| `DEFBYT` | `DEFBYT Defstring# German Amiga GFA-BASIC book: command and OS API extraction

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


## 8. Program control and error handling: Chapter 12

Extracted from printed pp. 331–361.

### 8.1 Lifecycle commands

| Command | Behavior described by the book |
|---|---|
| RUN | Restarts the current program and clears variables/screen. RUN "program-name" is also shown for loading and starting another program, with version/context caveats. |
| STOP | Suspends execution and displays an ALERT box, allowing continuation or entry to direct mode. |
| CONT | Continues after STOP at the following statement. Not available after CLEAR, listing edits, or introduction of new variables. |
| END | Stops the current program but retains variable contents and open files in the interpreter; direct mode can inspect them and CONT may resume. |
| SYSTEM / QUIT | Exits to Workbench or the caller (CLI/EXEC). QUIT is described as identical to SYSTEM; QUIT [x] is marked Version 3.0. |
| CLEAR | Sets numeric variables to zero, strings to empty, erases arrays and removes dimensions. The book says not to use it inside procedures or FOR/NEXT loops. |
| NEW | Clears the BASIC program and its variables from memory. |
| EDIT | Exits execution to the editor. |

### 8.2 Error handling

- ON ERROR GOSUB Procedure redirects BASIC/system errors to a procedure; the programmer owns error handling.
- ON ERROR restores normal error reporting/termination. The editor may insert GOSUB automatically in the described 3.0 environment.
- ERR is a reserved variable containing the error identifier.
- ERR$(index) returns the text for an error number; the book labels this function Version 3.0.
- FATAL distinguishes fatal system errors (-1) from ordinary BASIC errors (0), according to the book.
- ERROR number deliberately raises/simulates the selected error.
- RESUME is allowed only inside a procedure. RESUME retries the failed statement; RESUME NEXT continues with the following statement; RESUME Label continues at a label. After the error procedure returns, the interpreter resets error handling to normal, so ON ERROR GOSUB must be installed again if required.

Do not assume the book's special fatal-system-error recovery is reproducible in a modern host runtime.

### 8.3 Timer callbacks

The book states that the timer unit is 1/200 second (200 ticks = 1 second).

- AFTER Ticks [GOSUB] Procedure schedules a one-shot callback. It is dispatched only after the current BASIC statement completes; blocking operations can delay it.
- A later AFTER GOSUB replaces the previous pending callback. The described implementation cannot process multiple AFTER callbacks simultaneously.
- EVERY GOSUB is described as the periodic counterpart. The book suggests re-arming an AFTER handler before RETURN as an approximation; the extracted text does not establish complete EVERY edge-case semantics.

## 9. Intuition event handling: Chapters 15–16

Extracted from printed pp. 389–404. These are interpreter event facilities layered over Amiga Intuition messages.

### 9.1 Dispatch commands

| Command | Syntax / role described in the book |
|---|---|
| ON MENU [Zeit] | Waits for an Intuition message and dispatches the configured handler. The book recommends calling it repeatedly in a loop; without it, ON MENU…GOSUB handlers are not dispatched. |
| ON MENU GOSUB Procedure | Registers a menu-selection handler; MENU(0) identifies the selected entry. |
| ON MENU BUTTON GOSUB Procedure | Registers a mouse-button handler; inspect MENU() to determine which button changed and how. |
| ON MENUKEY GOSUB Procedure | Registers a keyboard handler; MENU(14) provides the key code in the described interface. |
| MENU KILL | Prevents further menu selection (shown in the menu-handler example). |
| SLEEP | Waits for an Intuition message; described as a simplified ON MENU wait using time parameter 1. |

The book warns that registering ON MENU GOSUB before defining a window with a menu can crash the program. If a named handler does not exist, the corresponding monitoring is disabled.

### 9.2 MENU() event record

| Field | Meaning described in the book |
|---|---|
| MENU(0) | Menu index / selected menu entry in the simple menu interface. |
| MENU(1) | Intuition message class; values correspond to IDCMP flags requested when the window was defined. |
| MENU(2) | Message-specific data. For menu-pick messages: menu number = MENU(2) AND 31; item = SHR(MENU(2), 5) AND 63; subitem = SHR(MENU(2), 11) AND 31. |
| MENU(3) | Qualifier/modifier-key state (e.g. Shift and Control). |
| MENU(4) | Address of the Intuition object responsible for the message (e.g. Gadget or Requester). |
| MENU(5), MENU(6) | Mouse X/Y coordinates relative to the window, or movement deltas for relevant mouse-move messages. |
| MENU(7), MENU(8) | The German book describes seconds and microseconds respectively. The English handbook says MENU(7) through MENU(10) are always populated independently of message type; reconcile the complete field layout against page images before implementation. |
| MENU(10) | Address of the Window structure that generated the message; WINDOW(address) maps it to a GFA window number. |
| MENU(14) | The German book's OCR assigns a keyboard code here, but the English interpreter handbook assigns the key code to MENU(2). This is conflicting source evidence; do not implement MENU(14) as the keyboard-code field without runtime/page-image verification. |

The book lists message classes for menu-pick, keyboard, gadget, window activation/refresh/resize, mouse movement/button, and requester events. Some numeric values in the OCR are damaged or ambiguous (digit grouping/leading zeros); do not encode an IDCMP constant table from OCR alone. Check the printed page images or primary interpreter manual first.

### 9.3 Event-loop implication

The sample pattern repeatedly waits for/dispatches Intuition messages (ON MENU or SLEEP) and processes MENU() values. The book warns that internal polling loops which do not yield to event handling can prevent event processing. Do not assume this Amiga event queue/scheduling behavior matches a non-Amiga runtime.


## 10. Cross-check against the English interpreter handbook and explicit 3.5 addendum

### 10.1 What the uploaded English PDF actually establishes

The title/introductory pages identify the publication as a handbook originally printed for Version 3.0. Its opening statement says that the supplied software is GFA BASIC 3.5 and that Chapter 13 is an addendum describing the extra features in Version 3.5. The contents distinguish the original Chapters 1–12 from Chapter 13, “GFA BASIC 3.5 Addendum”.

Therefore the evidence model for this PDF is:
- Chapters 1–12: the 3.0 reference baseline (with later 3.5 software supplied alongside the book).
- Chapter 13: explicit 3.5 additions.
- A construct described only in Chapters 1–12 is a documented 3.0 feature and may be inherited by the 3.5 interpreter, but this handbook does not individually re-certify every old construct as changed/unchanged in 3.5.
- Chapter 13 entries are direct 3.5 evidence.

### 10.2 Direct 3.5 additions (English handbook, Chapter 13)

The following are explicitly placed under “New features in GFA Basic 3.5” / “GFA-BASIC 3.5 Addendum”.

| Feature / command | Evidence and recorded behavior | Addendum pages |
|---|---|---:|
| Editor changes | Editor uses two extra bytes per program row versus versions 3.0–3.07; backward scrolling is accelerated; functions can be folded; search includes headers of closed procedures/functions; listing indents labels two characters to the left. | 13-1 |
| Tab editing | TAB, CTRL+TAB, LEFTSHIFT+TAB, RIGHTSHIFT+TAB provide next/previous tab stop, insert spaces to next tab stop, and remove spaces in a row. | 13-1 |
| MAT BASE 0/1 | Controls the row/column indexing offset for the matrix operations. Intended for use with OPTION BASE 0; the addendum says the default is MAT BASE 1. | 13-1 |
| MAT CLR a() | Sets every element of a numeric vector/matrix to zero; equivalent to ARRAYFILL a(),0. | 13-2 |
| MAT SET a()=x | Sets every element of a numeric vector/matrix to scalar x; equivalent to ARRAYFILL a(),x. | 13-2 |
| MAT ONE a() | Generates the identity/unit matrix for a square matrix. | 13-2 |
| MAT READ a() | Reads a previously dimensioned matrix/vector from DATA rows. | 13-3 |
| MAT PRINT [#i,]a()[,g,n] | Outputs a vector or matrix; vectors are one row with comma-separated elements, matrix rows are line-separated; optional #i redirects output. | 13-3 |
| MAT INPUT #i,a() | Listed as a matrix input operation. Its detailed behavior should be transcribed from the page text before implementing. | 13-3 |
| MAT CPY, MAT XCPY, MAT TRANS | Listed as matrix/vector copy and transpose operations. MAT TRANS a() transposes a square matrix in place; the original is replaced, and another transpose restores it. | 13-5–13-15 |
| MAT ADD, MAT SUB, MAT MUL | Element-wise addition/subtraction, matrix/vector multiplication, and scalar/dyadic products. Detailed forms include MAT ADD a()=b()+c(), MAT ADD a(),b(), MAT ADD a(),x; MAT SUB a()=b()-c(), MAT SUB a(),b(), MAT SUB a(),x; MAT MUL a()=b()*c(), MAT MUL x=a()*b(), MAT MUL x=a()*b()*c(), MAT MUL a(),x. | 13-16–13-22 |
| MAT NORM a(),0/1 | Normalizes rows (0) or columns (1) so the sum of squares in each normalized row/column is 1. | 13-22 onward |
| MAT DET, MAT QDET, MAT RANG, MAT INV | Determinant, quick determinant, rank and inverse operations are listed in the addendum. Extracted forms: MAT DET x=a([i,j])[,n], MAT QDET x=a([i,j])[,n], MAT RANG x=a([i,j])[,n], MAT INV a()=b(). Verify exact semantics and bounds in the corresponding pages before implementation. | 13-23–13-32 |
| FACT(n) | Calculates factorial; explicitly defined with 0! = 1. | 13-33 |
| VARIAT(n,k) | Calculates variations without repetition: n!/(n-k)!. | 13-33 |
| COMBIN(n,k) | Calculates combinations without repetition: n!/((n-k)!*k!). | 13-33 |
| OPENW negative screen index | New variant: if the screen number is negative, the window opens on the current/top screen. | 13-34 |
| PCOLOR foreground,background | Does the same as COLOR, but applies to actual screen output through PRINT. | 13-34 |
| DATA / _DATA= | DATA returns the current DATA-pointer position (zero if the next READ would be out of DATA); _DATA= restores/sets the pointer to a previously captured position. | 13-34 |
| _0 … _9 | Ten special longword variables, not floating-point. They survive CLEAR, LOAD, CHAIN and RUN; only NEW erases them. Suggested use includes retaining AllocMem addresses and library bases for cleanup across RUN/CHAIN. | 13-35 |
| GURU / GURU x | Enables/disables GURU checking and exposes the GURU number. With checking enabled, a GURU meditation can be redirected to ON ERROR GOSUB or reported as error #109. The addendum warns that compiled 3.5 programs cannot use RESUME NEXT or RESUME LABEL after a GURU-triggered ON ERROR call. | 13-36 |

This list is the high-confidence 3.5 delta from the uploaded handbook. Matrix operation details beyond the signatures and semantics above remain only partially extracted; do not fill them from generic linear algebra assumptions.

### 10.3 Corrections and clarifications from the English 3.0 baseline

The English handbook is more explicit than the German book in several places. Update the research catalogue accordingly:

- Window indices are explicitly documented as 0–31 in the English handbook (printed p. 9-26), not 0–15. The 0–15 range describes the built-in default window layouts (windows 1–15), not the full index range.
- The English syntax for OPENW is shown as OPENW no[,x,y,w,h,IDCMP,flags], optionally followed by scrno and bitmap. Chapter 13 adds the negative-screen-number behavior noted above.
- ON MENU KEY GOSUB proc is the spelling shown in the English handbook. The German OCR's ON MENUKEY spelling should not be treated as normative.
- The English handbook also documents ON MESSAGE GOSUB proc, absent from the earlier German extraction. Add it to the event command inventory.
- In the English handbook, keyboard input is described as the key code in MENU(2) for ON MENU KEY; the German extraction's MENU(14) claim is not supported by the English reference and should be treated as unverified/likely erroneous.
- The English handbook says MENU(7) through MENU(10) are always set independently of message type. MENU(10) is the window address. The earlier German extraction only listed MENU(7), MENU(8) as seconds/microseconds; keep the fuller field interpretation anchored to the English manual.
- SLEEP is explicitly equivalent to ON MENU 1; the ON MENU parameter is an Intui-tick timeout (tenths of a second). The German book's shorthand agrees, but the English manual supplies the exact unit and relationship.
- The English handbook gives explicit EVERY ticks GOSUB proc, EVERY STOP, EVERY CONT, AFTER ticks GOSUB proc, AFTER STOP, and AFTER CONT. Add these control forms; the German extraction did not capture the STOP/CONT variants.
- The English RESUME description says RESUME NEXT continues after the failing command, RESUME label jumps to a label, and plain RESUME retries the failing command. For fatal errors only RESUME label is permitted. This is more precise than the German summary.
- Loop documentation confirms ENDREPEAT condition as a replacement for UNTIL condition, and writes the WHILE terminator as END WHILE (two words) in the English manual. Preserve exact spelling variants separately rather than normalizing them without parser evidence.
- FOR documentation says the loop body is executed at least once and the counter remains at the first value that exceeds the end criterion. This is the handbook's documented behavior; test it against the actual interpreter before relying on it.

### 10.4 Version classification policy for the catalogue

Use these labels in future command inventories:
- 3.5-addendum-confirmed: explicitly described in Chapter 13.
- 3.0-reference: described in Chapters 1–12, without a separate 3.5 change statement.
- German-book-only: found only in the German book; retain as a lead, not a confirmed parser/runtime fact.
- conflicting-or-OCR-uncertain: sources differ or OCR is ambiguous; do not implement until checked against a page image or runtime.
- OS-library-call: Amiga API routine, not a GFA BASIC language keyword.

This distinction avoids treating the entire 3.0 reference as a fully verified 3.5 specification while still preserving the inherited command baseline.


## 11. AmigaDOS library: detailed Chapter 5.7 extraction

This expands §4.4 from the German book's printed pp. 120–132. The chapter says there are 31 routines. The entries below are OS-library calls exposed to GFA BASIC, not BASIC statements. Signatures retain the book's parameter names and level of detail; pointer arguments are addresses, not BASIC strings passed by value.

### 11.1 File I/O routines (printed pp. 120–123)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `Open` | `Filehandle = Open(Name, Modus)` | Opens the NUL-terminated name and returns a pointer to the AmigaDOS FileHandle structure; zero means failure. Modes described: 1005 opens an existing file for reading/writing; 1006 creates a file for writing and overwrites an existing same-name file; 1004 is like 1005 but exclusive while open. The book's wording for 1005 says writing or reading, and should not be silently narrowed. |
| `Seek` | `Position = Seek(Filehandle, Abstand, Modus)` | Repositions the next-byte pointer. Mode -1 is relative to file start, 0 relative to current position, and 1 relative to file end. Positive and negative offsets are allowed. The returned Position is the pointer position after the operation; `Seek(handle,0,0)` is shown to query the current position. |
| `Write` | `Anzahl = Write(Filehandle, Puffer, Länge)` | Writes Länge bytes from memory address Puffer. Returns actual bytes written; -1 indicates an error. |
| `Read` | `Anzahl = Read(Filehandle, Puffer, Länge)` | Reads up to Länge bytes into memory at Puffer. Returns actual bytes read; -1 indicates an error. |
| `Close` | `VOID Close(Filehandle)` | Closes the file opened by Open. The argument is the FileHandle pointer returned by Open. |
| `WaitForChar` | `Status = WaitForChar(Filehandle, Timeout)` | Waits Timeout microseconds for a character on the channel. Returns 0 if none arrives in the interval, -1 if a character arrives; the character can then be read with Read. |
| `Output` | `Filehandle = Output()` | Returns the standard output channel's FileHandle. |
| `Input` | `Filehandle = Input()` | Returns the standard input channel's FileHandle. |
| `IsInteractive` | `Status = IsInteractive(Filehandle)` | Returns -1 if the channel is interactive (input/output possible), otherwise 0. |
| `IoErr` | `Fehler = IoErr()` | Returns the error number from the most recent failed operation. The book says many other routines indicate failure by returning zero; IoErr then supplies the reason. |

For the later file/directory routines, the book uses a FileHandle-like integer to identify the channel where applicable, but lock-based routines instead take a Lock structure address. Do not conflate these two handle types.

### 11.2 Lock and directory routines (printed pp. 123–129)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `Lock` | `Lockstruktur = Lock(Name, Modus)` | Looks up a file or subdirectory and creates a Lock structure; zero indicates failure. The book describes mode -2 as read access that permits shared readers and mode -1 as write access restricted to the current program. |
| `UnLock` | `VOID UnLock(Lockstruktur)` | Removes a Lock structure created by Lock or DupLock. |
| `DupLock` | `Lockstruktur2 = DupLock(Lockstruktur1)` | Copies a Lock structure. The book says only read-enabled locks (mode -2) may be duplicated. |
| `Info` | `Status = Info(Lockstruktur, Infoadresse)` | Writes disk/volume information to a memory block. The lock may refer to a volume name, file, or directory; a drive name such as `DF0:` may be used. The buffer address must be 4-byte aligned. The example allocates 36 bytes with MALLOC, calls Info, reads status at offset +11, then frees the buffer. |
| `Examine` | `Status = Examine(Lockstruktur, Infoadresse)` | Writes information about the file/directory referred to by the lock into a caller-supplied buffer. The example reserves 260 bytes with MALLOC and reads fields by offsets. |
| `ExNext` | `Status = ExNext(Lockstruktur, Infoadresse)` | Retrieves the next entry in the directory represented by the lock and fills the Examine-style information buffer. Returns zero when no further entry exists; the appendix identifies DOS error 232 as “no more entries in directory”. |
| `ParentDir` | `neue_Lockstruktur = ParentDir(Lockstruktur)` | Returns a lock for the parent directory of the directory identified by the input lock. |
| `CurrentDir` | `alte_Lockstruktur = CurrentDir(Lockstruktur)` | Makes the supplied locked directory current and returns a lock for the directory that was current before the call. |
| `CreateDir` | `Lockstruktur = CreateDir(Name)` | Creates a subdirectory in the current directory and returns a lock structure for it. |

The book's example Examine buffer layout is recorded as follows (byte offsets, as printed pp. 128–129):

| Offset | Meaning |
|---:|---|
| 0–3 | Disk number |
| 4–7 | Entry type (>0 directory; otherwise file) |
| 8–115 | Entry name storage (the text says only 30 bytes are usable) |
| 116–119 | Protection/access status |
| 120–123 | Entry type, repeated |
| 124–127 | File length in bytes |
| 128–131 | Number of allocated blocks |
| 132–143 | Creation date |
| 144–259 | Comment storage (the text says up to 80 characters are usable) |

The four low protection bits are described as: bit 0 = not deletable; bit 1 = not executable; bit 2 = not overwritable; bit 3 = not readable. The Info and Examine structures are different: the former is 36 bytes in the example and requires 4-byte alignment; the latter is 260 bytes in the example. Do not reuse one layout for the other.

### 11.3 File metadata and process routines (printed pp. 129–132)

| Routine | Signature | Source-derived behavior and constraints |
|---|---|---|
| `SetProtection` | `Status = SetProtection(Name, Modus)` | Changes protection bits for a file or subdirectory. Only the lowest four mode bits are meaningful, with each set bit enabling the corresponding restriction listed above. |
| `Rename` | `Status = Rename(alter_Name, neuer_Name)` | Renames a file or directory. |
| `DeleteFile` | `Status = DeleteFile(Name)` | Deletes a file or directory. A directory must be empty; files within it must first be deleted individually. |
| `SetComment` | `Status = SetComment(Name, Kommentar)` | Attaches a comment of up to 80 characters to a file or directory. Both arguments are passed as pointers to NUL-terminated strings in the example. |
| `LoadSeg` | `Segment = LoadSeg(Name)` | Loads a program file into memory and returns the address of its first program segment. |
| `UnLoadSeg` | `VOID UnLoadSeg(Segment)` | Unloads a segment previously loaded by LoadSeg and releases its memory. |
| `DateStamp` | `VOID DateStamp(Adresse)` | Writes current date/time in AmigaDOS format into three longwords at the supplied address. |
| `Execute` | `Status = Execute(Kommando, Input, Output)` | Executes a CLI command. Kommando is a pointer to the command text; Input and Output are FileHandles for the chosen channels. |
| `Delay` | `VOID Delay(Zeit)` | Suspends the current process for Zeit units of 1/50 second. |
| `Exit` | `VOID Exit(Parameter)` | Terminates the current process/program and frees process-allocated memory. |
| `DeviceProc` | `Prozess = DeviceProc(Name)` | Returns the process identifier currently using the named I/O channel. |
| `CreateProc` | `Prozess = CreateProc(Name, Prior, Segment, Stack)` | Creates a new process under Name using the supplied priority, program segment and stack size. The chapter gives no further ABI-level detail here. |

The chapter notes that DOS functions `GetPacket` and `QueuePacket` are not directly callable from GFA BASIC at the time described. They are therefore not counted as callable members of the chapter's 31-routine list.

### 11.4 Error and safety notes from the source

- GFA BASIC's editor checks call syntax, but the book warns that semantically invalid OS arguments may escape BASIC error handling and produce an AmigaDOS error or a Guru Meditation.
- The examples consistently append `CHR$(0)` to names/comments and pass `VARPTR(string)` addresses to OS calls. Treat this as a calling-convention requirement for these examples, not as ordinary GFA BASIC string semantics.
- The chapter's sample code uses `MALLOC`/`MFREE` for the Info and Examine buffers. Preserve the stated size/alignment requirements; the book's 36-byte Info example and 260-byte Examine example are not interchangeable.
- The German text's numeric Open modes and Lock modes are source claims. Confirm them against the target AmigaDOS/NDK documentation before treating them as ABI constants in the Rust implementation.
- This is 3.0/3.03-era reference material. The chapter does not separately label these DOS calls as new or modified in GFA BASIC 3.5.


## 12. Printer output and hardcopy: Chapter 5.8

Chapter 5.8 (printed pp. 131–139) adds printer-specific commands and explains the Amiga printer devices. These entries belong to the I/O command inventory; they are not DOS library calls.

| Command/function | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `HARDCOPY` | `HARDCOPY`; extended form `HARDCOPY Rastport, Colormap, Modi, Abst_l, Abst_r, Breite, Höhe, Spalten, Zeilen, Flags` | Prints a graphical copy of the current screen or a selected raster area. The extended form accepts RastPort and ColorMap addresses, display mode, margins, dimensions, grid and flags. The chapter says at least one window must be open on the relevant screen for HARDCOPY to work. |
| `LPOS` | `Var = LPOS(dummy)` | Returns the current virtual print-head position in the printer's character buffer (maximum 255). The dummy argument is arbitrary and has no semantic meaning. After carriage return from LPRINT, the count restarts at zero. |
| `LLIST` | The prose describes printing the current program listing; the OCR renders the command heading as `LUST` and its abbreviation as `LL`. | The command sends the current program listing to the printer. It can be interrupted using the GFA Break key combination; the interpreter returns to readiness and the printer buffer is still output. Exact spelling should be checked against the printed heading before parser use. |
| `LPRINT` | `LPRINT [,'] "Text" [[;,'] Var1 [;,']]` | Works like PRINT but sends text/strings/values to the printer. The syntax is described as matching PRINT except for the AT(Xpos,Ypos) form. On logical device PRT:, standard printer control sequences are translated according to Preferences. |

### 12.1 HARDCOPY flags

The chapter describes the following flag bits in the extended HARDCOPY form (printed pp. 132–134). The OCR/table alignment is damaged, so retain the descriptions as source evidence and verify bit positions against the page image before encoding constants:

- Selects the horizontal and vertical scaling interpretation (1/1000-inch units, maximum width/height, or divisors of maximum width/height).
- Centers the printout.
- Preserves the screen image's aspect ratio on paper.
- Selects printer resolution; the text describes a three-bit field from lowest (`001`) to highest (`111`) resolution.
- Suppresses the final page feed, allowing multiple hardcopies to be joined.

### 12.2 Printer devices

The chapter distinguishes:
- `PRT:`: logical printer device with standard control-sequence translation based on Preferences.
- `PAR:`: parallel printer interface; sends control characters unfiltered, bypassing Preferences translation. The example opens it with `OPEN "O",#1,"PAR:"`, writes via `PRINT #1,...`, then closes it.
- `SER:`: serial interface, described as using the same file-open/write/close approach as PAR:.

The printer chapter includes extensive ESC/P-style control-sequence tables and examples for font, pitch, margins, tabs and character sets. Those strings are device-control payloads, not additional GFA BASIC keywords; they should be catalogued separately if printer emulation becomes an implementation target.


## 13. Sound and speech output: Chapter 5.9–5.10

This completes the remaining sound/speech material in Chapter 5 (printed pp. 139–144). It is source-derived from the German 1990 book and therefore belongs to the 3.0/V3.03 reference layer, not to the explicit 3.5 addendum.

### 13.1 SOUND and WAVE (printed pp. 139–141)

| Command | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `SOUND` | `SOUND Frequenz, Dauer [,Lautstärke] [,Kanal]` | Plays a tone on one of four Amiga sound channels. Frequency range: 20–15,000 Hz. Duration is a 16-bit value described as the number of repetitions of the WAVE envelope. Volume range: 0–255, default 127, linear. Channel range: 0–3, default 0. |
| `WAVE` | `WAVE Kanal, Hüllkurve()` | Sets the waveform/envelope for the selected channel. The envelope is a 256-element array; each element is in the range −128 to +127. The book recommends an integer-byte array for this data. |

Additional details stated by the book:

- Four independent channels are available, numbered 0–3.
- Stereo routing is fixed in the described setup: channels 0 and 3 are sent left; channels 1 and 2 right.
- If SOUND is used before a WAVE has been set, the interpreter generates a sine waveform for that channel.
- The sine waveform can also be selected explicitly with `WAVE Kanal,SIN()`.
- The note table gives note frequencies by octave; each octave doubles the frequency. Its examples identify A as 440 Hz in octave 3, 880 Hz in octave 4 and 1,760 Hz in octave 5.

The OCR extraction identifies the array as a byte-integer array but its displayed type-prefix character is not sufficiently reliable to transcribe as normative syntax. Confirm the exact declaration spelling from the page image or runtime before using it in parser tests.

### 13.2 SAY and voice parameters (printed pp. 142–143)

| Command | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `SAY` | `SAY Sprech$ [,Modi%()]` | Speaks the supplied phoneme-code string. The optional seven-element integer array controls voice parameters. |
| `TRANSLATE$` | `Sprech$=TRANSLATE$("Text")` | Converts natural-language text into phoneme code suitable for SAY. The book warns that the translator applies English pronunciation rules; German text may therefore sound unintelligible. |

The seven voice-control array elements are described as follows:

| Element | Meaning | Range / choices | Default |
|---:|---|---|---:|
| `Modi%(0)` | Sampling frequency; mainly affects voice pitch | 5,000–28,000 | 22,000 |
| `Modi%(1)` | Voice gender | 0 = male; 1 = female | 0 |
| `Modi%(2)` | Speaking speed, words per minute | 40–400 | 150 |
| `Modi%(3)` | Voice modulation | 0 = emphatic; 1 = monotone | 0 |
| `Modi%(4)` | Voice frequency/pitch | 65–320 | 110 |
| `Modi%(5)` | Voice volume | 0–64 | 64 |
| `Modi%(6)` | Output channel selection | 0–11, mapping below | 10 |

The channel-selection values for `Modi%(6)` are printed as:

| Value | Selected output |
|---:|---|
| 0 | Channel 0 |
| 1 | Channel 1 |
| 2 | Channel 2 |
| 3 | Channel 3 |
| 4 | Channels 0 and 1 |
| 5 | Channels 0 and 2 |
| 6 | Channels 3 and 1 |
| 7 | Channels 3 and 2 |
| 8 | Channel 0 and/or 3 |
| 9 | Channel 1 and/or 2 |
| 10 | Any free channel pair (default) |
| 11 | Any free channel |

The book says SAY requires the Amiga Narrator device in the boot disk's DEVS directory. If it is unavailable, the system may request the boot disk. It also gives an example of changing the voice from male to female by setting the second element (`Modi%(1)`) to 1.

### 13.3 Combining SAY and TRANSLATE$

The book explicitly permits the direct form:

```basic
SAY TRANSLATE$("HALLO")
```

It also notes that text can be rewritten phonetically to improve German output because TRANSLATE$ uses English pronunciation rules. The examples are suggestions for experimentation, not general conversion rules.

### 13.4 Evidence boundary

These commands and parameter descriptions are documented in the German book's 3.0/V3.03-era reference. The book does not identify them as 3.5 additions. The source's exact byte-array type prefix remains OCR-uncertain; the Narrator device requirement and channel map are retained as printed rather than replaced with modern Amiga audio assumptions.


## 14. Program text, DATA records and variable declarations: Chapter 6.3–6.4

This section captures the Chapter 6 material not yet detailed in §7: comments, DATA-pointer operations and global variable-type declarations. Source: German 1990 book, printed pp. 166–172 (3.0/V3.03-era reference).

### 14.1 Comments

| Form | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `REM` | `REM [Kommentar]` | Introduces a comment line; the text is ignored during program execution. |
| Apostrophe | `' Kommentar` | Alternative comment marker for REM. |
| Exclamation mark | `Befehlszeile ! Kommentartext` | Introduces an inline comment after a statement. At the beginning of a line, the editor converts it to the REM apostrophe form. |

The book warns that the inline comment marker is not available in the same way inside DATA lines, where the text is treated as DATA content.

### 14.2 DATA, READ and RESTORE

| Statement | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `DATA` | `DATA [Wertedatas [, ["Textdatas"] [, ...]]]` | Stores comma-separated numeric and/or text data for later READ statements. Text DATA does not generally need quotation marks unless the text contains commas. Without READ statements, DATA text is ignored like REM content. |
| `READ` | `READ Var [,Var2,Var3,Var2$, ...]` | Assigns successive DATA entries to the listed variables. Numeric READ requires numeric DATA; text READ treats a numeric-looking entry as text. Variables cannot be supplied as DATA values. |
| `RESTORE` | `RESTORE [Labelname]` | Resets the DATA pointer to the first DATA entry, or to the first DATA entry following the named label. The book also says a label may be used both as a GOTO target and as a RESTORE target. |

Additional constraints and observations:

- DATA entries are consumed in sequence. If more READ operations execute than there are DATA entries before program end, the interpreter reports an error.
- Numeric DATA may be written in decimal, binary, hexadecimal or octal notation.
- Text DATA entries are separated by commas. The book explains that quotation marks are needed when a text entry contains a comma, otherwise that comma is interpreted as a DATA separator.
- If a text DATA item is unquoted, the characters between its delimiting commas—including spaces—are read as text.
- RESTORE without a label points to the first DATA entry in the program.

### 14.3 Global variable-type declarations

The book describes DEF declarations as global naming rules that assign a variable type based on its leading character(s). A declaration string may contain individual letters and ranges. Examples in the book include `"a-c,x-z"` and `"i1,i2,i3"`.

| Declaration | Syntax | Type assigned by the book |
|---|---|---|
| `DEFBIT` | `DEFBIT Defstring$` | Boolean variables (the book's example assigns names beginning with `a`; Boolean suffix shown as `!`). |
| `DEFBYT` |  | 1-byte integer variables. |
| `DEFWRD` | `DEFWRD Defstring$` | 2-byte integer variables. |
| `DEFINT` | `DEFINT Defstring$` | 4-byte integer variables. |
| `DEFFLT` | `DEFFLT Defstring$` | 8-byte floating-point variables. |
| `DEFSTR` | `DEFSTR Defstring$` | String variables. |

The book gives these further details:

- A definition string identifies the first character or character patterns used to classify variable names. Its contents may be supplied as a string constant, variable or expression.
- A new global declaration replaces the previous declaration rule; the book recommends placing declarations near the beginning of the program.
- A variable with an explicit type postfix remains individually typed and takes precedence over the global naming rule.
- The examples show that a variable such as `l_string$` remains a string despite a preceding `DEFINT "i"`, while an un-suffixed `l_4byte` is treated as a 4-byte integer.
- The editor may accept `DEFSNG` or `DEFDBL`, but the book says it converts these spellings to `DEFFLT` during syntax checking. Preserve this as an editor-normalization claim, not as evidence of separate runtime types.

The OCR's visual rendering of the DEFBIT/DEFBYT type postfixes and some example letters is imperfect. The type widths and declaration names are legible, but exact suffix spelling and character-set parsing should be verified against page images or the interpreter before parser implementation.

### 14.4 LOCAL scope and GOTO boundary

The same chapter describes:

- `LOCAL Var [,Var2, ...]` declares variables local to the current PROCEDURE or FUNCTION. They can be used in that routine and routines called from it; a same-named variable in the main program or a higher-level routine remains distinct and becomes visible again after returning.
- `GOTO Label` branches to a label terminated by a colon. The label may use letters, digits and other characters; unlike variable names, it may begin with a digit.
- The book explicitly prohibits GOTO jumps into or out of FOR…NEXT loops and PROCEDUREs, and says such attempts produce an error.
- The label may also identify the DATA position used by `RESTORE Label`.

These scope and control-flow statements are source claims from the German book. They are not substitutes for runtime tests of nested procedure scope, label parsing or error behavior.


## 15. Procedures, functions and external code: Chapter 6.5–6.6

This section expands the existing §7 summary with details from the German book's printed pp. 172–191. It remains evidence for the book's 3.0/V3.03-era Amiga interpreter, not a claim of 3.5-specific behavior.

### 15.1 PROCEDURE, GOSUB and parameter passing (printed pp. 172–185)

The book's core form is:

```basic
PROCEDURE Name [(Variablenliste)]
  ... statements ...
RETURN
```

- A PROCEDURE is a named subprogram. `GOSUB Name[(Parameterliste)]` enters it; `RETURN` resumes at the statement following the GOSUB call.
- The receiving variables listed in the procedure header are local to the procedure. Same-named global variables are restored/visible again after returning.
- Parameter count, type and order at the call site must match the receiving-variable list.
- Receiving variables may be of mixed types. The book describes the parameter list as limited by the maximum BASIC line length (256 characters).
- Procedures may call other procedures and may call themselves recursively.
- `SUB` is an alternative spelling for `PROCEDURE`; `ENDPROC` and `ENDSUB` are alternative terminators that the interpreter converts to `RETURN`.
- Procedure names may contain letters, digits, underscore and period; the first character may be a digit.
- The computed form `ON Wert GOSUB Proc1[,Proc2,...]` selects a procedure by one-based index. A value of zero or a value greater than the number of targets continues with the next statement. The book says targets in ON…GOSUB forms must not expect parameter lists.

### 15.2 LOCAL and VAR parameter modes

The book documents two ways to make variables local or pass caller variables into a routine:

- `LOCAL Var[,Var2,...]` declares local variables inside a PROCEDURE or FUNCTION. These names are local to the current routine and are also available to routines called from that routine; a same-named variable in the main program or a higher-level routine remains distinct.
- A pointer parameter written with a leading `*` passes the address of a variable. The receiving parameter is itself a pointer; assigning through it changes the caller's variable.
- Alternatively, `VAR` in the routine header provides direct variable passing:

```basic
PROCEDURE Name([ordinary parameters] VAR Var1[,Var2,...])
FUNCTION Name([ordinary parameters] VAR Var1[,Var2,...])
```

The variables following VAR are placed at the end of the receiving-variable list and stand for the actual variables passed by GOSUB or FN. Their types must match. Constants and expressions cannot be passed as VAR variables; the book gives a “Wrong VAR type” error for mismatches. The corresponding global variable is temporarily accessible under the receiving name and changes made through it affect the caller's variable directly.

### 15.3 FUNCTION and DEFFN distinction (printed pp. 173–178)

The book distinguishes one-line `DEFFN` functions from multi-line `FUNCTION` routines.

- `DEFFN Name[(Varliste)]=Ausdruck` defines a compact expression function, invoked with `FN Name[(Parameterliste)]` or the `@Name` shorthand.
- A DEFFN definition is scanned/initialized at program start, so it may appear after the first call in source order. Its definition is limited by the 256-character program-line length.
- `FUNCTION Name[(Var1,Var2,...)]` starts a multi-line function and `ENDFUNC` closes its structural body. `RETURN Back` returns a value to the caller; the book explicitly distinguishes this from the procedure's bare `RETURN`.
- FUNCTION may return numeric or string results. For string results, the function name carries the `$` suffix (for example `FUNCTION Name$`).
- FUNCTION routines may recursively call themselves. The book warns that mutual or recursive DEFFN calls can become uninterruptible by the usual Break key sequence.
- The book documents the same local-variable and VAR passing concepts for FUNCTION as for PROCEDURE.

### 15.4 C:() calls to C-compiled routines (printed pp. 185–187)

The book describes `C:()` as a call interface for a C-compiled machine routine:

```basic
Var=C:Adressvar([Parameterliste])
```

- `Adressvar` contains the address of the machine routine.
- The optional parameter list contains numeric parameters passed using the described C convention.
- The stack is described as containing, in order, the return address, a 16-bit parameter count, and a 32-bit address pointing to a contiguous parameter block.
- Parameters in that block are represented as 32-bit values. String parameters are passed as the address of their first character.
- The returned value may be used in an expression, condition or assignment, or printed directly as `PRINT C:Var()`.
- The example notes that the called routine should return with RTS.

These are the book's stated calling-convention details; do not generalize them to arbitrary compiler ABIs without testing.

### 15.5 CALL to assembly routines (printed pp. 186–187)

The documented form is:

```basic
CALL Adressvar([Parameterliste])
```

- `Adressvar` contains the address of the assembly routine.
- If no parameters are passed, the book requires empty parentheses.
- The stack contains the return address, followed by a 16-bit parameter count and a pointer to the parameter block.
- Parameters are stored sequentially as 32-bit values. The default parameter representation is a 16-bit word; prefixing a parameter with `L:` requests a 32-bit long value.
- String parameters are passed by their starting address.
- The assembly example reads the return address and parameters from the stack and returns using RTS.

### 15.6 EXEC: CLI command and program launch (printed pp. 187–189)

The syntax is:

```basic
EXEC Kommando$,In,Out
```

- `Kommando$` contains the CLI command or program name, optionally with a path.
- `In` and `Out` are input/output channel numbers (file handles). The value −1 selects the current GFA-BASIC window/console channel.
- A file handle for another device or file may be obtained through the DOS Open operation and supplied as In or Out.
- The book distinguishes a synchronous launch, where GFA-BASIC waits until the called program exits, from a command prefixed with `Run`, where the child program runs concurrently and GFA-BASIC can continue.
- The example uses `EXEC "df0:Utilities/Calculator",-1,-1` for the waiting form and `EXEC "Run df0:clock"` for the concurrent form.

### 15.7 MONITOR and RCALL (printed pp. 189–191)

| Command | Syntax | Description in the book |
|---|---|---|
| `MONITOR` | `MONITOR [(Parameter)]` | Deliberately triggers an Illegal Instruction exception, allowing a monitor/debugger handler to take control. The optional value is delivered in DO to the called routine. The book warns that the exception vector must first be redirected to the MONITOR program; otherwise the normal system Guru handler may run. |
| `RCALL` | `RCALL Adresse,Feld%()` | Calls machine code at the supplied address with CPU registers initialized from an integer array of at least 16 elements. |

For RCALL, the book maps array elements 0–7 to data registers d0–d7 and elements 8–14 to address registers a0–a6. After the routine returns, the register values are written back to the array in the same order. The current user stack pointer (SP/a7) is returned in element 15 when OPTION BASE 0 is active, otherwise in element 16. OPTION BASE therefore affects the array indices used by this interface.

### 15.8 Evidence boundary

The stack layouts, register mapping and exception-vector behavior above are transcribed as described in the historical book. They are high-risk low-level interfaces: verify them with the actual target interpreter and Amiga ABI before implementing or documenting them as compatibility guarantees. The chapter does not establish that these interfaces are unchanged in later 3.5 builds.


## 16. Text operations: Chapter 7 (printed pp. 193–200)

This section extracts the book's built-in string operations and their stated behavior. The custom procedures shown in examples (such as ASTRING and CUT) are example code, not built-in commands. Source classification remains 3.0/V3.03-era.

### 16.1 String construction and character conversion

| Function | Syntax recorded in the book | Behavior |
|---|---|---|
| `SPACES` | `Var$=SPACES(Anz)` | Returns a string containing the requested number of spaces. |
| `STRINGS` | `Var$=STRINGS(Anz,Text)` or `Var$=STRINGS(Anz,Ascii)` | Repeats the supplied string or character the requested number of times. The book permits a character's ASCII value instead of a one-character string. |
| `PRED` | `Var$=PRED(Expr$)` | Returns the preceding ASCII character for the first character of Expr$. The book describes it as equivalent to CHR$(ASC(Expr$)-1). |
| `SUCC` | `Var$=SUCC(Expr$)` | Returns the next ASCII character for the first character of Expr$; equivalent to CHR$(ASC(Expr$)+1). |
| `UPPER$` | `Var$=UPPER$(Ziel$)` | Converts lowercase letters to uppercase; German umlauts are also converted. Other characters remain unchanged. |
| `TRIM$` | `Var$=TRIM$(Ziel$)` | Removes spaces at the beginning and end. An empty or all-space input returns an empty string. |

The book states a maximum string length of 32,767 characters.

### 16.2 Fixed-width string assignment

| Statement | Syntax | Behavior |
|---|---|---|
| `LSET` | `LSET Ziel$="Text"` | Copies text to the left side of Ziel$ without changing the target string's existing length. Longer text is truncated; shorter text is padded on the right with spaces. |
| `RSET` | `RSET Ziel$="Text"` | Same fixed-length behavior as LSET, but aligns the supplied text to the right and pads on the left. |
| `MID$` assignment | `MID$(Ziel$,Start[,Anz])="Text"` | Replaces part of an existing string beginning at Start, optionally limiting the number of copied characters. The original target length is retained. |

The book particularly associates LSET/RSET with fixed-length records in random-access files.

### 16.3 Substring and search functions

| Function | Syntax recorded in the book | Behavior |
|---|---|---|
| `LEN` | `Var=LEN(Var$)` | Returns the string length. |
| `LEFT$` | `Var$=LEFT$(Ziel$[,Anz])` | Returns the first Anz characters; without Anz, returns the first character. If Anz exceeds the string length, returns the whole string. |
| `MID$` | `Var$=MID$(Ziel$,Start[,Anz])` | Returns a substring beginning at Start. Without Anz, or when Anz exceeds the remaining length, returns the rest of the string. |
| `RIGHT$` | `Var$=RIGHT$(Ziel$[,Anz])` | Returns the final Anz characters; without Anz, returns the last character. |
| `INSTR` | `Var=INSTR(Ziel$,Such$[,Start])`; also documented as `INSTR([Start,]Ziel$,Such$)` | Returns the first matching position, optionally starting at Start. Search is case-sensitive. Returns 0 when not found; the book says two empty strings yield position 1. |
| `RINSTR` | `Var=RINSTR(Ziel$,Such$[,Start])`; also `RINSTR([Start,]Ziel$,Such$)` | Searches from the end of the target string and returns the first match encountered in that reverse search. |

For LEFT$/MID$/RIGHT$, an empty input string produces an empty result. The book's examples and wording use one-based character positions.

### 16.4 Example-only routines and encoding note

The chapter includes user-written procedures such as ASTRING (collecting all match positions) and CUT (splitting a string into bounded pieces). These are not built-ins. The examples encode integer positions/lengths in a binary string using `MKI$` and recover them with `CVI`; do not add ASTRING or CUT to the interpreter's built-in command inventory on the basis of these examples.

### 16.5 Evidence boundary

The syntax and behavior above are transcribed from the German book's Chapter 7. Exact abbreviation rules, treatment of out-of-range positions, and edge cases beyond those explicitly stated remain unverified. The source is not evidence that every item is unchanged in interpreter 3.5.


## 17. Arithmetic, numeric and bit operations: Chapter 8 (printed pp. 201–224)

This extraction records the arithmetic chapter's operators, statements and functions. It follows the German book's descriptions and examples; it is historical 3.0/V3.03-era evidence, not a claim that every behavior is unchanged in 3.5.

### 17.1 Operators and precedence

The book groups arithmetic operators as follows:

- Exponentiation (`^`).
- Unary minus.
- Multiplication and division (`*`, `/`).
- Integer division (`DIV`) and modulo (`MOD`).
- Addition and subtraction (`+`, `-`).
- Comparisons: equality, inequality (`<>` or `><`), greater/less than, and inclusive comparisons (`>=`, `<=`; alternate orderings are also shown).
- Boolean operators: `NOT`, `AND`, `OR`, `XOR`, `IMP`, `EQV`.

Parentheses have the highest priority; Boolean operators have the lowest. The book's displayed precedence table places comparison operators above Boolean operators and groups the Boolean operators at the bottom. Exact tie-breaking between operators at the same level is not specified by the OCR passage and should be tested separately.

### 17.2 Arithmetic statements and integer arithmetic functions

| Name | Form | Book's description |
|---|---|---|
| `INC` | `INC Var` | Increments the numeric variable by 1. |
| `DEC` | `DEC Var` | Decrements the numeric variable by 1. |
| `ADD` | `ADD Var,Wert` | Adds Wert to Var and stores the result in Var. |
| `SUB` | `SUB Var,Wert` | Subtracts Wert from Var and stores the result in Var. |
| `MUL` | `MUL Var,Wert` | Multiplies Var by Wert and stores the result in Var. |
| `DIV` | `DIV Var,Wert` | Divides Var by Wert and stores the result in Var. |
| `ADD()` | `ADD(Wert1,Wert2)` | Returns the integer result of addition. |
| `SUB()` | `SUB(Wert1,Wert2)` | Returns the integer result of subtraction. |
| `MUL()` | `MUL(Wert1,Wert2)` | Returns the integer result of multiplication. |
| `DIV()` | `DIV(Wert1,Wert2)` | Returns the integer-division result. |
| `MOD()` | `MOD(Wert1,Wert2)` | Returns the integer remainder. |

The book distinguishes the mutating statements ADD/SUB/MUL/DIV from their function forms: function arguments are not changed, and the function forms can be embedded in expressions, assignments and conditions. Real arguments are reduced to their integer parts before these operations; the result is an integer and no rounding is performed. For the statement forms, the book similarly notes that a real Wert is treated as an integer when Var is an integer variable.

### 17.3 Numeric functions

| Function | Syntax | Description in the book |
|---|---|---|
| `ABS` | `ABS(Arg)` | Absolute value. |
| `EVEN` | `EVEN(Arg)` | Returns −1 for an even integer argument and 0 otherwise (the OCR sentence is split; verify the truth convention in the page image). |
| `ODD` | `ODD(Arg)` | Corresponding odd-number test. |
| `FIX` | `FIX(Arg)` | Removes the fractional part (truncates toward zero); identical to TRUNC. |
| `TRUNC` | `TRUNC(Arg)` | Identical to FIX. |
| `INT` | `INT(Arg)` | Returns the next lower integer; for negative non-integers this moves farther from zero (e.g. the book gives INT(−12.33)=−13). |
| `FRAC` | `FRAC(Arg)` | Returns the fractional part; returns zero for an integer argument. |
| `ROUND` | `ROUND(Arg[,Stelle])` | Rounds mathematically to an integer or to the requested decimal place. A negative Stelle rounds to a place left of the decimal point. |
| `PRED` | `PRED(Arg)` | Previous integer. |
| `SUCC` | `SUCC(Arg)` | Next integer. |
| `SGN` | `SGN(Arg)` | Sign: 1 for positive, −1 for negative, 0 for zero. |
| `SQR` | `SQR(Arg)` | Square root. Higher roots may be expressed using fractional exponents. |
| `EXP` | `EXP(Arg)` | Exponential function with base e; the book requires Arg greater than zero. |
| `LOG` | `LOG(Arg)` | Natural logarithm (base e); the book requires Arg greater than zero. |
| `LOG10` | `LOG10(Arg)` | Decimal logarithm (base 10). |

The book's prose for EXP/LOG domain restrictions and its numeric examples should be checked against page images before turning these into runtime assertions; the OCR is damaged in several places.

### 17.4 Trigonometry, angle conversion and extrema

| Function / constant | Syntax | Description in the book |
|---|---|---|
| `SIN` / `COS` / `TAN` | `SIN(Arg)`, etc. | Trigonometric functions with angles in radians. |
| `ASIN` / `ACOS` / `ATN` | `ASIN(Arg)`, etc. | Inverse sine, cosine and tangent; ATN returns an angle in radians. |
| `DEG` | `DEG(Radiant)` | Converts radians to degrees; equivalent to Arg × 180 / PI. |
| `RAD` | `RAD(Grad)` | Converts degrees to radians. |
| `SINQ` / `COSQ` | `SINQ(Grad)`, `COSQ(Grad)` | Sixteenth-degree interpolated sine/cosine using degree input; the book claims roughly ten times the speed of SIN/COS. |
| `PI` | `PI` | Reserved constant for π, printed as approximately 3.1415926536. |
| `MIN` | `MIN(Expr1,Expr2[, ...])` | Returns the smallest value or, for string arguments, the lexicographically smallest string. |
| `MAX` | `MAX(Expr1,Expr2[, ...])` | Returns the largest value or, for string arguments, the lexicographically largest string. |

For MIN/MAX, the book requires all arguments in a call to have the same type. String comparison proceeds character by character using ASCII values; if one string is a prefix of the other, the longer string is considered greater.

### 17.5 Boolean and bitwise operations

The book documents Boolean functions as function-form equivalents of the corresponding operators: `AND(a,b)`, `OR(a,b)`, `XOR(a,b)`, `EQV(a,b)` and `IMP(a,b)`. It describes their results as integer values. Do not assume short-circuit evaluation; this chapter describes logical combination, not evaluation order.

| Function | Syntax | Description |
|---|---|---|
| `BCHG` | `BCHG(Wert,Bit)` | Toggles the selected bit. |
| `BCLR` | `BCLR(Wert,Bit)` | Clears the selected bit. |
| `BSET` | `BSET(Wert,Bit)` | Sets the selected bit. |
| `BTST` | `BTST(Wert,Bit)` | Tests the selected bit; returns −1 when set and 0 otherwise. |
| `BYTE` | `BYTE(Wert)` | Returns the unsigned low 8 bits. |
| `CARD` | `CARD(Wert)` | Returns the unsigned low 16 bits. |
| `WORD` | `WORD(Wert)` | Sign-extends a 16-bit value to 32 bits. |
| `SWAP` | `SWAP(Wert)` | Swaps the low and high 16-bit words of a 32-bit value. |
| `SHL` / `SHR` | `SHL(Wert,Bits)`, `SHR(Wert,Bits)` | Logical left/right shift. |
| `ROL` / `ROR` | `ROL(Wert,Bits)`, `ROR(Wert,Bits)` | Rotate left/right. |

For bit-indexed functions, bit numbering starts at zero from the right. The book says bit indices greater than 31 are not processed correctly and values larger than the maximum 32-bit unsigned value are not handled. It documents typed variants of shifts and rotates: the plain form works on a long (32-bit) value, `&` selects a word (16-bit) operation, and `!` selects a byte (8-bit) operation. The word variants are described as sign-extending bit 15 into the upper 16 bits; byte variants return only the low 8 bits.

### 17.6 Random-number generation

| Function / statement | Syntax | Description |
|---|---|---|
| `RANDOMIZE` | `RANDOMIZE [(Start)]` | Initializes the random-number generator. Reusing the same Start reproduces the same sequence. The argument is optional. |
| `RANDOM` | `RANDOM(n)` | Returns a 32-bit integer random value in the interval from zero (inclusive) to n (exclusive); n may be negative. |
| `RAND` | `RAND(n)` | Returns an unsigned 16-bit integer random value from zero (inclusive) to n (exclusive). Values of n above 65535 are reduced using n MOD 65535. |
| `RND` | `RND[(Arg)]` | Returns a 13-decimal-digit real random value from zero (inclusive) to one (exclusive). The optional argument is a dummy argument with no effect. |

### 17.7 Evidence boundary

This is a transcription of the German book's Chapter 8. Some OCR around parity tests, EXP/LOG descriptions and typed bit-operation variants is degraded; those details are flagged rather than silently normalized. Operator associativity, overflow behavior, division-by-zero behavior and exact coercion rules are not established by this extraction and require page-image or interpreter verification.


## 18. Graphics definitions and drawing operations: Chapter 9 (printed pp. 225–256)

The book introduces Amiga-specific graphics facilities and notes that Atari ST graphics commands could not all be carried over unchanged. The commands below are source-derived from the German 3.0/V3.03-era book. This section records the chapter's documented behavior; it does not claim later 3.5 compatibility.

### 18.1 Graphics state and drawing attributes

| Command | Syntax | Description in the book |
|---|---|---|
| `BOUNDARY` | `BOUNDARY Flag` | Enables (1) or disables (0) outlines around P-graphics objects such as PBOX and PCIRCLE. |
| `COLOR` | `COLOR Front,Back,OutLine` | Selects line/point drawing color, background color and outline color for P-prefixed objects. |
| `DEFFILL` | `DEFFILL [Farbe],[Stil][,Muster]`; also `DEFFILL [Farbe],Muster$` | Defines fill color, fill style and fill pattern for P-graphics and FILL. The book lists solid, dotted, hatched, user-defined and system-pattern choices. |
| `DEFLINE` | `DEFLINE [Stil]` | Selects line style for line-drawing commands. The book lists predefined styles and negative custom 15-bit patterns. It says line-style changes apply only to line width 1; wider lines are solid. |
| `DEFMOUSE` | `DEFMOUSE Form` or `DEFMOUSE Maus$` | Selects a predefined or user-defined mouse pointer shape. A pointer can be set separately per window. |
| `GRAPHMODE` | `GRAPHMODE Modus` | Selects the raster operation used when graphics are drawn over existing content. |
| `SETCOLOR` | `SETCOLOR Reg,Rot,Grün,Blau` or `SETCOLOR Reg,Mischwert` | Sets an active screen's hardware color register, using RGB components (0–15 each) or a combined value (0–4095). |
| `CLIP` | See §18.5 | Defines a graphics clipping rectangle and/or graphics coordinate origin. |

DEFFILL user-defined patterns are supplied as a binary string in MKI$ format. The book describes 16 words for a 16-row pattern, with additional 16-word blocks for additional bitplanes. It also provides a procedure that captures a 16×16 screen area in DEFFILL-compatible format; that procedure is example code, not a built-in.

The book lists GRAPHMODE modes JAM1 (transparent), JAM2 (replace), COMPLEMENT (invert affected pixels) and INVERSEVID. It says mode values may be added to combine certain operations. The exact numeric mode constants should be checked against page images before being encoded as normative constants.

SETCOLOR's combined value is described as red × 256 + green × 16 + blue, with each component in the range 0–15. The book also documents default Workbench-window color-register roles for registers 0–3; those are environment-specific defaults, not universal screen semantics.

### 18.2 Filled and unfilled primitives

| Command | Syntax | Description in the book |
|---|---|---|
| `BOX` / `PBOX` | `[P]BOX X_links,Y_oben,X_rechts,Y_unten` | Draws a rectangle; PBOX fills it using the current DEFFILL pattern. |
| `CIRCLE` / `PCIRCLE` | `[P]CIRCLE X_cent,Y_cent,Radius` | Draws a circle or filled circle. The book says the radius is limited to 511 because of the Blitter's graphics-area limitation. |
| `ELLIPSE` / `PELLIPSE` | `[P]ELLIPSE Xcent,Ycent,Xrad,Yrad` | Draws an ellipse or filled ellipse. |
| `POLYLINE` | `POLYLINE Pkte,Xp(),Yp()[OFFSET Xdiff,Ydiff]` | Draws a polygonal line through coordinate arrays. Up to 128 points are documented; to close the path, repeat the first point as the final point. |
| `POLYFILL` | `POLYFILL Pkte,Xp(),Yp()[OFFSET Xdiff,Ydiff]` | Draws and fills a polygon using current fill attributes. |
| `LINE` | `LINE X1,Y1,X2,Y2` | Draws a straight line using the current line attributes. |
| `PLOT` | `PLOT Xpos,Ypos` | Sets a pixel at the specified coordinate. |
| `POINT` | `POINT(Xpos,Ypos)` | Returns the color-register number of the specified screen pixel. |
| `FILL` | `FILL Xpos,Ypos[,Farbe]` | Flood-fills from the selected point until reaching boundaries. In the book's Amiga V3.0 description, the optional Farbe parameter makes only that color a boundary; other colors are filled. |
| `SCROLL` | `SCROLL Dx,Dy,X1,Y1[,X2,Y2]` | Scrolls a bitmap region. Positive Dx moves content left and positive Dy moves it down; the book warns that pixels pushed out are not saved and the exposed opposite edge is filled with the background color. |

The book notes that POLYLINE/POLYFILL use the current DEFLINE, DEFFILL or DEFMARK attributes. OFFSET shifts the polygon coordinates without rewriting the source coordinate arrays. The printed description of SCROLL's argument list is partly obscured in OCR; verify whether both lower-right coordinates are required before implementing its exact grammar.

### 18.3 Text and turtle/plotter graphics

| Command | Syntax | Description in the book |
|---|---|---|
| `TEXT` | `TEXT Xt,Yt,"Text"` | Draws text in graphics mode. The anchor is the lower-left point of the first character's character box, including when the first character is a space. Output is clipped to the screen. |
| `DRAW` | `DRAW TO Xpos,Ypos`; also coordinate-chain forms | Connects the current graphics point to the specified point, or draws a polyline through a sequence of points. |
| `DRAW$` | `DRAW Def$[,Const[,"Text"[,Var[,...]]]]` | Simulates LOGO-style turtle/plotter graphics using a command string. |
| `DRAW(Index)` | `DRAW(Index)` | Returns turtle state: X, Y, angle, X/Y scale and pen flag for indices 0–5. |
| `SETDRAW` | `SETDRAW Xpos,Ypos,Grad` | Sets the turtle's absolute position and heading. |

The DRAW$ command strings documented in the book include:

- `fd n` / `bk n`: move forward/backward by n pixels.
- `sx n` / `sy n`: set X/Y scaling for forward/backward movement; zero disables the respective scaling.
- `pu` / `pd`: pen up/down.
- `co n`: select line color.
- `dr x,y`: draw to an absolute coordinate.
- `ma x,y`: move to an absolute coordinate with pen up.
- `da x,y`: draw to a position relative to the current turtle position.
- `mr x,y`: move to a relative position.
- `lt n` / `rt n`: turn left/right by n degrees.
- `tt n`: set an absolute heading in degrees.

The book says distances, angles and coordinates may be expressions, constants, variables or values embedded in the DRAW$ command string. The number of comma-separated commands is limited by the maximum input-line length.

### 18.4 Screen capture and bitmap access

| Command | Syntax | Description in the book |
|---|---|---|
| `GET` | `GET X_links,Y_oben,X_rechts,Y_unten,Var$` | Reads a screen rectangle into a binary string as a bitmap. The specified corners must lie within the screen or the interpreter does not execute the command. |

The book explicitly distinguishes this graphics GET from disk-file GET#. The captured bitmap string can subsequently be used by bitmap transfer operations described elsewhere in the reference.

### 18.5 CLIP coordinate and clipping forms

The book documents these forms:

- `CLIP Xpos,Ypos,Breite,Höhe[OFFSET X,Y]`
- `CLIP X1,Y0 TO Xr,Yu[OFFSET X,Y]`
- `CLIP #Windownummer[OFFSET X,Y]`
- `CLIP OFFSET X,Y`
- `CLIP OFF`

CLIP can restrict graphics output to a rectangle, or set the graphics origin relative to the upper-left screen corner. The book says graphics such as LINE, BOX, PBOX and DRAW are clipped at the rectangle boundary, while PUT is excluded from clipping. A window-number form selects the window whose work area supplies the clipping rectangle. CLIP OFF disables clipping; CLIP OFFSET 0,0 restores the origin to zero.

### 18.6 Evidence boundary

This chapter contains Amiga-specific behavior and explicit V3.0 labels. Some OCR-sensitive details—especially SCROLL's full syntax, GRAPHMODE numeric values, and exact DRAW$ token grammar—remain unverified. Example procedures such as the magnifier, popup menu and custom fill-pattern copier are user code, not built-ins. Do not add those procedure names to the language command inventory.


## 19. Bitmap transfer, sprites and Blitter objects: Chapter 9.4.1–9.5 (printed pp. 261–278)

This section continues the graphics chapter beyond the primitive drawing commands. It records the book's Amiga-specific bitmap, sprite and BOB (Blitter Object) facilities. Source provenance remains the German book's 3.0/V3.03-era reference.

### 19.1 PUT and bitmap-string layout

| Command | Syntax | Description |
|---|---|---|
| `PUT` | `PUT X_links,Y_oben,Var$[,Modus[,Maske]]` | Draws a bitmap previously captured with GET at the specified upper-left coordinate. The stored width and height are retained; the destination may be partially or wholly outside the visible screen. |

The book distinguishes graphics PUT from disk-file PUT#. It describes the first three 16-bit words of the bitmap string as:

1. Width: X-right minus X-left.
2. Height: Y-bottom minus Y-top.
3. Number of words per row (the row width rounded up to a 16-bit boundary).

The bitmap data follows the header, row by row. Each row is padded to a whole number of 16-bit words; unused bits in the final word are not meaningful. For multiple bitplanes, the corresponding plane data is stored in successive blocks/planes and the combination of plane bits determines the color-register index.

The optional mode selects a raster operation. The book lists source/destination combinations including OR, exclusive-OR and inversion forms; without a mode it says Replace is used. It warns against negative mode values because they may cause malfunction. The optional mask parameter is present in the syntax but its detailed semantics are not sufficiently legible in the OCR passage to specify here.

### 19.2 SPRITE

The book documents the following forms:

- `SPRITE #Nummer,DefVar$`: define a sprite from a string in the described MKI$/bitmap-like sprite format.
- `SPRITE #Nummer[,TO]`: remove the selected sprite image (the OCR around the optional keyword is imperfect).
- `SPRITE #Nummer[,Xpos,Ypos]`: position a sprite on the screen.
- `SPRITE ON/OFF`: enable or disable sprites; the text also says this affects the mouse pointer.

Additional source details:

- Sprite 0 corresponds to the mouse pointer. The book says sprites are positioned in a 320×256 coordinate grid, independent of the screen's current resolution.
- The sprite image is described as a 16-pixel-wide raster with variable height. Four bytes are used per image row, representing two 16-bit bitplanes.
- A two-bit pixel value of 00 is transparent; the other three combinations select one of three sprite colors. The actual color registers depend on the sprite number.
- The book gives these color-register groups: sprite pairs 0/1 use registers 17–19; 2/3 use 21–23; 4/5 use 25–27; 6/7 use 29–31.

The exact syntax of the second form and the sprite string's byte ordering should be verified against the page image/runtime before implementation.

### 19.3 VSYNC

`VSYNC` waits for the next vertical blank interval. The book explains that synchronizing GET/PUT graphics output with the screen refresh can reduce interference/flicker. It cautions that operations taking longer than one refresh interval may still produce interference.

### 19.4 BOB / OBJECT command family

The book describes BOBs as Blitter-managed graphics objects used for fast movement/animation of larger graphic areas. The object commands documented in the scanned pages are:

| Command | Syntax recorded in the book | Description |
|---|---|---|
| `OBJECT.SHAPE` | `OBJECT.SHAPE ObjNummer,DefString$` or `OBJECT.SHAPE ObjNummer,AltesObjekt` | Defines an object's image from a definition string or copies the shape of an existing object. |
| `OBJECT.ON` | `OBJECT.ON [ObjNummer[, ...]]` | Displays selected objects; with no numbers, enables all defined objects. |
| `OBJECT.OFF` | `OBJECT.OFF [ObjNummer[, ...]]` | Hides selected objects but retains their definitions so they can be enabled again. |
| `OBJECT.CLOSE` | `OBJECT.CLOSE [ObjNummer[, ...]]` | Removes objects completely from the window; they must be defined again before reuse. |
| `OBJECT.CLIP` | `OBJECT.CLIP X_links,Y_oben,X_rechts,Y_unten` | Restricts the area in the window in which BOBs may operate. Coordinates are absolute within the window. |
| `OBJECT.START` | `OBJECT.START [ObjNummer[, ...]]` | Starts movement of selected objects; without numbers, starts all active objects. |
| `OBJECT.STOP` | `OBJECT.STOP [ObjNummer[, ...]]` | Stops movement of selected objects; without numbers, stops all objects. |
| `OBJECT.X` / `OBJECT.Y` | `OBJECT.X ObjNummer,X_Position`; `OBJECT.Y ObjNummer,Y_Position` | Sets an object's position within its window. |
| `OBJECT.VX` / `OBJECT.VY` | `OBJECT.VX ObjNummer,X_Geschwindigkeit`; `OBJECT.VY ObjNummer,Y_Geschwindigkeit` | Sets velocity in pixels per second. |
| `OBJECT.AX` / `OBJECT.AY` | `OBJECT.AX ObjNummer,PixSec`; `OBJECT.AY ObjNummer,PixSec` | Sets acceleration in the corresponding axis, described in pixels per second. |
| `OBJECT.PLANES` | `OBJECT.PLANES ObjNummer[,Bitebenen[,Ebenenwert]]` | Controls which bitplanes of the BOB are drawn and how cleared planes are filled; both values are bit masks. |
| `OBJECT.PRIORITY` | `OBJECT.PRIORITY ObjNummer,Wert` | Sets drawing priority; higher values are drawn later and appear above lower-priority objects. The book gives a range of 0–32767. |
| `OBJECT.HIT` | `OBJECT.HIT ObjNummer[,Wert1[,Wert2]]` | Configures which object collisions are reported using bit masks. |

The book also documents query forms:

- `OBJECT.X(ObjNummer)` and `OBJECT.Y(ObjNummer)` return current position.
- `OBJECT.VX(ObjNummer)` and `OBJECT.VY(ObjNummer)` return current velocity.
- `OBJECT.AX(ObjNummer)` and `OBJECT.AY(ObjNummer)` return current acceleration.

The OCR appears to misread the Y-velocity setter once as OBJECT.VV; the surrounding text and query form identify OBJECT.VY, but confirm the printed page before treating the correction as authoritative.

### 19.5 Collision reporting

- `ON COLLISION GOSUB Prozedur` selects the procedure called when an object collides with another object or the window boundary.
- `COLLISION(ObjNummer)` returns the number of the object involved in the collision.
- The book lists special negative results for window-edge collisions: −1 top, −2 left, −3 bottom and −4 right.
- `COLLISION(0)` returns the object number involved in the most recent collision; `COLLISION(-1)` returns the window number in which it occurred.

OBJECT.HIT uses two bit masks: the first identifies the object's collision participation mask and the second the mask used when another object's value is combined with it. The book says a collision is reported only when the bitwise combination is nonzero.

### 19.6 Evidence boundary

The sprite binary format, PUT raster modes, BOB bitplane masks and collision masks are low-level Amiga graphics details. They are recorded as the book describes them; do not infer undocumented parameter defaults or modern AmigaOS behavior. The exact sprite syntax, PUT mask semantics, OBJECT.AX/AY units and OBJECT.VY spelling require page-image/runtime verification.


## 20. Data conversion and number-format functions: Chapter 10 (printed pp. 279–290)

This section records the conversion functions and number-system material described by the German book. It is 3.0/V3.03-era source evidence.

### 20.1 Character and numeric conversions

| Function | Syntax | Description in the book |
|---|---|---|
| `ASC` | `ASC("Zeichen")` | Returns the ASCII value of the first character. An empty string returns 0. |
| `CHR$` | `CHR$(Wert)` | Returns the character corresponding to the value; the book describes the effective character value as Wert MOD 256. |
| `CFLOAT` | `CFLOAT(Wert)` | Converts an integer value to a real value; inverse of CINT. |
| `CINT` | `CINT(Wert)` | Converts a real value to an integer after mathematical rounding; the book contrasts this with INT. |
| `VAL` | `VAL(Var$)` | Converts the initial numeric portion of a string to a decimal numeric value. Parsing stops at the first non-convertible character; an empty string or a non-numeric first character yields zero. |
| `VAL?` | `VAL?(Var$)` | Returns the number of leading characters that can be converted as numeric notation, stopping at the first invalid character. |

The book says VAL recognizes decimal, hexadecimal, binary and octal prefixes. It shows hexadecimal forms using `$...`, `&...` and `&H...`, binary using `&X...`, and octal using `&O...`. It also says some standard hexadecimal prefixes are converted by the interpreter into GFA's `&H` form. The exact accepted prefix set and normalization behavior should be verified against the page image/runtime.

### 20.2 Binary, hexadecimal, octal and decimal string formatting

| Function | Syntax | Description |
|---|---|---|
| `BIN$` | `BIN$(Expr[,Stellen])` | Converts a numeric expression to a binary string. Optional width is 1–32 positions. |
| `HEX$` | `HEX$(Expr[,Stellen])` | Converts a numeric expression to a hexadecimal string. Optional width is 1–8 positions. |
| `OCT$` | `OCT$(Expr[,Stellen])` | Converts a numeric expression to an octal string. Optional width is 1–11 positions. |
| `STR$` | `STR$(Wert[,Stellen[,Real]])` | Converts a numeric value to a decimal text string. Optional width controls the output field; Real specifies the number of decimal places to round to. |

The book states that BIN$/HEX$/OCT$ accept numeric variables, constants, expressions or functions. It gives `&0` as an octal literal prefix. STR$ may include leading spaces to meet the requested field width; its optional Real argument controls rounding of decimal places.

### 20.3 Binary string encodings for numeric values

The book documents binary encoders and inverse decoders:

| Encoder | Input / output | Decoder | Input / output |
|---|---|---|---|
| `MKI$(Wert)` | 16-bit integer → 2-character string | `CVI(Var$)` | 2-character string → 16-bit integer |
| `MKL$(Wert)` | 32-bit integer → 4-character string | `CVL(Var$)` | 4-character string → 32-bit integer |
| `MKS$(Wert)` | Amiga GFA real → 4-character string | `CVS(Var$)` | 4-character string → Amiga GFA real |
| `MKD$(Wert)` | MBASIC or GFA real → 8-character string | `CVD(Var$)` | 8-character string → real value |

The OCR rendering of the last decoder is damaged (it appears as `CW`/a truncated form in the extracted text). The matching encoder MKD$ and the stated 8-character real format indicate the intended pair is CVD, but confirm the printed page before treating that spelling as verified.

The book emphasizes that these functions are useful for binary storage and data exchange between languages/interpreters whose numeric representations differ. They are also used in the book's own examples to pack integer coordinates into strings.

### 20.4 Number systems

The book introduces decimal (base 10), hexadecimal (base 16), binary (base 2) and octal (base 8). It explains positional value using powers of the base and notes that one hexadecimal digit represents four binary bits: two hex digits per byte, four per 16-bit word and eight per 32-bit longword. Hex digits 10–15 are represented by A–F.

This section is explanatory material rather than additional interpreter commands. Its examples demonstrate manual conversion by repeated division and remainders; they should not be treated as a separate built-in API.

### 20.5 Evidence boundary

The conversion-function signatures are mostly legible, but the CVD spelling and accepted VAL prefixes need page-image/runtime verification. The book does not fully specify malformed-input handling, numeric overflow, exponent parsing edge cases or locale-dependent decimal separators.


## 21. Array operations and sorting: Chapter 11.1 (printed pp. 291–304)

This section captures the first part of the book's Chapter 11 on arrays, memory and pointers. The material below covers array allocation, element insertion/deletion, initialization, base indexing and sorting. Source classification is the German book's 3.0/V3.03-era reference.

### 21.1 Array allocation and lifecycle

| Command | Syntax | Description in the book |
|---|---|---|
| `DIM` | `DIM Arr(Ind1[,Ind2,...])[,Arr2(...),...]` | Allocates storage and sets the dimensions for one or more numeric or string arrays. |
| `DIM?` | `DIM?(Feld())` | Returns the total number of elements in an array; returns 0 if the array has not been dimensioned. |
| `ERASE` | `ERASE Feld()[,Feld2(),...]` | Removes the dimensions and releases the storage occupied by one or more arrays. |
| `ARRAYFILL` | `ARRAYFILL Feld(),Var` | Assigns Var to every element of an already dimensioned numeric array. The value's type must match the array type. |
| `INSERT` | `INSERT Feld(Index)=Wert`; `INSERT Feld$(Index)=Text` | Inserts an element at Index, shifts higher elements up one position and discards the last element. |
| `DELETE` | `DELETE Feld(Index)`; `DELETE Feld$(Index)` | Deletes the element at Index, shifts higher elements down and clears the final element (zero or empty string). |

The book describes the number of elements in multidimensional arrays as the product of their dimension sizes, subject to OPTION BASE. It warns that large dimensions may relocate other variables, especially strings; machine code stored in string variables can therefore become unsafe if the program does not re-check addresses with VARPTR. It recommends INLINE for storing such routines.

### 21.2 OPTION BASE

`OPTION BASE` selects the first index of arrays (0 or 1). The book says it affects all dimensioned arrays and may be changed during a program; existing array indices are adjusted to the new base. Examples show `DIM Feld(5)` as indices 0–5 with base 0, or 1–5 with base 1. The book's array examples generally assume OPTION BASE 0 unless stated otherwise.

### 21.3 QSORT and SSORT

| Command | Syntax recorded in the book | Description |
|---|---|---|
| `QSORT` | `QSORT Feld([Sign])[,Anz[,Feld2%()]]` | Sorts a numeric array by numeric value, or a string array by character/ASCII order. |
| `QSORT` with custom string order | `QSORT Feld$([Sign]) [WITH Vorgabe%()][,Anz[,Feld2%()]]` | Sorts strings using a caller-supplied integer ordering table. |
| `SSORT` | `SSORT Feld([Sign])[,Anz[,Feld2%()]]` | Shell-sort variant with the same general options as QSORT. |
| `SSORT` with custom string order | `SSORT Feld$([Sign]) [WITH Vorgabe%()][,Anz[,Feld2%()]]` | String Shell sort using a custom ordering table. |

The book documents these options:

- A plus sign or no sign sorts ascending; a minus sign sorts descending. The sign is supplied inside the array's parentheses, for example `QSORT Feld%(-)`.
- `Anz` limits the number of elements sorted. The examples show a prefix of the array being sorted while the remaining elements stay in their original positions.
- An optional 4-byte integer array may be supplied as a parallel/index array. Its elements are reordered in step with the sorted array, preserving a mapping to the original positions.
- For string sorting, `WITH Vorgabe%()` supplies a custom character ordering. The book describes a table of at least 256 elements, whose entries define the ordering of ASCII characters. It demonstrates changing the relative order of lowercase letters and German umlauts.
- The book characterizes QSORT as recursive and generally faster, but requiring more memory; SSORT is described as generally slower but useful for arrays that are already partly sorted.

### 21.4 Evidence boundary

The OCR makes the command names and principal syntax legible, but exact array-base behavior when OPTION BASE changes after allocation, sort stability, treatment of equal keys, and all boundary/error cases are not specified here. The custom ordering table's exact integer element width is described as 1-, 2- or 4-byte integer in the prose; verify which array declarations the interpreter accepts before implementation.


## 22. Memory access, allocation and pointer operations: Chapter 11.2–11.4 (printed pp. 305–316)

The following API is described by the German book as Amiga GFA BASIC 3.0-era functionality. These operations expose raw memory and can corrupt the interpreter or operating system if used with incorrect addresses or sizes.

### 22.1 Block transfer and typed memory access

| Command / function | Syntax | Description |
|---|---|---|
| `BMOVE` | `BMOVE Quelle,Ziel,Anz` | Copies Anz bytes from Quelle to Ziel. The book explicitly allows overlapping source and destination ranges. Anz must not be zero; it suggests `MAX(1,Anz)` to prevent a zero-length transfer. |
| `BYTE{}` | `BYTE{Adresse}` / `BYTE{Adresse}=Wert` | Reads or writes one byte. |
| `CARD{}` | `CARD{Adresse}` / `CARD{Adresse}=Wert` | Reads or writes an unsigned 16-bit word. The book requires an even address. |
| `LONG{}` | `LONG{Adresse}` / `LONG{Adresse}=Wert` | Reads or writes a 32-bit longword. The book requires an even address. |
| `INT{}` / `WORD{}` | `INT{Adresse}`, `WORD{Adresse}`; assignment form `INT{Adresse}=Wert` | Reads or writes a signed 16-bit integer. The book says INT{} and WORD{} are two names for the same operation. |
| `FLOAT{}` | `FLOAT{Adresse}` / `FLOAT{Adresse}=Wert` | Reads or writes an 8-byte GFA BASIC 3.0 real value. |
| `SINGLE{}` | `SINGLE{Adresse}` / `SINGLE{Adresse}=Wert` | Reads or writes a 4-byte IEEE real value. |
| `DOUBLE{}` | `DOUBLE{Adresse}` / `DOUBLE{Adresse}=Wert` | Reads or writes an 8-byte IEEE real value. |
| `CHAR{}` | `CHAR{Adresse}` / `CHAR{Adresse}=Expr$` | Reads or writes a NUL-terminated C string. Writing appends a zero byte; reading stops at the first zero byte. |
| `PEEK` / `DPEEK` / `LPEEK` | `PEEK(Adresse)`, `DPEEK(Adresse)`, `LPEEK(Adresse)` | Reads 1, 2 or 4 bytes, respectively. The 2- and 4-byte forms require even addresses. |
| `POKE` / `DPOKE` / `LPOKE` | `POKE Adresse,Wert`, `DPOKE Adresse,Wert`, `LPOKE Adresse,Wert` | Writes 1, 2 or 4 bytes, respectively. DPOKE and LPOKE require even addresses. |
| `INLINE` | `INLINE Adresse%,Bytes` | Reserves up to 32,700 bytes in the program listing and returns the start address when execution reaches the INLINE line. |

The book also describes `DOUBLE{}` and `SINGLE{}` as IEEE-format accessors, while `FLOAT{}` uses the 8-byte GFA 3.0 real format. The unqualified LONG{} form is described as a 4-byte access. For CARD{} and LONG{}, alignment is required.

For signed 16-bit reads, the book describes bit 15 as the sign bit; values with that bit set are interpreted as negative. It notes that assigning the result to a byte variable can only retain unsigned values 0–255.

### 22.2 System memory allocation

| Function | Syntax | Description |
|---|---|---|
| `MALLOC` | `Back=MALLOC(Anz,Art)` | Reserves system memory and returns its start address; zero indicates failure. |
| `MFREE` | `Back=MFREE(Adresse,Anz)` | Releases a block previously reserved with MALLOC. The address and size must match the allocation. |

The book says MALLOC rounds Anz up to the next multiple of eight. Its Art parameter selects memory characteristics:

- `1`: Chip memory.
- `2`: non-movable memory.
- `4`: Fast memory.
- `65536`: initialize/clear the allocated block to zero.
- Values may be combined with bitwise OR; for example, `2 OR 1` requests non-movable Chip memory.

If neither Chip nor Fast memory is explicitly selected, the book says it first tries Fast memory and then Chip memory. It warns that an incorrect address or size passed to MFREE can cause a system crash.

### 22.3 BASIC workspace reservation

`RESERVE[Anz]` sets the size in bytes of the BASIC workspace (program plus variables). The book states that the default is 64 KiB when Anz is omitted, and explicitly warns that RESERVE clears the variable storage.

### 22.4 Pointer and address functions

| Function / form | Syntax | Description |
|---|---|---|
| `VARPTR` | `VARPTR(Var)` or `V:Var` | Returns the address of a numeric variable, or the address of the first character of a string. |
| `ARRPTR` | `ARRPTR(Var$)`, `ARRPTR(Feld())` | Returns the address of a string or array descriptor. |
| Pointer marker | `*Var`, `*Feld()` | Passing a variable/array with * passes its address/descriptor rather than its value. |
| Indirect access | `*Pointer=Value`; `Value=*Pointer` | Uses a pointer to access the referenced variable indirectly. |

The book distinguishes numeric variable addresses from string/array descriptors. It says VARPTR is useful for numeric variables and the first character of strings, while ARRPTR returns the descriptor address for strings and arrays. It also describes pointer arguments in procedures as both input and output parameters, avoiding copying values into locals and copying them back.

### 22.5 INLINE editor storage

INLINE reserves up to 32,700 bytes associated with a source listing line. The book explains that the editor can load, save, print or clear this storage through a Help-key menu when the cursor is on a checked INLINE line. Saving the program with the editor's Save, SAVE or PSAVE retains INLINE contents; saving only an ASCII listing does not preserve the binary payload. Merging an ASCII listing reserves the space again but leaves it empty.

The book says a newly created INLINE area is zero-filled. Deleting its source line releases the area. An INLINE line cannot contain a comment. Changing an existing INLINE declaration may prompt the editor to discard the old buffer and create a new one.

### 22.6 Evidence boundary

This is low-level, version-sensitive memory behavior. Address alignment, allocation flags, binary real layouts, INLINE persistence and pointer conventions are reported as described by the book, not as safe modern API recommendations. Verify all details against interpreter behavior and Amiga memory APIs before implementing them.


## 23. Amiga Exec library overview: Chapter 11.5 (printed pp. 317–328)

The book introduces Exec as the Amiga's multitasking, interrupt and memory-management library. It warns that these calls require detailed knowledge of AmigaOS internals and can destabilize the machine. It states that GFA BASIC can call the routines directly as functions, without the library-opening ceremony used by Amiga BASIC. The entries below are a source-derived index, not a complete ABI specification.

### 23.1 Tasks and scheduling

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `SetTaskPri` | `SetTaskPri(Task,Priorneu)` | Changes a task's priority and returns its previous priority. |
| `FindTask` | `FindTask(Name)` | Searches for a task by name and returns its address. |
| `AddTask` | `AddTask(TaskCB,InitialPC,FinalPC)` | Adds a new task to the system. |
| `RemTask` | `RemTask(Task)` | Terminates and removes a task. |
| `Forbid` | `Forbid()` | Disables task switching. |
| `Permit` | `Permit()` | Re-enables task switching. |

The book explains that task priorities range from −127 to +128 in its description, while its example uses priorities −5 and +5. Treat this range and the example as historical text, not as a reconciled ABI contract.

### 23.2 Interrupts, processor state and resident modules

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `Disable` / `Enable` | `Disable()`, `Enable()` | Disables or re-enables interrupts. |
| `Cause` | `Cause(Adr)` | Triggers a software interrupt at the supplied address. |
| `AddIntServer` | `AddIntServer(Name,Adr)` | Adds an interrupt-server routine. |
| `SetIntVector` | `SetIntVector(Nummer,Adrneu)` | Replaces an interrupt vector and returns the old structure address. |
| `SuperState` | `SuperState()` | Enters supervisor mode and returns the previous stack pointer. |
| `UserState` | `UserState(Stack)` | Returns to user mode using the saved stack value. |
| `SetSr` | `SetSr(Neu,Maske)` | Changes the 68000 status register and returns its old value. |
| `Supervisor` | `Supervisor()` | Switches the 68000 processor into supervisor mode. |
| `Debug` | `Debug()` | Activates the operating system's ROM-Wack/debug facility. |
| `InitCode` | `InitCode(Startwert,Versionsnummer)` | Initializes resident modules with the supplied start value and version. |
| `InitStruct` | `InitStruct(Init,Puffer,Größe)` | Initializes a memory structure from data at Init. |
| `MakeLibrary` | `MakeLibrary(...)` | Creates a function library and returns its address. |
| `FindResident` | `FindResident(Name)` | Searches for a resident structure by name. |
| `InitResident` | `InitResident(Adr,Seglist)` | Initializes a resident structure using a segment list. |

### 23.3 Memory lists and system memory

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `AvailMem` | `AvailMem(Art)` | Returns currently available memory for the requested memory type. |
| `AllocMem` | `AllocMem(Größe,Art)` | Allocates system memory; the book compares it with GFA MALLOC. |
| `FreeMem` | `FreeMem(Adr,Größe)` | Frees system memory; compared with GFA MFREE. |
| `Allocate` | `Allocate(Memheader,Größe)` | Allocates a block from a memory list. |
| `Deallocate` | `Deallocate(Memheader,Adr,Größe)` | Releases a block allocated from a memory list. |
| `AllocEntry` | `AllocEntry(MemList)` | Initializes/allocates memory-list entries. |
| `FreeEntry` | `FreeEntry(MemList)` | Frees blocks associated with a memory list. |

### 23.4 System lists and nodes

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `FindName` | `FindName(Start,Name)` | Searches a system node list by name. |
| `Enqueue` | `Enqueue(List,Node)` | Inserts a node into a system list. |
| `AddHead` / `AddTail` | `AddHead(List,Node)`, `AddTail(List,Node)` | Inserts a node at the head or tail of a doubly linked list. |
| `Insert` | `Insert(List,Node1,Node2)` | Inserts a node into a node list. |
| `Remove` | `Remove(Node)` | Removes a node from a list. |
| `RemHead` / `RemTail` | `RemHead(List)`, `RemTail(List)` | Removes and returns the first or last node. |

### 23.5 Signals, message ports and traps

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `AllocSignal` | `AllocSignal(Signalnr)` | Reserves an available signal bit. |
| `FreeSignal` | `FreeSignal(Signalnr)` | Releases a previously reserved signal bit. |
| `SetSignal` | `SetSignal(Signeu,Signmaske)` | Changes task signal state and returns the old state. |
| `SetExcept` | `SetExcept(Signeu,Maske)` | Selects exception signal bits and returns the old mask. |
| `Wait` | `Wait(Signal)` | Suspends the task until a signal is received. |
| `Signal` | `Signal(Task,Signal)` | Sends a signal to another task. |
| `AddPort` / `RemPort` | `AddPort(MsgPort)`, `RemPort(MsgPort)` | Adds or removes a message port. |
| `WaitPort` | `WaitPort(MsgPort)` | Waits for an event on a message port. |
| `PutMsg` / `GetMsg` | `PutMsg(MsgPort,Message)`, `GetMsg(MsgPort)` | Sends a message to a port or retrieves its next message. |
| `ReplyMsg` | `ReplyMsg(Message)` | Replies to a received message. |
| `AllocTrap` / `FreeTrap` | `AllocTrap(Trapnr)`, `FreeTrap(Trapnr)` | Reserves or releases a trap number. |

### 23.6 Libraries, resources and devices

| Routine | Syntax in the book | Purpose |
|---|---|---|
| `OpenLibrary` | `OpenLibrary(Name,Version)` | Opens a function library and returns its structure address. |
| `CloseLibrary` | `CloseLibrary(Adr)` | Closes a library. |
| `AddLibrary` / `RemLibrary` | `AddLibrary(Adr)`, `RemLibrary(Adr)` | Adds or removes a library from the system library list. |
| `SetFunction` | `SetFunction(Library,Funkoffset,Funkadr)` | Replaces a library function and returns the old function address. |
| `SumLibrary` | `SumLibrary(Library)` | Recalculates a library checksum. |
| `OpenResource` | `OpenResource(Name)` | Opens a resource by name. |
| `AddDevice` / `RemDevice` | `AddDevice(Adr)`, `RemDevice(Adr)` | Adds or removes a device from the system device list. |
| `OpenDevice` | `OpenDevice(Name,Unit,IORequest,Flags)` | Opens a device and initializes its IORequest. |
| `CloseDevice` | `CloseDevice(IORequest)` | Closes a device. |

### 23.7 Evidence boundary

The book deliberately presents these Exec calls only briefly and says detailed AmigaDOS/Exec knowledge is outside its scope. Argument layouts, structure definitions, calling conventions, return-code details and version-specific availability are not established here. Treat this section as a list of callable names and brief descriptions only; do not implement an Exec ABI from these summaries alone.


## 24. Program control, debugging and interpreter configuration: Chapter 12 (printed pp. 331–361)

This continuation completes the Chapter 12 extraction beyond the earlier run/stop, error and timer notes.

### 24.1 Interrupt control

| Command / variable | Syntax recorded in the book | Source-derived behavior |
|---|---|---|
| `EVERY STOP` | `EVERY STOP` | Suspends the recurring `EVERY ... GOSUB` interrupt until `EVERY CONT`. |
| `EVERY CONT` | `EVERY CONT` | Resumes the recurring EVERY interrupt. |
| `AFTER STOP` / `AFTER CONT` | `AFTER STOP`, `AFTER CONT` | Suspends/resumes the single scheduled AFTER interrupt. The book says a new AFTER request replaces the previous pending one; it does not support simultaneous independent AFTER requests. |
| `AFTER` | `AFTER Ticks [GOSUB] Prozedur` | Calls a procedure once after the specified number of 1/200-second ticks. Dispatch occurs after completion of a BASIC statement; blocking operations may defer it until execution resumes. |
| `EVERY` | `EVERY Ticks [GOSUB] Prozedur` | Repeatedly calls a procedure at the requested tick interval, with the same statement-boundary/deferred-dispatch qualification. |

The book's examples explicitly warn that interrupt procedures can interact recursively with application code. It demonstrates suspending EVERY around a time/date routine and restarting it afterwards. It also notes that AFTER can be re-armed from its own procedure, but this does not guarantee the same regular cadence as EVERY.

### 24.2 Debugging and trace output

| Command / variable | Syntax | Description |
|---|---|---|
| `DUMP` | `DUMP [DefinitionsString$] [TO Datei$]` | Lists variable contents and array dimensions; optional selectors filter names, labels, procedures and functions. Optional TO redirects output to a file. |
| `TRON Proc` | `TRON Prozedurname` | Routes trace processing to a user procedure rather than automatically printing each line. |
| `TRACE$` | `Var$=TRACE$` | In the TRON Proc context, returns the next BASIC source line as text. Outside that context it is empty. |
| `TROFF` | `TROFF` | Disables trace mode; the book says TRON and TRON Proc are switched off afterwards. |

The book says DUMP output identifies procedure names with `@`, functions with `FN`, and string functions with an additional `$`. It documents selectors for labels (colon prefix), procedures/functions (@ prefix), and initial-letter filtering. String values are truncated to 60 characters in the dump. Deleted names may remain in an internal reference list until Save/SetNew/Merge cleanup, so the dump can show stale entries.

The trace text says TRON Proc excludes AFTER/EVERY interrupt procedures and the debugging procedure itself from TRACE$ output. TROFF inside a TRON Proc routine has no effect.

### 24.3 Other Chapter 12 entries

| Command / variable | Syntax | Description |
|---|---|---|
| `DEFNUM` | `DEFNUM Stelle` | Sets the decimal digit position used for rounding values printed by output statements; the book gives positions 3–13 and says 13 restores normal output. It does not change internal calculation precision. |
| `DEFLIST` | `DEFLIST Format` | Selects the capitalization format used by interpreter direct-mode listings. The book says it is available only in the interpreter direct mode. |
| `MODE` | `MODE Modus` | Selects European/USA conventions for date and numeric formatting; the described mode is a two-bit vector. |
| `FALSE`, `TRUE` | reserved values | FALSE is 0; TRUE is -1. |
| `LET` | `LET Var=Wert`, `LET Var$=Text` | Assignment form retained for compatibility with other BASIC dialects; the book says it is otherwise unnecessary. |
| `SWAP` | `SWAP Var1,Var2`; array/pointer forms also shown | Exchanges values of like-typed variables, array elements, arrays (including dimensions), or a pointer/array descriptor. |
| `VOID` | `VOID Funktion` | Calls a function while discarding its return value; presented as a faster alternative to assigning an unused result. |
| `$ Text` | `$ Text` | Marks a source line as text storage for the V3.0 compiler; interpreted like REM or apostrophe comment in interpreter mode. |
| `ALERT` | `ALERT Icon%,Boxtext$,Def_Button%,Buttontext$,Backvar%` | Described as a three-button alert/requester function; Icon is explicitly not implemented in the Amiga V3.0 version. |

The book also documents `FALSE`/`TRUE` use in Boolean expressions and bit-flag examples. Its ALERT description limits the message to five lines of up to 40 characters each, with vertical bars separating lines and button labels; the selected button number is returned in Backvar%.

### 24.4 Evidence boundary

The OCR is noisy around DUMP selector punctuation, MODE bit descriptions and the exact formatting of the compiler-only dollar-line construct. Treat those spellings and edge cases as provisional pending page-image review. The compiler text-storage feature is not an interpreter statement in the described mode.

## 25. User interaction and input devices: Chapter 13 (printed pp. 363–374; partial)

The next chapter begins with requester/dialogue functions and mouse/joystick input. This extraction covers the readable material through the first joystick examples; the chapter continues beyond the current source chunk.

### 25.1 Requesters and file selection

| Command / function | Syntax recorded in the book | Description |
|---|---|---|
| `ALERT` | `ALERT Icon%,Boxtext$,Def_Button%,Buttontext$,Backvar%` | Displays an Intuition-style alert with up to three buttons and returns the selected button number. The book says the icon parameter is not implemented in Amiga V3.0. |
| `FILESELECT` | `FILESELECT "Titel","OKText","Pfad",Backvar$` | Displays a file-selection requester and returns the selected path/name in a string variable. |
| `MOUSE` | `MOUSE Xpos,Ypos,Tasten` | Returns mouse coordinates and button state. |
| `MOUSEX`, `MOUSEY`, `MOUSEK` | individual reserved variables | Return X position, Y position and mouse-button state. |
| `STICK` | `STICK(Modus)` | Compatibility setting for mouse/joystick port mode; the book says it has no effect in the Amiga implementation, although mouse/joystick commands select the relevant mode automatically. |
| `STICK()` | `STICK(Maus-Port)` | Returns joystick-direction bits for the selected port; the book documents port 0/1 and says port 0 requires joystick mode. |
| `STRIG()` | `STRIG(Maus-Port)` | Returns the fire-button state for the selected port. |

For FILESELECT, the book describes the returned string cases as follows: selected file returns its name with path; OK with no selection returns an empty string; Cancel returns an empty string. The chapter's examples add user-written extension/backup helper procedures, which are examples rather than built-in commands.

### 25.2 Mouse and joystick values

The MOUSE button status is described as a bit vector: 0 means no button, 1 left, 2 right, 3 both left and right, and 4 middle button. The individual MOUSEX/MOUSEY/MOUSEK variables expose the same values separately. The book notes that V3.0 may return negative coordinates when the pointer is left of or above the window/clip origin.

For STICK(), the book assigns the low four bits to joystick directions: bit 0 up, bit 1 down, bit 2 left, bit 3 right. STRIG(port) reports the fire button as 1 when pressed and 0 otherwise. These are source descriptions; hardware-port behavior should be verified on an Amiga runtime.

### 25.3 Example-only helper routine

The chapter includes a user-defined Rubberbox procedure that tracks a rectangle while the mouse is dragged, saves/restores the background with GET/PUT, and returns signed width/height through pointer parameters. It explicitly changes DEFLINE, COLOR and GRAPHMODE settings and expects callers to restore prior graphics state if needed. This is an example routine, not a built-in GFA command.

### 25.4 Evidence boundary

The chapter is only partially extracted here. The OCR has substantial corruption in example code, punctuation and German prose. Do not treat the example helper routines as built-ins or infer complete requester layout, mouse event semantics, or joystick hardware compatibility from these pages alone.


## 26. Window and screen programming: detailed flags and behavior from Chapters 14.1–14.2 (printed pp. 375–388)

This pass adds details from the chapter text and tables to the earlier command inventory. The book explicitly labels the described Amiga GFA window/screen forms as V3.0.

### 26.1 Window management details

The book describes GFA-managed Window numbers 0–15. Its default arrangement places window 0 across the Workbench screen below the title bar, windows 1–4 in quadrants, and windows 5–15 centered at the same general size as windows 1–4.

- `CLEARW Nummer` clears the specified window. The text says `CLS` affects only the current window and may fail to erase graphics at the right/bottom edges.
- `OPENW Nummer` opens or activates a window. The extended form is `OPENW Nummer,Xpos,Ypos,Breite,Höhe,IDCMP,Flaggen[,Screen[,BitMap]]`. X/Y and dimensions are in screen pixels. IDCMP selects Intuition messages delivered to the program; Flaggen selects window properties/gadgets. Screen and bitmap are optional.
- `FULLW Nummer` expands the window to the maximum screen width/height.
- `CLOSEW Nummer` closes a window; the text gives the range 0–15.
- `TITLEW Nummer,"Window-Titel"[,"Screen-Titel"]` sets the window title and optional screen title shown when the window is active.
- `FRONTW Nummer` and `BACKW Nummer` change the window's depth order.
- `MOVEW Nummer,Xpos,Ypos` moves it to an absolute screen position.
- `SIZEW Nummer,Breite,Höhe` changes its dimensions.
- `LIMITW Nummer,Xmin,Ymin,Xmax,Ymax` constrains the minimum/maximum size, including user sizing.
- `SETWPEN LinienStift,FüllStift` sets standard window frame/fill pens and must be called before opening the window.

The book warns that invalid window positions or dimensions can crash the Amiga. It also says ordinary window contents are the application programmer's responsibility: Intuition does not automatically preserve/redraw arbitrary graphics unless the appropriate window refresh strategy is selected. Text can be restored by rewriting it; graphics restoration is more involved.

### 26.2 IDCMP message names and window flags

The IDCMP table names these message classes: `SIZEVERIFY`, `NEWSIZE`, `REFRESHWINDOW`, `MOUSEBUTTONS`, `MOUSEMOVE`, `GADGETDOWN`, `GADGETUP`, `REQSET`, `MENUPICK`, `CLOSEWINDOW`, `RAWKEY`, `REQCLEAR`, `NEWPREFS`, `DISKINSERTED`, `DISKREMOVED`, `ACTIVEWINDOW`, `INACTIVEWINDOW`, `DELTAMOVE`, `VANILLAKEY`, and `INTUITICKS`. The printed decimal/hex/binary columns are damaged by OCR; do not use the OCR values as constants without checking the page image or primary Amiga headers.

The Window flags table names: `SMART_REFRESH`, `WINDOW_SIZING`, `WINDOW_DRAG`, `WINDOW_DEPTH`, `WINDOW_CLOSE`, `SIZE_RIGHT`, `SIMPLE_REFRESH`, `SUPER_BITMAP`, `BACKDROP`, `REPORTMOUSE`, `GIMMEZEROZERO`, `BORDERLESS`, `ACTIVATE`, `RMB_TRAP`, and `NOCAREREFRESH`. The example combines flags in an OPENW call, but the OCR of the table's binary/hex values is not reliable enough to establish all numeric constants.

### 26.3 Screen commands and display model

The chapter gives these Screen forms (V3.0):

| Command/function | Syntax | Description |
|---|---|---|
| `OPENS` | `OPENS Nummer[,Xpos,Ypos,Breite,Höhe,BitPlanes,Modus]` | Opens a screen; indices 0–15, with 0 representing Workbench. The text says X is retained for compatibility and should be 0 on the described hardware. |
| `WINDOW()` | `WINDOW(Nummer)`, `WINDOW(Adresse)` | Converts between GFA window number and the corresponding Window structure address. |
| `FRONTS` / `BACKS` | `FRONTS Nummer`, `BACKS Nummer` | Brings a screen to front or sends it behind other screens. |
| `CLOSES` | `CLOSES Nummer` | Closes the screen and all windows on it. |
| `TITLES` | `TITLES Nummer,TitelText` | Sets the screen's default title, shown when no window on that screen is active. |
| `MOVES` | `MOVES Nummer,Xpos,Ypos` | Moves a screen to an absolute position. |
| `SCREEN()` | `SCREEN(Nummer)`, `SCREEN(Adresse)` | Converts between GFA screen number and Screen structure address. |
| `SETSPEN` | `SETSPEN LinienStift,FlächenStift` | Sets screen title/detail and block/fill pens before opening the screen. |

The book describes screen mode bits named `GENLOCK VIDEO`, `INTERLACE`, `EXTRA HALFBRITE`, `HOLD AND MODIFY`, `SPRITES`, and `HIRES`. It gives mode values and resolution/plane tables, but OCR corrupts portions of the table; preserve the names only until values are visually verified. The text explains bitplanes as per-pixel bit storage and notes that increasing planes increases the number of representable colors and consumes graphics memory.

### 26.4 Safety and evidence boundary

The book warns that out-of-range window/screen geometry can cause an unhandled system crash. The OCR-corrupted IDCMP and Window flag numeric tables must not be used to implement constants. Screen mode names and qualitative bitplane explanation are legible, but resolution table entries need page-image validation.

## 27. BASIC menu programming: Chapter 15 (printed pp. 389–393)

The book describes a BASIC-level menu system built from a one-dimensional string array.

| Command | Syntax recorded in the book | Description |
|---|---|---|
| `MENU` | `MENU Menütext$()` | Installs/builds the pull-down menu from a one-dimensional string array. |
| `MENU` | `MENU Menüpunkt,Attribut` | Sets a menu item's attributes, including checkmark and enabled/disabled appearance. |
| `MENU` | `MENU Menüpunkt,Text` | Replaces the text of an existing menu item. The new text must not exceed the original allocated text length. |
| `MENU KEY` | `MENU KEY Menüpunkt,ASCIIcode` | Assigns a keyboard shortcut to a menu item; the book describes a right-Amiga-key combination. |
| `MENU KILL` | `MENU KILL` | Deactivates/removes the pull-down menu from the window without discarding the menu text array; MENU can install it again. |

The attribute values shown in the book include 64 and 256 for checkmark set/clear, and 16 and 64 for disabled/enabled appearance. The OCR/table layout does not unambiguously preserve which value maps to each operation; verify the page image before assigning constants.

The menu array format uses strings for menu titles and entries. A null string terminates each menu row; the book's example builds the array from DATA/READ and appends an empty string. A line of hyphens is used as a separator. Two leading spaces reserve room for a checkmark; a leading hyphen marks an item as disabled when the menu is installed. The first menu is described as a desktop/accessories menu following an ST-like convention, with later menus appended.

## 28. Event monitoring: Chapter 16 (printed pp. 395 onward; partial)

The book describes `MENU(Index)` as access to an integer event buffer populated with Intuition event data. The readable portion identifies these fields:

| Expression | Meaning in the book |
|---|---|
| `MENU(0)` | Menu number and selected item number within the menu string array. |
| `MENU(1)` | Intuition message class. |
| `MENU(2)` | Message-class-dependent code/data, e.g. selected menu item number. |
| `MENU(3)` | Qualifier keys such as Shift and Ctrl. |
| `MENU(4)` | Address of the Intuition object responsible for the message (e.g. Gadget or Window). |
| `MENU(5)`, `MENU(6)` | Mouse X/Y relative to the window's upper-left corner; negative values are possible when the window is offset. |
| `MENU(7)`, `MENU(8)` | Event timestamp seconds and microseconds. |
| `MENU(9)` | Window structure address from which the message originated. |

The text says the event data range can be queried from MENU(0) through MENU(10); the description of MENU(10) is present in the book's earlier overview as the Window address, while this later page labels the window pointer at MENU(9). This is an apparent source inconsistency; retain both observations and do not resolve the index without page-image/runtime verification.

The book's menu event example uses `ON MENU GOSUB Auswertung`, then reads MENU(0) in the handler and dispatches based on the selected menu number. It also shows `MENU KILL` during handling and a `SLEEP` loop while waiting for an event. Chapter 16 continues beyond the currently reviewed pages; event classes, additional fields and handler commands remain to be extracted.

### 28.1 Evidence boundary

The event-buffer field numbering is not fully consistent across the book's overview and detailed pages. The current extraction records the discrepancy rather than silently correcting it. Exact event class values and further event handlers require the remaining Chapter 16 pages and page-image checks.


## 29. Intuition event classes and dispatch: Chapter 16 continuation (printed pp. 397–404)

The continuation provides message-class examples and the BASIC handlers used to wait for and dispatch events.

### 29.1 Menu event decoding

For a menu-pick event, the book gives this bit-field decoding of `MENU(2)`:

- Menu number = `MENU(2) AND 31`
- Menu item = `SHR(MENU(2),5) AND 63`
- Submenu item = `SHR(MENU(2),11) AND 31`

It separately identifies `MENU(1)=256` as a menu-selection message. This is a different representation from the simpler menu index in MENU(0).

### 29.2 Message classes and event payloads

The following class values and descriptions are legible in the source. They are the book's Amiga V3.0-era examples; verify numeric values against the matching AmigaOS headers before implementation.

| MENU(1) value | Event described | Payload fields called out by the book |
|---:|---|---|
| 2 | Window size changed | `MENU(10)` contains the Window structure address. |
| 4 | Window refresh required | `MENU(10)` contains the Window address; application must redraw/restore window contents. |
| 8 | Mouse button event | `MENU(2)` contains button state; the book gives 104 for left press and 232 for right press, adding 1 when the button is released. `MENU(10)` identifies the Window. |
| 16 | Mouse moved | `MENU(5)`/`MENU(6)` contain absolute mouse coordinates relative to the Window. |
| 32 | Gadget pressed | `MENU(4)` is the Gadget structure address; `MENU(10)` is the Window address. |
| 64 | Gadget released | Same Gadget/Window address fields. |
| 128 | Requester opened in a Window | `MENU(4)` is the Requester structure address; `MENU(10)` is the Window address. |
| 512 | Requester cannot be displayed in the Window | `MENU(10)` is the Window address. |
| 4096 | Last Requester in the Window closed | `MENU(10)` is the Window address. |
| 1024 | Key pressed (raw-key event) | `MENU(2)` contains the keyboard code; `MENU(3)` contains qualifier state. |
| 2097152 | Key pressed (vanilla-key event) | `MENU(2)` contains the translated ASCII code. |
| 16384 | System Preferences changed | No additional payload detail in the extracted passage. |
| 32768 (OCR appears as 3276…) | Disk inserted | The OCR truncates/corrupts the numeric value; verify from page image. |
| 65536 (OCR appears in a damaged line) | Disk removed | Numeric association requires page-image verification. |
| 131072 | Workbench changed | The book says Workbench sends this when changes are made to it. |
| 524288 | Window became inactive because another Window was activated or the user clicked the screen | `MENU(10)` contains the Window address. |
| 4194303 | IntuiTick/time notification | The book says a message is sent every tenth of a second. |

The passage also describes a mouse-move variant where MENU(5)/MENU(6) report coordinate deltas relative to the previous position. The OCR around the corresponding message constant is damaged (rendered as “048576”); do not assign a numeric value from that fragment.

The book's event descriptions repeatedly identify `MENU(10)` as the Window structure pointer, convertible to a GFA Window number using `WINDOW(Adresse)`. This repeated detailed usage supports MENU(10); the earlier isolated apparent MENU(9) reading is likely OCR ambiguity, but remains recorded as an unresolved transcription discrepancy until page-image review.

### 29.3 Event waiting and callback registration

| Command | Syntax recorded in the book | Description |
|---|---|---|
| `SLEEP` | `SLEEP` | Waits for an Intuition message; presented as a simple event wait. The book says to include it inside internal wait loops so event handling can run while polling. |
| `ON MENU` | `ON MENU` or `ON MENU [Zeit]` | Waits for an event and dispatches to registered event procedures. Optional Zeit is in milliseconds and bounds the wait; the text links timed use to ON MENU BUTTON GOSUB. |
| `ON MENU GOSUB` | `ON MENU GOSUB Prozedurname` | Registers the procedure for pull-down menu selection. The handler reads MENU(0) to determine the selected item. If the procedure does not exist, menu monitoring is disabled. The book warns calling it before defining a Window with a menu can crash the program. |
| `ON MENU BUTTON GOSUB` | `ON MENU BUTTON GOSUB Prozedurname` | Registers a mouse-button event procedure; the handler inspects MENU() to determine which button was pressed/released and when. |
| `ON MENU KEY GOSUB` | `ON MENU KEY GOSUB Prozedurname` | Registers a keyboard event procedure. The passage says MENU(14) provides the key code; if the named procedure does not exist, keyboard monitoring is disabled. |

The source says ON MENU must be called repeatedly/in a loop if events are to be reliably received; without it, the ON MENU ... GOSUB procedures are not dispatched. It also explains that SLEEP is a simpler wait when no custom timeout is needed. A time-limited ON MENU wait is described as allowing the input device to register a mouse-button release, but only when ON MENU BUTTON GOSUB is also installed.

### 29.4 Chapter boundary and evidence

Chapter 16 ends at printed p. 404; Chapter 17 begins the separate compiler discussion. The compiler chapter must not be merged into the interpreter language/API inventory. Event constants and the mouse-button numeric states are transcribed from OCR and require page-image/AmigaOS-header validation before being treated as implementation constants.


## 30. GFA-BASIC 3.5 additions: vector and matrix operations, Chapter 18 (printed pp. 451–476)

**Version boundary:** Unlike the preceding Chapters 12–17, Chapter 18 explicitly introduces commands added in version 3.5. The book states that the existing commands were not fundamentally changed; it presents the following as the new 3.5 commands. This is direct evidence for the book's stated 3.5 feature set.

All command names in this section use the `MAT` prefix. Vectors are described as one-dimensional matrices; matrices are numeric arrays, usually two-dimensional.

### 30.1 Matrix base and initialization

| Command | Syntax recorded in the book | Description |
|---|---|---|
| `MAT BASE` | `MAT BASE 0` / `MAT BASE 1` | Sets the starting index used by matrix/vector operations. Default is 1; the book recommends matching OPTION BASE where arrays are dimensioned with base 0. |
| `MAT CLR` | `MAT CLR m()` | Clears all elements of the specified matrix/vector to zero, independent of its dimensionality. The book notes this is equivalent in effect to ARRAYFILL m(),0. |
| `MAT SET` | `MAT SET m()=x` | Sets every element to a constant value. |
| `MAT ONE` | `MAT ONE m()` | Creates an identity matrix; the source requires a square, two-dimensional matrix. |

### 30.2 Matrix input and output

| Command | Syntax | Description |
|---|---|---|
| `MAT READ` | `MAT READ m()` | Reads as many DATA values as required by the matrix dimensions and MAT BASE setting. The book says the tested version did not support reading multiple fields/vectors/matrices consecutively in one statement as READ does. |
| `MAT INPUT` | `MAT INPUT #channel,m()` | Reads matrix data from a file/channel. |
| `MAT PRINT` | `MAT PRINT [#channel,]m()[,g,n]` | Prints a matrix/vector, optionally to a channel. Optional g is total number display width and n is number of decimal places; the book relates this format to Pascal-style formatting and STR$. |

The source says MAT READ starts at the current DATA pointer and consumes the number of elements required by the target field. It does not support reading only a subrange of a field; insufficient DATA values cause an error. MAT PRINT calculates the row/column output itself, avoiding a user-written loop.

### 30.3 Copying, transposition and reflection

| Command | Syntax | Description |
|---|---|---|
| `MAT CPY` | `MAT CPY a([i,j])=b([k,l])[,h,w]` | Copies a source region to a destination region. i/j and k/l identify top-left coordinates; h/w give region height/width. Without a region size, the book also demonstrates whole-matrix copying. |
| `MAT XCPY` | `MAT XCPY a([i,j])=b([k,l])[,h,w]` | Copies with rows and columns exchanged; can convert row vectors to column vectors and vice versa. |
| `MAT TRANS` | `MAT TRANS a() [=b()]` | Transposes a matrix across its main diagonal. The book says the source/destination dimensions must be compatible; a square matrix can be transposed in place. |

The copy examples demonstrate whole matrix copies, one-row copies, one-column copies, and rectangular regions. The text says only elements whose indices exist in both source and destination are copied when maxima differ. Exact omitted-parameter defaults should be verified from runtime rather than inferred from the examples.

### 30.4 Element-wise arithmetic and matrix multiplication

| Command | Syntax | Description |
|---|---|---|
| `MAT ADD` | `MAT ADD a()=b()+c()`; `MAT ADD a(),b()`; `MAT ADD a(),x` | Element-wise addition into a destination matrix, in-place addition, or addition of a scalar to every element. |
| `MAT SUB` | `MAT SUB a()=b()-c()`; `MAT SUB a(),b()`; `MAT SUB a(),x` | Corresponding element-wise subtraction forms. |
| `MAT MUL` | `MAT MUL a()=b()*c()`; `MAT MUL x=a()*b()`; `MAT MUL a(),x` | Matrix multiplication and scalar multiplication forms. The book also shows chained matrix products. |
| `MAT NORM` | `MAT NORM v(),0` / `MAT NORM m(),1` | Normalizes a vector/matrix. For matrices the second argument selects row-wise (0) or column-wise (1) normalization; for vectors it is irrelevant. |

For MAT ADD and MAT SUB, operands must have equal dimensions and are combined element by element. In-place variants overwrite the first matrix. MAT MUL uses conventional matrix multiplication: the first matrix's column count must equal the second matrix's row count; the result dimensions are the first matrix's row count by the second matrix's column count. The book also demonstrates multiplying every element by a scalar.

MAT NORM scales a vector to unit length while retaining direction/sign. The book's explanation calculates vector length from the square root of the sum of squared components. For matrices it describes row/column normalization. Some OCR in the example listings is damaged; use the prose and command forms as evidence, not the corrupted example tokens.

### 30.5 Matrix analysis

| Command | Syntax | Description |
|---|---|---|
| `MAT DET` | `MAT DET y=m([i,j])[,d]` | Calculates the determinant of a square matrix; optional i/j and d select a submatrix extent. |
| `MAT QDET` | `MAT QDET y=m(n,n)[,d]` | Faster determinant calculation using an optimized routine; the book warns that small precision differences may occur. |
| `MAT RANG` | `MAT RANG y=m()` | Calculates the rank of a square matrix. |
| `MAT INV` | `MAT INV i()=m()` | Calculates the inverse matrix and stores it in i(). The source requires a square matrix. |

The book describes determinant as an indicator related to whether a matrix is solvable/invertible. The exact optional submatrix parameter grammar is partly OCR-damaged; verify before implementing.

### 30.6 Evidence and implementation cautions

Chapter 18 is a clearly labeled 3.5 addition in this German book. It provides syntax and conceptual behavior, but not a complete formal specification for dimension mismatch, aliasing, numeric precision, singular matrices, error codes, or every omitted argument. The OCR contains damaged brackets, parentheses and example listings; do not derive exact parser grammar from those corrupted examples. Treat all such edge cases as runtime-validation work.

## 31. GFA-BASIC 3.5 additions: mathematical helpers, DATA pointer and editor commands, Chapter 19 (printed pp. 477–482)

Chapter 19 begins with additional 3.5 functions and editor enhancements. The book explicitly places these among the new 3.5 features.

### 31.1 Mathematical functions

| Function | Syntax | Description |
|---|---|---|
| `FACT` | `FACT(n)` | Calculates factorial. The book says it is defined for integer values and demonstrates a result for FACT(421). |
| `VARIAT` | `VARIAT(m,k)` | Calculates the number of ordered variations of m elements taken k at a time, without repetition. |
| `COMBIN` | `COMBIN(m,k)` | Calculates the number of combinations of m elements taken k at a time. |

The book gives the standard combinatorial relationships:
- Variations: (m!/(m-k)!), with (m ≥ k).
- Combinations: (m! / ((m-k)! · k!)), with (m ≥ k).

The printed FACT discussion claims support through 421 and gives a large scientific-notation example. It does not specify overflow behavior or accepted numeric types beyond saying factorial is defined for integer values.

### 31.2 DATA pointer variable

| Variable | Syntax | Description |
|---|---|---|
| `_DATA` | Read `_DATA`; assign `_DATA=value` | Reads the current DATA-pointer position and allows that position to be saved/restored independently of RESTORE. |

The book says _DATA contains an integer position representing the current DATA read pointer. A previously read position can be stored and later assigned back to _DATA to resume reading from that point. It warns not to assign arbitrary positions; use positions that have actually been read. In the example, _DATA becomes zero at the end of the DATA list. Exact pointer origin/index semantics should be confirmed against the interpreter.

### 31.3 Editor keyboard additions

Chapter 19 also describes editor key combinations rather than BASIC language statements:

- Tab moves the cursor to the next tab stop without shifting source text.
- Shift+Tab moves to the previous tab stop.
- Tab+Shift inserts spaces up to the next tab stop.
- Shift+Tab (left/right variants are difficult to distinguish in OCR) is described for aligning comment text; one command deletes spaces on the current line.
- The book explicitly says the last two editor functions were not implemented in the test version it had available and recommends checking the installed version.

These are editor features, not interpreter commands. OCR damages the key labels for the final two combinations; preserve the prose-level description only until page-image review.

### 31.4 Evidence boundary

The book's explicit 3.5 introduction makes the functions and _DATA variable strong version-specific evidence. Exact factorial limits, numeric overflow, _DATA pointer indexing, and editor shortcut assignments are not fully specified. Do not infer runtime edge cases from the examples.


## 32. Version 3.5 error catalogue and command-index cross-check: Appendices B–D (printed pp. 487–497)

The appendices provide an explicit “Neue Fehlermeldungen ab Version 3.5” list and a command index. These are useful corroborating evidence for the 3.5 additions extracted above.

### 32.1 Explicitly listed 3.5 error codes

| Error code | Source wording / meaning |
|---:|---|
| 11 | Kein GFA-BASIC-3.5-Programm |
| 76 | Matrix operations only for one- or two-dimensional arrays |
| 77 | Matrices do not have the same order/dimensions |
| 78 | Vector product not defined |
| 79 | Matrix product not defined |
| 80 | Scalar product not defined |
| 81 | Transposition only for two-dimensional matrices |
| 82 | Matrix is not square |
| 83 | Transposition not defined |
| 86 | FACT/COMBIN/PERMUT not defined |
| 120 | Invalid or too-long argument line |
| 121 | File is not an executable program/object module |
| 122 | Invalid resident library addressed during load |

The book's appendix heading explicitly identifies the list as new errors from version 3.5. Error 11 is printed in the 3.5 error list as “Kein GFA-BASIC-3.5-Programm”; the command index and error appendix also identify the version-specific status. The factorial/combinatorics error line mentions PERMUT, although the Chapter 19 prose section extracted above names VARIAT and COMBIN; preserve this source difference rather than assuming PERMUT is an alias or typo.

The appendix gives explanatory DOS error descriptions for 120–122: malformed/overlong command arguments, a non-loadable file, and an invalid resident library. These are AmigaDOS-facing launch/load errors rather than ordinary BASIC syntax errors.

### 32.2 Command-index cross-checks

The command index independently lists the following Chapter 18/19 entries and page references:

- Matrix family: MAT BASE, MAT CLR, MAT SET, MAT ONE, MAT READ, MAT PRINT, MAT INPUT, MAT CPY, MAT XCPY, MAT TRANS, MAT ADD, MAT SUB, MAT MUL, MAT NORM, MAT DET, MAT QDET, MAT RANG and MAT INV (printed pp. 454–475).
- New mathematical functions: FACT (p. 477), VARIAT (p. 478), COMBIN (p. 479).
- DATA pointer operations: _DATA (p. 479).
- Compiler flags and linker options are indexed separately in Chapter 17 and remain compiler-tooling features, not interpreter commands.

This index confirms the page placement and spelling of the matrix/function families, but the OCR is unreliable for some abbreviations and page digits (for example MAT BASE appears as “545” in the index OCR while its chapter text places it on p. 454). Use the chapter body as the primary page/syntax evidence and the index only as a cross-check.

### 32.3 Scope and limitations

The command index is not a normative language specification. It confirms that the book covers these entries but does not establish parameter types, runtime semantics, or compatibility beyond the book's stated version. Error-code wording and numeric assignments should be checked against the English 3.5 addendum/interpreter reference before using them as implementation constants.

## 33. Compiler tooling (not interpreter language API): Chapter 17 (printed pp. 405–450)

This chapter describes the separate GFA-BASIC 3.0 compiler, linker, CLI tools and the MENUX compiler shell. It is recorded for completeness, but compiler switches are not interpreter statements and do not establish interpreter runtime semantics.

### 33.1 Compiler and linker workflow

The book distinguishes interpretation (execute BASIC source statement by statement) from compilation (translate source into object code) and linking (combine object module(s), library routines and options into an executable program). The described workflow is: select a .GFA source file, compile it to an object module, link the module, then test/execute the resulting program. The source file remains necessary for later changes.

The MENUX shell provides file selection, Compiler, Linker, Interpreter, Test, Execute and Quit functions. The book also describes CLI invocation using GFA_BCOM and GL.

### 33.2 Compiler options and source flags

The chapter describes options for optimization, interrupt checks, integer arithmetic, procedures/functions, error messages and stack reservation. OCR preserves these flag spellings; some descriptions are clearer than the exact punctuation/variant mapping.

| Flag | Source-described purpose |
|---|---|
| I+ / I- | Enable/disable checking for the compiler's keyboard break combination. |
| U+ / U- | Enable/disable interrupt routines (AFTER/EVERY). |
| S& | Select handling with 2-byte parameters/values, as described in the compiler's Select optimization section. |
| S< / S> | Select optimization for program size versus execution speed; the size-oriented algorithm is slower. |
| P> / P< | Procedures handled with GFA-managed procedure frames (recursion/RESUME support) versus compiled as plain 68000 subroutines. |
| F% | Function return parameters treated as Integer (as described in the flag table). |
| F> / F< | Controls generation of ENDFUNC; omitting it is possible when RETURN is used and may be faster. |
| E$ | Include textual error messages in the compiled program. |
| E# | Store/report error messages as numbers rather than full text. |
| %0 / %3 | Select integer division behavior; the chapter distinguishes integer division only when assigned to an integer target versus integer division throughout. |
| %6 | Force mixed integer/floating-point addition to floating-point arithmetic; the book warns this can be significantly slower. |
| *% / *& | Select integer long-word multiplication implementation (LMUL routine versus 68000 MULS instruction). |
| N+ / N- | Enable/disable additional overflow/loop-boundary checks. The faster default can permit an integer FOR/NEXT loop to wrap and continue indefinitely. |
| C+ / C- | Save/restore registers A3–A6 around CALL to assembler routines. |
| Mxxxx | Reserve xxxx bytes of stack memory for the compiled program. |
| X name | Mark an external assembler routine for linking with the named GFA library routine. |
| -s | Link symbol/debug information into the executable (linker option). |

The book says options can be set globally through the compiler shell/CLI and locally in source using dollar-prefixed compiler directives, e.g. $ U+, $%6, $ m2000, or $ s&,s<. These are compiler directives, not runtime BASIC statements.

### 33.3 CLI and linker switches

The compiler is invoked with a source filename followed by option flags. The linker is invoked separately; the book's examples use GL Test.

| Linker switch | Source-described purpose |
|---|---|
| -s | Include the symbol table for debugging. |
| +Lib | Use the named library instead of the default GFA library. |
| -Lname | Select the link library. |
| -Oname | Replace the default object module (e.g. TEST.O) with name.O. |
| -Pname | Name the generated executable. |
| -W | Enable the wait flag for disk changes. |

The book also describes adding additional object modules to the standard object and excluding/replacing the default TEST.O module. Exact punctuation/order of some linker switches is OCR-sensitive; verify against the compiler's own command help or original distribution before implementation.

### 33.4 Optimization guidance and compiler limitations

The examples show that compilation does not speed up operating-system drawing or I/O in the same way it speeds up arithmetic loops. The book recommends choosing integer types appropriate to the value range, using 2-byte or 1-byte values where sufficient, and using the integer multiplication option for suitable workloads. It gives a sieve benchmark with a large speedup in the compiled version, while a graphics-heavy example improves little.

The chapter warns about compiler-specific correctness/performance trade-offs:
- Loop termination checks may be delayed until NEXT; integer wraparound can therefore produce an endless loop unless the overflow-check option is enabled.
- Mixed integer/real addition can truncate fractional parts when converted to integer; the floating-point option avoids this at a performance cost.
- Plain 68000 procedure compilation is faster but removes GFA-managed recursion/RESUME handling.
- Skipping runtime error handling or reducing checks can produce unpredictable behavior.
- Debug symbols consume additional memory.

### 33.5 MENUX shell source and caveats

The book prints and annotates a sample compiler shell implemented in GFA-BASIC. It describes procedures for menu/key handling, file selection, compiler option-string generation, environment/linker arguments, compile/link execution, timing and output-window display. It identifies defects in the supplied shell and proposes source edits, including correcting a menu case index, redirecting version text to the output channel, configuring a working path and adjusting EXEC behavior to allow the shell to respond while a compiled program runs.

These are example application fixes, not changes to the BASIC language or compiler ABI. The printed listing contains extensive OCR damage and must not be copied as a verified working implementation.

### 33.6 Evidence boundary

Chapter 17 documents the GFA-BASIC 3.0 compiler tooling. The book's later Chapters 18–19 explicitly introduce interpreter-facing 3.5 features, so the compiler option list remains a separate compatibility layer. Exact flag aliases, command-line quoting, ABI/register preservation, stack sizing and generated-code behavior require compiler-binary/runtime verification.


## 34. Reconciliation against the English GFA-BASIC 3.5 Interpreter addendum

**Source boundary:** The English handbook identifies Chapters 1–12 as the original 3.0 manual and Chapters 13–14 as a 3.5 addendum. Its printed addendum pages are numbered 13-1 through 13-36. This is the primary direct 3.5 reference for additions not present in the German 1990 book's Chapter 18–19 treatment. The following is an additive reconciliation, not a replacement of the German book's wording.

### 34.1 Additional 3.5 features omitted from the German book extraction

| Feature | English addendum evidence | Source-derived behavior |
|---|---|---|
| `PCOLOR` | p. 13-34 | `PCOLOR foreground,background`; same color selection role as COLOR, but intended for subsequent PRINT output. |
| `GURU x` | p. 13-35 | x=0 disables GURU checking; nonzero enables handling of a GURU Meditation by invoking ON ERROR GOSUB or generating error #109. |
| `GURU` | p. 13-35 | Function returning the GURU number; example prints HEX$(GURU) in an ON ERROR handler. |
| `_0` … `_9` | p. 13-35 | Ten special longword variables, not floating-point variables. NEW clears them; CLEAR, LOAD, CHAIN and RUN do not. Suggested use includes retaining AllocMem addresses and library bases across RUN/CHAIN. |
| Negative screen number in `OPENW` | p. 13-34 | A negative screen number opens the window on the current (top) screen. |
| `MAT COPY` | pp. 13-6–13-10 | Four source/destination offset forms for copying whole matrices or subregions, in addition to the German book's MAT CPY form. |
| `MAT COPY` / `MAT XCPY` vector handling | pp. 13-11–13-13 | Explicit row-vector/column-vector rules, including the required dimensioning and which h/w parameters are effective. |

These are direct interpreter 3.5 addendum features. The German book's index and body do not provide equivalent detail for PCOLOR, GURU, _0…_9 or MAT COPY; do not attribute them to its Chapter 18/19 text.

### 34.2 MAT family constraints clarified by the English addendum

The English addendum states that all vector/matrix operations in its 3.5 section apply only to one- or two-dimensional fields with floating-point variables. This is a material constraint missing from the German book's broader “numeric variables” descriptions. The commands must therefore not be generalized to integer/byte/word arrays without runtime evidence.

- `MAT BASE 0/1`: meaningful in conjunction with OPTION BASE 0; sets the index offset for matrix operations. Default is 1. It affects MAT READ, MAT PRINT, MAT CPY, MAT XCPY, MAT ADD, MAT SUB and MAT MUL (the list is source-specific; do not silently extend it to every MAT command).
- `MAT CLR`, `MAT SET`, `MAT ONE`: CLEAR all elements to zero; set all elements to a scalar; create a square identity matrix respectively. The English prose calls MAT ONE a “uniform matrix” but defines diagonal elements as 1 and all others as 0.
- `MAT INPUT #channel,a()`: reads matrix/vector data from ASCII input; its format is the reverse of MAT PRINT and uses comma/line-feed separators.
- `MAT CPY`: copies h rows by w elements between explicit offsets. The addendum also documents four offset forms: destination offset/source whole matrix, destination whole matrix/source offset, both offsets, and both whole matrices. Only elements with matching source/destination indices are copied.
- `MAT XCPY`: same copy model but transposes the source while copying; the source array remains unchanged. For vectors, one must be dimensioned as a row vector and the other as a column vector. The addendum specifies when h or w is used and when zero prevents copying.
- `MAT TRANS a()`: in-place transpose; source says it is for square matrices. `MAT TRANS a()=b()` copies a transposed b() into dimension-compatible a().
- `MAT ADD` and `MAT SUB`: element-wise forms require matching dimensions; in-place forms overwrite the first matrix. Scalar forms add/subtract the scalar from every element.
- `MAT MUL a()=b()*c()`: standard row-by-column matrix product with compatible dimensions. For two vectors, the destination matrix receives their dyadic/external product. `MAT MUL x=a()*b()` for equal-length vectors returns the scalar (inner) product. `MAT MUL x=a()*b()*c()` is also documented for vector–matrix–vector multiplication.
- `MAT NORM a(),0/1`: normalizes by rows/columns so the sum of squares in each row/column is 1.
- `MAT DET`, `MAT QDET`, `MAT RANG`: operate on square matrices and support a selected n-by-n submatrix beginning at an i,j offset. QDET is the optimized determinant routine; the German book independently warns of small precision differences.
- `MAT INV b()=a()`: inverse of a square matrix a() into b(); source requires b() to be the same type as a().

### 34.3 German/English source differences and unresolved questions

1. **Numeric array type:** German Chapter 18 often says numeric fields; English addendum explicitly restricts its matrix/vector operations to floating-point fields. Use the stricter English constraint for the documented 3.5 addendum unless the interpreter proves otherwise.
2. **MAT COPY vs MAT CPY:** English addendum includes MAT COPY as a separate set of four offset forms. German Chapter 18 describes MAT CPY and its region syntax, but not these distinct MAT COPY forms. Do not collapse these spellings without parser/runtime evidence.
3. **MAT ONE wording:** German says identity matrix; English says “uniform matrix” then defines the identity matrix. Semantics agree despite terminology.
4. **MAT MUL:** German text covers matrix multiplication and scalar multiplication, while English additionally documents vector outer/dyadic product, vector scalar/inner product and vector–matrix–vector chain product.
5. **MAT NORM:** English makes the invariant explicit: row/column sum of squares becomes 1. German explains unit-length normalization but is less precise for matrix rows/columns.
6. **_DATA:** English states DATA is zero exactly when the next READ would be out of data; assigning _DATA restores only a value previously obtained from DATA. This clarifies the German book's less precise pointer description.
7. **Error #109 and GURU:** English addendum documents GURU-specific handling and warns that compiled programs cannot RESUME NEXT or RESUME LABEL after an ON ERROR call caused by a GURU. It recommends terminating promptly after #109; GURUs #110–#118 are described as less dangerous.
8. **FACT/VARIAT/COMBIN:** English addendum specifies 0!=1 and the combinatorial formulas. The German error appendix's code #86 says FACT/COMBIN/PERMUT not defined, while the chapter prose documents VARIAT. Preserve this source discrepancy; do not assume PERMUT is an alias.

### 34.4 Editor-specific 3.5 addendum

The English addendum p. 13-1 states:
- The editor uses two additional bytes per program row compared with versions 3.0–3.07; this accelerates backward scrolling and permits folding functions as well as procedures.
- Search can include header rows of folded/closed procedures or functions.
- Listing prints labels two characters further left, as with CASE.
- Tab moves to the next tab stop; Ctrl+Tab moves to the previous one; LeftShift+Tab inserts spaces up to the next tab stop; RightShift+Tab deletes blank spaces on one row up to/from the cursor.

This resolves the German book's OCR ambiguity for the two Shift+Tab actions. Treat the editor changes as editor behavior, not interpreter language syntax.


## 35. Final German-book index and appendix audit: printed pp. 486–508

The remaining back matter was checked against the Chapter 18–19 extraction. It contains no additional interpreter command descriptions, but it adds useful cross-checks and source boundaries.

### 35.1 German command-index evidence

The German alphabetical command index independently points to:
- `FACT` on printed p. 478.
- `VARIAT` on printed p. 478.
- `_DATA` / DATA-pointer operations on printed p. 479.
- Matrix/vector command family in Chapter 18, printed pp. 454–475, including `MAT CPY`, `MAT XCPY` and `MAT TRANS`.

The index corroborates the location of these entries but is not a complete 3.5 delta list. In particular, do not infer that a feature absent from the index does not exist: the English addendum separately documents PCOLOR, GURU, _0…_9 and the MAT COPY forms.

### 35.2 GFA-BASIC error appendix: additional exact GURU mapping

The German book's error appendix (printed p. 487) gives the following mapping:
| Error | Appendix wording |
|---:|---|
| 109 | Guru Meditation |
| 110 | Bus Error |
| 111 | Address Error |
| 112 | Illegal Instruction |
| 113 | Zero Divide |
| 114 | CHK Instruction |
| 115 | TRAPV Instruction |
| 116 | Privilege Violation |
| 117 | Trace |
| 118 | Line A Emulator |
| 119 | Line F Emulator |
| 120 | Argument line invalid or too long |
| 121 | No executable program |
| 122 | Invalid resident library addressed during load |

This gives a concrete exception-name table for the GURU handling introduced by the English 3.5 addendum. Note the boundary: the English addendum calls #110–#118 the “normal” GURUs $2–$A, while the German appendix additionally lists #119 as Line F Emulator. Do not silently extend the English statement's $2–$A range to #119.

The German appendix also lists #86 as “FACT/COMBIN/PERMUT not defined”. This remains inconsistent with the Chapter 19 function prose, which documents FACT, VARIAT and COMBIN. Treat PERMUT as an unresolved source spelling/compatibility question, not as a proven synonym for VARIAT.

### 35.3 DOS errors and appendix scope

The DOS error appendix lists AmigaDOS-level failures such as insufficient free store, full task table, invalid/overlong argument line, non-object/executable file, invalid resident library, object/lock errors, disk protection/fullness, missing device/disk and directory errors. These are operating-system error numbers and should remain distinct from GFA-BASIC interpreter error codes.

The last pages contain the alphabetical subject index and a source note crediting E. Lehmann's *Lineare Algebra mit dem Computer* for the matrix illustration example (German book printed p. 470). That credit identifies the example's external origin; it does not alter the GFA-BASIC command semantics described around it.

### 35.4 Completion boundary

The German book's substantive interpreter/compiler chapters and back matter are now indexed through its final printed page 508. The remaining work is no longer bulk text extraction: it is page-image verification of OCR-sensitive spellings/tables and runtime compatibility testing. The English 3.5 addendum has been separately reconciled through its final printed page 13-36. No claim is made that these manual-only sources establish undocumented runtime edge cases.


## 36. Page-image validation: IDCMP flags, Window flags and MENU attributes (printed pp. 378–379, 389)

Unlike the earlier OCR-only extraction, these three tables were checked directly against rendered page images. The decimal values and names below are legible in the source images.

### 36.1 IDCMP message flags (printed p. 378)

| Decimal | Hex | Flag name |
|---:|---|---|
| 1 | &H00000001 | SIZEVERIFY |
| 2 | &H00000002 | NEWSIZE |
| 4 | &H00000004 | REFRESHWINDOW |
| 8 | &H00000008 | MOUSEBUTTONS |
| 16 | &H00000010 | MOUSEMOVE |
| 32 | &H00000020 | GADGETDOWN |
| 64 | &H00000040 | GADGETUP |
| 128 | &H00000080 | REQSET |
| 256 | &H00000100 | MENUPICK |
| 512 | &H00000200 | CLOSEWINDOW |
| 1024 | &H00000400 | RAWKEY |
| 4096 | &H00001000 | REQCLEAR |
| 16384 | &H00004000 | NEWPREFS |
| 32768 | &H00008000 | DISKINSERTED |
| 65536 | &H00010000 | DISKREMOVED |
| 262144 | &H00040000 | ACTIVEWINDOW |
| 524288 | &H00080000 | INACTIVEWINDOW |
| 1048576 | &H00100000 | DELTAMOVE |
| 2097152 | &H00200000 | VANILLAKEY |
| 4194304 | &H00400000 | INTUITICKS |

The table is a bit-flag list: values are powers of two and can be combined as flags. This observation follows directly from the displayed decimal/hex columns; it does not establish any additional undocumented combinations or runtime delivery guarantees.

### 36.2 Window flags (printed p. 379)

| Decimal | Hex | Flag name |
|---:|---|---|
| 0 | &H00000000 | SMARTREFRESH |
| 1 | &H00000001 | WINDOW sizing |
| 2 | &H00000002 | WINDOWDRAG |
| 4 | &H00000004 | WINDOWDEPTH |
| 8 | &H00000008 | WINDOWCLOSE |
| 16 | &H00000010 | SIZEBRIGHT |
| 32 | &H00000020 | SIZEBOTTOM |
| 64 | &H00000040 | SIMPLEREFRESH |
| 128 | &H00000080 | SUPERBITMAP |
| 256 | &H00000100 | BACKDROP |
| 512 | &H00000200 | REPORTMOUSE |
| 1024 | &H00000400 | GIMMEZEROZERO |
| 2048 | &H00000800 | BORDERLESS |
| 4096 | &H00001000 | ACTIVATE |
| 65536 | &H00010000 | RMBTRAP |
| 131072 | &H00020000 | NOCARE refresh |

The source prints the second name as WINDOW SIZING (the OCR/text layer had merged it as WINDOW/SIZING) and the last as NOCARE REFRESH. The sample directly below the table is `OPENW 0,0,0,640,256,0,4111`, described as opening a window with all system gadgets and active display. Preserve the source spelling and the sample's literal final value; do not derive an alternative flag combination.

### 36.3 MENU item attributes (printed p. 389)

The page-image table reads:
- `64`: clear a checkmark.
- `256`: set (toggle/check) a checkmark.
- `16`: deactivate/ghost a menu item.
- `64`: activate a menu item.

The source therefore assigns decimal 64 to both “clear checkmark” and “activate menu item”. This is an internal duplicate in the printed source, not an OCR ambiguity. Do not silently change either value to a presumed correction. The text also states that a checkmark is requested by prefixing two spaces before the relevant menu-item text in MENU text.

### 36.4 Updated validation status

The IDCMP table, Window flags table and MENU attribute list are now page-image verified for printed pages 378, 379 and 389. Earlier OCR caveats for these exact numeric tables can be narrowed: the printed values are readable, but their interpretation against AmigaOS headers/runtime and the duplicate MENU value still require independent compatibility testing. Other OCR-sensitive areas (including SCROLL/GRAPHMODE/DRAW$, sprite/BOB syntax, and low-level API details) remain unvalidated.


## 37. Page-image validation: graphics scrolling, DRAW$ grammar and BOB velocity/planes (printed pp. 240, 245–250, 272–273)

These entries were checked against the rendered page images, replacing OCR-only readings for the exact syntax and spellings listed here.

### 37.1 GRAPHMODE examples (printed p. 240 and p. 250)

The page-240 example explicitly uses:
- `GRAPHMODE 3` with the adjacent comment “XOR-Modus für Flimmerbox”.
- `GRAPHMODE 1` with the adjacent comment “REPLACE-Modus an”.

Page 250 independently labels `GRAPHMODE 3` as XOR mode in its turtle-graphics example. These examples verify the two values and their described roles in this book; they do not provide a complete enumeration of all GRAPHMODE values or prove compatibility with other GFA versions.

### 37.2 SCROLL (printed pp. 245–246)

The page image confirms the exact syntax:

```basic
SCROLL Dx,Dy,X1,Y1,X2,Y2
```

The command shifts the screen bitmap rectangle bounded by the coordinate pairs (X1,Y1) and (X2,Y2). Dx and Dy separately specify shift direction and distance on X and Y. Positive Dx/Dy shift left/down; negative values shift in the opposite directions (right/up).

The example on p. 246 uses `SCROLL dx,dy,x1,y1,x2,y2` with random offsets and explicitly notes that pixels shifted out at the edges are not saved; the newly exposed opposite edge is filled with the background color. This edge behavior is directly stated in the source. The OCR caveat for SCROLL's parameter order and sign direction can therefore be removed.

### 37.3 DRAW$ (printed pp. 248–250)

The image confirms the header and syntax:

```basic
DRAW Def$, [Const, ["Def"] [,Var[, ...]]]
```

The command simulates plotter/turtle graphics on screen. Def$ may be an alphanumeric expression, string variable or text constant; the optional values may be expressions, constants or variables. The source limits the number of comma-separated individual values as described on p. 249 and states a maximum input-line length of 255 characters.

Documented turtle commands:
| Token | Meaning |
|---|---|
| `fd n` | Move pen forward n pixels. |
| `bk n` | Move pen backward n pixels. |
| `sx n`, `sy n` | Scale X/Y values used by fd and bk. |
| `sx 0`, `sy 0` | Disable the corresponding scaling. |
| `lt n`, `rt n` | Turn left/right by n degrees. |
| `tt n` | Set absolute direction to n degrees; degree convention is referenced to SETDRAW. |
| `ma x,y` | Move pen up to absolute position x,y. |
| `da x,y` | Move pen down to absolute position x,y and draw a line. |
| `mr x,y` | Move pen up by relative x,y from current turtle position. |
| `dr x,y` | Move pen down by relative x,y and draw a line. |
| `co n` | Set line color n (reference: COLOR). |
| `pu`, `pd` | Pen up/down. |

Page 250 confirms `DRAW()` as the turtle-attribute query. Its documented indices are 0=current X position, 1=current Y position, 2=current angle in degrees, 3=X scaling, 4=Y scaling, and 5=pen flag (-1=pen down, 0=pen up). The source says indices 0–4 return floating-point values.

### 37.4 OBJECT.VY and OBJECT.PLANES (printed p. 273)

The page image confirms the exact command spelling `OBJECT.VY` (not OBJECT.VY0 or OBJECT.VVO):

```basic
OBJECT.VY ObjNummer, Y_Geschwindigkeit
```

It sets an object's Y-direction velocity in pixels per second.

The same page confirms:

```basic
OBJECT.PLANES ObjNummer [,Bitebenen [,Ebenenwert]]
```

The accompanying prose describes the first value as selecting which bitplanes receive the BOB graphic. A set bit means the BOB graphic is drawn in that plane; clearing a bit removes that plane's color contribution. The optional Ebenenwert controls the state of otherwise disabled bitplanes, allowing colors to be added/removed from the BOB representation as described by the source. This is a low-level BOB operation; the book's prose is not a substitute for checking actual Amiga graphics-library behavior.

### 37.5 Validation boundary

The printed syntax, parameter order, sign convention, turtle tokens, DRAW() index meanings and OBJECT.VY spelling are now page-image verified. GRAPHMODE is verified only for the two explicit examples (1=REPLACE and 3=XOR); other values remain unspecified here. Runtime behavior, boundary clipping, floating-point precision and OBJECT.PLANES interaction with actual screen bitplanes still require interpreter/Amiga runtime tests.


## 38. Page-image validation: GET/PUT bitmap format and hardware sprites (printed pp. 257–268)

### 38.1 GET and PUT contract (printed pp. 257, 261)

The source describes `GET X_links,Y_oben,X_rechts,Y_unten,Var$` as capturing a screen rectangle into a string as a bitmap. All four corner coordinates must be inside the screen; otherwise the interpreter does not execute the command.

The page-image checked PUT syntax is:

```basic
PUT X_links,Y_oben,Var$ [,Modus,Maske]
```

PUT draws a previously captured GET image at the supplied upper-left position. The dimensions remain those stored by GET, so the destination lower-right corner follows from the stored width and height. The destination may be partially or wholly outside the screen. Without Modus, the source says Replace mode is used. It warns against negative Modus values because they can cause malfunctions.

### 38.2 PUT raster header and pixel planes (printed pp. 262–264)

The first three 16-bit words (six bytes) in a PUT string are:
1. Width minus one (`X_rechts - X_links`).
2. Height minus one (`Y_unten - Y_oben`).
3. Number of bitplanes.

The raster row width is padded to a multiple of 16 pixels. The source explicitly notes that unused trailing bits in the final word of a row contain unusable “bit garbage”. For a single bitplane, a set raster bit corresponds to a set pixel. With multiple planes, corresponding bits are overlaid; their bit-sum selects the color register. The page-264 example sums plane bits 1, 2 and 8 to obtain color register 11 for a pixel. This describes the source's string layout; it is not a guarantee of compatibility with every GFA version or screen mode.

### 38.3 PUT drawing modes (printed pp. 261–262)

The page-image checked mode table gives these hexadecimal values and descriptions:
| Name | Value | Source description |
|---|---:|---|
| DestInvert | `&H30` | Put the inverted screen area into the image. |
| SourceInvert | `&H50` | Invert the screen area within the image rectangle. |
| ExclDestSource | `&H60` | Exclusive-OR the screen area with the image. |
| OrDestSource | `&H80` | OR only the bits that are equal in screen and image. |
| DestSource | `&HC0` | Replace the screen area with the image. |

The German wording for OrDestSource is unusual (“only between bits that are equal”); it is retained as source wording rather than reinterpreted as a conventional Boolean OR definition. The optional Mask parameter is present in the syntax, but these pages do not fully define its semantics.

### 38.4 Hardware SPRITE (printed pp. 265–267)

The source lists four forms:
```basic
SPRITE #Nummer,Def.var$
SPRITE #Nummer [,0]
SPRITE #Nummer [,XPos,YPos]
SPRITE ON/OFF
```

The definition string is a bitmap-converted string. Each sprite is 16 pixels wide; each graphics row uses four bytes stored as ASCII characters. The two 16-bit bit rows are interpreted as two overlaid bitplanes, yielding four combinations: 00 transparent, 01 first color register, 10 second color register, 11 third color register.

The page-267 table assigns sprite-number pairs to color registers:
| Sprite numbers | Color registers |
|---|---|
| 0/1 | 17, 18, 19 |
| 2/3 | 21, 22, 23 |
| 4/5 | 25, 26, 27 |
| 6/7 | 29, 30, 31 |

The second form deletes a sprite; the source notes that sprite 0 is the mouse pointer and can be deleted this way. The position form uses a 320×256 coordinate raster independent of screen resolution. SPRITE ON/OFF enables or disables all sprites and also affects the mouse pointer. The sample creates a 64-byte sprite string, positions sprite #1 in a loop, synchronizes with VSYNC and finally issues SPRITE OFF.

### 38.5 Validation boundary

The GET/PUT syntax, coordinate restriction for GET, PUT placement behavior, five mode constants, three-word header, 16-pixel row padding and sprite format/color-register table are page-image checked. The source's mask parameter has not been fully specified in these pages; mask semantics, clipping details, color-register interaction on real Amiga hardware and sprite resource limits remain runtime/OS compatibility questions.
