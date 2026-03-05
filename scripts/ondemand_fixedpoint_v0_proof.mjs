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
const PHASE2_PROBE_CASE_IDS_V0 = [
  'typeset_demo_cjk_probe_v0',
  'typeset_demo_math_probe_v0',
  'typeset_demo_hyperref_links_probe_v0',
];

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertV0(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
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

async function verifyPhase2ProbeStatusesV0(galleryOutDir) {
  const reportPath = path.join(galleryOutDir, 'report.json');
  const report = JSON.parse((await readFile(reportPath)).toString('utf8'));
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

    probeChecks.push({
      case_id: caseId,
      expected_status: statusEntry.expected_status,
      actual_status: statusEntry.status,
      expected_vs_actual: statusEntry.expected_vs_actual,
      resource_hints_items: hintItems,
    });
  }
  return probeChecks;
}

async function runFixedpointProofV0(outDir) {
  const sourceDateEpochRaw = process.env.SOURCE_DATE_EPOCH ?? `${DEFAULT_SOURCE_DATE_EPOCH_V0}`;
  const sourceDateEpoch = Number.parseInt(`${sourceDateEpochRaw}`, 10);
  assertV0(Number.isInteger(sourceDateEpoch) && sourceDateEpoch > 0, 'SOURCE_DATE_EPOCH must be a positive integer');
  assertV0(process.env.TZ === 'UTC', 'TZ must be UTC');

  await mkdir(outDir, { recursive: true });
  const runRoot = await mkdtemp(path.join(outDir, 'run_'));
  const storeDir = path.join(runRoot, 'store');
  await mkdir(storeDir, { recursive: true });

  const iterations = [];
  let fixedpointReached = false;
  let sawImprovement = false;
  let fixedpointIteration = 0;
  let fixedpointGalleryOutDir = '';

  for (let iter = 1; iter <= MAX_ITERS_V0; iter += 1) {
    const galleryOutDir = path.join(runRoot, `gallery_iter_${iter}`);
    runGalleryV0(galleryOutDir, storeDir, sourceDateEpoch);

    const reportPath = path.join(galleryOutDir, 'report.json');
    const reportBytes = await readFile(reportPath);
    const report = JSON.parse(reportBytes.toString('utf8'));
    const resolvedResourcesCount = Number(report.resolved_resources_count ?? 0);

    const requestListPath = path.join(runRoot, `request_list_iter_${iter}.json`);
    await buildRequestListFromHintsV0({
      rootDir,
      reportPath,
      outputPath: requestListPath,
    });
    const requestList = JSON.parse((await readFile(requestListPath)).toString('utf8'));
    const requests = Array.isArray(requestList.requests) ? requestList.requests : [];
    const requestCount = Number(requestList.request_count ?? requests.length);

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

    const iteration = {
      iteration: iter,
      resolved_resources_count: resolvedResourcesCount,
      found: storeResult.foundCount,
      missing: storeResult.missingCount,
      request_count: requestCount,
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
      break;
    }
    throw new Error(`fixedpoint transition must improve or stabilize at iteration ${iter}`);
  }

  assertV0(fixedpointReached, `fixedpoint not reached within ${MAX_ITERS_V0} iterations`);
  const finalIter = iterations[iterations.length - 1];
  assertV0(finalIter.missing === 0, `fixedpoint must converge with missing=0, got ${finalIter.missing}`);

  const phase2GalleryOutDir = path.join(runRoot, `gallery_phase2_after_fixedpoint`);
  runGalleryV0(phase2GalleryOutDir, storeDir, sourceDateEpoch);
  const phase2ProbeChecks = await verifyPhase2ProbeStatusesV0(phase2GalleryOutDir);
  const phase2Report = JSON.parse(
    (await readFile(path.join(phase2GalleryOutDir, 'report.json'))).toString('utf8'),
  );
  const phase2ResolvedResources = Number(phase2Report.resolved_resources_count ?? 0);
  assertV0(
    phase2ResolvedResources === finalIter.resolved_resources_count,
    `phase2 rerun must preserve fixedpoint resolved count (${finalIter.resolved_resources_count} != ${phase2ResolvedResources})`,
  );

  const summary = {
    version: 1,
    schema: 'ondemand_fixedpoint_summary_v0',
    source_date_epoch: sourceDateEpoch,
    store_path: storeDir,
    iterations,
    fixedpoint_iteration: fixedpointIteration,
    fixedpoint_resolved_resources_count: finalIter.resolved_resources_count,
    fixedpoint_missing_count: finalIter.missing,
    phase2_probe_checks: phase2ProbeChecks,
    phase2_resolved_resources_count: phase2ResolvedResources,
    fixedpoint_gallery_relpath: path.relative(runRoot, fixedpointGalleryOutDir),
    phase2_gallery_relpath: path.relative(runRoot, phase2GalleryOutDir),
    final_status: 'PASS',
  };
  const summaryPath = path.join(outDir, 'ondemand_fixedpoint_summary.json');
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  return {
    summaryPath,
    storeDir,
    iterations,
    phase2ProbeChecks,
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
  console.log('PASS: on-demand fixedpoint proof v0');
}

if (import.meta.url === new URL(process.argv[1], 'file://').href) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: on-demand fixedpoint proof v0: ${message}`);
    process.exit(1);
  });
}
