# DQH-Amiga command coverage priorities

## Source and method

Inventory based on the original [vsimko/dqh-amiga](https://github.com/vsimko/dqh-amiga) source, primarily `dqh.lst` (2,150 lines), supported by split `src/*.lst` files. This is a usage inventory, not a claim that every listed token is a GFA BASIC built-in: project procedures such as `newlevel`, `tim`, `pis`, and `gameerror` are excluded from interpreter-command coverage.

Priorities follow frequency and centrality in DQH, then testability and risk. Native Amiga observations must remain distinct from Rust interpreter tests.

## Priority 1 — deterministic language features

Used heavily by DQH and suitable for exact result assertions:

- Numeric: `INT` (116 occurrences), `ABS`, `MIN`, `SGN`, `RAND` (random; smoke/statistical tests only).
- Strings: `CHR$`, `LEFT$`, `MID$`, `LEN`, `UPPER$`, `STR$`, `STRING$`, `ASC`, `MKI$`, `CVI`.
- Control flow: `IF/ELSE/ENDIF`, `SELECT/CASE/ENDSELECT`, `FOR/NEXT`, `WHILE/WEND`, `REPEAT/UNTIL`, `DO/LOOP`, `ON ... GOSUB`, `GOTO`, `GOSUB/RETURN`, `EXIT`.
- Data/storage: `DIM`, indexed arrays, `DATA/READ`, `ERASE`, `LOCAL`, `CLR`, `INC`, `DEC`, `ADD`, `SUB`, `MUL`.

The native probe now includes exact-result checks for DQH numeric/string primitives and WHILE, REPEAT and SELECT control flow.

## Priority 2 — graphics and screen/window lifecycle

Core to the game loop and rendering:

- Drawing: `COLOR`, `SETCOLOR`, `BOX`, `PBOX`, `PCIRCLE`, `LINE`, `PLOT`, `GET`, `PUT`, `SCROLL`.
- Screen/window: `OPENS`, `OPENW`, `CLEARW`, `CLOSEW`, `CLOSES`, `SCREEN()`, `WINDOW()`, `FRONTS`, `DISPLAY`, `RASTPORT`, `SETWPEN`, `SETSPEN`.

Next tests should run in a dedicated native graphics fixture, use fixed palette and geometry, and verify command completion plus screenshot evidence. Avoid coupling pixel-level assertions to OCR.

## Priority 3 — input, files and memory

- Input: `INKEY$`, `STICK`, `STRIG`.
- Files: `OPEN`, `CLOSE`, `EXIST`, `INPUT`, `INPUT$`, `READ`, `WRITE`.
- Memory/data representation: `BYTE{}`, `WORD`, `VARPTR`, `BMOVE`, `AllocMem`, `FreeMem`.

Keyboard/joystick input needs scripted emulator input; file tests should use the writable test ADF. Memory tests must allocate and release their own blocks and check bounds.

## Priority 4 — Amiga OS integration

DQH calls OS/library routines including `Open`, `Read`, `Close`, `Execute`, `OpenLibrary`, `CloseLibrary`, `Delay`, `Forbid`, `Permit`, `ActivateWindow`, `CloseWorkBench`, `OpenWorkBench`, `OpenDiskFont`, `CloseFont`, `SetFont`, `SetSoftStyle`, `AskSoftStyle`, `SetAPen`, `SetDrMd`, and `ScreenToFront`. Test these only in isolated emulator fixtures with explicit cleanup; they are not ordinary portable BASIC primitives.

## Explicit exclusions

DQH contains many project-defined procedures/functions (movement, menus, animation, scoring, level handling). They are not interpreter commands and must not be counted as missing language features. Some names in source are variables, labels, or Amiga OS library calls rather than GFA BASIC statements.

## Follow-up

1. Expand Priority 1 tests for remaining operators and branch forms.
2. Add a graphics/window lifecycle fixture for Priority 2.
3. Add deterministic file and memory fixtures for Priority 3.
4. Add scripted input and isolated OS integration tests for Priorities 3–4.
