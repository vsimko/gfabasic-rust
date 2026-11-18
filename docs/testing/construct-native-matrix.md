---
type: "Testing Guide"
title: "Construct-scoped Amiga GFA-BASIC native matrix"
---

# Construct-scoped Amiga GFA-BASIC native matrix

The `amiga-construct-matrix.yml` workflow selects construct/version pairs from each
construct's declarative `manifest.yaml`. Each selected pair is dispatched as a separate
matrix job through the matching native harness for the exact interpreter version
(3.041U, 3.5, or 3.52). The manifest is input only; native run records are written to
`tests/language-constructs/<construct>/results/<version>/`.

## Selection

- Pull requests compare the PR base and head with the repository selector.
- Pushes to `main` compare the before and after commits.
- Manual dispatch supports `validate-only`, `all`, and one `construct` (optionally
  restricted to one exact version).
- A changed construct manifest schedules every exact version declared by that manifest,
  including versions whose applicability/expectation is unknown or not-applicable.
  Those fields describe knowledge of behavior, not whether a version is tested.
- A changed shared probe schedules every version mapped to that probe. A changed
  version-specific probe schedules only its mapped version.
- Changes limited to generated result outputs do not schedule native jobs and must not
  create a self-triggering test loop.
- The existing `amiga-reference-matrix.yml` workflow remains the whole-suite baseline
  when it is dispatched directly or called without a selected version.

## Trust and licensed assets

The selector and manifest validation run without private credentials. Native jobs are
allowed only for a same-repository owner pull request, an owner-triggered main-branch
run, or an owner manual dispatch. Fork PRs and other untrusted events do not receive
`GFA_ASSETS_TOKEN`; their applicable matrix rows are reported as `UNTESTED`.

The preflight does not read the token; it is used only in the existing private-asset download steps. It is not printed,
written to artifacts, or passed to probe programs. The token's actual repository scope
still requires maintainer verification in issue #583. The reference and media-inventory
workflows tracked by #585 remain outside this matrix.

Commercial interpreter executables, Kickstart ROMs, and generated ADF disks must never
be included in uploaded artifacts. Artifacts retain only result records, screenshots,
OCR text, logs, and non-binary metadata.

## Result semantics

Each construct/version result records two independent dimensions:

- **Harness:** whether the emulator, editor, merge/run interaction, and identity evidence
  completed successfully.
- **Conformance:** one of `PASS`, `FAIL`, `DIFFERENT`, or `UNTESTED`.

`PASS` requires a successful harness, a screenshot of the running editor's version
menu, matching version OCR, and a complete GFATEST result for the construct ID.
`FAIL` is a valid interpreter-emitted failure record from an identified version.
`DIFFERENT` is an explicit interpreter-emitted, verified version-difference record.
`UNTESTED` covers unknown/not-applicable versions, skipped jobs, unavailable workflows
or licensed assets, incomplete harness runs, missing results, and uncertain editor
identity. It is never treated as a pass.

### Durable per-version outputs

Each job writes its own result record under
`tests/language-constructs/<construct>/results/<version>/`. The record is separate
from the manifest and captures at least the manifest revision/digest, probe revision
or digest, exact editor identity evidence, workflow/run URL, harness status,
conformance status, observed behavior, raw GFATEST output, and references to retained
diagnostics. Preserve FAIL, DIFFERENT, UNTESTED, and harness-failure observations;
a negative result is data, not a reason to skip that version. Do not commit commercial
interpreter binaries, ROMs, disk images, or other licensed payloads.

The aggregate summary is `PASS` only when every reported row is PASS or DIFFERENT;
any UNTESTED row makes the summary INCOMPLETE, and any FAIL makes it FAIL. Raw GFATEST
output and editor-identity evidence remain in the version output and CI artifacts.

## Local validation

```sh
python scripts/validate_language_constructs.py validate
python -m unittest discover -s scripts/tests -p 'test_*.py'
python scripts/validate_language_constructs.py select --all --format json
```
