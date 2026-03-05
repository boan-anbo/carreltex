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
