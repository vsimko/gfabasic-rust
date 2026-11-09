# GFA BASIC 3.0 Amiga — screen and window command details

Source: *GFA BASIC 3.5 Interpreter*, Chapter 9, printed pp. 9-15–9-36. These pages document the 3.0 base-manual Amiga screen/window API. The 3.5 addendum separately lists a negative-screen-number `OPENW` variant; that single variant must not promote the entire 3.0 chapter to 3.5.

## Screen model and modes

The manual describes screens as virtual display surfaces with independent resolutions. Screen index 0 is the Workbench screen; user screens use indices 1–31. The number simultaneously open is limited by available graphics memory and index slots. Maximum dimensions are 1024×1024, while the stated visible monitor area is up to 640×512. (9-15)

Documented resolution flags include HIRES, LACE, HIRES+LACE, EXTRA_HALF_WIDTH, HAM, GENLOCKVIDEO and SPRITES. The manual describes bitplane limits and visible areas for each mode; HAM requires at least five bitplanes and uses six bitplanes to select 4096 colors. GENLOCKVIDEO and SPRITES are hardware/display flags. (9-15–9-17)

## Screen command signatures

| Form | Source-described behavior | Page |
|---|---|---:|
| `OPENS nr[,x,y,w,h,bitp,mod]` | Opens/selects screen index 0–31; optional position, dimensions, bitplane count and display mode | 9-18–9-19 |
| `CLOSES nr` | Closes screen and all windows on it; releases its screen memory | 9-20 |
| `FRONTS nr` / `BACKS nr` | Moves indexed screen to foreground/background | 9-21 |
| `MOVES nr,x,y` | Moves screen to absolute coordinates; negative coordinates are rejected | 9-22 |
| `TITLES nr,str$` | Sets/clears screen title | 9-23 |
| `SETSPEN detail_pen,block_pen` | Sets screen title text and title-field pens | 9-24 |
| `SCREEN(nr)` / `SCREEN(adr)` | Converts screen index to address and address to index | 9-25 |

The manual warns that moving a screen while a window is open can prevent mouse movement and may trigger operating-system errors. (9-22)

## Window lifecycle and message model

Window indices are 0–31. Graphic/text output goes to the current window or RastPort; if neither has been selected, the interpreter automatically opens window 0 on the current screen. (9-26)

`OPENW` accepts a window number and optional position, size, IDCMP mask, flags, screen index and bitmap address. Default window 0 covers the screen except its title line; default windows 1–15 occupy quarter-screen positions. Negative/out-of-screen coordinates are rejected. The IDCMP mask selects Intuition events; the chapter enumerates size, refresh, mouse, gadget, requester, menu, close, keyboard, preferences, disk and window-activation messages. (9-27–9-29)

Window flags configure sizing/drag/depth/close gadgets, refresh strategies, backdrop, mouse reporting, zero-zero coordinates, borders, active state, public-screen trapping and bitmap behavior. The source distinguishes SMART_REFRESH, SIMPLE_REFRESH and SUPERBITMAP; with SUPERBITMAP the programmer supplies a bitmap at least as large as the window. (9-30–9-32)

| Form | Source-described behavior | Page |
|---|---|---:|
| `OPENW no[,x,y,w,h,IDCMP,flags[,scrno[,bitmap]]]` | Opens window with optional geometry, event mask, flags, target screen and bitmap | 9-27–9-32 |
| `CLOSEW no` | Closes indexed window; error if already closed | 9-32 |
| `CLEARW no` | Clears window contents | 9-33 |
| `TITLEW no,a$[,b$]` | Sets inactive and optional active title strings | 9-33 |
| `FRONTW no` / `BACKW no` | Changes window stacking order | 9-33 |
| `FULLW no` | Expands window to full screen | 9-33 |
| `MOVEW no,x,y` | Moves window to absolute screen coordinates | 9-34 |
| `SIZEW no,w,h` | Changes window dimensions | 9-34 |
| `LIMITW no,min_x,min_y,max_x,max_y` | Sets mouse/command resizing bounds | 9-34 |
| `SETWPEN detail_pen,block_pen` | Sets window title/detail and title-field pens | 9-35 |
| `WINDOW(no)` / `WINDOW(adr)` | Converts window index and window-structure address | 9-36 |

## 3.5-specific qualification

The addendum's negative-screen-number `OPENW` variant is explicitly 3.5 evidence: a negative screen number opens the window on the current top screen (printed p. 13-34 in the supplied interpreter scan). It does not confirm every 3.0 `OPENW` flag, IDCMP value or window lifecycle detail. Those remain inherited candidates.
