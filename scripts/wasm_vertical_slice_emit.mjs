import { mkdir, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');
const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'out'));
const DVI_FNT_DEF1 = 243;
const DVI_POSTPOST = 249;
const DEFAULT_FONT_SCALE_DESIGN = [0x00, 0x0a, 0x00, 0x00];

function tuneFontDefMetrics(bytes, offset) {
  if (offset + 14 >= bytes.length) {
    return;
  }
  for (let i = 0; i < 4; i += 1) {
    bytes[offset + 6 + i] = DEFAULT_FONT_SCALE_DESIGN[i];
    bytes[offset + 10 + i] = DEFAULT_FONT_SCALE_DESIGN[i];
  }
}

function normalizeXdvForHostPreview(rawBytes) {
  const bytes = new Uint8Array(rawBytes);
  const firstFontDef = bytes.indexOf(DVI_FNT_DEF1);
  if (firstFontDef < 0) {
    throw new Error('main.xdv missing DVI_FNT_DEF1');
  }

  for (let offset = firstFontDef; offset >= 0 && offset < bytes.length; offset = bytes.indexOf(DVI_FNT_DEF1, offset + 1)) {
    tuneFontDefMetrics(bytes, offset);
  }

  const postPostIndex = bytes.indexOf(DVI_POSTPOST);
  if (postPostIndex < 0) {
    throw new Error('main.xdv missing DVI_POSTPOST');
  }

  const fontDefLength = 28;
  const fontDefSliceEnd = firstFontDef + fontDefLength;
  if (fontDefSliceEnd > bytes.length) {
    throw new Error('main.xdv font definition is truncated');
  }
  const fontDefBytes = bytes.slice(firstFontDef, fontDefSliceEnd);
  const withPostambleFont = new Uint8Array(bytes.length + fontDefLength);
  withPostambleFont.set(bytes.slice(0, postPostIndex), 0);
  withPostambleFont.set(fontDefBytes, postPostIndex);
  withPostambleFont.set(bytes.slice(postPostIndex), postPostIndex + fontDefLength);

  return withPostambleFont;
}

async function run() {
  const ctx = await createCtx(rootDir);
  const mem = createMemHelpers(ctx);
  const helpers = createAssertHelpers(ctx, mem);
  const { addMountedFile, expectOk, readCompileLogBytes, readCompileReportJson, readMainXdvArtifactBytes } = helpers;

  const mainTex = new TextEncoder().encode(
    '\\documentclass{article}\n\\begin{document}\nHello, CarrelTeX WASM vertical slice.\n\\end{document}\n',
  );

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset failed');
  }
  if (addMountedFile('main.tex', mainTex, 'vertical_slice_main') !== 0) {
    throw new Error('mount_add_file(main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(vertical_slice)');
  const report = readCompileReportJson();
  if (report.status !== 'OK') {
    throw new Error(`compile_main report.status expected OK, got ${report.status}`);
  }
  const logBytes = readCompileLogBytes();
  const rawXdvBytes = readMainXdvArtifactBytes('compile_main(vertical_slice)');
  const xdvBytes = normalizeXdvForHostPreview(rawXdvBytes);

  await mkdir(outDir, { recursive: true });
  const xdvPath = path.join(outDir, 'main.xdv');
  const rawXdvPath = path.join(outDir, 'main.raw.xdv');
  const reportPath = path.join(outDir, 'report.json');
  const logPath = path.join(outDir, 'compile.log.bin');
  const summaryPath = path.join(outDir, 'summary.json');

  await writeFile(xdvPath, xdvBytes);
  await writeFile(rawXdvPath, rawXdvBytes);
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`);
  await writeFile(logPath, logBytes);

  const summary = {
    status: report.status,
    xdv_bytes: xdvBytes.length,
    xdv_raw_bytes: rawXdvBytes.length,
    log_bytes: logBytes.length,
    xdv_sha256: createHash('sha256').update(xdvBytes).digest('hex'),
    xdv_raw_sha256: createHash('sha256').update(rawXdvBytes).digest('hex'),
  };
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  console.log(`PASS: wasm vertical slice emitted ${xdvPath}`);
  console.log(`PASS: deterministic summary ${summaryPath}`);
  console.log(`PASS: xdv_sha256 ${summary.xdv_sha256}`);
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: wasm vertical slice emit: ${message}`);
  process.exit(1);
});
