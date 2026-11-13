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
floppy1=$RUNNER_TEMP/gfa-basic-3.5u.adf
EOF
export AMIBERRY_LOG="$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/amiberry.log"
export SDL_AUDIODRIVER=dummy
env -u SHELLOPTS xvfb-run -a -s "-screen 0 1024x768x24 +extension GLX" bash scripts/amiga-cli-probe-35u-emulator.sh
xdftool "$RUNNER_TEMP/gfatest.adf" read FACT.OUT artifacts/amiga-cli-probe/fact-output.txt
xdftool "$RUNNER_TEMP/gfatest.adf" read GFATEST.OUT artifacts/amiga-cli-probe/gfatest-results.txt
cat artifacts/amiga-cli-probe/fact-output.txt
cat artifacts/amiga-cli-probe/gfatest-results.txt
for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done
for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-pointer.png merge-file-selected.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run-dialog.png after-run.png; do
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
if ! grep -Fqi 'Merge File' artifacts/amiga-cli-probe/after-merge-ocr.txt && grep -Fqi 'FACT(1)' artifacts/amiga-cli-probe/after-merge-ocr.txt && grep -Fqi 'FACT(2)' artifacts/amiga-cli-probe/after-merge-ocr.txt; then
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=yes" | tee artifacts/amiga-cli-probe/result.txt
else
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=no" | tee artifacts/amiga-cli-probe/result.txt
  echo "::error::The editor screenshot does not show the merged LST source."
  exit 1
fi

# Assert the interpreter-written file, independent of screen OCR.
if grep -Eq 'FACT\(0\.\.2\):[[:space:]]*1[[:space:]]*1[[:space:]]*2' artifacts/amiga-cli-probe/fact-output.txt; then
  echo "FACT_PROGRAM_OUTPUT=1 1 2" | tee -a artifacts/amiga-cli-probe/result.txt
else
  echo "::error::FACT.OUT does not contain the expected output (1 1 2)."
  exit 1
fi

# Validate one interpreter-emitted terminal record for every requested
# fixture. The BASIC program performs each assertion before writing
# PASS/FAIL; the host only validates collection completeness.
python3 scripts/validate_gfatest_results.py \
  --expected-ids tests/interpreter-probes/amiga-3.5u/expected-ids.txt \
  --results artifacts/amiga-cli-probe/gfatest-results.txt
