import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');
const MAX_SEGMENT_TM_GAP_PT_V0 = 24.0;
const PAGE_WIDTH_PT_V0 = 612.0;
const PAGE_HEIGHT_PT_V0 = 792.0;
const MARGIN_PT_V0 = 72.0;

function parsePdfObjectsV0(pdfBytes) {
  const text = Buffer.from(pdfBytes).toString('utf8');
  const objects = [];
  const objectPattern = /(\d+)\s+0\s+obj\n([\s\S]*?)\nendobj\n/g;
  let match;
  while ((match = objectPattern.exec(text)) !== null) {
    objects.push({ id: Number.parseInt(match[1], 10), body: match[2] });
  }
  return objects;
}

function parsePdfRefIdsV0(body, key) {
  const marker = `${key} [`;
  const start = body.indexOf(marker);
  if (start < 0) return [];
  const valuesStart = start + marker.length;
  const valuesEnd = body.indexOf(']', valuesStart);
  if (valuesEnd < 0) return [];
  const fields = body.slice(valuesStart, valuesEnd).trim().split(/\s+/).filter(Boolean);
  const ids = [];
  for (let index = 0; index + 2 < fields.length; index += 3) {
    if (fields[index + 1] === '0' && fields[index + 2] === 'R') {
      const id = Number.parseInt(fields[index], 10);
      if (Number.isFinite(id)) ids.push(id);
    }
  }
  return ids;
}

function parsePdfAnnotationActionIdV0(body) {
  const match = body.match(/\/A\s+(\d+)\s+0\s+R/);
  if (!match) return null;
  const id = Number.parseInt(match[1], 10);
  return Number.isFinite(id) ? id : null;
}

function parsePdfAnnotationDestPageIdV0(body) {
  const match = body.match(/\/Dest \[(\d+)\s+0\s+R\s+\/(?:XYZ|Fit)\b/);
  if (!match) return null;
  const id = Number.parseInt(match[1], 10);
  return Number.isFinite(id) ? id : null;
}

function parsePdfActionUriV0(body) {
  const match = body.match(/\/URI \(([^)]*)\)/);
  return match ? match[1] : null;
}

function parsePdfAnnotationRectV0(body) {
  const match = body.match(/\/Rect \[([^\]]+)\]/);
  if (!match) return null;
  const values = match[1]
    .trim()
    .split(/\s+/)
    .map((value) => Number.parseFloat(value))
    .filter((value) => Number.isFinite(value));
  if (values.length !== 4) return null;
  return values;
}

function extractPageXObjectIdsV0(objects) {
  const pageObjects = objects.filter((obj) => obj.body.includes('/Type /Page /Parent'));
  return pageObjects.map((obj) => obj.id);
}

function parsePageContentStreamIdV0(pageBody) {
  const match = pageBody.match(/\/Contents\s+(\d+)\s+0\s+R/);
  if (!match) return null;
  const id = Number.parseInt(match[1], 10);
  return Number.isFinite(id) ? id : null;
}

function parseTmYForNeedleV0(streamBody, needle) {
  for (const line of streamBody.split('\n')) {
    if (!line.includes(' Tm ') || !line.includes(needle)) continue;
    const fields = line.trim().split(/\s+/).filter(Boolean);
    for (let index = 0; index + 6 < fields.length; index += 1) {
      const isTm =
        fields[index] === '1' &&
        fields[index + 1] === '0' &&
        fields[index + 2] === '0' &&
        fields[index + 3] === '1' &&
        fields[index + 6] === 'Tm';
      if (!isTm) continue;
      const y = Number.parseFloat(fields[index + 5]);
      if (Number.isFinite(y)) return y;
    }
  }
  return null;
}

function maxTmGapPtV0(pdfBytes) {
  const text = Buffer.from(pdfBytes).toString('utf8');
  const BASE_GLYPH_PT_V0 = 7.2;
  const glyphWidthPtForByteV0 = (byte) => {
    if (
      byte === 0x5b || // [
      byte === 0x5d || // ]
      byte === 0x7b || // {
      byte === 0x7d || // }
      byte === 0x3c || // <
      byte === 0x3e // >
    ) {
      return 0.0;
    }
    if (
      byte === 0x20 || // space
      byte === 0x2e || // .
      byte === 0x2c || // ,
      byte === 0x3b || // ;
      byte === 0x3a || // :
      byte === 0x21 || // !
      byte === 0x3f || // ?
      byte === 0x27 || // '
      byte === 0x22 || // "
      byte === 0x69 || // i
      byte === 0x6c || // l
      byte === 0x49 || // I
      byte === 0x7c // |
    ) {
      return BASE_GLYPH_PT_V0 * 0.5;
    }
    if (
      byte === 0x6d || // m
      byte === 0x77 || // w
      byte === 0x4d || // M
      byte === 0x57 // W
    ) {
      return BASE_GLYPH_PT_V0 * 1.5;
    }
    return BASE_GLYPH_PT_V0;
  };

  const parsePdfStringTokenV0 = (line, startIndex) => {
    if (line[startIndex] !== '(') return null;
    const out = [];
    let index = startIndex + 1;
    while (index < line.length) {
      const ch = line[index];
      if (ch === '\\') {
        index += 1;
        if (index >= line.length) return null;
        out.push(line[index]);
        index += 1;
        continue;
      }
      if (ch === ')') {
        return { text: out.join(''), end: index + 1 };
      }
      out.push(ch);
      index += 1;
    }
    return null;
  };

  const parseTmSegmentsForLineV0 = (line) => {
    const segments = [];
    let index = 0;
    while (index < line.length) {
      const tmStart = line.indexOf('1 0 0 1 ', index);
      if (tmStart < 0) break;
      const tmMatch = line
        .slice(tmStart)
        .match(/^1 0 0 1 ([+-]?\d+(?:\.\d+)?) ([+-]?\d+(?:\.\d+)?) Tm /);
      if (!tmMatch) break;
      const x = Number.parseFloat(tmMatch[1]);
      if (!Number.isFinite(x)) break;
      let cursor = tmStart + tmMatch[0].length;
      const openParen = line.indexOf('(', cursor);
      if (openParen < 0) break;
      const parsed = parsePdfStringTokenV0(line, openParen);
      if (!parsed) break;
      const tjStart = line.indexOf(' Tj', parsed.end);
      if (tjStart < 0) break;
      const fontMatch = line
        .slice(tmStart, parsed.end)
        .match(/\/(F[123])\s+[+-]?\d+(?:\.\d+)?\s+Tf\s+\(/);
      const font = fontMatch ? fontMatch[1] : '';
      segments.push({ x, text: parsed.text, font });
      index = tjStart + 3;
    }
    return segments;
  };

  const segmentAdvancePtV0 = (segmentText) => {
    const bytes = Buffer.from(segmentText, 'utf8');
    let total = 0;
    for (const byte of bytes) {
      total += glyphWidthPtForByteV0(byte);
    }
    return total;
  };

  let maxGapPt = 0;
  for (const line of text.split('\n')) {
    if (!line.includes(' Tm ') || !line.includes(' Tj')) continue;
    const segments = parseTmSegmentsForLineV0(line);
    if (segments.length < 2) continue;
    for (let segIndex = 1; segIndex < segments.length; segIndex += 1) {
      const previous = segments[segIndex - 1];
      const current = segments[segIndex];
      if (previous.text === '-' || /^\d+\.$/.test(previous.text)) {
        continue;
      }
      if (!previous.font || !current.font || previous.font === current.font) {
        continue;
      }
      const expectedCurrentX = previous.x + segmentAdvancePtV0(previous.text);
      const gapPt = current.x - expectedCurrentX;
      if (gapPt > maxGapPt) {
        maxGapPt = gapPt;
      }
    }
  }
  return maxGapPt;
}

async function run() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'out'));
  const fixturePath = path.join(
    rootDir,
    'scripts',
    'texlive_smoke',
    'fixtures',
    'typeset_demo_minimal_v0.tex',
  );
  const fixtureBytes = await readFile(fixturePath);

  const ctx = await createCtx(rootDir);
  const mem = createMemHelpers(ctx);
  const helpers = createAssertHelpers(ctx, mem);
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    readCompileReportJson,
    readMainPdfArtifactBytes,
    readMainXdvArtifactBytes,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset failed');
  }
  if (addMountedFile('main.tex', fixtureBytes, 'typeset_minimal_main') !== 0) {
    throw new Error('mount_add_file(main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize failed');
  }

  expectOk(
    ctx.compileMainTypeset(),
    'compile_main_typeset_v0(typeset_main)',
  );
  const report = readCompileReportJson();
  if (report.status !== 'OK') {
    throw new Error(`compile_main_typeset status expected OK, got ${report.status}`);
  }

  expectOk(ctx.renderMainPdf(), 'render_main_pdf_v0(typeset_minimal_main)');

  const logBytes = readCompileLogBytes();
  const xdvBytes = readMainXdvArtifactBytes('typeset_minimal_main(xdv)');
  const pdfBytes = readMainPdfArtifactBytes('typeset_minimal_main(pdf)');

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
  const maxTmGapPt = maxTmGapPtV0(pdfBytes);
  if (maxTmGapPt > MAX_SEGMENT_TM_GAP_PT_V0) {
    throw new Error(
      `segment Tm gap too large: ${maxTmGapPt.toFixed(2)}pt > ${MAX_SEGMENT_TM_GAP_PT_V0.toFixed(2)}pt`,
    );
  }
  summary.max_segment_tm_gap_pt = Number(maxTmGapPt.toFixed(2));
  summary.max_segment_tm_gap_threshold_pt = MAX_SEGMENT_TM_GAP_PT_V0;

  const objects = parsePdfObjectsV0(pdfBytes);
  const objectById = new Map(objects.map((obj) => [obj.id, obj.body]));
  const pageIds = extractPageXObjectIdsV0(objects);
  if (pageIds.length < 2) {
    throw new Error(`expected at least 2 PDF pages, got ${pageIds.length}`);
  }

  const pageOneBody = objectById.get(pageIds[0]);
  const pageTwoBody = objectById.get(pageIds[1]);
  if (!pageOneBody || !pageTwoBody) {
    throw new Error('expected page objects for first two pages');
  }
  const pageOneAnnotIds = parsePdfRefIdsV0(pageOneBody, '/Annots');
  const pageTwoAnnotIds = parsePdfRefIdsV0(pageTwoBody, '/Annots');
  if (pageOneAnnotIds.length === 0 || pageTwoAnnotIds.length === 0) {
    throw new Error('expected non-empty /Annots for page 1 and page 2');
  }

  const pageOneUris = [];
  const pageOneDestPageIds = [];
  for (const annotId of pageOneAnnotIds) {
    const annotBody = objectById.get(annotId);
    if (!annotBody || !annotBody.includes('/Subtype /Link')) {
      throw new Error(`page 1 annotation ${annotId} missing /Subtype /Link`);
    }
    const actionId = parsePdfAnnotationActionIdV0(annotBody);
    if (actionId) {
      const actionBody = objectById.get(actionId);
      if (!actionBody) throw new Error(`page 1 action ${actionId} missing`);
      const uri = parsePdfActionUriV0(actionBody);
      if (!uri) throw new Error(`page 1 action ${actionId} missing URI`);
      pageOneUris.push(uri);
    } else {
      const destPageId = parsePdfAnnotationDestPageIdV0(annotBody);
      if (!destPageId) throw new Error(`page 1 annotation ${annotId} missing URI action and /Dest`);
      pageOneDestPageIds.push(destPageId);
    }
    const rect = parsePdfAnnotationRectV0(annotBody);
    if (!rect) throw new Error(`page 1 annotation ${annotId} missing rect`);
    if (
      !(rect[2] > rect[0]) ||
      !(rect[3] > rect[1]) ||
      rect[0] < 0.0 ||
      rect[1] < 0.0 ||
      rect[2] > PAGE_WIDTH_PT_V0 ||
      rect[3] > PAGE_HEIGHT_PT_V0
    ) {
      throw new Error(`page 1 annotation ${annotId} has invalid rect`);
    }
  }

  const pageTwoUris = [];
  const pageTwoDestPageIds = [];
  for (const annotId of pageTwoAnnotIds) {
    const annotBody = objectById.get(annotId);
    if (!annotBody || !annotBody.includes('/Subtype /Link')) {
      throw new Error(`page 2 annotation ${annotId} missing /Subtype /Link`);
    }
    const actionId = parsePdfAnnotationActionIdV0(annotBody);
    if (actionId) {
      const actionBody = objectById.get(actionId);
      if (!actionBody) throw new Error(`page 2 action ${actionId} missing`);
      const uri = parsePdfActionUriV0(actionBody);
      if (!uri) throw new Error(`page 2 action ${actionId} missing URI`);
      pageTwoUris.push(uri);
    } else {
      const destPageId = parsePdfAnnotationDestPageIdV0(annotBody);
      if (!destPageId) throw new Error(`page 2 annotation ${annotId} missing URI action and /Dest`);
      pageTwoDestPageIds.push(destPageId);
    }
    const rect = parsePdfAnnotationRectV0(annotBody);
    if (!rect) throw new Error(`page 2 annotation ${annotId} missing rect`);
    if (
      !(rect[2] > rect[0]) ||
      !(rect[3] > rect[1]) ||
      rect[0] < 0.0 ||
      rect[1] < 0.0 ||
      rect[2] > PAGE_WIDTH_PT_V0 ||
      rect[3] > PAGE_HEIGHT_PT_V0
    ) {
      throw new Error(`page 2 annotation ${annotId} has invalid rect`);
    }
  }
  if (pageOneUris.length === 0 || pageTwoUris.length === 0) {
    throw new Error('expected at least one external URI annotation on page 1 and page 2');
  }
  if (!pageOneUris.every((uri) => uri === 'https://example.com/page1')) {
    throw new Error(`expected page 1 URIs to be page1 links, got ${JSON.stringify(pageOneUris)}`);
  }
  if (!pageTwoUris.every((uri) => uri === 'https://example.com/page2')) {
    throw new Error(`expected page 2 URIs to be page2 links, got ${JSON.stringify(pageTwoUris)}`);
  }
  if (pageOneDestPageIds.some((id) => id < pageIds[0] || id > pageIds[pageIds.length - 1])) {
    throw new Error(`page 1 internal link destination out of bounds: ${JSON.stringify(pageOneDestPageIds)}`);
  }
  if (pageTwoDestPageIds.some((id) => id < pageIds[0] || id > pageIds[pageIds.length - 1])) {
    throw new Error(`page 2 internal link destination out of bounds: ${JSON.stringify(pageTwoDestPageIds)}`);
  }

  const streamOneId = parsePageContentStreamIdV0(pageOneBody);
  const streamTwoId = parsePageContentStreamIdV0(pageTwoBody);
  if (!streamOneId || !streamTwoId) {
    throw new Error('expected content streams for first two pages');
  }
  const streamOneBody = objectById.get(streamOneId);
  const streamTwoBody = objectById.get(streamTwoId);
  if (!streamOneBody || !streamTwoBody) {
    throw new Error('missing content stream object bodies');
  }
  const footnoteOneY = parseTmYForNeedleV0(streamOneBody, '(1 First demo footnote text');
  const footnoteTwoY = parseTmYForNeedleV0(streamTwoBody, '(2 Second demo footnote text');
  if (footnoteOneY == null || footnoteTwoY == null) {
    throw new Error('expected footnote lines on both page 1 and page 2');
  }
  if (
    !(footnoteOneY >= MARGIN_PT_V0 && footnoteOneY <= 140.0) ||
    !(footnoteTwoY >= MARGIN_PT_V0 && footnoteTwoY <= 140.0)
  ) {
    throw new Error(
      `expected page footnotes near bottom margin, got y1=${footnoteOneY}, y2=${footnoteTwoY}`,
    );
  }

  summary.pdf_page_count = pageIds.length;
  summary.page_one_annotation_count = pageOneAnnotIds.length;
  summary.page_two_annotation_count = pageTwoAnnotIds.length;
  summary.page_one_footnote_y_pt = Number(footnoteOneY.toFixed(2));
  summary.page_two_footnote_y_pt = Number(footnoteTwoY.toFixed(2));

  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);

  console.log(`PASS: wasm typeset minimal emitted ${xdvPath}`);
  console.log(`PASS: wasm typeset minimal emitted ${pdfPath}`);
  console.log(`PASS: deterministic summary ${summaryPath}`);
  console.log(`PASS: xdv_sha256 ${summary.xdv_sha256}`);
  console.log(`PASS: pdf_sha256 ${summary.pdf_sha256}`);
  console.log(
    `PASS: max_segment_tm_gap_pt ${summary.max_segment_tm_gap_pt.toFixed(2)} <= ${MAX_SEGMENT_TM_GAP_PT_V0.toFixed(2)}`,
  );
  console.log(
    `PASS: multipage+annots+footnotes pages=${summary.pdf_page_count} p1_annots=${summary.page_one_annotation_count} p2_annots=${summary.page_two_annotation_count}`,
  );
}

run().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: wasm typeset minimal emit pdf: ${message}`);
  process.exit(1);
});
