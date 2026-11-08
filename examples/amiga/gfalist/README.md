# gfalist

Reads tokenized GFA BASIC version 3.x source files (`.GFA`) and converts
them to human-readable ASCII listings (`.LST`).

This is the fixed version of the original ONS tool. It is useful for
inspecting historical tokenized programs and producing text diffs; it
does not merge binary GFA files.

The upstream package identifies the program as part of the GFABASIC4
development package, copyright (C) 1992-2001 Peter Backes, and licenses
it under GNU GPL version 2 only. See [COPYING](COPYING).

The source archive also contains a format note at [doc/gfa_frmt.txt](doc/gfa_frmt.txt)
and an example tokenized program at [tests/SP_SEK_V.GFA](tests/SP_SEK_V.GFA).

The archived README describes GFA BASIC version 3.x input support. This
tool is a format/conversion reference, not a runtime or language conformance
test for this Rust interpreter.
