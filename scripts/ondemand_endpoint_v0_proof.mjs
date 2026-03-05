import http from 'node:http';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createEndpointOnDemandResolverV0 } from './wasm_smoke_js/ondemand_resolver_v0.mjs';

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
  const resources = new Map();

  const texBytes = Buffer.from('endpoint-v0-tex-resource\n', 'utf8');
  const texSha = sha256HexV0(texBytes);
  resources.set('/xetex/tex/demo_pkg', {
    status: 200,
    body: texBytes,
    headers: {
      'content-type': 'application/octet-stream',
      fileid: 'tex_demo_pkg_v0',
    },
    sha256: texSha,
  });

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
    throw new Error('failed to resolve stub server address');
  }
  const endpoint = `http://127.0.0.1:${address.port}`;

  return {
    endpoint,
    server,
    hitCount,
    texSha,
    close: () =>
      new Promise((resolve, reject) => {
        server.close((error) => (error ? reject(error) : resolve()));
      }),
  };
}

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'ondemand_endpoint_v0'));
  await rm(outDir, { recursive: true, force: true });
  await mkdir(outDir, { recursive: true });

  const stub = await startStubServerV0();
  try {
    const resolver = await createEndpointOnDemandResolverV0({
      endpoint: stub.endpoint,
      timeoutMs: 2000,
    });

    const request = {
      kind: 'texmf',
      format: 'tex',
      name: 'demo_pkg',
      variant: 'v0',
      resolver_id: resolver.resolverId,
    };

    const firstFound = await resolver.resolve(request);
    assertV0(firstFound.tag === 'Found', 'first fetch must be Found');
    assertV0(firstFound.cache_hit === false, 'first fetch must be cache miss');
    assertV0(firstFound.sha256 === stub.texSha, 'first fetch sha mismatch');
    assertV0(firstFound.stable_id === 'tex_demo_pkg_v0', 'stable_id must come from fileid header');

    const secondFound = await resolver.resolve(request);
    assertV0(secondFound.tag === 'Found', 'second fetch must be Found');
    assertV0(secondFound.cache_hit === true, 'second fetch must be cache hit');
    assertV0(secondFound.sha256 === stub.texSha, 'second fetch sha mismatch');

    const notFoundRequest = {
      ...request,
      name: 'missing_pkg',
    };
    const firstMiss = await resolver.resolve(notFoundRequest);
    assertV0(firstMiss.tag === 'NotFound', 'first miss must be NotFound');
    assertV0(firstMiss.cache_hit === false, 'first miss must be cache miss');
    const secondMiss = await resolver.resolve(notFoundRequest);
    assertV0(secondMiss.tag === 'NotFound', 'second miss must be NotFound');
    assertV0(secondMiss.cache_hit === true, 'second miss must be cache hit');

    const hitsBeforeUnsafe = new Map(stub.hitCount);
    const slashReject = await resolver.resolve({
      ...request,
      name: 'demo/pkg',
    });
    const dotdotReject = await resolver.resolve({
      ...request,
      name: '../demo_pkg',
    });
    assertV0(slashReject.tag === 'NotFound', 'slash path must be rejected');
    assertV0(dotdotReject.tag === 'NotFound', 'dotdot path must be rejected');
    assertV0(
      (stub.hitCount.get('/xetex/tex/demo/pkg') ?? 0) === 0
      && (stub.hitCount.get('/xetex/tex/../demo_pkg') ?? 0) === 0,
      'unsafe paths must not hit endpoint',
    );
    assertV0(
      (stub.hitCount.get('/xetex/tex/demo_pkg') ?? 0) === (hitsBeforeUnsafe.get('/xetex/tex/demo_pkg') ?? 0),
      'unsafe probes must not mutate valid-path hit count',
    );

    assertV0((stub.hitCount.get('/xetex/tex/demo_pkg') ?? 0) === 1, '200 path should be fetched once');
    assertV0((stub.hitCount.get('/xetex/tex/missing_pkg') ?? 0) === 1, '404 path should be fetched once');

    const report = {
      endpoint: stub.endpoint,
      resolver_id: resolver.resolverId,
      deterministic_sha256: stub.texSha,
      memoization: {
        found_cache_second_hit: secondFound.cache_hit,
        miss_cache_second_hit: secondMiss.cache_hit,
      },
      hit_count: {
        '/xetex/tex/demo_pkg': stub.hitCount.get('/xetex/tex/demo_pkg') ?? 0,
        '/xetex/tex/missing_pkg': stub.hitCount.get('/xetex/tex/missing_pkg') ?? 0,
      },
      path_safety: {
        slash: slashReject.tag,
        dotdot: dotdotReject.tag,
      },
    };
    const reportPath = path.join(outDir, 'report.json');
    await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`);

    console.log(`PASS: endpoint resolver_id ${resolver.resolverId}`);
    console.log(`PASS: 200 memoization sha256 ${stub.texSha}`);
    console.log('PASS: 404 memoization cache hit on second miss');
    console.log('PASS: basename-only path safety rejects "/" and ".."');
    console.log(`PASS: endpoint proof report ${reportPath}`);
    console.log('PASS: on-demand endpoint v0 proof');
  } finally {
    await stub.close();
  }
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: on-demand endpoint v0 proof: ${message}`);
  process.exit(1);
});
