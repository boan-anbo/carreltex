#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
ONDEMAND_OUT_DIR="${OUT_DIR}_ondemand_v1"
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

rm -rf "$OUT_DIR" "$ONDEMAND_OUT_DIR" "$STORE_DIR" "$HINT_STORE_DIR_A" "$HINT_STORE_DIR_B" "$FIXTURE_SOURCE_DIR" "$BASELINE_ROOT"
mkdir -p \
  "$OUT_DIR" \
  "$FIXTURE_SOURCE_DIR/xetex/tex" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/sections" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/chapters" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/appendices" \
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
printf 'fixture-bytes-for-ondemand-input-probe-main\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_ondemand_input_probe_v0"
printf 'fixture-bytes-for-ondemand-include-probe-main\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_ondemand_include_probe_v0"
printf 'fixture-bytes-for-chapter-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_intro.tex"
printf 'fixture-bytes-for-chapter-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_appendix.tex"
printf 'fixture-bytes-for-chapters-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__intro.tex"
printf 'fixture-bytes-for-chapters-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__appendix.tex"
printf 'fixture-bytes-for-sections-intro-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/intro.tex"
printf 'fixture-bytes-for-chapters-ch1-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters/ch1.tex"
printf 'fixture-bytes-for-sections-intro-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__intro.tex"
printf 'fixture-bytes-for-chapters-ch1-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__ch1.tex"
printf 'fixture-bytes-for-ondemand-extra-section-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand/extra_section.tex"
printf 'fixture-bytes-for-ondemand-chapter-one-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand/chapter_one.tex"
printf 'fixture-bytes-for-ondemand-extra-section-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand__extra_section.tex"
printf 'fixture-bytes-for-ondemand-chapter-one-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand__chapter_one.tex"
printf 'fixture-bytes-for-appendices-apx-a-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices/apx_a.tex"
printf 'fixture-bytes-for-appendices-apx-b-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices/apx_b.tex"
printf 'fixture-bytes-for-appendices-apx-a-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices__apx_a.tex"
printf 'fixture-bytes-for-appendices-apx-b-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices__apx_b.tex"
printf 'fixture-bytes-for-demo-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/demo.png"
printf 'fixture-bytes-for-probe-figure-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/probe-figure.png"
printf 'fixture-bytes-for-figs-diagram-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__diagram.pdf"
printf 'fixture-bytes-for-figs-demo-graphic-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__demo_graphic.pdf"
printf 'fixture-bytes-for-plots-demo-graphic-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/plots__demo_graphic.pdf"
printf 'fixture-bytes-for-figs-banner-graphic-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__banner_graphic.pdf"
printf 'fixture-bytes-for-figs-sub-banner-graphic-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__sub__banner_graphic.pdf"
printf 'fixture-bytes-for-assets-figs-multi-probe-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__figs__multi_probe.pdf"
printf 'fixture-bytes-for-assets-plots-multi-probe-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__plots__multi_probe.pdf"
printf 'fixture-bytes-for-assets-hires-chart-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__hires__chart.pdf"
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
printf 'fixture-bytes-for-foo-bar-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/foo__bar.sty"
printf 'fixture-bytes-for-fooopts-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/fooopts.sty"
printf 'fixture-bytes-for-baropts-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/baropts.sty"
printf 'fixture-bytes-for-pkgoptsdemo-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/pkgoptsdemo.sty"
printf 'fixture-bytes-for-packmulti-a-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__a.sty"
printf 'fixture-bytes-for-packmulti-b-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__b.sty"
printf 'fixture-bytes-for-packmulti-c-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__c.sty"
printf 'fixture-bytes-for-natbib-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/natbib.sty"
printf 'fixture-bytes-for-memoir-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/memoir.cls"
printf 'fixture-bytes-for-classoptsdemo-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/classoptsdemo.cls"
printf 'fixture-bytes-for-classoptsmulti-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/classoptsmulti.cls"
printf 'fixture-bytes-for-memoirplus-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/memoirplus.cls"
printf 'fixture-bytes-for-article-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/article.cls"
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
const packagePathRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'foo__bar.sty' && request.variant === 'typeset',
);
if (!packagePathRequest) {
  console.error('FAIL: request list must include package hint request for foo__bar.sty');
  process.exit(1);
}
const passOptionsPackageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'fooopts.sty' && request.variant === 'typeset',
);
if (!passOptionsPackageRequest) {
  console.error('FAIL: request list must include package hint request for fooopts.sty');
  process.exit(1);
}
const requireWithOptionsPackageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'baropts.sty' && request.variant === 'typeset',
);
if (!requireWithOptionsPackageRequest) {
  console.error('FAIL: request list must include package hint request for baropts.sty');
  process.exit(1);
}
const usepackageMultiOptionsRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'pkgoptsdemo.sty' && request.variant === 'typeset',
);
if (!usepackageMultiOptionsRequest) {
  console.error('FAIL: request list must include package hint request for pkgoptsdemo.sty');
  process.exit(1);
}
const usepackageMultipackageRequestA = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'packmulti__a.sty' && request.variant === 'typeset',
);
if (!usepackageMultipackageRequestA) {
  console.error('FAIL: request list must include package hint request for packmulti__a.sty');
  process.exit(1);
}
const usepackageMultipackageRequestB = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'packmulti__b.sty' && request.variant === 'typeset',
);
if (!usepackageMultipackageRequestB) {
  console.error('FAIL: request list must include package hint request for packmulti__b.sty');
  process.exit(1);
}
const requireMultipackageRequestC = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'packmulti__c.sty' && request.variant === 'typeset',
);
if (!requireMultipackageRequestC) {
  console.error('FAIL: request list must include package hint request for packmulti__c.sty');
  process.exit(1);
}
const classOptionsRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'cls' && request.name === 'classoptsdemo.cls' && request.variant === 'typeset',
);
if (!classOptionsRequest) {
  console.error('FAIL: request list must include class hint request for classoptsdemo.cls');
  process.exit(1);
}
const memoirClassRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'cls' && request.name === 'memoir.cls' && request.variant === 'typeset',
);
if (!memoirClassRequest) {
  console.error('FAIL: request list must include class hint request for memoir.cls');
  process.exit(1);
}
const memoirPlusClassRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'cls' && request.name === 'memoirplus.cls' && request.variant === 'typeset',
);
if (!memoirPlusClassRequest) {
  console.error('FAIL: request list must include class hint request for memoirplus.cls');
  process.exit(1);
}
const classOptionsMultiRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'cls' && request.name === 'classoptsmulti.cls' && request.variant === 'typeset',
);
if (!classOptionsMultiRequest) {
  console.error('FAIL: request list must include class hint request for classoptsmulti.cls');
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
const ondemandInputProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'ondemand__extra_section.tex' && request.variant === 'typeset',
);
if (!ondemandInputProbeRequest) {
  console.error('FAIL: request list must include on-demand input hint request for ondemand__extra_section.tex');
  process.exit(1);
}
const ondemandIncludeProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'ondemand__chapter_one.tex' && request.variant === 'typeset',
);
if (!ondemandIncludeProbeRequest) {
  console.error('FAIL: request list must include on-demand include hint request for ondemand__chapter_one.tex');
  process.exit(1);
}
const includeOnlyProbeRequestA = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'appendices__apx_a.tex' && request.variant === 'typeset',
);
if (!includeOnlyProbeRequestA) {
  console.error('FAIL: request list must include includeonly hint request for appendices__apx_a.tex');
  process.exit(1);
}
const includeOnlyProbeRequestB = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'appendices__apx_b.tex' && request.variant === 'typeset',
);
if (!includeOnlyProbeRequestB) {
  console.error('FAIL: request list must include includeonly hint request for appendices__apx_b.tex');
  process.exit(1);
}
const graphicsOptsRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'figs__diagram.pdf' && request.variant === 'typeset',
);
if (!graphicsOptsRequest) {
  console.error('FAIL: request list must include includegraphics ext/dir hint request for figs__diagram.pdf');
  process.exit(1);
}
const graphicspathRequestA = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'figs__demo_graphic.pdf' && request.variant === 'typeset',
);
if (!graphicspathRequestA) {
  console.error('FAIL: request list must include graphicspath hint request for figs__demo_graphic.pdf');
  process.exit(1);
}
const graphicspathRequestB = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'plots__demo_graphic.pdf' && request.variant === 'typeset',
);
if (!graphicspathRequestB) {
  console.error('FAIL: request list must include graphicspath hint request for plots__demo_graphic.pdf');
  process.exit(1);
}
const graphicspathExplicitRequestA = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'figs__banner_graphic.pdf' && request.variant === 'typeset',
);
if (!graphicspathExplicitRequestA) {
  console.error('FAIL: request list must include explicit-ext graphicspath hint request for figs__banner_graphic.pdf');
  process.exit(1);
}
const graphicspathExplicitRequestB = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'figs__sub__banner_graphic.pdf' && request.variant === 'typeset',
);
if (!graphicspathExplicitRequestB) {
  console.error('FAIL: request list must include explicit-ext graphicspath hint request for figs__sub__banner_graphic.pdf');
  process.exit(1);
}
const graphicsMultipathRequestA = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'assets__figs__multi_probe.pdf' && request.variant === 'typeset',
);
if (!graphicsMultipathRequestA) {
  console.error('FAIL: request list must include multipath graphicspath hint request for assets__figs__multi_probe.pdf');
  process.exit(1);
}
const graphicsMultipathRequestB = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'assets__plots__multi_probe.pdf' && request.variant === 'typeset',
);
if (!graphicsMultipathRequestB) {
  console.error('FAIL: request list must include multipath graphicspath hint request for assets__plots__multi_probe.pdf');
  process.exit(1);
}
const graphicsTypePathRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'pdf' && request.name === 'assets__hires__chart.pdf' && request.variant === 'typeset',
);
if (!graphicsTypePathRequest) {
  console.error('FAIL: request list must include includegraphics type/path hint request for assets__hires__chart.pdf');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'bad__danger_graphic.pdf' || request.name === 'abs__danger_graphic.pdf')) {
  console.error('FAIL: request list must fail-closed for unsafe graphicspath entries');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'unsafe__bad_chart.pdf')) {
  console.error('FAIL: request list must fail-closed for unsafe includegraphics option path entries');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'evil.sty' || request.name === 'evil__pkg.sty')) {
  console.error('FAIL: request list must fail-closed for unsafe usepackage entries');
  process.exit(1);
}
if (listA.requests.some((request) => request.name === 'evilpkg.sty')) {
  console.error('FAIL: request list must fail-closed for unsafe usepackage multipackage entries');
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

const documentclassPkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_documentclass_opts_probe_v0', 'summary.json'), 'utf8'),
);
if (documentclassPkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present for documentclass opts probe after first run');
  process.exit(1);
}
const documentclassPkgoptArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_documentclass_opts_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(documentclassPkgoptArtifact?.entries) || documentclassPkgoptArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for documentclass opts probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_documentclass_opts_probe_v0', 'pkgopt_v0', documentclassPkgoptArtifact.entries);
if (!documentclassPkgoptArtifact.entries.some((entry) => entry.package === 'memoir')) {
  console.error('FAIL: expected documentclass opts probe pkgopt entries to include memoir target');
  process.exit(1);
}

const passOptionsClassPkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_passoptionstoclass_probe_v0', 'summary.json'), 'utf8'),
);
if (passOptionsClassPkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present for passoptionstoclass probe after first run');
  process.exit(1);
}
const passOptionsClassPkgoptArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_passoptionstoclass_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(passOptionsClassPkgoptArtifact?.entries) || passOptionsClassPkgoptArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for passoptionstoclass probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_passoptionstoclass_probe_v0', 'pkgopt_v0', passOptionsClassPkgoptArtifact.entries);
if (!passOptionsClassPkgoptArtifact.entries.some((entry) => entry.package === 'memoir')) {
  console.error('FAIL: expected passoptionstoclass probe pkgopt entries to include memoir target');
  process.exit(1);
}

const classOptionsMultiPkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_documentclass_opts_multi_probe_v0', 'summary.json'), 'utf8'),
);
if (classOptionsMultiPkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present for documentclass opts multi probe after first run');
  process.exit(1);
}
const classOptionsMultiPkgoptArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_documentclass_opts_multi_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(classOptionsMultiPkgoptArtifact?.entries) || classOptionsMultiPkgoptArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for documentclass opts multi probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_documentclass_opts_multi_probe_v0', 'pkgopt_v0', classOptionsMultiPkgoptArtifact.entries);
const passOptionsClassMultiEntry = classOptionsMultiPkgoptArtifact.entries.find(
  (entry) => entry.command === 'PassOptionsToClass' && entry.package === 'classoptsmulti',
);
if (!passOptionsClassMultiEntry) {
  console.error('FAIL: expected PassOptionsToClass pkgopt entry for classoptsmulti in multi probe');
  process.exit(1);
}
if (JSON.stringify(passOptionsClassMultiEntry.options) !== JSON.stringify(['twoside', 'openright', 'draft'])) {
  console.error('FAIL: expected PassOptionsToClass options deduped+ordered as [twoside,openright,draft]');
  process.exit(1);
}
const documentclassMultiEntry = classOptionsMultiPkgoptArtifact.entries.find(
  (entry) => entry.command === 'documentclass' && entry.package === 'classoptsmulti',
);
if (!documentclassMultiEntry) {
  console.error('FAIL: expected documentclass pkgopt entry for classoptsmulti in multi probe');
  process.exit(1);
}
if (JSON.stringify(documentclassMultiEntry.options) !== JSON.stringify(['openright', 'draft', 'twoside'])) {
  console.error('FAIL: expected documentclass options deduped+ordered as [openright,draft,twoside]');
  process.exit(1);
}

const usepackageMultiPkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_opts_multi_probe_v0', 'summary.json'), 'utf8'),
);
if (usepackageMultiPkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present for usepackage opts multi probe after first run');
  process.exit(1);
}
const usepackageMultiPkgoptArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_opts_multi_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(usepackageMultiPkgoptArtifact?.entries) || usepackageMultiPkgoptArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for usepackage opts multi probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_usepackage_opts_multi_probe_v0', 'pkgopt_v0', usepackageMultiPkgoptArtifact.entries);
const usepackageMultiEntry = usepackageMultiPkgoptArtifact.entries.find(
  (entry) => entry.command === 'usepackage' && entry.package === 'pkgoptsdemo',
);
if (!usepackageMultiEntry) {
  console.error('FAIL: expected usepackage pkgopt entry for pkgoptsdemo in multi probe');
  process.exit(1);
}
if (JSON.stringify(usepackageMultiEntry.options) !== JSON.stringify(['table', 'dvipsnames', 'svgnames'])) {
  console.error('FAIL: expected usepackage options deduped+ordered as [table,dvipsnames,svgnames]');
  process.exit(1);
}

const multipackagePkgoptSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_multipackage_probe_v0', 'summary.json'), 'utf8'),
);
if (multipackagePkgoptSummary?.typed_artifacts?.pkgopt?.present !== true) {
  console.error('FAIL: expected pkgopt typed artifact present for usepackage multipackage probe after first run');
  process.exit(1);
}
const multipackagePkgoptArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_multipackage_probe_v0', 'pkgopt_v0.json'), 'utf8'),
);
if (!Array.isArray(multipackagePkgoptArtifact?.entries) || multipackagePkgoptArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty pkgopt_v0.entries for usepackage multipackage probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_usepackage_multipackage_probe_v0', 'pkgopt_v0', multipackagePkgoptArtifact.entries);
const multipackageUseA = multipackagePkgoptArtifact.entries.find(
  (entry) => entry.command === 'usepackage' && entry.package === 'packmulti/a',
);
if (!multipackageUseA || JSON.stringify(multipackageUseA.options) !== JSON.stringify(['optc', 'opta'])) {
  console.error('FAIL: expected usepackage multipackage entry for packmulti/a with options [optc,opta]');
  process.exit(1);
}
const multipackageUseB = multipackagePkgoptArtifact.entries.find(
  (entry) => entry.command === 'usepackage' && entry.package === 'packmulti/b',
);
if (!multipackageUseB || JSON.stringify(multipackageUseB.options) !== JSON.stringify(['optc', 'opta'])) {
  console.error('FAIL: expected usepackage multipackage entry for packmulti/b with options [optc,opta]');
  process.exit(1);
}
const multipackageRequireC = multipackagePkgoptArtifact.entries.find(
  (entry) => entry.command === 'RequirePackage' && entry.package === 'packmulti/c',
);
if (!multipackageRequireC || JSON.stringify(multipackageRequireC.options) !== JSON.stringify(['optb', 'opta'])) {
  console.error('FAIL: expected RequirePackage multipackage entry for packmulti/c with options [optb,opta]');
  process.exit(1);
}

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
if (resourceHints.resource_hints_v0_version !== 1) {
  console.error('FAIL: expected report.resource_hints_v0.resource_hints_v0_version=1 after first run');
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
const allowedHintTypes = new Set([
  'tex_input',
  'tex_include',
  'tex_includeonly',
  'package_file',
  'class_file',
  'graphics_path',
  'bib_resource',
  'bib_style',
  'hyperref_url',
]);
const fixtureBytesByCase = new Map();
for (const [index, entry] of resourceHints.entries.entries()) {
  if (entry?.kind !== 'resource_hint') {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid kind`);
    process.exit(1);
  }
  if (typeof entry?.case_id !== 'string' || entry.case_id.length === 0) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid case_id`);
    process.exit(1);
  }
  if (typeof entry?.hint_type !== 'string' || !allowedHintTypes.has(entry.hint_type)) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid hint_type`);
    process.exit(1);
  }
  if (typeof entry?.value !== 'string' || entry.value.length === 0) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] invalid value`);
    process.exit(1);
  }
  const sourceSpan = entry?.source_span;
  if (!sourceSpan || !Number.isInteger(sourceSpan.start_byte) || !Number.isInteger(sourceSpan.end_byte)) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] missing source_span`);
    process.exit(1);
  }
  if (sourceSpan.start_byte < 0 || sourceSpan.end_byte <= sourceSpan.start_byte) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] source_span must satisfy start<end`);
    process.exit(1);
  }
  if (!fixtureBytesByCase.has(entry.case_id)) {
    const fixturePath = path.join(outDir, entry.case_id, 'main.tex');
    fixtureBytesByCase.set(entry.case_id, fs.readFileSync(fixturePath));
  }
  const fixtureBytes = fixtureBytesByCase.get(entry.case_id);
  if (sourceSpan.end_byte > fixtureBytes.length) {
    console.error(`FAIL: resource_hints_v0.entries[${index}] source_span out of fixture bounds`);
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
  ['texmf', 'tex', 'appendices__apx_a.tex', 'typeset'],
  ['texmf', 'tex', 'appendices__apx_b.tex', 'typeset'],
  ['texmf', 'sty', 'xcolor.sty', 'typeset'],
  ['texmf', 'sty', 'foo__bar.sty', 'typeset'],
  ['texmf', 'sty', 'fooopts.sty', 'typeset'],
  ['texmf', 'sty', 'baropts.sty', 'typeset'],
  ['texmf', 'sty', 'pkgoptsdemo.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__a.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__b.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__c.sty', 'typeset'],
  ['texmf', 'cls', 'classoptsdemo.cls', 'typeset'],
  ['texmf', 'cls', 'classoptsmulti.cls', 'typeset'],
  ['texmf', 'cls', 'memoir.cls', 'typeset'],
  ['texmf', 'cls', 'memoirplus.cls', 'typeset'],
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
  ['texmf', 'pdf', 'figs__demo_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'plots__demo_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'figs__banner_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'figs__sub__banner_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__figs__multi_probe.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__plots__multi_probe.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__hires__chart.pdf', 'typeset'],
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
const baselineCmpClasses = new Set(['MATCH', 'DIFF_OK', 'DIFF_SUSPECT', 'MISSING_BASELINE', 'SKIP']);
if (report?.typed_artifacts_version !== 1) {
  console.error('FAIL: expected report.typed_artifacts_version=1 after rerun');
  process.exit(1);
}
const deltaPolicy = report?.delta_policy_v1;
if (!deltaPolicy || typeof deltaPolicy !== 'object') {
  console.error('FAIL: expected report.delta_policy_v1 after rerun');
  process.exit(1);
}
if (typeof deltaPolicy.path !== 'string' || deltaPolicy.path.length === 0) {
  console.error('FAIL: expected report.delta_policy_v1.path');
  process.exit(1);
}
if (typeof deltaPolicy.sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(deltaPolicy.sha256)) {
  console.error('FAIL: expected report.delta_policy_v1.sha256');
  process.exit(1);
}
if (deltaPolicy.ok_cases_require_match !== true) {
  console.error('FAIL: expected report.delta_policy_v1.ok_cases_require_match=true');
  process.exit(1);
}
if (typeof deltaPolicy.ok_allowlist_case_count !== 'number' || deltaPolicy.ok_allowlist_case_count < 0) {
  console.error('FAIL: expected report.delta_policy_v1.ok_allowlist_case_count');
  process.exit(1);
}
if (typeof deltaPolicy.metrics_thresholds !== 'object' || !deltaPolicy.metrics_thresholds) {
  console.error('FAIL: expected report.delta_policy_v1.metrics_thresholds');
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
if (resourceHints.resource_hints_v0_version !== 1) {
  console.error('FAIL: expected report.resource_hints_v0.resource_hints_v0_version=1 after rerun');
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
if (resolvedCount < 45) {
  console.error(`FAIL: expected resolved_resources_count >= 45 after includegraphics multipath expansion, got ${resolvedCount}`);
  process.exit(1);
}
const okStatuses = statuses.filter((entry) => entry.status === 'OK');
if (okStatuses.length <= 0) {
  console.error('FAIL: expected at least one OK case in fixture gallery report');
  process.exit(1);
}
const documentclassInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_documentclass_invalid_probe_v0');
if (!documentclassInvalidStatus || documentclassInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_documentclass_invalid_probe_v0 status INVALID');
  process.exit(1);
}
const documentclassEmptyOptsInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_documentclass_emptyopts_invalid_probe_v0');
if (!documentclassEmptyOptsInvalidStatus || documentclassEmptyOptsInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_documentclass_emptyopts_invalid_probe_v0 status INVALID');
  process.exit(1);
}
const usepackageEmptyOptsInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_usepackage_emptyopts_invalid_probe_v0');
if (!usepackageEmptyOptsInvalidStatus || usepackageEmptyOptsInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_usepackage_emptyopts_invalid_probe_v0 status INVALID');
  process.exit(1);
}
const usepackageMultipackageInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_usepackage_multipackage_invalid_probe_v0');
if (!usepackageMultipackageInvalidStatus || usepackageMultipackageInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_usepackage_multipackage_invalid_probe_v0 status INVALID');
  process.exit(1);
}
const graphicsOptionsInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_graphics_opts_invalid_probe_v0');
if (!graphicsOptionsInvalidStatus || graphicsOptionsInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_graphics_opts_invalid_probe_v0 status INVALID');
  process.exit(1);
}
const resourceHintsInvalidStatus = statuses.find((entry) => entry.case_id === 'typeset_demo_resource_hints_invalid_probe_v0');
if (!resourceHintsInvalidStatus || resourceHintsInvalidStatus.status !== 'INVALID') {
  console.error('FAIL: expected typeset_demo_resource_hints_invalid_probe_v0 status INVALID');
  process.exit(1);
}
for (const status of okStatuses) {
  if (status.baseline_match !== 'MATCH') {
    console.error(`FAIL: expected baseline_match=MATCH for OK case ${status.case_id}`);
    process.exit(1);
  }
  if (status?.baseline_cmp_v1?.class !== 'MATCH') {
    console.error(`FAIL: expected baseline_cmp_v1.class=MATCH for OK case ${status.case_id}`);
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
  const baselineCmp = status?.baseline_cmp_v1;
  if (!baselineCmp || typeof baselineCmp !== 'object') {
    console.error(`FAIL: case ${status.case_id} missing baseline_cmp_v1`);
    process.exit(1);
  }
  if (!baselineCmpClasses.has(baselineCmp.class)) {
    console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.class invalid: ${baselineCmp.class}`);
    process.exit(1);
  }
  if (!Array.isArray(baselineCmp.reasons) || baselineCmp.reasons.length === 0) {
    console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.reasons missing`);
    process.exit(1);
  }
  const metrics = baselineCmp.metrics;
  if (!metrics || typeof metrics !== 'object') {
    console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.metrics missing`);
    process.exit(1);
  }
  const requiredMetricKeys = [
    'page_count',
    'total_lines',
    'total_glyphs',
    'max_line_glyphs',
    'annots_count',
    'footnote_marker_count',
    'xdv_sha256',
    'pdf_sha256',
  ];
  for (const key of requiredMetricKeys) {
    if (!(key in metrics)) {
      console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.metrics missing ${key}`);
      process.exit(1);
    }
  }
  if (typeof metrics.page_count !== 'number' || metrics.page_count < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.page_count`);
    process.exit(1);
  }
  if (typeof metrics.total_lines !== 'number' || metrics.total_lines < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.total_lines`);
    process.exit(1);
  }
  if (typeof metrics.total_glyphs !== 'number' || metrics.total_glyphs < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.total_glyphs`);
    process.exit(1);
  }
  if (typeof metrics.max_line_glyphs !== 'number' || metrics.max_line_glyphs < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.max_line_glyphs`);
    process.exit(1);
  }
  if (typeof metrics.annots_count !== 'number' || metrics.annots_count < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.annots_count`);
    process.exit(1);
  }
  if (typeof metrics.footnote_marker_count !== 'number' || metrics.footnote_marker_count < 0) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.footnote_marker_count`);
    process.exit(1);
  }
  if (typeof metrics.xdv_sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(metrics.xdv_sha256)) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.xdv_sha256`);
    process.exit(1);
  }
  if (typeof metrics.pdf_sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(metrics.pdf_sha256)) {
    console.error(`FAIL: case ${status.case_id} invalid baseline_cmp_v1.metrics.pdf_sha256`);
    process.exit(1);
  }
  if (status.baseline_match === 'MATCH' && baselineCmp.class !== 'MATCH') {
    console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.class must be MATCH when baseline_match=MATCH`);
    process.exit(1);
  }
  if (status.baseline_match === 'MISSING' && baselineCmp.class !== 'MISSING_BASELINE') {
    console.error(`FAIL: case ${status.case_id} baseline_cmp_v1.class must be MISSING_BASELINE when baseline_match=MISSING`);
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
console.log('PASS: resolved_resources_count meets floor >= 45');
console.log(`PASS: baseline_match MATCH for all OK cases (${okStatuses.length})`);
console.log('PASS: baseline_cmp_v1 policy+metrics gates pass');
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

node - "$ROOT_DIR" "$ONDEMAND_OUT_DIR" "$STORE_DIR" "$FIXTURE_SOURCE_DIR" <<'NODE'
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const { spawn } = require('node:child_process');

const rootDir = process.argv[2];
const ondemandOutDir = process.argv[3];
const storeDir = process.argv[4];
const fixtureSourceDir = process.argv[5];
const sourceDateEpoch = process.env.SOURCE_DATE_EPOCH ?? '1700000000';

const assert = (condition, message) => {
  if (!condition) {
    console.error(`FAIL: ${message}`);
    process.exit(1);
  }
};

const makeStableId = (prefix, parts) => {
  const token = parts.join('_').replace(/[^A-Za-z0-9_:-]/g, '_');
  return `${prefix}_${token}`.slice(0, 120);
};

const mapEndpointPathToFixturePath = (pathname) => {
  const parts = pathname.split('/').filter(Boolean);
  if (parts.length === 0) {
    return null;
  }
  if (parts[0] === 'xetex' && parts.length === 3) {
    return {
      filePath: path.join(fixtureSourceDir, 'xetex', parts[1], parts[2]),
      stableId: makeStableId('fileid', parts),
      headerName: 'fileid',
    };
  }
  if (parts[0] === 'fontconfig' && parts.length === 3) {
    return {
      filePath: path.join(fixtureSourceDir, 'fontconfig', parts[1], parts[2]),
      stableId: makeStableId('fontid', parts),
      headerName: 'fontid',
    };
  }
  return null;
};

const server = http.createServer((req, res) => {
  const pathname = new URL(req.url ?? '/', 'http://127.0.0.1').pathname;
  const mapped = mapEndpointPathToFixturePath(pathname);
  if (!mapped || !fs.existsSync(mapped.filePath)) {
    res.statusCode = 404;
    res.end('not found');
    return;
  }
  const bytes = fs.readFileSync(mapped.filePath);
  if (!bytes.length) {
    res.statusCode = 404;
    res.end('not found');
    return;
  }
  res.statusCode = 200;
  res.setHeader('content-type', 'application/octet-stream');
  res.setHeader(mapped.headerName, mapped.stableId);
  res.end(bytes);
});

const runNodeProcess = (args, env) =>
  new Promise((resolve) => {
    const child = spawn('node', args, {
      cwd: rootDir,
      stdio: ['ignore', 'pipe', 'pipe'],
      env,
    });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString('utf8');
    });
    child.stderr.on('data', (chunk) => {
      stderr += chunk.toString('utf8');
    });
    child.on('close', (code) => {
      resolve({ code, stdout, stderr });
    });
  });

server.listen(0, '127.0.0.1', async () => {
  const address = server.address();
  if (!address || typeof address !== 'object') {
    console.error('FAIL: could not bind ondemand fixture endpoint');
    process.exit(1);
  }
  const endpoint = `http://127.0.0.1:${address.port}`;
  try {
    const baselineOutDir = `${ondemandOutDir}_baseline`;
    fs.rmSync(baselineOutDir, { recursive: true, force: true });
    fs.rmSync(ondemandOutDir, { recursive: true, force: true });

    const baselineRun = await runNodeProcess(
      [path.join(rootDir, 'scripts', 'wasm_fixture_gallery_v0.mjs'), baselineOutDir],
      {
        ...process.env,
        SOURCE_DATE_EPOCH: sourceDateEpoch,
        TZ: 'UTC',
        TEXLIVE_RESOLVER_BACKEND_V0: 'offline_store_v0',
        TEXLIVE_STORE_DIR_V0: storeDir,
      },
    );
    if (baselineRun.code !== 0) {
      process.stderr.write(baselineRun.stdout ?? '');
      process.stderr.write(baselineRun.stderr ?? '');
      console.error('FAIL: baseline gallery run for ondemand integration failed');
      process.exit(1);
    }

    const ondemandRun = await runNodeProcess(
      [path.join(rootDir, 'scripts', 'wasm_fixture_gallery_v0.mjs'), ondemandOutDir],
      {
        ...process.env,
        SOURCE_DATE_EPOCH: sourceDateEpoch,
        TZ: 'UTC',
        TEXLIVE_RESOLVER_BACKEND_V0: 'offline_store_v0',
        TEXLIVE_STORE_DIR_V0: storeDir,
        TEXLIVE_ENDPOINT: endpoint,
        WASM_GALLERY_ENABLE_ONDEMAND_V1: '1',
        WASM_GALLERY_ONDEMAND_MAX_ITERS_V1: '3',
      },
    );
    if (ondemandRun.code !== 0) {
      process.stderr.write(ondemandRun.stdout ?? '');
      process.stderr.write(ondemandRun.stderr ?? '');
      console.error('FAIL: ondemand-enabled fixture gallery run failed');
      process.exit(1);
    }

    const baselineReport = JSON.parse(fs.readFileSync(path.join(baselineOutDir, 'report.json'), 'utf8'));
    const ondemandReport = JSON.parse(fs.readFileSync(path.join(ondemandOutDir, 'report.json'), 'utf8'));
    assert(Array.isArray(ondemandReport.statuses), 'ondemand report statuses missing');
    assert(
      typeof ondemandReport.resolved_resources_count === 'number',
      'ondemand report missing resolved_resources_count',
    );
    assert(
      ondemandReport.resolved_resources_count > baselineReport.resolved_resources_count,
      'ondemand run must increase resolved_resources_count',
    );
    const requiredCases = [
      'typeset_demo_ondemand_input_probe_v0',
      'typeset_demo_ondemand_include_probe_v0',
    ];
    for (const caseId of requiredCases) {
      const status = ondemandReport.statuses.find((entry) => entry.case_id === caseId);
      assert(status, `ondemand report missing status for ${caseId}`);
      assert(typeof status.config_hash === 'string' && status.config_hash.length > 0, `${caseId} missing config_hash`);
      assert(status.input_sha256 && typeof status.input_sha256.main_tex === 'string', `${caseId} missing input sha`);
      assert(Number.isInteger(status.missing_before), `${caseId} missing_before not integer`);
      assert(Number.isInteger(status.missing_after), `${caseId} missing_after not integer`);
      assert(Number.isInteger(status.resolved_resources_count), `${caseId} resolved_resources_count not integer`);
      assert(status.missing_before > 0, `${caseId} missing_before must be > 0`);
      assert(status.missing_after < status.missing_before, `${caseId} missing_after must decrease`);
      assert(status.ondemand_v1?.attempted === true, `${caseId} ondemand attempt flag missing`);
      assert(
        status.status === 'NI' || status.status === 'INVALID' || status.status === 'MISMATCH',
        `${caseId} status must remain fail-closed`,
      );
    }

    console.log(`PASS: ondemand run resolved_resources_count ${baselineReport.resolved_resources_count} -> ${ondemandReport.resolved_resources_count}`);
    console.log('PASS: ondemand per-case missing_before/missing_after fields and retries verified');
  } finally {
    server.close();
  }
});
NODE

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
