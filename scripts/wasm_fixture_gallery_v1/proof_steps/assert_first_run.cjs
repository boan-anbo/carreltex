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
const labelsPath = path.join(outDir, 'typeset_demo_labels_probe_v0', 'labels_v1.json');
if (!fs.existsSync(labelsPath)) {
  console.error('FAIL: expected labels_v1.json artifact after first run');
  process.exit(1);
}
const labelsShaFirst = labelsSummary.typed_artifacts.labels.artifact_sha256;
if (typeof labelsShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(labelsShaFirst)) {
  console.error('FAIL: expected labels artifact sha256 in first summary');
  process.exit(1);
}
const labelsArtifactFirst = JSON.parse(fs.readFileSync(labelsPath, 'utf8'));
if (!Array.isArray(labelsArtifactFirst?.entries)) {
  console.error('FAIL: expected labels_v1.entries array in first-run artifact');
  process.exit(1);
}
if (labelsArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty labels_v1.entries for labels probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_labels_probe_v0', 'labels_v1', labelsArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('labels_v1'), `${labelsShaFirst}\n`);
if (labelsSummary?.typed_artifacts?.refs?.present !== true) {
  console.error('FAIL: expected refs typed artifact present after first run');
  process.exit(1);
}
const refsPath = path.join(outDir, 'typeset_demo_labels_probe_v0', 'refs_v1.json');
if (!fs.existsSync(refsPath)) {
  console.error('FAIL: expected refs_v1.json artifact after first run');
  process.exit(1);
}
const refsShaFirst = labelsSummary.typed_artifacts.refs.artifact_sha256;
if (typeof refsShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(refsShaFirst)) {
  console.error('FAIL: expected refs artifact sha256 in first summary');
  process.exit(1);
}
const refsArtifactFirst = JSON.parse(fs.readFileSync(refsPath, 'utf8'));
if (!Array.isArray(refsArtifactFirst?.entries)) {
  console.error('FAIL: expected refs_v1.entries array in first-run artifact');
  process.exit(1);
}
if (refsArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty refs_v1.entries for labels probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_labels_probe_v0', 'refs_v1', refsArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('refs_v1'), `${refsShaFirst}\n`);

const pagerefProbeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pageref_probe_v2', 'summary.json'), 'utf8'),
);
if (pagerefProbeSummary?.typed_artifacts?.pageref?.present !== true) {
  console.error('FAIL: expected pageref typed artifact present for pageref probe after first run');
  process.exit(1);
}
const pagerefIncludeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pageref_include_probe_v2', 'summary.json'), 'utf8'),
);
if (pagerefIncludeSummary?.typed_artifacts?.pageref?.present !== true) {
  console.error('FAIL: expected pageref typed artifact present for pageref include probe after first run');
  process.exit(1);
}
const pagerefUnresolvedSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pageref_unresolved_probe_v2', 'summary.json'), 'utf8'),
);
if (pagerefUnresolvedSummary?.typed_artifacts?.pageref?.present !== true) {
  console.error('FAIL: expected pageref typed artifact present for pageref unresolved probe after first run');
  process.exit(1);
}
const pagerefPath = path.join(outDir, 'typeset_demo_pageref_probe_v2', 'pageref_v2.json');
if (!fs.existsSync(pagerefPath)) {
  console.error('FAIL: expected pageref_v2.json artifact after first run');
  process.exit(1);
}
const pagerefShaFirst = pagerefProbeSummary.typed_artifacts.pageref.artifact_sha256;
if (typeof pagerefShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(pagerefShaFirst)) {
  console.error('FAIL: expected pageref artifact sha256 in first summary');
  process.exit(1);
}
const pagerefArtifactFirst = JSON.parse(fs.readFileSync(pagerefPath, 'utf8'));
if (!Array.isArray(pagerefArtifactFirst?.entries)) {
  console.error('FAIL: expected pageref_v2.entries array in first-run artifact');
  process.exit(1);
}
if (pagerefArtifactFirst?.schema !== 'pageref_v2') {
  console.error('FAIL: expected pageref_v2 schema in first-run artifact');
  process.exit(1);
}
if (pagerefArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty pageref_v2.entries for pageref probe');
  process.exit(1);
}
for (const [index, entry] of pagerefArtifactFirst.entries.entries()) {
  if (typeof entry?.key !== 'string' || entry.key.length === 0) {
    console.error(`FAIL: pageref_v2.entries[${index}] missing key`);
    process.exit(1);
  }
  if (typeof entry?.resolved !== 'boolean') {
    console.error(`FAIL: pageref_v2.entries[${index}] missing resolved boolean`);
    process.exit(1);
  }
  if (!(entry.anchor_id === null || (Number.isInteger(entry.anchor_id) && entry.anchor_id > 0))) {
    console.error(`FAIL: pageref_v2.entries[${index}] invalid anchor_id`);
    process.exit(1);
  }
  if (!(entry.page_no === null || (Number.isInteger(entry.page_no) && entry.page_no > 0))) {
    console.error(`FAIL: pageref_v2.entries[${index}] invalid page_no`);
    process.exit(1);
  }
  if (!Array.isArray(entry?.occurrences) || entry.occurrences.length <= 0) {
    console.error(`FAIL: pageref_v2.entries[${index}] missing occurrences`);
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_pageref_probe_v2', 'pageref_v2', pagerefArtifactFirst.entries);
const pagerefIncludeArtifactFirst = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pageref_include_probe_v2', 'pageref_v2.json'), 'utf8'),
);
if (!Array.isArray(pagerefIncludeArtifactFirst?.entries) || pagerefIncludeArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty pageref_v2.entries for pageref include probe');
  process.exit(1);
}
const pagerefIncludeEntryFirst = pagerefIncludeArtifactFirst.entries.find((entry) => entry.key === 'sec:two');
if (!pagerefIncludeEntryFirst) {
  console.error('FAIL: expected pageref include probe to include key sec:two');
  process.exit(1);
}
if (pagerefIncludeEntryFirst.resolved === false) {
  if (pagerefIncludeEntryFirst.page_no !== null || pagerefIncludeEntryFirst.anchor_id !== null) {
    console.error('FAIL: unresolved first-run pageref include entry must keep null page_no/anchor_id');
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_pageref_include_probe_v2', 'pageref_v2', pagerefIncludeArtifactFirst.entries);
const pagerefUnresolvedArtifactFirst = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_pageref_unresolved_probe_v2', 'pageref_v2.json'), 'utf8'),
);
if (!Array.isArray(pagerefUnresolvedArtifactFirst?.entries) || pagerefUnresolvedArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty pageref_v2.entries for pageref unresolved probe');
  process.exit(1);
}
if (!pagerefUnresolvedArtifactFirst.entries.some((entry) => entry.resolved === false && entry.page_no === null)) {
  console.error('FAIL: expected pageref unresolved probe to include unresolved entry');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_pageref_unresolved_probe_v2', 'pageref_v2', pagerefUnresolvedArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('pageref_v2'), `${pagerefShaFirst}\n`);

const tocSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'summary.json'), 'utf8'),
);
if (tocSummary?.typed_artifacts?.toc?.present !== true) {
  console.error('FAIL: expected toc typed artifact present after first run');
  process.exit(1);
}
const tocPath = path.join(outDir, 'typeset_demo_toc_probe_v0', 'toc_v2.json');
if (!fs.existsSync(tocPath)) {
  console.error('FAIL: expected toc_v2.json artifact after first run');
  process.exit(1);
}
const tocShaFirst = tocSummary.typed_artifacts.toc.artifact_sha256;
if (typeof tocShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(tocShaFirst)) {
  console.error('FAIL: expected toc artifact sha256 in first summary');
  process.exit(1);
}
const tocArtifactFirst = JSON.parse(fs.readFileSync(tocPath, 'utf8'));
if (!Array.isArray(tocArtifactFirst?.entries)) {
  console.error('FAIL: expected toc_v2.entries array in first-run artifact');
  process.exit(1);
}
if (tocArtifactFirst?.schema !== 'toc_v2') {
  console.error('FAIL: expected toc_v2 schema in first-run artifact');
  process.exit(1);
}
if (tocArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty toc_v2.entries for toc probe');
  process.exit(1);
}
for (const [index, entry] of tocArtifactFirst.entries.entries()) {
  if (!Number.isInteger(entry?.level) || entry.level < 1 || entry.level > 2) {
    console.error(`FAIL: toc_v2.entries[${index}] must have level in [1,2]`);
    process.exit(1);
  }
  if (typeof entry?.anchor_id !== 'string' || !/^h[1-9]\d*$/.test(entry.anchor_id)) {
    console.error(`FAIL: toc_v2.entries[${index}] must have anchor_id like hN`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.page_no) || entry.page_no <= 0) {
    console.error(`FAIL: toc_v2.entries[${index}].page_no must be positive integer`);
    process.exit(1);
  }
}
const tocDistinctPagesFirst = new Set(tocArtifactFirst.entries.map((entry) => entry.page_no));
if (tocDistinctPagesFirst.size < 2 || ![...tocDistinctPagesFirst].some((pageNo) => pageNo >= 2)) {
  console.error('FAIL: expected toc_v2 probe to include mixed page_no values including page 2');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_toc_probe_v0', 'toc_v2', tocArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('toc_v2'), `${tocShaFirst}\n`);

const bibSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'summary.json'), 'utf8'),
);
if (bibSummary?.typed_artifacts?.bib?.present !== true) {
  console.error('FAIL: expected bib typed artifact present after first run');
  process.exit(1);
}
if (bibSummary?.typed_artifacts?.cite?.present !== true) {
  console.error('FAIL: expected cite typed artifact present after first run');
  process.exit(1);
}
const bibPath = path.join(outDir, 'typeset_demo_bib_probe_v0', 'bib_v1.json');
if (!fs.existsSync(bibPath)) {
  console.error('FAIL: expected bib_v1.json artifact after first run');
  process.exit(1);
}
const citePath = path.join(outDir, 'typeset_demo_bib_probe_v0', 'cite_v1.json');
if (!fs.existsSync(citePath)) {
  console.error('FAIL: expected cite_v1.json artifact after first run');
  process.exit(1);
}
const bibShaFirst = bibSummary.typed_artifacts.bib.artifact_sha256;
if (typeof bibShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(bibShaFirst)) {
  console.error('FAIL: expected bib artifact sha256 in first summary');
  process.exit(1);
}
const citeShaFirst = bibSummary.typed_artifacts.cite.artifact_sha256;
if (typeof citeShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(citeShaFirst)) {
  console.error('FAIL: expected cite artifact sha256 in first summary');
  process.exit(1);
}
const bibArtifactFirst = JSON.parse(fs.readFileSync(bibPath, 'utf8'));
if (!Array.isArray(bibArtifactFirst?.entries)) {
  console.error('FAIL: expected bib_v1.entries array in first-run artifact');
  process.exit(1);
}
if (bibArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty bib_v1.entries for bib probe');
  process.exit(1);
}
if (bibArtifactFirst?.schema !== 'bib_v1') {
  console.error('FAIL: expected bib_v1 schema in first-run artifact');
  process.exit(1);
}
for (const [index, entry] of bibArtifactFirst.entries.entries()) {
  if (typeof entry?.key !== 'string' || entry.key.length === 0) {
    console.error(`FAIL: bib_v1.entries[${index}] missing key`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.ordinal) || entry.ordinal <= 0) {
    console.error(`FAIL: bib_v1.entries[${index}] invalid ordinal`);
    process.exit(1);
  }
  if (typeof entry?.text_sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(entry.text_sha256)) {
    console.error(`FAIL: bib_v1.entries[${index}] invalid text_sha256`);
    process.exit(1);
  }
}
const expectedBibOrder = ['demo-key', 'aux-key', 'third-key'];
const actualBibOrder = bibArtifactFirst.entries.map((entry) => entry.key);
if (JSON.stringify(actualBibOrder) !== JSON.stringify(expectedBibOrder)) {
  console.error(`FAIL: expected bib_v1 key order ${JSON.stringify(expectedBibOrder)} but got ${JSON.stringify(actualBibOrder)}`);
  process.exit(1);
}
const bibOrdinals = bibArtifactFirst.entries.map((entry) => entry.ordinal);
if (JSON.stringify(bibOrdinals) !== JSON.stringify([1, 2, 3])) {
  console.error(`FAIL: expected bib_v1 ordinals [1,2,3] but got ${JSON.stringify(bibOrdinals)}`);
  process.exit(1);
}
const citeArtifactFirst = JSON.parse(fs.readFileSync(citePath, 'utf8'));
if (!Array.isArray(citeArtifactFirst?.entries)) {
  console.error('FAIL: expected cite_v1.entries array in first-run artifact');
  process.exit(1);
}
if (citeArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty cite_v1.entries for bib probe');
  process.exit(1);
}
if (citeArtifactFirst?.schema !== 'cite_v1') {
  console.error('FAIL: expected cite_v1 schema in first-run artifact');
  process.exit(1);
}
for (const [index, entry] of citeArtifactFirst.entries.entries()) {
  if (typeof entry?.key !== 'string' || entry.key.length === 0) {
    console.error(`FAIL: cite_v1.entries[${index}] missing key`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.line_index) || entry.line_index <= 0) {
    console.error(`FAIL: cite_v1.entries[${index}] invalid line_index`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.cite_order) || entry.cite_order <= 0) {
    console.error(`FAIL: cite_v1.entries[${index}] invalid cite_order`);
    process.exit(1);
  }
  if (entry?.resolved !== true) {
    console.error(`FAIL: cite_v1.entries[${index}] expected resolved=true`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.ordinal) || entry.ordinal <= 0) {
    console.error(`FAIL: cite_v1.entries[${index}] invalid ordinal`);
    process.exit(1);
  }
}
const expectedCiteOrder = ['demo-key', 'aux-key', 'third-key', 'demo-key', 'aux-key'];
const actualCiteOrder = citeArtifactFirst.entries.map((entry) => entry.key);
if (JSON.stringify(actualCiteOrder) !== JSON.stringify(expectedCiteOrder)) {
  console.error(`FAIL: expected cite_v1 occurrence order ${JSON.stringify(expectedCiteOrder)} but got ${JSON.stringify(actualCiteOrder)}`);
  process.exit(1);
}
const actualCiteOrdinals = citeArtifactFirst.entries.map((entry) => entry.ordinal);
if (JSON.stringify(actualCiteOrdinals) !== JSON.stringify([1, 2, 3, 1, 2])) {
  console.error(`FAIL: expected cite_v1 ordinal mapping [1,2,3,1,2] but got ${JSON.stringify(actualCiteOrdinals)}`);
  process.exit(1);
}
const citeOrderByKey = new Map();
for (const entry of citeArtifactFirst.entries) {
  if (!citeOrderByKey.has(entry.key)) {
    citeOrderByKey.set(entry.key, entry.cite_order);
  } else if (citeOrderByKey.get(entry.key) !== entry.cite_order) {
    console.error(`FAIL: cite_v1 cite_order drift for key ${entry.key}`);
    process.exit(1);
  }
}
if (JSON.stringify([...citeOrderByKey.entries()]) !== JSON.stringify([
  ['demo-key', 1],
  ['aux-key', 2],
  ['third-key', 3],
])) {
  console.error(`FAIL: expected cite_v1 cite_order mapping [[demo-key,1],[aux-key,2],[third-key,3]] but got ${JSON.stringify([...citeOrderByKey.entries()])}`);
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_bib_probe_v0', 'bib_v1', bibArtifactFirst.entries);
assertEntrySourceSpans('typeset_demo_bib_probe_v0', 'cite_v1', citeArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('bib_v1'), `${bibShaFirst}\n`);
fs.writeFileSync(firstRunShaPath('cite_v1'), `${citeShaFirst}\n`);

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

const packageCaptureSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_capture_probe_v1', 'summary.json'), 'utf8'),
);
if (packageCaptureSummary?.typed_artifacts?.packages?.present !== true) {
  console.error('FAIL: expected packages typed artifact present for usepackage capture probe after first run');
  process.exit(1);
}
const packageCapturePath = path.join(outDir, 'typeset_demo_usepackage_capture_probe_v1', 'packages_v1.json');
if (!fs.existsSync(packageCapturePath)) {
  console.error('FAIL: expected packages_v1.json artifact for usepackage capture probe');
  process.exit(1);
}
const packageCaptureArtifact = JSON.parse(fs.readFileSync(packageCapturePath, 'utf8'));
if (packageCaptureArtifact?.schema !== 'packages_v1') {
  console.error('FAIL: expected packages_v1 schema for usepackage capture probe');
  process.exit(1);
}
if (!Array.isArray(packageCaptureArtifact?.entries) || packageCaptureArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty packages_v1.entries for usepackage capture probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_usepackage_capture_probe_v1', 'packages_v1', packageCaptureArtifact.entries);
const hyperrefPackageEntry = packageCaptureArtifact.entries.find((entry) => entry.name === 'hyperref.sty');
if (!hyperrefPackageEntry) {
  console.error('FAIL: expected packages_v1 to include hyperref.sty entry for usepackage capture probe');
  process.exit(1);
}
if (JSON.stringify(hyperrefPackageEntry.options) !== JSON.stringify(['unicode'])) {
  console.error('FAIL: expected usepackage capture probe options normalized as [unicode]');
  process.exit(1);
}
const packagesShaFirst = packageCaptureSummary.typed_artifacts.packages.artifact_sha256;
if (typeof packagesShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(packagesShaFirst)) {
  console.error('FAIL: expected packages_v1 artifact sha256 in first summary');
  process.exit(1);
}
fs.writeFileSync(firstRunShaPath('packages_v1'), `${packagesShaFirst}\n`);

const mathProbeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_math_probe_v0', 'summary.json'), 'utf8'),
);
if (mathProbeSummary?.status !== 'OK' || mathProbeSummary?.compile_status !== 'OK') {
  console.error('FAIL: expected typeset_demo_math_probe_v0 status/compile_status OK after first run');
  process.exit(1);
}
if (mathProbeSummary?.typed_artifacts?.packages?.present !== true) {
  console.error('FAIL: expected packages_v1 artifact present for typeset_demo_math_probe_v0 after first run');
  process.exit(1);
}
const mathProbePackagesArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_math_probe_v0', 'packages_v1.json'), 'utf8'),
);
if (!Array.isArray(mathProbePackagesArtifact?.entries) || mathProbePackagesArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty packages_v1 entries for typeset_demo_math_probe_v0');
  process.exit(1);
}
if (!mathProbePackagesArtifact.entries.some((entry) => entry.name === 'amsmath.sty')) {
  console.error('FAIL: expected typeset_demo_math_probe_v0 packages_v1 entries to include amsmath.sty');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_math_probe_v0', 'packages_v1', mathProbePackagesArtifact.entries);
const mathProbeResourceHintsArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_math_probe_v0', 'resource_hints_v0.json'), 'utf8'),
);
if (!Array.isArray(mathProbeResourceHintsArtifact?.entries)) {
  console.error('FAIL: expected resource_hints_v0.entries for typeset_demo_math_probe_v0');
  process.exit(1);
}
if (!mathProbeResourceHintsArtifact.entries.some((entry) => entry.hint_type === 'package_file' && entry.value === 'amsmath.sty')) {
  console.error('FAIL: expected typeset_demo_math_probe_v0 resource_hints_v0 package_file=amsmath.sty');
  process.exit(1);
}

const cjkProbeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_cjk_probe_v0', 'summary.json'), 'utf8'),
);
if (cjkProbeSummary?.status !== 'OK' || cjkProbeSummary?.compile_status !== 'OK') {
  console.error('FAIL: expected typeset_demo_cjk_probe_v0 status/compile_status OK after first run');
  process.exit(1);
}
if (cjkProbeSummary?.typed_artifacts?.packages?.present !== true) {
  console.error('FAIL: expected packages_v1 artifact present for typeset_demo_cjk_probe_v0 after first run');
  process.exit(1);
}
const cjkProbePackagesArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_cjk_probe_v0', 'packages_v1.json'), 'utf8'),
);
if (!Array.isArray(cjkProbePackagesArtifact?.entries) || cjkProbePackagesArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty packages_v1 entries for typeset_demo_cjk_probe_v0');
  process.exit(1);
}
if (!cjkProbePackagesArtifact.entries.some((entry) => entry.name === 'xeCJK.sty')) {
  console.error('FAIL: expected typeset_demo_cjk_probe_v0 packages_v1 entries to include xeCJK.sty');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_cjk_probe_v0', 'packages_v1', cjkProbePackagesArtifact.entries);
const cjkProbeResourceHintsArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_cjk_probe_v0', 'resource_hints_v0.json'), 'utf8'),
);
if (!Array.isArray(cjkProbeResourceHintsArtifact?.entries)) {
  console.error('FAIL: expected resource_hints_v0.entries for typeset_demo_cjk_probe_v0');
  process.exit(1);
}
if (!cjkProbeResourceHintsArtifact.entries.some((entry) => entry.hint_type === 'package_file' && entry.value === 'xeCJK.sty')) {
  console.error('FAIL: expected typeset_demo_cjk_probe_v0 resource_hints_v0 package_file=xeCJK.sty');
  process.exit(1);
}

const packageMultiCaptureArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_multi_capture_probe_v1', 'packages_v1.json'), 'utf8'),
);
if (!Array.isArray(packageMultiCaptureArtifact?.entries) || packageMultiCaptureArtifact.entries.length < 2) {
  console.error('FAIL: expected multiple packages_v1 entries for usepackage multi capture probe');
  process.exit(1);
}
const hasGraphicx = packageMultiCaptureArtifact.entries.some((entry) => entry.name === 'graphicx.sty');
const hasXcolor = packageMultiCaptureArtifact.entries.some((entry) => entry.name === 'xcolor.sty');
if (!hasGraphicx || !hasXcolor) {
  console.error('FAIL: expected packages_v1 multi capture entries for graphicx.sty and xcolor.sty');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_usepackage_multi_capture_probe_v1', 'packages_v1', packageMultiCaptureArtifact.entries);

const packageNormalizeArtifact = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_usepackage_opts_normalize_probe_v1', 'packages_v1.json'), 'utf8'),
);
if (!Array.isArray(packageNormalizeArtifact?.entries) || packageNormalizeArtifact.entries.length <= 0) {
  console.error('FAIL: expected non-empty packages_v1 entries for usepackage options normalize probe');
  process.exit(1);
}
const normalizeEntry = packageNormalizeArtifact.entries.find((entry) => entry.name === 'foo.sty');
if (!normalizeEntry) {
  console.error('FAIL: expected packages_v1 normalize probe entry for foo.sty');
  process.exit(1);
}
if (JSON.stringify(normalizeEntry.options) !== JSON.stringify(['a', 'b'])) {
  console.error('FAIL: expected normalized options [a,b] for usepackage options normalize probe');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_usepackage_opts_normalize_probe_v1', 'packages_v1', packageNormalizeArtifact.entries);

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
const graphicsPath = path.join(outDir, 'typeset_demo_graphics_probe_v0', 'graphics_v2.json');
if (!fs.existsSync(graphicsPath)) {
  console.error('FAIL: expected graphics_v2.json artifact after first run');
  process.exit(1);
}
const graphicsShaFirst = graphicsSummary.typed_artifacts.graphics.artifact_sha256;
if (typeof graphicsShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(graphicsShaFirst)) {
  console.error('FAIL: expected graphics artifact sha256 in first summary');
  process.exit(1);
}
const graphicsArtifactFirst = JSON.parse(fs.readFileSync(graphicsPath, 'utf8'));
if (!Array.isArray(graphicsArtifactFirst?.entries)) {
  console.error('FAIL: expected graphics_v2.entries array in first-run artifact');
  process.exit(1);
}
if (graphicsArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty graphics_v2.entries for graphics probe');
  process.exit(1);
}
for (const entry of graphicsArtifactFirst.entries) {
  if (typeof entry?.resolver_path !== 'string' || entry.resolver_path.length === 0) {
    console.error('FAIL: expected graphics_v2 entry resolver_path');
    process.exit(1);
  }
  if (typeof entry?.opts !== 'object' || entry.opts === null) {
    console.error('FAIL: expected graphics_v2 entry opts object');
    process.exit(1);
  }
  if (!(entry.opts.width_pt > 0) || !(entry.opts.height_pt > 0)) {
    console.error('FAIL: expected graphics_v2 entry positive width_pt/height_pt');
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_graphics_probe_v0', 'graphics_v2', graphicsArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('graphics_v2'), `${graphicsShaFirst}\n`);

const floatSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_float_probe_v0', 'summary.json'), 'utf8'),
);
if (floatSummary?.typed_artifacts?.float?.present !== true) {
  console.error('FAIL: expected float typed artifact present after first run');
  process.exit(1);
}
const floatPath = path.join(outDir, 'typeset_demo_float_probe_v0', 'float_v0.json');
if (!fs.existsSync(floatPath)) {
  console.error('FAIL: expected float_v0.json artifact after first run');
  process.exit(1);
}
const floatShaFirst = floatSummary.typed_artifacts.float.artifact_sha256;
if (typeof floatShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(floatShaFirst)) {
  console.error('FAIL: expected float artifact sha256 in first summary');
  process.exit(1);
}
const floatArtifactFirst = JSON.parse(fs.readFileSync(floatPath, 'utf8'));
if (!Array.isArray(floatArtifactFirst?.entries)) {
  console.error('FAIL: expected float_v0.entries array in first-run artifact');
  process.exit(1);
}
if (floatArtifactFirst?.schema !== 'float_v0') {
  console.error('FAIL: expected float_v0 schema in first-run artifact');
  process.exit(1);
}
if (floatArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty float_v0.entries for float probe');
  process.exit(1);
}
let sawTopPlacement = false;
let sawInlinePlacement = false;
const floatPageNos = new Set();
for (const [index, entry] of floatArtifactFirst.entries.entries()) {
  if (typeof entry?.float_id !== 'string' || !/^flt[1-9]\d*$/.test(entry.float_id)) {
    console.error(`FAIL: float_v0.entries[${index}] invalid float_id`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.figure_ordinal) || entry.figure_ordinal !== index + 1) {
    console.error(`FAIL: float_v0.entries[${index}] invalid figure_ordinal`);
    process.exit(1);
  }
  if (!(entry?.placement_hint === 'inline' || entry?.placement_hint === 't')) {
    console.error(`FAIL: float_v0.entries[${index}] invalid placement_hint`);
    process.exit(1);
  }
  if (entry.placement_hint === 't') {
    sawTopPlacement = true;
  }
  if (entry.placement_hint === 'inline') {
    sawInlinePlacement = true;
  }
  if (typeof entry?.caption_summary !== 'string' || entry.caption_summary.trim().length === 0) {
    console.error(`FAIL: float_v0.entries[${index}] invalid caption_summary`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.anchor_id) || entry.anchor_id <= 0) {
    console.error(`FAIL: float_v0.entries[${index}] invalid anchor_id`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.page_no) || entry.page_no <= 0) {
    console.error(`FAIL: float_v0.entries[${index}] invalid page_no`);
    process.exit(1);
  }
  floatPageNos.add(entry.page_no);
}
if (!sawTopPlacement || !sawInlinePlacement) {
  console.error('FAIL: expected float_v0 entries to include both top and inline placements');
  process.exit(1);
}
if (floatPageNos.size < 2) {
  console.error('FAIL: expected float_v0 entries to span at least two pages');
  process.exit(1);
}
assertEntrySourceSpans('typeset_demo_float_probe_v0', 'float_v0', floatArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('float_v0'), `${floatShaFirst}\n`);

const mathSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_minimal_v0', 'summary.json'), 'utf8'),
);
if (mathSummary?.typed_artifacts?.math?.present !== true) {
  console.error('FAIL: expected math typed artifact present for minimal demo after first run');
  process.exit(1);
}
const mathPath = path.join(outDir, 'typeset_demo_minimal_v0', 'math_v2.json');
if (!fs.existsSync(mathPath)) {
  console.error('FAIL: expected math_v2.json artifact after first run');
  process.exit(1);
}
const mathShaFirst = mathSummary.typed_artifacts.math.artifact_sha256;
if (typeof mathShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(mathShaFirst)) {
  console.error('FAIL: expected math artifact sha256 in first summary');
  process.exit(1);
}
const mathArtifactFirst = JSON.parse(fs.readFileSync(mathPath, 'utf8'));
if (!Array.isArray(mathArtifactFirst?.entries)) {
  console.error('FAIL: expected math_v2.entries array in first-run artifact');
  process.exit(1);
}
if (mathArtifactFirst?.schema !== 'math_v2') {
  console.error('FAIL: expected math_v2 schema in first-run artifact');
  process.exit(1);
}
if (mathArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty math_v2.entries for minimal demo');
  process.exit(1);
}
for (const [index, entry] of mathArtifactFirst.entries.entries()) {
  if (!Number.isInteger(entry?.ordinal) || entry.ordinal <= 0) {
    console.error(`FAIL: math_v2.entries[${index}] has invalid ordinal`);
    process.exit(1);
  }
  if (entry.ordinal !== index + 1) {
    console.error(`FAIL: math_v2.entries[${index}] ordinal is not stable sequence`);
    process.exit(1);
  }
  if (typeof entry?.payload_preview !== 'string' || entry.payload_preview.trim().length === 0) {
    console.error(`FAIL: math_v2.entries[${index}] has invalid payload_preview`);
    process.exit(1);
  }
  if (!/^[\x20-\x7e]+$/.test(entry.payload_preview)) {
    console.error(`FAIL: math_v2.entries[${index}] payload_preview must be ASCII printable`);
    process.exit(1);
  }
  if (typeof entry?.anchor_id !== 'string' || !/^eq[1-9]\d*$/.test(entry.anchor_id)) {
    console.error(`FAIL: math_v2.entries[${index}] has invalid anchor_id`);
    process.exit(1);
  }
  if (entry.anchor_id !== `eq${entry.ordinal}`) {
    console.error(`FAIL: math_v2.entries[${index}] anchor_id/ordinal mismatch`);
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_minimal_v0', 'math_v2', mathArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('math_v2'), `${mathShaFirst}\n`);

const tableSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_minimal_v0', 'summary.json'), 'utf8'),
);
if (tableSummary?.typed_artifacts?.table?.present !== true) {
  console.error('FAIL: expected table typed artifact present for minimal demo after first run');
  process.exit(1);
}
const tablePath = path.join(outDir, 'typeset_demo_minimal_v0', 'table_v2.json');
if (!fs.existsSync(tablePath)) {
  console.error('FAIL: expected table_v2.json artifact after first run');
  process.exit(1);
}
const tableShaFirst = tableSummary.typed_artifacts.table.artifact_sha256;
if (typeof tableShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(tableShaFirst)) {
  console.error('FAIL: expected table artifact sha256 in first summary');
  process.exit(1);
}
const tableArtifactFirst = JSON.parse(fs.readFileSync(tablePath, 'utf8'));
if (!Array.isArray(tableArtifactFirst?.entries)) {
  console.error('FAIL: expected table_v2.entries array in first-run artifact');
  process.exit(1);
}
if (tableArtifactFirst?.schema !== 'table_v2') {
  console.error('FAIL: expected table_v2 schema in first-run artifact');
  process.exit(1);
}
if (tableArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty table_v2.entries for minimal demo');
  process.exit(1);
}
for (const [index, entry] of tableArtifactFirst.entries.entries()) {
  if (typeof entry?.anchor_id !== 'string' || !/^tbl[1-9]\d*$/.test(entry.anchor_id)) {
    console.error(`FAIL: table_v2.entries[${index}] invalid anchor_id`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.row_count) || entry.row_count <= 0) {
    console.error(`FAIL: table_v2.entries[${index}] invalid row_count`);
    process.exit(1);
  }
  if (!Number.isInteger(entry?.column_count) || entry.column_count <= 0) {
    console.error(`FAIL: table_v2.entries[${index}] invalid column_count`);
    process.exit(1);
  }
  if (typeof entry?.align_spec !== 'string' || !/^[lcr]+$/.test(entry.align_spec)) {
    console.error(`FAIL: table_v2.entries[${index}] invalid align_spec`);
    process.exit(1);
  }
  if (entry.align_spec.length !== entry.column_count) {
    console.error(`FAIL: table_v2.entries[${index}] align_spec length mismatch column_count`);
    process.exit(1);
  }
  if (!Array.isArray(entry?.rows) || entry.rows.length !== entry.row_count) {
    console.error(`FAIL: table_v2.entries[${index}] rows mismatch row_count`);
    process.exit(1);
  }
  for (const [rowIndex, row] of entry.rows.entries()) {
    if (!Array.isArray(row) || row.length !== entry.column_count || row.some((cell) => typeof cell !== 'string' || cell.length === 0)) {
      console.error(`FAIL: table_v2.entries[${index}].rows[${rowIndex}] invalid row payload`);
      process.exit(1);
    }
  }
  if (!Array.isArray(entry?.column_widths_pt) || entry.column_widths_pt.length !== entry.column_count) {
    console.error(`FAIL: table_v2.entries[${index}] column_widths_pt mismatch column_count`);
    process.exit(1);
  }
  if (!entry.column_widths_pt.every((value) => Number.isFinite(value) && value >= 0)) {
    console.error(`FAIL: table_v2.entries[${index}] column_widths_pt contains invalid width`);
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_minimal_v0', 'table_v2', tableArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('table_v2'), `${tableShaFirst}\n`);

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
const typedKeys = ['toc', 'labels', 'bib', 'cite', 'pageref', 'hyperref', 'pkgopt', 'packages', 'graphics', 'float', 'input', 'math', 'table'];
for (const key of typedKeys) {
  const value = typedArtifactShaMap[key];
  if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) {
    console.error(`FAIL: expected report.typed_artifact_sha256.${key} sha256 after first run`);
    process.exit(1);
  }
}

const inputProbeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_input_probe_v0', 'summary.json'), 'utf8'),
);
if (inputProbeSummary?.typed_artifacts?.input?.present !== true) {
  console.error('FAIL: expected input_v1 artifact present for typeset_demo_input_probe_v0');
  process.exit(1);
}
if (!(inputProbeSummary?.typed_artifacts?.input?.items > 0)) {
  console.error('FAIL: expected non-empty input_v1 entries for typeset_demo_input_probe_v0');
  process.exit(1);
}

const includeProbeSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_include_probe_v0', 'summary.json'), 'utf8'),
);
if (includeProbeSummary?.typed_artifacts?.input?.present !== true) {
  console.error('FAIL: expected input_v1 artifact present for typeset_demo_include_probe_v0');
  process.exit(1);
}
if (!(includeProbeSummary?.typed_artifacts?.input?.items > 0)) {
  console.error('FAIL: expected non-empty input_v1 entries for typeset_demo_include_probe_v0');
  process.exit(1);
}

fs.writeFileSync(
  path.join(baselineRoot, 'typed_artifact_sha256_first.json'),
  `${JSON.stringify(typedArtifactShaMap, null, 2)}\n`,
);
fs.writeFileSync(firstRunShaPath('resolved_resources_count'), `${Number(report.resolved_resources_count ?? 0)}\n`);
