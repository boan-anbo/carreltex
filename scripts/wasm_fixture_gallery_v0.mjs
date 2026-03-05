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
const TYPED_ARTIFACT_KEYS_V0 = ['toc', 'labels', 'bib', 'hyperref', 'pkgopt'];
const MAX_TOC_ENTRIES_V0 = 256;
const MAX_TOC_TITLE_BYTES_V0 = 256;
const MAX_LABEL_ENTRIES_V0 = 256;
const MAX_LABEL_VALUE_BYTES_V0 = 256;
const MAX_BIB_ENTRIES_V0 = 256;
const MAX_BIB_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_ENTRIES_V0 = 256;
const MAX_PKGOPT_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_OPTIONS_PER_ENTRY_V0 = 64;

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
      id: 'typeset_demo_graphics_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pkgopt_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pkgopt_probe_v0.tex',
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
    version: 1,
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
        source_span: {
          start_byte: index,
          end_byte: titleGroup.next,
        },
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
      });
    }
    index = pkgGroup.next;
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
  const resourceCommands = new Set(['addbibresource', 'bibliography']);

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
      for (const value of splitCommaValuesV0(resourceGroup.value)) {
        addBibEntryV0(entries, { kind: 'resource_hint', command, value });
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
        addBibEntryV0(entries, { kind: 'cite_key', command, key });
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
        links.push({ command: 'url', target: urlGroup.value });
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
        links.push({ command: 'href', target: urlGroup.value });
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
    version: 1,
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
    version: 1,
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
    version: 1,
    schema: 'hyperref_v0',
    links: extractHyperrefLinksFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'hyperref_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.links.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitBibTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: 1,
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
    version: 1,
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
    typed_artifacts: buildTypedArtifactsPlaceholderV0(),
  };
  await emitTypedArtifactsV0(caseSpec, caseOutDir, summary.typed_artifacts, fixtureBytes);
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
    statuses: summaries.map((summary) => ({
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
