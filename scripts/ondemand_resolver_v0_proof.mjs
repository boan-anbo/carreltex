import { mkdir, rm, writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createOfflineOnDemandResolverV0 } from './wasm_smoke_js/ondemand_resolver_v0.mjs';

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

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'ondemand_resolver_v0'));
  const storeDir = path.join(outDir, 'store');
  const blobsDir = path.join(storeDir, 'blobs');
  await rm(outDir, { recursive: true, force: true });
  await mkdir(blobsDir, { recursive: true });

  const blobBytes = Buffer.from('resolver-v0-deterministic-bytes\n', 'utf8');
  const blobSha = sha256HexV0(blobBytes);
  await writeFile(path.join(blobsDir, blobSha), blobBytes);

  const index = {
    version: 1,
    entries: [
      {
        kind: 'texmf',
        format: 'sty',
        name: 'demo_pkg',
        variant: 'v0',
        sha256: blobSha,
        stable_id: 'texmf_sty_demo_pkg_v0',
      },
    ],
  };
  const indexPath = path.join(storeDir, 'index.json');
  await writeFile(indexPath, `${JSON.stringify(index, null, 2)}\n`);

  const resolver = await createOfflineOnDemandResolverV0({
    rootDir,
    storeDir,
  });

  const expectedIndexSha = sha256HexV0(await readFile(indexPath));
  assertV0(
    resolver.resolverId === `offline-store-v0:${expectedIndexSha}`,
    'resolver_id must include index hash',
  );

  const request = {
    kind: 'texmf',
    format: 'sty',
    name: 'demo_pkg',
    variant: 'v0',
    resolver_id: resolver.resolverId,
  };
  const first = await resolver.resolve(request);
  assertV0(first.tag === 'Found', 'first resolve must be Found');
  assertV0(first.cache_hit === false, 'first resolve must be cache miss');
  assertV0(first.sha256 === blobSha, 'first resolve sha mismatch');
  assertV0(Buffer.compare(Buffer.from(first.bytes), blobBytes) === 0, 'first resolve bytes mismatch');

  const second = await resolver.resolve(request);
  assertV0(second.tag === 'Found', 'second resolve must be Found');
  assertV0(second.cache_hit === true, 'second resolve must be cache hit');
  assertV0(second.sha256 === blobSha, 'second resolve sha mismatch');
  assertV0(Buffer.compare(Buffer.from(second.bytes), blobBytes) === 0, 'second resolve bytes mismatch');

  const unsafeSlash = await resolver.resolve({
    ...request,
    name: 'demo/pkg',
  });
  assertV0(unsafeSlash.tag === 'NotFound', 'slash path must be rejected');
  const unsafeDotDot = await resolver.resolve({
    ...request,
    name: '../demo_pkg',
  });
  assertV0(unsafeDotDot.tag === 'NotFound', 'dotdot path must be rejected');

  const report = {
    resolver_id: resolver.resolverId,
    index_sha256: expectedIndexSha,
    request,
    deterministic_sha256: blobSha,
    cache_second_hit: second.cache_hit,
    path_safety: {
      slash: unsafeSlash.tag,
      dotdot: unsafeDotDot.tag,
    },
  };
  const reportPath = path.join(outDir, 'report.json');
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`);

  console.log(`PASS: resolver_id ${resolver.resolverId}`);
  console.log(`PASS: deterministic_sha256 ${blobSha}`);
  console.log('PASS: cache hit on second resolve');
  console.log('PASS: path safety rejects "/" and ".."');
  console.log(`PASS: resolver proof report ${reportPath}`);
  console.log('PASS: on-demand resolver v0 proof');
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: on-demand resolver v0 proof: ${message}`);
  process.exit(1);
});
