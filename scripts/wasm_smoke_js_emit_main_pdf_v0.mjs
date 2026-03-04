import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { mkdir, readFile, writeFile } from 'node:fs/promises';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';

// WASM smoke tool (preview only)
//
// Purpose:
// - Prove end-to-end artifact plumbing for the browser/WASM lane by producing:
//   1) an intermediate `main.xdv` artifact (ABI name; currently a strict DVI-v2-ish subset), and
//   2) a minimal, viewable `main.pdf` derived from that artifact.
//
// Non-goals:
// - This is NOT the v2+ "real" XDV->PDF converter (xdvipdfmx-equivalent); it only understands a tiny
//   opcode subset and emits a debug PDF for quick inspection.
//
const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const outDir = process.argv[2]
  ? path.resolve(process.cwd(), process.argv[2])
  : path.join(rootDir, 'target', 'wasm_pdf_smoke');

const inputTexPath = process.argv[3] ? path.resolve(process.cwd(), process.argv[3]) : null;

const ctx = await createCtx(rootDir);
const mem = createMemHelpers(ctx);
const helpers = createAssertHelpers(ctx, mem);

const { addMountedFile, expectOk, readCompileReportJson, readCompileLogBytes, readMainXdvArtifactBytes } = helpers;

const defaultMainTex = '\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n';
const mainBytes = inputTexPath ? new Uint8Array(await readFile(inputTexPath)) : new TextEncoder().encode(defaultMainTex);

expectOk(ctx.mountReset(), 'mount_reset');
expectOk(addMountedFile('main.tex', mainBytes, 'main_tex'), 'mount_add_file(main.tex)');
expectOk(ctx.mountFinalize(), 'mount_finalize');

expectOk(ctx.compileRequestReset(), 'compile_request_reset_v0');
const requestEntrypoint = new TextEncoder().encode('main.tex');
const setEntrypointCode = mem.callWithBytes(requestEntrypoint, 'compile_request_entrypoint', (ptr, len) =>
  ctx.compileRequestSetEntrypoint(ptr, len),
);
expectOk(setEntrypointCode, 'compile_request_set_entrypoint_v0(main.tex)');
expectOk(ctx.compileRequestSetEpoch(1700000000n), 'compile_request_set_source_date_epoch_v0');
expectOk(ctx.compileRequestSetMaxLogBytes(4096), 'compile_request_set_max_log_bytes_v0');
expectOk(ctx.compileRequestSetOkMaxLineGlyphs(256), 'compile_request_set_ok_max_line_glyphs_v0(256)');
expectOk(ctx.compileRequestSetOkMaxLinesPerPage(200), 'compile_request_set_ok_max_lines_per_page_v0(200)');

// Make the emitted DVI v2 easier to view as a PDF:
// - monospaced glyph cell: 1pt (engine default; PDF preview can scale separately)
// - line height: 12pt
expectOk(ctx.compileRequestSetOkGlyphAdvanceSp(65_536), 'compile_request_set_ok_glyph_advance_sp_v0(1pt)');
expectOk(ctx.compileRequestSetOkLineAdvanceSp(786_432), 'compile_request_set_ok_line_advance_sp_v0(12pt)');

const compileStatusCode = ctx.compileRun();
if (compileStatusCode !== 0) {
  const report = readCompileReportJson();
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  throw new Error(
    `compile_run_v0 failed: code=${compileStatusCode} report.status=${report.status} missing_components=${JSON.stringify(
      report.missing_components,
    )} log_bytes=${logBytes.length} log=${JSON.stringify(logText)}`,
  );
}
const report = readCompileReportJson();
if (report.status !== 'OK') {
  throw new Error(`compile_run report.status expected OK, got ${report.status}`);
}

const logBytes = readCompileLogBytes();
if (logBytes.length !== 0) {
  throw new Error(`compile_run expected empty log, got ${logBytes.length} bytes`);
}

const xdvBytes = readMainXdvArtifactBytes('compile_run_v0');

function readI24Be(bytes, index) {
  const raw = (bytes[index] << 16) | (bytes[index + 1] << 8) | bytes[index + 2];
  if ((raw & 0x80_0000) !== 0) return raw | ~0x00ff_ffff;
  return raw;
}

function escapePdfStringByte(byte) {
  if (byte === 0x28) return '\\(';
  if (byte === 0x29) return '\\)';
  if (byte === 0x5c) return '\\\\';
  if (byte === 0x0a) return '\\n';
  if (byte === 0x0d) return '\\r';
  if (byte === 0x09) return '\\t';
  if (byte < 0x20 || byte > 0x7e) return '';
  return String.fromCharCode(byte);
}

function parseDviV2TextPages(bytes) {
  const DVI_PRE = 247;
  const DVI_BOP = 139;
  const DVI_EOP = 140;
  const DVI_POST = 248;
  const DVI_POSTPOST = 249;
  const DVI_FNT_DEF1 = 243;
  const DVI_FNT_NUM_0 = 171;
  const DVI_RIGHT3 = 145;
  const DVI_DOWN3 = 160;
  const DVI_ID_V2 = 2;

  let index = 0;
  const readU8 = () => bytes[index++];
  const readU32Be = () => {
    const value = (bytes[index] << 24) | (bytes[index + 1] << 16) | (bytes[index + 2] << 8) | bytes[index + 3];
    index += 4;
    return value >>> 0;
  };

  if (readU8() !== DVI_PRE) throw new Error('dvi: missing PRE');
  if (readU8() !== DVI_ID_V2) throw new Error('dvi: expected id=2');
  readU32Be();
  readU32Be();
  readU32Be();
  const commentLen = readU8();
  index += commentLen;

  const pages = [];
  while (bytes[index] === DVI_BOP) {
    readU8();
    index += 11 * 4;

    if (readU8() !== DVI_FNT_DEF1) throw new Error('dvi: missing FNT_DEF1');
    const fontId = readU8();
    if (fontId !== 0) throw new Error(`dvi: expected font id 0, got ${fontId}`);
    index += 12;
    const areaLen = readU8();
    const nameLen = readU8();
    index += areaLen + nameLen;
    if (readU8() !== DVI_FNT_NUM_0) throw new Error('dvi: missing FNT_NUM_0');

    let hSp = 0;
    let vSp = 0;
    const glyphs = [];

    while (true) {
      const op = readU8();
      if (op === DVI_EOP) break;
      if (op === DVI_RIGHT3) {
        const delta = readI24Be(bytes, index);
        index += 3;
        hSp += delta;
        continue;
      }
      if (op === DVI_DOWN3) {
        const delta = readI24Be(bytes, index);
        index += 3;
        vSp += delta;
        continue;
      }
      if (op >= 0x20 && op <= 0x7e) {
        glyphs.push({ byte: op, hSp, vSp });
        continue;
      }
      throw new Error(`dvi: unexpected opcode ${op} at index=${index - 1}`);
    }
    pages.push({ glyphs });
  }

  if (readU8() !== DVI_POST) throw new Error('dvi: missing POST');
  index += 4;
  readU32Be();
  readU32Be();
  readU32Be();
  const maxHSp = readU32Be();
  const maxVSp = readU32Be();
  index += 4;

  if (bytes[index] !== DVI_POSTPOST) throw new Error('dvi: missing POSTPOST');
  return { pages, maxHSp, maxVSp };
}

function buildPdfFromDviV2(parsed) {
  // NOTE: Preview-only renderer.
  //
  // We draw a monospaced, debug-style PDF from a tiny DVI-v2-ish opcode subset. This is not real
  // LaTeX typesetting; it is meant for quick, deterministic inspection only.
  //
  // Because OK layout may use fractional widths for some glyphs (see engine tests), we render each
  // glyph at its DVI position but apply a small collision-avoidance step to prevent overprinting.
  const renderScaleX = 6;
  const renderScaleY = 1;
  const ptPerSpX = (1 / 65536) * renderScaleX;
  const ptPerSpY = (1 / 65536) * renderScaleY;
  const marginPt = 72;
  const fontSizePt = 10;
  const charWidthPt = fontSizePt * 0.6; // Courier is roughly 600/1000 em width

  const pageWidthPt = Math.max(612, parsed.maxHSp * ptPerSpX + marginPt * 2);
  const pageHeightPt = Math.max(792, parsed.maxVSp * ptPerSpY + marginPt * 2 + 2 * fontSizePt);

  const pageCount = parsed.pages.length;
  if (pageCount <= 0) {
    throw new Error('pdf: expected at least one page');
  }

  const catalogId = 1;
  const pagesId = 2;
  const fontId = 3;
  const pageIds = [];
  const contentIds = [];
  let nextId = 4;
  for (let i = 0; i < pageCount; i++) {
    pageIds.push(nextId++);
    contentIds.push(nextId++);
  }

  const objectsById = new Map();
  objectsById.set(catalogId, `<< /Type /Catalog /Pages ${pagesId} 0 R >>`);
  objectsById.set(
    pagesId,
    `<< /Type /Pages /Count ${pageCount} /Kids [${pageIds.map((id) => `${id} 0 R`).join(' ')}] >>`,
  );
  objectsById.set(fontId, `<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>`);

  for (let i = 0; i < pageCount; i++) {
    const page = parsed.pages[i];
    const lastXByVSp = new Map();

    let content = 'BT\n';
    content += `/F1 ${fontSizePt} Tf\n`;
    for (const glyph of page.glyphs) {
      let x = marginPt + glyph.hSp * ptPerSpX;
      const y = pageHeightPt - marginPt - fontSizePt - glyph.vSp * ptPerSpY;
      const s = escapePdfStringByte(glyph.byte);
      if (s.length === 0) continue;

      const lastX = lastXByVSp.get(glyph.vSp);
      if (typeof lastX === 'number' && x <= lastX + charWidthPt * 0.8) {
        x = lastX + charWidthPt;
      }
      lastXByVSp.set(glyph.vSp, x);

      content += `1 0 0 1 ${x.toFixed(3)} ${y.toFixed(3)} Tm (${s}) Tj\n`;
    }
    content += 'ET\n';
    const contentBytes = Buffer.from(content, 'binary');

    objectsById.set(contentIds[i], `<< /Length ${contentBytes.length} >>\nstream\n${content}endstream`);
    objectsById.set(
      pageIds[i],
      `<< /Type /Page /Parent ${pagesId} 0 R /Resources << /Font << /F1 ${fontId} 0 R >> >> /MediaBox [0 0 ${pageWidthPt.toFixed(
        2,
      )} ${pageHeightPt.toFixed(2)}] /Contents ${contentIds[i]} 0 R >>`,
    );
  }

  const offsets = new Array(nextId).fill(0);
  const chunks = [];
  const push = (text) => chunks.push(Buffer.from(text, 'binary'));
  const currentLen = () => chunks.reduce((sum, b) => sum + b.length, 0);

  push('%PDF-1.4\n%\u00FF\u00FF\u00FF\u00FF\n');
  for (let id = 1; id < nextId; id++) {
    const body = objectsById.get(id);
    if (!body) throw new Error(`pdf: missing object id ${id}`);
    offsets[id] = currentLen();
    push(`${id} 0 obj\n${body}\nendobj\n`);
  }

  const xrefOffset = currentLen();
  let xref = `xref\n0 ${nextId}\n0000000000 65535 f \n`;
  for (let id = 1; id < nextId; id++) {
    xref += `${String(offsets[id]).padStart(10, '0')} 00000 n \n`;
  }
  push(xref);
  push(`trailer\n<< /Size ${nextId} /Root ${catalogId} 0 R >>\nstartxref\n${xrefOffset}\n%%EOF\n`);
  return Buffer.concat(chunks);
}

const parsed = parseDviV2TextPages(xdvBytes);
const pdfBytes = buildPdfFromDviV2(parsed);

await mkdir(outDir, { recursive: true });
await writeFile(path.join(outDir, 'main.tex'), mainBytes);
await writeFile(path.join(outDir, 'main.xdv'), xdvBytes);
await writeFile(path.join(outDir, 'main.pdf'), pdfBytes);
await writeFile(path.join(outDir, 'report.json'), JSON.stringify(report, null, 2) + '\n');

console.log(`PASS: wasm emit main.pdf (${pdfBytes.length} bytes) -> ${outDir}`);
