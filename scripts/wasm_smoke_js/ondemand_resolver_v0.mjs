import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';

const DEFAULT_INDEX_V0 = {
  version: 1,
  entries: [],
};

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

export async function createOfflineOnDemandResolverV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? process.cwd());
  const storeDir = path.resolve(options.storeDir ?? path.join(rootDir, 'target', 'texlive_store_v0'));
  const indexPath = path.join(storeDir, 'index.json');
  const blobsDir = path.join(storeDir, 'blobs');

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
      return { tag: 'NotFound' };
    }
    if (requestResolverId !== resolverId) {
      return { tag: 'NotFound' };
    }

    const key = makeRequestKeyV0(kind, format, name, variant);
    const cached = cache.get(key);
    if (cached) {
      return {
        tag: 'Found',
        bytes: cached.bytes,
        sha256: cached.sha256,
        stable_id: cached.stable_id,
        cache_hit: true,
      };
    }

    const found = entries.find(
      (entry) =>
        entry.kind === kind
        && entry.format === format
        && entry.name === name
        && entry.variant === variant,
    );
    if (!found) {
      return { tag: 'NotFound' };
    }

    const blobPath = path.join(blobsDir, found.sha256);
    const bytes = await readFile(blobPath).catch(() => null);
    if (!bytes) {
      return { tag: 'NotFound' };
    }
    const actualSha = sha256HexV0(bytes);
    if (actualSha !== found.sha256) {
      return { tag: 'NotFound' };
    }

    const result = {
      bytes: new Uint8Array(bytes),
      sha256: actualSha,
      stable_id: found.stable_id,
    };
    cache.set(key, result);

    return {
      tag: 'Found',
      bytes: result.bytes,
      sha256: result.sha256,
      stable_id: result.stable_id,
      cache_hit: false,
    };
  }

  return {
    resolverId,
    indexSha256,
    storeDir,
    indexPath,
    resolve,
  };
}
