#!/usr/bin/env bash
set -euo pipefail
cat > "$RUNNER_TEMP/amiga-35u-ab-c.uae" <<EOF
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
export AMIBERRY_LOG="$GITHUB_WORKSPACE/artifacts/amiga-35u-ab-c/amiberry.log"
export SDL_AUDIODRIVER=dummy
env -u SHELLOPTS xvfb-run -a -s "-screen 0 1024x768x24 +extension GLX" bash scripts/amiga-35u-ab-c-emulator.sh

for name in FACT.OUT DIM.OUT MAT.OUT; do
  if xdftool "$RUNNER_TEMP/gfatest.adf" read "$name" "artifacts/amiga-35u-ab-c/$name"; then
    echo "OUTPUT_PRESENT=$name"
  else
    echo "OUTPUT_MISSING=$name" | tee -a artifacts/amiga-35u-ab-c/output-status.txt
  fi
done

# A and B must complete and be durably closed before C starts.
grep -Fxq 'CASE-A:BEFORE' artifacts/amiga-35u-ab-c/FACT.OUT
grep -Fxq 'CASE-A:HELLO' artifacts/amiga-35u-ab-c/FACT.OUT
grep -Fxq 'CASE-B:DIM-PASS' artifacts/amiga-35u-ab-c/DIM.OUT
! grep -Fq 'CASE-B:DIM-FAIL' artifacts/amiga-35u-ab-c/DIM.OUT
echo "CASE_A_BASELINE=PASS" | tee artifacts/amiga-35u-ab-c/result.txt
echo "CASE_B_DIM_ARRAY=PASS" | tee -a artifacts/amiga-35u-ab-c/result.txt

if test -s artifacts/amiga-35u-ab-c/MAT.OUT && grep -Fxq 'CASE-C:MAT-ONE-PASS' artifacts/amiga-35u-ab-c/MAT.OUT; then
  echo "CASE_C_MAT_ONE=PASS" | tee -a artifacts/amiga-35u-ab-c/result.txt
elif test -s artifacts/amiga-35u-ab-c/MAT.OUT && grep -Fq 'CASE-C:MAT-ONE-FAIL' artifacts/amiga-35u-ab-c/MAT.OUT; then
  echo "CASE_C_MAT_ONE=FAIL (statement returned but matrix was incorrect)" | tee -a artifacts/amiga-35u-ab-c/result.txt
  exit 1
else
  tesseract artifacts/amiga-35u-ab-c/after-run.png stdout --psm 6 2>/dev/null > artifacts/amiga-35u-ab-c/after-run-ocr.txt || true
  if grep -Eiq 'task held|finish ALL disk activity' artifacts/amiga-35u-ab-c/after-run-ocr.txt; then
    echo "CASE_C_MAT_ONE=TASK_HELD_CONFIRMED" | tee -a artifacts/amiga-35u-ab-c/result.txt
  elif grep -Eiq 'Illegal instruction|B-Trap|F-line|exception' artifacts/amiga-35u-ab-c/amiberry.log; then
    echo "CASE_C_MAT_ONE=EMULATOR_EXCEPTION" | tee -a artifacts/amiga-35u-ab-c/result.txt
  else
    echo "CASE_C_MAT_ONE=NO_OUTPUT_UNCLASSIFIED" | tee -a artifacts/amiga-35u-ab-c/result.txt
  fi
  # A missing MAT.OUT alone is not proof of a task-held requester or cause.
fi

for screenshot in editor-before-merge.png merge-dialog.png merge-file-row-pointer.png merge-file-selected.png after-shortcut.png merge-gadget-pointer.png after-merge.png after-run-dialog.png after-run.png; do
  test -s "artifacts/amiga-35u-ab-c/$screenshot"
done
