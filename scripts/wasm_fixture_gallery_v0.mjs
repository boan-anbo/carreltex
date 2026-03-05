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
const EXPECTED_STATUS_VALUES_V0 = new Set([STATUS_OK_V0, STATUS_NI_V0, STATUS_INVALID_V0, STATUS_FAIL_V0]);
const TYPED_ARTIFACT_KEYS_V0 = ['toc', 'labels', 'bib', 'hyperref', 'pkgopt', 'graphics'];
const TYPED_ARTIFACTS_VERSION_V0 = 1;
const MAX_TOC_ENTRIES_V0 = 256;
const MAX_TOC_TITLE_BYTES_V0 = 256;
const MAX_LABEL_ENTRIES_V0 = 256;
const MAX_LABEL_VALUE_BYTES_V0 = 256;
const MAX_BIB_ENTRIES_V0 = 256;
const MAX_BIB_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_ENTRIES_V0 = 256;
const MAX_PKGOPT_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_OPTIONS_PER_ENTRY_V0 = 64;
const MAX_GRAPHICS_ENTRIES_V0 = 256;
const MAX_GRAPHICS_PATH_BYTES_V0 = 256;
const MAX_RESOURCE_HINT_ENTRIES_V0 = 512;
const MAX_RESOURCE_HINT_VALUE_BYTES_V0 = 256;

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

function expectedVsActualV0(expectedStatus, actualStatus) {
  return expectedStatus === actualStatus ? 'MATCH' : 'MISMATCH';
}

async function loadGalleryManifestV0() {
  const manifestPath = path.join(rootDir, 'scripts', 'wasm_fixture_gallery_v0_manifest.json');
  const bytes = await readFile(manifestPath);
  let parsed;
  try {
    parsed = JSON.parse(bytes.toString('utf8'));
  } catch {
    throw new Error(`invalid gallery manifest json: ${manifestPath}`);
  }
  const casesRaw = Array.isArray(parsed?.cases) ? parsed.cases : [];
  if (casesRaw.length === 0) {
    throw new Error(`gallery manifest has no cases: ${manifestPath}`);
  }

  const byId = new Map();
  for (const raw of casesRaw) {
    const id = raw?.id;
    const tagsRaw = Array.isArray(raw?.tags) ? raw.tags : [];
    const expectedStatus = raw?.expected_status;
    const purpose = raw?.purpose;
    if (typeof id !== 'string' || id.length === 0) {
      throw new Error(`gallery manifest case has invalid id: ${manifestPath}`);
    }
    if (byId.has(id)) {
      throw new Error(`gallery manifest has duplicate case id '${id}': ${manifestPath}`);
    }
    if (!EXPECTED_STATUS_VALUES_V0.has(expectedStatus)) {
      throw new Error(`gallery manifest case '${id}' has invalid expected_status '${expectedStatus}'`);
    }
    if (typeof purpose !== 'string' || purpose.trim() === '') {
      throw new Error(`gallery manifest case '${id}' has invalid purpose`);
    }
    const tags = tagsRaw
      .filter((tag) => typeof tag === 'string' && tag.trim() !== '')
      .map((tag) => tag.trim());
    byId.set(id, {
      tags,
      expected_status: expectedStatus,
      purpose: purpose.trim(),
    });
  }
  return {
    path: manifestPath,
    byId,
  };
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
    {
      id: 'typeset_demo_toc_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_toc_probe_v0.tex',
    },
    {
      id: 'typeset_demo_labels_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_labels_probe_v0.tex',
    },
    {
      id: 'typeset_demo_hyperref_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bib_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bib_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bibstyle_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bibstyle_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bib_resources_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bib_resources_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pkgopt_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pkgopt_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_probe_v0.tex',
    },
    {
      id: 'typeset_demo_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_package_require_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_package_require_probe_v0.tex',
    },
    {
      id: 'typeset_demo_resource_hints_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_resource_hints_probe_v0.tex',
    },
    {
      id: 'typeset_demo_nested_path_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_nested_path_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_opts_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_opts_probe_v0.tex',
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

  const discovered = [...texliveFixtures, ...okFixtures];
  const manifest = await loadGalleryManifestV0();

  const merged = discovered.map((caseSpec) => {
    const metadata = manifest.byId.get(caseSpec.id);
    if (!metadata) {
      throw new Error(`gallery manifest missing discovered case '${caseSpec.id}'`);
    }
    return {
      ...caseSpec,
      tags: metadata.tags,
      expected_status: metadata.expected_status,
      purpose: metadata.purpose,
    };
  });

  for (const manifestId of manifest.byId.keys()) {
    if (!merged.find((item) => item.id === manifestId)) {
      throw new Error(`gallery manifest contains unknown case '${manifestId}'`);
    }
  }

  return {
    cases: merged,
    manifestPath: manifest.path,
  };
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
      tags: item.tags,
      expected_status: item.expected_status,
    })),
  };
  return sha256HexV0(Buffer.from(JSON.stringify(config)));
}

function buildTypedArtifactsPlaceholderV0() {
  const typedArtifacts = {};
  for (const key of TYPED_ARTIFACT_KEYS_V0) {
    typedArtifacts[key] = {
      present: false,
      items: 0,
    };
  }
  return typedArtifacts;
}

async function emitPlaceholderTypedArtifactV0(caseOutDir, artifactName, schemaName) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: schemaName,
    entries: [],
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = `${artifactName}.json`;
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

function isAsciiLetterByteV0(byte) {
  return (byte >= 0x41 && byte <= 0x5a) || (byte >= 0x61 && byte <= 0x7a);
}

function isSafeResolverTokenV0(value) {
  return typeof value === 'string'
    && value.length > 0
    && !value.includes('/')
    && !value.includes('\\')
    && !value.includes('..');
}

function skipSpacesV0(bytes, start) {
  let index = start;
  while (index < bytes.length && (bytes[index] === 0x20 || bytes[index] === 0x09 || bytes[index] === 0x0a || bytes[index] === 0x0d)) {
    index += 1;
  }
  return index;
}

function readBracedGroupV0(bytes, start) {
  let index = skipSpacesV0(bytes, start);
  if (index >= bytes.length || bytes[index] !== 0x7b) {
    return { ok: false, next: start, value: '' };
  }
  index += 1;
  const begin = index;
  let depth = 1;
  while (index < bytes.length) {
    const byte = bytes[index];
    if (byte === 0x7b) {
      depth += 1;
    } else if (byte === 0x7d) {
      depth -= 1;
      if (depth === 0) {
        const value = Buffer.from(bytes.slice(begin, index)).toString('utf8').trim();
        return { ok: true, next: index + 1, value };
      }
    }
    index += 1;
  }
  return { ok: false, next: start, value: '' };
}

function readBracketGroupV0(bytes, start) {
  let index = skipSpacesV0(bytes, start);
  if (index >= bytes.length || bytes[index] !== 0x5b) {
    return { ok: false, next: start, value: '' };
  }
  index += 1;
  const begin = index;
  let depth = 1;
  while (index < bytes.length) {
    const byte = bytes[index];
    if (byte === 0x5b) {
      depth += 1;
    } else if (byte === 0x5d) {
      depth -= 1;
      if (depth === 0) {
        const value = Buffer.from(bytes.slice(begin, index)).toString('utf8').trim();
        return { ok: true, next: index + 1, value };
      }
    }
    index += 1;
  }
  return { ok: false, next: start, value: '' };
}

function extractTocEntriesFromSourceV0(sourceBytes) {
  const commandLevel = new Map([
    ['section', 1],
    ['subsection', 2],
    ['subsubsection', 3],
    ['paragraph', 4],
    ['subparagraph', 5],
  ]);
  const entries = [];

  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');
    const level = commandLevel.get(command);
    if (!level) {
      index = commandIndex;
      continue;
    }

    let next = skipSpacesV0(sourceBytes, commandIndex);
    if (next < sourceBytes.length && sourceBytes[next] === 0x2a) {
      next += 1;
    }
    next = skipSpacesV0(sourceBytes, next);

    const shortTitle = readBracketGroupV0(sourceBytes, next);
    if (shortTitle.ok) {
      next = shortTitle.next;
    }

    const titleGroup = readBracedGroupV0(sourceBytes, next);
    if (!titleGroup.ok) {
      index = commandIndex;
      continue;
    }

    if (titleGroup.value.length > 0) {
      const titleBytes = Buffer.from(titleGroup.value, 'utf8');
      if (titleBytes.length > MAX_TOC_TITLE_BYTES_V0) {
        throw new Error(`toc_v0 title exceeds cap ${MAX_TOC_TITLE_BYTES_V0}`);
      }
      entries.push({
        level,
        title: titleGroup.value,
        source_span: buildSourceSpanV0(sourceBytes, index, titleGroup.next, 'toc_v0'),
      });
      if (entries.length > MAX_TOC_ENTRIES_V0) {
        throw new Error(`toc_v0 entries exceed cap ${MAX_TOC_ENTRIES_V0}`);
      }
    }

    index = titleGroup.next;
  }
  return entries;
}

function splitCommaValuesV0(rawValue) {
  return rawValue
    .split(',')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
}

function ensureDefaultExtensionV0(value, extension) {
  if (value.includes('.')) {
    return value;
  }
  return `${value}.${extension}`;
}

function normalizePathHintTokenV0(rawValue, hintType) {
  if (typeof rawValue !== 'string') {
    return null;
  }
  const trimmed = rawValue.trim();
  if (trimmed.length === 0) {
    return null;
  }
  if (trimmed.startsWith('/') || trimmed.startsWith('\\')) {
    throw new Error(`resource_hints_v0 ${hintType} rejects absolute path '${trimmed}'`);
  }
  if (/^[A-Za-z]:([\\/]|$)/.test(trimmed)) {
    throw new Error(`resource_hints_v0 ${hintType} rejects drive path '${trimmed}'`);
  }
  const normalizedSeparators = trimmed.replace(/\\/g, '/');
  const segments = normalizedSeparators
    .split('/')
    .map((segment) => segment.trim())
    .filter((segment) => segment.length > 0 && segment !== '.');
  if (segments.length === 0) {
    return null;
  }
  if (segments.some((segment) => segment === '..' || segment.includes('..'))) {
    throw new Error(`resource_hints_v0 ${hintType} has unsafe path token '${trimmed}'`);
  }
  if (segments.some((segment) => !/^[A-Za-z0-9._-]+$/.test(segment))) {
    throw new Error(`resource_hints_v0 ${hintType} has unsupported path segment '${trimmed}'`);
  }
  const normalized = segments.join('__');
  if (!isSafeResolverTokenV0(normalized)) {
    throw new Error(`resource_hints_v0 ${hintType} normalized token is unsafe '${normalized}'`);
  }
  return normalized;
}

function parseOptionAssignmentsV0(rawValue) {
  const assignments = new Map();
  for (const item of splitCommaValuesV0(rawValue)) {
    const equalsIndex = item.indexOf('=');
    if (equalsIndex <= 0 || equalsIndex === item.length - 1) {
      continue;
    }
    const key = item.slice(0, equalsIndex).trim().toLowerCase();
    let value = item.slice(equalsIndex + 1).trim();
    while (value.length >= 2 && value.startsWith('{') && value.endsWith('}')) {
      value = value.slice(1, -1).trim();
    }
    if (key.length === 0 || value.length === 0) {
      continue;
    }
    assignments.set(key, value);
  }
  return assignments;
}

function addBibEntryV0(entries, entry) {
  const value = entry.kind === 'cite_key' ? entry.key : entry.value;
  const valueBytes = Buffer.from(value, 'utf8');
  if (valueBytes.length > MAX_BIB_VALUE_BYTES_V0) {
    throw new Error(`bib_v0 value exceeds cap ${MAX_BIB_VALUE_BYTES_V0}`);
  }
  entries.push(entry);
  if (entries.length > MAX_BIB_ENTRIES_V0) {
    throw new Error(`bib_v0 entries exceed cap ${MAX_BIB_ENTRIES_V0}`);
  }
}

function buildSourceSpanV0(sourceBytes, startByte, endByte, artifactName) {
  if (!Number.isInteger(startByte) || !Number.isInteger(endByte)) {
    throw new Error(`${artifactName} source_span must use integer byte offsets`);
  }
  if (startByte < 0 || endByte < 0 || endByte <= startByte) {
    throw new Error(`${artifactName} source_span must satisfy 0 <= start < end`);
  }
  if (endByte > sourceBytes.length) {
    throw new Error(`${artifactName} source_span exceeds source byte length`);
  }
  return {
    start_byte: startByte,
    end_byte: endByte,
  };
}

function addPkgoptEntryV0(entries, entry) {
  const packageBytes = Buffer.from(entry.package, 'utf8');
  if (packageBytes.length > MAX_PKGOPT_VALUE_BYTES_V0) {
    throw new Error(`pkgopt_v0 package exceeds cap ${MAX_PKGOPT_VALUE_BYTES_V0}`);
  }
  if (entry.options.length > MAX_PKGOPT_OPTIONS_PER_ENTRY_V0) {
    throw new Error(`pkgopt_v0 options exceed cap ${MAX_PKGOPT_OPTIONS_PER_ENTRY_V0}`);
  }
  for (const option of entry.options) {
    const optionBytes = Buffer.from(option, 'utf8');
    if (optionBytes.length > MAX_PKGOPT_VALUE_BYTES_V0) {
      throw new Error(`pkgopt_v0 option exceeds cap ${MAX_PKGOPT_VALUE_BYTES_V0}`);
    }
  }
  entries.push(entry);
  if (entries.length > MAX_PKGOPT_ENTRIES_V0) {
    throw new Error(`pkgopt_v0 entries exceed cap ${MAX_PKGOPT_ENTRIES_V0}`);
  }
}

function extractPkgoptEntriesFromSourceV0(sourceBytes) {
  const entries = [];
  const packageCommands = new Set(['usepackage', 'RequirePackage']);
  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');
    if (!packageCommands.has(command)) {
      index = commandIndex;
      continue;
    }

    const optGroup = readBracketGroupV0(sourceBytes, commandIndex);
    if (!optGroup.ok || optGroup.value.length === 0) {
      index = commandIndex;
      continue;
    }

    const pkgGroup = readBracedGroupV0(sourceBytes, optGroup.next);
    if (!pkgGroup.ok || pkgGroup.value.length === 0) {
      index = commandIndex;
      continue;
    }

    const options = splitCommaValuesV0(optGroup.value);
    const packages = splitCommaValuesV0(pkgGroup.value);
    for (const pkgName of packages) {
      addPkgoptEntryV0(entries, {
        command,
        package: pkgName,
        options,
        source_span: buildSourceSpanV0(sourceBytes, index, pkgGroup.next, 'pkgopt_v0'),
      });
    }
    index = pkgGroup.next;
  }
  return entries;
}

function extractGraphicsEntriesFromSourceV0(sourceBytes) {
  const entries = [];
  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');
    if (command !== 'includegraphics') {
      index = commandIndex;
      continue;
    }

    let next = commandIndex;
    const optGroup = readBracketGroupV0(sourceBytes, next);
    if (optGroup.ok) {
      next = optGroup.next;
    }

    const pathGroup = readBracedGroupV0(sourceBytes, next);
    if (!pathGroup.ok || pathGroup.value.length === 0) {
      index = commandIndex;
      continue;
    }
    const pathBytes = Buffer.from(pathGroup.value, 'utf8');
    if (pathBytes.length > MAX_GRAPHICS_PATH_BYTES_V0) {
      throw new Error(`graphics_v0 path exceeds cap ${MAX_GRAPHICS_PATH_BYTES_V0}`);
    }
    entries.push({
      command,
      path: pathGroup.value,
      source_span: buildSourceSpanV0(sourceBytes, index, pathGroup.next, 'graphics_v0'),
    });
    if (entries.length > MAX_GRAPHICS_ENTRIES_V0) {
      throw new Error(`graphics_v0 entries exceed cap ${MAX_GRAPHICS_ENTRIES_V0}`);
    }
    index = pathGroup.next;
  }
  return entries;
}

function addResourceHintEntryV0(entries, sourceBytes, hintType, value, startByte, endByte) {
  const valueBytes = Buffer.from(value, 'utf8');
  if (valueBytes.length > MAX_RESOURCE_HINT_VALUE_BYTES_V0) {
    throw new Error(`resource_hints_v0 value exceeds cap ${MAX_RESOURCE_HINT_VALUE_BYTES_V0}`);
  }
  entries.push({
    kind: 'resource_hint',
    hint_type: hintType,
    value,
    source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'resource_hints_v0'),
  });
  if (entries.length > MAX_RESOURCE_HINT_ENTRIES_V0) {
    throw new Error(`resource_hints_v0 entries exceed cap ${MAX_RESOURCE_HINT_ENTRIES_V0}`);
  }
}

function extractResourceHintEntriesFromSourceV0(sourceBytes) {
  const entries = [];
  const seen = new Set();
  let index = 0;

  const addHintValues = (hintType, values, startByte, endByte, defaultExtension = null) => {
    for (const rawValue of values) {
      if (hintType === 'hyperref_url') {
        const normalizedUrl = typeof rawValue === 'string' ? rawValue.trim() : '';
        if (normalizedUrl.length === 0) {
          continue;
        }
        const dedupeKey = `${hintType}\u0000${normalizedUrl}`;
        if (seen.has(dedupeKey)) {
          continue;
        }
        seen.add(dedupeKey);
        addResourceHintEntryV0(entries, sourceBytes, hintType, normalizedUrl, startByte, endByte);
        continue;
      }
      const normalizedPath = normalizePathHintTokenV0(rawValue, hintType);
      if (!normalizedPath) {
        continue;
      }
      const normalized = defaultExtension ? ensureDefaultExtensionV0(normalizedPath, defaultExtension) : normalizedPath;
      const dedupeKey = `${hintType}\u0000${normalized.toLowerCase()}`;
      if (seen.has(dedupeKey)) {
        continue;
      }
      seen.add(dedupeKey);
      addResourceHintEntryV0(entries, sourceBytes, hintType, normalized, startByte, endByte);
    }
  };

  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');

    if (command === 'input' || command === 'include') {
      const group = readBracedGroupV0(sourceBytes, commandIndex);
      if (!group.ok || group.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues(command === 'input' ? 'tex_input' : 'tex_include', splitCommaValuesV0(group.value), index, group.next, 'tex');
      index = group.next;
      continue;
    }

    if (command === 'usepackage' || command === 'RequirePackage') {
      let next = commandIndex;
      const optionsGroup = readBracketGroupV0(sourceBytes, next);
      if (optionsGroup.ok) {
        next = optionsGroup.next;
      }
      const packageGroup = readBracedGroupV0(sourceBytes, next);
      if (!packageGroup.ok || packageGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('package_file', splitCommaValuesV0(packageGroup.value), index, packageGroup.next, 'sty');
      index = packageGroup.next;
      continue;
    }

    if (command === 'includegraphics') {
      let next = commandIndex;
      const optionsGroup = readBracketGroupV0(sourceBytes, next);
      const graphicsOptions = optionsGroup.ok ? parseOptionAssignmentsV0(optionsGroup.value) : new Map();
      if (optionsGroup.ok) {
        next = optionsGroup.next;
      }
      const graphicsGroup = readBracedGroupV0(sourceBytes, next);
      if (!graphicsGroup.ok || graphicsGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const extRaw = (graphicsOptions.get('ext') ?? graphicsOptions.get('extension') ?? '').trim();
      const extNormalized = /^[a-z0-9]+$/i.test(extRaw.replace(/^\./, '')) ? extRaw.replace(/^\./, '') : '';
      const dirRaw = (graphicsOptions.get('dir') ?? graphicsOptions.get('path') ?? '').trim();
      const dirNormalized = normalizePathHintTokenV0(dirRaw, 'graphics_path');
      const candidates = [];
      for (const value of splitCommaValuesV0(graphicsGroup.value)) {
        const withDir = dirNormalized ? `${dirNormalized}/${value}` : value;
        const withExt = extNormalized.length > 0 ? ensureDefaultExtensionV0(withDir, extNormalized) : withDir;
        candidates.push(withExt);
      }
      addHintValues('graphics_path', candidates, index, graphicsGroup.next);
      index = graphicsGroup.next;
      continue;
    }

    if (command === 'addbibresource' || command === 'bibliography') {
      let next = commandIndex;
      if (command === 'addbibresource') {
        const optionsGroup = readBracketGroupV0(sourceBytes, next);
        if (optionsGroup.ok) {
          next = optionsGroup.next;
        }
      }
      const bibGroup = readBracedGroupV0(sourceBytes, next);
      if (!bibGroup.ok || bibGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('bib_resource', splitCommaValuesV0(bibGroup.value), index, bibGroup.next, 'bib');
      index = bibGroup.next;
      continue;
    }

    if (command === 'bibliographystyle') {
      const styleGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!styleGroup.ok || styleGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('bib_style', splitCommaValuesV0(styleGroup.value), index, styleGroup.next, 'bst');
      index = styleGroup.next;
      continue;
    }

    if (command === 'url') {
      const urlGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!urlGroup.ok || urlGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('hyperref_url', [urlGroup.value], index, urlGroup.next);
      index = urlGroup.next;
      continue;
    }

    if (command === 'href') {
      const urlGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!urlGroup.ok || urlGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const textGroup = readBracedGroupV0(sourceBytes, urlGroup.next);
      const endOffset = textGroup.ok ? textGroup.next : urlGroup.next;
      addHintValues('hyperref_url', [urlGroup.value], index, endOffset);
      index = endOffset;
      continue;
    }

    index = commandIndex;
  }

  return entries;
}

function extractBibEntriesFromSourceV0(sourceBytes) {
  const entries = [];
  const citeCommands = new Set([
    'cite',
    'citet',
    'citep',
    'parencite',
    'textcite',
    'autocite',
    'footcite',
    'citeauthor',
    'citeyear',
    'citeyearpar',
    'nocite',
  ]);
  const resourceCommands = new Set(['addbibresource', 'bibliography', 'bibliographystyle']);

  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');

    if (resourceCommands.has(command)) {
      let next = commandIndex;
      if (command === 'addbibresource') {
        const optGroup = readBracketGroupV0(sourceBytes, next);
        if (optGroup.ok) {
          next = optGroup.next;
        }
      }
      const resourceGroup = readBracedGroupV0(sourceBytes, next);
      if (!resourceGroup.ok) {
        index = commandIndex;
        continue;
      }
      const entryKind = command === 'bibliographystyle' ? 'style_hint' : 'resource_hint';
      for (const value of splitCommaValuesV0(resourceGroup.value)) {
        addBibEntryV0(entries, {
          kind: entryKind,
          command,
          value,
          source_span: buildSourceSpanV0(sourceBytes, index, resourceGroup.next, 'bib_v0'),
        });
      }
      index = resourceGroup.next;
      continue;
    }

    if (citeCommands.has(command)) {
      let next = commandIndex;
      for (let i = 0; i < 2; i += 1) {
        const optGroup = readBracketGroupV0(sourceBytes, next);
        if (!optGroup.ok) {
          break;
        }
        next = optGroup.next;
      }
      const citeGroup = readBracedGroupV0(sourceBytes, next);
      if (!citeGroup.ok) {
        index = commandIndex;
        continue;
      }
      for (const key of splitCommaValuesV0(citeGroup.value)) {
        addBibEntryV0(entries, {
          kind: 'cite_key',
          command,
          key,
          source_span: buildSourceSpanV0(sourceBytes, index, citeGroup.next, 'bib_v0'),
        });
      }
      index = citeGroup.next;
      continue;
    }

    index = commandIndex;
  }
  return entries;
}

function extractHyperrefLinksFromSourceV0(sourceBytes) {
  const links = [];
  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');
    if (command === 'url') {
      const urlGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (urlGroup.ok && urlGroup.value.length > 0) {
        links.push({
          command: 'url',
          target: urlGroup.value,
          source_span: buildSourceSpanV0(sourceBytes, index, urlGroup.next, 'hyperref_v0'),
        });
      }
      index = urlGroup.ok ? urlGroup.next : commandIndex;
      continue;
    }
    if (command === 'href') {
      const urlGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!urlGroup.ok) {
        index = commandIndex;
        continue;
      }
      const textGroup = readBracedGroupV0(sourceBytes, urlGroup.next);
      if (textGroup.ok && urlGroup.value.length > 0) {
        links.push({
          command: 'href',
          target: urlGroup.value,
          source_span: buildSourceSpanV0(sourceBytes, index, textGroup.next, 'hyperref_v0'),
        });
      }
      index = textGroup.ok ? textGroup.next : urlGroup.next;
      continue;
    }
    index = commandIndex;
  }
  return links;
}

function extractLabelEntriesFromSourceV0(sourceBytes) {
  const entries = [];
  let index = 0;
  while (index < sourceBytes.length) {
    if (sourceBytes[index] !== 0x5c) {
      index += 1;
      continue;
    }
    let commandIndex = index + 1;
    while (commandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[commandIndex])) {
      commandIndex += 1;
    }
    if (commandIndex === index + 1) {
      index += 1;
      continue;
    }
    const command = Buffer.from(sourceBytes.slice(index + 1, commandIndex)).toString('ascii');
    if (command !== 'label' && command !== 'ref') {
      index = commandIndex;
      continue;
    }
    const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
    if (!keyGroup.ok) {
      index = commandIndex;
      continue;
    }
    if (keyGroup.value.length > 0) {
      const valueBytes = Buffer.from(keyGroup.value, 'utf8');
      if (valueBytes.length > MAX_LABEL_VALUE_BYTES_V0) {
        throw new Error(`labels_v0 value exceeds cap ${MAX_LABEL_VALUE_BYTES_V0}`);
      }
      entries.push({
        command,
        key: keyGroup.value,
        source_span: buildSourceSpanV0(sourceBytes, index, keyGroup.next, 'labels_v0'),
      });
      if (entries.length > MAX_LABEL_ENTRIES_V0) {
        throw new Error(`labels_v0 entries exceed cap ${MAX_LABEL_ENTRIES_V0}`);
      }
    }
    index = keyGroup.next;
  }
  return entries;
}

async function emitLabelsTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'labels_v0',
    entries: extractLabelEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'labels_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitTocTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'toc_v0',
    entries: extractTocEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'toc_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitHyperrefTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'hyperref_v0',
    entries: extractHyperrefLinksFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'hyperref_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitBibTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'bib_v0',
    entries: extractBibEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'bib_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitPkgoptTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'pkgopt_v0',
    entries: extractPkgoptEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'pkgopt_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitGraphicsTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'graphics_v0',
    entries: extractGraphicsEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'graphics_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitResourceHintsArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: 1,
    schema: 'resource_hints_v0',
    entries: extractResourceHintEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'resource_hints_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitTypedArtifactsV0(caseSpec, caseOutDir, typedArtifacts, fixtureBytes) {
  if (caseSpec.id === 'typeset_demo_toc_probe_v0') {
    typedArtifacts.toc = await emitTocTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_labels_probe_v0') {
    typedArtifacts.labels = await emitLabelsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_bib_probe_v0') {
    typedArtifacts.bib = await emitBibTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_hyperref_probe_v0') {
    typedArtifacts.hyperref = await emitHyperrefTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_pkgopt_probe_v0') {
    typedArtifacts.pkgopt = await emitPkgoptTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_graphics_probe_v0') {
    typedArtifacts.graphics = await emitGraphicsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
}

async function buildResourceHintsRollupV0(outDir, summaries) {
  const entries = [];
  const seen = new Set();
  const sortedSummaries = [...summaries].sort((left, right) => left.case_id.localeCompare(right.case_id));

  for (const summary of sortedSummaries) {
    const caseId = summary.case_id;
    const resourceHints = summary.resource_hints_v0 ?? {};
    const relpath = resourceHints.artifact_relpath;
    if (resourceHints.present !== true || typeof relpath !== 'string' || relpath.length === 0) {
      continue;
    }

    const bytes = await readFile(path.join(outDir, caseId, relpath));
    const payload = JSON.parse(bytes.toString('utf8'));
    if (payload?.version !== 1 || payload?.schema !== 'resource_hints_v0' || !Array.isArray(payload?.entries)) {
      throw new Error(`invalid resource_hints_v0 artifact for case ${caseId}`);
    }

    for (const entry of payload.entries) {
      const hintType = typeof entry?.hint_type === 'string' ? entry.hint_type : '';
      const value = typeof entry?.value === 'string' ? entry.value : '';
      if (!hintType || !value) {
        continue;
      }
      const dedupeKey = `${caseId}\x1f${hintType}\x1f${value}`;
      if (seen.has(dedupeKey)) {
        continue;
      }
      seen.add(dedupeKey);
      entries.push({
        case_id: caseId,
        hint_type: hintType,
        value,
      });
    }
  }

  entries.sort((left, right) => {
    const caseCmp = left.case_id.localeCompare(right.case_id);
    if (caseCmp !== 0) {
      return caseCmp;
    }
    const typeCmp = left.hint_type.localeCompare(right.hint_type);
    if (typeCmp !== 0) {
      return typeCmp;
    }
    return left.value.localeCompare(right.value);
  });

  return {
    version: 1,
    entries,
  };
}

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
    if (hintType === 'tex_input' || hintType === 'tex_include') {
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
      if (typeof entry?.path === 'string' && entry.path.length > 0) {
        addTexmfRequest(entry.path, 'graphic', 'graphics_path');
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
  baselineDir,
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
    tags: caseSpec.tags,
    expected_status: caseSpec.expected_status,
    expected_vs_actual: expectedVsActualV0(caseSpec.expected_status, caseStatus),
    purpose: caseSpec.purpose,
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
    typed_artifacts_version: TYPED_ARTIFACTS_VERSION_V0,
    typed_artifacts: buildTypedArtifactsPlaceholderV0(),
  };
  summary.resource_hints_v0 = await emitResourceHintsArtifactV0(caseOutDir, fixtureBytes);
  await emitTypedArtifactsV0(caseSpec, caseOutDir, summary.typed_artifacts, fixtureBytes);
  const typedArtifactRequests = await collectResolverRequestsFromResourceHintsV0(
    caseSpec,
    caseOutDir,
    summary.resource_hints_v0,
  );
  for (const request of typedArtifactRequests) {
    const resolutionFromHint = await resolver.resolve({
      kind: request.kind,
      format: request.format,
      name: request.name,
      variant: request.variant,
      resolver_id: resolver.resolverId,
    });
    if (resolutionFromHint.tag !== 'Found') {
      continue;
    }
    resolvedResources.push({
      kind: request.kind,
      format: request.format,
      name: request.name,
      variant: request.variant,
      stable_id: resolutionFromHint.stable_id,
      sha256: resolutionFromHint.sha256,
      cache_hit: resolutionFromHint.cache_hit,
    });
  }
  summary.baseline_match = await computeBaselineMatchV0(caseSpec.id, summary.artifact_sha256, baselineDir);
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
  const storeDir = path.resolve(process.env.TEXLIVE_STORE_DIR_V0 ?? path.join(rootDir, 'target', 'texlive_store_v0'));
  const baselineDir = process.env.TEXLIVE_BASELINE_DIR
    ? path.resolve(process.env.TEXLIVE_BASELINE_DIR)
    : '';
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
  const { cases, manifestPath } = await loadFixtureCasesV0();
  const resolver = await createOnDemandResolverV0({
    backend: process.env.TEXLIVE_RESOLVER_BACKEND_V0,
    endpoint: process.env.TEXLIVE_ENDPOINT,
    rootDir,
    storeDir,
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
        baselineDir,
      ),
    );
  }

  const report = {
    engine_rev: engineRev,
    source_date_epoch: sourceDateEpoch,
    resolver_id: resolver.resolverId,
    store_dir: storeDir,
    baseline_dir: baselineDir || null,
    manifest_path: manifestPath,
    config_hash: configHash,
    typed_artifacts_version: TYPED_ARTIFACTS_VERSION_V0,
    case_count: summaries.length,
    resolved_resources_count: summaries.reduce(
      (sum, summary) => sum + (Array.isArray(summary.resolved_resources) ? summary.resolved_resources.length : 0),
      0,
    ),
    case_artifact_sha256: Object.fromEntries(
      summaries.map((summary) => [
        summary.case_id,
        {
          main_xdv: summary.artifact_sha256.main_xdv,
          main_pdf: summary.artifact_sha256.main_pdf,
          typed_artifacts: Object.fromEntries(
            TYPED_ARTIFACT_KEYS_V0.map((key) => [
              key,
              summary.typed_artifacts?.[key]?.artifact_sha256 ?? null,
            ]),
          ),
        },
      ]),
    ),
    typed_artifact_sha256: Object.fromEntries(
      TYPED_ARTIFACT_KEYS_V0.map((key) => {
        const digestPayload = {
          version: 1,
          schema: `${key}_sha_rollup_v0`,
          entries: summaries
            .map((summary) => ({
              case_id: summary.case_id,
              artifact_sha256: summary.typed_artifacts?.[key]?.artifact_sha256 ?? null,
            }))
            .sort((left, right) => left.case_id.localeCompare(right.case_id)),
        };
        return [key, sha256HexV0(Buffer.from(JSON.stringify(digestPayload), 'utf8'))];
      }),
    ),
    resource_hints_v0: await buildResourceHintsRollupV0(outDir, summaries),
    statuses: summaries.map((summary) => ({
      typed_artifacts_version: summary.typed_artifacts_version,
      case_id: summary.case_id,
      tags: summary.tags,
      expected_status: summary.expected_status,
      expected_vs_actual: summary.expected_vs_actual,
      baseline_match: summary.baseline_match,
      typed_artifacts_presence: Object.fromEntries(
        TYPED_ARTIFACT_KEYS_V0.map((key) => [key, summary.typed_artifacts?.[key]?.present === true]),
      ),
      status: summary.status,
      artifact_sha256: summary.artifact_sha256,
    })),
  };
  for (const summary of summaries) {
    if (summary.typed_artifacts_version !== TYPED_ARTIFACTS_VERSION_V0) {
      throw new Error(
        `typed_artifacts_version mismatch for case ${summary.case_id}: expected ${TYPED_ARTIFACTS_VERSION_V0}, got ${summary.typed_artifacts_version}`,
      );
    }
  }
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
