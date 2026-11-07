# Amiga GFA BASIC 3.0 base-manual inventory

> **Version boundary:** This is a research inventory, not a 3.5 compatibility claim. The supplied *GFA BASIC 3.5 Interpreter* manual says that it was originally printed for version 3.0 and that version 3.5 additions are described in its addendum (Chapter 13 in this scan). The material below comes from base-manual chapters. Keep it outside the normative 3.5 registry until each item is reconciled against a 3.5-specific source or runtime observation.

Primary source: AM-INTERPRETER-35, *GFA BASIC 3.5 Interpreter*, base-manual chapters 2–12. References below are printed page numbers, not PDF viewer pages.

## Chapter inventory from the contents

| Chapter | Subject | Initial printed-page range / topics | 3.5 status |
|---|---|---|---|
| 2 | Variables and memory handling | pp. 2-2–2-44: variable types, arrays, DEFxxx, LET, CLEAR/CLR, ERASE, DIM, OPTION BASE, INSERT/DELETE, ARRAY FILL, conversions, sorting, dates/time, memory access and allocation | Base-manual 3.0; not independently confirmed for 3.5 |
| 3 | Operators | pp. 3-2–3-16: precedence, logical, arithmetic, string and comparison operators | Base-manual 3.0; not independently confirmed for 3.5 |
| 4 | Numeric functions and strings | starts p. 4-2; arithmetic, trigonometry, conversion, numeric and string functions | Base-manual 3.0; not independently confirmed for 3.5 |
| 5 | Input and output | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 6 | Disk handling and file organization | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 7 | Program handling | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 8 | Graphics | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 9 | Event, menu, screen and window handling | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 10 | Advanced programming | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 11 | System routines | Chapter listed in contents | Individual declarations/pages not yet extracted |
| 12 | Appendices | Chapter listed in contents | Tables/index still to be extracted |

## Chapter 2 — variables and memory

The contents list variable types (p. 2-2), arrays (p. 2-4), DEFxxx (p. 2-5), LET (p. 2-6), CLEAR/CLR (p. 2-7), ERASE (p. 2-8), DIM (p. 2-9), OPTION BASE (p. 2-10), INSERT/DELETE (p. 2-11), ARRAY FILL (p. 2-12), conversion and string functions (pp. 2-13–2-18), SWAP (p. 2-20), QSORT/SSORT (p. 2-22), constants (p. 2-25), dates/time (pp. 2-26–2-28), FRE/TYPE/DUMP/VARPTR/ARRPTR, POKE/PEEK, ABSOLUTE, VOID, MALLOC/MFREE, BMOVE and RESERVE (pp. 2-29–2-44).

Page-level excerpts reviewed so far:

- Arrays (printed pp. 2-3–2-4): arrays may use any variable type and are created with DIM; DIM? returns the number of elements. The manual describes a six-byte array descriptor and in-memory layout. These are implementation-relevant representation details, but remain 3.0 source facts.
- DEFxxx (p. 2-5): DEFBIT, DEFBYT, DEFINT, DEFWRD, DEFFLT and DEFSTR define default variable types based on variable-name prefixes.
- LET (p. 2-6): assigns an arithmetic or string expression to a matching numeric or string variable; the manual says LET is normally optional.
- CLEAR/CLR (p. 2-7): CLEAR erases all variables and fields and has documented execution restrictions; CLR clears listed scalar variables and cannot erase arrays.
- ERASE (p. 2-8): erases complete arrays; erased arrays must be dimensioned again before use.
- DIM/DIM? (p. 2-9): dimensions numeric/string arrays; the manual documents multidimensional arrays and a 65,535-element limit on the last dimension.
- OPTION BASE (p. 2-10): controls whether array indexing starts at zero or one. Its interaction with matrix commands is separately documented in the 3.5 addendum and must not be conflated with the base-manual description.

## Chapter 3 — operators

The operator hierarchy is documented on printed p. 3-2. The manual orders parentheses, string operations, prefixes/power, multiplication/division, integer division/modulo, addition/subtraction, comparisons, logical operations and NOT. The scan's OCR corrupts some operator glyphs; do not use OCR text alone to normalize the complete precedence table.

Page-level excerpts reviewed:

- Logical operators (pp. 3-3–3-9): NOT, AND, OR, XOR, IMP and EQV operate bitwise on 32-bit integer values. In logical contexts, zero is false and every nonzero number is true; TRUE is -1. The manual supplies truth tables and examples.
- AND (p. 3-5): true only when both logical operands are true; for numeric operands it operates bitwise.
- OR (p. 3-6): true when at least one operand is true; numeric use is bitwise.
- XOR (p. 3-7): true when exactly one logical operand is true; bitwise use sets bits present in exactly one operand.
- IMP (p. 3-8): implication is false only for true followed by false; operand order matters.
- EQV (p. 3-9): equivalence is true when both logical values match; bitwise behavior is described as the complement of XOR.
- String comparisons (pp. 3-13–3-16): comparison uses character codes in sequence and stops at the first differing character or when one string ends. The manual documents >, <, >=, <=, and <>, including alternate spellings for greater/less-or-equal operators.

The exact precedence and operator token spellings should be checked against page images before adding individual operator declarations. No 3.5 availability is inferred here.

## Chapter 4 — numeric functions and strings

Initial page-level excerpts reviewed:

- ADD/SUB/MUL/DIV commands (pp. 4-2–4-3): mutate the first numeric variable by adding, subtracting, multiplying or dividing by the second expression. The manual warns that integer forms do not perform overflow checking and that fractional operands are treated as integers for integer variables.
- Integer arithmetic functions (p. 4-3): ADD(x,y), SUB(x,y), MUL(x,y), DIV(x,y), and MOD(x,y) are also described as expression functions and may be grouped in Polish notation. Decimal places are ignored by these integer arithmetic functions.
- SIN/COS/TAN and inverse functions (p. 4-5): use radians.
- SINQ/COSQ (p. 4-6): use degrees and an interpolated lookup table; the manual describes interpolation in 1/16-degree steps.
- DEG/RAD (p. 4-7): convert between radians and degrees.
- SQR (p. 4-8): square-root function; a negative argument produces an error.
- EXP/LOG/LOG10 (p. 4-9): exponential, natural logarithm and base-10 logarithm.
- DEC/INC (p. 4-10): decrement/increment a numeric variable; the manual warns of missing integer overflow checking.
- PRED/SUCC (p. 4-11): predecessor/successor for integer expressions; string forms are documented separately.
- ABS/SGN (p. 4-12): absolute value and sign (-1, 0, 1).
- INT/TRUNC/FIX/FRAC (p. 4-13 onward): the manual distinguishes truncation from floor for negative values and documents the relation between TRUNC and FRAC.

The remainder of Chapter 4, including the string-function families and exact syntax/argument forms, still needs systematic page review. The list above is not a complete built-in catalogue.

## Promotion policy

A candidate can enter api/registry/language-core/ for version 3.5 only after one of the following:

1. A 3.5-specific primary source confirms the declaration and its semantics; or
2. A controlled observation on the 3.5 interpreter establishes behavior, with reproducible test conditions and recorded binary/version provenance.

Otherwise retain it as a version-3.0 source fact or an unresolved compatibility candidate. Do not infer 3.5 compatibility merely because a base-manual topic appears in the same PDF as the 3.5 addendum.

## Remaining work

- Extract the full Chapter 2 and 4 alphabetical/index tables.
- Review the page images for all operator spellings and precedence.
- Extract Chapters 5–12 into topic inventories, then select declarations for detailed review.
- Reconcile each candidate against the 3.5 interpreter and, where possible, the supplied German editions without merging their pagination or version claims.
- Keep the public 3.5 catalogue separate from this 3.0 baseline until evidence supports promotion.

## Complete Chapter 4 topic index (base-manual 3.0)

The contents pages enumerate the following additional topics. This list is a discovery index, not proof of detailed syntax or 3.5 availability.

| Printed page | Topic headings |
|---|---|
| 4-2 | ADD / SUB / MUL / DIV / MOD |
| 4-5 | SIN / COS / TAN |
| 4-6 | SINQ / COSQ |
| 4-7 | DEG / RAD |
| 4-8 | SQR |
| 4-9 | EXP / LOG |
| 4-10 | DEC / INC |
| 4-11 | PRED / SUCC |
| 4-12 | ABS / SGN |
| 4-13 | INT / TRUNC / FIX / FRAC |
| 4-14 | ROUND |
| 4-15 | ODD / EVEN |
| 4-16 | MIN / MAX |
| 4-17 | RND / RANDOM |
| 4-19 | BCLR / BSET / BCHG / BTST |
| 4-21 | SHL / SHR |
| 4-22 | ROL / ROR |
| 4-23 | AND / OR / XOR |
| 4-24 | IMP / EQV |
| 4-25 | SWAP |
| 4-26 | BYTE / CARD / WORD |
| 4-27 | LEFT$ / RIGHT$ |
| 4-28 | MID$ |
| 4-30 | INSTR |
| 4-31 | RINSTR |
| 4-32 | STRING$ / SPACE$ / SPC |
| 4-33 | PRED / SUCC (string forms also discussed) |
| 4-34 | LEN / TRIM$ |
| 4-35 | UPPER$ |
| 4-36 | LSET / RSET |

The Chapter 4 index contains 29 topic rows. Several entries are related to Chapter 2 (for example SWAP, conversions and string operations); preserve separate source sections and do not assume the same syntax/semantics merely from repeated headings.

## Remaining chapter contents — discovery index

The supplied manual's contents also identify the following base-manual 3.0 feature groups. Exact command declarations, semantics and page images still need review.

- **Chapter 5, Input and output:** INKEY$, INPUT, LINE INPUT, FORM INPUT, PRINT, PRINT USING, MODE, DEFNUM, CRSxxx/POS/TAB, DATA, MOUSE, LPRINT/LPOS, HARDCOPY, SOUND/WAVE (printed pp. 5-2–5-22).
- **Chapter 6, Disk handling and file organization:** file specification, random access, CHDIR/DIRS/DFREE, DIR/FILES, EXIST, OPEN, LOF/LOC/EOF/CLOSE/TOUCH, NAME/RENAME/KILL, BLOAD/BSAVE/BGET/BPUT, INP/OUT, INPITOO, INPUT #/LINE INPUT, PRINT #/WRITE #, STORE/RECALL, SEEK/RELSEEK, FIELD, GET/PUT/RECORD (printed pp. 6-2–6-21).
- **Chapter 7, Program handling:** loops; procedures/functions; IF/ELSE/ENDIF; ELSE IF; ONGOSUB; SELECT CASE; FOR/NEXT; REPEAT/UNTIL; WHILE/WEND; DO/LOOP; DO WHILE/UNTIL; EXIT IF; GOSUB/PROCEDURE; LOCAL; @func/FUNCTION; DEFFN; ON BREAK; ON ERROR; ERROR/ERR; EVERY/AFTER; REM; GOTO; PAUSE/DELAY; END/EDIT/STOP; NEW; LOAD; SAVE/PSAVE; LIST/LLIST; CHAIN; RUN; SYSTEM/QUIT; TRON/TROFF/TRACE; $, INLINE, C:, MONITOR, CALL and RCALL (printed pp. 7-2–7-55).
- **Chapter 8, Graphics:** SETCOLOR, DEFFILL, BOUNDARY, DEFLINE, GRAPHMODE, RASTPORT, CLIP, PLOT/LINE/DRAW, DRAW/SETDRAW, BOX/PBOX, CIRCLE/ELLIPSE, POLYLINE, POINT, FILL, CLS, TEXT, GET/PUT, VSYNC, DISPLAY (printed pp. 8-2–8-24).
- **Chapter 9, Event/menu/screen/window handling:** event handling, pull-down menus, MENU/MENUO, ON MENU and ON MENU...GOSUB, MENU KILL/MENU X,Y, OPENS/CLOSES, FRONTS/BACKS, MOVES, TITLES, SETSPEN, SCREEN, OPENW, CLEARW/TITLEW/BACKW/FULLW, MOVEW/SIZEW/LIMITW, SETWPEN, WINDOW, ALERT and FILESELECT (printed pp. 9-2–9-37).
- **Chapter 10, Advanced programming:** CLI call/_dosCmd$, EXEC, library variables, SPRITE, STICK/STRIG, SAY/TRANSLATE$, FD-files, blitter objects and OBJECT commands (printed pp. 10-2–10-14).
- **Chapter 11, System routines:** system routine calling plus Exec, Graphics, Workbench, Intuition, DiskFont, Layers and DOS libraries (printed pp. 11-2–11-85). Treat these as platform/OS API surface, not automatically as language-core declarations.
- **Chapter 12, Appendices:** keyboard scan codes, ASCII table, special ASCII codes, fill-in patterns, Amiga-DOS error messages, table of commands and table of operating-system functions (printed pp. 12-2–12-19).

This is the source table-of-contents extraction only. It does not assert that every listed 3.0 item exists unchanged in 3.5. In particular, Chapter 11 must be split into language-level wrappers versus Amiga OS library calls when the detailed inventory is created.

### Further Chapter 4 behavior extracted from the base-manual pages

These are concise source notes for the 3.0 baseline only; exact 3.5 compatibility remains unresolved.

- **ROUND (printed p. 4-14):** ROUND(x) rounds to a whole number; ROUND(x,n) rounds to n decimal places. A negative n rounds before the decimal point (the manual's example ROUND(155,-1) gives 160).
- **ODD/EVEN (p. 4-15):** return -1 for true and 0 for false; zero is even.
- **MIN/MAX (p. 4-16):** accept a list of numeric or string expressions and return the smallest/largest item.
- **RND/RANDOM/RAND/RANDOMIZE (pp. 4-17–4-18):** RND returns a real value in [0,1); RANDOM(x) returns an integer in [0,x); RAND(y) returns a 16-bit integer in [0,y); RANDOMIZE seeds the generator and repeating a seed reproduces the sequence. The manual says RANDOMIZE without an argument or with zero initializes the generator.
- **BCLR/BSET/BCHG/BTST (pp. 4-19–4-20):** bit indices are zero-based and masked to 0–31; operations clear, set, toggle, or test a bit in a 32-bit integer expression. BTST returns 1/0 in this section (note that the separate logical-operator section describes TRUE as -1; retain the page-local behavior and test it before normalizing).
- **SHL/SHR (pp. 4-21–4-22):** shift a numeric expression by a count; default form uses a longword, suffix & selects word length, suffix | selects byte length. The manual documents sign-extension/bit clearing differences for word and byte forms.
- **ROL/ROR (p. 4-22):** rotate bits within the selected longword/word/byte width; unlike shifts, the bit shifted out re-enters at the opposite end.
- **AND/OR/XOR functions (p. 4-23):** bitwise combination of integer expressions; AND keeps bits set in both, OR keeps bits set in either, XOR keeps bits set in exactly one.
- **IMP/EQV functions (p. 4-24):** IMP clears a bit set in x but not y; EQV sets bits whose corresponding input bits match.
- **SWAP function (p. 4-25):** exchanges the upper and lower 16-bit words of a 32-bit expression. The manual explicitly distinguishes this function from the SWAP command described in Chapter 2.
- **BYTE/CARD/WORD (p. 4-26):** BYTE returns the low 8 bits; CARD returns the low 16 bits; WORD extends a 16-bit word to a longword by copying bit 15 into bits 16–31.
- **LEFT$/RIGHT$ (p. 4-27):** return the requested number of characters from the left/right; if the count is omitted, return one character.
- **MID$ function (p. 4-28):** returns a substring from a 1-based position; omitted length returns the remainder; a start beyond the string length returns an empty string.
- **MID$ command (p. 4-29):** overwrites characters in an existing string variable from the requested position; it does not change the target string's length and stops if the target is too short.
- **INSTR (p. 4-30):** searches for a substring, optionally starting at a specified position; returns zero when absent. The manual describes the result as the position after which the substring is contained, so confirm indexing carefully before implementation.
- **RINSTR (p. 4-31):** searches from the rightmost character or from the supplied position.
- **STRING$/SPACE$/SPC (p. 4-32):** STRING$ repeats a string or character code; SPACE$ creates a string of spaces; SPC emits spacing in PRINT rather than creating a string.
- **PRED/SUCC string forms (p. 4-33):** return the previous/next character by ASCII value, using the first character of the input string.
- **LEN/TRIM$ (p. 4-34):** LEN returns string length; TRIM$ removes spaces at both ends.
- **UPPER$ (p. 4-35):** converts lowercase characters to uppercase.
- **LSET/RSET (p. 4-36):** the contents page identifies left/right string assignment; exact padding/truncation semantics still need extraction and page-image review.

The base-manual contents index and page-level descriptions together now provide a candidate inventory for the numeric, bitwise and string families. This is still not a 3.5 claim and should not be imported into the canonical 3.5 API registry without compatibility evidence.

### Further Chapter 2 behavior extracted from the base-manual pages

These notes describe the 3.0 baseline in the supplied manual. They are not evidence that the same behavior is present unchanged in Amiga interpreter 3.5.

- **Variable types (printed pp. 2-2–2-3):** the manual describes Boolean, byte, word, longword integer, 8-byte floating-point, and string variables. It gives postfixes for Boolean/byte/word/longword/string and no postfix for float. Strings are limited to 32,767 characters. The page also documents string and array descriptors and distinguishes VARPTR (element/string data address) from ARRPTR or * (descriptor address).
- **Array descriptors (pp. 2-3–2-4):** a six-byte descriptor stores a four-byte address and two-byte dimension count; dimension bounds precede the element data. The example documents column-major element ordering for a two-dimensional array. Treat this as source-described 3.0 memory layout, not an implementation requirement for the Rust interpreter unless 3.5 compatibility confirms it.
- **DEFxxx (p. 2-5):** DEFBIT, DEFBYT, DEFWRD, DEFINT, DEFFLT and DEFSTR set default types for variable-name prefixes. An explicit postfix takes priority; declarations remain in effect until changed. The default is DEFFLT. The editor replaces DEFSNG/DEFDBL with DEFFLT.
- **OPTION BASE (p. 2-10):** the default is zero if omitted. OPTION BASE 0 permits a zero element; OPTION BASE 1 starts indices at one. The manual says changing the option does not rewrite array contents, but shifts the interpreted position of elements by one.
- **INSERT/DELETE (p. 2-11):** INSERT writes a value at an array index, shifts higher-indexed values upward and discards the last element. DELETE removes the indexed value, shifts later values down, and resets the final element to zero (or an empty string for string arrays).
- **ARRAYFILL (p. 2-12):** fills every element of a numeric array with an arithmetic expression. The assigned value must fit the storage range of the element type; the manual gives a byte/word-range error example.
- **ASC/CHR$ (p. 2-13):** ASC returns the ASCII code of the first character, or zero for an empty string. CHR$ returns a character using only the low byte (equivalent to x AND 255).
- **BIN$/OCT$/HEX$ (p. 2-14):** convert integer expressions to binary, octal or hexadecimal strings; optional width controls the number of output places, with page-specific ranges.
- **STR$ (p. 2-15):** converts a numeric expression to text. Optional width inserts leading spaces/truncates from the left according to the documented RIGHT$(SPACE$(n)+STR$(x),n) relation. A third argument selects a formatted slice/rounding form; the examples round the selected numeric output.
- **VAL/VAL? (p. 2-16):** parse a numeric prefix from a string and stop at the first unrecognized character; return zero if no numeric prefix exists. The manual also lists hexadecimal, binary and octal prefixes. VAL? returns the number of characters consumed (the examples use it for this purpose).
- **CINT/CFLOAT (p. 2-17):** CINT rounds a floating-point expression to an integer; CFLOAT converts an integer expression to floating point. The manual explicitly says these are only meaningful to the compiler, which is an important interpreter/compiler boundary to preserve.
- **CVI/CVL/CVS and MKI$/MKL$/MKS$/MKF$/MKD$ (pp. 2-18–2-19):** convert between numeric values and their internal binary string representations for interchange with other programming languages. The prose/table has naming inconsistencies (CVS versus CVF, and MKS$ versus MKF$); do not normalize the exact floating-point pair until the page image and another edition/runtime are checked.

The Chapter 2 base-manual pages therefore expose several implementation-relevant distinctions: storage type versus expression type, array descriptor versus element address, compiler-only conversions, and binary-string conversion versus ASCII VAL/STR$ conversion. All remain 3.0 candidates pending 3.5 confirmation.


## Chapter 5 — input and output: page-level notes

The following details were checked against the chapter text. They describe the 3.0 base-manual behavior only.

- **INKEY$ (printed p. 5-2):** reads a keyboard character. Modifier/control keys are not returned as ordinary characters; special keys such as function keys return a CSI sequence beginning with CHR$(155), followed by additional characters.
- **INPUT (pp. 5-3–5-4):** reads one or more numeric/string variables, optionally with a prompt. It is window-based and opens standard window 0 if no window is open. A semicolon after the prompt adds "? "; a comma places the cursor immediately after the prompt. Numeric input rejects non-numeric text and requests re-entry.
- **LINE INPUT (p. 5-5):** reads string variables while allowing commas inside the entered text. Editing before confirmation follows INPUT.
- **FORM INPUT / FORM INPUT AS (p. 5-6):** edits a string variable with a declared field width from 1 to 255 characters. FORM INPUT AS displays the variable's current value for editing.
- **PRINT / PRINT AT / WRITE / LOCATE (pp. 5-7–5-8):** PRINT writes to the most recently opened window; PRINT AT positions output at a column/line; LOCATE positions the cursor; WRITE emits comma-separated values in a format intended for reading back with INPUT. The manual distinguishes comma, semicolon and apostrophe separators.
- **PRINT USING (p. 5-9):** formatted screen output; the format string controls numeric/string field presentation. Full placeholder grammar still needs page-image review.
- **MODE, DEFNUM, cursor-positioning and DATA, mouse, printer, hardcopy, SOUND/WAVE:** present in the contents at pp. 5-11–5-22; detailed syntax has not yet been extracted here.

## Chapter 6 — disk and file organization: page-level notes

- **INP/OUT (printed p. 6-13):** INP(#n) reads one byte from an opened data channel; OUT #n writes byte values to an opened channel. The documented channel-number range is 0–99.
- **INPUT$ (p. 6-14):** reads a specified number of characters from the keyboard, or from a file when a channel number is supplied. The manual documents the optional channel argument.
- **INPUT # / LINE INPUT # (p. 6-15):** reads comma-separated individual values/variable lists from an opened data file; LINE INPUT # reads a line into string variable(s).
- **PRINT # / WRITE # (p. 6-16):** writes expressions through a data channel. PRINT # supports a USING format; PRINT # does not support PRINT AT.
- **Chapter boundary:** file-based INPUT/LINE INPUT/PRINT are related to the screen commands in Chapter 5 but have channel-specific semantics. Keep them as distinct declarations when later modelling the API.

The remaining Chapter 6 topics (open modes, file position/status, random-access records, BLOAD/BSAVE and STORE/RECALL) are still only present in the contents-derived index above.

## Chapter 9 — file selector note

- **FILESELECT (printed pp. 9-37–9-38):** displays a file-selector requester. The title and OK-gadget text are supplied as strings; a path string selects the directory/file to display; the output string receives the selected file name, or an empty string when Cancel is selected. The text also describes drive switching and accepting a file path typed into the selector. This is a 3.0 source description, not a verified 3.5 compatibility claim.

## Chapter 10 — hardware sprite and input-device notes

- **SPRITE (printed pp. 10-5–10-6):** defines, frees, moves, enables and disables Amiga hardware sprites. The manual describes eight sprite numbers (0–7), with sprite 0 assigned to the mouse; each sprite is 16 pixels wide and uses a bitmap string whose rows are four bytes. Coordinates are expressed in the lowest screen resolution. SPRITE ON/OFF affects all defined sprites, including the mouse.
- **Sprite colour registers (p. 10-6):** paired sprites share colour registers; the manual lists register groups 17–19, 21–23, 25–27 and 29–31.
- **STICK/STRIG (p. 10-7):** query joystick position and fire-button state for ports 0 and 1. The manual notes that port 0 is shared with mouse/Intuition behavior; querying it can therefore interact with OS input handling.
- **CLI/_dosCmd$, EXEC, library variables, SAY/TRANSLATE$, FD files, blitter objects and OBJECT commands:** are identified by the chapter contents. They need separate page-level extraction. In particular, OBJECT/BOB functionality should be treated as Amiga-specific advanced graphics, not folded into the portable language core.

## Extraction scope and next checks

This pass adds page-level evidence for selected Chapter 5, 6, 9 and 10 topics. Chapters 7, 8 and 11 remain contents-level inventories. Chapter 11 should be split between BASIC-facing wrappers and direct Amiga OS library calls before registry modelling. For all chapters, the printed page references are locators into the supplied manual's 3.0 base text; none of these notes independently establishes 3.5 availability.


## Chapter 7 — program handling: page-level notes

These are 3.0 base-manual descriptions, not confirmed 3.5 grammar.

- **IF / ELSE / ENDIF (printed pp. 7-4–7-5):** IF evaluates a Boolean expression; THEN is optional. The true branch runs between IF and ELSE/ENDIF; the false branch runs between ELSE and ENDIF. The text defines logical true as any nonzero numeric expression.
- **ELSE IF / ELSEIF (pp. 7-6–7-7):** chained conditions form a single conditional structure. The first satisfied branch runs and control continues after ENDIF; ELSE is the fallback. The editor spelling shown in syntax and examples varies between ELSE IF and ELSEIF; preserve this as an alias/spelling question until lexer/editor evidence is checked.
- **ON GOSUB (p. 7-8):** selects the indexed procedure from a list. Decimal places in the selector are ignored; an out-of-range selector calls no procedure. The listed procedures take no parameters, and execution resumes after the ON GOSUB statement.
- **SELECT / CASE / DEFAULT / ENDSELECT (pp. 7-9–7-10):** branches on a numeric expression or a short string constant (maximum four characters). CASE supports a single value, an inclusive TO range, or a comma-separated list. DEFAULT is the fallback; OTHERWISE is documented as an editor synonym replaced by DEFAULT.
- **FOR / NEXT (pp. 7-13–7-14):** the contents execute at least once according to the manual's description. DOWNTO is an alternative to a negative STEP; ENDFOR is an editor replacement for NEXT. The OCR text around termination/step edge cases needs page-image review before precise conformance tests.
- **REPEAT / UNTIL (p. 7-15):** body executes at least once, then UNTIL is tested; a false condition repeats. ENDREPEAT is documented as an editor replacement for UNTIL.
- **WHILE / WEND (p. 7-16):** precondition loop; condition is checked before each body execution. END WHILE is replaced with WEND.
- **DO / LOOP variants (pp. 7-17–7-19):** bare DO/LOOP is endless. DO WHILE/DO UNTIL tests at the beginning; LOOP WHILE/LOOP UNTIL tests at the end. The manual describes nine combinations. ENDDO is an editor replacement for LOOP.
- **EXIT IF (p. 7-20):** exits the active loop when its Boolean condition is satisfied. The manual explicitly notes that, unlike older versions, EXIT IF is also allowed inside IF/ENDIF and SELECT/ENDSELECT structures.
- **Procedures/functions, error/event handlers and program lifecycle:** the chapter index identifies GOSUB/PROCEDURE, LOCAL, FUNCTION/DEFFN, ON BREAK, ON ERROR, ERROR/ERR, EVERY/AFTER, GOTO, PAUSE/DELAY, END/EDIT/STOP, LOAD/SAVE/CHAIN/RUN and system calls. Their detailed parameter and control-flow semantics still need extraction.

## Chapter 8 — graphics: page-level notes

- **SETCOLOR / COLOR (printed p. 8-2):** SETCOLOR sets RGB components in a screen color register (component intensity 0–15), or accepts a packed red*512 + green*16 + blue value. Each open screen has its own palette. COLOR selects foreground, background and fill-outline colors; background behavior is tied to GRAPHMODE 1 in the description.
- **DEFFILL (pp. 8-3–8-4):** configures fill color, style and pattern for PBOX, PCIRCLE, PELLIPSE, POLYFILL and FILL. Styles are empty, filled, dotted and hatched. The alternate form accepts a bit-pattern string; the manual describes width/height constraints and a maximum string length of 257.
- **CLIP (p. 8-9):** restricts graphics output to a rectangular region, with forms using origin plus width/height, opposite corners, or a window number. OFFSET adjusts the origin; CLIP OFF disables clipping. The manual explicitly excludes GET and PUT from this clipping behavior.
- **PLOT / LINE / DRAW (pp. 8-10–8-14):** PLOT writes one point; LINE draws between two coordinate pairs; DRAW supports relative/absolute path-style movement commands. DRAW also exposes inquiry functions for current position, angle, scaling and pen state. OCR-sensitive DRAW command strings should be checked against page images before grammar work.
- **BOX / PBOX (p. 8-15):** draw an outlined or filled rectangle from two opposite corners.
- **CIRCLE / PCIRCLE / ELLIPSE / PELLIPSE (p. 8-16):** draw outlined or filled circles/ellipses using center coordinates and radii.
- **POLYLINE / POLYFILL (p. 8-17):** use coordinate arrays for polygon corners; first and last points are automatically connected. Optional OFFSET shifts the coordinates; POLYFILL uses the DEFFILL pattern.
- **POINT (p. 8-18):** returns the color value at a specified screen coordinate.
- **Graphics API boundary:** the later RASTPORT section (p. 8-8) permits selecting a raw Amiga RastPort address and explicitly warns that incorrect setup can crash the system. This is a platform/unsafe memory boundary, not ordinary portable drawing syntax.

## Chapter 11 — system routines: classification note

Chapter 11 is a catalogue of direct Amiga operating-system library calls, not a portable BASIC language-core feature list. Its introduction (printed pp. 11-1–11-2) says the interpreter assists with syntax/indentation for system calls, but invalid calls are left to operating-system error handling (the manual names the GURU error). The chapter is organized by Exec, Graphics, Workbench, Intuition, DiskFont, Layers and DOS libraries (contents pp. 11-2–11-85). Individual routines should be modelled under an Amiga platform API profile, with library, call signature, pointer/structure requirements and OS-version constraints. The contents list alone is not enough to claim a stable or safe Rust wrapper.


## Chapter 11 — Amiga OS system-call catalogue

The table of contents and library sections establish the following API families (printed pages):

| Library family | Manual pages | Classification |
|---|---:|---|
| Exec | 11-3–11-23 | Tasking, memory, signals, devices, lists and low-level system services |
| Graphics | 11-24–11-53 | RastPorts, bitmaps, drawing, sprites, copper and display structures |
| Workbench | 11-54–11-57 | Workbench objects, icons, tool types and startup integration |
| Intuition | 11-58–11-75 | Screens, windows, gadgets, menus, input and requesters |
| DiskFont | 11-76–11-77 | Font discovery and loading |
| Layers | 11-78–11-84 | Layer creation, locking, refresh and ordering |
| DOS | 11-85–11-92 | Files, directories, locks, process/CLI and I/O |

The appendix's Table of Libraries (printed pp. 12-19–12-28) is an alphabetical cross-reference of these calls and their Chapter 11 page locations. It is not a separate API specification.

### Consequences for a Rust-facing catalogue

- Preserve the library identity with each call; do not flatten same-named calls into one global function namespace.
- Record the exact AmigaOS library, routine name, parameter order, return convention and source page.
- Treat addresses as typed pointers to specific OS structures where the source identifies them; the manual's BASIC-side transport type is not a safe Rust type.
- Mark ownership/lifetime and allocation/free pairs explicitly. The manual's brief descriptions do not always provide enough information to infer them.
- Separate calls that manipulate global OS state, tasks, interrupts, hardware registers or supervisor mode from ordinary memory-safe wrappers. A direct call can crash the system or corrupt another task.
- The current source is a 3.0-era OS-call reference. Exact OS release/ROM compatibility and 3.5 interpreter exposure still require independent verification.

The following library-specific material was found in the supplied scan: Exec pages include task, signal, memory and device operations; Graphics pages include drawing/bitmap and display structures; Workbench pages include object/icon operations; Intuition pages include window/screen/input management; DiskFont covers font operations; Layers covers layer lifecycle and locking; DOS covers file and CLI operations. This is a family-level inventory only; routine-by-routine extraction remains open.

## Chapter 12 — appendices: reference material, not new language features

Chapter 12 contains support tables and indexes rather than a new command family:

- Keyboard scan codes (printed p. 12-2).
- ASCII table and special ASCII codes (pp. 12-3–12-5).
- Fill patterns (p. 12-6).
- AmigaDOS error messages (p. 12-7).
- Alphabetical command/function table (pp. 12-8–12-18).
- Alphabetical operating-system function table (pp. 12-19–12-28).

The two alphabetical tables are navigation aids into Chapters 2–11; they must not be counted as independent declarations. The character, scan-code and fill-pattern tables are supporting constants/data and should be transcribed only where a concrete implementation or conformance test needs them. The DOS error list can seed an error-code mapping, but code names and numeric values require page-level extraction and OS-version qualification.

## Chapter 13 — verified 3.5 addendum index and reconciliation candidates

Unlike Chapters 2–12, Chapter 13 explicitly identifies itself as the GFA BASIC 3.5 addendum. The table of contents lists the following 3.5-specific topics:

- Editor enhancements: backward scrolling, search in folded procedure/function headers, listing labels, and tab-key functions (printed p. 13-1).
- Floating-point vector/matrix operations: MAT BASE, MAT CLR, MAT SET, MAT ONE, MAT READ, MAT PRINT, MAT INPUT, MAT CPY, MAT XCPY, MAT TRANS and MAT COPY (pp. 13-1–13-6).
- Matrix operations: MAT ADD, MAT SUB, MAT MUL, MAT NORM, MAT DET, MAT QDET, MAT RANG and MAT INV (from p. 13-16).
- FACT, VARIAT and COMBIN (p. 13-33).
- OPENW negative-screen-number variant, PCOLOUR, DATA pointer inspection/assignment via DATA and _DATA=, special variables _0…_9, and GURU handling (pp. 13-34–13-36).

The first matrix pages state that the addendum's matrix/vector functions operate only on one- and two-dimensional fields with floating-point variables. MAT BASE changes the row/column indexing offset for specified matrix commands; the documented default is 1, and MAT BASE 0/1 is intended in conjunction with OPTION BASE 0. MAT CLR and MAT SET fill all elements; MAT ONE creates a square identity matrix. MAT READ consumes DATA rows and MAT PRINT formats vectors/matrices. These are directly supported by the 3.5 addendum and can be reconciled into 3.5 declarations after exact syntax and remaining operation semantics are extracted.

Important boundary: the addendum's presence of an item is positive evidence for that 3.5 feature; absence from Chapter 13 is not proof that a 3.0 feature was removed. The older chapters still require runtime or 3.5-specific confirmation before claiming compatibility.


## Chapter 11 — selected routine-level extraction

The following signatures and behavior are transcribed from the supplied 3.0-era system-routine chapter. They are a first subset, not a complete binding specification.

### Exec-Library (printed pp. 11-5–11-7, 11-9)

- **AllocMem(byteSize, art) (p. 11-6):** allocates a memory block. Returns its address, or 0 if unavailable. The requested size is rounded up to a multiple of 8 bytes. The second argument selects memory attributes (the text gives MEMF_CHIP as an example).
- **AvailMem(art) (p. 11-6):** returns the currently free memory for the requested memory attributes.
- **Allocate(memheader, byteSize) (p. 11-5):** allocates from a task's own free-memory list. Returns the block address, or 0 on failure.
- **Deallocate(memHeader, memBlock, byteSize) (p. 11-7):** returns a block to the task's own free-memory pool. The manual states no return value.
- **AllocSignal(signalNum) (p. 11-6):** allocates a signal bit from the current task's pool; returns bit number 0–31 or -1. The input may be 0–31 or -1 to request any available bit.
- **CloseDevice(IO_request) (p. 11-7):** releases the device association for the task using an initialized I/O request structure. No return value.
- **DoIO(IO_request) (p. 11-9):** listed in the Exec API index and section. Its full signature/semantics should be captured from the page before exposing it in a typed wrapper.
- **Forbid() / Permit() (p. 11-11):** disable and re-enable Amiga task switching, respectively. These calls affect system-wide scheduling behavior and must not be wrapped as ordinary safe operations.
- **SuperState() / UserState(stack) (pp. 11-21–11-22):** switch the 68000 CPU into supervisor mode and back to user mode. The stack value returned by SuperState must be preserved for UserState. This is privileged, system-sensitive behavior.
- **FreeMem(memBlock, byteSize) (p. 11-11):** the routine is listed in the chapter index; the supplied scan's detailed description is to be checked before encoding the exact argument order and memory-attribute constraints.

### Graphics-Library (printed pp. 11-32–11-45)

- **Draw(rastport, x, y) (p. 11-33):** draws from the current drawing-pen position to (x,y) in the specified RastPort.
- **DrawCircle(rastport, x, y, radius) (p. 11-33):** draws a circle using the RastPort's drawing mode and colors.
- **DrawEllipse(rastport, x, y, x_radius, y_radius) (p. 11-33):** draws an ellipse in the specified RastPort.
- **ReadPixel(rastport, x, y) (p. 11-44):** returns the color-register number of the pixel at the given coordinates.
- **RectFill(rastport, x1, y1, x2, y2) (p. 11-44):** fills a rectangle using the current fill pattern, drawing mode and colors.
- **InitRastPort(rastport) (p. 11-39):** initializes a RastPort structure by clearing its fields and setting documented drawing defaults. No return value.
- **Move(rastport, x, y) (p. 11-42):** positions the graphics pen at new coordinates in the output RastPort. No return value.
- **LoadRGB4(viewport, colors, count) (p. 11-40):** assigns a set of RGB color-register values to a ViewPort. No return value.

### Intuition-Library (printed pp. 11-66–11-68)

- **OpenScreen(newscreen) (p. 11-68):** opens/displays a screen described by a NewScreen structure. Returns the Screen structure address or 0.
- **OpenWindow(newwindow) (p. 11-68):** opens a window described by a NewWindow structure. Returns the Window structure address.
- **ModifyIDCMP(window, flag) (p. 11-66):** changes the IDCMP event flags for a window. No return value.
- **MoveScreen(screen, deltax, deltay) (p. 11-67):** moves a screen by relative offsets. No return value.
- **MoveWindow(window, deltax, deltay) (p. 11-67):** moves a window by relative offsets. No return value.

### DOS-Library (printed pp. 11-88–11-91)

- **Lock(name, mode) (p. 11-88):** obtains a lock for a file or directory. ACCESS_READ (-2) allows shared read access; ACCESS_WRITE (-1) obtains a write lock. Returns a Lock structure address.
- **Open(name, mode) (p. 11-89):** opens a file and returns a file handle. MODE_OLDFILE (1005) opens an existing file; MODE_NEWFILE (1006) creates a file for writing and overwrites an existing file of the same name.
- **Close(filehandle) (p. 11-90):** closes an input or output file. The handle is the FileHandle structure address.
- **Read(filehandle, adr, count) (p. 11-90):** reads into a memory buffer and returns the number of bytes read.
- **Seek(filehandle, pos, mode) (pp. 11-90–11-91):** positions a file pointer and returns its old position. Modes documented are OFFSET_BEGINNING (1), OFFSET_END (-1) and OFFSET_CURRENT (0).
- **Rename(old, new) (p. 11-90):** renames a file or directory and returns TRUE/FALSE.
- **UnLock(lock) (p. 11-91):** releases a file/directory lock. No return value.

### Binding boundary

These routines transport addresses through the manual's generic BASIC numeric parameter notation. That notation does not define Rust pointer types, ownership, alignment, buffer length validity or lifetime. A future Rust layer must model each OS structure and allocation contract explicitly. The selected signatures above are documentation candidates only; they are not proof that the current interpreter exposes or safely validates every call.

## Chapter 13 — matrix/vector operation semantics (3.5 addendum)

The following details are explicitly in the 3.5 addendum, unlike the base-manual inventory above.

### Matrix/vector input and copying

- **MAT INPUT #i, a() (printed p. 13-4):** reads a matrix/vector from an ASCII file. Its format is the reverse of MAT PRINT; commas and row feeds may vary as with INPUT #.
- **MAT CPY a([i,j])=b([k,l])[,h,w] (pp. 13-5 onward):** copies a rectangular region of h rows and w elements from source matrix b to destination a. The source and destination start offsets are supplied separately. Only elements with corresponding source/destination indices are copied.
- **MAT COPY a()=b() (pp. 13-6, 13-10):** copies the complete matrix when dimensions are compatible; the text also describes copying only elements for which source and destination indices correspond. The addendum's explanatory paragraph contains an apparent wording/format inconsistency about region dimensions, so retain the examples and syntax as evidence and do not infer a more general resizing rule.
- **MAT XCPY a([i,j])=b([k,l])[,h,w] (pp. 13-11–13-12):** like MAT CPY, but transposes the source while copying; source matrix b remains unchanged.
- **Vector special cases (p. 13-12):** one-dimensional arrays are treated as row vectors. For column-vector treatment, the manual requires a two-dimensional declaration with one column. It states that MAT CPY should be used when both vectors have the same orientation, and gives a column-vector form for the operation.
- **MAT TRANS a()[=b()] (p. 13-15):** transposes a square matrix in place when no destination is supplied; the source text says the original matrix is lost in that form. The two-matrix form is listed in the syntax, but its detailed semantics need a separate page check.

### Arithmetic and linear algebra

- **MAT ADD a()=b()+c() (pp. 13-16 onward):** element-wise addition; source matrices/vectors must have the same order and result is written to a().
- **MAT ADD a(),b() / MAT ADD a(),x:** the addendum lists these in-place/constant variants; exact edge cases and rounding are not yet transcribed.
- **MAT SUB a()=b()-c(), MAT SUB a(),b(), MAT SUB a(),x:** corresponding subtraction forms listed by the addendum.
- **MAT MUL a()=b()*c() / MAT MUL x=a()*b() / MAT MUL x=a()*b()*c() / MAT MUL a(),x:** matrix product and scalar-result variants are listed; the manual examples include matrix×matrix and row-vector×matrix×column-vector calculations.
- **MAT NORM a(),0 / MAT NORM a(),1 (pp. 13-23–13-24):** defined for matrices and vectors. 0 normalizes by rows and 1 by columns; the sum of squares of the elements in each normalized row/column is 1.
- **MAT DET x=a([i,j])[,n] (pp. 13-25 onward):** computes the determinant of a square matrix; optional offsets and dimension select a submatrix.
- **MAT QDET x=a([i,j])[,n]:** computes the same determinant using a speed-optimized routine; the addendum warns that small accuracy differences may occur.
- **MAT RANG x=a([i,j])[,n] (p. 13-29):** computes the rank of a square matrix, optionally for a selected submatrix.
- **MAT INV a()=b() (p. 13-29):** computes the inverse of square matrix b() into a(); the destination must have the same dimensions/type as the source.

The exact page-level descriptions for MAT DET/QDET/RANG/INV should be checked in the scan before implementing numerical behavior. In particular, the source's use of the term rank and its submatrix parameters should be preserved rather than silently replaced with a textbook API interpretation.


## Follow-up: corrected and expanded details from the supplied scans

### Chapter 11 — additional DOS and Exec observations

- **DeleteFile(name) (printed p. 11-86):** deletes a file or an empty directory; returns TRUE/FALSE. IoErr can provide an error number.
- **Examine(lock, fib) (p. 11-86):** writes file/directory metadata from a Lock into a FileInfoBlock, including name, size, creation date and entry type. Returns -1 on success; otherwise IoErr provides the error.
- **ExNext(lock, fib) (p. 11-87):** enumerates subsequent directory entries. Examine supplies the first entry; ExNext is called once for each following entry. Returns -1 on successful completion.
- **Execute(instruction, in, out) (p. 11-87):** executes a CLI command string using input/output file handles. The manual states that the C directory on the boot disk must contain the CLI Run command.
- **Exit(ret) (p. 11-87):** terminates a program/process and returns the supplied value to CLI when started from CLI. The text says a Workbench-started process is ended and its memory/stack freed.
- **CreateDir(name) (p. 11-85):** creates a directory in the current directory. The description says a shared read lock is created on success; the text's return-value wording is internally inconsistent, so do not infer a stable return contract without checking the page image.
- **Delay(time) (p. 11-86):** suspends the current process for a time expressed in 1/50-second units.
- **FreeMem:** the English scan index identifies the routine on p. 11-11, but the companion German reference expresses the call as returning a size from an address and size pair. The exact English manual entry should be checked before treating its return type or signature as settled.
- The current scan search did not establish complete OpenDevice/DoIO/SendIO signatures. These remain unresolved; do not infer them from AmigaOS SDK knowledge.

### Chapter 13 — in-place matrix operations

The detailed MAT ADD text confirms the distinction between result and in-place forms:

- **MAT ADD a(),b() (printed p. 13-17):** requires matching matrix/vector order, adds b element-by-element into a, and overwrites a.
- **MAT ADD a(),x:** adds scalar x to every element of a and overwrites a.
- The addendum lists matching in-place forms for MAT SUB and scalar subtraction, but their full paragraphs still need extraction.
- The addendum also lists MAT MUL a(),x; the exact scalar multiplication/division interpretation should be read from the following page before implementation.

These observations refine the earlier summary: MAT ADD a(),b() and MAT ADD a(),x are explicitly in-place, not merely generic variants. The source's own examples should be the basis of conformance tests.


## Chapter 13 — completed MAT SUB/MUL and determinant details

Further page-level extraction from the 3.5 addendum resolves most of the previously open operation semantics.

### MAT SUB (printed pp. 13-18–13-19)

- **MAT SUB a()=b()-c():** b and c must have the same matrix/vector order; c is subtracted element-by-element from b and the result is written to a.
- **MAT SUB a(),b():** b is subtracted element-by-element from a; a is overwritten and its original contents are lost. Matching dimensions are required.
- **MAT SUB a(),x:** scalar x is subtracted from each element of a, in place. The page's prose has an OCR/typographical inconsistency ("matrix x"), but its example starts with a filled with 6, subtracts 5, and produces 1 in every element. The example therefore supports scalar subtraction from a; preserve the textual discrepancy in source review rather than treating the typo as a separate semantic rule.

### MAT MUL (printed pp. 13-20–13-22)

- **MAT MUL a()=b()*c():** conventional row-by-column matrix multiplication. The number of columns in b must equal the number of rows in c; a must have the row count of b and column count of c. For two vectors, the result is their dyadic/external product, written to matrix a.
- **MAT MUL x=a()*b():** defined for vectors with equal element counts. Returns their scalar/inner product, the sum of corresponding element products.
- **MAT MUL x=a()*b()*c():** defined for vector a, matrix b and vector c. The example computes a row-vector × matrix × column-vector scalar result.
- **MAT MUL a(),x:** this form appears in the operation-command syntax list (p. 13-16), but the extracted explanatory pages do not establish its semantics. Do not assume it means scalar multiplication; retain it as an unresolved syntax candidate pending page-image inspection or runtime observation.

### MAT DET / MAT QDET / MAT RANG / MAT INV (printed pp. 13-25–13-29)

- **MAT DET x=a([i,j])[,n]:** computes a determinant for a square matrix. With no explicit offsets, the origin follows MAT BASE (0,0 or 1,1); with i,j,n, the operation uses an n×n submatrix beginning at row i, column j.
- **MAT QDET x=a([i,j])[,n]:** same selection and result concept as MAT DET, optimized for speed rather than accuracy. The manual says the results are normally identical, but recommends MAT DET for critical matrices whose determinant is close to zero.
- **MAT RANG x=a([i,j])[,n]:** returns the rank of a square matrix, with the same optional submatrix selection convention. The manual's example reports rank 5 for the full matrix and rank 2 for a 3×3 submatrix starting at a(1,2).
- **MAT INV b()=a():** computes the inverse of square matrix a into b; b must be the same matrix type/dimensions as a. The addendum demonstrates checking the result by multiplying b×a to obtain the unity matrix.

This resolves the previously open MAT SUB and the documented matrix/vector product forms. It does not resolve the separate MAT MUL a(),x syntax.


## Chapter 11 — Exec device I/O and message lifecycle (printed pp. 11-7–11-18)

The scan search now resolves several earlier gaps in the Exec subset:

- **FreeMem(adr, count) (p. 11-11):** frees a block back to the system free-memory list and returns the size of the memory block in bytes. Parameters are the block address and its byte size. This settles the previously unresolved return value.
- **OpenDevice(devname, number, IORequest, flags) (p. 11-14):** opens a named device and initializes the supplied I/O request structure. Device number is documented as 0–4. Returns 0 on success, otherwise an error number.
- **CloseDevice(IORequest) (p. 11-7):** closes the device association represented by the initialized I/O request.
- **CreateExtIO(replyport, size) (p. 11-8):** allocates and initializes an extended I/O request; returns its address. The replyport must be an allocated and initialized MsgPort. The size argument is described as the amount of memory to reserve for the MsgPort structure.
- **CreateStdIO(replyport) (p. 11-8):** allocates and initializes an IOStdReq; returns its address. The replyport must be an initialized message port.
- **DoIO(IORequest) (p. 11-9):** asks the device driver to perform the command described by the initialized I/O request. Returns 0 on success, otherwise an error number.
- **SendIO(IORequest) (p. 11-18):** asks the device driver to start the command described by an initialized I/O request. No return value is documented in the entry.
- **AbortIO(IORequest) (p. 11-2):** attempts to abort the last I/O request; no return value is documented.
- **GetMsg(MsgPort) (p. 11-11):** obtains the next message from a message port. The received message should subsequently be returned with ReplyMsg.
- **ReplyMsg(message) (p. 11-18):** sends a processed message back; no return value.
- **PutMsg(msgport, message) (p. 11-15):** appends a message to the specified message-port queue; no return value.

### I/O and message safety notes

The manual describes the request structures and lifecycle but does not specify Rust ownership, cancellation races, asynchronous completion guarantees, or buffer lifetime rules. A safe wrapper must not allow an I/O request, its message port, or any associated data buffer to be freed while the device can still access it. This is an implementation constraint inferred from the documented allocate/open/start/abort/close lifecycle, not a Rust API contract stated by the manual.

The earlier FreeMem note is superseded: the English Chapter 11 scan does document FreeMem(adr,count) and its byte-count return value on p. 11-11.


## Chapter 11 — Exec task, signal, and system-list routines (printed pp. 11-10–11-22)

Further extraction from the English GFA-BASIC 3.0 base manual documents these Exec-Library calls. These are 3.0 reference findings, not evidence that the current Rust interpreter implements them.

### Lookup and task control

- **FindName(start, name) (p. 11-10):** searches a system node list for a node with the given name; returns the node address or 0. `start` is a node/list address; `name` points to a zero-terminated string.
- **FindPort(name) (p. 11-10):** searches the system message-port list; returns the MsgPort address or 0. The name is a pointer to a zero-terminated string.
- **FindResident(name) (p. 11-10):** attempts to locate a resident structure; returns its address or 0. The name is a pointer to a zero-terminated string.
- **FindTask(name) (p. 11-10):** searches task lists and returns the TaskControlBlock address. An empty name string requests the current task's control block.
- **SetTaskPri(task, pri) (p. 11-20):** changes a task's priority and returns its previous priority. The documented range is -128 through 127.
- **Wait(signal) (p. 11-22):** suspends the current task until one or more signals arrive. The parameter is a signal bit/mask as described by the manual.
- **Signal(task, signal) (p. 11-21):** sends the specified signal bit(s) to a task; no return value is documented.
- **SetSignal(newsig, sigmask) (p. 11-20):** sets the task's received-signal state and returns the old signal bit/state. Both arguments are integer expressions; the mask is 32-bit.
- **SetExcept(new, mask) (p. 11-19):** selects which of the 32 signal bits cause exceptions and returns the old signal bit/state. The manual labels both arguments as integer expressions.
- **SetIntVector(number, interrupt) (p. 11-20):** installs code/data pointers for an interrupt and returns the address of the previous Interrupt structure. The interrupt number is a PAULA bit number from 0 to 14; the second argument is the new Interrupt structure address.

### I/O completion and message-port waiting

- **WaitIO(IORequest) (p. 11-22):** waits for the specified I/O request to finish; returns 0 or an error number. The argument is an IORequest structure address.
- **WaitPort(msgport) (p. 11-23):** waits for an event from the specified message port. The entry describes the argument as a message-structure address despite naming it `msgport`; preserve this source wording as an ambiguity rather than silently correcting its type.

### Source and implementation boundaries

- The English manual's Find* entries explicitly require zero-terminated name strings and return raw structure addresses. A Rust binding therefore needs a separately defined string-buffer and pointer-lifetime contract; the BASIC `iexp` annotation alone does not supply one.
- Signal and interrupt calls operate on task/OS state. Treat them as unsafe platform interfaces until the interpreter's calling convention, synchronization, and interrupt-context rules are independently established.
- The companion German 3.5 book contains additional explanatory material (including AllocSignal/AllocTrap), but those details are intentionally not used to expand this 3.0 English-base inventory. Keep 3.5 evidence separately version-scoped.


## Chapter 11 — Exec allocation, interrupt masking, and list primitives (printed pp. 11-5–11-12)

The English base-manual scan also supports the following additional Exec entries:

- **AllocSignal(signalNum) (p. 11-5):** allocates a signal bit from the current task's available signal pool. Returns a bit number 0–31 or -1. The requested bit is 0–31, or -1 to request any available bit.
- **FreeSignal(sigbit) (p. 11-12):** releases a previously allocated signal bit; the documented range is 0–31. No return value.
- **AllocTrap(trapNum) (p. 11-5):** allocates a trap number from the current task's available trap numbers. The requested trap number is 0–15, or -1 for any available trap. The entry does not state an explicit return-value contract in the extracted text.
- **FreeTrap(trapnr) (p. 11-12):** releases an allocated trap number for reuse in the task; range 0–15. No return value.
- **Disable() (p. 11-8):** described as similar to Forbid, but additionally disables interrupts. No return value is documented.
- **Enable() (p. 11-9):** activates interrupts after a Disable. No return value is documented.
- **Enqueue(list, node) (p. 11-9):** inserts/appends a node to a system list; no return value. Both arguments are addresses of the list and node structures.

### Safety and evidence notes

- AllocSignal/FreeSignal and AllocTrap/FreeTrap are task-scoped resource pairs in the manual. Their pairing and ranges should be reflected in any wrapper; do not treat the returned integer as a generic application identifier.
- Disable/Enable affects interrupt delivery, not merely task scheduling. It must not be exposed as an ordinary safe concurrency primitive without a platform-specific critical-section and restoration contract.
- These declarations are sourced from Chapters 11 pages of the English manual labelled GFA-BASIC 3.0 Amiga. They are base-manual candidates only; no 3.5 support claim is made here.


## Chapter 11 — Graphics-Library drawing, raster, and palette calls (printed pp. 11-30–11-49)

Additional Graphics-Library entries extracted from the English GFA-BASIC 3.0 base manual:

- **ClearEOL(rastport) (p. 11-30):** erases a text rectangle in the specified RastPort.
- **ClearRectRegion(region, rectangle) (p. 11-30):** clears areas of a Region that lie inside a clipping Rectangle; returns TRUE/FALSE.
- **ClearRegion(region) (p. 11-30):** clears all clipping rectangles belonging to a Region.
- **ClipBlit(srcRastPort, srcX, srcY, destRastPort, destX, destY, sizeX, sizeY, minTerm) (pp. 11-30–11-31):** copies a rectangular RastPort bitmap area to another RastPort using the specified logical blitter operand.
- **CopySBitMap(layer) (p. 11-31):** copies a Superbitmap into a Layer bitmap; the Layer must first be defined as a Superbitmap.
- **CWait(clist, xbeam, ybeam) (p. 11-31):** adds a Copper instruction, waits for the video beam position, and performs the instruction.
- **DisownBlitter() (p. 11-31):** releases the blitter for other tasks.
- **DisposeRegion(region) (p. 11-32):** frees a Region and its associated Rectangle structures.
- **DoCollision(rastport) (p. 11-32):** tests graphic elements for collisions.
- **DrawGList(rastport, viewport) (p. 11-33):** draws sprites and Bobs in the current graphic-elements list and associates virtual-sprite Copper instructions with the ViewPort.
- **Flood(rastport, mode, x, y) (p. 11-34):** fills an area using the current fill pattern and foreground/background colors. Mode 1 fills the area matching the start-point color; mode 0 fills pixels not matching the current AOlPen color (the source's spelling is retained).
- **FreeColorMap(colormap) (p. 11-34), FreeCopList(coplist) and FreeCprList(cprlist) (p. 11-35):** release the corresponding ColorMap/CopList/CprList allocations.
- **FreeGBuffers(animob, rastport, buf) (p. 11-35):** releases animation-object buffers previously allocated by GetGBuffers.
- **FreeSprite(spritenr) (p. 11-35):** frees a hardware sprite for other tasks; sprite number 0–7.
- **FreeVPortCopLists(viewport) (p. 11-35):** deallocates Copper-list memory associated with a ViewPort.
- **GetColorMap(count) (p. 11-35):** allocates and initializes a ColorMap; returns its address. `count` is the number of color registers.
- **GetGBuffers(animob, rastport, buffer) (p. 11-35):** allocates buffers for an AnimOb; returns TRUE/FALSE. The buffer argument selects double buffering when TRUE.
- **GetRGB4(cmap, regnr) (p. 11-36):** returns a ColorMap color-table entry; register number 0–63.
- **GetScreenData(buffer, size, type, screen) (p. 11-36):** copies all or part of a Screen structure into a caller buffer; returns TRUE/FALSE. Types are WBENCHSCREEN (1) or CUSTOMSCREEN (15); the screen argument is ignored for type 1.
- **InitRastPort(rastport) (p. 11-39):** zeroes RastPort fields, with FgPen, AOlPen and LinePtn set to -1; JAM2 drawing mode and font depend on Topaz 80/60 resolution. No return value.
- **InitTmpRas(tmpras, buffer, size) (p. 11-39):** initializes a chip-memory block for Area functions; the block must be allocated. No return value.
- **InitView(view) and InitVPort(viewport) (p. 11-39):** initialize View/ViewPort structures with defaults or cleared fields; no return value.
- **InstallClipRegion(layer, region) (p. 11-39):** installs a clipping Region in a Layer; returns the last Region address or 0.
- **LoadRGB4(viewport, colors, count) (p. 11-40):** loads a color-register table into a ViewPort.
- **LoadView(view) (p. 11-40):** makes the View's Copper instruction list the current display view.
- **MakeVPort(view, viewport) (p. 11-40):** creates ViewPort data for a View; the entry notes the second RasInfo requirement when DUALPF is set.
- **Move(rastport, x, y) (p. 11-41):** positions the RastPort drawing pen.
- **RasSize(w, h) (p. 11-44):** returns the byte size of a raster bitplane for the given width and height.
- **ReadPixel(rastport, x, y) (p. 11-44):** returns the color-register number at the specified pixel.
- **RectFill(rastport, x1, y1, x2, y2) (p. 11-44):** fills a rectangle with the current fill pattern, drawing mode and colors.
- **ScrollRaster(rastport, deltax, deltay, x, y, w, h) (p. 11-45):** scrolls a RastPort area; the manual describes x/y as the upper-left and w/h as lower-right coordinates.
- **ScrollVPort(viewport, deltax, deltay) (p. 11-45):** scrolls a Screen by changing ViewPort coordinates relative to their previous values.
- **SetAPen/SetBPen(rastport, nr) (p. 11-46):** set foreground/background color-register numbers.
- **SetCollision(number, routine, geisinfo) (p. 11-47):** installs a user collision handler at function number 0–15, with routine and Geisinfo addresses.
- **SetDrMd(rastport, mode), SetDrPt(rastport, data) (pp. 11-47–11-48):** set drawing mode and 16-bit line pattern.
- **SetFont(rastport, textfont) (p. 11-48):** applies a TextFont; returns 0 if no error.
- **SetOPen(rastport, register) and SetRast(rastport, register) (pp. 11-48–11-49):** set outline pen or fill the entire raster with a color register.
- **SetRGB4(viewport, register, red, green, blue) and SetRGB4CM(colormap, register, red, green, blue) (p. 11-49):** store color values; register ranges are 0–31 and RGB intensity components 0–15.
- **SetSoftStyle(rastport, new, mask) (p. 11-49):** changes font style bits selected by the mask; returns the mask value according to the entry.

### Graphics API safety and source caveats

- Most calls take addresses of mutable OS structures and/or caller-owned buffers. The manual does not define aliasing, allocation provenance, or lifetimes; a Rust interface must not infer these from BASIC's `iexp` type.
- Copper, blitter, sprite, and display-view operations interact with shared hardware/display state. Keep them behind an explicit Amiga platform boundary rather than treating them as portable drawing functions.
- The source uses inconsistent or OCR-sensitive names (for example AOlPen, CWait, FreeCprList); preserve source spellings in evidence notes and normalize only after checking page images.
