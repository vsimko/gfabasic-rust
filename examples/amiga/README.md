# Amiga GFA-BASIC examples

Imported from user-provided Aminet archives `GFA_Menus.lha`,
`GFA_Gadgets.lha`, `DemoMaker.lha`, and `GFA_CLI.lha`.
The separate `gfalist.lha` archive contains a GFA tokenized-source listing
utility and format notes.

## Included source listings

- [GFA_Menus/OS-MENU.LST](GFA_Menus/OS-MENU.LST) — menu, menu-key and
  AmigaOS message-port handling (`WaitPort`, `GetMsg`, `ReplyMsg`),
  `CLOSEWINDOW` and `MENUPICK`.
- [GFA_CLI/COMMANDLINE.LST](GFA_CLI/COMMANDLINE.LST) — command-line argument
  extraction using `@commandline$` and `@numargs`.
- [GFA_CLI/CONPRINT.LST](GFA_CLI/CONPRINT.LST) — CLI output through file `*`.
- [GFA_CLI/systemtaglist.LST](GFA_CLI/systemtaglist.LST) — launching an
  external command through AmigaOS DOS library calls.
- [GFA_CLI/CLI.txt](GFA_CLI/CLI.txt) — package documentation by Frank Reibold.

## GFA file-format reference

- [gfalist/doc/gfa_frmt.txt](gfalist/doc/gfa_frmt.txt) — archived technical
  notes about tokenized GFA file layout, variable tables and function tokens.
- [Frontend comparison](../../docs/research/gfalist-format-vs-rust-frontend.md)
  — separates binary `.GFA` decoding from textual source lexing and records
  the currently uncovered format areas.
- [gfalist/README.md](gfalist/README.md) — description and provenance of the
  GPL-2.0-only GFA 3.x listing converter.

## Other archive contents inspected

- GFA_Gadgets contains a readable `GADGETS.LST` listing and notes describing
  buttons, string gadgets, checkboxes, sliders, arrows and invisible hit areas.
- DemoMaker 0.45 contains four `.demo` project files and AmigaGuide
  documentation. It is a separate demo-authoring application, not a
  GFA-BASIC conformance suite.
- The archives also contain tokenized `.GFA` files and Amiga executables;
  these binary files are not all included in this source-oriented import.

## Scope and caveats

These are historical examples and format notes, not tests validated against
this Rust interpreter. Some examples use AmigaOS calls and direct memory
structure access. They do not directly test the disputed `MENU 64` attribute
or the `PERMUT` function.
