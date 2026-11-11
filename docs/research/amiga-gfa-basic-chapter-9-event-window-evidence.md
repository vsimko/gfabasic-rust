---
type: "Research Note"
title: "Amiga GFA BASIC Chapter 9 — event, menu, screen and window evidence"
---

# Amiga GFA BASIC Chapter 9 — event, menu, screen and window evidence

## Scope and version boundary

Primary source: `AM-INTERPRETER-35`, *GFA-BASIC 3.5 Interpreter*, Chapter 9, “Event-, menu-, screen- and window handling”, printed pages 9-2–9-39. The supplied manual identifies the base chapters as GFA-BASIC 3.0 and Chapter 13 as the 3.5 addendum. The Chapter 9 inventory below is therefore **3.0 base-manual evidence**, not proof that all listed event and Intuition behavior is available in 3.5.

Printed page labels are the citations; PDF viewer page numbers are not interchangeable with them.

## Chapter 9 inventory

| Area | Construct / source spelling | Source-described behavior and constraints | Printed pages |
|---|---|---|---:|
| Event polling | `ON MENU t`, `SLEEP` | ON MENU waits for an Intuition event for the specified number of tenths-of-a-second ticks; SLEEP is equivalent to ON MENU 1. The manual says active windows generate Intui-timer events. | 9-2, 9-9 |
| Pull-down menu model | BASIC menu string array `m$()` | Menu titles and entries are represented by a string array. The manual defines its own terms for menu titles, entries and subentries; preserve that source terminology when mapping menu indexes. | 9-3 |
| Menu creation | `MENU m$()` | Displays a pull-down menu from a string array. Empty strings delimit entries/menus; a leading minus marks a non-selectable entry and an exclamation mark introduces subentries. | 9-4 |
| Menu message data | `MENU(x)`, `MENU(0..11)` | Exposes event-message fields including selected BASIC-menu array index, IDCMP class, code, qualifier, object address, mouse coordinates, time and window address. Event-specific meanings are documented in the following tables. | 9-5–9-8 |
| Event handler registration | `ON MENU GOSUB proc`, `ON MENU KEY GOSUB proc`, `ON MENU BUTTON GOSUB proc`, `ON MESSAGE GOSUB proc` | Registers procedures for menu selection, keyboard, mouse-button or general messages. Delivery depends on the IDCMP flags configured for the window; observation occurs when ON MENU or SLEEP is executed. | 9-10–9-11 |
| Menu mutation and shortcuts | `MENU KILL`, `MENU x,y`, `MENU x,str$`, `MENU KEY x,y` | Disables the pull-down menu and ON MENU settings, changes an entry's presentation/check state or text, and assigns/removes a keyboard shortcut. The manual gives numeric option values and shortcut behavior. | 9-12–9-13 |
| Screen lifecycle | `OPENS`, `CLOSES`, `FRONTS`, `BACKS`, `MOVES`, `TITLES` | Opens/closes indexed screens and changes their stacking order, position and title. Screen 0 is Workbench; user screen indexes are 1–31. Screen movement rejects negative coordinates. | 9-15–9-23 |
| Screen setup and identity | `SETSPEN`, `SCREEN(no)`, `SCREEN(adr)` | Sets title/detail pens and converts between a screen index and its structure address. Resolution flags and bitplane limits are described with screen setup. | 9-15–9-25 |
| Window creation and messages | `OPENW` | Opens an indexed window with optional geometry, IDCMP mask, flags, target screen and bitmap. The chapter enumerates event masks and window flags, including refresh modes and SUPERBITMAP requirements. | 9-26–9-32 |
| Window lifecycle and geometry | `CLOSEW`, `CLEARW`, `TITLEW`, `FRONTW`, `BACKW`, `FULLW`, `MOVEW`, `SIZEW`, `LIMITW` | Closes, clears, titles, stacks, expands, moves and resizes windows; LIMITW sets minimum and maximum size bounds. | 9-32–9-34 |
| Window setup and identity | `SETWPEN`, `WINDOW(no)`, `WINDOW(adr)` | Sets window title/detail pens and converts between a window index and its structure address. | 9-35–9-36 |
| Alert requester | `ALERT sym,text$[,default,button$[,choice]]` | Displays an alert requester; text supports four lines of up to 40 characters, button labels are separated by a vertical bar, and the selected button index is returned. The manual says `sym` does not apply to this version. | 9-37 |
| File selector requester | `FILESELECT title$,oktext$,path$,name$` | Displays a file selector. The initial path may identify a drive/folder or file; the selected file is returned in name$, while Cancel returns an empty string. | 9-38–9-39 |

## Explicit 3.5 addendum evidence

The 3.5 addendum separately documents one Chapter 9-related variant:

| Registry ID | Variant | 3.5 evidence | Boundary |
|---|---|---|---|
| `gfa.amiga.windows.openw-negative-screen` | `OPENW` with a negative screen number | Chapter 13, printed p. 13-34: opens the window on the current top screen | This confirms only the negative-screen-number variant. It does not promote all Chapter 9 OPENW flags, IDCMP messages, screen/window operations or requester behavior to 3.5. |

## 3.52 runtime evidence

The tracked native 3.52 regression batch currently contains only `fact-zero`, `arithmetic`, `for-next` and `strings`. None exercises Chapter 9 event, menu, screen, window or requester behavior. There is therefore no retained 3.52 runtime observation for this chapter in the repository.

Future probes need a graphics-capable emulator/reference setup and explicit observable assertions for event delivery, menu selection, window lifecycle and requester return values. Listing acceptance or command parsing alone is not behavioral conformance.

## Classification and follow-up

- Keep Chapter 9 base-manual entries as 3.0 evidence until each construct is reconciled with explicit 3.5 addendum text or a controlled accepted 3.5 observation.
- Keep the negative-screen-number OPENW variant as a narrow 3.5 declaration; do not infer 3.5 support for other OPENW options.
- Classify screen/window/event commands as Amiga platform API where appropriate, and separate editor-only or direct OS-library operations from language-core constructs.
- Resolve menu message field meanings and numeric IDCMP/flag values against page images before creating declaration-level registry entries.
- No Chapter 9 entry should affect the final 3.5 denominator or implementation numerator solely because it appears in this base chapter.

## Related repository evidence

- [Expanded Chapter 9 screen/window details](amiga-gfa-basic-3.0-screen-window-details.md)
- [Chapter 9 event and window evidence](amiga-gfa-basic-3.0-events-windows-evidence.md)
- [Amiga 3.5 addendum inventory](amiga-gfa-basic-35-addendum-inventory.md)
- [Amiga 3.5 declaration registry](../../api/registry/amiga/special-and-platform-commands.toml)
- [Native 3.52 regression batch](../../tests/interpreter-probes/amiga-3.52/regression-suite.LST)
- [Issue #42 — reference and conformance corpus](https://github.com/vsimko/gfabasic-rust/issues/42)
- [Issue #307 — evidence-based compatibility metrics](https://github.com/vsimko/gfabasic-rust/issues/307)
