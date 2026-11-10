---
type: "Testing Guide"
title: "Original Amiga interpreter reference asset"
---

# Original Amiga interpreter reference asset

The project targets **Amiga GFA BASIC interpreter 3.5**. A separately supplied
GFA BASIC 3.52 (1990) interpreter is retained in the private repository
`vsimko/gfabasic-private-assets` for version-specific research only. Do not
treat 3.52 observations as evidence for 3.5.

## Prepare the private 3.52 executable

The manual GitHub Actions workflow
[`Prepare Amiga reference interpreter`](../../.github/workflows/amiga-reference.yml)
downloads the `GFABasic` executable from the private assets repository and
checks its exact size, SHA-256, and Amiga HUNK_HEADER magic before packaging it
as a short-lived workflow artifact.

Required repository secret in `vsimko/gfabasic-rust`:

- `GFA_ASSETS_TOKEN`: fine-grained PAT limited to
  `vsimko/gfabasic-private-assets`, with **Contents: read-only** permission.

The workflow is deliberately `workflow_dispatch` only. It does not run for
pull requests or ordinary pushes, and it never copies the binary into this
public repository.

## Prepare and run it

The **Prepare Amiga reference interpreter** workflow only fetches, verifies,
and packages the executable. It does not execute it.

The separate **Amiga GFA BASIC 3.52 native Merge and Run** workflow boots
Amiberry with the private interpreter and licensed Kickstart ROM, merges an
ASCII listing through the native editor, and runs it. Its current regression
batch writes diagnostic output to `FACT.OUT` and terminal fixture records to
`GFATEST.OUT`. The host validates the exact requested fixture IDs with
`scripts/validate_gfatest_results.py`; it does not manufacture PASS records.

The native workflow can be run from **Actions** with **Run workflow**. Pull
requests that change the workflow also exercise it. It requires the
`GFA_ASSETS_TOKEN` secret and the licensed private assets described above.
Download the `amiga-gfa-basic-3.52-native-merge-run` artifact to inspect the
raw result file, emulator diagnostics, and screenshots.

## Emulator prerequisites and limits

The artifact contains only the `GFABasic` executable. It does not contain an
Amiga Kickstart ROM, Workbench disk, or full disk image. A separately licensed
and configured Amiga environment is required before the interpreter can be
executed in FS-UAE or another Amiga emulator. This workflow only fetches,
validates, and packages the executable; it does not claim that the interpreter
has been run.

Do not commit commercial interpreter binaries, ROMs, or disk images to this
public repository. Keep any version-specific observations clearly labelled
with the exact interpreter version and media provenance.
