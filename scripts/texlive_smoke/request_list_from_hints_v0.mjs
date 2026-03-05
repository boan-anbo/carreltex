import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDirDefaultV0 = path.resolve(__dirname, '..', '..');

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

function normalizeVariantFromCaseIdV0(caseId) {
  if (typeof caseId !== 'string' || caseId.length === 0) {
    return 'typeset';
  }
  return caseId.startsWith('ok_') ? 'ok' : 'typeset';
}

function normalizeTexmfNameV0(rawValue, hintType, caseId) {
  if (typeof rawValue !== 'string' || rawValue.trim() === '') {
    throw new Error(`resource hint ${hintType} in case ${caseId} must be non-empty`);
  }
  const value = rawValue.trim();
  if (!isSafeTokenV0(value)) {
    throw new Error(`resource hint ${hintType} in case ${caseId} has unsafe token '${value}'`);
  }
  return value;
}

function inferTexmfFormatV0(name, fallback) {
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

function requestKeyV0(request) {
  return `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
}

function sortRequestsV0(requests) {
  return [...requests].sort((left, right) => requestKeyV0(left).localeCompare(requestKeyV0(right)));
}

function parseFontconfigHintV0(value, caseId) {
  const prefix = 'fontconfig:';
  if (!value.startsWith(prefix)) {
    return null;
  }
  const payload = value.slice(prefix.length);
  const firstColon = payload.indexOf(':');
  if (firstColon <= 0 || firstColon === payload.length - 1) {
    throw new Error(`resource hint hyperref_url in case ${caseId} has invalid fontconfig token`);
  }
  const variant = payload.slice(0, firstColon).trim();
  const name = payload.slice(firstColon + 1).trim();
  if (!isSafeTokenV0(variant) || !isSafeTokenV0(name)) {
    throw new Error(`resource hint hyperref_url in case ${caseId} has unsafe fontconfig token`);
  }
  return {
    kind: 'fontconfig',
    format: 'name',
    name,
    variant,
  };
}

export async function buildRequestListFromHintsV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? rootDirDefaultV0);
  const reportPath = path.resolve(
    options.reportPath ?? path.join(rootDir, 'target', 'wasm_fixture_gallery_v0', 'report.json'),
  );
  const outputPath = path.resolve(
    options.outputPath ?? path.join(rootDir, 'target', 'wasm_fixture_gallery_v0', 'request_list_from_hints_v0.json'),
  );

  const reportBytes = await readFile(reportPath);
  let report;
  try {
    report = JSON.parse(reportBytes.toString('utf8'));
  } catch {
    throw new Error(`invalid report json: ${reportPath}`);
  }

  if (report?.typed_artifacts_version !== 1) {
    throw new Error(`report typed_artifacts_version must be 1, got ${String(report?.typed_artifacts_version)}`);
  }
  const resourceHints = report?.resource_hints_v0;
  if (!resourceHints || typeof resourceHints !== 'object' || resourceHints.version !== 1) {
    throw new Error('report.resource_hints_v0 must exist with version=1');
  }
  if (!Array.isArray(resourceHints.entries)) {
    throw new Error('report.resource_hints_v0.entries must be an array');
  }

  const requestsByKey = new Map();
  let skippedExternalHyperrefCount = 0;

  for (const [index, entry] of resourceHints.entries.entries()) {
    const caseId = typeof entry?.case_id === 'string' ? entry.case_id : '';
    const hintType = typeof entry?.hint_type === 'string' ? entry.hint_type : '';
    const value = typeof entry?.value === 'string' ? entry.value.trim() : '';
    if (!caseId || !hintType || !value) {
      throw new Error(`resource hint at index ${index} is invalid`);
    }
    const variant = normalizeVariantFromCaseIdV0(caseId);

    if (hintType === 'graphics_path') {
      const name = normalizeTexmfNameV0(value, hintType, caseId);
      const request = {
        kind: 'texmf',
        format: inferTexmfFormatV0(name, 'graphic'),
        name,
        variant,
      };
      requestsByKey.set(requestKeyV0(request), request);
      continue;
    }

    if (hintType === 'bib_resource') {
      const name = normalizeTexmfNameV0(value, hintType, caseId);
      const request = {
        kind: 'texmf',
        format: inferTexmfFormatV0(name, 'bib'),
        name,
        variant,
      };
      requestsByKey.set(requestKeyV0(request), request);
      continue;
    }

    if (hintType === 'hyperref_url') {
      const fontconfigRequest = parseFontconfigHintV0(value, caseId);
      if (fontconfigRequest) {
        requestsByKey.set(requestKeyV0(fontconfigRequest), fontconfigRequest);
        continue;
      }
      let parsedUrl = null;
      try {
        parsedUrl = new URL(value);
      } catch {
        throw new Error(`resource hint hyperref_url in case ${caseId} must be URL or fontconfig token`);
      }
      if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {
        skippedExternalHyperrefCount += 1;
        continue;
      }
      throw new Error(`resource hint hyperref_url in case ${caseId} has unsupported scheme ${parsedUrl.protocol}`);
    }
  }

  const requests = sortRequestsV0(Array.from(requestsByKey.values()));
  const output = {
    version: 1,
    source_report_sha256: sha256HexV0(reportBytes),
    engine_rev: report?.engine_rev ?? '',
    config_hash: report?.config_hash ?? '',
    typed_artifacts_version: report?.typed_artifacts_version,
    resource_hints_version: resourceHints.version,
    request_count: requests.length,
    skipped_external_hyperref_count: skippedExternalHyperrefCount,
    requests,
  };

  const outputBytes = Buffer.from(`${JSON.stringify(output, null, 2)}\n`, 'utf8');
  await writeFile(outputPath, outputBytes);

  return {
    reportPath,
    outputPath,
    requestCount: requests.length,
    outputSha256: sha256HexV0(outputBytes),
  };
}

async function runCliV0() {
  const reportPathArg = process.argv[2];
  if (!reportPathArg) {
    throw new Error('usage: node scripts/texlive_smoke/request_list_from_hints_v0.mjs <report.json> [output.json]');
  }
  const outputPathArg = process.argv[3];
  const result = await buildRequestListFromHintsV0({
    reportPath: reportPathArg,
    outputPath: outputPathArg,
  });
  console.log(`PASS: request list from hints ${result.outputPath}`);
  console.log(`PASS: request_count ${result.requestCount}`);
  console.log(`PASS: request_list_sha256 ${result.outputSha256}`);
}

if (import.meta.url === new URL(process.argv[1], 'file://').href) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: request list from hints v0: ${message}`);
    process.exit(1);
  });
}
