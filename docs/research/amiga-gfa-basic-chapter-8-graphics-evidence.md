---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 8 — graphics compatibility evidence"
---

# Amiga GFA BASIC Chapter 8 — graphics compatibility evidence

## Scope and version boundary

Primary source: `AM-INTERPRETER-35`, *GFA-BASIC 3.5 Interpreter*, Chapter 8, “Graphics”, printed pages 8-2–8-24 (PDF viewer pages 232–254). The manual identifies Chapters 2–12 as the GFA-BASIC 3.0 Amiga base manual and treats Chapter 13 onward as the 3.5 addendum. Therefore the graphics entries below are **documented 3.0 baseline candidates**, not proof of availability in 3.5.

Printed page labels are the citations. PDF viewer page numbers are included only to help locate the scan and must not be used as printed-page citations.

Evidence dimensions are independent:

- **Documented (3.0):** described on the Chapter 8 page cited below.
- **Documented (3.5):** explicitly present in the 3.5 addendum.
- **Observed (3.52):** exercised against a named 3.52 interpreter build with a retained result artifact.
- **Implemented:** implemented in this repository.
- **Conformant:** a passing executable test against an accepted interpreter for the claimed version.

## Chapter 8 inventory

| Area | Construct / source spelling | Source-described behavior and constraints | Printed page |
|---|---|---|---:|
| Palette | `SETCOLOR register,red,green,blue`; `SETCOLOR register,mix` | Sets a screen palette register. RGB components are 0–15; packed value is `red*512 + green*16 + blue`. Each open screen has its own palette. | 8-2 |
| Text and fill colors | `COLOR color` | Selects character foreground/background and fill-outline colors. Background is shown in GRAPHMODE 1 (JAM2); values depend on screen resolution. | 8-2 |
| Fill state | `DEFFILL [color],[style],[pattern]`; `DEFFILL [color],bitpattern$` | Sets fill color, style and pattern for PBOX, PCIRCLE, PELLIPSE, POLYFILL and FILL. Styles: 0 empty, 1 filled, 2 dotted, 3 hatched. Bit-pattern strings encode 16-bit-wide rows; maximum string length is 257. | 8-3–8-4 |
| Fill boundary | `BOUNDARY n` | Enables automatic framing for nonzero n; zero disables it. | 8-5 |
| Line style | `DEFLINE def` | Sets a 16-bit pattern for LINE, BOX, CIRCLE, ELLIPSE and POLYLINE. Values 0–6 select predefined styles. | 8-6 |
| Raster combination | `GRAPHMODE n` | Modes 0 (JAM1), 1 (JAM2), 2 (COMPLEMENT), 4 (INVERSID); combinations are allowed. The text describes OR/replacement, transparent, XOR and reverse-transparent behavior. | 8-7 |
| Drawing target | `RASTPORT adr` | Redirects graphics output to a screen/window/alert-box RastPort until changed or reset to zero. Screen RastPort output is not automatically clipped; incorrect structure offsets can crash the system. | 8-8 |
| Clipping | `CLIP x,y,w,h [OFFSET x0,y0]`; `CLIP x1,y1 TO x2,y2 [OFFSET x0,y0]`; `CLIP #n [OFFSET x0,y0]`; `CLIP OFFSET x,y`; `CLIP OFF` | Restricts drawing to a rectangle or window; OFFSET changes the origin. CLIP OFF disables clipping. GET and PUT are explicitly excluded. | 8-9 |
| Point/line/path drawing | `PLOT x,y`; `LINE x1,y1,x2,y2`; `DRAW [TO] ...` | PLOT draws a point; LINE draws between endpoints; DRAW supports the same coordinate/path forms and chained points. DEFLINE and COLOR affect line appearance. | 8-10–8-11 |
| LOGO-like drawing | `DRAW expression`; `DRAW(i)`; `SETDRAW x,y,w` | DRAW accepts movement/drawing instructions such as FD, BK, SX, SY, LT, RT, TT, MA, DA, MR, DR, CO, PU and PD. DRAW(0..5) reports position, angle, x/y scale and pen state. | 8-12–8-14 |
| Rectangles | `BOX x1,y1,x2,y2`; `PBOX x1,y1,x2,y2` | Draws outline or filled rectangle using opposite corners. | 8-15 |
| Circles and ellipses | `CIRCLE x,y,r`; `PCIRCLE x,y,r`; `ELLIPSE x,y,rx,ry`; `PELLIPSE x,y,rx,ry` | Outline and filled circle/ellipse primitives; PELLIPSE can draw a filled ellipse or ellipse segment. | 8-16 |
| Polygons | `POLYLINE n,x(),y() [OFFSET xoff,yoff]`; `POLYFILL n,x(),y() [OFFSET xoff,yoff]` | Uses parallel coordinate arrays; first and last vertices are automatically connected. POLYFILL uses DEFFILL state. | 8-17 |
| Pixel query | `POINT(x,y)` | Returns the color index at a screen coordinate; possible values depend on resolution (0–63). | 8-18 |
| Flood fill | `FILL x,y[,c]` | Fills a bounded shape from a seed point. Optional c constrains filling to points of that color; omitted c or c=-1 fills points differing from the seed color. | 8-19 |
| Text-area clear | `CLS [#n]` | Clears only a window's text area, not necessarily its complete graphics area; the manual suggests CLEARW where a full-window clear is required. | 8-20 |
| Graphics text | `TEXT x,y,expression` | Draws a string at graphics coordinates; x,y locate the upper-left corner of the first character. | 8-21 |
| Raster capture/blit | `GET x1,y1,x2,y2,section$`; `PUT x,y,section$[,mode,mask]` | GET stores a RastPort rectangle (described as 32,768 bytes) in a string. PUT redraws it with a raster-combination mode and optional bitplane mask. | 8-22 |
| Vertical sync | `VSYNC` | Waits for a vertical blank interrupt of the current screen; suggested for GET/PUT animation. | 8-23 |
| Display DMA | `DISPLAY ON`; `DISPLAY OFF` | Sets/clears the playfield DMA bit, allowing drawing to be completed before enabling display. | 8-24 |

## Explicit 3.5 addendum evidence

The registry already contains one directly relevant 3.5 addendum declaration:

| Registry ID | Name | 3.5 evidence | Implementation / conformance |
|---|---|---|---|
| `gfa.amiga.graphics.pcolor` | `PCOLOR` | Chapter 13, printed p. 13-34; foreground and optional background color for subsequent PRINT output | `not-implemented`; Linux support unknown; conformance not tested |

The addendum passage describes PCOLOR as otherwise behaving like COLOR. It does **not** establish that all Chapter 8 graphics statements, Amiga RastPort behavior, raster modes, or hardware display operations are available in 3.5. The registry entry is the only currently recorded graphics-specific 3.5 declaration in `api/registry/amiga/special-and-platform-commands.toml`.

## 3.52 interpreter evidence

The tracked native batch `tests/interpreter-probes/amiga-3.52/regression-suite.LST` currently covers FACT boundaries, arithmetic precedence, FOR/NEXT, and string concatenation/LEFT$. Its expected ID list contains only:

- `fact-zero`
- `arithmetic`
- `for-next`
- `strings`

**There is no graphics probe in the tracked 3.52 batch or expected-ID list.** Consequently this repository currently has no retained 3.52 observation for Chapter 8 graphics. Do not count the existence of the batch, its protocol validator, or a planned graphics fixture as a successful runtime test.

A future graphics probe batch should use independent IDs and capture the interpreter build identity, emulator configuration, run date, raw output, expected IDs and result checksum. Candidate probe groups (not yet executed) are:

1. Palette and drawing state: SETCOLOR, COLOR, DEFFILL, BOUNDARY, DEFLINE, GRAPHMODE.
2. Primitives and queries: PLOT, LINE, BOX/PBOX, CIRCLE/PCIRCLE, ELLIPSE/PELLIPSE, POINT.
3. Paths and fill: DRAW/SETDRAW, POLYLINE/POLYFILL, FILL, CLIP.
4. Text and capture: TEXT, CLS, GET/PUT.
5. Display timing/control: VSYNC and DISPLAY ON/OFF.

These tests need a graphics-capable reference environment and observable pixel/output assertions; merely loading a listing or checking that a command parses is not enough to claim graphics behavior.

## Registry and dashboard treatment

- Keep the Chapter 8 base-manual candidates outside the normative Amiga 3.5 registry until each construct has independent 3.5 evidence.
- Keep PCOLOR as a distinct 3.5 addendum declaration; do not use its presence to promote COLOR or the Chapter 8 graphics family.
- Do not add 3.0 candidates to the 3.5 implementation numerator or denominator solely because they appear in this PDF.
- Do not label the graphics inventory complete for the entire language: this note is bounded to Chapter 8, printed pp. 8-2–8-24.
- A 3.52 graphics observation, when obtained, remains a separate compatibility-evidence dimension and is not interchangeable with 3.5 conformance.

## Related repository evidence

- [Existing Chapter 8 graphics extraction](amiga-gfa-basic-3.0-graphics-evidence.md)
- [Expanded base-manual Chapter 8 details](amiga-base-manual-io-graphics-events-advanced-details.md)
- [Amiga 3.5 declaration registry](../../api/registry/amiga/special-and-platform-commands.toml)
- [Native 3.52 regression batch](../../tests/interpreter-probes/amiga-3.52/regression-suite.LST)
- [Native 3.52 expected result IDs](../../tests/interpreter-probes/amiga-3.52/expected-ids.txt)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
