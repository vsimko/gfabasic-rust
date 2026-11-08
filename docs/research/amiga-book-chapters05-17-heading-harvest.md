# Amiga book — command-heading harvest, chapters 5–17

Source: `Das Grosse Buch zu GFA-BASIC 3.5 Amiga.pdf`. This file records command headings surfaced by the page-level OCR pass. It is a navigation aid for subsequent verification, not a complete normalized API list. PDF page references are used.

## Chapter 5 — input/output (PDF pp. 75–144)

OCR-visible description headings include:
- FORM INPUT (p. 75)
- LINE INPUT (p. 82)
- LOCATE (p. 90)
- EXIST() (p. 98)
- KILL (p. 99)
- NAME (p. 102)
- RMDIR (p. 103)
- BGET (p. 104)
- BPUT (p. 105)
- EOF() (p. 106)
- LOF() (p. 108)
- TOUCH and STORE (p. 114)
- SOUND (p. 140)

The chapter contents additionally identify data input/output, screen operations, disk operations, file handling, random-access files, port I/O, Amiga DOS library, printer instructions, and sound/speech generation.

## Chapter 6 — program structure (PDF pp. 145–192)

OCR-visible headings:
- DO ... LOOP (p. 145)
- FOR ... NEXT (p. 146)
- REPEAT ... UNTIL (p. 147)
- EXIT IF (p. 149)
- DATA and READ (p. 168)
- REM (p. 169)
- FUNCTION ... RETURN ... ENDFUNC (p. 176)
- PROCEDURE ... ENDPROC (p. 184)
- MONITOR (p. 190)
- RCALL (p. 191)

Contents also cover conditional branches, scope declarations, variable declarations, subprograms, and assembler/C/PRG program calls.

## Chapter 7 — text operations (PDF pp. 193–200)

OCR-visible headings:
- MID$() assignment and extraction (p. 193 onward)
- LEN() (p. 196)
- SUCC() (p. 199)
- TRIM$() (p. 200)

The contents divide the chapter into string manipulation, string analysis, and string formatting.

## Chapter 8 — arithmetic (PDF pp. 201–224)

OCR-visible headings:
- MUL (p. 203)
- MOD() (p. 205)
- EVEN() and EXP() (p. 206)
- ACOS() (p. 210)
- SIN() (p. 213)
- BCHG() (p. 216)
- BTST() (p. 217)
- SWAP() and WORD() (p. 222)
- RANDOM() (p. 223)

The contents distinguish operators, mathematical operations, numeric functions, trigonometric functions, comparisons, bit operations, and random-value generation.

## Chapter 9 — graphics (PDF pp. 225–278)

OCR-visible headings:
- COLOR (p. 227)
- GRAPHMODE (p. 233)
- BOX / PBOX (p. 238)
- TEXT (p. 245)
- PUT (p. 261)
- VSYNC (p. 268)
- OBJECT.SHAPE, OBJECT.ON, OBJECT.OFF (p. 269–270)
- OBJECT.START, OBJECT.STOP (p. 271)
- OBJECT.AX, OBJECT.VY, OBJECT.PLANES (p. 272–273)
- OBJECT.HIT (p. 275)
- OBJECT.Y(), OBJECT.VX(), OBJECT.VY(), OBJECT.AY() and ON COLLISION GOSUB (p. 276–277)

Contents also identify graphics definitions, object graphics, line/point graphics, graphics operations, PUT-string organization, screen-memory organization, and object animation.

## Chapter 10 — data conversion (PDF pp. 279–290)

OCR-visible headings include CINT() (p. 285) and HEX$() (p. 286). The contents identify number systems as the chapter focus. Further conversion entries are present in Appendix D and require detailed page-by-page reconciliation.

## Chapter 11 — arrays, memory and pointers (PDF pp. 291–330)

OCR-visible headings:
- ARRAYFILL (p. 291)
- ERASE (p. 299)
- ABSOLUTE (p. 305)
- BMOVE (p. 306)
- FLOAT (p. 308)
- RESERVE (p. 315)

The contents divide this material into array operations, memory operations, memory management, pointer operations, and the Amiga Exec library.

## Chapter 12 — program control (PDF pp. 331–362)

OCR-visible headings:
- RUN (p. 333)
- PAUSE (p. 337)
- TIME$ (p. 339)
- FATAL (p. 342)
- AFTER / EVERY and EVERY CONT (pp. 348–352)

Contents additionally identify program start/end, erase functions, time operations, error handling, information queries, multitasking, and debugging.

## Chapter 13 — program/user interaction (PDF pp. 363–374)

OCR-visible heading:
- ALERT (p. 363)

The chapter includes interaction routines and example procedures; additional headings need a closer scan.

## Chapter 14 — window and screen programming (PDF pp. 375–388)

OCR-visible headings:
- FRONTW (p. 380)
- MOVEW and SIZEW (p. 381)
- WINDOW() (p. 383)
- CLOSES (p. 386)
- MOVES (p. 387)

The contents distinguish window commands from screen commands.

## Chapter 15 — menu programming (PDF pp. 389–394)

OCR-visible heading:
- MENU and menu-item attributes (p. 389)
- MENU KEY (p. 390)

## Chapter 16 — event monitoring (PDF pp. 395–404)

OCR-visible material:
- MENU(index) event buffer fields and event values (pp. 395–402)
- ON MENU (p. 403)
- ON MENU GOSUB and ON MENU BUTTON GOSUB (p. 404)

## Chapter 17 — GFA compiler (PDF pp. 405–450)

OCR-visible material includes compiler shell/menu use, compiler examples, compiler BASIC and advanced compiler use. This chapter is primarily compiler workflow and diagnostics, so not every heading represents a language command. Contents identify compiler examples/results, detailed operation (Workbench and CLI), linker error messages, effective compiler BASIC, advanced compiler use, and compiler-shell additions.

## Limits of this harvest

- The heading detector sampled OCR-visible heading-like lines near page starts; it does not guarantee every heading on a page was captured.
- OCR confuses punctuation and tokens (for example, percent suffixes, parentheses, and Amiga API names). Preserve uncertainty until the corresponding page image is checked.
- These chapters largely describe the V3.03 base interpreter according to the book introduction. Do not label entries as 3.5 additions without explicit version evidence.
