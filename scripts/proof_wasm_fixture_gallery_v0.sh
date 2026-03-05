#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
STORE_DIR="${OUT_DIR}_store"
BASELINE_ROOT="${OUT_DIR}_baseline"
BASELINE_DIR_A="$BASELINE_ROOT/run1"
BASELINE_DIR_B="$BASELINE_ROOT/run2"
REQUEST_LIST="$OUT_DIR/requests.json"
FIXTURE_SOURCE_DIR="$OUT_DIR/fixture_source_v0"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export SOURCE_DATE_EPOCH
export TZ=UTC

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

rm -rf "$OUT_DIR" "$STORE_DIR" "$BASELINE_ROOT"
mkdir -p "$OUT_DIR" "$FIXTURE_SOURCE_DIR/xetex/tex" "$BASELINE_ROOT"

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

node - "$OUT_DIR" "$BASELINE_ROOT" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');

const outDir = process.argv[2];
const baselineRoot = process.argv[3];

const firstRunShaPath = (name) => path.join(baselineRoot, `${name}_first.sha256`);

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
fs.writeFileSync(firstRunShaPath('labels_v0'), `${labelsShaFirst}\n`);

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
fs.writeFileSync(firstRunShaPath('toc_v0'), `${tocShaFirst}\n`);

const bibSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_capabilities_v0', 'summary.json'), 'utf8'),
);
if (bibSummary?.typed_artifacts?.bib?.present !== true) {
  console.error('FAIL: expected bib typed artifact present after first run');
  process.exit(1);
}
const bibPath = path.join(outDir, 'typeset_demo_capabilities_v0', 'bib_v0.json');
if (!fs.existsSync(bibPath)) {
  console.error('FAIL: expected bib_v0.json artifact after first run');
  process.exit(1);
}
const bibShaFirst = bibSummary.typed_artifacts.bib.artifact_sha256;
if (typeof bibShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(bibShaFirst)) {
  console.error('FAIL: expected bib artifact sha256 in first summary');
  process.exit(1);
}
fs.writeFileSync(firstRunShaPath('bib_v0'), `${bibShaFirst}\n`);

const hyperrefSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_hyperref_probe_v0', 'summary.json'), 'utf8'),
);
if (hyperrefSummary?.typed_artifacts?.hyperref?.present !== true) {
  console.error('FAIL: expected hyperref typed artifact present after first run');
  process.exit(1);
}
const hyperrefPath = path.join(outDir, 'typeset_demo_hyperref_probe_v0', 'hyperref_v0.json');
if (!fs.existsSync(hyperrefPath)) {
  console.error('FAIL: expected hyperref_v0.json artifact after first run');
  process.exit(1);
}
const hyperrefShaFirst = hyperrefSummary.typed_artifacts.hyperref.artifact_sha256;
if (typeof hyperrefShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(hyperrefShaFirst)) {
  console.error('FAIL: expected hyperref artifact sha256 in first summary');
  process.exit(1);
}
fs.writeFileSync(firstRunShaPath('hyperref_v0'), `${hyperrefShaFirst}\n`);
NODE

node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_A"
node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_B"

node - "$BASELINE_DIR_A" "$BASELINE_DIR_B" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const baselineDirA = process.argv[2];
const baselineDirB = process.argv[3];

const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
const readJson = (p) => JSON.parse(fs.readFileSync(p, 'utf8'));

const indexAPath = path.join(baselineDirA, 'index.json');
const indexBPath = path.join(baselineDirB, 'index.json');
const indexABytes = fs.readFileSync(indexAPath);
const indexBBytes = fs.readFileSync(indexBPath);
const indexASha = sha256(indexABytes);
const indexBSha = sha256(indexBBytes);
if (indexASha !== indexBSha) {
  console.error('FAIL: baseline generator must be deterministic (index.json sha mismatch)');
  process.exit(1);
}

const indexA = readJson(indexAPath);
const indexB = readJson(indexBPath);
if (indexA.engine_rev !== indexB.engine_rev || indexA.config_hash !== indexB.config_hash) {
  console.error('FAIL: baseline generator metadata mismatch between reruns');
  process.exit(1);
}
if (typeof indexA.engine_rev !== 'string' || indexA.engine_rev.length !== 40) {
  console.error('FAIL: baseline index missing valid engine_rev pin');
  process.exit(1);
}
if (!/^[0-9a-f]{64}$/.test(indexA.config_hash)) {
  console.error('FAIL: baseline index missing valid config_hash pin');
  process.exit(1);
}

const casesA = Array.isArray(indexA.cases) ? indexA.cases : [];
const casesB = Array.isArray(indexB.cases) ? indexB.cases : [];
if (casesA.length === 0 || casesA.length !== casesB.length) {
  console.error('FAIL: baseline index cases missing or length mismatch');
  process.exit(1);
}

for (const caseEntry of casesA) {
  const caseId = caseEntry.case_id;
  if (typeof caseId !== 'string' || caseId.length === 0) {
    console.error('FAIL: baseline case id invalid');
    process.exit(1);
  }
  const matchB = casesB.find((it) => it.case_id === caseId);
  if (!matchB) {
    console.error(`FAIL: baseline rerun missing case ${caseId}`);
    process.exit(1);
  }

  const filePairs = [
    ['main.xdv.sha256', 'main_xdv'],
    ['main.pdf.sha256', 'main_pdf'],
  ];
  for (const [filename, key] of filePairs) {
    const pathA = path.join(baselineDirA, caseId, filename);
    const pathB = path.join(baselineDirB, caseId, filename);
    const valueA = fs.readFileSync(pathA, 'utf8').trim();
    const valueB = fs.readFileSync(pathB, 'utf8').trim();
    if (valueA !== valueB) {
      console.error(`FAIL: baseline rerun mismatch for ${caseId}/${filename}`);
      process.exit(1);
    }
    if (!/^[0-9a-f]{64}$/.test(valueA)) {
      console.error(`FAIL: baseline file ${caseId}/${filename} must contain sha256`);
      process.exit(1);
    }
    if (caseEntry.artifact_sha256?.[key] !== valueA || matchB.artifact_sha256?.[key] !== valueA) {
      console.error(`FAIL: baseline index artifact sha mismatch for ${caseId}.${key}`);
      process.exit(1);
    }
  }
}

console.log(`PASS: baseline generator deterministic index_sha256 ${indexASha}`);
console.log(`PASS: baseline generator pinned engine_rev ${indexA.engine_rev}`);
console.log(`PASS: baseline generator pinned config_hash ${indexA.config_hash}`);
NODE

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$STORE_DIR" \
TEXLIVE_BASELINE_DIR="$BASELINE_DIR_A" \
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
const bibShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'bib_v0_first.sha256'), 'utf8').trim();
const hyperrefShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'hyperref_v0_first.sha256'), 'utf8').trim();

if (resolvedCount <= 0) {
  console.error('FAIL: expected at least one resolved resource in fixture gallery summaries');
  process.exit(1);
}
const okStatuses = statuses.filter((entry) => entry.status === 'OK');
if (okStatuses.length <= 0) {
  console.error('FAIL: expected at least one OK case in fixture gallery report');
  process.exit(1);
}
for (const status of okStatuses) {
  if (status.baseline_match !== 'MATCH') {
    console.error(`FAIL: expected baseline_match=MATCH for OK case ${status.case_id}`);
    process.exit(1);
  }
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

const bibSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_capabilities_v0', 'summary.json'), 'utf8'),
);
const bibArtifact = bibSummary?.typed_artifacts?.bib;
if (!bibArtifact || bibArtifact.present !== true) {
  console.error('FAIL: expected bib typed artifact present after second run');
  process.exit(1);
}
const bibShaSecond = bibArtifact.artifact_sha256;
if (bibShaSecond !== bibShaFirst) {
  console.error('FAIL: bib_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}

const hyperrefSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_hyperref_probe_v0', 'summary.json'), 'utf8'),
);
const hyperrefArtifact = hyperrefSummary?.typed_artifacts?.hyperref;
if (!hyperrefArtifact || hyperrefArtifact.present !== true) {
  console.error('FAIL: expected hyperref typed artifact present after second run');
  process.exit(1);
}
const hyperrefShaSecond = hyperrefArtifact.artifact_sha256;
if (hyperrefShaSecond !== hyperrefShaFirst) {
  console.error('FAIL: hyperref_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}

const reportCaseSha = report.case_artifact_sha256;
if (!reportCaseSha || typeof reportCaseSha !== 'object') {
  console.error('FAIL: report missing top-level case_artifact_sha256');
  process.exit(1);
}
for (const status of statuses) {
  const caseSha = reportCaseSha[status.case_id];
  if (!caseSha || typeof caseSha !== 'object') {
    console.error(`FAIL: report missing case_artifact_sha256 for ${status.case_id}`);
    process.exit(1);
  }
  if (typeof caseSha.main_xdv !== 'string' || typeof caseSha.main_pdf !== 'string') {
    console.error(`FAIL: report case_artifact_sha256 malformed for ${status.case_id}`);
    process.exit(1);
  }
  if (!caseSha.typed_artifacts || typeof caseSha.typed_artifacts !== 'object') {
    console.error(`FAIL: report case_artifact_sha256 missing typed_artifacts for ${status.case_id}`);
    process.exit(1);
  }
  for (const key of requiredTypedKeys) {
    const value = caseSha.typed_artifacts[key];
    if (!(value === null || (typeof value === 'string' && /^[0-9a-f]{64}$/.test(value)))) {
      console.error(`FAIL: report typed artifact sha invalid for ${status.case_id}.${key}`);
      process.exit(1);
    }
  }
}

console.log(`PASS: resolved_resources_count ${resolvedCount}`);
console.log(`PASS: baseline_match MATCH for all OK cases (${okStatuses.length})`);
console.log(`PASS: typed_artifacts keys ${requiredTypedKeys.join(',')}`);
console.log(`PASS: labels_v0 sha stable ${labelsShaSecond}`);
console.log(`PASS: toc_v0 sha stable ${tocShaSecond}`);
console.log(`PASS: bib_v0 sha stable ${bibShaSecond}`);
console.log(`PASS: hyperref_v0 sha stable ${hyperrefShaSecond}`);
console.log('PASS: report top-level case_artifact_sha256 present');
NODE

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
