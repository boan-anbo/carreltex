const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const baselineDirA = process.argv[2];
const baselineDirB = process.argv[3];
const outDir = process.argv[4];

const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
const readJson = (p) => JSON.parse(fs.readFileSync(p, 'utf8'));

const indexAPath = path.join(baselineDirA, 'index.json');
const indexBPath = path.join(baselineDirB, 'index.json');
const indexABytes = fs.readFileSync(indexAPath);
const indexBBytes = fs.readFileSync(indexBPath);
const indexASha = sha256(indexABytes);
const indexBSha = sha256(indexBBytes);
if (indexASha !== indexBSha) {
  console.error('FAIL: baseline generator must be deterministic (index.json sha mismatch)');
  process.exit(1);
}

const indexA = readJson(indexAPath);
const indexB = readJson(indexBPath);
const report = readJson(path.join(outDir, 'report.json'));
if (indexA.engine_rev !== indexB.engine_rev || indexA.config_hash !== indexB.config_hash) {
  console.error('FAIL: baseline generator metadata mismatch between reruns');
  process.exit(1);
}
if (typeof indexA.engine_rev !== 'string' || indexA.engine_rev.length !== 40) {
  console.error('FAIL: baseline index missing valid engine_rev pin');
  process.exit(1);
}
if (!/^[0-9a-f]{64}$/.test(indexA.config_hash)) {
  console.error('FAIL: baseline index missing valid config_hash pin');
  process.exit(1);
}
if (indexA.typed_artifacts_version !== 1 || indexB.typed_artifacts_version !== 1) {
  console.error('FAIL: baseline index missing typed_artifacts_version=1');
  process.exit(1);
}
if (!indexA.typed_artifact_sha256 || typeof indexA.typed_artifact_sha256 !== 'object') {
  console.error('FAIL: baseline index missing typed_artifact_sha256 map');
  process.exit(1);
}
if (!indexB.typed_artifact_sha256 || typeof indexB.typed_artifact_sha256 !== 'object') {
  console.error('FAIL: baseline index rerun missing typed_artifact_sha256 map');
  process.exit(1);
}
const typedKeys = ['toc', 'labels', 'bib', 'cite', 'hyperref', 'pkgopt', 'packages', 'graphics', 'input', 'math', 'table'];
for (const key of typedKeys) {
  const valueA = indexA.typed_artifact_sha256[key];
  const valueB = indexB.typed_artifact_sha256[key];
  if (!/^[0-9a-f]{64}$/.test(valueA) || !/^[0-9a-f]{64}$/.test(valueB)) {
    console.error(`FAIL: baseline index typed_artifact_sha256 invalid for key ${key}`);
    process.exit(1);
  }
  if (valueA !== valueB) {
    console.error(`FAIL: baseline index typed_artifact_sha256 mismatch across reruns for key ${key}`);
    process.exit(1);
  }
  if (report?.typed_artifact_sha256?.[key] !== valueA) {
    console.error(`FAIL: baseline index typed_artifact_sha256 mismatch vs report for key ${key}`);
    process.exit(1);
  }
}

const casesA = Array.isArray(indexA.cases) ? indexA.cases : [];
const casesB = Array.isArray(indexB.cases) ? indexB.cases : [];
if (casesA.length === 0 || casesA.length !== casesB.length) {
  console.error('FAIL: baseline index cases missing or length mismatch');
  process.exit(1);
}

for (const caseEntry of casesA) {
  const caseId = caseEntry.case_id;
  if (typeof caseId !== 'string' || caseId.length === 0) {
    console.error('FAIL: baseline case id invalid');
    process.exit(1);
  }
  const matchB = casesB.find((it) => it.case_id === caseId);
  if (!matchB) {
    console.error(`FAIL: baseline rerun missing case ${caseId}`);
    process.exit(1);
  }

  const filePairs = [
    ['main.xdv.sha256', 'main_xdv'],
    ['main.pdf.sha256', 'main_pdf'],
  ];
  for (const [filename, key] of filePairs) {
    const pathA = path.join(baselineDirA, caseId, filename);
    const pathB = path.join(baselineDirB, caseId, filename);
    const valueA = fs.readFileSync(pathA, 'utf8').trim();
    const valueB = fs.readFileSync(pathB, 'utf8').trim();
    if (valueA !== valueB) {
      console.error(`FAIL: baseline rerun mismatch for ${caseId}/${filename}`);
      process.exit(1);
    }
    if (!/^[0-9a-f]{64}$/.test(valueA)) {
      console.error(`FAIL: baseline file ${caseId}/${filename} must contain sha256`);
      process.exit(1);
    }
    if (caseEntry.artifact_sha256?.[key] !== valueA || matchB.artifact_sha256?.[key] !== valueA) {
      console.error(`FAIL: baseline index artifact sha mismatch for ${caseId}.${key}`);
      process.exit(1);
    }
  }
}

console.log(`PASS: baseline generator deterministic index_sha256 ${indexASha}`);
console.log(`PASS: baseline generator pinned engine_rev ${indexA.engine_rev}`);
console.log(`PASS: baseline generator pinned config_hash ${indexA.config_hash}`);
console.log(`PASS: baseline generator typed_artifacts_version ${indexA.typed_artifacts_version}`);
console.log('PASS: baseline generator typed_artifact_sha256 map present');
