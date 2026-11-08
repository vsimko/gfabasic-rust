# AROS m68k reference environment

The reference interpreter is an Amiga 68k executable. The CI environment must
therefore use the **AROS m68k** port, not an x86-64 AROS image or a hosted
Linux build.

## Selected operating-system image

Use the official AROS `amiga-m68k` nightly build from
[AROS nightly downloads](https://www.aros.org/cgi-bin/files?lang=en&type=nightly2)
(or its [SourceForge mirror](https://sourceforge.net/projects/aros/files/nightly2/)).
The published m68k build provides:

- `amiga-m68k-boot-floppy`: a bootable floppy image described by AROS as
  kicking the AROS ROM.
- `amiga-m68k-boot-iso`: the CD-ROM image containing the core AROS system
  files.

The m68k port is described by AROS as binary-compatible with AmigaOS. Do not
substitute `pc-x86_64`, `linux-x86_64`, or another architecture's build.

For emulator runs, keep the ROM and system media from the **same dated
nightly**. The boot ISO contains the AROS m68k ROM files under its
`boot/amiga/` directory in the standard nightly layout. Preserve the exact
nightly date, archive names, and SHA-256 hashes in each run record. Do not
silently follow a moving `latest` URL in a conformance run.

## Emulator

Use FS-UAE on the Linux CI runner. FS-UAE accepts configuration through a
configuration file or command-line options, including floppy and CD-ROM
media. It can also use its AROS replacement ROM when a preferred Kickstart
ROM is unavailable; for reproducibility, prefer the matching ROM files
distributed with the selected AROS nightly rather than an implicit emulator
default.

- [FS-UAE command-line usage](https://fs-uae.net/command-line/)
- [FS-UAE Kickstart and replacement-ROM behavior](https://fs-uae.net/docs/kickstarts/)
- [FS-UAE floppy image configuration](https://fs-uae.net/docs/floppy-disk-images/)

The emulator profile (CPU model, memory, chipset, ROM paths, boot media, and
display/serial settings) must be checked into the repository or generated
deterministically by the workflow. Use a headless display server such as
Xvfb if the selected FS-UAE build requires an X server.

## Planned CI stages

1. Select an explicit AROS nightly date; download the matching m68k boot ISO
   and, if needed, boot floppy from the official AROS download host.
2. Verify archive integrity, extract the ISO and matching ROM files, and
   record their hashes and upstream provenance.
3. Download and verify the private GFA BASIC reference executable using the
   existing `GFA_ASSETS_TOKEN` workflow secret.
4. Attach the interpreter to the emulated AROS environment without copying it
   into the public repository.
5. Boot AROS, launch the interpreter, and capture deterministic guest output
   or another machine-readable result. A successful emulator process exit
   alone is not proof that the interpreter started or ran correctly.
6. Upload logs and a short-lived result artifact. Keep ROMs, system images,
   and the commercial interpreter out of public artifacts unless their
   respective licences explicitly permit redistribution.

## Current status and limitations

The existing `amiga-reference.yml` workflow only fetches, validates, and
packages the private GFA BASIC 3.52 executable. It does **not** yet download
AROS, launch FS-UAE, boot an Amiga guest, or execute GFA BASIC.

The AROS m68k port is the selected free operating-system candidate. Its
compatibility with this particular GFA BASIC executable, and the exact
headless boot-and-launch procedure, still require a successful smoke test.
Keep 3.52 observations separate from the project's normative Amiga GFA BASIC
3.5 target.

AROS is an open-source AmigaOS-compatible reimplementation, not a commercial
Kickstart ROM. If a specific program operation proves incompatible with AROS,
the project may separately test a legally held Kickstart/AmigaOS setup; such
ROMs must remain private and must not be added to this public repository.
