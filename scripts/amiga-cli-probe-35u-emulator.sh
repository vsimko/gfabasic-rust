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
# Dialog placement varies between emulator sessions. Capture the current
# pointer and OCR the actual gfatest.LST row instead of assuming fixed Y.
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-uncalibrated.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
python3 - "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-uncalibrated.png" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/mouse-correction.txt" <<'PY'
import csv
import subprocess
import sys
from PIL import Image

image_path, output_path = sys.argv[1:]
image = Image.open(image_path).convert("RGB")
pixels = image.load()
red_x = []
red_y = []
for y in range(image.height):
    for x in range(image.width):
        red, green, blue = pixels[x, y]
        if red > 150 and red > green * 1.4 and red > blue * 1.4:
            red_x.append(x)
            red_y.append(y)
if not red_x:
    raise SystemExit("Could not locate the red Amiberry mouse pointer")
# The Amiga pointer hotspot is the arrow tip at the top-left, not its center.
pointer_x = min(red_x)
pointer_y = min(red_y)

tsv = subprocess.check_output(
    ["tesseract", image_path, "stdout", "--psm", "6", "tsv"],
    text=True,
)
words = list(csv.DictReader(tsv.splitlines(), delimiter="\t"))
matches = [w for w in words if "lst" in w["text"].lower()]
if not matches:
    raise SystemExit("OCR could not locate the LST filename extension in the Merge dialog")
# Tesseract splits gfatest.LST into two words on this screen. Match the
# distinctive LST token, not the larger GFATEST: path field at the bottom.
target = max(matches, key=lambda w: float(w["conf"]))
target_x = int(target["left"]) + int(target["width"]) / 2
target_y = int(target["top"]) + int(target["height"]) / 2
# Aim the arrow-tip hotspot directly at the center of the filename row.
# SEND_MOUSE has different effective scales on the two axes:
# X is approximately 0.5, while Y is approximately 0.25.
raw_dx = round((target_x - pointer_x) * 2)
raw_dy = round((target_y - pointer_y) * 4)
with open(output_path, "w", encoding="ascii") as output:
    output.write(f"{raw_dx} {raw_dy}\n")
print(
    f"Merge row center=({target_x:.1f},{target_y:.1f}), "
    f"pointer=({pointer_x:.1f},{pointer_y:.1f}), "
    f"raw correction=({raw_dx},{raw_dy})"
)
PY
read mouse_dx mouse_dy < "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/mouse-correction.txt"
while (( mouse_dx != 0 || mouse_dy != 0 )); do
  step_x=$(( mouse_dx > 240 ? 240 : mouse_dx < -240 ? -240 : mouse_dx ))
  step_y=$(( mouse_dy > 240 ? 240 : mouse_dy < -240 ? -240 : mouse_dy ))
  printf "SEND_MOUSE\\t%s\\t%s\\t0\\n" "$step_x" "$step_y" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  mouse_dx=$(( mouse_dx - step_x ))
  mouse_dy=$(( mouse_dy - step_y ))
  sleep 0.15
done
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-target.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
# Double-click the filename using the Amiga pointer arrow-tip hotspot.
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.10
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.5
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-double-clicked.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"


# The selected-row screenshot places the pointer near x=320,y=218.
# Move to the center of the Merge gadget (about x=323,y=420).
# Amiberry bounds each relative mouse event; use repeated small increments.
# SEND_MOUSE relative Y is scaled by the emulator; eight 50-unit steps
# move the pointer from the selected row to the Merge gadget center.
for _ in $(seq 1 8); do
printf "SEND_MOUSE\\t0\\t50\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.15
done
sleep 0.3
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-gadget-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 0.2
printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
sleep 4
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
tesseract "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" stdout 2>/dev/null > "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut-ocr.txt"
printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-merge.png" | timeout 5s nc -U -N "${ipc_socket_path:-}" || true

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
