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
