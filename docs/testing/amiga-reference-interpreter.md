# Original Amiga interpreter reference asset

The selected implementation dialect is Amiga GFA-BASIC 3.5. The separately
retained 3.52 executable is research material only; observations must remain
version-scoped and must not be promoted to 3.5 conformance.

## Reference-analysis workflow

The manual
[Prepare and analyze Amiga GFA-BASIC reference interpreter](../../.github/workflows/amiga-reference.yml)
workflow downloads the approved private 3.52 executable, verifies its recorded
identity, runs the configured Ghidra analysis, and validates the generated
analysis metadata. It does not execute the BASIC interpreter.

The workflow can read the private asset only through the repository secret
`GFA_ASSETS_TOKEN`. Its exact token scope and expiry are maintainer-managed
security facts; do not infer them from the workflow file. The same token is
used for private publication only when the private publication input is
explicitly enabled.

### Independent publication gates

Both workflow-dispatch inputs default to `false` and control different
destinations:

- `publish_private_analysis` gates the private-assets publisher. When enabled,
  it pushes generated metadata to `vsimko/gfabasic-private-assets` and creates
  or updates the private release. This path requires the separate human
  authorization tracked by [#598](https://github.com/vsimko/gfabasic-rust/issues/598).
- `publish_public_analysis` gates public analysis-artifact staging/upload and
  the separate `publish-public-analysis` job that creates or updates the
  public branch/PR. This job alone receives public-repository
  `contents: write` and `pull-requests: write`; it does not receive
  `GFA_ASSETS_TOKEN`. Public publication remains subject to the maintainer
  decision tracked by [#585](https://github.com/vsimko/gfabasic-rust/issues/585)
  and the workflow-hardening work in [#599](https://github.com/vsimko/gfabasic-rust/issues/599).

The two inputs are independent: enabling one does not enable the other. With
both inputs false, the analysis/download/validation path can still run, but
the private publisher step, public artifact upload, and public branch/PR job
are skipped by their respective conditions. This behavior is covered by
static workflow tests; no private-media workflow was dispatched for this
change.

The analysis job has only public-repository `contents: read` permission and
does not persist checkout credentials. The public publication job has the
minimum repository write permissions required for its branch/PR operation.
The public artifact is an allowlisted metadata package; it must not contain
the commercial executable, ROM/disk images, or Ghidra decompiler output such
as reconstructed C source.

Do not enable either publication input until its corresponding maintainer
authorization is recorded in the linked human-blocker issue. A successful
static validation does not itself grant publication permission.

## Native interpreter Merge + Run workflow

The separate
[Amiga native Merge + Run workflow](../../.github/workflows/amiga-native-merge-run.yml)
boots the selected interpreter build in Amiberry, merges an ASCII listing
through the native editor, and runs the resulting program. It is distinct
from the Ghidra reference-analysis workflow above.

Native observations must identify the exact executable build, emulator
configuration, and individual probe outcomes. A successful 3.52 or 3.5U run
does not establish behavior for the selected non-U 3.5 interpreter. Physical
Amiga hardware is not required for the documented emulator test path.

The native workflow uses private interpreter media and a licensed Kickstart
image. Keep those assets private; retained diagnostics and screenshots must
not include the original commercial binaries or ROM images. The outstanding
rights/ROM authorization and private asset questions remain tracked in
[#42](https://github.com/vsimko/gfabasic-rust/issues/42) and
[#585](https://github.com/vsimko/gfabasic-rust/issues/585).
