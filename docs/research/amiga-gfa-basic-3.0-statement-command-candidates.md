---
type: "Research Note"
title: "Amiga GFA BASIC 3.0 statement and command candidate catalogue"
---

# Amiga GFA BASIC 3.0 statement and command candidate catalogue

Source: *GFA BASIC 3.5 Interpreter*, Appendix 12 BASIC command index, printed pp. 12-8–12-18, cross-checked against the detailed chapter evidence files. The index is in the 3.0 base-manual portion of the book. Every row below is therefore a **3.0 candidate**, not an assertion of Amiga GFA BASIC 3.5 availability.

## Language-core statements and declarations

| Candidate | Source page(s) | Feature group | 3.5 evidence state |
|---|---:|---|---|
| LET | 2-6 | Assignment | inherited 3.0 candidate; 3.5 confirmation required |
| DEFBIT, DEFBYT, DEFWRD, DEFINT, DEFFLT, DEFSTR | 2-5 | Default variable typing | inherited 3.0 candidate; 3.5 confirmation required |
| DIM, OPTION BASE, ERASE, CLEAR, CLR | 2-7–2-10 | Variable/array lifecycle | inherited 3.0 candidate; 3.5 confirmation required |
| INSERT, DELETE, ARRAYFILL, SWAP, QSORT, SSORT | 2-11–2-22 | Array mutation and sorting | inherited 3.0 candidate; 3.5 confirmation required |
| POKE, DPOKE, LPOKE; ABSOLUTE; RESERVE; BMOVE; MALLOC, MFREE | 2-35–2-44 | Memory/address operations | inherited 3.0 candidate; host-address semantics unresolved |
| IF, ELSE, ELSE IF, ENDIF | 7-4–7-7 | Conditional control flow | inherited 3.0 candidate; 3.5 confirmation required |
| ON x GOSUB | 7-8 | Computed subroutine branch | inherited 3.0 candidate; 3.5 confirmation required |
| SELECT, CASE, DEFAULT, ENDSELECT | 7-9–7-10 | Multi-way selection | inherited 3.0 candidate; 3.5 confirmation required |
| FOR, NEXT, DOWNTO | 7-13–7-14 | Counted loops | inherited 3.0 candidate; exact termination edge cases unresolved |
| REPEAT, UNTIL | 7-15 | Post-tested loop | inherited 3.0 candidate; 3.5 confirmation required |
| WHILE, WEND | 7-16 | Pre-tested loop | inherited 3.0 candidate; 3.5 confirmation required |
| DO, LOOP, DO WHILE, DO UNTIL, LOOP WHILE, LOOP UNTIL | 7-17–7-19 | DO-loop variants | inherited 3.0 candidate; exact grammar needs visual review |
| EXIT IF | 7-20 | Conditional loop exit | inherited 3.0 candidate; 3.5 confirmation required |
| GOSUB, RETURN, PROCEDURE, LOCAL | 7-21–7-23 | Procedures/subroutines | inherited 3.0 candidate; scope and parameter semantics require extraction |
| FUNCTION, ENDFUNC, DEFFN, FN | 7-24–7-27 | User-defined functions | inherited 3.0 candidate; parameter/return semantics require extraction |
| ON BREAK CONT, ON BREAK GOSUB | 7-28 | Break handling | inherited 3.0 candidate; event/runtime behavior unresolved |
| ON ERROR, ON ERROR GOSUB, ERROR, ERR, RESUME NEXT | 7-29–7-31 | Error handling | inherited 3.0 candidate; 3.5 error semantics require confirmation |
| EVERY, AFTER | 7-32–7-34 | Timer events | inherited 3.0 candidate; scheduler behavior unresolved |
| REM, ! | 7-35 | Comments | inherited 3.0 candidate; lexical comment rules require confirmation |
| GOTO | 7-36 | Branching | inherited 3.0 candidate; label grammar requires confirmation |
| PAUSE, DELAY | 7-37 | Timing | inherited 3.0 candidate; platform timing semantics unresolved |
| END, STOP, EDIT | 7-38 | Program/editor control | inherited 3.0 candidate; editor-only aspects separated |
| NEW, LOAD, SAVE, PSAVE, LIST, LLIST, CHAIN, RUN, SYSTEM, QUIT | 7-39–7-45 | Program lifecycle/editor commands | inherited 3.0 candidate; distinguish language runtime from editor commands |
| TRON, TROFF, TRACES | 7-46–7-48 | Trace/debug output | inherited 3.0 candidate; output behavior requires confirmation |
| INLINE, RCALL, $ | 7-49–7-55 | Machine/compiler integration | inherited 3.0 candidate; Amiga-specific/compiled context; not portable core |
| DATA, READ, RESTORE | 5-15 | Inline data and read pointer | 3.0 forms are inherited candidates; 3.5 adds separate _DATA/_DATA= controls |

## Input/output and resource statements

| Candidate | Source page(s) | Feature group | 3.5 evidence state |
|---|---:|---|---|
| FORM INPUT, FORM INPUT AS | 5-6 | Form input | inherited 3.0 candidate; exact form grammar unresolved |
| PRINT, PRINT AT, PRINT USING, WRITE | 5-7–5-9 | Screen output | inherited 3.0 candidate; formatting grammar unresolved |
| DEFNUM, MODE | 5-11 | Numeric input/format mode | inherited 3.0 candidate |
| HTAB, VTAB, POS, CRSCOL, CRSLIN | 5-13 | Cursor state | inherited 3.0 candidate |
| MOUSE, MOUSEK, MOUSEX, MOUSEY | 5-17 | Pointer input | Amiga-specific inherited candidate |
| LPRINT, LPOS, HARDCOPY | 5-19–5-21 | Printer output | Amiga/device-specific inherited candidate |
| SOUND, WAVE | 5-22 | Sound output | Amiga-specific inherited candidate; detailed signatures require review |
| CHDIR, DIR, DIR$, DFREE, FILES, EXIST | 6-4–6-6 | Disk/directory operations | inherited 3.0 candidate; Amiga path semantics |
| OPEN, CLOSE, LOF, LOC, EOF, TOUCH | 6-7–6-9 | File channel lifecycle/metadata | inherited 3.0 candidate; 3.5 confirmation required |
| NAME, RENAME, KILL | 6-11 | File management | inherited 3.0 candidate |
| BLOAD, BSAVE, BGET, BPUT | 6-12 | Binary I/O | inherited 3.0 candidate; memory layout/byte behavior unresolved |
| INP, OUT, INPUT$ | 6-13–6-14 | Byte/character I/O | inherited 3.0 candidate; channel behavior unresolved |
| INPUT #, LINE INPUT #, PRINT #, WRITE # | 6-14–6-16 | Sequential file I/O | inherited 3.0 candidate; serialization/encoding unresolved |
| STORE, RECALL | 6-16–6-17 | String-array persistence | inherited 3.0 candidate; exact RECALL optional syntax OCR-sensitive |
| SEEK, RELSEEK | 6-18 | File positioning | inherited 3.0 candidate; offsets and EOF boundary behavior unresolved |
| FIELD, GET, PUT, RECORD | 6-20–6-21 | Random-access records | inherited 3.0 candidate; record layout and binary compatibility unresolved |

## Amiga graphics, screen, window and event commands

| Candidate | Source page(s) | Feature group | 3.5 evidence state |
|---|---:|---|---|
| COLOR, SETCOLOR, DEFFILL, BOUNDARY, DEFLINE, GRAPHMODE | 8-2–8-7 | Drawing state/palette/raster mode | Amiga-specific inherited candidate |
| RASTPORT, CLIP | 8-8–8-9 | RastPort target and clipping | Amiga-specific inherited candidate; unsafe OS pointer behavior |
| PLOT, LINE, DRAW, SETDRAW | 8-10–8-14 | Point/line/turtle drawing | Amiga-specific inherited candidate |
| BOX, PBOX, CIRCLE, PCIRCLE, ELLIPSE, PELLIPSE | 8-15–8-16 | Geometric primitives | Amiga-specific inherited candidate |
| POLYLINE, POLYFILL, POINT, FILL | 8-17–8-19 | Polygon and fill | Amiga-specific inherited candidate |
| CLS, TEXT, GET, PUT, VSYNC, DISPLAY | 8-20–8-24 | Text, capture, synchronization and display | Amiga-specific inherited candidate |
| MENU, MENU KILL, ON MENU, ON MENU GOSUB, ON MENU BUTTON GOSUB, ON MENU KEY GOSUB, ON MESSAGE GOSUB | 9-4–9-12 | Menu/event API | Amiga-specific inherited candidate; detailed event signatures incomplete |
| OPENS, CLOSES, FRONTS, BACKS, MOVES, TITLES, SETPEN, SETSTITLE, SCREEN | 9-17–9-24 | Screen management | Amiga-specific inherited candidate |
| OPENW, CLOSEW, CLEARW, TITLEW, BACKW, FRONTW, FULLW, MOVEW, SIZEW, LIMITW, SETWPEN, SETWTITLE, WINDOW | 9-26–9-35 | Window management | Amiga-specific inherited candidate; negative screen-number OPENW variant explicitly added in 3.5 |
| ALERT, FILESELECT | 9-36–9-38 | Requesters/file selector | Amiga-specific inherited candidate |

## Advanced Amiga integration

| Candidate | Source page(s) | Feature group | 3.5 evidence state |
|---|---:|---|---|
| _dosCmd$, EXEC | 10-2–10-3 | CLI/process integration | Amiga-specific inherited candidate |
| _DosBase, _IntBase, _GfxBase, _LayersBase | 10-4 | Library-base variables | Amiga-specific inherited candidate |
| SPRITE | 10-5–10-6 | Hardware sprites | Amiga-specific inherited candidate; hardware resource lifecycle |
| STICK, STRIG | 10-7–10-8 | Joystick input | Amiga-specific inherited candidate |
| SAY, TRANSLATE$ | 10-9 | Speech | Amiga-specific inherited candidate; narrator.device/translator.library dependency |
| FD files, RCALL | 10-10–10-12 | OS routine wrapper integration | Amiga-specific inherited candidate; register ABI and memory safety unresolved |
| OBJECT.* and ON COLLISION GOSUB | 10-13–10-16 | BOB graphics objects/collision | Amiga-specific inherited candidate; collision forms and object lifecycle need declaration-level extraction |

## Explicit 3.5-only additions are not included in the candidate rows above

The separate [3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md) records the explicit matrix/vector family, FACT/VARIAT/COMBIN, negative-screen-number OPENW variant, PCOLOR, _DATA/_DATA=, _0…_9, GURU, and editor changes. These are source-confirmed 3.5 additions and must not be mixed with the inherited 3.0 candidates.

## Coverage and remaining verification

This catalogue expands the 3.0 command index into candidate groups for statements, I/O, graphics, UI and advanced Amiga integration. It is not a complete declaration-by-declaration registry: function identifiers are inventoried separately, and Appendix 12 also lists Amiga OS library routines. The exact 3.5 availability of each inherited candidate remains unresolved unless independently confirmed by the addendum or a named runtime observation. No item is classified as removed merely because the addendum does not mention it.

To close issue #19, these rows must be reconciled individually with the 3.5 interpreter or source-specific evidence; syntax, argument/return types, side effects, errors and test IDs must be added where required. The original interpreter binary/build and a controlled test environment are needed to resolve behavior not stated by the available manuals.
