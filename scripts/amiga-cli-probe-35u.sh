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
floppy1=$RUNNER_TEMP/gfa-basic-3.52.zip
EOF
export AMIBERRY_LOG="$GITHUB_WORKSPACE/artifacts/amiga-cli-probe/amiberry.log"
export SDL_AUDIODRIVER=dummy

# Fail closed unless the prepared DF0 contains the exact staged listing.
# The editor Merge step is a separate boundary; this proves only the disk input.
mkdir -p artifacts/amiga-cli-probe
staged_listing="tests/interpreter-probes/amiga-3.5u/regression-suite.LST"
prepared_listing="artifacts/amiga-cli-probe/gfatest-from-df0.LST"
expected_hash="$(sha256sum "$staged_listing" | awk '{print $1}')"
printf '%s  %s\\n' "$expected_hash" "$staged_listing" > artifacts/amiga-cli-probe/staged-listing.sha256
xdftool "$RUNNER_TEMP/gfatest.adf" read gfatest.LST "$prepared_listing"
actual_hash="$(sha256sum "$prepared_listing" | awk '{print $1}')"
printf '%s  %s\\n' "$actual_hash" "$prepared_listing" > artifacts/amiga-cli-probe/df0-listing.sha256
if ! cmp -s "$staged_listing" "$prepared_listing"; then
  echo "::error::DF0 gfatest.LST differs from the staged regression fixture; refusing to launch Amiberry."
  diff -u "$staged_listing" "$prepared_listing" || true
  exit 1
fi
echo "DF0_LISTING_BYTE_MATCH=PASS" | tee artifacts/amiga-cli-probe/source-validation.txt

env -u SHELLOPTS xvfb-run -a -s "-screen 0 1024x768x24 +extension GLX" bash scripts/amiga-cli-probe-35u-emulator.sh

if xdftool "$RUNNER_TEMP/gfatest.adf" read FACT.OUT artifacts/amiga-cli-probe/fact-output.txt; then
  cat artifacts/amiga-cli-probe/fact-output.txt
else
  echo "MISSING: FACT.OUT" > artifacts/amiga-cli-probe/fact-output.txt.missing
  : > artifacts/amiga-cli-probe/fact-output.txt
fi

for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done

for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-before-click.png merge-file-row-y215.png after-merge.png after-run-dialog.png after-run.png; do
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

if ! grep -Fq 'BEFORE-MAT-ONE' artifacts/amiga-cli-probe/fact-output.txt; then
  echo "::error::Interpreter did not write BEFORE-MAT-ONE; editor launch or file output is not proven."
  exit 1
fi
echo "MAT_ONE_REACHED=before-operation" | tee -a artifacts/amiga-cli-probe/result.txt

if ! grep -Fq 'AFTER-MAT-ONE' artifacts/amiga-cli-probe/fact-output.txt; then
  echo "::error::MAT ONE did not return to the following statement."
  exit 1
fi
echo "MAT_ONE_RETURNED=PASS" | tee -a artifacts/amiga-cli-probe/result.txt

# GFA-BASIC formats numeric columns with variable-width whitespace.
# Match complete rows while accepting one or more spaces/tabs between values.
for pattern in \
  '(^|[^0-9])1[[:space:]]+0[[:space:]]+0([[:space:]]|$)' \
  '(^|[^0-9])0[[:space:]]+1[[:space:]]+0([[:space:]]|$)' \
  '(^|[^0-9])0[[:space:]]+0[[:space:]]+1([[:space:]]|$)'; do
  if ! grep -Eq "$pattern" artifacts/amiga-cli-probe/fact-output.txt; then
    echo "::error::MAT ONE output does not show the expected identity matrix row ($pattern); inspect raw FACT.OUT."
    exit 1
  fi
done
echo "MAT_ONE_IDENTITY_MATRIX=PASS" | tee -a artifacts/amiga-cli-probe/result.txt
