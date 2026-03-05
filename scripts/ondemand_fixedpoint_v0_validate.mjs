import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const REQUIRED_PROBE_CASE_IDS_V0 = new Set([
  'typeset_demo_cjk_probe_v0',
  'typeset_demo_math_probe_v0',
  'typeset_demo_hyperref_links_probe_v0',
]);

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertV0(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function isSha256V0(value) {
  return typeof value === 'string' && /^[0-9a-f]{64}$/.test(value);
}

function isFiniteNumberV0(value) {
  return typeof value === 'number' && Number.isFinite(value);
}

function validateStatusCountsV0(statusCounts, context) {
  assertV0(statusCounts && typeof statusCounts === 'object', `${context} status_counts missing`);
  const required = ['OK', 'NI', 'INVALID', 'FAIL', 'OTHER'];
  for (const key of required) {
    assertV0(Number.isInteger(statusCounts[key]) && statusCounts[key] >= 0, `${context} status_counts.${key} invalid`);
  }
}

function hasForbiddenTimestampKeysV0(value) {
  if (Array.isArray(value)) {
    return value.some((entry) => hasForbiddenTimestampKeysV0(entry));
  }
  if (!value || typeof value !== 'object') {
    return false;
  }
  for (const [key, nested] of Object.entries(value)) {
    if (key === 'source_date_epoch' || key === 'store_path') {
      continue;
    }
    if (key.toLowerCase().includes('timestamp') || key.toLowerCase() === 'time') {
      return true;
    }
    if (hasForbiddenTimestampKeysV0(nested)) {
      return true;
    }
  }
  return false;
}

function validateProbeChecksV0(probeChecks) {
  assertV0(Array.isArray(probeChecks), 'phase2_probe_checks must be array');
  const seen = new Set();
  for (const entry of probeChecks) {
    assertV0(entry && typeof entry === 'object', 'phase2 probe entry must be object');
    const caseId = `${entry.case_id ?? ''}`;
    assertV0(REQUIRED_PROBE_CASE_IDS_V0.has(caseId), `unexpected probe case_id ${caseId}`);
    seen.add(caseId);
    assertV0(entry.expected_vs_actual === 'MATCH', `probe expected_vs_actual mismatch for ${caseId}`);
    assertV0(entry.actual_status === 'NI' || entry.actual_status === 'INVALID', `probe status invalid for ${caseId}`);
    assertV0(isFiniteNumberV0(entry.resource_hints_items) && entry.resource_hints_items > 0, `probe hints must be non-empty for ${caseId}`);
    assertV0(isSha256V0(entry.resource_hints_sha256), `probe hints sha missing for ${caseId}`);
    assertV0(Number.isInteger(entry.resolved_resources_count) && entry.resolved_resources_count >= 0, `probe resolved count invalid for ${caseId}`);
  }
  for (const caseId of REQUIRED_PROBE_CASE_IDS_V0) {
    assertV0(seen.has(caseId), `missing probe case ${caseId}`);
  }
}

function validateIterationsV0(iterations) {
  assertV0(Array.isArray(iterations), 'iterations must be array');
  assertV0(iterations.length >= 2 && iterations.length <= 3, 'iterations length must be in [2,3]');
  let sawImprovement = false;
  for (let i = 0; i < iterations.length; i += 1) {
    const iter = iterations[i];
    assertV0(iter && typeof iter === 'object', `iteration ${i + 1} must be object`);
    assertV0(iter.iteration === i + 1, `iteration index mismatch at ${i + 1}`);
    for (const key of ['resolved_resources_count', 'found', 'missing', 'request_count']) {
      assertV0(Number.isInteger(iter[key]) && iter[key] >= 0, `iteration ${i + 1} ${key} invalid`);
    }
    assertV0(isSha256V0(iter.request_list_sha256), `iteration ${i + 1} request_list_sha256 missing`);
    assertV0(isSha256V0(iter.request_payload_sha256), `iteration ${i + 1} request_payload_sha256 missing`);
    assertV0(isSha256V0(iter.store_index_sha256), `iteration ${i + 1} store_index_sha256 missing`);
    assertV0(typeof iter.store_resolver_id === 'string' && iter.store_resolver_id.length > 0, `iteration ${i + 1} store_resolver_id missing`);
    validateStatusCountsV0(iter.status_counts, `iteration ${i + 1}`);
    validateProbeChecksV0(iter.probe_cases);

    if (i === 0) {
      continue;
    }
    const prev = iterations[i - 1];
    const resolvedDelta = iter.resolved_resources_count - prev.resolved_resources_count;
    const missingDelta = iter.missing - prev.missing;
    assertV0(!(resolvedDelta < 0 || missingDelta > 0), `iteration ${i + 1} regression`);
    if (resolvedDelta > 0 || missingDelta < 0) {
      sawImprovement = true;
    }
    if (resolvedDelta === 0 && missingDelta === 0) {
      assertV0(sawImprovement, `iteration ${i + 1} fixedpoint reached without prior improvement`);
    }
  }
}

function canonicalSummaryPayloadV0(summary) {
  const { determinism, version, ...rest } = summary;
  const iterations = Array.isArray(summary.iterations)
    ? summary.iterations.map((iter) => ({
      ...iter,
      store_resolver_id: '<store_resolver_id>',
      request_list_sha256: '<request_list_sha256>',
    }))
    : [];
  return {
    ...rest,
    store_path: '<store_path>',
    iterations,
  };
}

async function validateSummaryV0(summaryPath) {
  const summaryBytes = await readFile(summaryPath);
  const summary = JSON.parse(summaryBytes.toString('utf8'));

  assertV0(summary.version === 1, 'summary version must be 1');
  assertV0(summary.schema === 'ondemand_fixedpoint_summary_v0', 'summary schema mismatch');
  assertV0(Number.isInteger(summary.source_date_epoch) && summary.source_date_epoch > 0, 'summary source_date_epoch invalid');
  assertV0(typeof summary.store_path === 'string' && summary.store_path.length > 0, 'summary store_path missing');
  assertV0(summary.final_status === 'PASS', 'summary final_status must be PASS');
  assertV0(!hasForbiddenTimestampKeysV0(summary), 'summary contains forbidden timestamp fields');

  validateIterationsV0(summary.iterations);
  const finalIter = summary.iterations[summary.iterations.length - 1];
  assertV0(summary.fixedpoint_iteration === summary.iterations.length, 'fixedpoint_iteration mismatch');
  assertV0(summary.fixedpoint_resolved_resources_count === finalIter.resolved_resources_count, 'fixedpoint resolved mismatch');
  assertV0(summary.fixedpoint_missing_count === 0, 'fixedpoint missing must be 0');
  assertV0(summary.phase2_resolved_resources_count === finalIter.resolved_resources_count, 'phase2 resolved mismatch');
  validateStatusCountsV0(summary.phase2_status_counts, 'phase2');
  validateProbeChecksV0(summary.phase2_probe_checks);

  assertV0(
    summary.fixedpoint_gallery_relpath === `gallery_iter_${summary.fixedpoint_iteration}`,
    'fixedpoint_gallery_relpath mismatch',
  );
  assertV0(
    summary.phase2_gallery_relpath === 'gallery_phase2_after_fixedpoint',
    'phase2_gallery_relpath mismatch',
  );

  const determinism = summary.determinism;
  assertV0(determinism && typeof determinism === 'object', 'determinism object missing');
  assertV0(determinism.reruns === 2, 'determinism reruns must be 2');
  assertV0(isSha256V0(determinism.canonical_summary_sha256_a), 'determinism sha a missing');
  assertV0(isSha256V0(determinism.canonical_summary_sha256_b), 'determinism sha b missing');
  assertV0(determinism.canonical_summary_stable === true, 'determinism flag must be true');
  assertV0(
    determinism.canonical_summary_sha256_a === determinism.canonical_summary_sha256_b,
    'determinism sha mismatch',
  );

  const canonicalPayload = canonicalSummaryPayloadV0(summary);
  const canonicalSha = sha256HexV0(Buffer.from(JSON.stringify(canonicalPayload), 'utf8'));
  assertV0(canonicalSha === determinism.canonical_summary_sha256_a, 'determinism sha does not match summary payload');

  return {
    summary,
    summarySha256: sha256HexV0(summaryBytes),
    canonicalSha256: canonicalSha,
  };
}

async function runCliV0() {
  const summaryPath = path.resolve(
    process.argv[2] ?? path.join(rootDir, 'target', 'ondemand_fixedpoint_v0', 'ondemand_fixedpoint_summary.json'),
  );
  const result = await validateSummaryV0(summaryPath);
  console.log(`PASS: validated summary ${summaryPath}`);
  console.log(`PASS: summary_sha256 ${result.summarySha256}`);
  console.log(`PASS: canonical_summary_sha256 ${result.canonicalSha256}`);
  console.log('PASS: on-demand fixedpoint summary validation');
}

if (import.meta.url === new URL(process.argv[1], 'file://').href) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: on-demand fixedpoint summary validation: ${message}`);
    process.exit(1);
  });
}
