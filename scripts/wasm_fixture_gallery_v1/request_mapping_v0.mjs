import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { ensureDefaultExtensionV0, isSafeResolverTokenV0 } from './typed_artifacts_v0.mjs';
function inferTexmfFormatFromNameV0(name, fallback) {
  const dotIndex = name.lastIndexOf('.');
  if (dotIndex <= 0 || dotIndex === name.length - 1) {
    return fallback;
  }
  const ext = name.slice(dotIndex + 1).toLowerCase();
  if (!/^[a-z0-9]+$/.test(ext)) {
    return fallback;
  }
  return ext;
}

function parseFontconfigHintTokenV0(value) {
  const prefix = 'fontconfig:';
  if (!value.startsWith(prefix)) {
    return null;
  }
  const payload = value.slice(prefix.length);
  const firstColon = payload.indexOf(':');
  if (firstColon <= 0 || firstColon === payload.length - 1) {
    return null;
  }
  const variant = payload.slice(0, firstColon).trim();
  const name = payload.slice(firstColon + 1).trim();
  if (!isSafeResolverTokenV0(variant) || !isSafeResolverTokenV0(name)) {
    return null;
  }
  return {
    kind: 'fontconfig',
    format: 'name',
    name,
    variant,
    hint_type: 'hyperref_url',
  };
}

function resolverRequestKeyV0(request) {
  return `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
}

async function collectResolverRequestsFromResourceHintsV0(caseSpec, caseOutDir, resourceHintsArtifact) {
  if (resourceHintsArtifact?.present !== true) {
    return [];
  }
  const relpath = resourceHintsArtifact?.artifact_relpath;
  if (typeof relpath !== 'string' || relpath.length === 0) {
    return [];
  }
  const payload = JSON.parse((await readFile(path.join(caseOutDir, relpath))).toString('utf8'));
  const entries = Array.isArray(payload?.entries) ? payload.entries : [];
  const requestsByKey = new Map();

  const addTexmfRequest = (name, fallbackFormat, hintType) => {
    if (!isSafeResolverTokenV0(name)) {
      throw new Error(`unsafe resource hint token for ${hintType} in case ${caseSpec.id}`);
    }
    const format = inferTexmfFormatFromNameV0(name, fallbackFormat);
    if (!isSafeResolverTokenV0(format)) {
      throw new Error(`unsafe format token '${format}' for ${hintType} in case ${caseSpec.id}`);
    }
    const request = {
      kind: 'texmf',
      format,
      name,
      variant: caseSpec.mode,
      hint_type: hintType,
    };
    requestsByKey.set(resolverRequestKeyV0(request), request);
  };

  for (const entry of entries) {
    const hintType = typeof entry?.hint_type === 'string' ? entry.hint_type : '';
    const value = typeof entry?.value === 'string' ? entry.value : '';
    if (!hintType || !value) {
      continue;
    }
    if (hintType === 'graphics_path') {
      addTexmfRequest(value, 'graphic', hintType);
      continue;
    }
    if (hintType === 'bib_resource') {
      addTexmfRequest(ensureDefaultExtensionV0(value, 'bib'), 'bib', hintType);
      continue;
    }
    if (hintType === 'tex_input' || hintType === 'tex_include' || hintType === 'tex_includeonly') {
      addTexmfRequest(ensureDefaultExtensionV0(value, 'tex'), 'tex', hintType);
      continue;
    }
    if (hintType === 'package_file') {
      addTexmfRequest(ensureDefaultExtensionV0(value, 'sty'), 'sty', hintType);
      continue;
    }
    if (hintType === 'hyperref_url') {
      const fontconfigRequest = parseFontconfigHintTokenV0(value);
      if (fontconfigRequest) {
        requestsByKey.set(resolverRequestKeyV0(fontconfigRequest), fontconfigRequest);
      }
    }
  }

  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
}

async function collectResolverRequestsFromTypedArtifactsV0(caseSpec, caseOutDir, typedArtifacts) {
  const requestsByKey = new Map();

  const addTexmfRequest = (name, fallbackFormat, hintType) => {
    if (!isSafeResolverTokenV0(name)) {
      throw new Error(`unsafe resource hint token for ${hintType} in case ${caseSpec.id}`);
    }
    const format = inferTexmfFormatFromNameV0(name, fallbackFormat);
    if (!isSafeResolverTokenV0(format)) {
      throw new Error(`unsafe format token '${format}' for ${hintType} in case ${caseSpec.id}`);
    }
    const variant = caseSpec.mode;
    const request = {
      kind: 'texmf',
      format,
      name,
      variant,
      hint_type: hintType,
    };
    requestsByKey.set(resolverRequestKeyV0(request), request);
  };

  const graphicsRelpath = typedArtifacts?.graphics?.artifact_relpath;
  if (typedArtifacts?.graphics?.present === true && typeof graphicsRelpath === 'string' && graphicsRelpath.length > 0) {
    const graphicsPayload = JSON.parse((await readFile(path.join(caseOutDir, graphicsRelpath))).toString('utf8'));
    const graphicsEntries = Array.isArray(graphicsPayload?.entries) ? graphicsPayload.entries : [];
    for (const entry of graphicsEntries) {
      const resolverPath = typeof entry?.resolver_path === 'string' && entry.resolver_path.length > 0
        ? entry.resolver_path
        : null;
      const fallbackPath = typeof entry?.path === 'string' && entry.path.length > 0
        ? entry.path
        : null;
      const requestPath = resolverPath ?? fallbackPath;
      if (requestPath) {
        addTexmfRequest(requestPath, 'graphic', 'graphics_path');
      }
    }
  }

  const bibRelpath = typedArtifacts?.bib?.artifact_relpath;
  if (typedArtifacts?.bib?.present === true && typeof bibRelpath === 'string' && bibRelpath.length > 0) {
    const bibPayload = JSON.parse((await readFile(path.join(caseOutDir, bibRelpath))).toString('utf8'));
    const bibEntries = Array.isArray(bibPayload?.entries) ? bibPayload.entries : [];
    for (const entry of bibEntries) {
      if (entry?.kind === 'resource_hint' && typeof entry?.value === 'string' && entry.value.length > 0) {
        addTexmfRequest(entry.value, 'bib', 'bib_resource');
      }
    }
  }

  const hyperrefRelpath = typedArtifacts?.hyperref?.artifact_relpath;
  if (typedArtifacts?.hyperref?.present === true && typeof hyperrefRelpath === 'string' && hyperrefRelpath.length > 0) {
    const hyperrefPayload = JSON.parse((await readFile(path.join(caseOutDir, hyperrefRelpath))).toString('utf8'));
    const hyperrefEntries = Array.isArray(hyperrefPayload?.entries) ? hyperrefPayload.entries : [];
    for (const entry of hyperrefEntries) {
      if (typeof entry?.target !== 'string' || entry.target.length === 0) {
        continue;
      }
      const fontconfigRequest = parseFontconfigHintTokenV0(entry.target);
      if (!fontconfigRequest) {
        continue;
      }
      requestsByKey.set(resolverRequestKeyV0(fontconfigRequest), fontconfigRequest);
    }
  }

  const inputRelpath = typedArtifacts?.input?.artifact_relpath;
  if (typedArtifacts?.input?.present === true && typeof inputRelpath === 'string' && inputRelpath.length > 0) {
    const inputPayload = JSON.parse((await readFile(path.join(caseOutDir, inputRelpath))).toString('utf8'));
    const inputEntries = Array.isArray(inputPayload?.entries) ? inputPayload.entries : [];
    for (const entry of inputEntries) {
      if (typeof entry?.value !== 'string' || entry.value.length === 0) {
        continue;
      }
      const hintType = entry?.command === 'include' ? 'tex_include' : 'tex_input';
      addTexmfRequest(entry.value, 'tex', hintType);
    }
  }

  const packagesRelpath = typedArtifacts?.packages?.artifact_relpath;
  if (typedArtifacts?.packages?.present === true && typeof packagesRelpath === 'string' && packagesRelpath.length > 0) {
    const packagesPayload = JSON.parse((await readFile(path.join(caseOutDir, packagesRelpath))).toString('utf8'));
    const packageEntries = Array.isArray(packagesPayload?.entries) ? packagesPayload.entries : [];
    for (const entry of packageEntries) {
      if (typeof entry?.name !== 'string' || entry.name.length === 0) {
        continue;
      }
      addTexmfRequest(entry.name, 'sty', 'package_file');
    }
  }

  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
}

async function computeBaselineMatchV0(caseId, artifactSha256, baselineDir) {
  if (!baselineDir) {
    return null;
  }
  const caseDir = path.join(baselineDir, caseId);
  const xdvPath = path.join(caseDir, 'main.xdv.sha256');
  const pdfPath = path.join(caseDir, 'main.pdf.sha256');
  const [xdvExpectedBytes, pdfExpectedBytes] = await Promise.all([
    readFile(xdvPath).catch(() => null),
    readFile(pdfPath).catch(() => null),
  ]);
  if (!xdvExpectedBytes || !pdfExpectedBytes) {
    return 'MISSING';
  }
  const xdvExpected = xdvExpectedBytes.toString('utf8').trim();
  const pdfExpected = pdfExpectedBytes.toString('utf8').trim();
  if (!/^[0-9a-f]{64}$/.test(xdvExpected) || !/^[0-9a-f]{64}$/.test(pdfExpected)) {
    return 'MISMATCH';
  }
  if (xdvExpected === artifactSha256.main_xdv && pdfExpected === artifactSha256.main_pdf) {
    return 'MATCH';
  }
  return 'MISMATCH';
}

function buildResolverOutcomeEntryV0(request, resolution) {
  return {
    kind: request.kind,
    format: request.format,
    name: request.name,
    variant: request.variant,
    hint_type: request.hint_type ?? null,
    stable_id: resolution.stable_id,
    sha256: resolution.sha256,
    cache_hit: resolution.cache_hit,
  };
}

function buildResolverMissingEntryV0(request, resolution) {
  return {
    kind: request.kind,
    format: request.format,
    name: request.name,
    variant: request.variant,
    hint_type: request.hint_type ?? null,
    cache_hit: resolution.cache_hit,
  };
}

async function resolveRequestsWithResolverV0(resolver, requests) {
  const resolvedResources = [];
  const missingResources = [];
  for (const request of requests) {
    const resolution = await resolver.resolve({
      kind: request.kind,
      format: request.format,
      name: request.name,
      variant: request.variant,
      resolver_id: resolver.resolverId,
    });
    if (resolution.tag === 'Found') {
      resolvedResources.push(buildResolverOutcomeEntryV0(request, resolution));
      continue;
    }
    missingResources.push(buildResolverMissingEntryV0(request, resolution));
  }
  return {
    resolvedResources,
    missingResources,
  };
}

function normalizeStoreRequestFromEntryV0(entry) {
  const kind = typeof entry?.kind === 'string' ? entry.kind : '';
  const format = typeof entry?.format === 'string' ? entry.format : '';
  const name = typeof entry?.name === 'string' ? entry.name : '';
  const variant = typeof entry?.variant === 'string' ? entry.variant : '';
  const safeVariant = variant === '' || isSafeResolverTokenV0(variant);
  if (!isSafeResolverTokenV0(kind) || !isSafeResolverTokenV0(format) || !isSafeResolverTokenV0(name) || !safeVariant) {
    return null;
  }
  return { kind, format, name, variant };
}

async function loadStoreRequestsV0(storeDir) {
  const indexPath = path.join(storeDir, 'index.json');
  const indexBytes = await readFile(indexPath).catch(() => null);
  if (!indexBytes) {
    return [];
  }
  let parsed;
  try {
    parsed = JSON.parse(indexBytes.toString('utf8'));
  } catch {
    return [];
  }
  const entries = Array.isArray(parsed?.entries) ? parsed.entries : [];
  const requestsByKey = new Map();
  for (const entry of entries) {
    const normalized = normalizeStoreRequestFromEntryV0(entry);
    if (!normalized) {
      continue;
    }
    requestsByKey.set(resolverRequestKeyV0(normalized), normalized);
  }
  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
}

function mergeStoreRequestsV0(existingRequests, missingRequests) {
  const requestsByKey = new Map();
  for (const request of existingRequests) {
    requestsByKey.set(resolverRequestKeyV0(request), request);
  }
  for (const request of missingRequests) {
    const normalized = normalizeStoreRequestFromEntryV0(request);
    if (!normalized) {
      continue;
    }
    requestsByKey.set(resolverRequestKeyV0(normalized), normalized);
  }
  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
}


export {
  resolverRequestKeyV0,
  collectResolverRequestsFromResourceHintsV0,
  collectResolverRequestsFromTypedArtifactsV0,
  computeBaselineMatchV0,
  resolveRequestsWithResolverV0,
  normalizeStoreRequestFromEntryV0,
  loadStoreRequestsV0,
  mergeStoreRequestsV0,
};
