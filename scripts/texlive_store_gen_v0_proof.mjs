import http from 'node:http';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { mkdir, readFile, readdir, rm, stat, writeFile } from 'node:fs/promises';

import { generateTexliveStoreV0 } from './texlive_store_gen_v0.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function assertV0(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

async function startStubServerV0() {
  const hitCount = new Map();

  const resources = new Map([
    [
      '/xetex/tex/found_one',
      {
        status: 200,
        body: Buffer.from('store-gen-found-one\n', 'utf8'),
        headers: {
          fileid: 'file_found_one_v0',
          'content-type': 'application/octet-stream',
        },
      },
    ],
    [
      '/fontconfig/public/FoundSans',
      {
        status: 200,
        body: Buffer.from('store-gen-found-font\n', 'utf8'),
        headers: {
          fontid: 'font_foundsans_v0',
          'content-type': 'application/octet-stream',
        },
      },
    ],
  ]);

  const server = http.createServer((req, res) => {
    const url = req.url ?? '';
    hitCount.set(url, (hitCount.get(url) ?? 0) + 1);
    const resource = resources.get(url);
    if (!resource) {
      res.statusCode = 404;
      res.end('not found');
      return;
    }
    res.statusCode = resource.status;
    for (const [key, value] of Object.entries(resource.headers)) {
      res.setHeader(key, value);
    }
    res.end(resource.body);
  });

  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => resolve());
  });

  const address = server.address();
  if (!address || typeof address !== 'object') {
    throw new Error('failed to resolve server address');
  }

  return {
    endpoint: `http://127.0.0.1:${address.port}`,
    hitCount,
    close: () =>
      new Promise((resolve, reject) => {
        server.close((error) => (error ? reject(error) : resolve()));
      }),
  };
}

async function readStoreSnapshotV0(storeDir) {
  const indexBytes = await readFile(path.join(storeDir, 'index.json'));
  const index = JSON.parse(indexBytes.toString('utf8'));
  const blobsDir = path.join(storeDir, 'blobs');
  const blobNames = (await readdir(blobsDir)).sort();
  const blobShas = {};

  for (const name of blobNames) {
    const blobPath = path.join(blobsDir, name);
    const blobStats = await stat(blobPath);
    assertV0(blobStats.isFile(), `blob path is not file: ${name}`);
    const blobBytes = await readFile(blobPath);
    const actualSha = sha256HexV0(blobBytes);
    assertV0(actualSha === name, `blob filename must equal sha256: ${name}`);
    blobShas[name] = actualSha;
  }

  return {
    index,
    indexSha256: sha256HexV0(indexBytes),
    blobNames,
    blobShas,
  };
}

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'texlive_store_gen_v0_proof'));
  await rm(outDir, { recursive: true, force: true });
  await mkdir(outDir, { recursive: true });

  const requestListPath = path.join(outDir, 'requests.json');
  const requestList = {
    version: 1,
    requests: [
      { kind: 'texmf', format: 'tex', name: 'found_one', variant: 'v0' },
      { kind: 'fontconfig', format: 'otf', name: 'FoundSans', variant: 'public' },
      { kind: 'texmf', format: 'sty', name: 'missing_one', variant: 'v0' },
    ],
  };
  await writeFile(requestListPath, `${JSON.stringify(requestList, null, 2)}\n`);

  const stub = await startStubServerV0();
  try {
    const firstStoreDir = path.join(outDir, 'store_run1');
    const secondStoreDir = path.join(outDir, 'store_run2');

    const firstRun = await generateTexliveStoreV0({
      rootDir,
      requestListPath,
      storeDir: firstStoreDir,
      backend: 'endpoint_v0',
      endpoint: stub.endpoint,
      sourceDateEpoch: 1_700_000_000,
      timeoutMs: 2000,
    });

    const secondRun = await generateTexliveStoreV0({
      rootDir,
      requestListPath,
      storeDir: secondStoreDir,
      backend: 'endpoint_v0',
      endpoint: stub.endpoint,
      sourceDateEpoch: 1_700_000_000,
      timeoutMs: 2000,
    });

    const firstSnapshot = await readStoreSnapshotV0(firstStoreDir);
    const secondSnapshot = await readStoreSnapshotV0(secondStoreDir);

    assertV0(firstRun.foundCount === 2 && firstRun.missingCount === 1, 'first run must produce 2 found and 1 missing');
    assertV0(secondRun.foundCount === 2 && secondRun.missingCount === 1, 'second run must produce 2 found and 1 missing');

    assertV0(firstSnapshot.indexSha256 === secondSnapshot.indexSha256, 'index sha256 must be stable across runs');
    assertV0(JSON.stringify(firstSnapshot.blobNames) === JSON.stringify(secondSnapshot.blobNames), 'blob file set must be stable');
    assertV0(JSON.stringify(firstSnapshot.blobShas) === JSON.stringify(secondSnapshot.blobShas), 'blob sha map must be stable');
    assertV0(firstRun.indexSha256 === secondRun.indexSha256, 'run-reported index sha256 must match');

    const entries = Array.isArray(firstSnapshot.index.entries) ? firstSnapshot.index.entries : [];
    assertV0(entries.length === 2, 'index must contain two found entries');

    const expectedHitCounts = {
      '/xetex/tex/found_one': 2,
      '/fontconfig/public/FoundSans': 2,
      '/xetex/sty/missing_one': 2,
    };
    for (const [key, expected] of Object.entries(expectedHitCounts)) {
      assertV0((stub.hitCount.get(key) ?? 0) === expected, `expected ${expected} hits for ${key}`);
    }

    const report = {
      endpoint: stub.endpoint,
      request_list: requestListPath,
      first_store: firstStoreDir,
      second_store: secondStoreDir,
      index_sha256: firstSnapshot.indexSha256,
      blob_sha256: firstSnapshot.blobShas,
      deterministic: {
        index_sha256_stable: true,
        blob_sha256_stable: true,
      },
      hit_count: Object.fromEntries(expectedHitCounts ? Object.keys(expectedHitCounts).map((key) => [key, stub.hitCount.get(key) ?? 0]) : []),
    };
    const reportPath = path.join(outDir, 'report.json');
    await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`);

    console.log(`PASS: generated stores ${firstStoreDir} and ${secondStoreDir}`);
    console.log(`PASS: index_sha256 stable ${firstSnapshot.indexSha256}`);
    console.log('PASS: blob sha256 stable across reruns');
    console.log(`PASS: report ${reportPath}`);
    console.log('PASS: texlive store generator v0 proof');
  } finally {
    await stub.close();
  }
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: texlive store generator v0 proof: ${message}`);
  process.exit(1);
});
