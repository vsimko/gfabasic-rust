#!/usr/bin/env bash
# Emulator interaction only. Keep this as a standalone script so shell syntax
# is parsed once (no nested bash -c quoting inside the workflow YAML).
set +u
set -eo pipefail
amiberry -G -f "$RUNNER_TEMP/amiga-cli-probe.uae" -0 "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/factzero.adf" --log >"$AMIBERRY_LOG" 2>&1 &
emulator_pid=$!
cleanup() { kill "$emulator_pid" 2>/dev/null || true; wait "$emulator_pid" 2>/dev/null || true; }
trap cleanup EXIT
ipc_socket_path=""
for attempt in $(seq 1 45); do
  ipc_socket_path="$(find /run/user -type s -name amiberry.sock -print -quit 2>/dev/null || true)"
  if [ -n "$ipc_socket_path" ]; then break; fi
  sleep 1
done
test -n "$ipc_socket_path" || { cat "$AMIBERRY_LOG"; exit 1; }
sleep 35
# Insert and verify the test ADF through Amiberry IPC.
printf "INSERTFLOPPY\\t%s\\t0\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/factzero.adf" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/insert-floppy-ipc.txt"
printf "LIST_FLOPPIES\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/list-floppies-ipc.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/editor-before-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# Amiga raw keycode 0x51 (decimal 81) is F2, the Editor Merge command.
printf "SEND_KEY\\t81\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.3
printf "SEND_KEY\\t81\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# SEND_MOUSE uses relative dx/dy and the observed cursor scaling is
# about +61 screen px for dx=380 and +35 px for dy=70.
# Run #117 ended at (260,145), short of the factzero.LST row center
# (335,238). Two bounded (230,93) steps should cover the remaining
# (+75,+93); verify the pointer screenshot before clicking.
for _ in 1 2 3; do
  printf "SEND_MOUSE\\t380\\t70\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
done
for _ in 1 2; do
  printf "SEND_MOUSE\\t230\\t93\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
done
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-selected.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# AmigaOS requester shortcut: Left-Amiga-V activates the left gadget (Merge).
printf "SEND_KEY\\t102\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t52\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t52\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t102\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 1
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
# OCR is collected uniformly as a non-gating diagnostic after emulator shutdown.
# OCR can misread the requester title ("Merge File" as "Merge Abbruch"),
# so do not gate the fallback click on exact title recognition.
# The first calibrated move landed at (~260,360), on the left
# edge of the requester. From there the Merge gadget center is
# (~323,420); measured scale is ~0.34 screen px per IPC unit.
for _ in 1 2; do
  printf "SEND_MOUSE\\t-177\\t183\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
done
printf "SEND_MOUSE\\t206\\t198\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-gadget-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true

# Keep the known-good socket if it is still live. Merge may recreate
# Emulator IPC endpoint; rediscover it only when the original is gone.
if [ ! -S "${ipc_socket_path:-}" ]; then
  printf 'IPC_SOCKET_STALE_AFTER_MERGE=%s\\n' "${ipc_socket_path:-<empty>}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/ipc-after-merge.txt"
  ipc_socket_path="$(find /run /tmp -type s -name amiberry.sock -print -quit 2>/dev/null || true)"
fi
printf 'IPC_SOCKET_PATH_FOR_RUN=%s\\n' "${ipc_socket_path:-<empty>}" >>"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/ipc-after-merge.txt"
test -S "${ipc_socket_path:-}" || { echo 'Amiberry IPC socket unavailable after Merge' >&2; exit 1; }

# Execute the merged regression listing (Shift+F10), then dismiss
# the interpreter return/stop dialog with F10.
printf "SEND_KEY\\t96\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t89\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t89\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t96\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-run.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_KEY\\t68\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t68\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 1
if kill -0 "$emulator_pid" 2>/dev/null; then
  echo "EMULATOR_ALIVE_AFTER_MERGE=yes"
else
  echo "EMULATOR_ALIVE_AFTER_MERGE=no"
fi
if test -S "${ipc_socket_path:-}"; then
  echo "IPC_SOCKET_PRESENT_AFTER_MERGE=yes"
else
  echo "IPC_SOCKET_PRESENT_AFTER_MERGE=no"
fi
printf "LIST_FLOPPIES\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/post-merge-ipc.txt" || true

# Native editor shortcut: Shift+F1 opens the tokenized GFA Save requester.
# The floppy directory can take three to five seconds to populate; wait
# five seconds before capturing it and activating the default Save gadget.
printf "SEND_KEY\\t96\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t80\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t80\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t96\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/save-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
# Return/Enter is Amiga raw keycode 0x44 (decimal 68).
printf "SEND_KEY\\t68\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t68\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-save.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
kill -0 "$emulator_pid"
printf "QUIT\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
