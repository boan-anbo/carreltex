#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
STORE_DIR="${OUT_DIR}_store"
BASELINE_DIR="${OUT_DIR}_baseline"
REQUEST_LIST="$OUT_DIR/requests.json"
FIXTURE_SOURCE_DIR="$OUT_DIR/fixture_source_v0"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export SOURCE_DATE_EPOCH
export TZ=UTC

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

rm -rf "$OUT_DIR" "$STORE_DIR" "$BASELINE_DIR"
mkdir -p "$OUT_DIR" "$FIXTURE_SOURCE_DIR/xetex/tex"

printf 'fixture-bytes-for-typeset-minimal-v0\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_minimal_v0"

cat > "$REQUEST_LIST" <<'JSON'
{
  "version": 1,
  "requests": [
    { "kind": "texmf", "format": "tex", "name": "typeset_demo_minimal_v0", "variant": "typeset" },
    { "kind": "texmf", "format": "tex", "name": "missing_demo_case", "variant": "ok" }
  ]
}
JSON

TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$REQUEST_LIST" "$STORE_DIR"

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$STORE_DIR" \
node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"

node - "$OUT_DIR" "$BASELINE_DIR" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');

const outDir = process.argv[2];
const baselineDir = process.argv[3];

const matchCase = 'typeset_demo_minimal_v0';
const missingCase = 'ok_demo_v0';
const matchSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, matchCase, 'summary.json'), 'utf8'),
);

fs.mkdirSync(path.join(baselineDir, matchCase), { recursive: true });
fs.writeFileSync(
  path.join(baselineDir, matchCase, 'main.xdv.sha256'),
  `${matchSummary.artifact_sha256.main_xdv}\n`,
);
fs.writeFileSync(
  path.join(baselineDir, matchCase, 'main.pdf.sha256'),
  `${matchSummary.artifact_sha256.main_pdf}\n`,
);

fs.mkdirSync(path.join(baselineDir, missingCase), { recursive: true });
fs.writeFileSync(
  path.join(baselineDir, missingCase, 'main.xdv.sha256'),
  '0'.repeat(64) + '\n',
);
NODE

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$STORE_DIR" \
TEXLIVE_BASELINE_DIR="$BASELINE_DIR" \
node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"

if [[ ! -s "$OUT_DIR/report.json" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/report.json" >&2
  exit 1
fi

node - "$OUT_DIR" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');

const outDir = process.argv[2];
const report = JSON.parse(fs.readFileSync(path.join(outDir, 'report.json'), 'utf8'));
const statuses = Array.isArray(report.statuses) ? report.statuses : [];
const resolvedCount = Number(report.resolved_resources_count ?? 0);

if (resolvedCount <= 0) {
  console.error('FAIL: expected at least one resolved resource in fixture gallery summaries');
  process.exit(1);
}
const baselineMatches = statuses.filter((entry) => entry.baseline_match === 'MATCH').length;
const baselineMissing = statuses.filter((entry) => entry.baseline_match === 'MISSING').length;
if (baselineMatches <= 0) {
  console.error('FAIL: expected at least one baseline_match=MATCH');
  process.exit(1);
}
if (baselineMissing <= 0) {
  console.error('FAIL: expected at least one baseline_match=MISSING');
  process.exit(1);
}

console.log(`PASS: resolved_resources_count ${resolvedCount}`);
console.log(`PASS: baseline_match MATCH=${baselineMatches} MISSING=${baselineMissing}`);
NODE

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
