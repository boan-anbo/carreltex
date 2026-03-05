import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

import { createOnDemandResolverV0 } from './wasm_smoke_js/ondemand_resolver_v0.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDirDefaultV0 = path.resolve(__dirname, '..');

const DEFAULT_SOURCE_DATE_EPOCH_V0 = 1_700_000_000;
const STORE_BACKEND_FIXTURE_DIR_V0 = 'fixture_dir_v0';

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function isSafeTokenV0(value) {
  return typeof value === 'string'
    && value.length > 0
    && !value.includes('/')
    && !value.includes('\\')
    && !value.includes('..');
}

function normalizeVariantV0(variant) {
  if (variant === undefined || variant === null || variant === '') {
    return '';
  }
  if (!isSafeTokenV0(variant)) {
    return null;
  }
  return variant;
}

function normalizeStableIdV0(stableId, sha256) {
  if (isSafeTokenV0(stableId)) {
    return stableId;
  }
  return `sha256:${sha256}`;
}

function buildFixtureStableIdV0(request, sha256) {
  const variantPart = request.variant === '' ? 'default' : request.variant;
  const candidate = `fixture_${request.kind}_${request.format}_${variantPart}_${request.name}`;
  return normalizeStableIdV0(candidate, sha256);
}

function requestKeyV0(request) {
  return `${request.kind}\u001f${request.format}\u001f${request.name}\u001f${request.variant}`;
}

function normalizeRequestV0(rawRequest, index) {
  const request = {
    kind: rawRequest?.kind,
    format: rawRequest?.format,
    name: rawRequest?.name,
    variant: normalizeVariantV0(rawRequest?.variant),
  };
  if (!isSafeTokenV0(request.kind) || !isSafeTokenV0(request.format) || !isSafeTokenV0(request.name) || request.variant === null) {
    throw new Error(`invalid request at index ${index}`);
  }
  return request;
}

function sortEntriesV0(entries) {
  entries.sort((left, right) => {
    const leftKey = `${left.kind}\u0000${left.format}\u0000${left.name}\u0000${left.variant}`;
    const rightKey = `${right.kind}\u0000${right.format}\u0000${right.name}\u0000${right.variant}`;
    if (leftKey < rightKey) {
      return -1;
    }
    if (leftKey > rightKey) {
      return 1;
    }
    if (left.sha256 < right.sha256) {
      return -1;
    }
    if (left.sha256 > right.sha256) {
      return 1;
    }
    if (left.stable_id < right.stable_id) {
      return -1;
    }
    if (left.stable_id > right.stable_id) {
      return 1;
    }
    return 0;
  });
  return entries;
}

async function loadRequestListV0(requestListPath) {
  const bytes = await readFile(requestListPath);
  let parsed;
  try {
    parsed = JSON.parse(bytes.toString('utf8'));
  } catch {
    throw new Error(`invalid JSON: ${requestListPath}`);
  }

  const requestsRaw = Array.isArray(parsed) ? parsed : parsed?.requests;
  if (!Array.isArray(requestsRaw) || requestsRaw.length === 0) {
    throw new Error('request list must be a non-empty array or { requests: [...] }');
  }

  return requestsRaw.map((rawRequest, index) => normalizeRequestV0(rawRequest, index));
}

async function createFixtureDirResolverV0(options = {}) {
  const sourceDir = options.sourceDir;
  if (typeof sourceDir !== 'string' || sourceDir.trim() === '') {
    throw new Error('fixture_dir_v0 backend requires sourceDir');
  }
  const sourceDirResolved = path.resolve(sourceDir);
  const resolverConfigHash = sha256HexV0(
    Buffer.from(
      JSON.stringify({
        backend: STORE_BACKEND_FIXTURE_DIR_V0,
        sourceDir: sourceDirResolved,
      }),
      'utf8',
    ),
  );
  const resolverId = `fixture-dir-v0:${resolverConfigHash}`;
  const cache = new Map();

  async function resolve(request) {
    const kind = request?.kind;
    const format = request?.format;
    const name = request?.name;
    const variant = normalizeVariantV0(request?.variant);
    const requestResolverId = request?.resolver_id;
    if (!isSafeTokenV0(kind) || !isSafeTokenV0(format) || !isSafeTokenV0(name) || variant === null) {
      return { tag: 'NotFound', cache_hit: false };
    }
    if (requestResolverId !== resolverId) {
      return { tag: 'NotFound', cache_hit: false };
    }

    const key = requestKeyV0({ kind, format, name, variant });
    const cached = cache.get(key);
    if (cached) {
      if (cached.tag === 'Found') {
        return {
          tag: 'Found',
          bytes: cached.bytes,
          sha256: cached.sha256,
          stable_id: cached.stable_id,
          cache_hit: true,
        };
      }
      return { tag: 'NotFound', cache_hit: true };
    }

    const relPath = kind === 'fontconfig'
      ? path.join('fontconfig', variant, name)
      : path.join('xetex', format, name);
    const fullPath = path.join(sourceDirResolved, relPath);
    const bytes = await readFile(fullPath).catch(() => null);
    if (!bytes || bytes.length === 0) {
      cache.set(key, { tag: 'NotFound' });
      return { tag: 'NotFound', cache_hit: false };
    }

    const payload = new Uint8Array(bytes);
    const sha256 = sha256HexV0(payload);
    const stableId = buildFixtureStableIdV0({ kind, format, name, variant }, sha256);
    const found = {
      tag: 'Found',
      bytes: payload,
      sha256,
      stable_id: stableId,
    };
    cache.set(key, found);
    return {
      tag: 'Found',
      bytes: found.bytes,
      sha256: found.sha256,
      stable_id: found.stable_id,
      cache_hit: false,
    };
  }

  return {
    backend: STORE_BACKEND_FIXTURE_DIR_V0,
    resolverId,
    resolve,
  };
}

export async function generateTexliveStoreV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? rootDirDefaultV0);
  const requestListPath = path.resolve(options.requestListPath);
  const sourceDateEpochRaw = options.sourceDateEpoch ?? process.env.SOURCE_DATE_EPOCH ?? `${DEFAULT_SOURCE_DATE_EPOCH_V0}`;
  const sourceDateEpoch = Number.parseInt(`${sourceDateEpochRaw}`, 10);
  if (!Number.isInteger(sourceDateEpoch) || sourceDateEpoch <= 0) {
    throw new Error(`SOURCE_DATE_EPOCH must be a positive integer, got: ${sourceDateEpochRaw}`);
  }

  const storeDir = path.resolve(options.storeDir ?? path.join(rootDir, 'target', 'texlive_store_v0'));
  const blobsDir = path.join(storeDir, 'blobs');
  await rm(storeDir, { recursive: true, force: true });
  await mkdir(blobsDir, { recursive: true });

  const requests = await loadRequestListV0(requestListPath);
  const backend = options.backend ?? process.env.TEXLIVE_RESOLVER_BACKEND_V0;
  const sourceDir = options.sourceDir ?? process.env.TEXLIVE_STORE_SOURCE_DIR_V0;
  const resolver = backend === STORE_BACKEND_FIXTURE_DIR_V0
    ? await createFixtureDirResolverV0({ sourceDir })
    : await createOnDemandResolverV0({
      rootDir,
      storeDir,
      backend,
      endpoint: options.endpoint,
      timeoutMs: options.timeoutMs,
      fetchImpl: options.fetchImpl,
    });

  const entryMap = new Map();
  const missing = [];

  for (const request of requests) {
    const result = await resolver.resolve({
      ...request,
      resolver_id: resolver.resolverId,
    });

    if (result.tag !== 'Found') {
      missing.push({
        ...request,
        cache_hit: result.cache_hit,
      });
      continue;
    }

    const bytes = result.bytes instanceof Uint8Array ? result.bytes : new Uint8Array(result.bytes);
    const sha256 = sha256HexV0(bytes);
    if (sha256 !== result.sha256) {
      throw new Error(`resolver sha mismatch for ${request.kind}/${request.format}/${request.name}`);
    }

    const blobPath = path.join(blobsDir, sha256);
    await writeFile(blobPath, bytes);

    const entry = {
      ...request,
      sha256,
      stable_id: normalizeStableIdV0(result.stable_id, sha256),
    };
    const key = requestKeyV0(request);
    const existing = entryMap.get(key);
    if (!existing) {
      entryMap.set(key, entry);
      continue;
    }
    if (existing.sha256 !== entry.sha256 || existing.stable_id !== entry.stable_id) {
      throw new Error(`conflicting resolver result for request key ${key}`);
    }
  }

  const entries = sortEntriesV0(Array.from(entryMap.values()));
  const indexJson = {
    version: 1,
    entries,
  };
  const indexPath = path.join(storeDir, 'index.json');
  const indexBytes = Buffer.from(`${JSON.stringify(indexJson, null, 2)}\n`, 'utf8');
  await writeFile(indexPath, indexBytes);

  const summary = {
    version: 1,
    backend: resolver.backend,
    resolver_id: resolver.resolverId,
    source_date_epoch: sourceDateEpoch,
    request_count: requests.length,
    found_count: entries.length,
    missing_count: missing.length,
    index_sha256: sha256HexV0(indexBytes),
    resolved_resources: entries,
    missing_requests: missing,
  };
  const summaryPath = path.join(storeDir, 'summary.json');
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  return {
    storeDir,
    indexPath,
    summaryPath,
    indexSha256: summary.index_sha256,
    resolverId: resolver.resolverId,
    foundCount: entries.length,
    missingCount: missing.length,
  };
}

async function runCliV0() {
  const requestListArg = process.argv[2];
  if (!requestListArg) {
    throw new Error('usage: node scripts/texlive_store_gen_v0.mjs <request_list.json> [store_dir]');
  }

  const storeDirArg = process.argv[3];
  const result = await generateTexliveStoreV0({
    requestListPath: requestListArg,
    storeDir: storeDirArg,
  });

  console.log(`PASS: texlive store dir ${result.storeDir}`);
  console.log(`PASS: index_sha256 ${result.indexSha256}`);
  console.log(`PASS: found=${result.foundCount} missing=${result.missingCount}`);
}

if (import.meta.url === new URL(process.argv[1], 'file://').href) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: texlive store gen v0: ${message}`);
    process.exit(1);
  });
}
