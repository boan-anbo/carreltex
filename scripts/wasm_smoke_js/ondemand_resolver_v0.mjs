import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';

const DEFAULT_INDEX_V0 = {
  version: 1,
  entries: [],
};

const RESOLVER_BACKEND_OFFLINE_V0 = 'offline_store_v0';
const RESOLVER_BACKEND_ENDPOINT_V0 = 'endpoint_v0';
const DEFAULT_BACKEND_V0 = RESOLVER_BACKEND_OFFLINE_V0;
const DEFAULT_TIMEOUT_MS_V0 = 3000;
const MAX_ENDPOINT_BYTES_V0 = 4 * 1024 * 1024;

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

function makeRequestKeyV0(kind, format, name, variant) {
  return `${kind}\u001f${format}\u001f${name}\u001f${variant}`;
}

function makeNotFoundV0(cacheHit) {
  return { tag: 'NotFound', cache_hit: cacheHit };
}

function buildFoundResultV0(bytes, sha256, stableId, cacheHit) {
  return {
    tag: 'Found',
    bytes,
    sha256,
    stable_id: stableId,
    cache_hit: cacheHit,
  };
}

function normalizeEndpointV0(endpoint) {
  if (typeof endpoint !== 'string') {
    return '';
  }
  const trimmed = endpoint.trim();
  if (trimmed === '') {
    return '';
  }
  return trimmed.replace(/\/+$/, '');
}

function stableIdFromHeadersV0(headers, fallbackSha) {
  const headerStableId = headers.get('fileid') ?? headers.get('fontid');
  if (headerStableId && isSafeTokenV0(headerStableId)) {
    return headerStableId;
  }
  return `sha256:${fallbackSha}`;
}

async function readIndexEntriesV0(indexPath, blobsDir) {
  await mkdir(blobsDir, { recursive: true });

  let indexBytes;
  try {
    indexBytes = await readFile(indexPath);
  } catch (error) {
    if (error && error.code !== 'ENOENT') {
      throw error;
    }
    indexBytes = Buffer.from(`${JSON.stringify(DEFAULT_INDEX_V0, null, 2)}\n`, 'utf8');
    await writeFile(indexPath, indexBytes);
  }

  let parsedIndex;
  try {
    parsedIndex = JSON.parse(indexBytes.toString('utf8'));
  } catch {
    throw new Error(`invalid resolver index json: ${indexPath}`);
  }

  const entriesRaw = Array.isArray(parsedIndex.entries) ? parsedIndex.entries : [];
  const entries = [];
  for (const rawEntry of entriesRaw) {
    const kind = rawEntry?.kind;
    const format = rawEntry?.format;
    const name = rawEntry?.name;
    const variant = normalizeVariantV0(rawEntry?.variant);
    const sha256 = rawEntry?.sha256;
    const stableId = rawEntry?.stable_id;
    if (
      !isSafeTokenV0(kind)
      || !isSafeTokenV0(format)
      || !isSafeTokenV0(name)
      || variant === null
      || typeof sha256 !== 'string'
      || !/^[0-9a-f]{64}$/.test(sha256)
      || !isSafeTokenV0(stableId)
    ) {
      continue;
    }
    entries.push({
      kind,
      format,
      name,
      variant,
      sha256,
      stable_id: stableId,
    });
  }

  return {
    indexBytes,
    entries,
  };
}

export async function createOfflineOnDemandResolverV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? process.cwd());
  const storeDir = path.resolve(options.storeDir ?? path.join(rootDir, 'target', 'texlive_store_v0'));
  const indexPath = path.join(storeDir, 'index.json');
  const blobsDir = path.join(storeDir, 'blobs');

  const { indexBytes, entries } = await readIndexEntriesV0(indexPath, blobsDir);
  const indexSha256 = sha256HexV0(indexBytes);
  const resolverId = `offline-store-v0:${indexSha256}`;
  const cache = new Map();

  async function resolve(request) {
    const kind = request?.kind;
    const format = request?.format;
    const name = request?.name;
    const variant = normalizeVariantV0(request?.variant);
    const requestResolverId = request?.resolver_id;

    if (!isSafeTokenV0(kind) || !isSafeTokenV0(format) || !isSafeTokenV0(name) || variant === null) {
      return makeNotFoundV0(false);
    }
    if (requestResolverId !== resolverId) {
      return makeNotFoundV0(false);
    }

    const key = makeRequestKeyV0(kind, format, name, variant);
    const cached = cache.get(key);
    if (cached) {
      if (cached.tag === 'Found') {
        return buildFoundResultV0(cached.bytes, cached.sha256, cached.stable_id, true);
      }
      return makeNotFoundV0(true);
    }

    const found = entries.find(
      (entry) =>
        entry.kind === kind
        && entry.format === format
        && entry.name === name
        && entry.variant === variant,
    );
    if (!found) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }

    const blobPath = path.join(blobsDir, found.sha256);
    const bytes = await readFile(blobPath).catch(() => null);
    if (!bytes) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }
    const actualSha = sha256HexV0(bytes);
    if (actualSha !== found.sha256) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }

    const cachedFound = {
      tag: 'Found',
      bytes: new Uint8Array(bytes),
      sha256: actualSha,
      stable_id: found.stable_id,
    };
    cache.set(key, cachedFound);
    return buildFoundResultV0(cachedFound.bytes, cachedFound.sha256, cachedFound.stable_id, false);
  }

  return {
    backend: RESOLVER_BACKEND_OFFLINE_V0,
    resolverId,
    indexSha256,
    storeDir,
    indexPath,
    resolve,
  };
}

export async function createEndpointOnDemandResolverV0(options = {}) {
  const endpoint = normalizeEndpointV0(options.endpoint ?? process.env.TEXLIVE_ENDPOINT ?? '');
  const fetchImpl = options.fetchImpl ?? globalThis.fetch;
  const timeoutMs = Number.isInteger(options.timeoutMs) && options.timeoutMs > 0
    ? options.timeoutMs
    : DEFAULT_TIMEOUT_MS_V0;
  const endpointConfigHash = sha256HexV0(
    Buffer.from(JSON.stringify({ backend: RESOLVER_BACKEND_ENDPOINT_V0, endpoint })),
  );
  const resolverId = `endpoint-v0:${endpointConfigHash}`;
  const cache = new Map();

  async function resolve(request) {
    const kind = request?.kind;
    const format = request?.format;
    const name = request?.name;
    const variant = normalizeVariantV0(request?.variant);
    const requestResolverId = request?.resolver_id;

    if (!isSafeTokenV0(kind) || !isSafeTokenV0(format) || !isSafeTokenV0(name) || variant === null) {
      return makeNotFoundV0(false);
    }
    if (requestResolverId !== resolverId) {
      return makeNotFoundV0(false);
    }
    if (!endpoint || typeof fetchImpl !== 'function') {
      return makeNotFoundV0(false);
    }

    const key = makeRequestKeyV0(kind, format, name, variant);
    const cached = cache.get(key);
    if (cached) {
      if (cached.tag === 'Found') {
        return buildFoundResultV0(cached.bytes, cached.sha256, cached.stable_id, true);
      }
      return makeNotFoundV0(true);
    }

    let relPath;
    if (kind === 'fontconfig') {
      if (!isSafeTokenV0(variant)) {
        cache.set(key, { tag: 'NotFound' });
        return makeNotFoundV0(false);
      }
      relPath = `fontconfig/${variant}/${name}`;
    } else {
      relPath = `xetex/${format}/${name}`;
    }
    const url = `${endpoint}/${relPath}`;

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeoutMs);
    let response;
    try {
      response = await fetchImpl(url, { method: 'GET', signal: controller.signal });
    } catch {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    } finally {
      clearTimeout(timeoutId);
    }

    if (response.status === 404) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }
    if (response.status !== 200) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }

    const arrayBuffer = await response.arrayBuffer().catch(() => null);
    if (!arrayBuffer) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }
    const bytes = new Uint8Array(arrayBuffer);
    if (bytes.length === 0 || bytes.length > MAX_ENDPOINT_BYTES_V0) {
      cache.set(key, { tag: 'NotFound' });
      return makeNotFoundV0(false);
    }

    const sha256 = sha256HexV0(bytes);
    const stableId = stableIdFromHeadersV0(response.headers, sha256);
    const cachedFound = {
      tag: 'Found',
      bytes,
      sha256,
      stable_id: stableId,
    };
    cache.set(key, cachedFound);
    return buildFoundResultV0(cachedFound.bytes, cachedFound.sha256, cachedFound.stable_id, false);
  }

  return {
    backend: RESOLVER_BACKEND_ENDPOINT_V0,
    resolverId,
    endpoint,
    resolve,
  };
}

export async function createOnDemandResolverV0(options = {}) {
  const backend = options.backend ?? process.env.TEXLIVE_RESOLVER_BACKEND_V0 ?? DEFAULT_BACKEND_V0;
  if (backend === RESOLVER_BACKEND_ENDPOINT_V0) {
    return createEndpointOnDemandResolverV0(options);
  }
  return createOfflineOnDemandResolverV0(options);
}

