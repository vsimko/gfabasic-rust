#!/usr/bin/env bash
set -euo pipefail

# Validate screenshots, interpreter output, and the native Save result.
for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done
for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-pointer.png merge-file-selected.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run.png save-menu-pointer.png save-dialog.png save-button-pointer.png after-save.png; do
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
if grep -Eiq 'FACT[(]1[)]|FACT[(]2[)]' artifacts/amiga-cli-probe/after-merge-ocr.txt && grep -Fqi 'PRINT #1' artifacts/amiga-cli-probe/after-merge-ocr.txt; then
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=yes" | tee artifacts/amiga-cli-probe/result.txt
else
  echo "EDITOR_MERGE_LST_CONTENT_VISIBLE=no" | tee artifacts/amiga-cli-probe/result.txt
  echo "::error::The editor screenshot does not show the merged LST source."
  exit 1
fi

# Verify the interpreter executed the full regression listing and wrote
# its labelled results to the guest disk before checking tokenized Save.
xdftool artifacts/amiga-cli-probe/factzero.adf read FACT.OUT artifacts/amiga-cli-probe/FACT.OUT
grep -Fq 'FACT(0..2)' artifacts/amiga-cli-probe/FACT.OUT
grep -Fq 'ARITHMETIC:' artifacts/amiga-cli-probe/FACT.OUT
grep -Fq 'FOR-NEXT:' artifacts/amiga-cli-probe/FACT.OUT
grep -Fq 'STRINGS:' artifacts/amiga-cli-probe/FACT.OUT

# Verify tokenized Save created a non-empty .GFA on the ADF.
xdftool artifacts/amiga-cli-probe/factzero.adf list / all > artifacts/amiga-cli-probe/adf-after-save-list.txt
grep -Eiq 'factzero[.]GFA' artifacts/amiga-cli-probe/adf-after-save-list.txt
xdftool artifacts/amiga-cli-probe/factzero.adf read factzero.GFA artifacts/amiga-cli-probe/factzero.GFA
test -s artifacts/amiga-cli-probe/factzero.GFA
if cmp -s artifacts/amiga-cli-probe/factzero.LST artifacts/amiga-cli-probe/factzero.GFA; then
  echo "::error::The saved .GFA is byte-identical to the ASCII .LST."
  exit 1
fi
file artifacts/amiga-cli-probe/factzero.GFA | tee artifacts/amiga-cli-probe/gfa-file-info.txt
sha256sum artifacts/amiga-cli-probe/factzero.GFA > artifacts/amiga-cli-probe/gfa-sha256.txt
echo "TOKENIZED_GFA_SAVED=yes" | tee artifacts/amiga-cli-probe/save-result.txt
