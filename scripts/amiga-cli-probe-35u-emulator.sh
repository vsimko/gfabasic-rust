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
# Identify the running interpreter from the editor's own menu.
# SDL button bit 4 is the right mouse button. Keep it held until after
# SCREENSHOT returns; releasing it closes the menu.
printf "SEND_MOUSE\\t0\\t0\\t4\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/interpreter-version-menu.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
python3 - "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/interpreter-version-menu.png" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/interpreter-version-menu-ocr.txt" <<'PY'
from PIL import Image, ImageOps, ImageEnhance
import sys
image = Image.open(sys.argv[1]).convert("L")
crop = image.crop((0, 0, min(image.width, 420), min(image.height, 220)))
crop = ImageOps.autocontrast(crop.resize((crop.width * 3, crop.height * 3)))
ImageEnhance.Contrast(crop).enhance(2).save(sys.argv[1].replace(".png", "-upper-left.png"))
PY
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/interpreter-version-menu-upper-left.png" stdout --psm 6 2>/dev/null | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/interpreter-version-menu-ocr.txt"
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
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-selected.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

# The selected-row screenshot places the pointer near x=320,y=218.
# The Merge gadget center is at screenshot y~=364, not y=420.
# SEND_MOUSE relative Y is scaled by the emulator; six 50-unit steps
# move the pointer to the center of the Merge gadget.
for _ in $(seq 1 6); do
printf "SEND_MOUSE\\t0\\t50\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.15
done
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-gadget-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 1
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
python3 - "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr-input.png" <<'PY'
from PIL import Image, ImageOps, ImageEnhance
import sys
image = Image.open(sys.argv[1]).convert("L")
image = ImageOps.invert(image).resize((image.width * 3, image.height * 3))
ImageEnhance.Contrast(image).enhance(3).save(sys.argv[2])
PY
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr-input.png" stdout --psm 6 2>/dev/null | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr.txt"
if grep -Eiq 'Merge File|Cancel|Parent' "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge-ocr.txt"; then
  echo "MERGE_EDITOR_STATE=FAIL (Merge requester remains open)" | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-validation.txt"
  exit 1
fi
echo "MERGE_EDITOR_STATE=PASS (Merge requester closed)" | tee "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-validation.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" stdout 2>/dev/null > "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut-ocr.txt"

# Click the editor's visible Run gadget rather than relying on the Amiga-key
# shortcut, which can leave the listing open without starting the program.
# After Merge the calibrated pointer is near x=320,y=415 at 1024x768.
# Five bounded horizontal moves plus a short correction reach x~=630; fourteen vertical moves reach y~=65.
for _ in $(seq 1 5); do
printf "SEND_MOUSE\\t240\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
done
printf "SEND_MOUSE\\t40\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
for _ in $(seq 1 14); do
printf "SEND_MOUSE\\t0\\t-50\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.1
done
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/run-gadget-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
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
