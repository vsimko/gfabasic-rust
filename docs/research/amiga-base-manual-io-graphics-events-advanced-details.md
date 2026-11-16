---
type: "Research Note"
title: "Amiga GFA BASIC base-manual I/O, graphics, event, and advanced-programming details"
---

# Base-manual I/O, graphics, event, and advanced-programming details

## Scope and evidence boundary

This note expands the heading inventory for Chapters 5, 6, 8, 9, and 10 of the supplied *GFA BASIC 3.5 Interpreter* PDF. The book identifies the main text as the GFA-BASIC 3.0 Amiga manual; the 3.5 material is a separate addendum. Consequently, the entries below are **base-manual evidence**, not proof that every feature is present in interpreter 3.5.

Page references use the manual's printed chapter-page labels. The PDF viewer page is not the same number. The source pages are identified below as printed page ranges.

## Chapter 5 — Input and output (printed pp. 5-2–5-22)

### Keyboard and interactive input

- `INKEY$()` — returns a pressed key as a string/ASCII value; special keys use a CSI sequence beginning with `CHR$(155)`. The manual notes limitations for modifier keys (p. 5-2).
- `INPUT ["text",] var[,var...]` and `INPUT ["text";] var[,var...]` — interactive input into numeric or string variables; executes in a window, opening window 0 when required (pp. 5-3–5-4).
- `LINE INPUT ["text",] a$[,b$...]` and semicolon-prompt variant — line input into strings; commas in the entered text are retained (p. 5-5).
- `FORM INPUT n,a$` and `FORM INPUT n AS a$` — formatted string entry; n is documented as 1–255. The AS form initially displays the current string for editing (p. 5-6).

### Text output, formatting, cursor

- `PRINT expression`, `PRINT AT(column,line);expression`, `WRITE expression`, `LOCATE line,column` (pp. 5-7–5-8).
- PRINT separators (semicolon, comma, apostrophe) control output positioning; a trailing semicolon suppresses the final line ending. Screen output requires a window. Note the different argument order: PRINT AT uses column,line, LOCATE uses line,column.
- WRITE serializes expression lists as comma-separated data and quotes string values so they can be read back with INPUT.
- `PRINT USING formats,expression...` and `PRINT AT(column,line);USING formats,expression...` — formatted numeric and string output (pp. 5-9–5-10). Numeric formats include digit placeholders, decimal/grouping punctuation, sign and exponent forms; string formats include fixed-width and character placeholders. Consult the page image for exact glyph-level format syntax before implementing a parser.
- `MODE n` — changes numeric separators and date presentation used by PRINT USING, STR$, DATES, SETTIME, DATE$, and FILES (p. 5-11).
- `DEFNUM n` — sets decimal places for numeric PRINT output; internal numeric precision is unchanged (p. 5-12).
- `CRSCOL`, `CRSLIN`, `POS(x)`, `TAB(n)`, `HTAB column`, `VTAB line` (pp. 5-13–5-14). CRSCOL/CRSLIN report cursor coordinates; POS counts output characters since carriage return; TAB emits null characters until POS reaches n; HTAB/VTAB position the cursor.

### DATA, mouse, printer, sound

- `DATA const[,const...]`, `READ var[,var...]`, `RESTORE [label]` — sequential DATA pointer and optional label restore (pp. 5-15–5-16).
- `MOUSE mx,my,mk` — statement returning coordinates and button state through variables; `MOUSEX`, `MOUSEY`, `MOUSEK` — individual query functions (pp. 5-17–5-18). Coordinates are relative to the active output window; no window gives zero coordinates and positions outside the window can be negative. Button state is a bit/sum value in the documented 0–7 range.
- `LPRINT expression`, `LPOS(x)` — printer output and printer output-position query (p. 5-19).
- `HARDCOPY` and `HARDCOPY [adr,comap,modes,srcx,srcy,srcw,srch,destc,destl,spec]` — prints the current window/rastport or a specified source rectangle with optional raster/colormap/mode/destination parameters (pp. 5-20–5-21).
- `SOUND freq,duration[,vol[,chan]]`, `WAVE chan,wave()` — sound generation and waveform selection (p. 5-22). The manual gives volume 0–255, default 127, and channel pairs for left/right output.

**Classification note:** INKEY$, CRSCOL, CRSLIN, POS, MOUSEX, MOUSEY, MOUSEK, and LPOS are expression-returning functions. The remaining entries in this chapter are statements/commands.

## Chapter 6 — Disk handling and file organization (printed pp. 6-2–6-22)

- `EXIST(name$)` — TRUE when the named file exists, otherwise FALSE (p. 6-7).
- `OPEN mode$,[#]n,name$[,num]` — opens channel 0–99 in output, input, append, update, or random mode. For random mode, num specifies record length (p. 6-8).
- `LOF(#n)`, `LOC(#n)`, `EOF(#n)` — file length in bytes, current data-pointer position, and end-of-file test. `CLOSE [#n]` closes one or all channels; `TOUCH [#]n` updates the file's directory timestamp (pp. 6-9–6-10).
- `NAME old$ AS new$`, `RENAME old$ AS new$`, `KILL name$` — rename or delete a file; the manual states NAME requires both files on the same disk (p. 6-11).
- `BLOAD n$[,adr]`, `BSAVE n$,adr,num`, `BGET #n,adr,num`, `BPUT #n,adr,num` — load/save whole memory blocks or transfer portions through an open channel (p. 6-12).
- `INP(#n)`, `OUT #n,a[,b,c...]` — read/write byte values on an opened channel (p. 6-13).
- `INPUT$(num[,#n])` — reads num characters from keyboard or, when a channel is specified, from a file (p. 6-14).
- `INPUT #n,var[,var...]`, `LINE INPUT #n,a$[,b$...]` — structured or line-oriented file input (p. 6-15).
- `PRINT #n,expression`, `PRINT #n,USING form$,expression`, `WRITE #n,expression` — sequential file output; PRINT #n,AT is explicitly unsupported (p. 6-16).
- `STORE #i,x$()[,n[TO z]]`, `RECALL #i,x$()[,n[TO z]],x` — save/load string-array lines; RECALL reports the number of strings read in x (pp. 6-17–6-18).
- `SEEK #n,pos`, `RELSEEK #n,num` — absolute/relative data-pointer positioning; documented for files, not peripheral devices (p. 6-19).
- `FIELD #n,num AS set$[,...]` and `FIELD #n,num AT(x)[,...]` — define random-record fields as strings or map fields directly to numeric storage addresses (p. 6-20).
- `GET #n[,set]`, `PUT #n[,set]`, `RECORD #n,set` — read/write a random-access record and select the current record number (pp. 6-21–6-22).

The chapter distinguishes sequential channels from random-access records. Preserve the manual's channel range and record-length constraints in any implementation; do not infer behavior for devices where the manual explicitly restricts a command to files.

## Chapter 8 — Graphics (printed pp. 8-2–8-24)

- `SETCOLOR register,red,green,blue` / `SETCOLOR register,mix`; `COLOR front[,back[,out]]` — palette register components or packed mix value, and text/fill colors (p. 8-2).
- `DEFFILL [color],[style],[pattern]` or `DEFFILL [color],bitpattern$` — fill color/style/pattern for filled primitives; styles 0–3 and bit-pattern string representation are described (pp. 8-3–8-4).
- `BOUNDARY n` — enable/disable automatic framing of fill areas (p. 8-5).
- `DEFLINE def` — 16-bit line pattern used by LINE, BOX, CIRCLE, ELLIPSE, and POLYLINE (p. 8-6).
- `GRAPHMODE n` — drawing combination modes 0, 1, 2, and 4; combinations are allowed (p. 8-7).
- `RASTPORT adr` — selects the drawing rastport; the manual warns that direct structure offsets require correct setup and can crash the system (p. 8-8).
- `CLIP x,y,w,h[ OFFSET x0,y0]`, `CLIP x1,y1 TO x2,y2[ OFFSET x0,y0]`, `CLIP #n[OFFSET x0,y0]`, `CLIP OFFSET x,y`, `CLIP OFF` — restrict graphic output to a rectangle/window; GET and PUT are excluded (p. 8-9).
- `PLOT x,y`, `LINE x1,y1,x2,y2`, and DRAW coordinate forms — point, line, and connected paths (pp. 8-10–8-11).
- `DRAW expression`, `DRAW(i)`, `SETDRAW x,y,w` — LOGO-like pen movement language; DRAW(0..5) queries position, angle, scaling and pen state (pp. 8-12–8-14).
- `BOX x1,y1,x2,y2`, `PBOX ...`; `CIRCLE x,y,r`, `PCIRCLE ...`; `ELLIPSE x,y,rx,ry`, `PELLIPSE ...` — outline and filled primitives (pp. 8-15–8-16).
- `POLYLINE n,x0,y0[ OFFSET xoff,yoff]`, `POLYFILL n,x(),y()[ OFFSET xoff,yoff]` — polygon line/fill using coordinate arrays (p. 8-17).
- `POINT(x,y)` — returns the color index at a screen coordinate (p. 8-18).
- `FILL x,y[,c]` — flood fill from a coordinate, optionally bounded by a color (p. 8-19).
- `CLS [#n]` — clears the text area, not necessarily the complete graphics window (p. 8-20).
- `TEXT x,y,expression` — draws graphic text (p. 8-21).
- `GET x1,y1,x2,y2,section$`, `PUT x,y,section$[,mode,mask]` — capture and redraw rastport regions (p. 8-22).
- `VSYNC` — waits for vertical blank synchronization (p. 8-23).
- `DISPLAY ON`, `DISPLAY OFF` — enable/disable playfield DMA display while drawing (p. 8-24).

## Chapter 9 — Event, menu, screen, and window handling (printed pp. 9-2–9-39)

- Screen display modes and flags are described before the command pages; the manual includes standard, HIRES, lace, HAM, extra-half-bright, genlock, and hardware-sprite settings (pp. 9-2–9-17).
- `OPENS nr[,x,y,w,h,bitp,mod]` opens/selects a screen; `CLOSES no` closes it (pp. 9-18–9-20).
- `FRONTS no`, `BACKS no`, `MOVES no,x,y`, `TITLES no,str`, `SETSPEN detail_pen,block_pen` — screen stacking, position, title and title colors (pp. 9-21–9-24).
- `SCREEN(no)` and `SCREEN(adr)` — convert between screen index and screen-structure address (p. 9-25).
- `OPENW` has standard and extended forms with geometry, IDCMP event mask, window flags, optional screen index and bitmap; `CLOSEW no` closes a window (pp. 9-26–9-32). The manual documents window indices 0–31 and lists event mask values such as NEWSIZE, REFRESHWINDOW, MOUSEBUTTONS, MOUSEMOVE, GADGETDOWN/UP, MENUPICK, CLOSEWINDOW, RAWKEY, VANILLAKEY, activation/deactivation, disk events and timer ticks.
- Window flags include sizing/drag/depth/close gadgets, SMART_REFRESH, SIMPLE_REFRESH, SUPERBITMAP, BACKDROP, REPORTMOUSE, GIMMEZEROZERO, BORDERLESS, ACTIVE, and PUBTRAP (pp. 9-30–9-32).
- `CLEARW no`, `TITLEW no,a$[,b$]`, `FRONTW no`, `BACKW no`, `FULLW no` — clear, title, stack, or maximize a window (p. 9-33).
- `MOVEW no,x,y`, `SIZEW no,w,h`, `LIMITW no,min_x,min_y,max_x,max_y` — geometry and size limits (p. 9-34).
- `SETWPEN detail_pen,block_pen` — window title colors (p. 9-35).
- `WINDOW(nr)` / `WINDOW(adr)` — convert between window index and structure address (p. 9-36).
- `ALERT sym,text$,default,button$,choice` — alert requester; the manual explicitly says sym is for later versions and does not apply to this version (p. 9-37).
- `FILESELECT title$,oktext$,path$,name$` — file requester; selected path is returned in name$, with empty string on cancel (pp. 9-38–9-39).

## Chapter 10 — Advanced programming (printed pp. 10-2–10-16)

- CLI invocation and `_dosCmd$` — program name and optional command tail can be passed when launching GFA-BASIC from CLI; the tail is available in `_dosCmd$` (p. 10-2).
- `EXEC command$,in,out` — starts a CLI process; -1 preserves the corresponding standard input/output channel (p. 10-3).
- Library base variables: `_DosBase`, `_IntBase`, `_GfxBase`, `_LayersBase` — addresses of library bases already used internally by GFA-BASIC (p. 10-4).
- `SPRITE #n,data$`, `SPRITE #n`, `SPRITE #n,x,y`, `SPRITE ON`, `SPRITE OFF` — hardware sprite definition, release, positioning, and global visibility (pp. 10-5–10-6). The text describes eight sprite numbers 0–7, with sprite 0 assigned to the mouse.
- `STICK(p)`, `STRIG(p)` — joystick position and fire-button state (pp. 10-7–10-8). The manual also mentions a compatibility STICK statement that has no effect.
- `SAY x$[,m%()]`, `TRANSLATE$()` — phoneme speech and English-to-phoneme translation, requiring narrator.device and translator.library respectively (p. 10-9).
- FD-file example — converts Amiga FD library description files into GFA procedures; it is an example, not a BASIC statement (pp. 10-10–10-12).
- BOB/object support — overview and command subset: `OBJECT.AX`, `OBJECT.AY`, `OBJECT.CLIP`, `OBJECT.CLOSE`, `OBJECT.ON`, `OBJECT.OFF`, `OBJECT.PLANES`, `OBJECT.PRIORITY`, `OBJECT.SHAPE`, `OBJECT.START`, `OBJECT.STOP`, `OBJECT.VX`, `OBJECT.VY`, `OBJECT.X`, `OBJECT.Y`, `OBJECT.X(obj)`, `OBJECT.Y(obj)`, `OBJECT.VX(obj)`, `OBJECT.VY(obj)`, `OBJECT.AX(obj)`, `OBJECT.AY(obj)`, and `ON COLLISION GOSUB proc` (pp. 10-13–10-16).
- The manual explicitly marks COLLISION ON/OFF/STOP as unimplemented and notes that `COLLISION(-1)` does not return the window number (p. 10-13).

## Implementation and versioning cautions

1. Do not treat every heading as an expression function. Many are statements, commands, or OS-facing facilities; the signatures above preserve that distinction where the source makes it clear.
2. Preserve hardware/platform dependencies: screen/rastport structures, Amiga event masks, sprite DMA, sound channels, narrator.device, translator.library, and CLI channels are not portable abstractions.
3. The manual's explicit caveats (e.g. ALERT symbol parameter, unsupported collision commands, PRINT # AT restriction, CLIP exclusion of GET/PUT) are part of the evidence and should not be silently normalized away.
4. This note does not establish 3.5 support. Reconcile each candidate against the 3.5 addendum, independent 3.5 book material, and interpreter probes before adding it to the confirmed 3.5 language core.
