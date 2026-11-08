# Amiga GFA-BASIC examples

Imported from the user-provided Aminet archives `GFA_Menus.lha`,
`GFA_Gadgets.lha` and `DemoMaker.lha`.

## Included source listing

- [GFA_Menus/OS-MENU.LST](GFA_Menus/OS-MENU.LST) — readable GFA-BASIC listing
  demonstrating `MENU`, `MENU KEY`, AmigaOS message-port handling
  (`WaitPort`, `GetMsg`, `ReplyMsg`), `CLOSEWINDOW` and `MENUPICK`
  decoding, including menu and submenu selection.

## Other archive contents inspected

- GFA_Gadgets contains a readable `GADGETS.LST` listing and notes describing
  buttons, string gadgets, checkboxes, sliders, arrows and invisible hit areas.
- DemoMaker 0.45 contains four `.demo` project files and AmigaGuide
  documentation. It is a separate demo-authoring application, not a
  GFA-BASIC conformance suite.
- The archives also contain tokenized `.GFA` files and Amiga executables;
  those binary files are not included in this source-only import.

## Scope and caveats

These are historical Amiga examples, not tests validated against this Rust
interpreter. The menu and gadget examples use AmigaOS calls and direct memory
structure access. They do not directly test the disputed `MENU 64` attribute
or the `PERMUT` function.

The GFA_Menus package notes describe a workaround for GFA-BASIC event handling:
use the operating system message port instead of polling with `SLEEP`, and
decode `MENUPICK` into menu, item and subitem fields.
