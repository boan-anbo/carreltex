import { readdir, readFile, rm, mkdir, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { createCtx } from '../wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from '../wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from '../wasm_smoke_js/assert.mjs';
import { createOnDemandResolverV0 } from '../wasm_smoke_js/ondemand_resolver_v0.mjs';
import { generateTexliveStoreV0 } from '../texlive_store_gen_v0.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..', '..');

const DEFAULT_SOURCE_DATE_EPOCH_V0 = 1_700_000_000;
const DEFAULT_MAX_LOG_BYTES_V0 = 4096;
const STATUS_OK_V0 = 'OK';
const STATUS_NI_V0 = 'NI';
const STATUS_INVALID_V0 = 'INVALID';
const STATUS_FAIL_V0 = 'FAIL';
const STATUS_MISMATCH_V0 = 'MISMATCH';
const EXPECTED_STATUS_VALUES_V0 = new Set([STATUS_OK_V0, STATUS_NI_V0, STATUS_INVALID_V0, STATUS_FAIL_V0]);
const DEFAULT_ONDEMAND_FIXEDPOINT_MAX_ITERS_V1 = 3;
const TYPED_ARTIFACT_KEYS_V0 = ['toc', 'labels', 'refs', 'bib', 'cite', 'bibitems', 'cites', 'hyperref', 'pkgopt', 'graphics', 'input', 'math', 'table'];
const TYPED_ARTIFACTS_VERSION_V0 = 1;
const MAX_TOC_ENTRIES_V0 = 256;
const MAX_TOC_TITLE_BYTES_V0 = 256;
const MAX_LABEL_ENTRIES_V0 = 256;
const MAX_LABEL_VALUE_BYTES_V0 = 256;
const MAX_REF_ENTRIES_V0 = 256;
const MAX_REF_OCCURRENCES_PER_KEY_V0 = 256;
const MAX_BIB_ENTRIES_V0 = 256;
const MAX_BIB_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_ENTRIES_V0 = 256;
const MAX_PKGOPT_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_OPTIONS_PER_ENTRY_V0 = 64;
const MAX_GRAPHICS_ENTRIES_V0 = 256;
const MAX_GRAPHICS_PATH_BYTES_V0 = 256;
const MAX_INPUT_ENTRIES_V1 = 512;
const MAX_INPUT_INCLUDE_DEPTH_V1 = 32;
const MAX_MATH_ENTRIES_V0 = 256;
const MAX_MATH_PAYLOAD_BYTES_V0 = 1024;
const MAX_TABLE_ENTRIES_V0 = 64;
const MAX_TABLE_ROWS_PER_ENTRY_V0 = 64;
const MAX_TABLE_COLS_PER_ENTRY_V0 = 16;
const MAX_RESOURCE_HINT_ENTRIES_V0 = 512;
const MAX_RESOURCE_HINT_VALUE_BYTES_V0 = 256;
const RESOURCE_HINTS_V0_VERSION = 1;
const DELTA_POLICY_V1_SCHEMA = 'wasm_fixture_gallery_delta_policy_v1';
const DELTA_POLICY_V1_VERSION = 1;
const BASELINE_CMP_CLASS_MATCH_V1 = 'MATCH';
const BASELINE_CMP_CLASS_DIFF_OK_V1 = 'DIFF_OK';
const BASELINE_CMP_CLASS_DIFF_SUSPECT_V1 = 'DIFF_SUSPECT';
const BASELINE_CMP_CLASS_MISSING_V1 = 'MISSING_BASELINE';
const BASELINE_CMP_CLASS_SKIP_V1 = 'SKIP';
const BASELINE_CMP_CLASS_ALLOWLIST_V1 = new Set([
  BASELINE_CMP_CLASS_MATCH_V1,
  BASELINE_CMP_CLASS_DIFF_OK_V1,
  BASELINE_CMP_CLASS_DIFF_SUSPECT_V1,
  BASELINE_CMP_CLASS_MISSING_V1,
  BASELINE_CMP_CLASS_SKIP_V1,
]);
const RESOURCE_HINT_TYPE_ALLOWLIST_V0 = new Set([
  'tex_input',
  'tex_include',
  'tex_includeonly',
  'package_file',
  'class_file',
  'graphics_path',
  'bib_resource',
  'bib_style',
  'hyperref_url',
]);

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

function parseBoolEnvV0(value) {
  if (value === undefined || value === null) {
    return false;
  }
  const normalized = `${value}`.trim().toLowerCase();
  return normalized === '1' || normalized === 'true' || normalized === 'yes' || normalized === 'on';
}

function countRegexMatchesV1(text, regex) {
  let count = 0;
  let match;
  while ((match = regex.exec(text)) !== null) {
    count += 1;
  }
  return count;
}

function decodePdfStringBytesV1(encoded) {
  const out = [];
  for (let index = 0; index < encoded.length; index += 1) {
    const byte = encoded.charCodeAt(index) & 0xff;
    if (byte !== 0x5c) {
      out.push(byte);
      continue;
    }
    if (index + 1 >= encoded.length) {
      out.push(0x5c);
      continue;
    }
    const next = encoded.charCodeAt(index + 1) & 0xff;
    index += 1;
    if (next === 0x5c || next === 0x28 || next === 0x29) {
      out.push(next);
      continue;
    }
    if (next === 0x6e) {
      out.push(0x0a);
      continue;
    }
    if (next === 0x72) {
      out.push(0x0d);
      continue;
    }
    if (next === 0x74) {
      out.push(0x09);
      continue;
    }
    if (next === 0x62) {
      out.push(0x08);
      continue;
    }
    if (next === 0x66) {
      out.push(0x0c);
      continue;
    }
    if (next >= 0x30 && next <= 0x37) {
      let oct = String.fromCharCode(next);
      for (let i = 0; i < 2 && index + 1 < encoded.length; i += 1) {
        const octNext = encoded.charCodeAt(index + 1) & 0xff;
        if (octNext < 0x30 || octNext > 0x37) {
          break;
        }
        index += 1;
        oct += String.fromCharCode(octNext);
      }
      out.push(Number.parseInt(oct, 8) & 0xff);
      continue;
    }
    out.push(next);
  }
  return Uint8Array.from(out);
}

function extractPdfTextRunsV1(pdfText) {
  const runs = [];
  const regex = /\(((?:\\.|[^\\()])*)\)\s*Tj/g;
  let match;
  while ((match = regex.exec(pdfText)) !== null) {
    const encoded = match[1];
    const bytes = decodePdfStringBytesV1(encoded);
    runs.push({
      encoded,
      bytes,
      text: Buffer.from(bytes).toString('utf8'),
    });
  }
  return runs;
}

function computePdfMetricsV1(pdfBytes) {
  const pdfText = Buffer.from(pdfBytes).toString('latin1');
  const tmRegex = /(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s+Tm/g;
  const textRunRegex = /\(((?:\\.|[^\\()])*)\)\s*Tj/g;
  const lineGlyphCounts = [];
  const lineYValues = [];
  let totalGlyphs = 0;
  let footnoteMarkerCount = 0;
  let currentLineGlyphs = null;
  let tmMatch;
  let textRunMatch;
  tmRegex.lastIndex = 0;
  textRunRegex.lastIndex = 0;
  while (true) {
    const tmIndex = tmRegex.lastIndex;
    const tjIndex = textRunRegex.lastIndex;
    tmMatch = tmRegex.exec(pdfText);
    textRunMatch = textRunRegex.exec(pdfText);
    if (!tmMatch && !textRunMatch) {
      break;
    }
    const tmPos = tmMatch ? tmMatch.index : Number.POSITIVE_INFINITY;
    const tjPos = textRunMatch ? textRunMatch.index : Number.POSITIVE_INFINITY;
    if (tmPos <= tjPos) {
      if (currentLineGlyphs !== null) {
        lineGlyphCounts.push(currentLineGlyphs);
      }
      const y = Number.parseFloat(tmMatch[2]);
      if (Number.isFinite(y)) {
        lineYValues.push(y);
      }
      currentLineGlyphs = 0;
      textRunRegex.lastIndex = tjIndex;
      continue;
    }
    const runBytes = decodePdfStringBytesV1(textRunMatch[1]);
    const runText = Buffer.from(runBytes).toString('utf8');
    totalGlyphs += runBytes.length;
    if (currentLineGlyphs !== null) {
      currentLineGlyphs += runBytes.length;
    }
    const markerMatches = runText.match(/\^[0-9]+/g);
    if (markerMatches) {
      footnoteMarkerCount += markerMatches.length;
    }
    tmRegex.lastIndex = tmIndex;
  }
  if (currentLineGlyphs !== null) {
    lineGlyphCounts.push(currentLineGlyphs);
  }
  const linkRectRegex = /\/Rect\s*\[\s*(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s*\]/g;
  let minLinkRectWidthPt = null;
  let maxLinkRectWidthPt = null;
  let minLinkRectHeightPt = null;
  let maxLinkRectHeightPt = null;
  let linkRectCount = 0;
  let linkRectMatch;
  while ((linkRectMatch = linkRectRegex.exec(pdfText)) !== null) {
    const x0 = Number.parseFloat(linkRectMatch[1]);
    const y0 = Number.parseFloat(linkRectMatch[2]);
    const x1 = Number.parseFloat(linkRectMatch[3]);
    const y1 = Number.parseFloat(linkRectMatch[4]);
    if (![x0, y0, x1, y1].every((value) => Number.isFinite(value))) {
      continue;
    }
    const width = Math.abs(x1 - x0);
    const height = Math.abs(y1 - y0);
    minLinkRectWidthPt = minLinkRectWidthPt === null ? width : Math.min(minLinkRectWidthPt, width);
    maxLinkRectWidthPt = maxLinkRectWidthPt === null ? width : Math.max(maxLinkRectWidthPt, width);
    minLinkRectHeightPt = minLinkRectHeightPt === null ? height : Math.min(minLinkRectHeightPt, height);
    maxLinkRectHeightPt = maxLinkRectHeightPt === null ? height : Math.max(maxLinkRectHeightPt, height);
    linkRectCount += 1;
  }
  const textRuns = extractPdfTextRunsV1(pdfText);
  const pageCount = countRegexMatchesV1(pdfText, /\/Type\s*\/Page\b/g);
  const annotsCount = countRegexMatchesV1(pdfText, /\/Subtype\s*\/Link\b/g);
  const uriCount = countRegexMatchesV1(pdfText, /\/URI\s*\(/g);
  const linesCount = lineGlyphCounts.length;
  const maxLineGlyphs = lineGlyphCounts.length > 0 ? Math.max(...lineGlyphCounts) : 0;
  const minYPt = lineYValues.length > 0 ? Math.min(...lineYValues) : 0;
  const maxYPt = lineYValues.length > 0 ? Math.max(...lineYValues) : 0;
  return {
    page_count: pageCount,
    total_lines: linesCount,
    total_glyphs: totalGlyphs,
    max_line_glyphs: maxLineGlyphs,
    min_y_pt: minYPt,
    max_y_pt: maxYPt,
    annots_count: annotsCount,
    uri_count: uriCount,
    footnote_marker_count: footnoteMarkerCount,
    pdf_text_run_count: textRuns.length,
    min_link_rect_width_pt: minLinkRectWidthPt ?? 0,
    max_link_rect_width_pt: maxLinkRectWidthPt ?? 0,
    min_link_rect_height_pt: minLinkRectHeightPt ?? 0,
    max_link_rect_height_pt: maxLinkRectHeightPt ?? 0,
    link_rect_count: linkRectCount,
  };
}

function computeXdvMetricsV1(xdvBytes) {
  const xdvText = Buffer.from(xdvBytes).toString('latin1');
  const newlineCount = countRegexMatchesV1(xdvText, /\n/g);
  const formFeedCount = countRegexMatchesV1(xdvText, /\f/g);
  return {
    byte_length: xdvBytes.length,
    newline_count: newlineCount,
    formfeed_count: formFeedCount,
  };
}

function buildBaselineMetricsV1(xdvBytes, pdfBytes, logBytes, summary) {
  const xdvMetrics = computeXdvMetricsV1(xdvBytes);
  const pdfMetrics = computePdfMetricsV1(pdfBytes);
  return {
    schema: 'baseline_metrics_v1',
    xdv_sha256: summary.artifact_sha256.main_xdv,
    pdf_sha256: summary.artifact_sha256.main_pdf,
    xdv_bytes: xdvBytes.length,
    pdf_bytes: pdfBytes.length,
    log_bytes: logBytes.length,
    resolved_resources_count: Number(summary.resolved_resources_count ?? 0),
    missing_resources_count: Number(summary.missing_resources_count ?? 0),
    ...xdvMetrics,
    ...pdfMetrics,
  };
}

function normalizeThresholdV1(value, key) {
  if (!Number.isFinite(value) || value < 0) {
    throw new Error(`delta policy threshold '${key}' must be a non-negative number`);
  }
  return Number(value);
}

async function loadDeltaPolicyV1(policyPathRaw) {
  const policyPath = path.resolve(policyPathRaw);
  const bytes = await readFile(policyPath);
  let parsed;
  try {
    parsed = JSON.parse(bytes.toString('utf8'));
  } catch {
    throw new Error(`invalid delta policy json: ${policyPath}`);
  }
  if (parsed?.schema !== DELTA_POLICY_V1_SCHEMA) {
    throw new Error(`delta policy schema must be ${DELTA_POLICY_V1_SCHEMA}`);
  }
  if (parsed?.version !== DELTA_POLICY_V1_VERSION) {
    throw new Error(`delta policy version must be ${DELTA_POLICY_V1_VERSION}`);
  }
  const okCasesRequireMatch = parsed?.ok_cases_require_match !== false;
  const allowlistRaw = parsed?.ok_case_allowlist;
  const okCaseAllowlist = {};
  if (allowlistRaw && typeof allowlistRaw === 'object' && !Array.isArray(allowlistRaw)) {
    for (const [caseId, entry] of Object.entries(allowlistRaw)) {
      if (typeof caseId !== 'string' || caseId.trim() === '') {
        throw new Error('delta policy allowlist has invalid case id');
      }
      const reason = typeof entry?.reason === 'string' ? entry.reason.trim() : '';
      const expires = typeof entry?.expires === 'string' ? entry.expires.trim() : '';
      if (!reason) {
        throw new Error(`delta policy allowlist entry for ${caseId} missing reason`);
      }
      okCaseAllowlist[caseId] = {
        reason,
        expires,
      };
    }
  }
  const nonOkMismatchClass = `${parsed?.non_ok_mismatch_class ?? BASELINE_CMP_CLASS_DIFF_OK_V1}`;
  if (!BASELINE_CMP_CLASS_ALLOWLIST_V1.has(nonOkMismatchClass)) {
    throw new Error(`delta policy has unsupported non_ok_mismatch_class: ${nonOkMismatchClass}`);
  }
  const okAllowlistedMismatchClass = `${parsed?.ok_allowlisted_mismatch_class ?? BASELINE_CMP_CLASS_DIFF_OK_V1}`;
  if (!BASELINE_CMP_CLASS_ALLOWLIST_V1.has(okAllowlistedMismatchClass)) {
    throw new Error(`delta policy has unsupported ok_allowlisted_mismatch_class: ${okAllowlistedMismatchClass}`);
  }
  const missingBaselineClass = `${parsed?.missing_baseline_class ?? BASELINE_CMP_CLASS_MISSING_V1}`;
  if (!BASELINE_CMP_CLASS_ALLOWLIST_V1.has(missingBaselineClass)) {
    throw new Error(`delta policy has unsupported missing_baseline_class: ${missingBaselineClass}`);
  }
  const skipClass = `${parsed?.skip_class ?? BASELINE_CMP_CLASS_SKIP_V1}`;
  if (!BASELINE_CMP_CLASS_ALLOWLIST_V1.has(skipClass)) {
    throw new Error(`delta policy has unsupported skip_class: ${skipClass}`);
  }
  const thresholdsRaw = parsed?.metrics_thresholds ?? {};
  if (typeof thresholdsRaw !== 'object' || thresholdsRaw === null || Array.isArray(thresholdsRaw)) {
    throw new Error('delta policy metrics_thresholds must be an object');
  }
  const metricsThresholds = {
    max_page_count_delta: normalizeThresholdV1(thresholdsRaw.max_page_count_delta ?? 0, 'max_page_count_delta'),
    max_total_lines_delta: normalizeThresholdV1(thresholdsRaw.max_total_lines_delta ?? 0, 'max_total_lines_delta'),
    max_total_glyphs_delta: normalizeThresholdV1(thresholdsRaw.max_total_glyphs_delta ?? 0, 'max_total_glyphs_delta'),
    max_annots_delta: normalizeThresholdV1(thresholdsRaw.max_annots_delta ?? 0, 'max_annots_delta'),
    max_footnote_marker_delta: normalizeThresholdV1(
      thresholdsRaw.max_footnote_marker_delta ?? 0,
      'max_footnote_marker_delta',
    ),
  };
  return {
    path: policyPath,
    sha256: sha256HexV0(bytes),
    ok_cases_require_match: okCasesRequireMatch,
    ok_case_allowlist: okCaseAllowlist,
    non_ok_mismatch_class: nonOkMismatchClass,
    ok_allowlisted_mismatch_class: okAllowlistedMismatchClass,
    missing_baseline_class: missingBaselineClass,
    skip_class: skipClass,
    metrics_thresholds: metricsThresholds,
  };
}

function classifyBaselineCmpV1(caseSpec, summary, deltaPolicy, baselineDir) {
  const reasons = [];
  const metrics = summary.baseline_metrics_v1 ?? {};
  if (!baselineDir) {
    reasons.push('baseline_dir_unset');
    return {
      class: deltaPolicy.skip_class,
      reasons,
      metrics,
    };
  }
  if (summary.baseline_match === 'MISSING') {
    reasons.push('baseline_missing');
    return {
      class: deltaPolicy.missing_baseline_class,
      reasons,
      metrics,
    };
  }
  if (summary.baseline_match === 'MATCH') {
    reasons.push('artifact_sha_match');
    return {
      class: BASELINE_CMP_CLASS_MATCH_V1,
      reasons,
      metrics,
    };
  }
  const allowlisted = deltaPolicy.ok_case_allowlist[caseSpec.id];
  if (summary.status === STATUS_OK_V0) {
    if (allowlisted) {
      reasons.push(`ok_case_allowlisted:${allowlisted.reason}`);
      if (allowlisted.expires) {
        reasons.push(`allowlist_expires:${allowlisted.expires}`);
      }
      return {
        class: deltaPolicy.ok_allowlisted_mismatch_class,
        reasons,
        metrics,
      };
    }
    reasons.push('ok_case_baseline_mismatch');
    reasons.push('requires_match');
    return {
      class: BASELINE_CMP_CLASS_DIFF_SUSPECT_V1,
      reasons,
      metrics,
    };
  }
  reasons.push(`non_ok_status:${summary.status}`);
  reasons.push('fail_closed_non_ok');
  return {
    class: deltaPolicy.non_ok_mismatch_class,
    reasons,
    metrics,
  };
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
      ondemand_opt_in: raw?.ondemand_opt_in === true,
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
      id: 'typeset_demo_hyperref_links_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_links_probe_v0.tex',
    },
    {
      id: 'typeset_demo_cjk_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_cjk_probe_v0.tex',
    },
    {
      id: 'typeset_demo_math_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_math_probe_v0.tex',
    },
    {
      id: 'typeset_demo_fixedpoint_graphics_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_fixedpoint_graphics_probe_v0.tex',
    },
    {
      id: 'typeset_demo_fixedpoint_bibliography_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_fixedpoint_bibliography_probe_v0.tex',
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
      id: 'typeset_demo_graphicspath_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphicspath_explicit_ext_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_explicit_ext_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphicspath_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_multipath_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_multipath_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_opts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_opts_invalid_probe_v0.tex',
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
      id: 'typeset_demo_input_cycle_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_cycle_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_missing_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_missing_probe_v0.tex',
    },
    {
      id: 'typeset_demo_ondemand_input_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_ondemand_input_probe_v0.tex',
    },
    {
      id: 'typeset_demo_ondemand_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_ondemand_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_includeonly_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_includeonly_probe_v0.tex',
    },
    {
      id: 'typeset_demo_package_require_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_package_require_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pkgopt_require_pass_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pkgopt_require_pass_probe_v0.tex',
    },
    {
      id: 'typeset_demo_class_options_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_class_options_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_opts_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_opts_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_opts_multi_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_opts_multi_probe_v0.tex',
    },
    {
      id: 'typeset_demo_passoptionstoclass_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_passoptionstoclass_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_emptyopts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_emptyopts_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_opts_multi_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_opts_multi_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_multipackage_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_multipackage_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_multipackage_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_multipackage_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_emptyopts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_emptyopts_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_package_require_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_package_require_invalid_probe_v0.tex',
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
    {
      id: 'typeset_demo_resource_hints_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_resource_hints_invalid_probe_v0.tex',
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
      ondemand_opt_in: metadata.ondemand_opt_in,
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

function buildConfigHashV0(cases, sourceDateEpoch, resolverId, deltaPolicySha256) {
  const config = {
    runner: 'wasm_fixture_gallery_v0',
    source_date_epoch: sourceDateEpoch,
    tz: 'UTC',
    max_log_bytes: DEFAULT_MAX_LOG_BYTES_V0,
    resolver_id: resolverId,
    delta_policy_sha256: deltaPolicySha256,
    cases: cases.map((item) => ({
      id: item.id,
      mode: item.mode,
      fixture: item.fixtureRelPath,
      tags: item.tags,
      expected_status: item.expected_status,
      ondemand_opt_in: item.ondemand_opt_in === true,
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
  ]);
  const entries = [];
  const sourceText = Buffer.from(sourceBytes).toString('utf8');
  if (!sourceText.includes('\\tableofcontents')) {
    return entries;
  }

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
        throw new Error(`toc_v1 title exceeds cap ${MAX_TOC_TITLE_BYTES_V0}`);
      }
      const anchorId = `h${entries.length + 1}`;
      entries.push({
        level,
        title: titleGroup.value,
        anchor_id: anchorId,
        page: null,
        source_span: buildSourceSpanV0(sourceBytes, index, titleGroup.next, 'toc_v1'),
      });
      if (entries.length > MAX_TOC_ENTRIES_V0) {
        throw new Error(`toc_v1 entries exceed cap ${MAX_TOC_ENTRIES_V0}`);
      }
    }

    index = titleGroup.next;
  }
  return entries;
}

function isMathPayloadWhitespaceByteV0(byte) {
  return byte === 0x20 || byte === 0x09 || byte === 0x0a || byte === 0x0d;
}

function isSafeMathPayloadByteV0(byte) {
  return byte >= 0x20
    && byte <= 0x7e
    && byte !== 0x24
    && byte !== 0x5c
    && byte !== 0x5b
    && byte !== 0x5d
    && byte !== 0x7b
    && byte !== 0x7d
    && byte !== 0x3c
    && byte !== 0x3e;
}

function pushMathPayloadSpaceV0(payloadBytes) {
  if (payloadBytes.length > 0 && payloadBytes[payloadBytes.length - 1] !== 0x20) {
    payloadBytes.push(0x20);
  }
}

function trimMathPayloadTrailingSpaceV0(payloadBytes) {
  while (payloadBytes.length > 0 && payloadBytes[payloadBytes.length - 1] === 0x20) {
    payloadBytes.pop();
  }
}

function addMathEntryV1(entries, kind, payloadBytes, lineIndex, sourceBytes, startByte, endByte) {
  trimMathPayloadTrailingSpaceV0(payloadBytes);
  if (payloadBytes.length === 0) {
    throw new Error('math_v1 payload must be non-empty');
  }
  if (payloadBytes.length > MAX_MATH_PAYLOAD_BYTES_V0) {
    throw new Error(`math_v1 payload exceeds cap ${MAX_MATH_PAYLOAD_BYTES_V0}`);
  }
  if (!Number.isInteger(lineIndex) || lineIndex <= 0) {
    throw new Error('math_v1 line_index must be a positive integer');
  }
  const payload = Uint8Array.from(payloadBytes);
  entries.push({
    kind,
    payload_sha256: sha256HexV0(payload),
    line_index: lineIndex,
    source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'math_v1'),
  });
  if (entries.length > MAX_MATH_ENTRIES_V0) {
    throw new Error(`math_v1 entries exceed cap ${MAX_MATH_ENTRIES_V0}`);
  }
}

function extractMathEntriesFromSourceV1(sourceBytes) {
  const entries = [];
  let index = 0;
  let lineIndex = 1;
  while (index < sourceBytes.length) {
    const byte = sourceBytes[index];

    if (byte === 0x0a) {
      lineIndex += 1;
      index += 1;
      continue;
    }
    if (byte === 0x0d) {
      if (index + 1 < sourceBytes.length && sourceBytes[index + 1] === 0x0a) {
        index += 1;
      }
      lineIndex += 1;
      index += 1;
      continue;
    }

    if (byte === 0x24) {
      const startByte = index;
      const startLineIndex = lineIndex;
      const payloadBytes = [];
      let cursor = index + 1;
      let closed = false;
      while (cursor < sourceBytes.length) {
        const current = sourceBytes[cursor];
        if (current === 0x24) {
          addMathEntryV1(entries, 'inline', payloadBytes, startLineIndex, sourceBytes, startByte, cursor + 1);
          cursor += 1;
          index = cursor;
          closed = true;
          break;
        }
        if (isMathPayloadWhitespaceByteV0(current)) {
          pushMathPayloadSpaceV0(payloadBytes);
          if (current === 0x0a) {
            lineIndex += 1;
          } else if (current === 0x0d) {
            if (cursor + 1 < sourceBytes.length && sourceBytes[cursor + 1] === 0x0a) {
              cursor += 1;
            }
            lineIndex += 1;
          }
          cursor += 1;
          continue;
        }
        if (!isSafeMathPayloadByteV0(current)) {
          throw new Error(`math_v1 inline payload has unsupported byte 0x${current.toString(16).padStart(2, '0')}`);
        }
        payloadBytes.push(current);
        cursor += 1;
      }
      if (!closed) {
        throw new Error('math_v1 inline payload missing closing $ delimiter');
      }
      continue;
    }

    if (byte === 0x5c && index + 1 < sourceBytes.length && sourceBytes[index + 1] === 0x5b) {
      const startByte = index;
      const startLineIndex = lineIndex;
      const payloadBytes = [];
      let cursor = index + 2;
      let closed = false;
      while (cursor < sourceBytes.length) {
        if (sourceBytes[cursor] === 0x5c && cursor + 1 < sourceBytes.length && sourceBytes[cursor + 1] === 0x5d) {
          addMathEntryV1(entries, 'display', payloadBytes, startLineIndex, sourceBytes, startByte, cursor + 2);
          cursor += 2;
          index = cursor;
          closed = true;
          break;
        }
        const current = sourceBytes[cursor];
        if (isMathPayloadWhitespaceByteV0(current)) {
          pushMathPayloadSpaceV0(payloadBytes);
          if (current === 0x0a) {
            lineIndex += 1;
          } else if (current === 0x0d) {
            if (cursor + 1 < sourceBytes.length && sourceBytes[cursor + 1] === 0x0a) {
              cursor += 1;
            }
            lineIndex += 1;
          }
          cursor += 1;
          continue;
        }
        if (!isSafeMathPayloadByteV0(current)) {
          throw new Error(`math_v1 display payload has unsupported byte 0x${current.toString(16).padStart(2, '0')}`);
        }
        payloadBytes.push(current);
        cursor += 1;
      }
      if (!closed) {
        throw new Error('math_v1 display payload missing closing \\] delimiter');
      }
      continue;
    }

    index += 1;
  }
  return entries;
}

function indexOfSubarrayV0(bytes, needle, startIndex) {
  if (needle.length === 0) {
    return startIndex;
  }
  outer: for (let index = startIndex; index + needle.length <= bytes.length; index += 1) {
    for (let offset = 0; offset < needle.length; offset += 1) {
      if (bytes[index + offset] !== needle[offset]) {
        continue outer;
      }
    }
    return index;
  }
  return -1;
}

function tableGlyphWidthPtV0(byte) {
  const base = 7.2;
  if (
    byte === 0x20 // space
    || byte === 0x2e // .
    || byte === 0x2c // ,
    || byte === 0x3b // ;
    || byte === 0x3a // :
    || byte === 0x21 // !
    || byte === 0x3f // ?
    || byte === 0x27 // '
    || byte === 0x22 // "
    || byte === 0x69 // i
    || byte === 0x6c // l
    || byte === 0x49 // I
    || byte === 0x7c // |
  ) {
    return base * 0.5;
  }
  if (
    byte === 0x6d // m
    || byte === 0x77 // w
    || byte === 0x4d // M
    || byte === 0x57 // W
  ) {
    return base * 1.5;
  }
  return base;
}

function tableTextWidthPtV0(text) {
  const bytes = Buffer.from(text, 'utf8');
  let widthPt = 0;
  for (const byte of bytes) {
    widthPt += tableGlyphWidthPtV0(byte);
  }
  return Number(widthPt.toFixed(2));
}

function normalizeTableCellTextV0(rawCell) {
  return rawCell.replace(/\s+/g, ' ').trim();
}

function parseTabularRowsV1(bodyText, expectedColumnCount) {
  const normalizedBody = bodyText.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
  const rawRows = normalizedBody
    .split('\\\\')
    .map((row) => row.trim())
    .filter((row) => row.length > 0);
  if (rawRows.length === 0 || rawRows.length > MAX_TABLE_ROWS_PER_ENTRY_V0) {
    throw new Error(`table_v1 invalid row count ${rawRows.length}`);
  }
  const rows = [];
  for (const rawRow of rawRows) {
    const cells = rawRow.split('&').map(normalizeTableCellTextV0);
    if (cells.length !== expectedColumnCount) {
      throw new Error(`table_v1 row column count mismatch: expected ${expectedColumnCount}, got ${cells.length}`);
    }
    if (cells.some((cell) => cell.length === 0)) {
      throw new Error('table_v1 row contains empty cell');
    }
    rows.push(cells);
  }
  return rows;
}

function extractTableEntriesFromSourceV1(sourceBytes) {
  const beginMarker = Buffer.from('\\begin{tabular}{', 'utf8');
  const endMarker = Buffer.from('\\end{tabular}', 'utf8');
  const entries = [];
  let index = 0;

  while (index < sourceBytes.length) {
    const beginIndex = indexOfSubarrayV0(sourceBytes, beginMarker, index);
    if (beginIndex < 0) {
      break;
    }
    const alignStart = beginIndex + beginMarker.length;
    let alignEnd = alignStart;
    while (alignEnd < sourceBytes.length && sourceBytes[alignEnd] !== 0x7d) {
      alignEnd += 1;
    }
    if (alignEnd >= sourceBytes.length) {
      throw new Error('table_v1 tabular align spec missing closing }');
    }
    const alignSpec = Buffer.from(sourceBytes.slice(alignStart, alignEnd))
      .toString('utf8')
      .replace(/\s+/g, '');
    if (
      alignSpec.length === 0
      || alignSpec.length > MAX_TABLE_COLS_PER_ENTRY_V0
      || !/^[lcr]+$/.test(alignSpec)
    ) {
      throw new Error(`table_v1 unsupported align spec '${alignSpec}'`);
    }
    const bodyStart = alignEnd + 1;
    const endIndex = indexOfSubarrayV0(sourceBytes, endMarker, bodyStart);
    if (endIndex < 0) {
      throw new Error('table_v1 tabular missing end marker');
    }
    const bodyText = Buffer.from(sourceBytes.slice(bodyStart, endIndex)).toString('utf8');
    const rows = parseTabularRowsV1(bodyText, alignSpec.length);

    const columnWidthsPt = Array.from({ length: alignSpec.length }, () => 0);
    for (const row of rows) {
      for (let col = 0; col < alignSpec.length; col += 1) {
        columnWidthsPt[col] = Math.max(columnWidthsPt[col], tableTextWidthPtV0(row[col]));
      }
    }

    entries.push({
      anchor_id: `tbl${entries.length + 1}`,
      align_spec: alignSpec,
      column_count: alignSpec.length,
      row_count: rows.length,
      column_widths_pt: columnWidthsPt,
      rows,
      source_span: buildSourceSpanV0(sourceBytes, beginIndex, endIndex + endMarker.length, 'table_v1'),
    });
    if (entries.length > MAX_TABLE_ENTRIES_V0) {
      throw new Error(`table_v1 entries exceed cap ${MAX_TABLE_ENTRIES_V0}`);
    }
    index = endIndex + endMarker.length;
  }
  return entries;
}

function splitCommaValuesV0(rawValue) {
  return rawValue
    .split(',')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
}

function dedupeValuesPreserveOrderV0(values) {
  const deduped = [];
  const seen = new Set();
  for (const value of values) {
    const key = value.toLowerCase();
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    deduped.push(value);
  }
  return deduped;
}

function mergePkgoptUsepackageEntriesV0(entries) {
  const merged = [];
  const indexByKey = new Map();
  for (const entry of entries) {
    const command = typeof entry?.command === 'string' ? entry.command : '';
    if (command !== 'usepackage' && command !== 'RequirePackage') {
      merged.push(entry);
      continue;
    }
    const packageName = typeof entry?.package === 'string' ? entry.package : '';
    const key = `${command}\u0000${packageName.toLowerCase()}`;
    const existingIndex = indexByKey.get(key);
    if (existingIndex === undefined) {
      indexByKey.set(key, merged.length);
      merged.push({
        ...entry,
        options: dedupeValuesPreserveOrderV0(Array.isArray(entry.options) ? entry.options : []),
      });
      continue;
    }
    const existing = merged[existingIndex];
    const mergedOptions = dedupeValuesPreserveOrderV0([
      ...(Array.isArray(existing.options) ? existing.options : []),
      ...(Array.isArray(entry.options) ? entry.options : []),
    ]);
    const start = Math.min(
      existing.source_span?.start_byte ?? Number.MAX_SAFE_INTEGER,
      entry.source_span?.start_byte ?? Number.MAX_SAFE_INTEGER,
    );
    const end = Math.max(
      existing.source_span?.end_byte ?? 0,
      entry.source_span?.end_byte ?? 0,
    );
    merged[existingIndex] = {
      ...existing,
      options: mergedOptions,
      source_span: {
        start_byte: start,
        end_byte: end,
      },
    };
  }
  return merged;
}

function splitCommaOptionsStrictV0(rawValue, context) {
  const values = [];
  for (const chunk of rawValue.split(',')) {
    const trimmed = chunk.trim();
    if (trimmed.length === 0) {
      throw new Error(`${context} has empty option entry`);
    }
    values.push(trimmed);
  }
  return dedupeValuesPreserveOrderV0(values);
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

function parseIncludegraphicsOptionsStrictV0(rawValue) {
  const assignments = new Map();
  for (const chunk of rawValue.split(',')) {
    const trimmed = chunk.trim();
    if (trimmed.length === 0) {
      throw new Error('resource_hints_v0 includegraphics options has empty entry');
    }
    const equalsIndex = trimmed.indexOf('=');
    if (equalsIndex < 0) {
      continue;
    }
    const key = trimmed.slice(0, equalsIndex).trim().toLowerCase();
    let value = trimmed.slice(equalsIndex + 1).trim();
    while (value.length >= 2 && value.startsWith('{') && value.endsWith('}')) {
      value = value.slice(1, -1).trim();
    }
    if (key.length === 0 || value.length === 0) {
      throw new Error(`resource_hints_v0 includegraphics option '${trimmed}' is malformed`);
    }
    assignments.set(key, value);
  }
  return assignments;
}

function parseGraphicspathValuesV0(rawValue) {
  const values = [];
  let index = 0;
  while (index < rawValue.length) {
    while (index < rawValue.length && /\s/.test(rawValue[index])) {
      index += 1;
    }
    if (index >= rawValue.length) {
      break;
    }
    if (rawValue[index] !== '{') {
      throw new Error(`resource_hints_v0 graphicspath has malformed entry '${rawValue}'`);
    }
    index += 1;
    const start = index;
    while (index < rawValue.length && rawValue[index] !== '}') {
      if (rawValue[index] === '{') {
        throw new Error(`resource_hints_v0 graphicspath rejects nested braces '${rawValue}'`);
      }
      index += 1;
    }
    if (index >= rawValue.length || rawValue[index] !== '}') {
      throw new Error(`resource_hints_v0 graphicspath has unterminated entry '${rawValue}'`);
    }
    const value = rawValue.slice(start, index).trim();
    if (value.length > 0) {
      values.push(value);
    }
    index += 1;
  }
  if (values.length === 0) {
    throw new Error(`resource_hints_v0 graphicspath requires at least one path '${rawValue}'`);
  }
  return values;
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
  const packageCommands = new Set([
    'usepackage',
    'RequirePackage',
    'PassOptionsToPackage',
    'RequirePackageWithOptions',
    'PassOptionsToClass',
    'documentclass',
  ]);
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

    let options = [];
    let packages = [];
    let endOffset = commandIndex;

    if (command === 'PassOptionsToPackage' || command === 'PassOptionsToClass') {
      const optionGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!optionGroup.ok || optionGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const packageGroup = readBracedGroupV0(sourceBytes, optionGroup.next);
      if (!packageGroup.ok || packageGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      if (command === 'PassOptionsToClass' || command === 'usepackage' || command === 'RequirePackage') {
        options = splitCommaOptionsStrictV0(optionGroup.value, 'pkgopt_v0 PassOptionsToClass');
      } else {
        options = dedupeValuesPreserveOrderV0(splitCommaValuesV0(optionGroup.value));
      }
      packages = dedupeValuesPreserveOrderV0(splitCommaValuesV0(packageGroup.value));
      endOffset = packageGroup.next;
    } else if (command === 'documentclass') {
      const optGroup = readBracketGroupV0(sourceBytes, commandIndex);
      let next = commandIndex;
      if (optGroup.ok) {
        next = optGroup.next;
      }
      const classGroup = readBracedGroupV0(sourceBytes, next);
      if (!classGroup.ok || classGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      options = optGroup.ok ? splitCommaOptionsStrictV0(optGroup.value, 'pkgopt_v0 documentclass options') : [];
      if (options.length === 0) {
        index = classGroup.next;
        continue;
      }
      packages = dedupeValuesPreserveOrderV0(splitCommaValuesV0(classGroup.value));
      endOffset = classGroup.next;
    } else if (command === 'RequirePackageWithOptions') {
      const packageGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!packageGroup.ok || packageGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      options = ['withoptions'];
      packages = dedupeValuesPreserveOrderV0(splitCommaValuesV0(packageGroup.value));
      endOffset = packageGroup.next;
    } else {
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
      options = dedupeValuesPreserveOrderV0(splitCommaValuesV0(optGroup.value));
      packages = dedupeValuesPreserveOrderV0(splitCommaValuesV0(pkgGroup.value));
      endOffset = pkgGroup.next;
    }

    for (const pkgName of packages) {
      addPkgoptEntryV0(entries, {
        command,
        package: pkgName,
        options,
        source_span: buildSourceSpanV0(sourceBytes, index, endOffset, 'pkgopt_v0'),
      });
    }
    index = endOffset;
  }
  return mergePkgoptUsepackageEntriesV0(entries);
}

function extractGraphicsEntriesFromSourceV0(sourceBytes) {
  const allowedExtensions = new Set(['png', 'jpg', 'jpeg', 'pdf']);
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
    const pathAsWritten = pathGroup.value.trim();
    if (pathAsWritten.length === 0) {
      index = pathGroup.next;
      continue;
    }
    const normalizedToken = normalizePathHintTokenV0(pathAsWritten, 'graphics_path');
    if (normalizedToken === null) {
      index = pathGroup.next;
      continue;
    }
    const resolverPath = ensureDefaultExtensionV0(normalizedToken, 'png');
    const dotIndex = resolverPath.lastIndexOf('.');
    const extension = dotIndex >= 0 ? resolverPath.slice(dotIndex + 1).toLowerCase() : '';
    if (!allowedExtensions.has(extension)) {
      throw new Error(`graphics_v1 unsupported extension '${extension}'`);
    }

    const pathBytes = Buffer.from(pathAsWritten, 'utf8');
    if (pathBytes.length > MAX_GRAPHICS_PATH_BYTES_V0) {
      throw new Error(`graphics_v1 path exceeds cap ${MAX_GRAPHICS_PATH_BYTES_V0}`);
    }
    entries.push({
      command,
      path: pathAsWritten,
      resolver_path: resolverPath,
      source_span: buildSourceSpanV0(sourceBytes, index, pathGroup.next, 'graphics_v1'),
    });
    if (entries.length > MAX_GRAPHICS_ENTRIES_V0) {
      throw new Error(`graphics_v1 entries exceed cap ${MAX_GRAPHICS_ENTRIES_V0}`);
    }
    index = pathGroup.next;
  }
  return entries;
}

function normalizeInputIncludeMountPathV1(rawValue) {
  if (typeof rawValue !== 'string') {
    return null;
  }
  const trimmed = rawValue.trim();
  if (trimmed.length === 0) {
    return null;
  }
  if (trimmed.startsWith('/') || trimmed.startsWith('\\')) {
    return null;
  }
  if (trimmed.includes('\\')) {
    return null;
  }
  if (/^[A-Za-z]:([\\/]|$)/.test(trimmed)) {
    return null;
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
    return null;
  }
  if (segments.some((segment) => !/^[A-Za-z0-9._-]+$/.test(segment))) {
    return null;
  }
  const normalizedPath = segments.join('/');
  const lastSegment = segments[segments.length - 1];
  if (lastSegment.includes('.')) {
    return normalizedPath;
  }
  return `${normalizedPath}.tex`;
}

function resolverNameFromMountPathV1(mountPath) {
  if (typeof mountPath !== 'string' || mountPath.length === 0) {
    return null;
  }
  const name = mountPath.replace(/\//g, '__');
  if (!isSafeResolverTokenV0(name)) {
    return null;
  }
  return name;
}

function extractInputIncludeDirectivesFromSourceV1(sourceBytes, sourcePath) {
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
    if (command !== 'input' && command !== 'include') {
      index = commandIndex;
      continue;
    }
    const group = readBracedGroupV0(sourceBytes, commandIndex);
    if (!group.ok || group.value.length === 0) {
      index = commandIndex;
      continue;
    }
    const values = splitCommaValuesV0(group.value);
    for (const rawValue of values) {
      const mountPath = normalizeInputIncludeMountPathV1(rawValue);
      if (!mountPath) {
        continue;
      }
      const resolverName = resolverNameFromMountPathV1(mountPath);
      if (!resolverName) {
        continue;
      }
      entries.push({
        command,
        hint_type: command === 'input' ? 'tex_input' : 'tex_include',
        source_path: sourcePath,
        value: mountPath,
        resolver_name: resolverName,
        source_span: buildSourceSpanV0(sourceBytes, index, group.next, 'input_v1'),
      });
    }
    index = group.next;
  }
  return entries;
}

async function collectInputIncludeGraphV1(sourceBytes, resolver, caseSpec) {
  const queue = [{
    source_path: 'main.tex',
    source_bytes: toUint8ArrayV0(sourceBytes),
    depth: 0,
  }];
  const parsedPaths = new Set(['main.tex']);
  const resolvedByMountPath = new Map();
  const resolverRequestsByKey = new Map();
  const graphEntries = [];

  while (queue.length > 0) {
    const current = queue.shift();
    const directives = extractInputIncludeDirectivesFromSourceV1(current.source_bytes, current.source_path);
    for (const directive of directives) {
      if (graphEntries.length >= MAX_INPUT_ENTRIES_V1) {
        throw new Error(`input_v1 entries exceed cap ${MAX_INPUT_ENTRIES_V1}`);
      }
      const request = {
        kind: 'texmf',
        format: 'tex',
        name: directive.resolver_name,
        variant: caseSpec.mode,
        hint_type: directive.hint_type,
      };
      resolverRequestsByKey.set(resolverRequestKeyV0(request), request);

      let resolution = resolvedByMountPath.get(directive.value);
      if (!resolution) {
        resolution = await resolver.resolve({
          kind: request.kind,
          format: request.format,
          name: request.name,
          variant: request.variant,
          resolver_id: resolver.resolverId,
        });
        resolvedByMountPath.set(directive.value, resolution);
      }

      const resolved = resolution.tag === 'Found';
      if (current.source_path === 'main.tex') {
        graphEntries.push({
          command: directive.command,
          source_path: directive.source_path,
          value: directive.value,
          resolver_name: directive.resolver_name,
          source_span: directive.source_span,
        });
      }

      if (!resolved || parsedPaths.has(directive.value)) {
        continue;
      }
      if (current.depth + 1 > MAX_INPUT_INCLUDE_DEPTH_V1) {
        throw new Error(`input_v1 include depth exceeds cap ${MAX_INPUT_INCLUDE_DEPTH_V1}`);
      }
      parsedPaths.add(directive.value);
      queue.push({
        source_path: directive.value,
        source_bytes: toUint8ArrayV0(resolution.bytes),
        depth: current.depth + 1,
      });
    }
  }

  graphEntries.sort((left, right) => {
    const sourceCmp = left.source_path.localeCompare(right.source_path);
    if (sourceCmp !== 0) {
      return sourceCmp;
    }
    const startCmp = left.source_span.start_byte - right.source_span.start_byte;
    if (startCmp !== 0) {
      return startCmp;
    }
    const endCmp = left.source_span.end_byte - right.source_span.end_byte;
    if (endCmp !== 0) {
      return endCmp;
    }
    const commandCmp = left.command.localeCompare(right.command);
    if (commandCmp !== 0) {
      return commandCmp;
    }
    return left.value.localeCompare(right.value);
  });

  const mountedFiles = [...resolvedByMountPath.entries()]
    .filter(([, resolution]) => resolution.tag === 'Found')
    .sort(([leftPath], [rightPath]) => leftPath.localeCompare(rightPath))
    .map(([mountPath, resolution]) => [mountPath, toUint8ArrayV0(resolution.bytes)]);

  const resolverRequests = [...resolverRequestsByKey.values()]
    .sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));

  return {
    entries: graphEntries,
    mounted_files: mountedFiles,
    resolver_requests: resolverRequests,
  };
}

async function emitInputTypedArtifactV1(caseOutDir, inputEntries) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'input_v1',
    entries: inputEntries,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'input_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

function addResourceHintEntryV0(entries, sourceBytes, caseId, hintType, value, startByte, endByte) {
  const valueBytes = Buffer.from(value, 'utf8');
  if (valueBytes.length > MAX_RESOURCE_HINT_VALUE_BYTES_V0) {
    throw new Error(`resource_hints_v0 value exceeds cap ${MAX_RESOURCE_HINT_VALUE_BYTES_V0}`);
  }
  entries.push({
    kind: 'resource_hint',
    case_id: caseId,
    hint_type: hintType,
    value,
    source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'resource_hints_v0'),
  });
  if (entries.length > MAX_RESOURCE_HINT_ENTRIES_V0) {
    throw new Error(`resource_hints_v0 entries exceed cap ${MAX_RESOURCE_HINT_ENTRIES_V0}`);
  }
}

function extractResourceHintEntriesFromSourceV0(sourceBytes, caseId) {
  const entries = [];
  const seen = new Set();
  let graphicspathPrefixes = [];
  let index = 0;

  const addHintValues = (
    hintType,
    values,
    startByte,
    endByte,
    defaultExtension = null,
    ignoreInvalidPathToken = false,
  ) => {
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
        addResourceHintEntryV0(entries, sourceBytes, caseId, hintType, normalizedUrl, startByte, endByte);
        continue;
      }
      let normalizedPath;
      try {
        normalizedPath = normalizePathHintTokenV0(rawValue, hintType);
      } catch (error) {
        if (ignoreInvalidPathToken) {
          continue;
        }
        throw error;
      }
      if (!normalizedPath) {
        continue;
      }
      const normalized = defaultExtension ? ensureDefaultExtensionV0(normalizedPath, defaultExtension) : normalizedPath;
      const dedupeKey = `${hintType}\u0000${normalized.toLowerCase()}`;
      if (seen.has(dedupeKey)) {
        continue;
      }
      seen.add(dedupeKey);
      addResourceHintEntryV0(entries, sourceBytes, caseId, hintType, normalized, startByte, endByte);
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

    if (command === 'includeonly') {
      const group = readBracedGroupV0(sourceBytes, commandIndex);
      if (!group.ok || group.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('tex_includeonly', splitCommaValuesV0(group.value), index, group.next, 'tex');
      index = group.next;
      continue;
    }

    if (command === 'usepackage' || command === 'RequirePackage') {
      let next = commandIndex;
      const optionsGroup = readBracketGroupV0(sourceBytes, next);
      if (optionsGroup.ok) {
        splitCommaOptionsStrictV0(optionsGroup.value, 'resource_hints_v0 usepackage options');
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

    if (command === 'documentclass') {
      const optGroup = readBracketGroupV0(sourceBytes, commandIndex);
      let next = commandIndex;
      if (optGroup.ok) {
        splitCommaOptionsStrictV0(optGroup.value, 'resource_hints_v0 documentclass options');
        next = optGroup.next;
      }
      const classGroup = readBracedGroupV0(sourceBytes, next);
      if (!classGroup.ok || classGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('class_file', splitCommaValuesV0(classGroup.value), index, classGroup.next, 'cls');
      index = classGroup.next;
      continue;
    }

    if (command === 'RequirePackageWithOptions') {
      const packageGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!packageGroup.ok || packageGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('package_file', splitCommaValuesV0(packageGroup.value), index, packageGroup.next, 'sty');
      index = packageGroup.next;
      continue;
    }

    if (command === 'PassOptionsToPackage') {
      const optionGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!optionGroup.ok || optionGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const packageGroup = readBracedGroupV0(sourceBytes, optionGroup.next);
      if (!packageGroup.ok || packageGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('package_file', splitCommaValuesV0(packageGroup.value), index, packageGroup.next, 'sty');
      index = packageGroup.next;
      continue;
    }

    if (command === 'PassOptionsToClass') {
      const optionGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!optionGroup.ok || optionGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      splitCommaOptionsStrictV0(optionGroup.value, 'resource_hints_v0 PassOptionsToClass');
      const classGroup = readBracedGroupV0(sourceBytes, optionGroup.next);
      if (!classGroup.ok || classGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      addHintValues('class_file', splitCommaValuesV0(classGroup.value), index, classGroup.next, 'cls');
      index = classGroup.next;
      continue;
    }

    if (command === 'includegraphics') {
      let next = commandIndex;
      const optionsGroup = readBracketGroupV0(sourceBytes, next);
      const graphicsOptions = optionsGroup.ok ? parseIncludegraphicsOptionsStrictV0(optionsGroup.value) : new Map();
      if (optionsGroup.ok) {
        next = optionsGroup.next;
      }
      const graphicsGroup = readBracedGroupV0(sourceBytes, next);
      if (!graphicsGroup.ok || graphicsGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const extRaw = (
        graphicsOptions.get('ext')
        ?? graphicsOptions.get('extension')
        ?? graphicsOptions.get('type')
        ?? ''
      ).trim();
      let extNormalized = '';
      if (extRaw.length > 0) {
        const extCandidate = extRaw.replace(/^\./, '');
        if (!/^[a-z0-9]+$/i.test(extCandidate)) {
          throw new Error(`resource_hints_v0 includegraphics rejects extension '${extRaw}'`);
        }
        extNormalized = extCandidate;
      }
      const dirRaw = (graphicsOptions.get('dir') ?? graphicsOptions.get('path') ?? '').trim();
      const dirNormalized = normalizePathHintTokenV0(dirRaw, 'graphics_path');
      const fileRaw = (graphicsOptions.get('file') ?? graphicsOptions.get('filename') ?? '').trim();
      const includeValues = fileRaw.length > 0 ? splitCommaValuesV0(fileRaw) : splitCommaValuesV0(graphicsGroup.value);
      const candidates = [];
      for (const value of includeValues) {
        const useGraphicspathPrefixes = !dirNormalized
          && graphicspathPrefixes.length > 0
          && !value.includes('/')
          && !value.includes('\\');
        if (useGraphicspathPrefixes) {
          for (const prefix of graphicspathPrefixes) {
            const withDir = `${prefix}/${value}`;
            const withExt = extNormalized.length > 0 ? ensureDefaultExtensionV0(withDir, extNormalized) : withDir;
            candidates.push(withExt);
          }
        } else {
          const withDir = dirNormalized ? `${dirNormalized}/${value}` : value;
          const withExt = extNormalized.length > 0 ? ensureDefaultExtensionV0(withDir, extNormalized) : withDir;
          candidates.push(withExt);
        }
      }
      addHintValues('graphics_path', candidates, index, graphicsGroup.next);
      index = graphicsGroup.next;
      continue;
    }

    if (command === 'graphicspath') {
      const pathGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!pathGroup.ok || pathGroup.value.length === 0) {
        index = commandIndex;
        continue;
      }
      const prefixes = [];
      for (const rawValue of parseGraphicspathValuesV0(pathGroup.value)) {
        try {
          const normalized = normalizePathHintTokenV0(rawValue, 'graphics_path');
          if (normalized && normalized.length > 0) {
            prefixes.push(normalized);
          }
        } catch {
          continue;
        }
      }
      graphicspathPrefixes = prefixes;
      index = pathGroup.next;
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

function validateResourceHintEntriesV0(entries, fixtureBytes, caseId) {
  if (!Array.isArray(entries)) {
    throw new Error(`resource_hints_v0 entries must be array for case ${caseId}`);
  }
  for (const [index, entry] of entries.entries()) {
    if (entry?.kind !== 'resource_hint') {
      throw new Error(`resource_hints_v0 entry[${index}] invalid kind for case ${caseId}`);
    }
    if (entry?.case_id !== caseId) {
      throw new Error(`resource_hints_v0 entry[${index}] missing/invalid case_id for case ${caseId}`);
    }
    const hintType = typeof entry?.hint_type === 'string' ? entry.hint_type : '';
    if (!RESOURCE_HINT_TYPE_ALLOWLIST_V0.has(hintType)) {
      throw new Error(`resource_hints_v0 entry[${index}] unknown hint_type '${hintType}' for case ${caseId}`);
    }
    if (typeof entry?.value !== 'string' || entry.value.trim() === '') {
      throw new Error(`resource_hints_v0 entry[${index}] missing value for case ${caseId}`);
    }
    const sourceSpan = entry?.source_span;
    if (!sourceSpan || !Number.isInteger(sourceSpan.start_byte) || !Number.isInteger(sourceSpan.end_byte)) {
      throw new Error(`resource_hints_v0 entry[${index}] missing source_span for case ${caseId}`);
    }
    if (sourceSpan.start_byte < 0 || sourceSpan.end_byte <= sourceSpan.start_byte || sourceSpan.end_byte > fixtureBytes.length) {
      throw new Error(`resource_hints_v0 entry[${index}] source_span out of bounds for case ${caseId}`);
    }
  }
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

function extractBibitemsAndCitesFromSourceV1(sourceBytes) {
  const bibitems = [];
  const bibOrdinalByKey = new Map();
  const citesByKey = new Map();
  const citeEntriesV1 = [];
  const citeOrderByKey = new Map();
  const citeCommands = new Set(['cite']);
  let inBibliography = false;
  let index = 0;

  const addBibitem = (key, text, startByte, endByte) => {
    const keyBytes = Buffer.from(key, 'utf8');
    if (keyBytes.length > MAX_BIB_VALUE_BYTES_V0) {
      throw new Error(`bibitems_v1 key exceeds cap ${MAX_BIB_VALUE_BYTES_V0}`);
    }
    const textBytes = Buffer.from(text, 'utf8');
    if (textBytes.length > MAX_BIB_VALUE_BYTES_V0 * 8) {
      throw new Error(`bibitems_v1 text exceeds cap ${MAX_BIB_VALUE_BYTES_V0 * 8}`);
    }
    if (bibOrdinalByKey.has(key)) {
      throw new Error(`bibitems_v1 duplicate key '${key}'`);
    }
    const ordinal = bibitems.length + 1;
    const sourceSpan = buildSourceSpanV0(sourceBytes, startByte, endByte, 'bibitems_v1');
    bibOrdinalByKey.set(key, ordinal);
    bibitems.push({
      key,
      ordinal,
      text_sha256: sha256HexV0(textBytes),
      source_span: sourceSpan,
    });
    if (bibitems.length > MAX_BIB_ENTRIES_V0) {
      throw new Error(`bibitems_v1 entries exceed cap ${MAX_BIB_ENTRIES_V0}`);
    }
  };

  const addCiteOccurrence = (key, startByte, endByte) => {
    const keyBytes = Buffer.from(key, 'utf8');
    if (keyBytes.length > MAX_BIB_VALUE_BYTES_V0) {
      throw new Error(`cites_v1 key exceeds cap ${MAX_BIB_VALUE_BYTES_V0}`);
    }
    let entry = citesByKey.get(key);
    if (!entry) {
      entry = {
        key,
        occurrences: [],
        resolved: false,
        ordinal: null,
        source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'cites_v1'),
      };
      citesByKey.set(key, entry);
    }
    entry.occurrences.push({
      line_index: lineIndexForByteOffsetV1(sourceBytes, startByte),
    });
    if (entry.occurrences.length > MAX_REF_OCCURRENCES_PER_KEY_V0) {
      throw new Error(`cites_v1 occurrences exceed cap ${MAX_REF_OCCURRENCES_PER_KEY_V0} for key ${key}`);
    }
    let citeOrder = citeOrderByKey.get(key);
    if (citeOrder === undefined) {
      citeOrder = citeOrderByKey.size + 1;
      citeOrderByKey.set(key, citeOrder);
    }
    citeEntriesV1.push({
      key,
      line_index: lineIndexForByteOffsetV1(sourceBytes, startByte),
      cite_order: citeOrder,
      resolved: false,
      ordinal: null,
      source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'cite_v1'),
    });
    if (citeEntriesV1.length > MAX_BIB_ENTRIES_V0) {
      throw new Error(`cite_v1 entries exceed cap ${MAX_BIB_ENTRIES_V0}`);
    }
  };

  const normalizeBibitemText = (bytes) => {
    const raw = Buffer.from(bytes).toString('utf8');
    return raw.replace(/\s+/g, ' ').trim();
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

    if (command === 'begin' || command === 'end') {
      const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!envGroup.ok) {
        index = commandIndex;
        continue;
      }
      const envName = envGroup.value.trim();
      if (envName === 'thebibliography') {
        inBibliography = command === 'begin';
      }
      index = envGroup.next;
      continue;
    }

    if (inBibliography && command === 'bibitem') {
      const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!keyGroup.ok) {
        index = commandIndex;
        continue;
      }
      const key = keyGroup.value.trim();
      if (!isSafeLabelRefKeyValueV1(key)) {
        index = keyGroup.next;
        continue;
      }
      let cursor = keyGroup.next;
      let textEnd = cursor;
      while (cursor < sourceBytes.length) {
        if (sourceBytes[cursor] !== 0x5c) {
          cursor += 1;
          continue;
        }
        let nestedCommandEnd = cursor + 1;
        while (
          nestedCommandEnd < sourceBytes.length
          && isAsciiLetterByteV0(sourceBytes[nestedCommandEnd])
        ) {
          nestedCommandEnd += 1;
        }
        if (nestedCommandEnd === cursor + 1) {
          cursor += 1;
          continue;
        }
        const nestedCommand = Buffer.from(sourceBytes.slice(cursor + 1, nestedCommandEnd)).toString('ascii');
        if (nestedCommand === 'bibitem') {
          break;
        }
        if (nestedCommand === 'end') {
          const envGroup = readBracedGroupV0(sourceBytes, nestedCommandEnd);
          if (envGroup.ok && envGroup.value.trim() === 'thebibliography') {
            break;
          }
        }
        cursor = nestedCommandEnd;
      }
      textEnd = cursor;
      const text = normalizeBibitemText(sourceBytes.slice(keyGroup.next, textEnd));
      if (text.length > 0) {
        addBibitem(key, text, index, textEnd);
      }
      index = textEnd;
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
        if (!isSafeLabelRefKeyValueV1(key)) {
          continue;
        }
        addCiteOccurrence(key, index, citeGroup.next);
      }
      index = citeGroup.next;
      continue;
    }

    index = commandIndex;
  }

  const cites = [...citesByKey.values()].sort((left, right) => left.key.localeCompare(right.key));
  for (const cite of cites) {
    const ordinal = bibOrdinalByKey.get(cite.key);
    if (ordinal) {
      cite.resolved = true;
      cite.ordinal = ordinal;
    }
  }
  if (cites.length > MAX_BIB_ENTRIES_V0) {
    throw new Error(`cites_v1 entries exceed cap ${MAX_BIB_ENTRIES_V0}`);
  }

  const bibByKey = new Map(bibitems.map((entry) => [entry.key, entry]));
  const seenBibKeys = new Set();
  const bibEntriesV1 = [];
  for (const citeEntry of citeEntriesV1) {
    const key = citeEntry.key;
    if (seenBibKeys.has(key)) {
      continue;
    }
    seenBibKeys.add(key);
    const bibitem = bibByKey.get(key);
    if (!bibitem) {
      throw new Error(`bib_v1 unresolved cite key '${key}'`);
    }
    const ordinal = bibEntriesV1.length + 1;
    bibEntriesV1.push({
      key,
      ordinal,
      text_sha256: bibitem.text_sha256,
      source_span: bibitem.source_span,
    });
    if (bibEntriesV1.length > MAX_BIB_ENTRIES_V0) {
      throw new Error(`bib_v1 entries exceed cap ${MAX_BIB_ENTRIES_V0}`);
    }
  }
  const bibOrdinalByResolvedKey = new Map(bibEntriesV1.map((entry) => [entry.key, entry.ordinal]));
  for (const citeEntry of citeEntriesV1) {
    const ordinal = bibOrdinalByResolvedKey.get(citeEntry.key);
    if (ordinal !== undefined) {
      citeEntry.resolved = true;
      citeEntry.ordinal = ordinal;
    }
  }

  return {
    bibitems,
    cites,
    bibEntriesV1,
    citeEntriesV1,
  };
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

function isSafeLabelRefKeyValueV1(value) {
  return typeof value === 'string'
    && value.length > 0
    && !value.includes(' ')
    && !value.includes('..')
    && !value.startsWith('/')
    && /^[A-Za-z0-9:_./-]+$/.test(value);
}

function lineIndexForByteOffsetV1(sourceBytes, offset) {
  let lineIndex = 1;
  for (let index = 0; index < offset && index < sourceBytes.length; index += 1) {
    if (sourceBytes[index] === 0x0a) {
      lineIndex += 1;
    }
  }
  return lineIndex;
}

function extractLabelsAndRefsFromSourceV1(sourceBytes) {
  const labelsByKey = new Map();
  const refsByKey = new Map();
  let nextAnchorId = 1;
  let pendingLabelTarget = null;
  let inFigure = false;
  let index = 0;

  const setPendingHeading = (level, title) => {
    pendingLabelTarget = {
      anchor_id: nextAnchorId,
      kind: 'heading',
      level,
      title: title.length > 0 ? title : null,
    };
    nextAnchorId += 1;
  };

  const setPendingFigure = (title) => {
    pendingLabelTarget = {
      anchor_id: nextAnchorId,
      kind: 'figure',
      level: null,
      title: title.length > 0 ? title : null,
    };
    nextAnchorId += 1;
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
    if (command === 'begin') {
      const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!envGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      const envName = envGroup.value.trim();
      if (envName === 'figure') {
        inFigure = true;
      }
      pendingLabelTarget = null;
      index = envGroup.next;
      continue;
    }
    if (command === 'end') {
      const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!envGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      const envName = envGroup.value.trim();
      if (envName === 'figure') {
        inFigure = false;
      }
      pendingLabelTarget = null;
      index = envGroup.next;
      continue;
    }
    if (command === 'section' || command === 'subsection') {
      const titleGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!titleGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      const level = command === 'section' ? 1 : 2;
      setPendingHeading(level, titleGroup.value.trim());
      index = titleGroup.next;
      continue;
    }
    if (command === 'caption' && inFigure) {
      const captionGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!captionGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      setPendingFigure(captionGroup.value.trim());
      index = captionGroup.next;
      continue;
    }
    if (command === 'label') {
      const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!keyGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      const key = keyGroup.value.trim();
      if (
        pendingLabelTarget
        && isSafeLabelRefKeyValueV1(key)
        && !labelsByKey.has(key)
      ) {
        const keyBytes = Buffer.from(key, 'utf8');
        if (keyBytes.length > MAX_LABEL_VALUE_BYTES_V0) {
          throw new Error(`labels_v1 key exceeds cap ${MAX_LABEL_VALUE_BYTES_V0}`);
        }
        labelsByKey.set(key, {
          key,
          anchor_id: pendingLabelTarget.anchor_id,
          kind: pendingLabelTarget.kind,
          level: pendingLabelTarget.level,
          title: pendingLabelTarget.title,
          source_span: buildSourceSpanV0(sourceBytes, index, keyGroup.next, 'labels_v1'),
        });
        if (labelsByKey.size > MAX_LABEL_ENTRIES_V0) {
          throw new Error(`labels_v1 entries exceed cap ${MAX_LABEL_ENTRIES_V0}`);
        }
      }
      pendingLabelTarget = null;
      index = keyGroup.next;
      continue;
    }
    if (command === 'ref') {
      const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
      if (!keyGroup.ok) {
        index = commandIndex;
        pendingLabelTarget = null;
        continue;
      }
      const key = keyGroup.value.trim();
      if (isSafeLabelRefKeyValueV1(key)) {
        let entry = refsByKey.get(key);
        if (!entry) {
          entry = {
            key,
            occurrences: [],
            resolved: false,
            source_span: buildSourceSpanV0(sourceBytes, index, keyGroup.next, 'refs_v1'),
          };
          refsByKey.set(key, entry);
        }
        entry.occurrences.push({
          line_index: lineIndexForByteOffsetV1(sourceBytes, index),
          anchor_id: null,
        });
        if (entry.occurrences.length > MAX_REF_OCCURRENCES_PER_KEY_V0) {
          throw new Error(`refs_v1 occurrences exceed cap ${MAX_REF_OCCURRENCES_PER_KEY_V0} for key ${key}`);
        }
      }
      pendingLabelTarget = null;
      index = keyGroup.next;
      continue;
    }

    pendingLabelTarget = null;
    index = commandIndex;
  }

  const labels = [...labelsByKey.values()].sort((left, right) => left.key.localeCompare(right.key));
  const refs = [...refsByKey.values()].sort((left, right) => left.key.localeCompare(right.key));
  for (const refEntry of refs) {
    const labelEntry = labelsByKey.get(refEntry.key);
    if (!labelEntry) {
      continue;
    }
    refEntry.resolved = true;
    for (const occurrence of refEntry.occurrences) {
      occurrence.anchor_id = labelEntry.anchor_id;
    }
  }
  if (refs.length > MAX_REF_ENTRIES_V0) {
    throw new Error(`refs_v1 entries exceed cap ${MAX_REF_ENTRIES_V0}`);
  }
  return {
    labels,
    refs,
  };
}

async function emitLabelsTypedArtifactV0(caseOutDir, fixtureBytes) {
  const extracted = extractLabelsAndRefsFromSourceV1(fixtureBytes);
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'labels_v1',
    entries: extracted.labels,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'labels_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitRefsTypedArtifactV0(caseOutDir, fixtureBytes) {
  const extracted = extractLabelsAndRefsFromSourceV1(fixtureBytes);
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'refs_v1',
    entries: extracted.refs,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'refs_v1.json';
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
    schema: 'toc_v1',
    entries: extractTocEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'toc_v1.json';
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

async function emitBibTypedArtifactV1(caseOutDir, bibEntries) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'bib_v1',
    entries: bibEntries,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'bib_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitCiteTypedArtifactV1(caseOutDir, citeEntries) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'cite_v1',
    entries: citeEntries,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'cite_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitBibitemsTypedArtifactV0(caseOutDir, fixtureBytes) {
  const extracted = extractBibitemsAndCitesFromSourceV1(fixtureBytes);
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'bibitems_v1',
    entries: extracted.bibitems,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'bibitems_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitCitesTypedArtifactV0(caseOutDir, fixtureBytes) {
  const extracted = extractBibitemsAndCitesFromSourceV1(fixtureBytes);
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'cites_v1',
    entries: extracted.cites,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'cites_v1.json';
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
    schema: 'graphics_v1',
    entries: extractGraphicsEntriesFromSourceV0(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'graphics_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitMathTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'math_v1',
    entries: extractMathEntriesFromSourceV1(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'math_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitTableTypedArtifactV0(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'table_v1',
    entries: extractTableEntriesFromSourceV1(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'table_v1.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitResourceHintsArtifactV0(caseOutDir, fixtureBytes, mode) {
  const caseId = path.basename(caseOutDir);
  const entries = mode === 'typeset' ? extractResourceHintEntriesFromSourceV0(fixtureBytes, caseId) : [];
  validateResourceHintEntriesV0(entries, fixtureBytes, caseId);
  const payload = {
    version: RESOURCE_HINTS_V0_VERSION,
    resource_hints_v0_version: RESOURCE_HINTS_V0_VERSION,
    schema: 'resource_hints_v0',
    entries,
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

async function emitEmptyResourceHintsArtifactV0(caseOutDir) {
  const payload = {
    version: RESOURCE_HINTS_V0_VERSION,
    resource_hints_v0_version: RESOURCE_HINTS_V0_VERSION,
    schema: 'resource_hints_v0',
    entries: [],
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'resource_hints_v0.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: 0,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitTypedArtifactsV0(
  caseSpec,
  caseOutDir,
  typedArtifacts,
  fixtureBytes,
  inputEntries = [],
) {
  if (caseSpec.id === 'typeset_demo_toc_probe_v0') {
    typedArtifacts.toc = await emitTocTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_labels_probe_v0') {
    typedArtifacts.labels = await emitLabelsTypedArtifactV0(caseOutDir, fixtureBytes);
    typedArtifacts.refs = await emitRefsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_bib_probe_v0') {
    const extractedBib = extractBibitemsAndCitesFromSourceV1(fixtureBytes);
    typedArtifacts.bib = await emitBibTypedArtifactV1(caseOutDir, extractedBib.bibEntriesV1);
    typedArtifacts.cite = await emitCiteTypedArtifactV1(caseOutDir, extractedBib.citeEntriesV1);
  }
  if (caseSpec.id === 'typeset_demo_minimal_v0') {
    typedArtifacts.bibitems = await emitBibitemsTypedArtifactV0(caseOutDir, fixtureBytes);
    typedArtifacts.cites = await emitCitesTypedArtifactV0(caseOutDir, fixtureBytes);
    typedArtifacts.math = await emitMathTypedArtifactV0(caseOutDir, fixtureBytes);
    typedArtifacts.table = await emitTableTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_hyperref_probe_v0') {
    typedArtifacts.hyperref = await emitHyperrefTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (
    caseSpec.id === 'typeset_demo_pkgopt_probe_v0'
    || caseSpec.id === 'typeset_demo_pkgopt_require_pass_probe_v0'
    || caseSpec.id === 'typeset_demo_class_options_probe_v0'
    || caseSpec.id === 'typeset_demo_documentclass_opts_probe_v0'
    || caseSpec.id === 'typeset_demo_documentclass_opts_multi_probe_v0'
    || caseSpec.id === 'typeset_demo_passoptionstoclass_probe_v0'
    || caseSpec.id === 'typeset_demo_usepackage_opts_multi_probe_v0'
    || caseSpec.id === 'typeset_demo_usepackage_multipackage_probe_v0'
  ) {
    typedArtifacts.pkgopt = await emitPkgoptTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_graphics_probe_v0') {
    typedArtifacts.graphics = await emitGraphicsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.mode === 'typeset' && Array.isArray(inputEntries) && inputEntries.length > 0) {
    typedArtifacts.input = await emitInputTypedArtifactV1(caseOutDir, inputEntries);
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
    if (
      payload?.version !== RESOURCE_HINTS_V0_VERSION
      || payload?.resource_hints_v0_version !== RESOURCE_HINTS_V0_VERSION
      || payload?.schema !== 'resource_hints_v0'
      || !Array.isArray(payload?.entries)
    ) {
      throw new Error(`invalid resource_hints_v0 artifact for case ${caseId}`);
    }
    const fixtureBytes = await readFile(path.join(outDir, caseId, 'main.tex'));
    validateResourceHintEntriesV0(payload.entries, fixtureBytes, caseId);

    for (const entry of payload.entries) {
      const hintType = typeof entry?.hint_type === 'string' ? entry.hint_type : '';
      const value = typeof entry?.value === 'string' ? entry.value : '';
      const sourceSpan = entry?.source_span;
      if (!hintType || !value) {
        continue;
      }
      const dedupeKey = `${caseId}\x1f${hintType}\x1f${value}\x1f${sourceSpan.start_byte}\x1f${sourceSpan.end_byte}`;
      if (seen.has(dedupeKey)) {
        continue;
      }
      seen.add(dedupeKey);
      entries.push({
        kind: 'resource_hint',
        case_id: caseId,
        hint_type: hintType,
        value,
        source_span: {
          start_byte: sourceSpan.start_byte,
          end_byte: sourceSpan.end_byte,
        },
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
    version: RESOURCE_HINTS_V0_VERSION,
    resource_hints_v0_version: RESOURCE_HINTS_V0_VERSION,
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
    if (hintType === 'tex_input' || hintType === 'tex_include' || hintType === 'tex_includeonly') {
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

  const inputRelpath = typedArtifacts?.input?.artifact_relpath;
  if (typedArtifacts?.input?.present === true && typeof inputRelpath === 'string' && inputRelpath.length > 0) {
    const inputPayload = JSON.parse((await readFile(path.join(caseOutDir, inputRelpath))).toString('utf8'));
    const inputEntries = Array.isArray(inputPayload?.entries) ? inputPayload.entries : [];
    for (const entry of inputEntries) {
      if (typeof entry?.value !== 'string' || entry.value.length === 0) {
        continue;
      }
      const hintType = entry?.command === 'include' ? 'tex_include' : 'tex_input';
      addTexmfRequest(entry.value, 'tex', hintType);
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

function buildResolverOutcomeEntryV0(request, resolution) {
  return {
    kind: request.kind,
    format: request.format,
    name: request.name,
    variant: request.variant,
    hint_type: request.hint_type ?? null,
    stable_id: resolution.stable_id,
    sha256: resolution.sha256,
    cache_hit: resolution.cache_hit,
  };
}

function buildResolverMissingEntryV0(request, resolution) {
  return {
    kind: request.kind,
    format: request.format,
    name: request.name,
    variant: request.variant,
    hint_type: request.hint_type ?? null,
    cache_hit: resolution.cache_hit,
  };
}

async function resolveRequestsWithResolverV0(resolver, requests) {
  const resolvedResources = [];
  const missingResources = [];
  for (const request of requests) {
    const resolution = await resolver.resolve({
      kind: request.kind,
      format: request.format,
      name: request.name,
      variant: request.variant,
      resolver_id: resolver.resolverId,
    });
    if (resolution.tag === 'Found') {
      resolvedResources.push(buildResolverOutcomeEntryV0(request, resolution));
      continue;
    }
    missingResources.push(buildResolverMissingEntryV0(request, resolution));
  }
  return {
    resolvedResources,
    missingResources,
  };
}

function normalizeStoreRequestFromEntryV0(entry) {
  const kind = typeof entry?.kind === 'string' ? entry.kind : '';
  const format = typeof entry?.format === 'string' ? entry.format : '';
  const name = typeof entry?.name === 'string' ? entry.name : '';
  const variant = typeof entry?.variant === 'string' ? entry.variant : '';
  const safeVariant = variant === '' || isSafeResolverTokenV0(variant);
  if (!isSafeResolverTokenV0(kind) || !isSafeResolverTokenV0(format) || !isSafeResolverTokenV0(name) || !safeVariant) {
    return null;
  }
  return { kind, format, name, variant };
}

async function loadStoreRequestsV0(storeDir) {
  const indexPath = path.join(storeDir, 'index.json');
  const indexBytes = await readFile(indexPath).catch(() => null);
  if (!indexBytes) {
    return [];
  }
  let parsed;
  try {
    parsed = JSON.parse(indexBytes.toString('utf8'));
  } catch {
    return [];
  }
  const entries = Array.isArray(parsed?.entries) ? parsed.entries : [];
  const requestsByKey = new Map();
  for (const entry of entries) {
    const normalized = normalizeStoreRequestFromEntryV0(entry);
    if (!normalized) {
      continue;
    }
    requestsByKey.set(resolverRequestKeyV0(normalized), normalized);
  }
  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
}

function mergeStoreRequestsV0(existingRequests, missingRequests) {
  const requestsByKey = new Map();
  for (const request of existingRequests) {
    requestsByKey.set(resolverRequestKeyV0(request), request);
  }
  for (const request of missingRequests) {
    const normalized = normalizeStoreRequestFromEntryV0(request);
    if (!normalized) {
      continue;
    }
    requestsByKey.set(resolverRequestKeyV0(normalized), normalized);
  }
  return [...requestsByKey.values()].sort((left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)));
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
  deltaPolicy,
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
  let inputInclusionGraph = {
    entries: [],
    mounted_files: [],
    resolver_requests: [],
  };

  try {
    if (ctx.mountReset() !== 0) {
      throw new Error('mount_reset failed');
    }
    if (helpers.addMountedFile('main.tex', fixtureBytes, `${caseSpec.id}_main`) !== 0) {
      throw new Error('mount_add_file(main.tex) failed');
    }
    if (caseSpec.mode === 'typeset') {
      inputInclusionGraph = await collectInputIncludeGraphV1(fixtureBytes, resolver, caseSpec);
      for (const [mountPath, mountBytes] of inputInclusionGraph.mounted_files) {
        if (mountPath === 'main.tex') {
          continue;
        }
        const mountLabel = `${caseSpec.id}_${mountPath.replaceAll('/', '__')}`;
        if (helpers.addMountedFile(mountPath, mountBytes, mountLabel) !== 0) {
          throw new Error(`mount_add_file(${mountPath}) failed`);
        }
      }
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

  const resolverRequestsByKey = new Map();
  const rootResolverRequest = {
    kind: 'texmf',
    format: 'tex',
    name: caseSpec.id,
    variant: caseSpec.mode,
    hint_type: 'entrypoint',
  };
  resolverRequestsByKey.set(resolverRequestKeyV0(rootResolverRequest), rootResolverRequest);
  for (const request of inputInclusionGraph.resolver_requests) {
    resolverRequestsByKey.set(resolverRequestKeyV0(request), request);
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
    input_sha256: {
      main_tex: sha256HexV0(fixtureBytes),
    },
    resolver_id: resolver.resolverId,
    resolved_resources: [],
    missing_resources: [],
    resolved_resources_count: 0,
    missing_resources_count: 0,
    typed_artifacts_version: TYPED_ARTIFACTS_VERSION_V0,
    typed_artifacts: buildTypedArtifactsPlaceholderV0(),
  };
  try {
    summary.resource_hints_v0 = await emitResourceHintsArtifactV0(caseOutDir, fixtureBytes, caseSpec.mode);
  } catch (error) {
    summary.resource_hints_v0 = await emitEmptyResourceHintsArtifactV0(caseOutDir);
    caseStatus = STATUS_INVALID_V0;
    const message = error instanceof Error ? error.message : String(error);
    errorMessage = errorMessage ? `${errorMessage}; ${message}` : message;
    summary.status = caseStatus;
  }
  await emitTypedArtifactsV0(
    caseSpec,
    caseOutDir,
    summary.typed_artifacts,
    fixtureBytes,
    inputInclusionGraph.entries,
  );
  const typedArtifactRequests = await collectResolverRequestsFromResourceHintsV0(
    caseSpec,
    caseOutDir,
    summary.resource_hints_v0,
  );
  for (const request of typedArtifactRequests) {
    resolverRequestsByKey.set(resolverRequestKeyV0(request), request);
  }
  const resolverRequests = [...resolverRequestsByKey.values()].sort(
    (left, right) => resolverRequestKeyV0(left).localeCompare(resolverRequestKeyV0(right)),
  );
  const resolverOutcomes = await resolveRequestsWithResolverV0(resolver, resolverRequests);
  summary.resolved_resources = resolverOutcomes.resolvedResources;
  summary.missing_resources = resolverOutcomes.missingResources;
  summary.resolved_resources_count = resolverOutcomes.resolvedResources.length;
  summary.missing_resources_count = resolverOutcomes.missingResources.length;
  summary.expected_vs_actual = expectedVsActualV0(caseSpec.expected_status, summary.status);
  summary.baseline_metrics_v1 = buildBaselineMetricsV1(xdvBytes, pdfBytes, logBytes, summary);
  summary.baseline_match = await computeBaselineMatchV0(caseSpec.id, summary.artifact_sha256, baselineDir);
  summary.baseline_cmp_v1 = classifyBaselineCmpV1(caseSpec, summary, deltaPolicy, baselineDir);
  if (errorMessage) {
    summary.error = errorMessage;
  }

  await writeFile(path.join(caseOutDir, 'main.xdv'), xdvBytes);
  await writeFile(path.join(caseOutDir, 'main.pdf'), pdfBytes);
  await writeFile(path.join(caseOutDir, 'compile.log.bin'), logBytes);
  await writeFile(path.join(caseOutDir, 'summary.json'), `${JSON.stringify(summary, null, 2)}\n`);

  return summary;
}

async function runWasmFixtureGalleryV1() {
  const outDir = path.resolve(process.argv[2] ?? path.join(rootDir, 'target', 'wasm_fixture_gallery_v0'));
  const storeDir = path.resolve(process.env.TEXLIVE_STORE_DIR_V0 ?? path.join(rootDir, 'target', 'texlive_store_v0'));
  const baselineDir = process.env.TEXLIVE_BASELINE_DIR
    ? path.resolve(process.env.TEXLIVE_BASELINE_DIR)
    : '';
  const deltaPolicyPath = path.resolve(
    process.env.WASM_GALLERY_DELTA_POLICY_V1
      ?? path.join(rootDir, 'scripts', 'wasm_fixture_gallery_delta_policy_v1.json'),
  );
  const onDemandEnabled = parseBoolEnvV0(
    process.env.WASM_GALLERY_ENABLE_ONDEMAND_V1 ?? process.env.WASM_GALLERY_ONDEMAND_ENABLE_V1,
  );
  const onDemandMaxItersRaw = process.env.WASM_GALLERY_ONDEMAND_MAX_ITERS_V1 ?? `${DEFAULT_ONDEMAND_FIXEDPOINT_MAX_ITERS_V1}`;
  const onDemandMaxIters = Number.parseInt(onDemandMaxItersRaw, 10);
  if (!Number.isInteger(onDemandMaxIters) || onDemandMaxIters <= 0 || onDemandMaxIters > 5) {
    throw new Error(`WASM_GALLERY_ONDEMAND_MAX_ITERS_V1 must be an integer in [1,5], got: ${onDemandMaxItersRaw}`);
  }
  const onDemandEndpoint = typeof process.env.TEXLIVE_ENDPOINT === 'string'
    ? process.env.TEXLIVE_ENDPOINT.trim()
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
  const resolverBackend = process.env.TEXLIVE_RESOLVER_BACKEND_V0;
  const resolver = await createOnDemandResolverV0({
    backend: process.env.TEXLIVE_RESOLVER_BACKEND_V0,
    endpoint: onDemandEndpoint,
    rootDir,
    storeDir,
  });
  const deltaPolicy = await loadDeltaPolicyV1(deltaPolicyPath);
  const configHash = buildConfigHashV0(cases, sourceDateEpoch, resolver.resolverId, deltaPolicy.sha256);
  const buildCaseResolver = () =>
    createOnDemandResolverV0({
      backend: resolverBackend,
      endpoint: onDemandEndpoint,
      rootDir,
      storeDir,
    });

  await rm(outDir, { recursive: true, force: true });
  await mkdir(outDir, { recursive: true });

  const ctx = await createCtx(rootDir);
  const mem = createMemHelpers(ctx);
  const helpers = createAssertHelpers(ctx, mem);
  const summaries = [];
  for (const caseSpec of cases) {
    const initialSummary = await runCaseV0(
      ctx,
      mem,
      helpers,
      outDir,
      caseSpec,
      sourceDateEpoch,
      engineRev,
      configHash,
      await buildCaseResolver(),
      baselineDir,
      deltaPolicy,
    );
    let finalSummary = initialSummary;
    const missingBefore = Number(initialSummary.missing_resources_count ?? 0);
    const resolvedBefore = Number(initialSummary.resolved_resources_count ?? 0);
    const onDemandTriggered = onDemandEnabled && caseSpec.ondemand_opt_in === true && missingBefore > 0;
    const onDemandAttempted = onDemandTriggered && onDemandEndpoint.length > 0;
    let onDemandIterations = 0;
    let onDemandStoreFound = 0;
    let onDemandStoreMissing = missingBefore;
    if (onDemandAttempted) {
      for (let iter = 1; iter <= onDemandMaxIters; iter += 1) {
        const missingRequests = Array.isArray(finalSummary.missing_resources) ? finalSummary.missing_resources : [];
        if (missingRequests.length === 0) {
          break;
        }
        const existingStoreRequests = await loadStoreRequestsV0(storeDir);
        const mergedRequests = mergeStoreRequestsV0(existingStoreRequests, missingRequests);
        if (mergedRequests.length === 0) {
          break;
        }
        const caseOutDir = path.join(outDir, caseSpec.id);
        const requestListPath = path.join(caseOutDir, `ondemand_requests_iter_${iter}.json`);
        await writeFile(
          requestListPath,
          `${JSON.stringify({ version: 1, requests: mergedRequests }, null, 2)}\n`,
        );
        const storeResult = await generateTexliveStoreV0({
          rootDir,
          requestListPath,
          storeDir,
          backend: 'endpoint_v0',
          endpoint: onDemandEndpoint,
          sourceDateEpoch,
        });
        onDemandIterations = iter;
        onDemandStoreFound = Number(storeResult.foundCount ?? 0);
        onDemandStoreMissing = Number(storeResult.missingCount ?? 0);
        const rerunSummary = await runCaseV0(
          ctx,
          mem,
          helpers,
          outDir,
          caseSpec,
          sourceDateEpoch,
          engineRev,
          configHash,
          await buildCaseResolver(),
          baselineDir,
          deltaPolicy,
        );
        const improved = rerunSummary.resolved_resources_count > finalSummary.resolved_resources_count
          || rerunSummary.missing_resources_count < finalSummary.missing_resources_count;
        finalSummary = rerunSummary;
        if (!improved || rerunSummary.missing_resources_count === 0) {
          break;
        }
      }
    }
    finalSummary.missing_before = missingBefore;
    finalSummary.missing_after = Number(finalSummary.missing_resources_count ?? 0);
    finalSummary.resolved_resources_before = resolvedBefore;
    finalSummary.resolved_resources_after = Number(finalSummary.resolved_resources_count ?? 0);
    finalSummary.status_for_report = finalSummary.expected_vs_actual === 'MATCH'
      ? finalSummary.status
      : STATUS_MISMATCH_V0;
    finalSummary.ondemand_v1 = {
      enabled: onDemandEnabled,
      case_opt_in: caseSpec.ondemand_opt_in === true,
      triggered: onDemandTriggered,
      attempted: onDemandAttempted,
      endpoint_present: onDemandEndpoint.length > 0,
      max_iters: onDemandMaxIters,
      iterations: onDemandIterations,
      store_found: onDemandStoreFound,
      store_missing: onDemandStoreMissing,
      missing_before: missingBefore,
      missing_after: finalSummary.missing_after,
      resolved_before: resolvedBefore,
      resolved_after: finalSummary.resolved_resources_after,
    };
    const caseOutDir = path.join(outDir, caseSpec.id);
    await writeFile(path.join(caseOutDir, 'summary.json'), `${JSON.stringify(finalSummary, null, 2)}\n`);
    summaries.push(finalSummary);
  }

  const report = {
    engine_rev: engineRev,
    source_date_epoch: sourceDateEpoch,
    resolver_id: resolver.resolverId,
    store_dir: storeDir,
    baseline_dir: baselineDir || null,
    delta_policy_v1: {
      path: deltaPolicy.path,
      sha256: deltaPolicy.sha256,
      ok_cases_require_match: deltaPolicy.ok_cases_require_match,
      ok_allowlist_case_count: Object.keys(deltaPolicy.ok_case_allowlist).length,
      non_ok_mismatch_class: deltaPolicy.non_ok_mismatch_class,
      ok_allowlisted_mismatch_class: deltaPolicy.ok_allowlisted_mismatch_class,
      missing_baseline_class: deltaPolicy.missing_baseline_class,
      skip_class: deltaPolicy.skip_class,
      metrics_thresholds: deltaPolicy.metrics_thresholds,
    },
    manifest_path: manifestPath,
    config_hash: configHash,
    ondemand_v1: {
      enabled: onDemandEnabled,
      endpoint_present: onDemandEndpoint.length > 0,
      max_iters: onDemandMaxIters,
    },
    typed_artifacts_version: TYPED_ARTIFACTS_VERSION_V0,
    case_count: summaries.length,
    missing_before_total: summaries.reduce(
      (sum, summary) => sum + Number(summary.missing_before ?? summary.missing_resources_count ?? 0),
      0,
    ),
    missing_after_total: summaries.reduce(
      (sum, summary) => sum + Number(summary.missing_after ?? summary.missing_resources_count ?? 0),
      0,
    ),
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
          main_tex: summary.input_sha256?.main_tex ?? null,
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
      config_hash: summary.config_hash,
      input_sha256: summary.input_sha256,
      baseline_match: summary.baseline_match,
      baseline_cmp_v1: summary.baseline_cmp_v1,
      resolved_resources_count: summary.resolved_resources_count,
      missing_before: Number(summary.missing_before ?? summary.missing_resources_count ?? 0),
      missing_after: Number(summary.missing_after ?? summary.missing_resources_count ?? 0),
      ondemand_v1: summary.ondemand_v1,
      typed_artifacts_presence: Object.fromEntries(
        TYPED_ARTIFACT_KEYS_V0.map((key) => [key, summary.typed_artifacts?.[key]?.present === true]),
      ),
      status: summary.status_for_report ?? summary.status,
      artifact_sha256: summary.artifact_sha256,
    })),
  };
  for (const summary of summaries) {
    if (summary.typed_artifacts_version !== TYPED_ARTIFACTS_VERSION_V0) {
      throw new Error(
        `typed_artifacts_version mismatch for case ${summary.case_id}: expected ${TYPED_ARTIFACTS_VERSION_V0}, got ${summary.typed_artifacts_version}`,
      );
    }
    const cmpClass = summary?.baseline_cmp_v1?.class;
    if (!BASELINE_CMP_CLASS_ALLOWLIST_V1.has(cmpClass)) {
      throw new Error(`baseline_cmp_v1 class invalid for case ${summary.case_id}`);
    }
    const cmpReasons = summary?.baseline_cmp_v1?.reasons;
    if (!Array.isArray(cmpReasons) || cmpReasons.length === 0) {
      throw new Error(`baseline_cmp_v1 reasons missing for case ${summary.case_id}`);
    }
    if (
      deltaPolicy.ok_cases_require_match
      && baselineDir
      && summary.status === STATUS_OK_V0
      && cmpClass !== BASELINE_CMP_CLASS_MATCH_V1
      && !deltaPolicy.ok_case_allowlist[summary.case_id]
    ) {
      throw new Error(`OK case baseline_cmp_v1 must be MATCH for ${summary.case_id}`);
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

export {
  sha256HexV0,
  readArtifactBytesV0,
  readLogBytesV0,
  entrypointSetOkV0,
  loadGalleryManifestV0,
  loadFixtureCasesV0,
  loadDeltaPolicyV1,
  classifyBaselineCmpV1,
  buildBaselineMetricsV1,
  computeBaselineMatchV0,
  buildConfigHashV0,
  buildTypedArtifactsPlaceholderV0,
  mergeStoreRequestsV0,
  normalizeStoreRequestFromEntryV0,
  loadStoreRequestsV0,
  runCaseV0,
  runWasmFixtureGalleryV1,
};

const isDirectRun = process.argv[1]
  ? import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href
  : false;

if (isDirectRun) {
  runWasmFixtureGalleryV1().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: wasm fixture gallery v0: ${message}`);
    process.exit(1);
  });
}
