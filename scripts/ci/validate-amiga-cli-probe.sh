#!/usr/bin/env bash
set -euo pipefail

# OCR is diagnostic evidence only. File-system assertions below are the
# authoritative checks for interpreter output and native Save.
for diagnostic in insert-floppy-ipc.txt list-floppies-ipc.txt stdin-save.LST; do
  if test -f "amiga-volume/$diagnostic"; then
    cp "amiga-volume/$diagnostic" "artifacts/amiga-cli-probe/$diagnostic"
  else
    printf "MISSING: %s\\n" "$diagnostic" > "artifacts/amiga-cli-probe/$diagnostic.missing"
  fi
done
for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-pointer.png merge-file-selected.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run.png save-dialog.png after-first-save.png; do
  test -s "artifacts/amiga-cli-probe/$screenshot"
done

# Keep every original screenshot and create a sidecar OCR transcript for it.
# Preprocessing is intentionally diagnostic: OCR failures never fail the job.
mkdir -p artifacts/amiga-cli-probe/ocr
for screenshot in artifacts/amiga-cli-probe/*.png; do
  [ -f "$screenshot" ] || continue
  name="$(basename "$screenshot" .png)"
  python3 - "$screenshot" "artifacts/amiga-cli-probe/ocr/$name-ocr-input.png" <<'PY'
import sys
from PIL import Image, ImageOps, ImageEnhance
image = Image.open(sys.argv[1]).convert("L")
image = ImageOps.invert(image).resize((image.width * 3, image.height * 3))
ImageEnhance.Contrast(image).enhance(3).save(sys.argv[2])
PY
  if tesseract "artifacts/amiga-cli-probe/ocr/$name-ocr-input.png" "artifacts/amiga-cli-probe/ocr/$name" --psm 6 2>/dev/null; then
    :
  else
    printf 'OCR failed for %s (diagnostic only)\\n' "$screenshot" > "artifacts/amiga-cli-probe/ocr/$name.txt"
  fi
done
find artifacts/amiga-cli-probe/ocr -maxdepth 1 -type f -name '*-ocr-input.png' -delete
find artifacts/amiga-cli-probe/ocr -maxdepth 1 -type f -name '*.txt' -print | sort > artifacts/amiga-cli-probe/ocr/index.txt

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
