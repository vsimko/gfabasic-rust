---
type: "Research Note"
title: "Amiga runtime workflow OCR audit"
---

# OCR use in the Amiga runtime workflows

Issue: [#440](https://github.com/vsimko/gfabasic-rust/issues/440)

## Scope and inspected implementation

The repository currently contains two Amiga emulator workflows that install OCR tooling:

- [`.github/workflows/amiga-native-merge-run.yml`](../../.github/workflows/amiga-native-merge-run.yml) is the native 3.52 Merge + Run workflow integrated in PR #382.
- [`.github/workflows/amiga-cli-probe.yml`](../../.github/workflows/amiga-cli-probe.yml) is the earlier CLI/regression and tokenized-Save workflow introduced in PR #343. It is **not removed or disabled**: it still runs for matching pull-request path changes and supports `workflow_dispatch`. Treat it as a separately triggerable workflow, not merely historical source.

The repository's [`.github/workflows/amiga-reference.yml`](../../.github/workflows/amiga-reference.yml) does not install or invoke OCR.

### Native Merge + Run workflow

The native workflow installs:

- `tesseract-ocr` to recognize text from emulator screenshots;
- `python3-pil` to normalize the Amiga display screenshot before recognition;
- `xvfb` and Amiberry to run and capture the GUI without a physical display.

It uses OCR in two places:

1. `after-shortcut.png` is passed to Tesseract and saved as `after-shortcut-ocr.txt`. This is diagnostic evidence only; the OCR output is not used in a pass/fail condition.
2. `after-merge.png` is converted to grayscale, inverted, enlarged 3×, and contrast-enhanced. Tesseract writes `after-merge-ocr.txt`. The workflow then gates the job on a text heuristic: the output must not contain `Merge File` and must contain `FACT(1)` and `FACT(2)`. This is the only OCR-dependent pass/fail check in the native workflow.

### Earlier CLI probe workflow and scripts

The still-triggerable `amiga-cli-probe.yml` workflow also installs `tesseract-ocr` and `python3-pil`. Its called scripts divide the work as follows:

- `scripts/ci/amiga-cli-probe.sh` captures the editor/merge/run/save screenshots and drives the Amiberry UI. It contains an OCR diagnostic comment but does not itself invoke Tesseract.
- `scripts/ci/validate-amiga-cli-probe.sh` loops over every captured PNG, converts it to grayscale, inverts and enlarges it 3×, applies contrast enhancement, then invokes Tesseract to create sidecar transcripts under `artifacts/amiga-cli-probe/ocr/`.
- OCR failures in this validator are explicitly diagnostic-only; they do not fail the job. The authoritative assertions are the guest-written `FACT.OUT`, the tokenized `.GFA` file, and the Direct Mode save result.

Thus, the legacy workflow's OCR use is broader in screenshot coverage but has **no OCR-dependent pass/fail assertion**. Its Tesseract/Pillow packages currently support artifact diagnostics only.

## What is already verified without OCR

The workflows control Amiberry through its Unix-socket IPC:

- `INSERTFLOPPY` and `LIST_FLOPPIES` provide structured disk-control evidence.
- `SEND_KEY` and `SEND_MOUSE` drive editor interaction.
- `SCREENSHOT` captures GUI states, but the image itself is not structured evidence.
- The validators use `xdftool` to extract guest-written result files and host-side checks to validate their contents and saved-file properties.

These file-based assertions are stronger than screen OCR for proving that the guest program executed and emitted expected results. They do **not** prove that the editor's Merge action loaded the intended listing: result files alone cannot distinguish a successful merge from another way the program might have become available in the guest environment.

The Amiberry IPC interface, as used here, can capture screenshots and send input, but neither workflow retrieves the editor's text buffer or a structured “merge completed” event. No non-OCR machine-readable replacement for the native workflow's post-merge GUI assertion has been demonstrated.

## Assessment

| Workflow/check | Current evidence | OCR necessary? |
|---|---|---|
| Native emulator startup and disk insertion | Amiberry process, IPC replies, disk listing, log | No |
| Native editor navigation and input | IPC commands and screenshots | No for input delivery; screenshots remain visual diagnostics |
| Native textual listing merged into editor | Post-merge screenshot plus OCR heuristic | No equivalent structured assertion currently demonstrated |
| Native BASIC execution and output | Guest-written result files validated on host | No |
| Native `after-shortcut.png` text extraction | OCR text artifact only | Not required for pass/fail |
| Legacy CLI probe screenshot transcripts | OCR sidecars for all captured PNGs | Not required for pass/fail; diagnostic only |
| Reference workflow | No OCR installation or invocation found | No |

OCR is inherently a weaker signal than guest-written result files: recognition can vary with display scaling, palette, font, screenshot timing, and OCR engine version. The native post-merge heuristic is indirect: absence of the requester title plus two source tokens is not formal proof that the complete intended listing was merged. Conversely, removing it today would remove the only automated assertion of the editor-side merge state.

## Recommendation

**Retain OCR narrowly for the native post-merge editor-state check for now.** Keep interpreter-written output and regression records as the authoritative execution assertions.

The legacy CLI probe's OCR is diagnostic-only. Its invocation can be removed independently if its sidecar transcripts are not useful during failure analysis; doing so would also allow removing its Tesseract/Pillow installation. Do not remove those packages from the native workflow while its post-merge heuristic remains. This distinction avoids treating all OCR use as equally necessary.

A future replacement for the native assertion should expose a deterministic guest-side signal for the merge operation (for example, a saved editor buffer or an independently verified state/result file) and validate it without relying on screen text. Until such a signal is demonstrated, keep the post-merge OCR assertion explicitly classified as a GUI smoke-test heuristic—not language conformance evidence. Neither workflow establishes Amiga 3.5 behavior: the available runtime is 3.52.

## Evidence reviewed

- Native workflow: `.github/workflows/amiga-native-merge-run.yml`.
- Earlier still-triggerable workflow: `.github/workflows/amiga-cli-probe.yml`.
- Native and legacy scripts: `scripts/ci/amiga-cli-probe.sh` and `scripts/ci/validate-amiga-cli-probe.sh`.
- Reference workflow: `.github/workflows/amiga-reference.yml`.
- Historical integration: PRs [#343](https://github.com/vsimko/gfabasic-rust/pull/343) and [#382](https://github.com/vsimko/gfabasic-rust/pull/382).
- Runtime scope and version limitation: [Amiga emulator experiment lab](amiga-emulator-lab.md) and [3.52 runtime baseline](amiga-gfa-basic-352-runtime-baseline.md).

This is a workflow implementation audit, not an execution experiment. No workflow was changed and no new emulator run was performed.
