---
name: amiberry-emulator-control
description: Operate, configure, automate, and diagnose the Amiberry emulator used by this repository's Amiga CI harnesses. Use for emulator lifecycle, UAE configuration, IPC, input injection, screenshots, and emulator-level failures; use amiga-gfa-basic-regression-testing for GFA BASIC fixtures and language-test semantics.
---

# Amiberry emulator control

This skill covers the emulator boundary only. It does not define how to write GFA BASIC probes, interpret language behavior, or validate GFATEST results. For those tasks, follow [amiga-gfa-basic-regression-testing](../amiga-gfa-basic-regression-testing/SKILL.md) and its canonical probe guide.

## Required context

1. Read `AGENTS.md` and follow the repository agent workflow.
2. Inspect the exact workflow and shell/Python harness being used; do not assume all Amiberry jobs share one launch mode or disk layout.
3. Prefer the repository's pinned/known-good invocation and configuration over inventing new emulator flags.
4. Treat emulator version, host OS, display backend, chipset, CPU, memory, ROM, and mounted media as part of the reproducibility record.

## Emulator lifecycle

- Run headless sessions under the workflow's established virtual display (currently `xvfb-run` in the CI harness) and use the configured SDL audio backend when required.
- Capture stdout/stderr to a persistent log. Preserve the exact launch command, exit status, Amiberry version, and effective UAE configuration.
- Track the emulator PID and install an EXIT cleanup trap that terminates and waits for the process. Do not leave background emulator processes between tests.
- Wait for the IPC socket to appear, then verify that it responds. Socket creation can precede readiness; retry short, idempotent setup/query requests with a bounded timeout.
- After actions that may restart or recreate the emulator IPC endpoint, verify the old socket and rediscover it if necessary. Never assume the original socket path remains valid.
- Send a graceful `QUIT` when possible, then ensure the process has exited. Use forced termination only as cleanup fallback.

## Configuration and media

- Keep UAE configuration explicit and reproducible: CPU, chipset, Kickstart ROM path, chip/fast/bogo memory, display dimensions, sound, and floppy paths.
- Obtain ROMs and commercial interpreter media only through the repository's private-asset process. Verify expected size and checksums where the workflow provides them.
- Keep original distribution media separate from writable test media when the test requires it. Record drive assignments (DF0/DF1) and the guest-visible paths expected by the program.
- Inspect prepared ADF contents and bootability with the repository's existing `amitools/xdftool` workflow before launching Amiberry.
- Never upload or commit commercial binaries, Kickstart ROMs, or disk images. Follow workflow artifact exclusions and licensing boundaries.

## IPC and input automation

The repository currently uses Amiberry's Unix-domain socket (typically named `amiberry.sock`) and tab-delimited newline-terminated commands. The small helper `scripts/amiberry-ipc.py` validates `OK`/ `ERROR` responses and retries startup-empty responses. Prefer that helper when the workflow uses it; otherwise follow the exact tested transport in the harness.

Commands observed in this repository harness include:

- `INSERTFLOPPY <path> <drive>`, `LIST_FLOPPIES`, and `EJECT_FLOPPY <drive>`
- `SCREENSHOT <path>`
- `SEND_KEY <raw-keycode> <pressed-state>`
- `SEND_MOUSE <dx> <dy> <button-state>`
- `QUIT`

Use only commands confirmed by the installed Amiberry IPC implementation/version or an existing successful repository run. Do not assume this list is a complete or stable public IPC API.

- Raw Amiga keycodes, modifier combinations, mouse deltas, and GUI coordinates are harness-specific. Reuse calibrated values only with the same resolution, display scaling, requester state, and Amiberry version.
- Inject key-down and key-up separately, with bounded delays where the guest application needs them. Always release modifiers and buttons even after an error.
- Prefer semantic IPC operations over mouse-coordinate automation. When GUI interaction is unavoidable, take screenshots before and after each significant action and verify the visible state before proceeding.
- Never treat OCR alone as proof that a requester or editor state is correct; inspect screenshots when OCR is ambiguous.

## Failure isolation

Diagnose in layers and preserve evidence before changing automation:

1. **Host setup:** Amiberry installed, version captured, virtual display available, ROM readable, expected config present.
2. **Process:** launch command correct, process alive, log has no immediate fatal error.
3. **IPC:** socket exists, command receives a valid response, socket remains live after state-changing actions.
4. **Media:** requested drive contains the intended image; inspect floppy listing and ADF layout/boot block.
5. **Guest UI:** capture screenshots around boot, requester/editor transitions, and completion. Check focus, resolution, scaling, and whether a requester is still open.
6. **Guest application:** only after emulator and UI state are established, hand off to the regression-testing skill to diagnose GFA BASIC execution and result files.

Do not classify a missing guest output file as an emulator or interpreter defect without locating the failed layer. Keep raw logs, IPC replies, screenshots, ROM/media hashes, and workflow run identifiers together.

## Validation and reporting

- Re-run the smallest workflow/job that exercises the changed emulator behavior, then the relevant full native workflow.
- Report the exact Amiberry version, host/display mode, UAE configuration, media/drive setup, and workflow run.
- Distinguish observed behavior from assumptions. A successful boot or screenshot does not establish successful GFA BASIC execution.
- Keep timing waits bounded and explain any empirically calibrated delay; avoid arbitrary sleeps when a state/IPC check can be used.

## Boundaries

- Do not bypass private-asset access controls or source ROMs/interpreters from unapproved locations.
- Do not commit licensed ROMs, commercial software, or disk images.
- Do not broaden this skill into GFA BASIC test authoring, manual research, or GFATEST protocol rules; those belong to the linked skills and canonical docs.
