# Import notes

## Archive contents reviewed

The uploaded `gfalist.lha` archive contains the GPL-2.0-only C implementation,
its build recipe, man page, token tables, decoder, a tokenized test program,
and an additional archived test fixture (`tests/gb36test.a`).

The binary executable `gfalist` from the archive is intentionally not treated
as source code and is not copied into this repository.

## Build observations

The upstream Makefile generates `version.h` from `HISTORY` using
`verextr.sh`. It then builds `libsky.a` from `sky.c` and `tables.c`,
and links the `gfalist` command from `gfalist.c` and `charset.c`.

The archive does not include a checked-in `version.h`; a reproducible build
must run the generation step. The Makefile targets GCC and uses POSIX
interfaces such as `getopt`.

## Format and compatibility observations

- The README describes input as tokenized GFA BASIC version 3.x files
  (`.GFA`) and output as readable ASCII listings (`.LST`).
- The format note documents the header offset-pointer table, variable
  identifier tables and function-token ranges.
- The C implementation explicitly rejects older file versions 1, 2 and 3
  and a special `GF...` file signature (version 70); it does not claim to
  decode every historical GFA file format.
- The archived test recipe uses `gb36test.a` and compares generated listings
  with expected output. `SP_SEK_V.GFA` is used for the INLINE-data test.

## Relevance to gfabasic-rust

Treat this package as a historical file-format reference and potential
differential oracle for tokenized source decoding, not as an interpreter
implementation or language conformance suite. Its version checks and token
tables need to be compared with the supplied 3.5 manuals before being used
as normative behavior.

The archive is licensed under GNU GPL version 2 only; retain its
`COPYING` file and attribution when redistributing the implementation.
