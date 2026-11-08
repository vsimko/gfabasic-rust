# GFA BASIC 3.0 Amiga — output, cursor, mouse and sound evidence

Source: *GFA BASIC 3.5 Interpreter*, Chapter 5, printed pp. 5-7–5-22. This is the 3.0 base-manual I/O chapter; do not treat it as automatic proof of 3.5 retention.

## Text output and formatting

- `PRINT` writes expressions to the current output window. The chapter documents optional positioning (`PRINT AT`), formatting with `USING`, separators, and control of cursor/output position. (5-7–5-10)
- `WRITE` emits values in a comma-separated representation intended for later input. `PRINT USING` is a separate formatted-output path. (5-7–5-9)
- `MODE n` selects decimal/thousands separators and date presentation. The table defines four modes and notes that number formatting applies to PRINT USING and STR$ while date mode affects DATES, SETTIME, DATE$ assignment and FILES. (5-11)
- `DEFNUM` sets numeric input/output formatting behavior; exact parameter details should be taken from its own entry. (5-12)

## Cursor and text-screen state

The chapter indexes `HTAB`, `VTAB`, `POS`, `CRSCOL` and `CRSLIN` as cursor/position operations. The chapter also includes tab and position inquiry functions. (5-13) The manual's editor tab stops are a separate editor feature and should not be confused with runtime `HTAB`/`VTAB`.

## Mouse input

`MOUSE` returns mouse coordinates/button state through variables; `MOUSEK`, `MOUSEX` and `MOUSEY` expose button and coordinate values. The detailed command page is 5-17. These are Amiga pointer/window integrations, not portable keyboard or console input semantics.

## Printer and hardcopy

- `LPRINT` writes to the printer device; `LPOS()` reports printer position. (5-19)
- `HARDCOPY` prints a screen/window image. The manual describes a specification value controlling page contents, width, printer resolution and form-feed behavior; the default is documented as `&H84`. (5-20–5-21)

## Sound output

The index lists `SOUND` and `WAVE` on printed p. 5-22. The chapter page identifies these as sound-output operations, but the OCR extraction is not sufficient to establish complete signatures, waveform encoding, timing, device dependencies or error behavior. Those details remain explicitly untranscribed pending page-image review.

## Evidence boundary

This file adds command-family evidence and page locators; it is not a complete syntax reference. Numeric formatting, printer devices, mouse state and sound output remain 3.0 candidates. Confirm 3.5 behavior before adding declarations to the normative registry; do not map Amiga printer/sound devices to Linux by name alone.