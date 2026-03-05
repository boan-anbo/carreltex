import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDirDefaultV0 = path.resolve(__dirname, '..', '..', '..');

function sha256HexV0(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function isSha256HexV0(value) {
  return typeof value === 'string' && /^[0-9a-f]{64}$/.test(value);
}

function isSafeCaseIdV0(caseId) {
  return typeof caseId === 'string'
    && caseId.length > 0
    && !caseId.includes('/')
    && !caseId.includes('\\')
    && !caseId.includes('..');
}

function sortCasesV0(cases) {
  return [...cases].sort((left, right) => left.case_id.localeCompare(right.case_id));
}

function normalizeCaseArtifactV0(caseId, caseArtifact) {
  if (!caseArtifact || typeof caseArtifact !== 'object') {
    throw new Error(`report.case_artifact_sha256 missing for case '${caseId}'`);
  }
  const xdvSha = caseArtifact.main_xdv;
  const pdfSha = caseArtifact.main_pdf;
  if (!isSha256HexV0(xdvSha) || !isSha256HexV0(pdfSha)) {
    throw new Error(`report.case_artifact_sha256 invalid for case '${caseId}'`);
  }
  return {
    main_xdv: xdvSha,
    main_pdf: pdfSha,
  };
}

export async function generateBaselinePackV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? rootDirDefaultV0);
  const galleryOutDir = path.resolve(
    options.galleryOutDir
      ?? path.join(rootDir, 'target', 'wasm_fixture_gallery_v0'),
  );
  const baselineOutDir = path.resolve(
    options.baselineOutDir
      ?? path.join(rootDir, 'target', 'texlive_smoke', 'baselines_v0', 'generated'),
  );

  const reportPath = path.join(galleryOutDir, 'report.json');
  const reportBytes = await readFile(reportPath);
  let report;
  try {
    report = JSON.parse(reportBytes.toString('utf8'));
  } catch {
    throw new Error(`invalid report json: ${reportPath}`);
  }

  const engineRev = report?.engine_rev;
  const configHash = report?.config_hash;
  const sourceDateEpoch = report?.source_date_epoch;
  const typedArtifactsVersion = report?.typed_artifacts_version;
  const typedArtifactShaMap = report?.typed_artifact_sha256;
  const statuses = Array.isArray(report?.statuses) ? report.statuses : [];
  const caseArtifactSha = report?.case_artifact_sha256;

  if (typeof engineRev !== 'string' || engineRev.length !== 40) {
    throw new Error('report.engine_rev must be a 40-char git sha');
  }
  if (!isSha256HexV0(configHash)) {
    throw new Error('report.config_hash must be sha256 hex');
  }
  if (!Number.isInteger(sourceDateEpoch) || sourceDateEpoch <= 0) {
    throw new Error('report.source_date_epoch must be a positive integer');
  }
  if (!Number.isInteger(typedArtifactsVersion) || typedArtifactsVersion <= 0) {
    throw new Error('report.typed_artifacts_version must be a positive integer');
  }
  if (!typedArtifactShaMap || typeof typedArtifactShaMap !== 'object') {
    throw new Error('report.typed_artifact_sha256 must be present');
  }
  const typedArtifactShaEntries = Object.entries(typedArtifactShaMap)
    .filter(([key]) => typeof key === 'string' && key.length > 0)
    .sort(([left], [right]) => left.localeCompare(right));
  if (typedArtifactShaEntries.length === 0) {
    throw new Error('report.typed_artifact_sha256 must contain at least one artifact key');
  }
  for (const [key, value] of typedArtifactShaEntries) {
    if (!isSha256HexV0(value)) {
      throw new Error(`report.typed_artifact_sha256.${key} must be sha256 hex`);
    }
  }
  if (!Array.isArray(statuses) || statuses.length === 0) {
    throw new Error('report.statuses must be a non-empty array');
  }
  if (!caseArtifactSha || typeof caseArtifactSha !== 'object') {
    throw new Error('report.case_artifact_sha256 must be present');
  }

  const normalizedCases = sortCasesV0(statuses.map((status) => {
    const caseId = status?.case_id;
    if (!isSafeCaseIdV0(caseId)) {
      throw new Error(`invalid case id '${String(caseId)}'`);
    }
    return {
      case_id: caseId,
      status: typeof status?.status === 'string' ? status.status : 'FAIL',
      expected_status: typeof status?.expected_status === 'string' ? status.expected_status : 'FAIL',
      artifact_sha256: normalizeCaseArtifactV0(caseId, caseArtifactSha[caseId]),
    };
  }));

  await rm(baselineOutDir, { recursive: true, force: true });
  for (const item of normalizedCases) {
    const caseDir = path.join(baselineOutDir, item.case_id);
    await mkdir(caseDir, { recursive: true });
    await writeFile(path.join(caseDir, 'main.xdv.sha256'), `${item.artifact_sha256.main_xdv}\n`);
    await writeFile(path.join(caseDir, 'main.pdf.sha256'), `${item.artifact_sha256.main_pdf}\n`);
  }

  const indexJson = {
    version: 1,
    source: 'wasm_fixture_gallery_v0',
    engine_rev: engineRev,
    config_hash: configHash,
    typed_artifacts_version: typedArtifactsVersion,
    typed_artifact_sha256: Object.fromEntries(typedArtifactShaEntries),
    source_date_epoch: sourceDateEpoch,
    report_sha256: sha256HexV0(reportBytes),
    case_count: normalizedCases.length,
    cases: normalizedCases,
  };
  const indexBytes = Buffer.from(`${JSON.stringify(indexJson, null, 2)}\n`, 'utf8');
  const indexPath = path.join(baselineOutDir, 'index.json');
  await writeFile(indexPath, indexBytes);

  return {
    baselineOutDir,
    reportPath,
    indexPath,
    indexSha256: sha256HexV0(indexBytes),
    caseCount: normalizedCases.length,
    engineRev,
    configHash,
  };
}

async function runCliV0() {
  const galleryOutArg = process.argv[2];
  if (!galleryOutArg) {
    console.error('usage: node scripts/texlive_smoke/baselines_v0/generate_v0.mjs <gallery_out_dir> [baseline_out_dir]');
    process.exit(2);
  }

  const baselineOutArg = process.argv[3];
  const result = await generateBaselinePackV0({
    galleryOutDir: galleryOutArg,
    baselineOutDir: baselineOutArg,
  });
  console.log(`PASS: baseline pack generated ${result.baselineOutDir}`);
  console.log(`PASS: baseline index sha256 ${result.indexSha256}`);
}

const entryHref = process.argv[1] ? new URL(`file://${path.resolve(process.argv[1])}`).href : '';
if (import.meta.url === entryHref) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: baseline pack generator v0: ${message}`);
    process.exit(1);
  });
}
