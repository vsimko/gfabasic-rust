---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — event, menu, screen and window evidence"
---

# GFA BASIC 3.0 Amiga — event, menu, screen and window evidence

Source: *GFA-BASIC 3.0 Amiga*, Chapter 9, “Event-, menu-, screen- and window handling”, printed pages 9-2–9-39 (approximately Interpreter PDF pages 256–293). The chapter is part of the 3.0 base manual, not the separate 3.5 addendum.

## Chapter structure and event/menu surface

The contents page groups the chapter into event handling (9-2), pull-down menus (9-3), menu commands (`MENU`, `MENU()`, `ON MENU`, `ON MENU...GOSUB`, menu removal and coordinates; 9-4–9-12), screen commands (9-14 onward), and window commands (9-25 onward). (Contents, PDF page 8–9)

The event/menu API is therefore a distinct Amiga UI subsystem rather than a collection of generic language statements. The contents list is evidence of command names and organization; details for individual menu and event forms should be transcribed from their own pages before assigning semantics or signatures.

## Screens

The chapter lists screen open/close, front/back, movement, title and pen-setting operations. The contents identify `OPENS`, `CLOSES`, `FRONTS/BACKS`, `MOVES`, `TITLES` and `SETSPEN` (9-17–9-23), followed by `SCREEN` (9-24). These are screen-management APIs, not generic text-mode output.

## Windows

The contents identify `OPENW` (9-26), `CLEARW/TITLEW/BACKW/FULLW` (9-32), `MOVEW/SIZEW/LIMITW` (9-33), `SETWPEN` (9-34), and `WINDOW` (9-35).

- `CLEARW` clears a window; `TITLEW` sets its title text; `FRONTW` and `BACKW` change stacking order; `FULLW` expands a window to full-screen size. (9-32)
- `MOVEW` sets a window's screen position; `SIZEW` changes dimensions; `LIMITW` sets minimum and maximum dimensions, applying to mouse resizing and command-driven resizing. (9-33)
- `SETWPEN` sets window title detail and block pen colors. (9-34)
- `WINDOW(nr)` returns a window index or, when passed an address, resolves the index from the window structure address. (9-35)

## Requesters and file selection

- `ALERT` displays an alert requester with message lines and button labels; the selected button number is returned through an array argument. The manual says its `sym` parameter is reserved for later versions and does not apply to the version described. (9-36)
- `FILESELECT` displays a file selector; the selected path/name is returned in a string, while Cancel yields an empty string. The documented path can identify a drive/directory and optionally a preselected file. (9-37–9-38)

## Evidence boundary

This source establishes the documented 3.0 Amiga UI command surface and selected semantics. It does not establish that these APIs are implemented by this repository or that their behavior is portable beyond Amiga. The 3.5 addendum contents separately list `OPENW` (13-34); the base chapter's `OPENW` entry must not be mistaken for evidence of a newly introduced 3.5 feature.
