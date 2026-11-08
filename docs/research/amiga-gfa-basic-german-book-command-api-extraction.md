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
