# Amiga GFA BASIC commands observed in dqh-amiga

- **Status:** Source-derived inventory; not a complete language/API specification.
- **Source:** `vsimko/dqh-amiga`, combined source file `dqh.lst` (game source; GFA-BASIC dialect/version is not independently established here).
- **Target relevance:** Amiga-specific syntax and OS-call usage candidates for the Amiga GFA BASIC 3.5 research. A source occurrence proves use in this program, not by itself the complete syntax or 3.5 compatibility contract.
- **Related:** [Amiga GFA BASIC 3.5 primary reference](amiga-gfa-basic-35-primary-reference.md), [Amiga API reference](../reference/amiga-gfa-api.md).

This inventory records the GFA BASIC constructs and Amiga system/library calls identified while inspecting `dqh.lst`. It intentionally separates interpreter commands from direct OS calls.

## GFA BASIC screen and window commands

Observed in the source:
`OPENS`, `OPENW`, `CLOSES`, `CLOSEW`, `SCREEN`, `WINDOW`, `DISPLAY`, `FRONTS`, `RASTPORT`, `SETWPEN`, `SETSPEN`, `SETCOLOR`, `COLOR`, `CLEARW`, `GET`, `PUT`, `SCROLL`, `BOX`, `PBOX`, `PCIRCLE`, `LINE`, `PLOT`, `BMOVE`.

The 3.0 Amiga interpreter manual's screen/window chapter additionally indexes the following related forms. They are **manual-derived candidates**, not claimed as occurrences in `dqh.lst`:
`MOVES`, `BACKS`, `TITLES`, `TITLEW`, `FULLW`, `MOVEW`, `SIZEW`, `LIMITW`, `FRONTW`, `BACKW`.

The manual describes `OPENS` as opening a screen and `OPENW` as opening a window. The screen command family includes `OPENS`, `CLOSES`, `MOVES`, `FRONTS`, `BACKS`, `TITLES`; the window family includes `OPENW`, `CLOSEW`, `CLEARW`, `TITLEW`, `FULLW`, `MOVEW`, `SIZEW`, `LIMITW`, `FRONTW`, `BACKW`. The manual's printed pages for the relevant chapter begin at 9-14; `OPENS` is on 9-17–9-18, `SCREEN` on 9-25, and `OPENW` on 9-26–9-27. These are 3.0 base-manual references and need feature-level 3.5 confirmation.

## Direct Amiga OS/library calls observed

Calls written with the source's `~` call notation:
`ActivateWindow`, `Close`, `CloseFont`, `CloseLibrary`, `CloseWorkBench`, `Delay`, `Execute`, `Forbid`, `FreeMem`, `Move`, `OpenWorkBench`, `Permit`, `ScreenToFront`, `SetAPen`, `SetDrMd`, `SetFont`, `SetSoftStyle`, `Text`.

Additional direct Amiga calls observed without that notation:
`AllocMem`, `OpenLibrary`, `OpenDiskFont`.

The supplied interpreter manual's Amiga system-routine appendix indexes several of these names, including `OpenLibrary` (printed p. 11-15), `OpenDiskFont` (p. 11-76), `CloseFont` (p. 11-77), `ScreenToFront` (p. 11-72), `SetAPen` (p. 11-47), `SetDrMd` (p. 11-48), `SetFont` (p. 11-48), and `SetSoftStyle` (p. 11-49). The manual is a 3.0 base manual with a separate 3.5 addendum; treat these as historical API evidence pending 3.5 reconciliation.

## Other GFA BASIC constructs observed in the source

- **Memory and typed access:** `BYTE{...}`, `CARD{...}`, `WORD{...}`, `LONG{...}`, `DIM`, `ERASE`, `RESERVE`, `VARPTR`.
- **Procedures and functions:** `PROCEDURE`, `RETURN`, `FUNCTION`, `ENDFUNC`, `DEFFN`, `LOCAL`.
- **Control flow and error/event handling:** `ON ERROR GOSUB`, `ON BREAK GOSUB`, `ON ... GOSUB`, `SELECT`, `CASE`, `ENDSELECT`, `IF`, `ELSE`, `ENDIF`, `FOR`, `NEXT`, `DO`, `LOOP`, `WHILE`, `WEND`, `REPEAT`, `UNTIL`, `GOTO`, `GOSUB`.
- **Source/data and file I/O:** `MERGE`, `DATA`, `READ`, `OPEN`, `CLOSE`, `INPUT`, `WRITE`, `EXIST`.
- **Other:** `EDIT`, `INKEY$`, `STRIG`, `STICK`.

These are observed spellings/forms, not a complete grammar. In particular, `CREATE` in the game source is a user-defined procedure, not an Amiga OS call. `OpenScreen` and `OpenWindow` are documented Amiga library routines in the manual's system-call index, but are not included in the direct-call list above unless a source occurrence is confirmed.

## Follow-up

- Extract exact source locations and call argument lists from `dqh.lst`.
- Reconcile each GFA command with the 3.5 interpreter manual; do not promote 3.0-only manual evidence to confirmed 3.5 behavior.
- For OS calls, record library ownership, argument/structure types, return values, and resource lifetime before defining runtime APIs.
