#!/usr/bin/env bash
set -euo pipefail
cat > "$RUNNER_TEMP/amiga-cli-probe.uae" <<EOF
use_gui=no
cpu_type=68000
chipset=ocs
kickstart_rom_file=$KICKSTART_ROM_PATH
chipmem_size=2
fastmem_size=8
bogomem_size=0
gfx_width=640
gfx_height=512
gfx_fullscreen=false
sound_output=0
floppy0=$RUNNER_TEMP/gfatest.adf
floppy1=$RUNNER_TEMP/gfa-basic-3.041u.adf
EOF
export AMIBERRY_LOG="$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/amiberry.log"
export SDL_AUDIODRIVER=dummy
env -u SHELLOPTS xvfb-run -a -s "-screen 0 1024x768x24 +extension GLX" bash -c '
  set +u
  set -eo pipefail
  amiberry -G -f "$RUNNER_TEMP/amiga-cli-probe.uae" -0 "$RUNNER_TEMP/gfatest.adf" -1 "$RUNNER_TEMP/gfa-basic-3.041u.adf" --log >"$AMIBERRY_LOG" 2>&1 &
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
  sleep 0.5
  printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-row-pointer.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  printf "SEND_MOUSE\\t0\\t0\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
  printf "SEND_MOUSE\\t0\\t0\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.5
  printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/merge-file-selected.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"

  # Left-Amiga-V activates the Merge gadget in the requester.
  printf "SEND_KEY\\t102\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
  printf "SEND_KEY\\t52\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
  printf "SEND_KEY\\t52\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
  printf "SEND_KEY\\t102\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 3
  printf "SCREENSHOT\\t%s\\n" "$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/after-shortcut.png" | timeout 5s nc -U -N "${ipc_socket_path:-}"
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
  printf "SEND_KEY\\t68\\t1\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  sleep 0.2
  printf "SEND_KEY\\t68\\t0\\n" | timeout 5s nc -U -N "${ipc_socket_path:-}"
  # Allow the BASIC runtime and filesystem to finish flushing the
  # interpreter-written output files before requesting emulator exit.
  sleep 3

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
'
xdftool "$RUNNER_TEMP/gfatest.adf" read GFATEST.OUT artifacts/amiga-cli-probe/gfatest-results.txt
cat artifacts/amiga-cli-probe/gfatest-results.txt
for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done
for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-pointer.png merge-file-selected.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run-dialog.png; do
  test -s "artifacts/amiga-cli-probe/$screenshot"
done
# Amiga Kickstart screenshots use bright text on a saturated blue background.
# Normalize/invert/upscale before OCR; default Tesseract silently missed the source text.
python3 - <<'PY'
from PIL import Image, ImageOps, ImageEnhance
image = Image.open("artifacts/amiga-cli-probe/after-merge.png").convert("L")
image = ImageOps.invert(image).resize((image.width * 3, image.height * 3))
image = ImageEnhance.Contrast(image).enhance(3)
image.save("artifacts/amiga-cli-probe/after-merge-ocr-input.png")
PY
tesseract artifacts/amiga-cli-probe/after-merge-ocr-input.png stdout --psm 6 2>/dev/null | tee artifacts/amiga-cli-probe/after-merge-ocr.txt
if ! grep -Fqi 'Merge File' artifacts/amiga-cli-probe/after-merge-ocr.txt && grep -Fqi 'GFATEST' artifacts/amiga-cli-probe/after-merge-ocr.txt; then
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=yes" | tee artifacts/amiga-cli-probe/result.txt
else
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=no" | tee artifacts/amiga-cli-probe/result.txt
  echo "::error::The editor screenshot does not show the merged LST source."
  exit 1
fi

# Validate one interpreter-emitted terminal record for every requested
# fixture. The BASIC program performs each assertion before writing
# PASS/FAIL; the host only validates collection completeness.
python3 scripts/validate_gfatest_results.py \
  --expected-ids tests/interpreter-probes/amiga-3.041u/expected-ids.txt \
  --results artifacts/amiga-cli-probe/gfatest-results.txt
