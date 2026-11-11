---
type: "Research Note"
title: "Amiga runtime workflow OCR audit"
---

# OCR use in the Amiga runtime workflow

Issue: [#440](https://github.com/vsimko/gfabasic-rust/issues/440)

## Scope and inspected implementation

The active workflow is [`.github/workflows/amiga-native-merge-run.yml`](../../.github/workflows/amiga-native-merge-run.yml). Its predecessor, `amiga-cli-probe.yml`, was introduced in PR #343 and was superseded by the native 3.52 Merge + Run workflow in PR #382. The current workflow is the relevant implementation to maintain; the predecessor shows how the current OCR checks evolved.

The active workflow installs:

- `tesseract-ocr` to recognize text from emulator screenshots;
- `python3-pil` to normalize the Amiga display screenshot before recognition;
- `xvfb` and Amiberry to run and capture the GUI without a physical display.

The workflow uses OCR in two places:

1. `after-shortcut.png` is passed to Tesseract and saved as `after-shortcut-ocr.txt`. This is diagnostic evidence only; the OCR output is not used in a pass/fail condition.
2. `after-merge.png` is converted to grayscale, inverted, enlarged 3×, and contrast-enhanced. Tesseract writes `after-merge-ocr.txt`. The workflow then gates the job on a text heuristic: the output must not contain `Merge File` and must contain `FACT(1)` and `FACT(2)`. This is the only OCR-dependent pass/fail check in the active workflow.

The earlier PR #343 workflow used the same two screenshot/OCR stages and the same basic post-merge heuristic. The current workflow additionally validates the interpreter-generated result files with structured text checks and `scripts/validate_gfatest_results.py`.

## What is already verified without OCR

The workflow controls Amiberry through its Unix-socket IPC:

- `INSERTFLOPPY` and `LIST_FLOPPIES` provide structured disk-control evidence.
- `SEND_KEY` and `SEND_MOUSE` drive the editor interaction.
- `SCREENSHOT` captures GUI states, but the image itself is not structured evidence.
- After the run, `xdftool` extracts `FACT.OUT` and `GFATEST.OUT` from the ADF.
- A host-side grep validates the expected FACT output, and `validate_gfatest_results.py` checks the expected regression record IDs and results.

These file-based assertions are stronger than screen OCR for proving that the guest program executed and emitted expected results. They do **not** prove that the editor's Merge action loaded the intended listing: the result file alone cannot distinguish a successful merge from another way the program might have become available in the guest environment.

The Amiberry IPC interface, as used here, can capture screenshots and send input, but the workflow does not use it to retrieve the editor's text buffer or a structured “merge completed” event. The available implementation therefore has no demonstrated non-OCR machine-readable replacement for the post-merge GUI assertion.

## Assessment

| Check | Current evidence | OCR necessary? |
|---|---|---|
| Emulator startup and disk insertion | Amiberry process, IPC replies, disk listing, log | No |
| Editor navigation and input | IPC commands and screenshots | No for input delivery; screenshots remain visual diagnostics |
| Textual listing merged into editor | Post-merge screenshot plus OCR heuristic | No equivalent structured assertion currently demonstrated |
| BASIC program execution and output | Guest-written `FACT.OUT` and `GFATEST.OUT`, validated on host | No |
| Post-run dialog screenshot | Screenshot retained as artifact | No OCR assertion |
| `after-shortcut.png` text extraction | OCR text artifact only | Not required for pass/fail |

OCR is inherently a weaker signal than the guest-written result files: recognition can vary with display scaling, palette, font, screenshot timing, and OCR engine version. The current heuristic is also indirect: the absence of the requester title plus two source tokens is not a formal proof that the complete intended listing was merged. Conversely, removing it today would remove the only automated assertion of the editor-side merge state.

## Recommendation

**Retain OCR narrowly for the post-merge editor-state check for now; do not remove it from the workflow yet.** Keep the interpreter-written output and regression records as the authoritative execution assertions. Keep `after-shortcut-ocr.txt` diagnostic-only, or remove that OCR invocation if its text is not useful during failure analysis.

A future replacement should expose a deterministic guest-side signal for the merge operation (for example, a saved editor buffer or an independently verified state/result file) and validate it without relying on screen text. Until such a signal is demonstrated, keep the post-merge OCR assertion explicitly classified as a GUI smoke-test heuristic—not language conformance evidence. Do not let it establish Amiga 3.5 behavior: the workflow exercises the available 3.52 interpreter only.

## Evidence reviewed

- Current workflow: `.github/workflows/amiga-native-merge-run.yml`.
- Historical predecessor: PR [#343](https://github.com/vsimko/gfabasic-rust/pull/343).
- Current workflow integration: PR [#382](https://github.com/vsimko/gfabasic-rust/pull/382).
- Runtime scope and version limitation: [Amiga emulator experiment lab](amiga-emulator-lab.md) and [3.52 runtime baseline](amiga-gfa-basic-352-runtime-baseline.md).

This is a workflow implementation audit, not an execution experiment. No workflow was changed and no new emulator run was performed.
