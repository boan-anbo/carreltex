#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
STORE_DIR="${OUT_DIR}_store"
HINT_STORE_DIR_A="${OUT_DIR}_store_from_hints_a"
HINT_STORE_DIR_B="${OUT_DIR}_store_from_hints_b"
BASELINE_ROOT="${OUT_DIR}_baseline"
BASELINE_DIR_A="$BASELINE_ROOT/run1"
BASELINE_DIR_B="$BASELINE_ROOT/run2"
BASELINE_PACKS_ROOT="${BASELINE_ROOT}/packs"
BASELINE_AUTO_PACK_DIR="${BASELINE_PACKS_ROOT}/auto_pack"
REQUEST_LIST="$OUT_DIR/requests.json"
HINT_REQUEST_LIST_A="$OUT_DIR/request_list_from_hints_a.json"
HINT_REQUEST_LIST_B="$OUT_DIR/request_list_from_hints_b.json"
COMBINED_REQUEST_LIST_A="$OUT_DIR/requests_combined_from_hints_a.json"
COMBINED_REQUEST_LIST_B="$OUT_DIR/requests_combined_from_hints_b.json"
FIXTURE_SOURCE_DIR="${OUT_DIR}_fixture_source_v0"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export SOURCE_DATE_EPOCH
export TZ=UTC

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

rm -rf "$OUT_DIR" "$STORE_DIR" "$HINT_STORE_DIR_A" "$HINT_STORE_DIR_B" "$FIXTURE_SOURCE_DIR" "$BASELINE_ROOT"
mkdir -p \
  "$OUT_DIR" \
  "$FIXTURE_SOURCE_DIR/xetex/tex" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/sections" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/chapters" \
  "$FIXTURE_SOURCE_DIR/xetex/bib" \
  "$FIXTURE_SOURCE_DIR/xetex/bst" \
  "$FIXTURE_SOURCE_DIR/xetex/png" \
  "$FIXTURE_SOURCE_DIR/xetex/pdf" \
  "$FIXTURE_SOURCE_DIR/xetex/sty" \
  "$FIXTURE_SOURCE_DIR/xetex/cls" \
  "$FIXTURE_SOURCE_DIR/fontconfig/public" \
  "$BASELINE_ROOT" \
  "$BASELINE_PACKS_ROOT"

printf 'fixture-bytes-for-typeset-minimal-v0\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_minimal_v0"
printf 'fixture-bytes-for-chapter-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_intro.tex"
printf 'fixture-bytes-for-chapter-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_appendix.tex"
printf 'fixture-bytes-for-chapters-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__intro.tex"
printf 'fixture-bytes-for-chapters-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__appendix.tex"
printf 'fixture-bytes-for-sections-intro-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/intro.tex"
printf 'fixture-bytes-for-chapters-ch1-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters/ch1.tex"
printf 'fixture-bytes-for-sections-intro-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__intro.tex"
printf 'fixture-bytes-for-chapters-ch1-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__ch1.tex"
printf 'fixture-bytes-for-demo-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/demo.png"
printf 'fixture-bytes-for-probe-figure-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/probe-figure.png"
printf 'fixture-bytes-for-figs-diagram-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__diagram.pdf"
printf 'fixture-bytes-for-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/refs.bib"
printf 'fixture-bytes-for-styleprobe-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/styleprobe_refs.bib"
printf 'fixture-bytes-for-multiadd-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multiadd_refs.bib"
printf 'fixture-bytes-for-multibib-a-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multibib_a.bib"
printf 'fixture-bytes-for-multibib-b-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multibib_b.bib"
printf 'fixture-bytes-for-legacyrefs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/legacyrefs.bib"
printf 'fixture-bytes-for-bib-deep-refs-local-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/bib__deep__refs-local.bib"
printf 'fixture-bytes-for-legacy-deeprefs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/legacy__deeprefs.bib"
printf 'fixture-bytes-for-plain-bst\n' > "$FIXTURE_SOURCE_DIR/xetex/bst/plain.bst"
printf 'fixture-bytes-for-xcolor-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/xcolor.sty"
printf 'fixture-bytes-for-natbib-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/natbib.sty"
printf 'fixture-bytes-for-memoir-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/memoir.cls"
printf 'fixture-bytes-for-found-sans\n' > "$FIXTURE_SOURCE_DIR/fontconfig/public/FoundSans"

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

node "$ROOT_DIR/scripts/texlive_smoke/request_list_from_hints_v0.mjs" \
  "$OUT_DIR/report.json" \
  "$HINT_REQUEST_LIST_A"
node "$ROOT_DIR/scripts/texlive_smoke/request_list_from_hints_v0.mjs" \
  "$OUT_DIR/report.json" \
  "$HINT_REQUEST_LIST_B"

node - "$HINT_REQUEST_LIST_A" "$HINT_REQUEST_LIST_B" <<'NODE'
const fs = require('node:fs');
const crypto = require('node:crypto');

const requestListAPath = process.argv[2];
const requestListBPath = process.argv[3];
const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
const isSha256 = (value) => typeof value === 'string' && /^[0-9a-f]{64}$/.test(value);
const isSafeToken = (value) => typeof value === 'string' && value.length > 0 && !value.includes('/') && !value.includes('\\') && !value.includes('..');

const bytesA = fs.readFileSync(requestListAPath);
const bytesB = fs.readFileSync(requestListBPath);
const shaA = sha256(bytesA);
const shaB = sha256(bytesB);
if (shaA !== shaB) {
  console.error('FAIL: request_list_from_hints_v0 output must be deterministic');
  process.exit(1);
}

const listA = JSON.parse(bytesA.toString('utf8'));
const listB = JSON.parse(bytesB.toString('utf8'));
if (listA.version !== 1 || listB.version !== 1) {
  console.error('FAIL: request list version must be 1');
  process.exit(1);
}
if (listA.request_count !== listA.requests.length || listB.request_count !== listB.requests.length) {
  console.error('FAIL: request_count must match requests length');
  process.exit(1);
}
if (listA.typed_artifacts_version !== 1 || listB.typed_artifacts_version !== 1) {
  console.error('FAIL: request list typed_artifacts_version must be 1');
  process.exit(1);
}
if (!isSha256(listA.source_report_sha256) || !isSha256(listB.source_report_sha256)) {
  console.error('FAIL: request list source_report_sha256 missing');
  process.exit(1);
}
if (listA.source_report_sha256 !== listB.source_report_sha256) {
  console.error('FAIL: request list source_report_sha256 mismatch across reruns');
  process.exit(1);
}
const graphicsRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'png' && request.name === 'demo.png' && request.variant === 'typeset',
);
if (!graphicsRequest) {
  console.error('FAIL: request list must include graphics hint request for demo.png');
  process.exit(1);
}
const fontRequest = listA.requests.find(
  (request) => request.kind === 'fontconfig' && request.format === 'name' && request.name === 'FoundSans' && request.variant === 'public',
);
if (!fontRequest) {
  console.error('FAIL: request list must include font hint request for fontconfig public/FoundSans');
  process.exit(1);
}
const inputRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'chapter_intro.tex' && request.variant === 'typeset',
);
if (!inputRequest) {
  console.error('FAIL: request list must include input hint request for chapter_intro.tex');
  process.exit(1);
}
const includeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'chapter_appendix.tex' && request.variant === 'typeset',
);
if (!includeRequest) {
  console.error('FAIL: request list must include include hint request for chapter_appendix.tex');
  process.exit(1);
}
const packageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'xcolor.sty' && request.variant === 'typeset',
);
if (!packageRequest) {
  console.error('FAIL: request list must include package hint request for xcolor.sty');
  process.exit(1);
}
const bibRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'refs.bib' && request.variant === 'typeset',
);
if (!bibRequest) {
  console.error('FAIL: request list must include bib hint request for refs.bib');
  process.exit(1);
}
const nestedInputRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'chapters__intro.tex' && request.variant === 'typeset',
);
if (!nestedInputRequest) {
  console.error('FAIL: request list must include nested input hint request for chapters__intro.tex');
  process.exit(1);
}
const nestedIncludeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'chapters__appendix.tex' && request.variant === 'typeset',
);
if (!nestedIncludeRequest) {
  console.error('FAIL: request list must include nested include hint request for chapters__appendix.tex');
  process.exit(1);
}
const inputProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'sections__intro.tex' && request.variant === 'typeset',
);
if (!inputProbeRequest) {
  console.error('FAIL: request list must include input probe hint request for sections__intro.tex');
  process.exit(1);
}
const includeProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'chapters__ch1.tex' && request.variant === 'typeset',
);
if (!includeProbeRequest) {
  console.error('FAIL: request list must include include probe hint request for chapters__ch1.tex');
  process.exit(1);
}
const graphicsOptsRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'figs__diagram.pdf' && request.variant === 'typeset',
);
if (!graphicsOptsRequest) {
  console.error('FAIL: request list must include includegraphics ext/dir hint request for figs__diagram.pdf');
  process.exit(1);
}
const nestedBibRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'bib__deep__refs-local.bib' && request.variant === 'typeset',
);
if (!nestedBibRequest) {
  console.error('FAIL: request list must include nested addbibresource hint request for bib__deep__refs-local.bib');
  process.exit(1);
}
const bibStyleRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bst' && request.name === 'plain.bst' && request.variant === 'typeset',
);
if (!bibStyleRequest) {
  console.error('FAIL: request list must include bibliographystyle hint request for plain.bst');
  process.exit(1);
}
const bibStyleResourceRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'styleprobe_refs.bib' && request.variant === 'typeset',
);
if (!bibStyleResourceRequest) {
  console.error('FAIL: request list must include bibliography hint request for styleprobe_refs.bib');
  process.exit(1);
}
const multiAddBibRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'multiadd_refs.bib' && request.variant === 'typeset',
);
if (!multiAddBibRequest) {
  console.error('FAIL: request list must include addbibresource hint request for multiadd_refs.bib');
  process.exit(1);
}
const multiBibARequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'multibib_a.bib' && request.variant === 'typeset',
);
if (!multiBibARequest) {
  console.error('FAIL: request list must include bibliography list hint request for multibib_a.bib');
  process.exit(1);
}
const multiBibBRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'bib' && request.name === 'multibib_b.bib' && request.variant === 'typeset',
);
if (!multiBibBRequest) {
  console.error('FAIL: request list must include bibliography list hint request for multibib_b.bib');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'multiadd_refs.bib.bib')) {
  console.error('FAIL: request list must not duplicate extension for addbibresource multiadd_refs.bib');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'demo-image.png')) {
  console.error('FAIL: request list contains stale graphics hint demo-image.png');
  process.exit(1);
}

for (const request of listA.requests) {
  if (!(request.kind === 'texmf' || request.kind === 'fontconfig')) {
    console.error(`FAIL: request kind must be texmf or fontconfig, got ${request.kind}`);
    process.exit(1);
  }
  if (!isSafeToken(request.format) || !isSafeToken(request.name) || !isSafeToken(request.variant)) {
    console.error('FAIL: request tokens must be safe basename-only values');
    process.exit(1);
  }
}

console.log(`PASS: request_list_from_hints_v0 deterministic sha256 ${shaA}`);
console.log(`PASS: request_list_from_hints_v0 schema request_count ${listA.request_count}`);
NODE

node - "$HINT_REQUEST_LIST_A" "$COMBINED_REQUEST_LIST_A" <<'NODE'
const fs = require('node:fs');
const hintsPath = process.argv[2];
const outputPath = process.argv[3];
const hints = JSON.parse(fs.readFileSync(hintsPath, 'utf8'));
const baseRequest = {
  kind: 'texmf',
  format: 'tex',
  name: 'typeset_demo_minimal_v0',
  variant: 'typeset',
};
const requests = [baseRequest, ...(Array.isArray(hints.requests) ? hints.requests : [])];
const deduped = [];
const seen = new Set();
for (const request of requests) {
  const key = `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
  if (seen.has(key)) {
    continue;
  }
  seen.add(key);
  deduped.push(request);
}
const output = {
  version: 1,
  requests: deduped,
};
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
NODE

node - "$HINT_REQUEST_LIST_B" "$COMBINED_REQUEST_LIST_B" <<'NODE'
const fs = require('node:fs');
const hintsPath = process.argv[2];
const outputPath = process.argv[3];
const hints = JSON.parse(fs.readFileSync(hintsPath, 'utf8'));
const baseRequest = {
  kind: 'texmf',
  format: 'tex',
  name: 'typeset_demo_minimal_v0',
  variant: 'typeset',
};
const requests = [baseRequest, ...(Array.isArray(hints.requests) ? hints.requests : [])];
const deduped = [];
const seen = new Set();
for (const request of requests) {
  const key = `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
  if (seen.has(key)) {
    continue;
  }
  seen.add(key);
  deduped.push(request);
}
const output = {
  version: 1,
  requests: deduped,
};
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
NODE

node - "$OUT_DIR" "$BASELINE_ROOT" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const outDir = process.argv[2];
const baselineRoot = process.argv[3];

const firstRunShaPath = (name) => path.join(baselineRoot, `${name}_first.sha256`);
const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
const assertEntrySourceSpans = (caseId, artifactName, entries) => {
  const sourceLen = fs.readFileSync(path.join(outDir, caseId, 'main.tex')).length;
  entries.forEach((entry, entryIndex) => {
    const span = entry?.source_span;
    if (!span || typeof span !== 'object') {
      console.error(`FAIL: ${artifactName}.entries[${entryIndex}] missing source_span`);
      process.exit(1);
    }
    const start = span.start_byte;
    const end = span.end_byte;
    if (!Number.isInteger(start) || !Number.isInteger(end)) {
      console.error(`FAIL: ${artifactName}.entries[${entryIndex}] source_span must use integer offsets`);
      process.exit(1);
    }
    if (start < 0 || end <= start || end > sourceLen) {
      console.error(`FAIL: ${artifactName}.entries[${entryIndex}] source_span out of bounds for ${caseId}`);
      process.exit(1);
    }
  });
};

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
const labelsArtifactFirst = JSON.parse(fs.readFileSync(labelsPath, 'utf8'));
if (!Array.isArray(labelsArtifactFirst?.entries)) {
  console.error('FAIL: expected labels_v0.entries array in first-run artifact');
  process.exit(1);
}
if (labelsArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty labels_v0.entries for labels probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_labels_probe_v0', 'labels_v0', labelsArtifactFirst.entries);
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
const tocArtifactFirst = JSON.parse(fs.readFileSync(tocPath, 'utf8'));
if (!Array.isArray(tocArtifactFirst?.entries)) {
  console.error('FAIL: expected toc_v0.entries array in first-run artifact');
  process.exit(1);
}
if (tocArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty toc_v0.entries for toc probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_toc_probe_v0', 'toc_v0', tocArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('toc_v0'), `${tocShaFirst}\n`);

const bibSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'summary.json'), 'utf8'),
);
if (bibSummary?.typed_artifacts?.bib?.present !== true) {
  console.error('FAIL: expected bib typed artifact present after first run');
  process.exit(1);
}
const bibPath = path.join(outDir, 'typeset_demo_bib_probe_v0', 'bib_v0.json');
if (!fs.existsSync(bibPath)) {
  console.error('FAIL: expected bib_v0.json artifact after first run');
  process.exit(1);
}
const bibShaFirst = bibSummary.typed_artifacts.bib.artifact_sha256;
if (typeof bibShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(bibShaFirst)) {
  console.error('FAIL: expected bib artifact sha256 in first summary');
  process.exit(1);
}
const bibArtifactFirst = JSON.parse(fs.readFileSync(bibPath, 'utf8'));
if (!Array.isArray(bibArtifactFirst?.entries)) {
  console.error('FAIL: expected bib_v0.entries array in first-run artifact');
  process.exit(1);
}
if (bibArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty bib_v0.entries for bib probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_bib_probe_v0', 'bib_v0', bibArtifactFirst.entries);
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
const hyperrefArtifactFirst = JSON.parse(fs.readFileSync(hyperrefPath, 'utf8'));
if (!Array.isArray(hyperrefArtifactFirst?.entries)) {
  console.error('FAIL: expected hyperref_v0.entries array in first-run artifact');
  process.exit(1);
}
if (hyperrefArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected hyperref_v0.entries to include at least one link in probe fixture');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_hyperref_probe_v0', 'hyperref_v0', hyperrefArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('hyperref_v0'), `${hyperrefShaFirst}\n`);

const pkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pkgopt_probe_v0', 'summary.json'), 'utf8'),
);
if (pkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present after first run');
  process.exit(1);
}
const pkgoptPath = path.join(outDir, 'typeset_demo_pkgopt_probe_v0', 'pkgopt_v0.json');
if (!fs.existsSync(pkgoptPath)) {
  console.error('FAIL: expected pkgopt_v0.json artifact after first run');
  process.exit(1);
}
const pkgoptShaFirst = pkgoptSummary.typed_artifacts.pkgopt.artifact_sha256;
if (typeof pkgoptShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(pkgoptShaFirst)) {
  console.error('FAIL: expected pkgopt artifact sha256 in first summary');
  process.exit(1);
}
const pkgoptArtifactFirst = JSON.parse(fs.readFileSync(pkgoptPath, 'utf8'));
if (!Array.isArray(pkgoptArtifactFirst?.entries)) {
  console.error('FAIL: expected pkgopt_v0.entries array in first-run artifact');
  process.exit(1);
}
if (pkgoptArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for pkgopt probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_pkgopt_probe_v0', 'pkgopt_v0', pkgoptArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('pkgopt_v0'), `${pkgoptShaFirst}\n`);

const graphicsSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_graphics_probe_v0', 'summary.json'), 'utf8'),
);
if (graphicsSummary?.typed_artifacts?.graphics?.present !== true) {
  console.error('FAIL: expected graphics typed artifact present after first run');
  process.exit(1);
}
const graphicsPath = path.join(outDir, 'typeset_demo_graphics_probe_v0', 'graphics_v0.json');
if (!fs.existsSync(graphicsPath)) {
  console.error('FAIL: expected graphics_v0.json artifact after first run');
  process.exit(1);
}
const graphicsShaFirst = graphicsSummary.typed_artifacts.graphics.artifact_sha256;
if (typeof graphicsShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(graphicsShaFirst)) {
  console.error('FAIL: expected graphics artifact sha256 in first summary');
  process.exit(1);
}
const graphicsArtifactFirst = JSON.parse(fs.readFileSync(graphicsPath, 'utf8'));
if (!Array.isArray(graphicsArtifactFirst?.entries)) {
  console.error('FAIL: expected graphics_v0.entries array in first-run artifact');
  process.exit(1);
}
if (graphicsArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty graphics_v0.entries for graphics probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_graphics_probe_v0', 'graphics_v0', graphicsArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('graphics_v0'), `${graphicsShaFirst}\n`);

const report = JSON.parse(fs.readFileSync(path.join(outDir, 'report.json'), 'utf8'));
if (report?.typed_artifacts_version !== 1) {
  console.error('FAIL: expected report.typed_artifacts_version=1 after first run');
  process.exit(1);
}
const resourceHints = report?.resource_hints_v0;
if (!resourceHints || typeof resourceHints !== 'object') {
  console.error('FAIL: expected report.resource_hints_v0 after first run');
  process.exit(1);
}
if (resourceHints.version !== 1) {
  console.error('FAIL: expected report.resource_hints_v0.version=1 after first run');
  process.exit(1);
}
if (!Array.isArray(resourceHints.entries)) {
  console.error('FAIL: expected report.resource_hints_v0.entries array after first run');
  process.exit(1);
}
if (resourceHints.entries.length <= 0) {
  console.error('FAIL: expected non-empty report.resource_hints_v0.entries after first run');
  process.exit(1);
}
for (const [index, entry] of resourceHints.entries.entries()) {
  if (typeof entry?.case_id !== 'string' || entry.case_id.length === 0) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid case_id`);
    process.exit(1);
  }
  if (typeof entry?.hint_type !== 'string' || entry.hint_type.length === 0) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid hint_type`);
    process.exit(1);
  }
  if (typeof entry?.value !== 'string' || entry.value.length === 0) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid value`);
    process.exit(1);
  }
}
fs.writeFileSync(firstRunShaPath('resource_hints_v0'), `${sha256(Buffer.from(JSON.stringify(resourceHints)))}\n`);
const typedArtifactShaMap = report?.typed_artifact_sha256;
if (!typedArtifactShaMap || typeof typedArtifactShaMap !== 'object') {
  console.error('FAIL: expected report.typed_artifact_sha256 map after first run');
  process.exit(1);
}
const typedKeys = ['toc', 'labels', 'bib', 'hyperref', 'pkgopt', 'graphics'];
for (const key of typedKeys) {
  const value = typedArtifactShaMap[key];
  if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) {
    console.error(`FAIL: expected report.typed_artifact_sha256.${key} sha256 after first run`);
    process.exit(1);
  }
}
fs.writeFileSync(
  path.join(baselineRoot, 'typed_artifact_sha256_first.json'),
  `${JSON.stringify(typedArtifactShaMap, null, 2)}\n`,
);
fs.writeFileSync(firstRunShaPath('resolved_resources_count'), `${Number(report.resolved_resources_count ?? 0)}\n`);
NODE

TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$COMBINED_REQUEST_LIST_A" "$HINT_STORE_DIR_A"
TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$COMBINED_REQUEST_LIST_B" "$HINT_STORE_DIR_B"

node - "$HINT_STORE_DIR_A" "$HINT_STORE_DIR_B" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const storeDirA = process.argv[2];
const storeDirB = process.argv[3];
const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');

const indexAPath = path.join(storeDirA, 'index.json');
const indexBPath = path.join(storeDirB, 'index.json');
const summaryAPath = path.join(storeDirA, 'summary.json');
const summaryBPath = path.join(storeDirB, 'summary.json');
const indexASha = sha256(fs.readFileSync(indexAPath));
const indexBSha = sha256(fs.readFileSync(indexBPath));
if (indexASha !== indexBSha) {
  console.error('FAIL: texlive_store_gen_v0 from hints must be deterministic');
  process.exit(1);
}
const summaryA = JSON.parse(fs.readFileSync(summaryAPath, 'utf8'));
const summaryB = JSON.parse(fs.readFileSync(summaryBPath, 'utf8'));
if (summaryA.index_sha256 !== summaryB.index_sha256 || summaryA.found_count !== summaryB.found_count) {
  console.error('FAIL: texlive_store_gen_v0 hint summaries must match across reruns');
  process.exit(1);
}
const indexA = JSON.parse(fs.readFileSync(indexAPath, 'utf8'));
const entries = Array.isArray(indexA?.entries) ? indexA.entries : [];
const hasEntry = (kind, format, name, variant) => entries.some(
  (entry) => entry.kind === kind && entry.format === format && entry.name === name && entry.variant === variant,
);
const requiredEntries = [
  ['texmf', 'tex', 'typeset_demo_minimal_v0', 'typeset'],
  ['texmf', 'tex', 'chapter_intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapter_appendix.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__appendix.tex', 'typeset'],
  ['texmf', 'tex', 'sections__intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__ch1.tex', 'typeset'],
  ['texmf', 'sty', 'xcolor.sty', 'typeset'],
  ['texmf', 'bib', 'refs.bib', 'typeset'],
  ['texmf', 'bib', 'styleprobe_refs.bib', 'typeset'],
  ['texmf', 'bib', 'multiadd_refs.bib', 'typeset'],
  ['texmf', 'bib', 'multibib_a.bib', 'typeset'],
  ['texmf', 'bib', 'multibib_b.bib', 'typeset'],
  ['texmf', 'bib', 'legacyrefs.bib', 'typeset'],
  ['texmf', 'bib', 'bib__deep__refs-local.bib', 'typeset'],
  ['texmf', 'bib', 'legacy__deeprefs.bib', 'typeset'],
  ['texmf', 'bst', 'plain.bst', 'typeset'],
  ['texmf', 'sty', 'natbib.sty', 'typeset'],
  ['texmf', 'png', 'demo.png', 'typeset'],
  ['texmf', 'png', 'probe-figure.png', 'typeset'],
  ['texmf', 'pdf', 'figs__diagram.pdf', 'typeset'],
  ['fontconfig', 'name', 'FoundSans', 'public'],
];
for (const [kind, format, name, variant] of requiredEntries) {
  if (!hasEntry(kind, format, name, variant)) {
    console.error(`FAIL: expected hint-driven store entry ${kind}/${format}/${variant}/${name}`);
    process.exit(1);
  }
}
if (!(summaryA.found_count >= requiredEntries.length && summaryA.missing_count >= 1)) {
  console.error(`FAIL: expected hint-driven store found>=${requiredEntries.length} and missing>=1, got found=${summaryA.found_count} missing=${summaryA.missing_count}`);
  process.exit(1);
}
console.log(`PASS: texlive_store_gen_v0 from hints deterministic index_sha256 ${indexASha}`);
console.log(`PASS: texlive_store_gen_v0 from hints found=${summaryA.found_count} missing=${summaryA.missing_count}`);
NODE

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$HINT_STORE_DIR_A" \
node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"

node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_A"
node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_B"

node - "$BASELINE_DIR_A" "$BASELINE_DIR_B" "$OUT_DIR" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const baselineDirA = process.argv[2];
const baselineDirB = process.argv[3];
const outDir = process.argv[4];

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
const report = readJson(path.join(outDir, 'report.json'));
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
if (indexA.typed_artifacts_version !== 1 || indexB.typed_artifacts_version !== 1) {
  console.error('FAIL: baseline index missing typed_artifacts_version=1');
  process.exit(1);
}
if (!indexA.typed_artifact_sha256 || typeof indexA.typed_artifact_sha256 !== 'object') {
  console.error('FAIL: baseline index missing typed_artifact_sha256 map');
  process.exit(1);
}
if (!indexB.typed_artifact_sha256 || typeof indexB.typed_artifact_sha256 !== 'object') {
  console.error('FAIL: baseline index rerun missing typed_artifact_sha256 map');
  process.exit(1);
}
const typedKeys = ['toc', 'labels', 'bib', 'hyperref', 'pkgopt', 'graphics'];
for (const key of typedKeys) {
  const valueA = indexA.typed_artifact_sha256[key];
  const valueB = indexB.typed_artifact_sha256[key];
  if (!/^[0-9a-f]{64}$/.test(valueA) || !/^[0-9a-f]{64}$/.test(valueB)) {
    console.error(`FAIL: baseline index typed_artifact_sha256 invalid for key ${key}`);
    process.exit(1);
  }
  if (valueA !== valueB) {
    console.error(`FAIL: baseline index typed_artifact_sha256 mismatch across reruns for key ${key}`);
    process.exit(1);
  }
  if (report?.typed_artifact_sha256?.[key] !== valueA) {
    console.error(`FAIL: baseline index typed_artifact_sha256 mismatch vs report for key ${key}`);
    process.exit(1);
  }
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
console.log(`PASS: baseline generator typed_artifacts_version ${indexA.typed_artifacts_version}`);
console.log('PASS: baseline generator typed_artifact_sha256 map present');
NODE

node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_AUTO_PACK_DIR"

rm -rf "$STORE_DIR"
cp -R "$HINT_STORE_DIR_A" "$STORE_DIR"

WASM_FIXTURE_GALLERY_SKIP_PROOF_V0=1 \
WASM_FIXTURE_GALLERY_NO_OPEN_V0=1 \
WASM_FIXTURE_GALLERY_AUTO_BASELINE_PACK_V0=1 \
TEXLIVE_BASELINE_PACKS_DIR_V0="$BASELINE_PACKS_ROOT" \
"$ROOT_DIR/scripts/open_wasm_fixture_gallery_v0.sh" "$OUT_DIR"

if [[ ! -s "$OUT_DIR/report.json" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/report.json" >&2
  exit 1
fi

node - "$OUT_DIR" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const outDir = process.argv[2];
const report = JSON.parse(fs.readFileSync(path.join(outDir, 'report.json'), 'utf8'));
const statuses = Array.isArray(report.statuses) ? report.statuses : [];
const resolvedCount = Number(report.resolved_resources_count ?? 0);
const resolvedCountFirst = Number(
  fs.readFileSync(path.join(`${outDir}_baseline`, 'resolved_resources_count_first.sha256'), 'utf8').trim(),
);
const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
if (report?.typed_artifacts_version !== 1) {
  console.error('FAIL: expected report.typed_artifacts_version=1 after rerun');
  process.exit(1);
}
const resourceHints = report?.resource_hints_v0;
if (!resourceHints || typeof resourceHints !== 'object') {
  console.error('FAIL: expected report.resource_hints_v0 after rerun');
  process.exit(1);
}
if (resourceHints.version !== 1) {
  console.error('FAIL: expected report.resource_hints_v0.version=1 after rerun');
  process.exit(1);
}
if (!Array.isArray(resourceHints.entries)) {
  console.error('FAIL: expected report.resource_hints_v0.entries array after rerun');
  process.exit(1);
}
const resourceHintsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'resource_hints_v0_first.sha256'), 'utf8').trim();
const resourceHintsShaSecond = sha256(Buffer.from(JSON.stringify(resourceHints)));
if (resourceHintsShaSecond !== resourceHintsShaFirst) {
  console.error('FAIL: report.resource_hints_v0 must be stable across reruns');
  process.exit(1);
}
const requiredTypedKeys = ['toc', 'labels', 'bib', 'hyperref', 'pkgopt', 'graphics'];
const labelsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'labels_v0_first.sha256'), 'utf8').trim();
const tocShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'toc_v0_first.sha256'), 'utf8').trim();
const bibShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'bib_v0_first.sha256'), 'utf8').trim();
const hyperrefShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'hyperref_v0_first.sha256'), 'utf8').trim();
const pkgoptShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'pkgopt_v0_first.sha256'), 'utf8').trim();
const graphicsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'graphics_v0_first.sha256'), 'utf8').trim();

if (resolvedCount <= 0) {
  console.error('FAIL: expected at least one resolved resource in fixture gallery summaries');
  process.exit(1);
}
if (!(resolvedCount > resolvedCountFirst)) {
  console.error(
    `FAIL: expected resolved_resources_count to increase after hint-driven store (${resolvedCountFirst} -> ${resolvedCount})`,
  );
  process.exit(1);
}
if (resolvedCount < 25) {
  console.error(`FAIL: expected resolved_resources_count >= 25 after input/include hint expansion, got ${resolvedCount}`);
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
if (resourceHints.entries.some((entry) => entry.case_id === 'ok_demo_v0')) {
  console.error('FAIL: expected no resource_hints_v0 entries for ok_demo_v0');
  process.exit(1);
}

for (const status of statuses) {
  if (status?.typed_artifacts_version !== 1) {
    console.error(`FAIL: case ${status.case_id} status missing typed_artifacts_version=1`);
    process.exit(1);
  }
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
  if (summary?.typed_artifacts_version !== 1) {
    console.error(`FAIL: case ${status.case_id} summary missing typed_artifacts_version=1`);
    process.exit(1);
  }
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
const labelsArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_labels_probe_v0', 'labels_v0.json'), 'utf8'),
);
if (!Array.isArray(labelsArtifactSecond?.entries) || labelsArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty labels_v0.entries after rerun');
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
const tocArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'toc_v0.json'), 'utf8'),
);
if (!Array.isArray(tocArtifactSecond?.entries) || tocArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty toc_v0.entries after rerun');
  process.exit(1);
}

const bibSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'summary.json'), 'utf8'),
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
const bibArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'bib_v0.json'), 'utf8'),
);
if (!Array.isArray(bibArtifactSecond?.entries) || bibArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty bib_v0.entries after rerun');
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
const hyperrefArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_hyperref_probe_v0', 'hyperref_v0.json'), 'utf8'),
);
if (!Array.isArray(hyperrefArtifactSecond?.entries) || hyperrefArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty hyperref_v0.entries array after rerun');
  process.exit(1);
}

const pkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pkgopt_probe_v0', 'summary.json'), 'utf8'),
);
const pkgoptArtifact = pkgoptSummary?.typed_artifacts?.pkgopt;
if (!pkgoptArtifact || pkgoptArtifact.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present after second run');
  process.exit(1);
}
const pkgoptShaSecond = pkgoptArtifact.artifact_sha256;
if (pkgoptShaSecond !== pkgoptShaFirst) {
  console.error('FAIL: pkgopt_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const pkgoptArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pkgopt_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(pkgoptArtifactSecond?.entries) || pkgoptArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries after rerun');
  process.exit(1);
}

const graphicsSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_graphics_probe_v0', 'summary.json'), 'utf8'),
);
const graphicsArtifact = graphicsSummary?.typed_artifacts?.graphics;
if (!graphicsArtifact || graphicsArtifact.present !== true) {
  console.error('FAIL: expected graphics typed artifact present after second run');
  process.exit(1);
}
const graphicsShaSecond = graphicsArtifact.artifact_sha256;
if (graphicsShaSecond !== graphicsShaFirst) {
  console.error('FAIL: graphics_v0 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const graphicsArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_graphics_probe_v0', 'graphics_v0.json'), 'utf8'),
);
if (!Array.isArray(graphicsArtifactSecond?.entries) || graphicsArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty graphics_v0.entries after rerun');
  process.exit(1);
}

const reportCaseSha = report.case_artifact_sha256;
if (!reportCaseSha || typeof reportCaseSha !== 'object') {
  console.error('FAIL: report missing top-level case_artifact_sha256');
  process.exit(1);
}
const reportTypedSha = report.typed_artifact_sha256;
if (!reportTypedSha || typeof reportTypedSha !== 'object') {
  console.error('FAIL: report missing top-level typed_artifact_sha256');
  process.exit(1);
}
const reportTypedShaFirst = JSON.parse(
  fs.readFileSync(path.join(`${outDir}_baseline`, 'typed_artifact_sha256_first.json'), 'utf8'),
);
for (const key of requiredTypedKeys) {
  const value = reportTypedSha[key];
  if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) {
    console.error(`FAIL: report typed_artifact_sha256.${key} missing or invalid`);
    process.exit(1);
  }
  if (value !== reportTypedShaFirst[key]) {
    console.error(`FAIL: report typed_artifact_sha256.${key} must be stable across reruns`);
    process.exit(1);
  }
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
console.log(`PASS: resolved_resources_count increased from ${resolvedCountFirst} to ${resolvedCount}`);
console.log('PASS: resolved_resources_count meets floor >= 25');
console.log(`PASS: baseline_match MATCH for all OK cases (${okStatuses.length})`);
console.log(`PASS: typed_artifacts keys ${requiredTypedKeys.join(',')}`);
console.log('PASS: typed_artifacts_version gate 1');
console.log(`PASS: resource_hints_v0 sha stable ${resourceHintsShaSecond}`);
console.log('PASS: resource_hints_v0 excludes ok_demo_v0');
console.log(`PASS: labels_v0 sha stable ${labelsShaSecond}`);
console.log(`PASS: toc_v0 sha stable ${tocShaSecond}`);
console.log(`PASS: bib_v0 sha stable ${bibShaSecond}`);
console.log(`PASS: hyperref_v0 sha stable ${hyperrefShaSecond}`);
console.log(`PASS: pkgopt_v0 sha stable ${pkgoptShaSecond}`);
console.log(`PASS: graphics_v0 sha stable ${graphicsShaSecond}`);
console.log('PASS: report typed_artifact_sha256 map present and stable');
console.log('PASS: report top-level case_artifact_sha256 present');
NODE

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
