#!/usr/bin/env bash
set +u
set -eo pipefail
set +u
set -eo pipefail
amiberry -G -f "$RUNNER_TEMP/amiga-cli-probe.uae" -0 "$RUNNER_TEMP/gfatest.adf" -1 "$RUNNER_TEMP/gfa-basic-3.5u.adf" --log >"$AMIBERRY_LOG" 2>&1 &
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
# Turbo mode removes emulated floppy latency for these unprotected ADF test disks.
printf "SET_FLOPPY_SPEED\\t0\\n" | timeout 5s nc -U -N "$ipc_socket_path" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-set.txt"
printf "GET_FLOPPY_SPEED\\n" | timeout 5s nc -U -N "$ipc_socket_path" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-get.txt"
cat "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-set.txt"
cat "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-get.txt"
grep -Eq "(^|[^0-9])0([^0-9]|$)" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-get.txt" || { echo "Amiberry did not confirm turbo floppy speed"; exit 1; }
sleep 35
# Verify that the test disk (DF0) and untouched distribution disk (DF1) are mounted.
printf "INSERTFLOPPY\\t%s\\t0\\n" "$RUNNER_TEMP/gfatest.adf" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/insert-floppy-ipc.txt"
printf "LIST_FLOPPIES\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/list-floppies-ipc.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/editor-before-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# F2 opens the native ASCII Merge requester (GFA BASIC editor shortcut).
printf "SEND_KEY\\t81\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t81\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# DF0 is already selected. Move the pointer from the top-left corner
# to the visible gfatest.LST row (about x=340,y=211 in the screenshot).
# Amiberry applies a bounded relative delta per IPC event, so split motion.
for _ in $(seq 1 4); do
printf "SEND_MOUSE\\t240\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
done
printf "SEND_MOUSE\\t0\\t176\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_MOUSE\\t0\\t112\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_MOUSE\\t0\\t112\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-selected.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# The LST row is selected. Move down to the Merge gadget and click it.
for _ in $(seq 1 3); do
printf "SEND_MOUSE\\t0\\t112\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
done
printf "SEND_MOUSE\\t0\\t48\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_MOUSE\\t0\\t48\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-gadget-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.3
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" stdout 2>/dev/null > "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut-ocr.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true

tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" stdout 2>/dev/null > "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut-ocr.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true

# Native editor Run shortcut: Shift+F10 (Amiga raw keycodes).
printf "SEND_KEY\\t96\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t89\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t89\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t96\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-run-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
# Capture the running editor/runtime after invoking Run. Output mode "O"
# should create the DF0 result files without an interactive requester.
sleep 8
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-run.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
# Allow the BASIC runtime and filesystem to finish flushing the
# interpreter-written output files before requesting emulator exit.
sleep 5

kill -0 "$emulator_pid"
printf "QUIT\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
# QUIT is asynchronous. Wait for Amiberry to close the floppy
# cleanly; the EXIT trap is only a timeout fallback.
for attempt in $(seq 1 10); do
if ! kill -0 "$emulator_pid" 2>/dev/null; then break; fi
sleep 1
done
if kill -0 "$emulator_pid" 2>/dev/null; then
echo "Amiberry did not exit after QUIT; cleanup will terminate it."
else
wait "$emulator_pid" || true
fi
