import { readdir, readFile, rm, mkdir, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';
import { createOnDemandResolverV0 } from './wasm_smoke_js/ondemand_resolver_v0.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const DEFAULT_SOURCE_DATE_EPOCH_V0 = 1_700_000_000;
const DEFAULT_MAX_LOG_BYTES_V0 = 4096;
const STATUS_OK_V0 = 'OK';
const STATUS_NI_V0 = 'NI';
const STATUS_INVALID_V0 = 'INVALID';
const STATUS_FAIL_V0 = 'FAIL';

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function toUint8ArrayV0(view) {
  return view instanceof Uint8Array ? view : new Uint8Array(view);
}

function readArtifactBytesV0(ctx, lenFn, copyFn, label) {
  const len = lenFn();
  if (!Number.isInteger(len) || len < 0 || len > 32 * 1024 * 1024) {
    throw new Error(`${label}: invalid artifact length ${len}`);
  }
  if (len === 0) {
    return new Uint8Array();
  }
  const outPtr = ctx.alloc(len);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`${label}: alloc failed for len=${len}`);
  }
  try {
    const written = copyFn(outPtr, len);
    if (written !== len) {
      throw new Error(`${label}: expected ${len} written, got ${written}`);
    }
    return new Uint8Array(ctx.memory.buffer, outPtr, len).slice();
  } finally {
    ctx.dealloc(outPtr, len);
  }
}

function readLogBytesV0(ctx) {
  const len = ctx.logLen();
  if (!Number.isInteger(len) || len < 0 || len > 4 * 1024 * 1024) {
    throw new Error(`compile log: invalid length ${len}`);
  }
  if (len === 0) {
    return new Uint8Array();
  }
  const outPtr = ctx.alloc(len);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`compile log: alloc failed for len=${len}`);
  }
  try {
    const written = ctx.logCopy(outPtr, len);
    if (written !== len) {
      throw new Error(`compile log: expected ${len} written, got ${written}`);
    }
    return new Uint8Array(ctx.memory.buffer, outPtr, len).slice();
  } finally {
    ctx.dealloc(outPtr, len);
  }
}

function mapCaseStatusV0(reportStatus, compileCode) {
  if (reportStatus === 'OK') {
    return compileCode === 0 ? STATUS_OK_V0 : STATUS_FAIL_V0;
  }
  if (reportStatus === 'NOT_IMPLEMENTED') {
    return STATUS_NI_V0;
  }
  if (reportStatus === 'INVALID_INPUT') {
    return STATUS_INVALID_V0;
  }
  return STATUS_FAIL_V0;
}

async function loadFixtureCasesV0() {
  const texliveFixtures = [
    {
      id: 'typeset_demo_minimal_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_minimal_v0.tex',
    },
    {
      id: 'typeset_demo_capabilities_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_capabilities_v0.tex',
    },
  ];

  const okFixtureDir = path.join(rootDir, 'scripts', 'wasm_smoke_js', 'fixtures');
  const entries = await readdir(okFixtureDir, { withFileTypes: true });
  const okFixtures = entries
    .filter((entry) => entry.isFile() && entry.name.endsWith('.tex'))
    .map((entry) => entry.name)
    .sort()
    .map((name) => {
      const stem = name.slice(0, -4);
      return {
        id: stem.startsWith('ok_') ? stem : `ok_${stem}`,
        mode: 'ok',
        fixtureRelPath: `scripts/wasm_smoke_js/fixtures/${name}`,
      };
    });

  return [...texliveFixtures, ...okFixtures];
}

function entrypointSetOkV0(ctx, mem, entrypoint) {
  const bytes = new TextEncoder().encode(entrypoint);
  return mem.callWithBytes(bytes, 'entrypoint', (ptr, len) =>
    ctx.compileRequestSetEntrypoint(ptr, len),
  );
}

function buildConfigHashV0(cases, sourceDateEpoch, resolverId) {
  const config = {
    runner: 'wasm_fixture_gallery_v0',
    source_date_epoch: sourceDateEpoch,
    tz: 'UTC',
    max_log_bytes: DEFAULT_MAX_LOG_BYTES_V0,
    resolver_id: resolverId,
    cases: cases.map((item) => ({
      id: item.id,
      mode: item.mode,
      fixture: item.fixtureRelPath,
    })),
  };
  return sha256HexV0(Buffer.from(JSON.stringify(config)));
}

async function runCaseV0(
  ctx,
  mem,
  helpers,
  outDir,
  caseSpec,
  sourceDateEpoch,
  engineRev,
  configHash,
  resolver,
) {
  const caseOutDir = path.join(outDir, caseSpec.id);
  await mkdir(caseOutDir, { recursive: true });

  const fixturePath = path.join(rootDir, caseSpec.fixtureRelPath);
  const fixtureBytes = toUint8ArrayV0(await readFile(fixturePath));
  await writeFile(path.join(caseOutDir, 'main.tex'), fixtureBytes);

  let compileCode = -1;
  let renderCode = -1;
  let report = { status: 'INVALID_INPUT' };
  let caseStatus = STATUS_FAIL_V0;
  let errorMessage = '';

  try {
    if (ctx.mountReset() !== 0) {
      throw new Error('mount_reset failed');
    }
    if (helpers.addMountedFile('main.tex', fixtureBytes, `${caseSpec.id}_main`) !== 0) {
      throw new Error('mount_add_file(main.tex) failed');
    }
    if (ctx.mountFinalize() !== 0) {
      throw new Error('mount_finalize failed');
    }

    if (caseSpec.mode === 'typeset') {
      compileCode = ctx.compileMainTypesetMinimal();
    } else if (caseSpec.mode === 'ok') {
      if (ctx.compileRequestReset() !== 0) {
        throw new Error('compile_request_reset_v0 failed');
      }
      if (entrypointSetOkV0(ctx, mem, 'main.tex') !== 0) {
        throw new Error('compile_request_set_entrypoint_v0 failed');
      }
      if (ctx.compileRequestSetEpoch(BigInt(sourceDateEpoch)) !== 0) {
        throw new Error('compile_request_set_source_date_epoch_v0 failed');
      }
      if (ctx.compileRequestSetMaxLogBytes(DEFAULT_MAX_LOG_BYTES_V0) !== 0) {
        throw new Error('compile_request_set_max_log_bytes_v0 failed');
      }
      compileCode = ctx.compileRun();
    } else {
      throw new Error(`unsupported case mode: ${caseSpec.mode}`);
    }

    report = helpers.readCompileReportJson();
    caseStatus = mapCaseStatusV0(report.status, compileCode);
  } catch (error) {
    caseStatus = STATUS_FAIL_V0;
    errorMessage = error instanceof Error ? error.message : String(error);
    try {
      report = helpers.readCompileReportJson();
    } catch {
      report = { status: 'INVALID_INPUT' };
    }
  }

  const logBytes = (() => {
    try {
      return readLogBytesV0(ctx);
    } catch {
      return new Uint8Array();
    }
  })();
  const xdvBytes = (() => {
    try {
      return readArtifactBytesV0(
        ctx,
        ctx.artifactMainXdvLen,
        ctx.artifactMainXdvCopy,
        `${caseSpec.id}:main.xdv`,
      );
    } catch {
      return new Uint8Array();
    }
  })();

  if (xdvBytes.length > 0) {
    renderCode = ctx.renderMainPdf();
  }
  const pdfBytes = renderCode === 0
    ? readArtifactBytesV0(
      ctx,
      ctx.artifactMainPdfLen,
      ctx.artifactMainPdfCopy,
      `${caseSpec.id}:main.pdf`,
    )
    : new Uint8Array();

  const expectedXdv = caseStatus === STATUS_OK_V0;
  const expectedPdf = caseStatus === STATUS_OK_V0;
  if (expectedXdv && xdvBytes.length === 0) {
    caseStatus = STATUS_FAIL_V0;
    errorMessage = errorMessage || 'expected non-empty main.xdv for OK case';
  }
  if (expectedPdf && pdfBytes.length === 0) {
    caseStatus = STATUS_FAIL_V0;
    errorMessage = errorMessage || 'expected non-empty main.pdf for OK case';
  }

  const resolvedResources = [];
  const resolution = await resolver.resolve({
    kind: 'texmf',
    format: 'tex',
    name: caseSpec.id,
    variant: caseSpec.mode,
    resolver_id: resolver.resolverId,
  });
  if (resolution.tag === 'Found') {
    resolvedResources.push({
      kind: 'texmf',
      format: 'tex',
      name: caseSpec.id,
      variant: caseSpec.mode,
      stable_id: resolution.stable_id,
      sha256: resolution.sha256,
      cache_hit: resolution.cache_hit,
    });
  }

  const summary = {
    case_id: caseSpec.id,
    mode: caseSpec.mode,
    fixture: caseSpec.fixtureRelPath,
    engine_rev: engineRev,
    config_hash: configHash,
    source_date_epoch: sourceDateEpoch,
    status: caseStatus,
    compile_status: report.status ?? 'INVALID_INPUT',
    compile_code: compileCode,
    render_code: renderCode,
    artifact_bytes: {
      main_xdv: xdvBytes.length,
      main_pdf: pdfBytes.length,
      compile_log: logBytes.length,
    },
    artifact_sha256: {
      main_xdv: sha256HexV0(xdvBytes),
      main_pdf: sha256HexV0(pdfBytes),
    },
    resolver_id: resolver.resolverId,
    resolved_resources: resolvedResources,
  };
  if (errorMessage) {
    summary.error = errorMessage;
  }

  await writeFile(path.join(caseOutDir, 'main.xdv'), xdvBytes);
  await writeFile(path.join(caseOutDir, 'main.pdf'), pdfBytes);
  await writeFile(path.join(caseOutDir, 'compile.log.bin'), logBytes);
  await writeFile(path.join(caseOutDir, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);

  return summary;
}

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'wasm_fixture_gallery_v0'));
  const sourceDateEpochRaw = process.env.SOURCE_DATE_EPOCH ?? `${DEFAULT_SOURCE_DATE_EPOCH_V0}`;
  const sourceDateEpoch = Number.parseInt(sourceDateEpochRaw, 10);
  if (!Number.isInteger(sourceDateEpoch) || sourceDateEpoch <= 0) {
    throw new Error(`SOURCE_DATE_EPOCH must be a positive integer, got: ${sourceDateEpochRaw}`);
  }
  const tz = process.env.TZ;
  if (tz !== 'UTC') {
    throw new Error(`TZ must be UTC, got: ${tz ?? '<unset>'}`);
  }

  const engineRev = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: rootDir,
    encoding: 'utf8',
  }).trim();
  const cases = await loadFixtureCasesV0();
  const resolver = await createOnDemandResolverV0({
    backend: process.env.TEXLIVE_RESOLVER_BACKEND_V0,
    endpoint: process.env.TEXLIVE_ENDPOINT,
    rootDir,
    storeDir: path.join(rootDir, 'target', 'texlive_store_v0'),
  });
  const configHash = buildConfigHashV0(cases, sourceDateEpoch, resolver.resolverId);

  await rm(outDir, { recursive: true, force: true });
  await mkdir(outDir, { recursive: true });

  const ctx = await createCtx(rootDir);
  const mem = createMemHelpers(ctx);
  const helpers = createAssertHelpers(ctx, mem);
  const summaries = [];
  for (const caseSpec of cases) {
    summaries.push(
      await runCaseV0(
        ctx,
        mem,
        helpers,
        outDir,
        caseSpec,
        sourceDateEpoch,
        engineRev,
        configHash,
        resolver,
      ),
    );
  }

  const report = {
    engine_rev: engineRev,
    source_date_epoch: sourceDateEpoch,
    resolver_id: resolver.resolverId,
    config_hash: configHash,
    case_count: summaries.length,
    statuses: summaries.map((summary) => ({
      case_id: summary.case_id,
      status: summary.status,
      artifact_sha256: summary.artifact_sha256,
    })),
  };
  await writeFile(path.join(outDir, 'report.json'), `${JSON.stringify(report, null, 2)}\n`);

  if (summaries.some((summary) => summary.status === STATUS_FAIL_V0)) {
    throw new Error('gallery contains FAIL cases');
  }

  console.log(`PASS: fixture gallery report ${path.join(outDir, 'report.json')}`);
  for (const summary of summaries) {
    console.log(`PASS: case ${summary.case_id} status ${summary.status}`);
    console.log(`PASS: ${summary.case_id} xdv_sha256 ${summary.artifact_sha256.main_xdv}`);
    console.log(`PASS: ${summary.case_id} pdf_sha256 ${summary.artifact_sha256.main_pdf}`);
  }
  console.log('PASS: wasm fixture gallery v0');
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: wasm fixture gallery v0: ${message}`);
  process.exit(1);
});
