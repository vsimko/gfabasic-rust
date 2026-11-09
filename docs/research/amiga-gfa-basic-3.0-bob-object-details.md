---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — BOB and OBJECT API evidence"
---

# GFA BASIC 3.0 Amiga — BOB and OBJECT API evidence

Source: *GFA BASIC 3.5 Interpreter*, Chapter 10, printed pp. 10-13–10-16. This is Amiga GFA BASIC 3.0 base-manual material.

## BOB model

The manual describes BOBs (blitter objects) as display objects compatible with Amiga BASIC OBJECT commands, with documented exceptions around collision handling. This is a platform-specific graphics facility and not a language-core array/object model. (10-13)

## Indexed OBJECT surface

The command index identifies the following object properties/operations: `OBJECT.AX`, `OBJECT.AY`, `OBJECT.CLIP`, `OBJECT.CLOSE`, `OBJECT.OFF`, `OBJECT.ON`, `OBJECT.PLANES`, `OBJECT.PRIORITY`, `OBJECT.SHAPE`, `OBJECT.START`, `OBJECT.STOP`, `OBJECT.VX`, `OBJECT.VY`, `OBJECT.X`, and `OBJECT.Y`. The chapter's detailed pages are 10-14–10-16.

These names cover acceleration, clipping, lifecycle, plane selection, priority, shape, movement/velocity and position. The index establishes the name/page family, but exact assignment/read forms, array layouts, coordinate units, collision semantics and resource cleanup must be taken from each individual entry before implementation.

## Collision control

The index lists `COLLISION ON/OFF/STOP` on printed p. 10-13 and `ON COLLISION GOSUB` on p. 10-16. These are BOB collision/event operations, distinct from generic ON GOSUB control flow. The collision callback behavior and object-pair reporting require detailed page review.

## Evidence boundary

All findings are 3.0 Amiga platform candidates. They do not establish interpreter 3.5 retention, Linux/SDL support, or that BOB semantics can be implemented as ordinary software sprites without observable differences. Hardware/blitter resource ownership and event timing remain platform-specific.
