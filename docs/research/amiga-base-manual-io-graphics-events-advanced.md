---
type: "Research Note"
title: "Amiga GFA BASIC base-manual I/O, graphics, event, and advanced-programming extraction"
---

# I/O, graphics, event, and advanced-programming forms

> Source-derived chapter inventory from the 3.0 base text included in the supplied *GFA BASIC 3.5 Interpreter* manual. The manual's opening note says it was originally printed for version 3.0 and that Chapter 13 alone is the 3.5 addendum. Therefore, this extraction is evidence for the documented 3.0 base language, **not by itself proof of availability or identical behavior in GFA BASIC 3.5**.

## Scope and transcription policy

This note covers Chapters 5, 6, 8, 9, and 10. Names and page locators below follow the printed contents and chapter headings. Compound headings are retained as groups where the contents page groups related forms. This is a heading-level extraction: it does not infer undocumented signatures or semantics from names alone. Printed page references are manual page labels, not PDF viewer page numbers.

## Chapter 5 — Input and output

| Manual heading / forms | Printed pages | Source scope |
|---|---:|---|
| `INKEY$` | 5-2 | Keyboard character input |
| `INPUT` | 5-3 | Interactive variable input |
| `LINE INPUT` | 5-5 | Line-oriented string input |
| `FORM INPUT` | 5-6 | Bounded string input |
| `PRINT` | 5-7 | Screen output and cursor positioning |
| `PRINT USING` | 5-9 | Formatted output |
| `MODE` | 5-11 | Numeric output formatting mode |
| `DEFNUM` | 5-12 | Default numeric output precision |
| `CRSCOL`, `CRSLIN`, `POS`, `TAB`, `HTAB`, `VTAB` | 5-13 | Cursor position and tabulation |
| `DATA`, `READ`, `RESTORE` | 5-15 | Reading constants embedded in a program |
| `MOUSE`, `MOUSEX`, `MOUSEY`, `MOUSEK` | 5-17 | Mouse position and button state |
| `LPRINT`, `LPOS` | 5-19 | Printer output and printer cursor position |
| `HARDCOPY` | 5-20 | Screen/window hardcopy |
| `SOUND`, `WAVE` | 5-22 | Sound output and waveform definition |

## Chapter 6 — Disk handling and file organization

| Manual heading / forms | Printed pages | Source scope |
|---|---:|---|
| File specification | 6-2 | Path and file naming conventions; explanatory section |
| Random access | 6-3 | Records and fields; explanatory section |
| `CHDIR`, `DIR$`, `DFREE` | 6-4 | Current directory and free disk space |
| `DIR`, `FILES` | 6-6 | Directory listings |
| `EXIST` | 6-7 | File existence test |
| `OPEN` | 6-8 | Open file/device channel |
| `LOF`, `LOC`, `EOF`, `CLOSE`, `TOUCH` | 6-9 | File length, position, end-of-file, closing, and timestamp |
| `NAME`, `RENAME`, `KILL` | 6-11 | Rename and delete files |
| `BLOAD`, `BSAVE`, `BGET`, `BPUT` | 6-12 | Binary file and memory transfer |
| `INP`, `OUT` | 6-13 | Byte input/output on an opened channel |
| `INPUT$()` | 6-14 | Read a character string from keyboard or channel |
| `INPUT#`, `LINE INPUT` | 6-15 | Read from a file channel |
| `PRINT#`, `WRITE#` | 6-16 | Write to a file channel |
| `STORE`, `RECALL` | 6-17 | Store/restore string-array data |
| `SEEK`, `RELSEEK` | 6-19 | Position a file data pointer |
| `FIELD` | 6-20 | Define fields in random-access records |
| `GET`, `PUT`, `RECORD` | 6-21 | Read/write random-access records |

## Chapter 8 — Graphics

| Manual heading / forms | Printed pages | Source scope |
|---|---:|---|
| `SETCOLOR` | 8-2 | Color register configuration |
| `DEFFILL` | 8-3 | Fill color, style, and pattern |
| `BOUNDARY` | 8-5 | Automatic fill boundary control |
| `DEFLINE` | 8-6 | Line style definition |
| `GRAPHMODE` | 8-7 | Graphics combination mode |
| `RASTPORT` | 8-8 | Select drawing rastport |
| `CLIP` | 8-9 | Clipping region and offset |
| `PLOT`, `LINE`, `DRAW` | 8-10 | Point, line, and relative/connected drawing |
| `DRAW`, `SETDRAW` | 8-12 | Drawing pattern and drawing state |
| `BOX`, `PBOX` | 8-15 | Outlined and filled rectangles |
| `CIRCLE`, `ELLIPSE` | 8-16 | Circle and ellipse drawing |
| `POLYLINE` | 8-17 | Connected line segments |
| `POINT` | 8-18 | Read a pixel/color value |
| `FILL` | 8-19 | Area filling |
| `CLS` | 8-20 | Clear graphics area |
| `TEXT` | 8-21 | Text output in graphics context |
| `GET`, `PUT` | 8-22 | Capture and restore image regions |
| `VSYNC` | 8-23 | Synchronize with vertical blank |
| `DISPLAY` | 8-24 | Display bitmap/image data |

## Chapter 9 — Event, menu, screen, and window handling

| Manual heading / forms | Printed pages | Source scope |
|---|---:|---|
| Event handling | 9-2 | Introductory event-handling section |
| Pull-down menus | 9-3 | Introductory menu section |
| `MENU` | 9-4 | Menu definition |
| `MENU()` | 9-5 | Menu-related function/form as printed in the contents |
| `ON MENU` | 9-9 | Menu event handler |
| `ON MENU...GOSUB` | 9-10 | Menu event subroutine handler |
| `MENU KILL`, `MENU X,Y` | 9-12 | Menu removal and item positioning |
| Screen commands | 9-14 | Introductory screen-command section |
| `OPENS` | 9-17 | Open a screen |
| `CLOSES` | 9-19 | Close a screen |
| `FRONTS`, `BACKS` | 9-20 | Screen depth ordering |
| `MOVES` | 9-21 | Move a screen |
| `TITLES` | 9-22 | Set screen title |
| `SETSPEN` | 9-23 | Set screen detail/block pens |
| `SCREEN` | 9-24 | Screen index/address lookup |
| Window commands | 9-25 | Introductory window-command section |
| `OPENW`, `CLOSEW` | 9-26 | Open and close a window |
| `CLEARW`, `TITLEW`, `FRONTW`, `BACKW`, `FULLW` | 9-32 | Clear, title, depth, and sizing state |
| `MOVEW`, `SIZEW`, `LIMITW` | 9-33 | Move, resize, and constrain a window |
| `SETWPEN` | 9-34 | Set window detail/block pens |
| `WINDOW` | 9-35 | Window index/address lookup |
| `ALERT` | 9-36 | Alert requester |
| `FILESELECT` | 9-37 | File selector requester |

## Chapter 10 — Advanced programming

| Manual heading / forms | Printed pages | Source scope |
|---|---:|---|
| CLI call / `_dosCmd$` | 10-2 | Start GFA-BASIC from CLI and inspect command-line context |
| `EXEC` | 10-3 | Start a CLI process |
| Library variables | 10-4 | Library base variables (`DosBase`, `IntBase`, `GfxBase`, `LayersBase`) |
| `SPRITE` | 10-5 | Hardware sprite data, positioning, and enable/disable |
| `STICK`, `STRIG` | 10-7 | Joystick/mouse-port state |
| `SAY`, `TRANSLATE$()` | 10-9 | Speech output and phoneme translation |
| FD-files | 10-10 | Include/use Amiga function definition files; explanatory/program example |
| Blitter objects | 10-13 | BOB concept and setup; explanatory section |
| `OBJECT` command family | 10-14 | BOB/object positioning, acceleration, clipping, enablement, and movement |

## Version and classification boundary

The supplied English manual explicitly identifies Chapters 1–12 as the 3.0 base text and Chapter 13 as the 3.5 addendum. Accordingly:

- Entries in this note are source-attested in the manual's 3.0 base chapters.
- A heading in the contents is evidence that the manual documents the form, not a complete grammar or runtime test.
- This note does not promote any entry into the repository's confirmed 3.5 language-core inventory.
- Chapter 13 additions must be evaluated separately; absence from these base chapters does not prove absence from 3.5.

## Source locator

*GFA BASIC 3.5 Interpreter*, GFA Data Media, first printed May 1990. The title/contents matter on PDF pp. 3–10 state that the manual was originally printed for version 3.0 and Chapter 13 is the 3.5 addendum. Chapter 5 begins at printed p. 5-1 (PDF p. 131); Chapter 6 at p. 6-1 (PDF p. 153); Chapter 8 at PDF p. 231; Chapter 9 at p. 9-1 (PDF p. 255); and Chapter 10 at p. 10-1 (PDF p. 295).
