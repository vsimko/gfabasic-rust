# GFA-BASIC command index — first-pass extraction

Source: supplied *Das Grosse GFA-BASIC-Buch zum Amiga* PDF, Appendix D, “Verzeichnis der GFA-BASIC-Befehle”, printed pages 493–497.

## Scope and caveats

The book itself describes GFA-BASIC 3.0 and says it covers approximately 360 commands and functions. Appendix D is an alphabetical index, not a normative language specification. It mixes commands, functions, constants, variables, object properties and aliases. OCR is imperfect; questionable spellings are retained with a trailing `?`. Page numbers below are index references where legible. This is a candidate inventory, **not** a verified count of unique callable items and **not** a 3.5 compatibility claim. The separate 3.5 addendum remains the only source for the currently scoped 28 declarations.

## Alphabetical entries transcribed from Appendix D

The following names were recovered from the five index pages. Compound forms are kept together pending classification and alias normalization.

### A–C (printed p. 493)

`1`, `$`, `0`, `*`, `ABS()`, `ABSOLUTE {AB}`, `ACOS()`, `ADD {AD}`, `ADD()`, `AFTER CONT {AF CONT}`, `AFTER STOP {AF STOP}`, `AFTER x GOSUB {AF}`, `ALERT {A}`, `AND()`, `ARRAYFILL {ARR}`, `ARRPTR`, `ASC()`, `ASIN()`, `ATN()`, `BACKS`, `BACKW {BA}`, `BCHG()`, `BCLR()`, `BGET {BG}`, `BIN$()`, `BLOAD {BL}`, `BMOVE {B}`, `BOUNDARY {BOU}`, `BOX, PBOX`, `BPUT {BP}`, `BSAVE {BS}`, `BSET()`, `BTST()`, `BYTE()`, `BYTE(), QADR(), LONG()`, `CALL`, `C...` (OCR damaged), `C...` (OCR damaged), `C...` (OCR damaged), `CHAIN {CHAI}`, `CHAR()`, `CHDIR {CHD}`, `CHR$()`, `CINT()`, `CIRCLE, PCIRCLE`, `CLEAR {CLE}`, `CLEARW`, `CLIP`, `CLOSE {C}`, `CLOSE, CLOSES`, `CLOSEW`, `CLR`, `...` (OCR damaged), `COLLISION()`, `COLOR {C}`, `COMBIN()`, `CONT {CON}`, `COS()`, `COSH()`, `CRSCOL`, `CRSLIN`, `CVL(), CVS(), CVO()`, `DATA {D}`, `DATE$`.

### D–L (printed pp. 493–494)

`DEC`, `DEFBIT`, `DEFBYT`, `DEFFILL`, `DEFFLT`, `DEFFN`, `DEFINT`, `DEFLINE`, `DEFLIST`, `DEFMOUSE`, `DEFNUM`, `DEFSTR`, `DEFWRD`, `DEG()`, `DELAY()`, `DELETE {DEL}`, `DFREE()`, `DIM`, `DIM?()`, `DIR`, `DIR$()`, `DIV`, `DIV()`, `DO LOOP`, `DOUBLE{}, SINGLE{}`, `DRAW$ {DR}`, `DRAW {DR}`, `DRAW()`, `DUMP {DU}`, `EDIT {ED}`, `ELLIPSE, PELLIPSE`, `ELSE IF`, `END`, `ENDSELECT`, `EOF()`, `E...` (OCR damaged), `ERASE {ERA}`, `ERR`, `ERR$()`, `ERROR {ER}`, `EVEN()`, `EVERY CONT`, `EVERY STOP`, `EVERY x GOSUB`, `EXEC {EXE}`, `EXIST()`, `EXIT IF`, `EXP()`, `FACT()`, `FALSE`, `FATAL`, `FIELD`, `FILES`, `FILESELECT`, `FILL`, `FIX()`, `FLOAT{}`, `FLIP`, `FOR NEXT`, `FORM INPUT AS`, `FRAC()`, `FRE()`, `FRONTS`, `FRONTW`, `FULLW`, `FUNCTION ... RETURN ... ENDFUNC`, `GET`, `GET #`, `GOSUB`, `GOTO`, `GRAPHMODE`, `HARDCOPY`, `HEX$()`, `HTAB`, `IF [ELSE] ENDIF`, `IMP()`, `INC`, `INKEY$`, `INLINE`, `INP`, `INPUT`, `INPUT #`, `INPUT$()`, `INSERT`, `INSTR()`, `INTO`, `INT{}, JWORD{}`, `KILL {K}`, `LEFT$()`, `LEN()`, `LET {LE}`, `LIMITW`, `LINE INPUT`, `LINE`, `LIST`, `LLIST`, `LOAD`.

### L–O (printed p. 495)

`LOC()`, `LOCAL`, `LOCATE`, `LOF()`, `LOG()`, `LPOS()`, `LPRINT`, `LSET`, `MALLOC()`, `MAT BASE`, `MAT CLR`, `MAT SET`, `MAT ONE`, `MAT READ`, `MAT PRINT`, `MAT INPUT`, `MAT CPY`, `MAT XCPY`, `MAT TRANS`, `MAT ADD`, `MAT SUB`, `MAT MUL`, `MAT NORM`, `MAT DET`, `MAT QDET`, `MAT RANG`, `MAT INV`, `MAX()`, `MENUKEY`, `MENUKILL`, `MENUMENU`, `MENUMENUTEXT$()`, `MENUTEXT`, `MENU(Index)`, `MFREE()`, `MID$()`, `MID$()=`, `MIN()`, `MKDIR`, `MKI$(), MKL$(), MKS$(), MKD$()`, `MOD()`, `MOD`, `MONITOR`, `MOUSE`, `MOUSEX, MOUSEY`, `MOVES`, `MOVEW`, `MUL`, `MUL()`, `NAME`, `NEW`, `OBJECT.* properties` (see note), `OCT$()`, `ODD()`, `ON GOSUB`, `ON BREAK [CONT] [GOSUB]`, `ON COLLISION GOSUB`, `ON MENU`, `ON MENU BUTTON GOSUB`, `ON MENU GOSUB`, `ON MENU KEY GOSUB`, `OPEN`, `OPENS`, `OPENW`, `OPTION BASE`, `OR()`.

Object-property names individually visible in OCR: `OBJECT.AX`, `OBJECT.AX()`, `OBJECT.AY`, `OBJECT.AY()`, `OBJECT.CLIP`, `OBJECT.CLOSE`, `OBJECT.OFF?`, `OBJECT.ON`, `OBJECT.PLANES`, `OBJECT.PRIORITY`, `OBJECT.SHAPE`, `OBJECT.START`, `OBJECT.STOP`, `OBJECT.VX`, `OBJECT.VX()`, `OBJECT.VY`, `OBJECT.VY()`, `OBJECT.X`, `OBJECT.X()`, `OBJECT.Y`, `OBJECT.Y()`. OCR has several damaged spellings in this group; confirm from page images.

### O–S (printed p. 496)

`OR()`, `OUT`, `PAUSE`, `PEEK, DPEEK, LPEEK`, `PI`, `PLOT`, `POINT()`, `POKE, DPOKE, LPOKE`, `POLYFILL`, `POLYLINE`, `POS()`, `PRED()`, `PRED()` (duplicate OCR entry; page reference differs), `PRINT USING`, `PRINT`, `PROCEDURE ... ENDPROC`, `PROCEDURE ... RETURN`, `PSAVE`, `PUT#`, `PUT`, `QSORT`, `QUIT`, `RAD()`, `RAND()`, `RANDOM()`, `RANDOMIZE`, `RCALL`, `READ`, `RECALL`, `RECORD`, `RELSEEK`, `REM`, `RENAME`, `REPEAT ... UNTIL`, `RESET`, `RESTORE`, `RESTORE` (duplicate-looking OCR entry), `RESUME`, `RIGHT$()`, `RINSTR()`, `RMDIR`, `RND()`, `ROL()`, `ROR()`, `ROUND()`, `RSET`, `RUN`, `SAVE`, `SAY`, `SCREENR`, `SCROLL`, `SEEK`, `SELECT`, `SETCOLOR`, `SETDRAW`, `SETSPEN`, `SETTIME`, `SETWPEN`, `SGN()`, `SHL()`, `SHR()`, `SIN()`, `SINH()`, `SIZEW`, `SLEEP`, `SOUND`, `SPACE$()`, `SPC()`, `SPRITE`, `SQR()`, `SSORT`, `STICK`, `STICK()`, `STOP`, `STORE`, `STR$()`, `STRIG()`, `STRING$()`, `SUB`, `SUB ... ENDSUB`, `SUB()`, `SUCC()`, `SUCC()` (duplicate-looking OCR entry), `SWAP`, `SWAP()`.

### S–X (printed p. 497)

`SYSTEM`, `TAB()`, `TAN()`, `TEXT`, `TIMES`, `TIMER`, `TITLES`, `TITLEW`, `TOUCH`, `TRACE$`, `TRANSLATE$()`, `TRIM$()`, `TROFF`, `TRON Proc`, `TRUE`, `TRUNC()`, `TYPE()`, `UPPER$()`, `VAL()`, `VAL?()`, `VAR`, `VARIAT()`, `VARPTR`, `VOID`, `VSYNC`, `VTAB`, `WAVE`, `WHILE WEND`, `WINDOW()`, `WORD()`, `WAIT`, `XOR()`.

## Known 3.5 addendum-only entries

The separately extracted 3.5 interpreter addendum includes these named additions/variants: `FACT()`, `VARIAT()`, `COMBIN()`, negative-screen-number `OPENW`, `PCOLOR`, `_DATA`, `_DATA=`, `_0`–`_9`, `GURU` statement and `GURU()`, and the `MAT` family. These overlap with some names in the 3.0 index (for example FACT, VARIAT and MAT operations). Count them as version-confirmed variants/declarations only after deduplication at declaration level.

## Counting policy for subsequent cleanup

- Preserve this raw candidate list and source page reference.
- Normalize only after checking the page image; retain an `original_index_text` field for every entry.
- Distinguish callable functions from statements, constants, variables, properties and editor/UI operations.
- Keep aliases as aliases of one declaration, not separate API items.
- Track evidence scope separately: `amiga-3.0-book-index`, `amiga-3.5-addendum`, or `unverified`.
- Do not publish the approximate “360” as an exact denominator.
