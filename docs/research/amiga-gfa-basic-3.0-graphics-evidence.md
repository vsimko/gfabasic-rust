# GFA BASIC 3.0 Amiga — graphics evidence

Source: *GFA-BASIC 3.0 Amiga*, Chapter 8, “Graphics”, printed pages 8-2–8-24 (Interpreter PDF pages 232–254). The handbook states that the manual was originally written for 3.0; Chapter 13 is the separate 3.5 addendum. This file records the Chapter 8 baseline only.

## Drawing state and raster configuration

- `SETCOLOR register,red,green,blue` sets RGB components (each 0–15) in a screen color register; the packed form is `SETCOLOR register,mix`, with `mix=red*512+green*16+blue`. `COLOR` selects character foreground/background and fill-outline colors. Palette availability depends on screen resolution and the current screen. (8-2)
- `DEFFILL` selects fill color, style and pattern, or accepts a bit-pattern string. Styles are empty, filled, dotted and hatched; predefined patterns are referenced in Appendix 12. (8-3–8-4)
- `BOUNDARY` toggles automatic fill-area framing. (8-5)
- `DEFLINE` defines a 16-bit line pattern, including predefined values 0–6. (8-6)
- `GRAPHMODE` selects raster combination modes: JAM1 (0), JAM2 (1), COMPLEMENT (2) and INVERSVID (4); the text describes replacement, transparent, XOR and reverse-transparent behavior. Modes may be combined. (8-7)

## Raster target and clipping

- `RASTPORT adr` redirects subsequent graphics output to a screen/window/alert-box RastPort. The manual explicitly warns that incorrect setup can crash the system and that screen RastPort output is not automatically clipped. (8-8)
- `CLIP` accepts rectangle coordinates or a window number, optional `OFFSET`, and `CLIP OFF` disables clipping. The manual says clipping does not apply to `GET` and `PUT`. (8-9)

## Drawing and filled primitives

- `PLOT`, `LINE`, and `DRAW` provide point, line, chained-coordinate and LOGO/HPGL-like drawing. (8-10–8-11)
- The extended `DRAW` language includes forward/backward movement, turns, scaling, absolute/relative movement and draw, pen up/down, color, and state inquiry through `DRAW(0..5)`; `SETDRAW` sets position and angle. (8-12–8-14)
- Primitive commands: `BOX/PBOX`, `CIRCLE/PCIRCLE`, `ELLIPSE/PELLIPSE`, `POLYLINE/POLYFILL`. Polygon coordinates are supplied in parallel arrays; the first and last vertices are automatically connected. (8-15–8-17)
- `POINT(x,y)` reads the color at a screen coordinate. `FILL` fills a bounded region, optionally constrained by a color. (8-18–8-19)
- `CLS` clears the text area of a window; the manual distinguishes this from clearing all graphics in a window. `TEXT` renders a string at graphics coordinates. (8-20–8-21)

## Screen capture and display timing

- `GET` stores a RastPort rectangle in a string; `PUT` draws that saved section with a selectable raster-combination mode and optional bitplane mask. The text gives a 32,768-byte section description and lists representative modes. (8-22)
- `VSYNC` waits for a vertical blank interrupt of the current screen and is suggested for animation using `GET/PUT`. (8-23)
- `DISPLAY ON/OFF` sets or clears the playfield DMA bit so drawing can be completed before display is enabled. (8-24)

## Evidence boundary

These are descriptions of the 3.0 baseline chapter. They do not establish that every command is unchanged in 3.5, nor do they establish implementation or Linux compatibility in this repository. The 3.5 addendum contents list contains `PCOLOUR` (13-34), but that does not by itself prove how it relates to or replaces the Chapter 8 color commands.
