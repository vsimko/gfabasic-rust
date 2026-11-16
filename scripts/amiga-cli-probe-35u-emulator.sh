#!/usr/bin/env bash
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
python3 scripts/amiberry-ipc.py "$ipc_socket_path" SET_FLOPPY_SPEED 0 | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-set.txt"
python3 scripts/amiberry-ipc.py "$ipc_socket_path" GET_FLOPPY_SPEED | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-get.txt"
grep -Eq "(^|[^0-9])0([^0-9]|$)" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/floppy-speed-get.txt" || { echo "Amiberry did not confirm turbo floppy speed"; exit 1; }
sleep 35
printf "INSERTFLOPPY\\t%s\\t0\\n" "$RUNNER_TEMP/gfatest.adf" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/insert-floppy-ipc.txt"
printf "LIST_FLOPPIES\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" >"$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/list-floppies-ipc.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/editor-before-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_KEY\\t81\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_KEY\\t81\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
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
# The file row is at the fixed Amiga screen coordinate Y=215 px.
# Keep the existing X position: the pointer is already over gfatest.LST.
# SEND_MOUSE relative Y is scaled by approximately 0.25.
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-before-click.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
python3 - "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-before-click.png" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/mouse-y-correction.txt" <<'PY'
from PIL import Image
import sys

image = Image.open(sys.argv[1]).convert("RGB")
pixels = image.load()
red_y = []
for y in range(image.height):
    for x in range(image.width):
        red, green, blue = pixels[x, y]
        if red > 150 and red > green * 1.4 and red > blue * 1.4:
            red_y.append(y)
if not red_y:
    raise SystemExit("Could not locate the red Amiberry mouse pointer")
pointer_y = min(red_y)
raw_dy = round((215 - pointer_y) * 4)
with open(sys.argv[2], "w", encoding="ascii") as output:
    output.write(str(raw_dy) + chr(10))
print(f"Keeping X unchanged; pointer Y={pointer_y}, target Y=215, raw Y correction={raw_dy}")
PY
read mouse_dy < "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/mouse-y-correction.txt"
if (( mouse_dy != 0 )); then
  printf "SEND_MOUSE\\t0\\t%s\\t0\\n" "$mouse_dy" | timeout 5s nc -U -N "${ipc_socket_path:-}"
fi
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-y215.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
# Double-click gfatest.LST at the existing X position and fixed Y=215 px.
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 1
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" stdout 2>/dev/null > "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr.txt"

# OCR is not proof of source identity, but a recognized contradictory token is
# sufficient reason to stop. In particular, do not run when MAT SET is shown
# for the M O / MAT ONE probe; preserve the screenshot for manual inspection.
if grep -Eiq 'MAT[[:space:]]+SET' "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr.txt"; then
  echo "MERGE_EDITOR_TOKEN_CONTRADICTION=MAT_SET" | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-validation.txt"
  echo "Editor OCR shows MAT SET for the MAT ONE abbreviation probe; refusing to run an unverified listing."
  exit 1
fi
echo "MERGE_EDITOR_OCR_NO_KNOWN_CONTRADICTION=PASS (not proof of source identity)" | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-validation.txt"

# Run the merged listing with Shift+F10 (Amiga raw keycodes: LShift=0x60,
# F10=0x59). Press the modifier first and release it last.
printf "SEND_KEY\\t96\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_KEY\\t89\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_KEY\\t89\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
printf "SEND_KEY\\t96\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-run-dialog.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 8
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-run.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 5
kill -0 "$emulator_pid"
printf "QUIT\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true
for attempt in $(seq 1 10); do
if ! kill -0 "$emulator_pid" 2>/dev/null; then break; fi
sleep 1
done
if kill -0 "$emulator_pid" 2>/dev/null; then
echo "Amiberry did not exit after QUIT; cleanup will terminate it."
else
wait "$emulator_pid" || true
fi
