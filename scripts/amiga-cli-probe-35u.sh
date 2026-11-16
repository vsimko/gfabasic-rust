#!/usr/bin/env bash
set -euo pipefail
cat > "$RUNNER_TEMP/amiga-cli-probe.uae" <<EOF
use_gui=no
cpu_type=68020/68881
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
floppy1=$RUNNER_TEMP/gfa-basic-3.5u.adf
EOF
export AMIBERRY_LOG="$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/amiberry.log"
export SDL_AUDIODRIVER=dummy
env -u SHELLOPTS xvfb-run -a -s "-screen 0 1024x768x24 +extension GLX" bash scripts/amiga-cli-probe-35u-emulator.sh
if xdftool "$RUNNER_TEMP/gfatest.adf" read FACT.OUT artifacts/amiga-cli-probe/fact-output.txt; then
  cat artifacts/amiga-cli-probe/fact-output.txt
else
  echo "MISSING: FACT.OUT" > artifacts/amiga-cli-probe/fact-output.missing
  : > artifacts/amiga-cli-probe/fact-output.txt
fi
if xdftool "$RUNNER_TEMP/gfatest.adf" read GFATEST.OUT artifacts/amiga-cli-probe/gfatest-results.txt; then
  cat artifacts/amiga-cli-probe/gfatest-results.txt
else
  echo "MISSING: GFATEST.OUT" > artifacts/amiga-cli-probe/gfatest-results.missing
  : > artifacts/amiga-cli-probe/gfatest-results.txt
fi
for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done
for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-uncalibrated.png merge-file-row-target.png merge-file-double-clicked.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run-dialog.png after-run.png; do
  test -s "artifacts/amiga-cli-probe/$screenshot"
done
python3 - <<'PY'
from PIL import Image, ImageOps, ImageEnhance
image = Image.open("artifacts/amiga-cli-probe/after-merge.png").convert("L")
image = ImageOps.invert(image).resize((image.width * 3, image.height * 3))
image = ImageEnhance.Contrast(image).enhance(3)
image.save("artifacts/amiga-cli-probe/after-merge-ocr-input.png")
PY
if tesseract artifacts/amiga-cli-probe/after-merge-ocr-input.png stdout --psm 6 2>/dev/null | tee artifacts/amiga-cli-probe/after-merge-ocr.txt; then
  echo "AFTER_MERGE_OCR=available"
else
  echo "AFTER_MERGE_OCR=unavailable (diagnostic only)" | tee artifacts/amiga-cli-probe/after-merge-ocr.txt
fi
echo "EDITOR_MERGE_OCR_CAPTURED=yes (diagnostic only)" | tee artifacts/amiga-cli-probe/result.txt

if ! grep -Fq 'LAUNCH-CONTROL:FACT:PASS' artifacts/amiga-cli-probe/fact-output.txt; then
  echo "::error::Editor Run did not write the FACT.OUT launch-control marker."
  exit 1
fi
echo "LAUNCH_CONTROL_FACT_OUTPUT=PASS" | tee -a artifacts/amiga-cli-probe/result.txt

if ! grep -Fq 'GFATEST:launch-control:PASS' artifacts/amiga-cli-probe/gfatest-results.txt; then
  echo "::error::Editor Run did not write the GFATEST.OUT launch-control marker."
  exit 1
fi
echo "LAUNCH_CONTROL_GFATEST_OUTPUT=PASS" | tee -a artifacts/amiga-cli-probe/result.txt
