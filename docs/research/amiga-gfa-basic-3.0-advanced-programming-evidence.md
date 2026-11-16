---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — advanced programming evidence"
---

# GFA BASIC 3.0 Amiga — advanced programming evidence

Source: *GFA-BASIC 3.0 Amiga*, Chapter 10, “Advanced programming”, printed pages 10-2–10-16 (Interpreter PDF pages 296–310). This is 3.0 base-manual material.

## CLI and process integration

- `_dosCmd$` exposes command characters passed after the program name when GFA BASIC is started from CLI; the chapter also documents CLI startup options for loading and optionally executing a program. (10-2)
- `EXEC command$,in,out` starts a CLI process. The two channel arguments select standard input/output; -1 retains the current channel. The examples distinguish waiting for a process from starting a parallel process. (10-3)

## Library bases and direct OS calls

The predefined library-base variables listed are `_DosBase`, `_IntBase`, `_GfxBase`, and `_LayersBase`. The text says these libraries are already resident/used internally by GFA BASIC and can be called directly without opening them. (10-4) Chapter 11 then documents the system routines grouped by library.

## Hardware and input devices

- `SPRITE` defines, positions, enables and disables Amiga hardware sprites. The manual describes eight sprite numbers (0–7), with sprite 0 assigned to the mouse, and gives the bitmap row layout and paired color-register behavior. (10-5–10-6)
- `STICK(p)` reads joystick position and `STRIG(p)` reads fire-button state. The text distinguishes ports 0 and 1 and notes that port 0 is integrated with Intuition mouse handling; it also says the compatibility `STICK` command has no effect. (10-7–10-8)

## Speech

`SAY` emits speech using Amiga phoneme codes; `TRANSLATE$()` converts English text to phoneme codes. The manual states the required `narrator.device` and `translator.library` dependencies and documents a parameter array for voice characteristics. It cautions that translation is often incorrect for languages other than English. (10-9)

## FD files and BOBs

- The FD-files section explains how supplied Amiga GFA procedure-definition files can be converted to listings and included with `MERGE`; it shows how library offsets and register arguments map into `RCALL` wrappers. (10-10–10-12)
- The BOB (blitter object) section says GFA BASIC 3.0 supports BOB display/use and describes compatibility with Amiga BASIC OBJECT commands, while explicitly listing collision-related exceptions. (10-13)
- The documented OBJECT family includes acceleration, clipping, close, on/off, plane selection, priority, shape, start/stop, velocity, position and inquiry functions; `ON COLLISION GOSUB` is also listed. (10-14–10-16)

## Evidence boundary

This chapter is platform-specific: CLI process creation, library bases, hardware sprites, joystick ports, speech devices/libraries, register-level FD wrappers and BOBs are Amiga integration, not evidence of platform-independent language semantics. All statements above are attributed to the 3.0 base chapter. Do not infer 3.5 support or repository implementation from the manual alone.
