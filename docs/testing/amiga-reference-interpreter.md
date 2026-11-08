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

## Run it

1. Open the [Actions tab](https://github.com/vsimko/gfabasic-rust/actions).
2. Select **Prepare Amiga reference interpreter**.
3. Select **Run workflow** on the `main` branch (or the branch containing the workflow).
4. Download the `gfa-basic-3.52-amiga` artifact from the successful run.

## Emulator environment

**AROS m68k is the selected free operating-system candidate** for the Amiga
reference environment. It is distinct from the x86-64 AROS port. See the
[AROS m68k emulator plan](aros-m68k-emulator.md) for the selected upstream
images, FS-UAE setup references, provenance requirements, and planned CI
stages.

The current workflow still only fetches, validates, and packages the GFA BASIC
executable. It does not yet download AROS, boot FS-UAE, or execute the
interpreter. A successful AROS boot and GFA BASIC launch smoke test remain
required before runtime observations can be claimed.

If AROS proves insufficient for a specific behavior, a legally held Amiga
Kickstart/AmigaOS setup may be evaluated separately. Keep any such ROM files
private; do not commit them or commercial disk images to this public
repository. Keep all observations labelled with the exact interpreter version,
OS image, emulator profile, and media provenance.
