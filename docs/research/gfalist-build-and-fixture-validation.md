# gfalist build and fixture validation

## Scope

The uploaded `gfalist.lha` archive was extracted locally and its upstream
C implementation was built and exercised against the bundled fixtures.
This is a validation of the archived listing converter, not of the Rust
interpreter and not of GFA BASIC runtime semantics.

## Build result

The upstream Makefile does not link with the default modern GCC settings
because `charset` is tentatively defined in a header and also defined in
`charset.c`. With GCC's default `-fno-common`, the linker reports a
multiple-definition error.

The unmodified upstream sources build successfully when using the legacy
common-symbol behavior:

```sh
make clean
make -j2 CFLAGS='-g3 -O2 -Wall -fcommon'
```

The build emits pointer-signedness warnings in the historical token tables
and converter code. They are not fatal with this build command. No source
changes were made to suppress or reinterpret those warnings.

## Bundled test result

After the build, the archive's test recipe was run:

```sh
make -C tests
```

Results:

- Seven `gb36test.a` fixtures (`default.gfa`, `default2.gfa`,
  `default3.gfa`, `default4.gfa`, `default5.gfa`, `hell.gfa`,
  `sky.gfa`) were extracted and decoded.
- Each generated listing matched its archived expected `.lst` file:
  all seven `.out` diff files were zero bytes.
- The `SP_SEK_V.GFA` INLINE-data test completed and emitted a 404-byte
  `INLINE-pss.inl` file.

This establishes reproducibility for the archive's own bundled fixtures
under the tested Linux/GCC environment. It does not establish that these
fixtures cover all GFA BASIC 3.5 features or that the decoder handles all
historical file variants.

## Repository import / licensing boundary

The archive identifies the converter as GPL version 2 only and attributes
the original program to Peter Backes (1992–2001), with later Makefile
changes attributed to Markus Hoffmann. Any redistribution of the C
implementation must retain its license and attribution. It must remain
clearly separated from the repository's MIT-licensed project code; this
validation note alone does not relicense or incorporate the C sources.

The archive's generated executable, object files, static library, and
generated test outputs are build artifacts and should not be committed.
The source archive's `COPYING` and original source files should be retained
together if the implementation is imported in a later dedicated change.

## Implication for the Rust project

The converter can serve as a differential oracle for its documented
GFA 3.x input subset: feed a tokenized `.GFA` fixture to the converter and
compare its ASCII listing with an independently implemented decoder.

Before using its token tables as normative GFA BASIC 3.5 data, identify the
fixture's origin/version and compare the token spellings against the supplied
Amiga 3.5 manual. The converter's own documentation describes support as
limited and version-specific.
