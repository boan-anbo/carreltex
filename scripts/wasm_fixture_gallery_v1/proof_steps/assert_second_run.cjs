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
const requiredTypedKeys = ['toc', 'labels', 'refs', 'bib', 'cite', 'hyperref', 'pkgopt', 'graphics', 'math', 'table'];
const labelsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'labels_v1_first.sha256'), 'utf8').trim();
const refsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'refs_v1_first.sha256'), 'utf8').trim();
const tocShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'toc_v1_first.sha256'), 'utf8').trim();
const bibShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'bib_v1_first.sha256'), 'utf8').trim();
const citeShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'cite_v1_first.sha256'), 'utf8').trim();
const hyperrefShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'hyperref_v0_first.sha256'), 'utf8').trim();
const pkgoptShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'pkgopt_v0_first.sha256'), 'utf8').trim();
const graphicsShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'graphics_v1_first.sha256'), 'utf8').trim();
const mathShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'math_v1_first.sha256'), 'utf8').trim();
const tableShaFirst = fs.readFileSync(path.join(`${outDir}_baseline`, 'table_v1_first.sha256'), 'utf8').trim();

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
  console.error('FAIL: labels_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const labelsArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_labels_probe_v0', 'labels_v1.json'), 'utf8'),
);
if (!Array.isArray(labelsArtifactSecond?.entries) || labelsArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty labels_v1.entries after rerun');
  process.exit(1);
}
const refsArtifact = labelsSummary?.typed_artifacts?.refs;
if (!refsArtifact || refsArtifact.present !== true) {
  console.error('FAIL: expected refs typed artifact present after second run');
  process.exit(1);
}
const refsShaSecond = refsArtifact.artifact_sha256;
if (refsShaSecond !== refsShaFirst) {
  console.error('FAIL: refs_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const refsArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_labels_probe_v0', 'refs_v1.json'), 'utf8'),
);
if (!Array.isArray(refsArtifactSecond?.entries) || refsArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty refs_v1.entries after rerun');
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
  console.error('FAIL: toc_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const tocArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_toc_probe_v0', 'toc_v1.json'), 'utf8'),
);
if (!Array.isArray(tocArtifactSecond?.entries) || tocArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty toc_v1.entries after rerun');
  process.exit(1);
}
if (tocArtifactSecond?.schema !== 'toc_v1') {
  console.error('FAIL: expected toc_v1 schema after rerun');
  process.exit(1);
}
for (const [index, entry] of tocArtifactSecond.entries.entries()) {
  if (!Number.isInteger(entry?.level) || entry.level < 1 || entry.level > 2) {
    console.error(`FAIL: rerun toc_v1.entries[${index}] must have level in [1,2]`);
    process.exit(1);
  }
  if (typeof entry?.anchor_id !== 'string' || !/^h[1-9]\d*$/.test(entry.anchor_id)) {
    console.error(`FAIL: rerun toc_v1.entries[${index}] must have anchor_id like hN`);
    process.exit(1);
  }
  if (entry?.page !== null) {
    console.error(`FAIL: rerun toc_v1.entries[${index}].page must be null in v1`);
    process.exit(1);
  }
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
  console.error('FAIL: bib_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const bibArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'bib_v1.json'), 'utf8'),
);
if (!Array.isArray(bibArtifactSecond?.entries) || bibArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty bib_v1.entries after rerun');
  process.exit(1);
}

const citeArtifact = bibSummary?.typed_artifacts?.cite;
if (!citeArtifact || citeArtifact.present !== true) {
  console.error('FAIL: expected cite typed artifact present after second run');
  process.exit(1);
}
const citeShaSecond = citeArtifact.artifact_sha256;
if (citeShaSecond !== citeShaFirst) {
  console.error('FAIL: cite_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const citeArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_bib_probe_v0', 'cite_v1.json'), 'utf8'),
);
if (!Array.isArray(citeArtifactSecond?.entries) || citeArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty cite_v1.entries after rerun');
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
  console.error('FAIL: graphics_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const graphicsArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_graphics_probe_v0', 'graphics_v1.json'), 'utf8'),
);
if (!Array.isArray(graphicsArtifactSecond?.entries) || graphicsArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty graphics_v1.entries after rerun');
  process.exit(1);
}

const mathSummary = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_minimal_v0', 'summary.json'), 'utf8'),
);
const mathArtifact = mathSummary?.typed_artifacts?.math;
if (!mathArtifact || mathArtifact.present !== true) {
  console.error('FAIL: expected math typed artifact present after second run');
  process.exit(1);
}
const mathShaSecond = mathArtifact.artifact_sha256;
if (mathShaSecond !== mathShaFirst) {
  console.error('FAIL: math_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const mathArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_minimal_v0', 'math_v1.json'), 'utf8'),
);
if (!Array.isArray(mathArtifactSecond?.entries) || mathArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty math_v1.entries after rerun');
  process.exit(1);
}

const tableArtifact = mathSummary?.typed_artifacts?.table;
if (!tableArtifact || tableArtifact.present !== true) {
  console.error('FAIL: expected table typed artifact present after second run');
  process.exit(1);
}
const tableShaSecond = tableArtifact.artifact_sha256;
if (tableShaSecond !== tableShaFirst) {
  console.error('FAIL: table_v1 artifact sha256 must be stable across reruns');
  process.exit(1);
}
const tableArtifactSecond = JSON.parse(
  fs.readFileSync(path.join(outDir, 'typeset_demo_minimal_v0', 'table_v1.json'), 'utf8'),
);
if (!Array.isArray(tableArtifactSecond?.entries) || tableArtifactSecond.entries.length <= 0) {
  console.error('FAIL: expected non-empty table_v1.entries after rerun');
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
console.log(`PASS: labels_v1 sha stable ${labelsShaSecond}`);
console.log(`PASS: refs_v1 sha stable ${refsShaSecond}`);
console.log(`PASS: toc_v1 sha stable ${tocShaSecond}`);
console.log(`PASS: bib_v1 sha stable ${bibShaSecond}`);
console.log(`PASS: cite_v1 sha stable ${citeShaSecond}`);
console.log(`PASS: hyperref_v0 sha stable ${hyperrefShaSecond}`);
console.log(`PASS: pkgopt_v0 sha stable ${pkgoptShaSecond}`);
console.log(`PASS: graphics_v1 sha stable ${graphicsShaSecond}`);
console.log(`PASS: math_v1 sha stable ${mathShaSecond}`);
console.log(`PASS: table_v1 sha stable ${tableShaSecond}`);
console.log('PASS: report typed_artifact_sha256 map present and stable');
console.log('PASS: report top-level case_artifact_sha256 present');
