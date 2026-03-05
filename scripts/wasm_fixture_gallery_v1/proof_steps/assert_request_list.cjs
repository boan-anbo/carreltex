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
const hyperrefPackageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'hyperref.sty' && request.variant === 'typeset',
);
if (!hyperrefPackageRequest) {
  console.error('FAIL: request list must include package hint request for hyperref.sty');
  process.exit(1);
}
const graphicxPackageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'graphicx.sty' && request.variant === 'typeset',
);
if (!graphicxPackageRequest) {
  console.error('FAIL: request list must include package hint request for graphicx.sty');
  process.exit(1);
}
const normalizedOptionsPackageRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.name === 'foo.sty' && request.variant === 'typeset',
);
if (!normalizedOptionsPackageRequest) {
  console.error('FAIL: request list must include normalized option package hint request for foo.sty');
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
const inputCycleProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'cycles__a.tex' && request.variant === 'typeset',
);
if (!inputCycleProbeRequest) {
  console.error('FAIL: request list must include input cycle probe hint request for cycles__a.tex');
  process.exit(1);
}
const inputMissingProbeRequest = listA.requests.find(
  (request) => request.kind === 'texmf' && request.format === 'tex' && request.name === 'missing__section.tex' && request.variant === 'typeset',
);
if (!inputMissingProbeRequest) {
  console.error('FAIL: request list must include input missing probe hint request for missing__section.tex');
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
