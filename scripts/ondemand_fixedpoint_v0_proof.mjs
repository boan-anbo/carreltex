import { mkdtemp, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { createServer } from 'node:http';
import { execFileSync } from 'node:child_process';

import { buildRequestListFromHintsV0 } from './texlive_smoke/request_list_from_hints_v0.mjs';
import { generateTexliveStoreV0 } from './texlive_store_gen_v0.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const DEFAULT_SOURCE_DATE_EPOCH_V0 = 1_700_000_000;
const MAX_ITERS_V0 = 3;
const FIXEDPOINT_SUMMARY_SCHEMA_V0 = 'ondemand_fixedpoint_summary_v0';
const FIXEDPOINT_SUMMARY_VERSION_V0 = 2;
const PHASE2_PROBE_CASE_IDS_V0 = [
  'typeset_demo_cjk_probe_v0',
  'typeset_demo_math_probe_v0',
  'typeset_demo_hyperref_links_probe_v0',
  'typeset_demo_fixedpoint_graphics_probe_v0',
  'typeset_demo_fixedpoint_bibliography_probe_v0',
];

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function stableRequestPayloadShaV0(requests) {
  return sha256HexV0(Buffer.from(JSON.stringify(requests), 'utf8'));
}

function assertV0(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

async function readJsonFileV0(jsonPath) {
  return JSON.parse((await readFile(jsonPath)).toString('utf8'));
}

async function countResolvedResourcesFromSummariesV0(galleryOutDir, statuses) {
  let count = 0;
  for (const statusEntry of statuses) {
    const caseId = `${statusEntry?.case_id ?? ''}`;
    if (caseId.length === 0) {
      continue;
    }
    const summaryPath = path.join(galleryOutDir, caseId, 'summary.json');
    const summary = await readJsonFileV0(summaryPath);
    const resolvedResources = Array.isArray(summary.resolved_resources) ? summary.resolved_resources : [];
    count += resolvedResources.length;
  }
  return count;
}

function endpointPathForRequestV0(request) {
  if (request.kind === 'fontconfig') {
    return `/fontconfig/${request.variant}/${request.name}`;
  }
  return `/xetex/${request.format}/${request.name}`;
}

function stableIdForRequestV0(request) {
  const variantPart = request.variant === '' ? 'default' : request.variant;
  return `fixedpoint_${request.kind}_${request.format}_${variantPart}_${request.name}`;
}

function buildStubResourceMapV0(requests) {
  const map = new Map();
  for (const request of requests) {
    const endpointPath = endpointPathForRequestV0(request);
    const payload = Buffer.from(
      `fixedpoint-v0:${request.kind}:${request.format}:${request.variant}:${request.name}\n`,
      'utf8',
    );
    map.set(endpointPath, {
      bytes: payload,
      stableId: stableIdForRequestV0(request),
      sha256: sha256HexV0(payload),
      kind: request.kind,
    });
  }
  return map;
}

async function startStubServerV0(resourceMap) {
  const server = createServer((req, res) => {
    const pathname = new URL(req.url ?? '/', 'http://127.0.0.1').pathname;
    const entry = resourceMap.get(pathname);
    if (!entry) {
      res.statusCode = 404;
      res.end('not found');
      return;
    }
    res.statusCode = 200;
    res.setHeader('content-type', 'application/octet-stream');
    if (entry.kind === 'fontconfig') {
      res.setHeader('fontid', entry.stableId);
    } else {
      res.setHeader('fileid', entry.stableId);
    }
    res.end(entry.bytes);
  });

  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });

  const address = server.address();
  assertV0(address && typeof address === 'object', 'stub server failed to bind');
  const endpoint = `http://127.0.0.1:${address.port}`;
  return {
    endpoint,
    close: () =>
      new Promise((resolve, reject) => {
        server.close((error) => {
          if (error) {
            reject(error);
            return;
          }
          resolve();
        });
      }),
  };
}

function runGalleryV0(galleryOutDir, storeDir, sourceDateEpoch) {
  execFileSync('node', [path.join(rootDir, 'scripts', 'wasm_fixture_gallery_v0.mjs'), galleryOutDir], {
    cwd: rootDir,
    stdio: 'pipe',
    env: {
      ...process.env,
      SOURCE_DATE_EPOCH: `${sourceDateEpoch}`,
      TZ: 'UTC',
      TEXLIVE_RESOLVER_BACKEND_V0: 'offline_store_v0',
      TEXLIVE_STORE_DIR_V0: storeDir,
    },
  });
}

function countStatusesV0(statuses) {
  const counts = {
    OK: 0,
    NI: 0,
    INVALID: 0,
    FAIL: 0,
    OTHER: 0,
  };
  for (const statusEntry of statuses) {
    const status = statusEntry?.status;
    if (status === 'OK' || status === 'NI' || status === 'INVALID' || status === 'FAIL') {
      counts[status] += 1;
    } else {
      counts.OTHER += 1;
    }
  }
  return counts;
}

function summarizeHintTypesV0(resourceHintsPayload) {
  const entries = Array.isArray(resourceHintsPayload?.entries) ? resourceHintsPayload.entries : [];
  const counts = {};
  for (const entry of entries) {
    const hintType = `${entry?.hint_type ?? ''}`;
    if (!hintType) {
      continue;
    }
    counts[hintType] = (counts[hintType] ?? 0) + 1;
  }
  const sorted = Object.entries(counts).sort((a, b) => a[0].localeCompare(b[0]));
  return Object.fromEntries(sorted);
}

function canonicalizeProbeCaseChecksV0(probeChecks) {
  return [...probeChecks].sort((left, right) => `${left.case_id}`.localeCompare(`${right.case_id}`));
}

function buildIterationRowsV0(iterations) {
  const rows = iterations.map((iter) => ({
    iteration: iter.iteration,
    resolved_resources_count: iter.resolved_resources_count,
    found: iter.found,
    missing: iter.missing,
    request_count: iter.request_count,
    status_counts: iter.status_counts,
  }));
  rows.sort((left, right) => left.iteration - right.iteration);
  return rows;
}

function buildProbeCaseIndexV0(probeChecks) {
  const sorted = canonicalizeProbeCaseChecksV0(probeChecks);
  const index = {};
  for (const entry of sorted) {
    index[entry.case_id] = {
      expected_status: entry.expected_status,
      actual_status: entry.actual_status,
      expected_vs_actual: entry.expected_vs_actual,
      resource_hints_items: entry.resource_hints_items,
      resource_hints_sha256: entry.resource_hints_sha256,
      resolved_resources_count: entry.resolved_resources_count,
      hint_type_counts: entry.hint_type_counts,
    };
  }
  return index;
}

async function buildProbeCaseTableV0(galleryOutDir, report) {
  const statuses = Array.isArray(report.statuses) ? report.statuses : [];
  const probeChecks = [];
  for (const caseId of PHASE2_PROBE_CASE_IDS_V0) {
    const statusEntry = statuses.find((entry) => entry?.case_id === caseId);
    assertV0(statusEntry, `phase2 probe case missing from report: ${caseId}`);
    assertV0(
      statusEntry.expected_vs_actual === 'MATCH',
      `phase2 probe case expected_vs_actual mismatch for ${caseId}`,
    );
    assertV0(
      statusEntry.status === 'NI' || statusEntry.status === 'INVALID',
      `phase2 probe case status must be NI/INVALID for ${caseId}`,
    );

    const summaryPath = path.join(galleryOutDir, caseId, 'summary.json');
    const summary = JSON.parse((await readFile(summaryPath)).toString('utf8'));
    const resourceHints = summary.resource_hints_v0 ?? {};
    assertV0(resourceHints.present === true, `phase2 probe resource_hints missing for ${caseId}`);
    const hintItems = Number(resourceHints.items ?? 0);
    assertV0(hintItems > 0, `phase2 probe resource_hints must be non-empty for ${caseId}`);
    const hintSha = `${resourceHints.artifact_sha256 ?? ''}`;
    assertV0(/^[0-9a-f]{64}$/.test(hintSha), `phase2 probe resource_hints sha missing for ${caseId}`);
    const hintRelpath = `${resourceHints.artifact_relpath ?? ''}`;
    assertV0(hintRelpath.length > 0, `phase2 probe resource_hints relpath missing for ${caseId}`);
    const resolvedResources = Array.isArray(summary.resolved_resources) ? summary.resolved_resources : [];
    const hintPayloadPath = path.join(galleryOutDir, caseId, hintRelpath);
    const hintPayload = await readJsonFileV0(hintPayloadPath);
    const hintTypeCounts = summarizeHintTypesV0(hintPayload);
    assertV0(
      Object.keys(hintTypeCounts).length > 0,
      `phase2 probe resource_hints type counts must be non-empty for ${caseId}`,
    );

    probeChecks.push({
      case_id: caseId,
      expected_status: statusEntry.expected_status,
      actual_status: statusEntry.status,
      expected_vs_actual: statusEntry.expected_vs_actual,
      resource_hints_items: hintItems,
      resource_hints_sha256: hintSha,
      resolved_resources_count: resolvedResources.length,
      hint_type_counts: hintTypeCounts,
    });
  }
  return canonicalizeProbeCaseChecksV0(probeChecks);
}

function buildMonotonicityTransitionsV0(iterations) {
  const transitions = [];
  for (let i = 1; i < iterations.length; i += 1) {
    const previous = iterations[i - 1];
    const current = iterations[i];
    const resolvedDelta = current.resolved_resources_count - previous.resolved_resources_count;
    const missingDelta = current.missing - previous.missing;
    transitions.push({
      from_iteration: previous.iteration,
      to_iteration: current.iteration,
      resolved_delta: resolvedDelta,
      missing_delta: missingDelta,
      found_delta: current.found - previous.found,
      request_delta: current.request_count - previous.request_count,
      resolved_non_decreasing: resolvedDelta >= 0,
      missing_non_increasing: missingDelta <= 0,
      improvement_or_fixedpoint: resolvedDelta > 0 || missingDelta < 0 || (resolvedDelta === 0 && missingDelta === 0),
      improvement: resolvedDelta > 0 || missingDelta < 0,
      fixedpoint_step: resolvedDelta === 0 && missingDelta === 0,
    });
  }
  return transitions;
}

function assertMonotonicityTransitionsV0(iterations, transitions) {
  assertV0(Array.isArray(transitions), 'monotonicity transitions must be array');
  assertV0(transitions.length === Math.max(0, iterations.length - 1), 'monotonicity transition length mismatch');
  let sawImprovement = false;
  let sawFixedpoint = false;
  for (let i = 0; i < transitions.length; i += 1) {
    const transition = transitions[i];
    const previous = iterations[i];
    const current = iterations[i + 1];
    assertV0(transition.from_iteration === previous.iteration, `monotonicity from_iteration mismatch at ${i + 1}`);
    assertV0(transition.to_iteration === current.iteration, `monotonicity to_iteration mismatch at ${i + 1}`);
    assertV0(
      transition.resolved_delta === current.resolved_resources_count - previous.resolved_resources_count,
      `monotonicity resolved_delta mismatch at ${i + 1}`,
    );
    assertV0(
      transition.missing_delta === current.missing - previous.missing,
      `monotonicity missing_delta mismatch at ${i + 1}`,
    );
    assertV0(
      transition.found_delta === current.found - previous.found,
      `monotonicity found_delta mismatch at ${i + 1}`,
    );
    assertV0(
      transition.request_delta === current.request_count - previous.request_count,
      `monotonicity request_delta mismatch at ${i + 1}`,
    );
    assertV0(transition.resolved_non_decreasing, `monotonicity resolved regression at ${i + 1}`);
    assertV0(transition.missing_non_increasing, `monotonicity missing regression at ${i + 1}`);
    assertV0(transition.improvement_or_fixedpoint, `monotonicity neither improved nor fixedpoint at ${i + 1}`);
    if (transition.improvement) {
      sawImprovement = true;
    }
    if (transition.fixedpoint_step) {
      sawFixedpoint = true;
      assertV0(i === transitions.length - 1, 'fixedpoint step must be final transition');
    } else {
      assertV0(transition.improvement, `transition ${i + 1} before fixedpoint must improve`);
    }
  }
  assertV0(sawImprovement, 'fixedpoint run must contain at least one improvement step');
  assertV0(sawFixedpoint, 'fixedpoint run must contain one fixedpoint step');
  return {
    transitions,
    saw_improvement: sawImprovement,
    reached_fixedpoint: sawFixedpoint,
  };
}

async function runFixedpointPassV0(outDir, sourceDateEpoch) {
  const runRoot = await mkdtemp(path.join(outDir, 'run_'));
  const storeDir = path.join(runRoot, 'store');
  await mkdir(storeDir, { recursive: true });

  const iterations = [];
  let fixedpointReached = false;
  let sawImprovement = false;
  let fixedpointIteration = 0;
  let fixedpointGalleryOutDir = '';
  let fixedpointReport = null;

  for (let iter = 1; iter <= MAX_ITERS_V0; iter += 1) {
    const galleryOutDir = path.join(runRoot, `gallery_iter_${iter}`);
    runGalleryV0(galleryOutDir, storeDir, sourceDateEpoch);

    const reportPath = path.join(galleryOutDir, 'report.json');
    const report = await readJsonFileV0(reportPath);
    const statuses = Array.isArray(report.statuses) ? report.statuses : [];
    assertV0(statuses.length > 0, `gallery report statuses missing at iteration ${iter}`);
    assertV0(Number.isInteger(report.case_count), `gallery report case_count missing at iteration ${iter}`);
    assertV0(report.case_count === statuses.length, `gallery report case_count mismatch at iteration ${iter}`);

    const resolvedResourcesCount = Number(report.resolved_resources_count ?? 0);
    const recomputedResolvedCount = await countResolvedResourcesFromSummariesV0(galleryOutDir, statuses);
    assertV0(
      resolvedResourcesCount === recomputedResolvedCount,
      `resolved_resources_count mismatch at iteration ${iter}`,
    );

    const requestListPath = path.join(runRoot, `request_list_iter_${iter}.json`);
    await buildRequestListFromHintsV0({
      rootDir,
      reportPath,
      outputPath: requestListPath,
    });
    const requestList = await readJsonFileV0(requestListPath);
    const requests = Array.isArray(requestList.requests) ? requestList.requests : [];
    const requestCount = Number(requestList.request_count ?? requests.length);
    assertV0(requestCount === requests.length, `request_count mismatch at iteration ${iter}`);
    const requestListSha256 = sha256HexV0(Buffer.from(JSON.stringify(requestList), 'utf8'));
    const requestPayloadSha256 = stableRequestPayloadShaV0(requests);

    const resourceMap = buildStubResourceMapV0(requests);
    const stub = await startStubServerV0(resourceMap);
    let storeResult;
    try {
      storeResult = await generateTexliveStoreV0({
        rootDir,
        requestListPath,
        storeDir,
        backend: 'endpoint_v0',
        endpoint: stub.endpoint,
        sourceDateEpoch,
      });
    } finally {
      await stub.close();
    }
    const storeSummary = await readJsonFileV0(path.join(storeDir, 'summary.json'));
    const storeFound = Number(storeSummary.found_count ?? storeResult.foundCount);
    const storeMissing = Number(storeSummary.missing_count ?? storeResult.missingCount);
    assertV0(storeFound === storeResult.foundCount, `store found_count mismatch at iteration ${iter}`);
    assertV0(storeMissing === storeResult.missingCount, `store missing_count mismatch at iteration ${iter}`);
    assertV0(Number(storeSummary.request_count ?? requestCount) === requestCount, `store request_count mismatch at iteration ${iter}`);
    assertV0(
      typeof storeSummary.index_sha256 === 'string' && /^[0-9a-f]{64}$/.test(storeSummary.index_sha256),
      `store index_sha256 missing at iteration ${iter}`,
    );
    assertV0(
      typeof storeSummary.resolver_id === 'string' && storeSummary.resolver_id.length > 0,
      `store resolver_id missing at iteration ${iter}`,
    );

    const statusCounts = countStatusesV0(statuses);
    assertV0(statusCounts.OTHER === 0, `unsupported case status found at iteration ${iter}`);
    const probeCases = await buildProbeCaseTableV0(galleryOutDir, report);

    const iteration = {
      iteration: iter,
      resolved_resources_count: resolvedResourcesCount,
      found: storeFound,
      missing: storeMissing,
      request_count: requestCount,
      request_list_sha256: requestListSha256,
      request_payload_sha256: requestPayloadSha256,
      store_index_sha256: storeSummary.index_sha256,
      store_resolver_id: storeSummary.resolver_id,
      status_counts: statusCounts,
      probe_cases: probeCases,
    };
    iterations.push(iteration);

    if (iterations.length < 2) {
      continue;
    }
    const previous = iterations[iterations.length - 2];
    const current = iterations[iterations.length - 1];
    const resolvedDelta = current.resolved_resources_count - previous.resolved_resources_count;
    const missingDelta = current.missing - previous.missing;
    const regression = resolvedDelta < 0 || missingDelta > 0;
    assertV0(!regression, `fixedpoint regression at iteration ${iter}`);

    const improved = resolvedDelta > 0 || missingDelta < 0;
    const noChange = resolvedDelta === 0 && missingDelta === 0;
    if (improved) {
      sawImprovement = true;
      continue;
    }
    if (noChange) {
      assertV0(sawImprovement, 'fixedpoint reached without prior improvement');
      fixedpointReached = true;
      fixedpointIteration = iter;
      fixedpointGalleryOutDir = galleryOutDir;
      fixedpointReport = report;
      break;
    }
    throw new Error(`fixedpoint transition must improve or stabilize at iteration ${iter}`);
  }

  assertV0(fixedpointReached, `fixedpoint not reached within ${MAX_ITERS_V0} iterations`);
  const finalIter = iterations[iterations.length - 1];
  assertV0(finalIter.missing === 0, `fixedpoint must converge with missing=0, got ${finalIter.missing}`);
  assertV0(fixedpointReport !== null, 'fixedpoint report missing');

  const phase2GalleryOutDir = path.join(runRoot, 'gallery_phase2_after_fixedpoint');
  runGalleryV0(phase2GalleryOutDir, storeDir, sourceDateEpoch);
  const phase2Report = await readJsonFileV0(path.join(phase2GalleryOutDir, 'report.json'));
  const phase2ProbeChecks = await buildProbeCaseTableV0(phase2GalleryOutDir, phase2Report);
  const phase2StatusCounts = countStatusesV0(Array.isArray(phase2Report.statuses) ? phase2Report.statuses : []);
  const phase2ResolvedResources = Number(phase2Report.resolved_resources_count ?? 0);
  assertV0(
    phase2ResolvedResources === finalIter.resolved_resources_count,
    `phase2 rerun must preserve fixedpoint resolved count (${finalIter.resolved_resources_count} != ${phase2ResolvedResources})`,
  );

  const monotonicityTransitions = buildMonotonicityTransitionsV0(iterations);
  const monotonicity = assertMonotonicityTransitionsV0(iterations, monotonicityTransitions);
  const iterationRows = buildIterationRowsV0(iterations);
  const phase2ProbeRows = canonicalizeProbeCaseChecksV0(phase2ProbeChecks);
  const phase2ProbeCaseIndex = buildProbeCaseIndexV0(phase2ProbeRows);

  return {
    summary_schema: FIXEDPOINT_SUMMARY_SCHEMA_V0,
    summary_version: FIXEDPOINT_SUMMARY_VERSION_V0,
    schema: FIXEDPOINT_SUMMARY_SCHEMA_V0,
    source_date_epoch: sourceDateEpoch,
    store_path: storeDir,
    iterations,
    iteration_rows: iterationRows,
    monotonicity,
    fixedpoint_iteration: fixedpointIteration,
    fixedpoint_resolved_resources_count: finalIter.resolved_resources_count,
    fixedpoint_missing_count: finalIter.missing,
    phase2_probe_checks: phase2ProbeChecks,
    phase2_probe_rows: phase2ProbeRows,
    phase2_probe_case_index: phase2ProbeCaseIndex,
    phase2_status_counts: phase2StatusCounts,
    phase2_resolved_resources_count: phase2ResolvedResources,
    fixedpoint_gallery_relpath: path.relative(runRoot, fixedpointGalleryOutDir),
    phase2_gallery_relpath: path.relative(runRoot, phase2GalleryOutDir),
    final_status: 'PASS',
  };
}

function canonicalizeObjectV0(value) {
  if (Array.isArray(value)) {
    return value.map((item) => canonicalizeObjectV0(item));
  }
  if (!value || typeof value !== 'object') {
    return value;
  }
  const sortedKeys = Object.keys(value).sort((a, b) => a.localeCompare(b));
  const out = {};
  for (const key of sortedKeys) {
    out[key] = canonicalizeObjectV0(value[key]);
  }
  return out;
}

function canonicalSummaryPayloadV0(summary) {
  const iterations = Array.isArray(summary.iterations)
    ? summary.iterations.map((iter) => ({
      ...iter,
      store_resolver_id: '<store_resolver_id>',
      request_list_sha256: '<request_list_sha256>',
    }))
    : [];
  const iterationRows = Array.isArray(summary.iteration_rows)
    ? summary.iteration_rows.map((row) => ({ ...row }))
    : [];
  const phase2ProbeRows = Array.isArray(summary.phase2_probe_rows)
    ? summary.phase2_probe_rows.map((entry) => ({ ...entry }))
    : [];
  const phase2ProbeCaseIndex = summary.phase2_probe_case_index && typeof summary.phase2_probe_case_index === 'object'
    ? { ...summary.phase2_probe_case_index }
    : {};
  const transitions = Array.isArray(summary?.monotonicity?.transitions)
    ? summary.monotonicity.transitions.map((item) => ({ ...item }))
    : [];
  const monotonicity = {
    transitions,
    saw_improvement: summary?.monotonicity?.saw_improvement === true,
    reached_fixedpoint: summary?.monotonicity?.reached_fixedpoint === true,
  };
  return {
    ...summary,
    store_path: '<store_path>',
    iterations,
    iteration_rows: iterationRows,
    phase2_probe_rows: phase2ProbeRows,
    phase2_probe_case_index: phase2ProbeCaseIndex,
    monotonicity,
  };
  return canonicalizeObjectV0(base);
}

async function runFixedpointProofV0(outDir) {
  const sourceDateEpochRaw = process.env.SOURCE_DATE_EPOCH ?? `${DEFAULT_SOURCE_DATE_EPOCH_V0}`;
  const sourceDateEpoch = Number.parseInt(`${sourceDateEpochRaw}`, 10);
  assertV0(Number.isInteger(sourceDateEpoch) && sourceDateEpoch > 0, 'SOURCE_DATE_EPOCH must be a positive integer');
  assertV0(process.env.TZ === 'UTC', 'TZ must be UTC');

  await mkdir(outDir, { recursive: true });
  const summaryA = await runFixedpointPassV0(outDir, sourceDateEpoch);
  const summaryB = await runFixedpointPassV0(outDir, sourceDateEpoch);

  const canonicalA = canonicalSummaryPayloadV0(summaryA);
  const canonicalB = canonicalSummaryPayloadV0(summaryB);
  const canonicalABytes = Buffer.from(JSON.stringify(canonicalA), 'utf8');
  const canonicalBBytes = Buffer.from(JSON.stringify(canonicalB), 'utf8');
  const canonicalShaA = sha256HexV0(canonicalABytes);
  const canonicalShaB = sha256HexV0(canonicalBBytes);
  await writeFile(path.join(outDir, 'ondemand_fixedpoint_canonical_a.json'), `${JSON.stringify(canonicalA, null, 2)}\n`);
  await writeFile(path.join(outDir, 'ondemand_fixedpoint_canonical_b.json'), `${JSON.stringify(canonicalB, null, 2)}\n`);
  assertV0(canonicalShaA === canonicalShaB, 'ondemand fixedpoint summary must be deterministic across reruns');

  const summary = {
    ...summaryA,
    determinism: {
      reruns: 2,
      canonical_summary_sha256_a: canonicalShaA,
      canonical_summary_sha256_b: canonicalShaB,
      canonical_summary_stable: canonicalShaA === canonicalShaB,
    },
  };
  const summaryPath = path.join(outDir, 'ondemand_fixedpoint_summary.json');
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  return {
    summaryPath,
    storeDir: summaryA.store_path,
    iterations: summaryA.iterations,
    phase2ProbeChecks: summaryA.phase2_probe_checks,
    determinism: summary.determinism,
  };
}

async function runCliV0() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'ondemand_fixedpoint_v0'));
  const result = await runFixedpointProofV0(outDir);
  const finalIter = result.iterations[result.iterations.length - 1];
  console.log(`PASS: fixedpoint summary ${result.summaryPath}`);
  console.log(`PASS: fixedpoint store ${result.storeDir}`);
  console.log(`PASS: fixedpoint iterations ${result.iterations.length}`);
  console.log(`PASS: final resolved=${finalIter.resolved_resources_count} missing=${finalIter.missing}`);
  console.log(`PASS: phase2 probes ${result.phase2ProbeChecks.length} status+hint checks`);
  console.log(
    `PASS: deterministic summary sha256 ${result.determinism.canonical_summary_sha256_a}`,
  );
  console.log('PASS: on-demand fixedpoint proof v0');
}

if (import.meta.url === new URL(process.argv[1], 'file://').href) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: on-demand fixedpoint proof v0: ${message}`);
    process.exit(1);
  });
}
