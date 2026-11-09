#!/usr/bin/env bash
set -euo pipefail

: "${GFA_ASSETS_TOKEN:?GFA_ASSETS_TOKEN secret is required}"

repo_url="https://github.com/vsimko/gfabasic-private-assets.git"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

auth="$(printf 'x-access-token:%s' "$GFA_ASSETS_TOKEN" | base64 -w0)"
git -c "http.extraheader=AUTHORIZATION: basic $auth" clone --depth 1 "$repo_url" "$work_dir/repo"
cd "$work_dir/repo"

target="analysis/amiga-3.52/generated"
mkdir -p "$target" analysis/amiga-3.52/annotations
find "$target" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
cp -a "$GITHUB_WORKSPACE/analysis-output/." "$target/"

cat > analysis/amiga-3.52/README.md <<'EOF'
# GFA BASIC 3.52 Amiga interpreter analysis

Generated from the privately held 3.52 executable by the gfabasic-rust
workflow. This is reverse-engineering evidence for version 3.52, not a
verified specification for the project's normative GFA BASIC 3.5 target.

- generated/ contains reproducible Ghidra exports, including decompiled C.
- annotations/ is reserved for durable human/AI annotations and should not
  be overwritten by generated runs.
- The original executable remains at the repository root as GFABasic.
EOF

if [ -z "$(find analysis/amiga-3.52/annotations -mindepth 1 -print -quit)" ]; then
  cat > analysis/amiga-3.52/annotations/README.md <<'EOF'
# Analysis annotations

Keep curated function names, command mappings, behavioral observations, and
documentation discrepancies here. Generated Ghidra output lives separately
under ../generated/ and may be replaced by later workflow runs.
EOF
fi

git add analysis/amiga-3.52
if git diff --cached --quiet; then
  echo "Private analysis repository already contains the current generated output."
  exit 0
fi

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git commit -m "Update generated GFA BASIC 3.52 reverse-engineering data"
git -c "http.extraheader=AUTHORIZATION: basic $auth" push origin HEAD:main
