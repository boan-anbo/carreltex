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

const tocSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'summary.json'), 'utf8'),
);
if (tocSummary?.typed_artifacts?.toc?.present !== true) {
  console.error('FAIL: expected toc typed artifact present after first run');
  process.exit(1);
}
const tocPath = path.join(outDir, 'typeset_demo_toc_probe_v0', 'toc_v1.json');
if (!fs.existsSync(tocPath)) {
  console.error('FAIL: expected toc_v1.json artifact after first run');
  process.exit(1);
}
const tocShaFirst = tocSummary.typed_artifacts.toc.artifact_sha256;
if (typeof tocShaFirst !== 'string' || !/^[0-9a-f]{64}$/.test(tocShaFirst)) {
  console.error('FAIL: expected toc artifact sha256 in first summary');
  process.exit(1);
}
const tocArtifactFirst = JSON.parse(fs.readFileSync(tocPath, 'utf8'));
if (!Array.isArray(tocArtifactFirst?.entries)) {
  console.error('FAIL: expected toc_v1.entries array in first-run artifact');
  process.exit(1);
}
if (tocArtifactFirst?.schema !== 'toc_v1') {
  console.error('FAIL: expected toc_v1 schema in first-run artifact');
  process.exit(1);
}
if (tocArtifactFirst.entries.length <= 0) {
  console.error('FAIL: expected non-empty toc_v1.entries for toc probe');
  process.exit(1);
}
for (const [index, entry] of tocArtifactFirst.entries.entries()) {
  if (!Number.isInteger(entry?.level) || entry.level < 1 || entry.level > 2) {
    console.error(`FAIL: toc_v1.entries[${index}] must have level in [1,2]`);
    process.exit(1);
  }
  if (typeof entry?.anchor_id !== 'string' || !/^h[1-9]\d*$/.test(entry.anchor_id)) {
    console.error(`FAIL: toc_v1.entries[${index}] must have anchor_id like hN`);
    process.exit(1);
  }
  if (entry?.page !== null) {
    console.error(`FAIL: toc_v1.entries[${index}].page must be null in v1`);
    process.exit(1);
  }
}
assertEntrySourceSpans('typeset_demo_toc_probe_v0', 'toc_v1', tocArtifactFirst.entries);
fs.writeFileSync(firstRunShaPath('toc_v1'), `${tocShaFirst}\n`);

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
