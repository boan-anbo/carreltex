import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDirDefaultV0 = path.resolve(__dirname, '..', '..', '..');

function isSafeSegmentV0(name) {
  return typeof name === 'string'
    && name.length > 0
    && !name.includes('/')
    && !name.includes('\\')
    && !name.includes('..');
}

async function readJsonV0(filePath) {
  const bytes = await readFile(filePath);
  try {
    return JSON.parse(bytes.toString('utf8'));
  } catch {
    throw new Error(`invalid json: ${filePath}`);
  }
}

async function collectCandidateDirsV0(rootDir) {
  const entries = await readdir(rootDir, { withFileTypes: true }).catch(() => []);
  const dirs = [];
  for (const entry of entries) {
    if (!entry.isDirectory() || !isSafeSegmentV0(entry.name)) {
      continue;
    }
    dirs.push(path.join(rootDir, entry.name));
  }
  return dirs.sort((left, right) => left.localeCompare(right));
}

export async function selectBaselinePackForReportV0(options = {}) {
  const rootDir = path.resolve(options.rootDir ?? rootDirDefaultV0);
  const reportPath = path.resolve(options.reportPath);
  const baselinesRoot = path.resolve(
    options.baselinesRoot
      ?? path.join(rootDir, 'target', 'texlive_smoke', 'baselines_v0'),
  );

  const report = await readJsonV0(reportPath);
  const engineRev = report?.engine_rev;
  const configHash = report?.config_hash;
  if (typeof engineRev !== 'string' || engineRev.length !== 40) {
    throw new Error(`report missing valid engine_rev: ${reportPath}`);
  }
  if (typeof configHash !== 'string' || !/^[0-9a-f]{64}$/.test(configHash)) {
    throw new Error(`report missing valid config_hash: ${reportPath}`);
  }

  const candidateDirs = await collectCandidateDirsV0(baselinesRoot);
  let selected = '';
  for (const candidateDir of candidateDirs) {
    const indexPath = path.join(candidateDir, 'index.json');
    const index = await readJsonV0(indexPath).catch(() => null);
    if (!index || typeof index !== 'object') {
      continue;
    }
    if (index?.source !== 'wasm_fixture_gallery_v0') {
      continue;
    }
    if (index?.engine_rev !== engineRev || index?.config_hash !== configHash) {
      continue;
    }
    selected = candidateDir;
    break;
  }
  if (!selected) {
    throw new Error(
      `no baseline pack matches engine_rev=${engineRev} config_hash=${configHash} under ${baselinesRoot}`,
    );
  }
  return selected;
}

async function runCliV0() {
  const reportArg = process.argv[2];
  if (!reportArg) {
    console.error('usage: node scripts/texlive_smoke/baselines_v0/select_v0.mjs <gallery_report_json> [baselines_root]');
    process.exit(2);
  }
  const baselinesRoot = process.argv[3];
  const selected = await selectBaselinePackForReportV0({
    reportPath: reportArg,
    baselinesRoot,
  });
  console.log(selected);
}

const entryHref = process.argv[1] ? new URL(`file://${path.resolve(process.argv[1])}`).href : '';
if (import.meta.url === entryHref) {
  runCliV0().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: baseline pack selector v0: ${message}`);
    process.exit(1);
  });
}
