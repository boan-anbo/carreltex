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

const labelsSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_labels_probe_v0', 'summary.json'), 'utf8'),
);
if (labelsSummary?.typed_artifacts?.labels?.present !== true) {
  console.error('FAIL: expected labels typed artifact present after first run');
  process.exit(1);
}
const labelsPath = path.join(outDir, 'typeset_demo_labels_probe_v0', 'labels_v0.json');
if (!fs.existsSync(labelsPath)) {
  console.error('FAIL: expected labels_v0.json artifact after first run');
  process.exit(1);
}
const labelsShaFirst = labelsSummary.typed_artifacts.labels.artifact_sha256;
if (typeof labelsShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(labelsShaFirst)) {
  console.error('FAIL: expected labels artifact sha256 in first summary');
  process.exit(1);
}
fs.writeFileSync(path.join(baselineDir, 'labels_v0_first.sha256'), `${labelsShaFirst}\n`);

const tocSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'summary.json'), 'utf8'),
);
if (tocSummary?.typed_artifacts?.toc?.present !== true) {
  console.error('FAIL: expected toc typed artifact present after first run');
  process.exit(1);
}
const tocPath = path.join(outDir, 'typeset_demo_toc_probe_v0', 'toc_v0.json');
if (!fs.existsSync(tocPath)) {
  console.error('FAIL: expected toc_v0.json artifact after first run');
  process.exit(1);
}
const tocShaFirst = tocSummary.typed_artifacts.toc.artifact_sha256;
if (typeof tocShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(tocShaFirst)) {
  console.error('FAIL: expected toc artifact sha256 in first summary');
  process.exit(1);
}
fs.writeFileSync(path.join(baselineDir, 'toc_v0_first.sha256'), `${tocShaFirst}\n`);
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
const requiredTypedKeys = ['toc', 'labels', 'bib', 'hyperref'];
const labelsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'labels_v0_first.sha256'), 'utf8').trim();
const tocShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'toc_v0_first.sha256'), 'utf8').trim();

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

for (const status of statuses) {
  const typedPresence = status.typed_artifacts_presence;
  if (!typedPresence || typeof typedPresence !== 'object') {
    console.error(`FAIL: case ${status.case_id} missing typed_artifacts_presence`);
    process.exit(1);
  }
  for (const key of requiredTypedKeys) {
    if (typeof typedPresence[key] !== 'boolean') {
      console.error(`FAIL: case ${status.case_id} missing typed_artifacts_presence.${key} boolean`);
      process.exit(1);
    }
  }
  const summaryPath = path.join(outDir, status.case_id, 'summary.json');
  const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
  const typedArtifacts = summary.typed_artifacts;
  if (!typedArtifacts || typeof typedArtifacts !== 'object') {
    console.error(`FAIL: case ${status.case_id} missing typed_artifacts`);
    process.exit(1);
  }
  for (const key of requiredTypedKeys) {
    const payload = typedArtifacts[key];
    if (!payload || typeof payload !== 'object') {
      console.error(`FAIL: case ${status.case_id} missing typed_artifacts.${key}`);
      process.exit(1);
    }
    if (typeof payload.present !== 'boolean' || typeof payload.items !== 'number') {
      console.error(`FAIL: case ${status.case_id} typed_artifacts.${key} schema mismatch`);
      process.exit(1);
    }
  }
}

const labelsSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_labels_probe_v0', 'summary.json'), 'utf8'),
);
const labelsArtifact = labelsSummary?.typed_artifacts?.labels;
if (!labelsArtifact || labelsArtifact.present !== true) {
  console.error('FAIL: expected labels typed artifact present after second run');
  process.exit(1);
}
const labelsShaSecond = labelsArtifact.artifact_sha256;
if (labelsShaSecond !== labelsShaFirst) {
  console.error('FAIL: labels_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}

const tocSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'summary.json'), 'utf8'),
);
const tocArtifact = tocSummary?.typed_artifacts?.toc;
if (!tocArtifact || tocArtifact.present !== true) {
  console.error('FAIL: expected toc typed artifact present after second run');
  process.exit(1);
}
const tocShaSecond = tocArtifact.artifact_sha256;
if (tocShaSecond !== tocShaFirst) {
  console.error('FAIL: toc_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}

console.log(`PASS: resolved_resources_count ${resolvedCount}`);
console.log(`PASS: baseline_match MATCH=${baselineMatches} MISSING=${baselineMissing}`);
console.log(`PASS: typed_artifacts keys ${requiredTypedKeys.join(',')}`);
console.log(`PASS: labels_v0 sha stable ${labelsShaSecond}`);
console.log(`PASS: toc_v0 sha stable ${tocShaSecond}`);
NODE

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
