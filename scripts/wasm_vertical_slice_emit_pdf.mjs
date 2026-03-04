import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'out'));
  const fixturePath = path.join(rootDir, 'scripts', 'wasm_smoke_js', 'fixtures', 'ok_demo_capabilities_v0.tex');
  const fixtureBytes = await readFile(fixturePath);

  const ctx = await createCtx(rootDir);
  const mem = createMemHelpers(ctx);
  const helpers = createAssertHelpers(ctx, mem);
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    readCompileReportJson,
    readMainXdvArtifactBytes,
    readMainPdfArtifactBytes,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset failed');
  }
  if (addMountedFile('main.tex', fixtureBytes, 'vertical_slice_pdf_main') !== 0) {
    throw new Error('mount_add_file(main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(vertical_slice_pdf)');
  const report = readCompileReportJson();
  if (report.status !== 'OK') {
    throw new Error(`compile_main report.status expected OK, got ${report.status}`);
  }

  expectOk(ctx.renderMainPdf(), 'render_main_pdf_v0(vertical_slice_pdf)');

  const logBytes = readCompileLogBytes();
  const xdvBytes = readMainXdvArtifactBytes('vertical_slice_pdf(xdv)');
  const pdfBytes = readMainPdfArtifactBytes('vertical_slice_pdf(pdf)');

  await mkdir(outDir, { recursive: true });
  const xdvPath = path.join(outDir, 'main.xdv');
  const pdfPath = path.join(outDir, 'main.pdf');
  const reportPath = path.join(outDir, 'report.json');
  const logPath = path.join(outDir, 'compile.log.bin');
  const summaryPath = path.join(outDir, 'summary.json');

  await writeFile(xdvPath, xdvBytes);
  await writeFile(pdfPath, pdfBytes);
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`);
  await writeFile(logPath, logBytes);

  const summary = {
    status: report.status,
    xdv_bytes: xdvBytes.length,
    pdf_bytes: pdfBytes.length,
    log_bytes: logBytes.length,
    xdv_sha256: createHash('sha256').update(xdvBytes).digest('hex'),
    pdf_sha256: createHash('sha256').update(pdfBytes).digest('hex'),
  };
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  console.log(`PASS: wasm vertical slice emitted ${xdvPath}`);
  console.log(`PASS: wasm vertical slice emitted ${pdfPath}`);
  console.log(`PASS: deterministic summary ${summaryPath}`);
  console.log(`PASS: xdv_sha256 ${summary.xdv_sha256}`);
  console.log(`PASS: pdf_sha256 ${summary.pdf_sha256}`);
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: wasm vertical slice emit pdf: ${message}`);
  process.exit(1);
});

