# Amiga GFA BASIC 3.0 command index — source transcription

Source: *GFA BASIC 3.5 Interpreter*, Appendix 12, “Table of commands”, printed pp. 12-8–12-18 (PDF pp. 410–420). The manual says Chapters 2–12 are the original 3.0 baseline and Chapter 13 is the 3.5 addendum. This is a 3.0 discovery index, not a normative 3.5 declaration list.

## Reading this index

- Names and printed-page references below follow the appendix index; language-core and Amiga-specific APIs are mixed in its alphabetical table.
- An index entry proves only that a name is indexed at a page. It does not establish complete syntax, semantics, interpreter/compiler availability, or 3.5 retention.
- OCR-damaged names are marked for visual verification. Do not normalize them into registry identifiers without checking the page image and detailed entry.
- The index is a completeness cross-check, not a replacement for declaration-level documentation.

## Alphabetical index

| Name | Printed page | Area / qualification |
|---|---:|---|
| ! | 7-35 | Comment marker |
| $ | 7-49 | Compiler directive; version scope to verify |
| *, +, -, @, {}, ~ | 2-33–2-40; 3-10–3-13; 7-24 | Operators / typed memory / call notation; exact glyphs need review |
| _DosBase, _GfxBase, _IntBase, _LayersBase | 10-4 | Amiga OS library bases |
| _dosCmd$ | 10-2 | CLI command tail |
| ABS(), ACOS(), ASIN(), ATN(), COS(), COSQ(), DEG(), EXP(), LOG(), LOG100, RAD(), SGN(), SIN(), SINQ(), SQR(), TAN() | 4-5–4-12 | Numeric functions; LOG100 spelling needs visual check |
| ADD, ADD(), SUB, SUB(), MUL(), DIV, DIV(), MOD() | 4-2–4-3 | Arithmetic operators/functions; see detailed function inventory |
| AFTER, EVERY | 7-32 | Timer/event statements |
| ALERT, FILESELECT | 9-36–9-38 | Amiga requester APIs |
| AND, EQV, IMP, NOT, OR, XOR | 3-4–3-9 | Logical operators |
| AND(), EQV(), IMP(), OR(), XOR() | 4-23–4-24 | Bitwise functions |
| ARRAYFILL, DIM, DIM?(), ERASE, INSERT, DELETE, OPTION BASE, QSORT, SSORT, SWAP | 2-8–2-22 | Array declarations and operations |
| ARRPTR(), VARPTR(), TYPE(), FRE, DUMP, VOID, RESERVE | 2-29–2-44 | Variable, memory and runtime introspection |
| ASC(), CHR$(), HEX$(), OCT$(), STR$(), VAL(), VAL?() | 2-13–2-16 | Conversion functions; VAL? punctuation needs review |
| BACKS, FRONTS, MOVES, OPENS, CLOSES, SCREEN(), TITLES, SETPEN, SETSTITLE | 9-17–9-24 | Amiga screen APIs |
| BACKW, CLEARW, CLOSEW, FULLW, FRONTW, LIMITW, MOVEW, OPENW, SETWPEN, SETWTITLE, SIZEW, WINDOW() | 9-26–9-35 | Amiga window APIs |
| BGET, BLOAD, BPUT, BSAVE, CHDIR, CLOSE, DFREE(), DIR, DIR$(), EXIST(), FILES, KILL, LOF(), LOC(), NAME, OPEN, RENAME, RELSEEK, SEEK, TOUCH | 6-4–6-19 | Disk and file operations |
| BOUNDARY, BOX, CIRCLE, CLIP, CLS, COLOR, DEFFILL, DEFLINE, DISPLAY, DRAW, ELLIPSE, FILL, GRAPHMODE, LINE, PBOX, PCIRCLE, PELLIPSE, PLOT, POINT(), POLYFILL, POLYLINE, RASTPORT, SETCOLOR, SETDRAW, TEXT, VSYNC | 8-2–8-24 | Amiga graphics APIs |
| BCHG(), BSET(), BTST(), SHL(), SHR(), ROL(), ROR(), BYTE(), WORD() | 4-19–4-26 | Bit and numeric conversion operations; exact suffixes need source review |
| BYTE{}, CARD{}, CHAR{}, DOUBLE{}, FLOAT{}, INT{}, LONG{}, SINGLE{}, WORD{} | 2-37 | Typed memory access forms |
| BMOVE, DPEEK(), DPOKE, LPEEK(), LPOKE, PEEK(), POKE, MALLOC(), MFREE(), ABSOLUTE | 2-35–2-43 | Memory/address operations; Amiga address semantics |
| CASE, ELSE, ELSE IF, ENDIF, IF, SELECT | 7-4–7-9 | Conditional statements |
| CHAIN, EDIT, END, LIST, LLIST, LOAD, NEW, PSAVE, QUIT, RUN, SAVE, STOP, SYSTEM | 7-38–7-45 | Program/editor/system commands |
| DATA, READ, RESTORE | 5-15 | DATA statement and pointer operations |
| DATES, SETTIME, TIMES, TIMER | 2-26–2-28 | Date/time values and commands |
| DEFBIT, DEFBYT, DEFFLT, DEFINT, DEFSTR, DEFWRD, LET | 2-5–2-6 | Variable typing and assignment |
| DEFFN, ENDFUNC, FN, FUNCTION, LOCAL, PROCEDURE, RETURN | 7-21–7-26 | Procedures and user-defined functions |
| DELAY, PAUSE | 7-37 | Timing/delay statements |
| DO, DO UNTIL, DO WHILE, LOOP, LOOP UNTIL, LOOP WHILE, REPEAT, UNTIL, WHILE, WEND, FOR, NEXT, DOWNTO, EXIT IF | 7-13–7-20 | Loop statements |
| ERR, ERROR, ON BREAK CONT, ON BREAK GOSUB, ON ERROR, ON ERROR GOSUB, RESUME NEXT | 7-28–7-31 | Error handling |
| EXEC, INLINE, MONITOR, RCALL, SAY, SPRITE, STICK, STRIG, TRANSLATE$() | 7-50–7-55; 10-3–10-9 | Amiga CLI, machine, hardware and speech integration |
| FIELD, GET, PUT, RECORD, RECALL, STORE | 6-17–6-21 | Sequential and random-access file operations; GET/PUT also have graphics overloads |
| FORM INPUT, INKEY$, INPUT, INPUT$(), LINE INPUT, MOUSE, MOUSEK, MOUSEX, MOUSEY | 5-2–5-17 | Keyboard and pointer input |
| HARDCOPY, HTAB, LOCATE, LPOS(), LPRINT, MODE, POS(), PRINT, PRINT AT, PRINT USING, TAB(), VTAB, WRITE | 5-7–5-20 | Text, cursor and printer I/O |
| INP(), OUT | 6-13 | Byte I/O |
| INPUT #, LINE INPUT #, PRINT #, WRITE # | 6-14–6-16 | File-channel text I/O |
| LEFT$(), MID$(), RIGHT$(), INSTR(), RINSTR(), SPACE$(), SPC(), STRING$(), TRIM$(), UPPER$() | 4-27–4-35 | String functions |
| MAX(), MIN(), ODD(), EVEN(), PRED(), SUCC(), ROUND(), TRUNC(), FIX(), FRAC(), INT() | 4-11–4-16 | Numeric utility functions |
| MENU, MENU KILL, ON MENU, ON MENU BUTTON GOSUB, ON MENU GOSUB, ON MENU KEY GOSUB, ON MESSAGE GOSUB | 9-4–9-12 | Amiga menu/event APIs |
| OBJECT.AX, OBJECT.AY, OBJECT.CLIP, OBJECT.CLOSE, OBJECT.OFF, OBJECT.ON, OBJECT.PLANES, OBJECT.PRIORITY, OBJECT.SHAPE, OBJECT.START, OBJECT.STOP, OBJECT.VX, OBJECT.VY, OBJECT.X, OBJECT.Y | 10-14–10-16 | Amiga BOB object API |
| ON COLLISION GOSUB | 10-16 | Amiga BOB event |
| ON x GOSUB, GOSUB, GOTO, REM, TRON, TROFF, TRACES | 7-8; 7-21; 7-35–7-48 | Branching, comments and tracing |
| PRINT#, INPUT#, WRITE#, BLOAD, BSAVE, BGET, BPUT | 6-11–6-16 | File and binary operations |
| RANDOM(), RANDOMIZE, RAND(), RND() | 4-17 | Random-number functions |
| CINT(), CFLOAT() | 2-17 | Compiler-only conversions, not interpreter functions |
| CVD(), CVI(), CVL(), CVS(), MKD$(), MKI$(), MKL$(), MKS$() | 2-18 | Typed binary conversions; CVD/MKD$ OCR spellings disputed |
| SAY, SOUND, WAVE | 5-22; 10-9 | Sound/speech output; distinguish interpreter and Amiga facilities |
| TRUE, FALSE, PI | 2-25 | Constants |
| DATA, RESTORE, READ | 5-15 | 3.0 DATA statements; 3.5 pointer extensions are separate below |
| COLLISION ON/OFF/STOP | 10-13 | BOB collision control |
| _0…_9 | 13-35 | Explicit 3.5 special variables; not in 3.0 base index |

## Explicit 3.5 addendum names — separate version scope

The supplied interpreter scan's Chapter 13 explicitly documents:

- Linear matrix/vector operations: MAT BASE, MAT CLR, MAT SET, MAT ONE, MAT READ, MAT PRINT, MAT INPUT, MAT CPY, MAT XCPY, MAT TRANS, MAT ADD, MAT SUB, MAT MUL, MAT NORM, MAT DET, MAT QDET, MAT RANG, MAT INV (printed pp. 13-1–13-39).
- Additional commands/features: FACT, VARIAT, COMBIN, negative-screen-number OPENW variant, PCOLOR, _DATA, _DATA=, _0…_9, and GURU statement/function (printed pp. 13-33–13-36).
- Editor row/folding/search/listing changes and Tab/Ctrl+Tab/Shift+Tab operations (printed p. 1-25).

Full semantics and the alternate pagination crosswalk are in [the 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md).

## Separate Amiga OS routine index

Appendix 12, printed pp. 12-19–12-28, contains a separate alphabetical table of Amiga OS library calls (Exec, Graphics, Workbench, Intuition, DiskFont, Layers and DOS). It is a platform-call index, not part of the language-core declaration count. The source extraction of representative library families is in [system-routines evidence](amiga-gfa-basic-3.0-system-routines-evidence.md); the full per-call signatures still need their Chapter 11 entries reviewed before registry declarations are created.

## Completion status

This closes the appendix command-index discovery pass only. It does not close issue #19: the 3.0 base candidates still require declaration-level 3.5 confirmation or controlled interpreter observations; OCR-sensitive spellings require visual review; and the complete language-core/platform-API split, exact signatures and conformance fixtures must be linked to the canonical inventory. Absence from the 3.5 addendum is not evidence that a 3.0 feature was removed or retained.