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
const TYPED_ARTIFACT_KEYS_V0 = ['toc', 'labels', 'refs', 'pageref', 'bib', 'cite', 'bibitems', 'cites', 'hyperref', 'pkgopt', 'packages', 'graphics', 'float', 'input', 'math', 'table'];
const TYPED_ARTIFACTS_VERSION_V0 = 1;
const MAX_TOC_ENTRIES_V0 = 256;
const MAX_TOC_TITLE_BYTES_V0 = 256;
const MAX_LABEL_ENTRIES_V0 = 256;
const MAX_LABEL_VALUE_BYTES_V0 = 256;
const MAX_REF_ENTRIES_V0 = 256;
const MAX_REF_OCCURRENCES_PER_KEY_V0 = 256;
const MAX_PAGEREF_ENTRIES_V2 = 256;
const MAX_PAGEREF_OCCURRENCES_PER_KEY_V2 = 256;
const MAX_BIB_ENTRIES_V0 = 256;
const MAX_BIB_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_ENTRIES_V0 = 256;
const MAX_PKGOPT_VALUE_BYTES_V0 = 256;
const MAX_PKGOPT_OPTIONS_PER_ENTRY_V0 = 64;
const MAX_PACKAGES_ENTRIES_V1 = 256;
const MAX_PACKAGES_NAME_BYTES_V1 = 256;
const MAX_PACKAGES_OPTIONS_PER_ENTRY_V1 = 64;
const MAX_PACKAGES_OPTION_BYTES_V1 = 256;
const MAX_GRAPHICS_ENTRIES_V0 = 256;
const MAX_GRAPHICS_PATH_BYTES_V0 = 256;
const DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2 = 180.0;
const DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2 = 120.0;
const MAX_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2 = 468.0;
const MAX_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2 = 288.0;
const MAX_FLOAT_ENTRIES_V0 = 256;
const MAX_FLOAT_CAPTION_SUMMARY_BYTES_V0 = 128;
const MAX_INPUT_ENTRIES_V1 = 512;
const MAX_INPUT_INCLUDE_DEPTH_V1 = 32;
const MAX_MATH_ENTRIES_V0 = 256;
const MAX_MATH_PAYLOAD_BYTES_V0 = 1024;
const MAX_MATH_PAYLOAD_PREVIEW_BYTES_V2 = 96;
const MAX_TABLE_ENTRIES_V0 = 64;
const MAX_TABLE_ROWS_PER_ENTRY_V0 = 64;
const MAX_TABLE_COLS_PER_ENTRY_V0 = 16;
const MAX_RESOURCE_HINT_ENTRIES_V0 = 512;
const MAX_RESOURCE_HINT_VALUE_BYTES_V0 = 256;
const RESOURCE_HINTS_V0_VERSION = 1;
const DVI_PRE_OPCODE_V2 = 247;
const DVI_BOP_OPCODE_V2 = 139;
const DVI_EOP_OPCODE_V2 = 140;
const DVI_POST_OPCODE_V2 = 248;
const DVI_POSTPOST_OPCODE_V2 = 249;
const DVI_FNT_DEF1_OPCODE_V2 = 243;
const DVI_FNT_NUM_0_OPCODE_V2 = 171;
const DVI_RIGHT3_OPCODE_V2 = 145;
const DVI_DOWN3_OPCODE_V2 = 160;
const DVI_ID_V2 = 2;
const DVI_TRAILER_BYTE_V2 = 223;
const DVI_NUM_V2 = 25_400_000;
const DVI_DEN_V2 = 473_628_672;
const DVI_MAG_V2 = 1000;
const DVI_FONT_NAME_V2 = 'carreltex-v0';
const TOC_PLACEHOLDER_MARKER_V2 = '!toc';
const TOC_ENTRY_LINE_PREFIX_V2 = '!toc ';
const SECTION_HEADING_PREFIX_V2 = '@S ';
const SUBSECTION_HEADING_PREFIX_V2 = '@s ';
const FIGURE_BOX_LINE_V2 = '!gbox';
const FOOTNOTE_LINE_PREFIX_V2 = '!f ';
const HREF_URL_LINE_PREFIX_V2 = '!u ';
const LABEL_LINE_PREFIX_V2 = '!l ';
const REF_LINE_PREFIX_V2 = '!r ';
const PAGEREF_LINE_PREFIX_V2 = '!pr ';
const REF_ANCHOR_LINK_LINE_PREFIX_V2 = '!ra ';
const PAGEREF_PAGE_LINK_LINE_PREFIX_V2 = '!rp ';
const EQUATION_LINE_PREFIX_V2 = '!eq ';
const BIBITEM_LINE_PREFIX_V2 = '!b ';
const CITE_LINE_PREFIX_V2 = '!c ';
const TABLE_SPEC_LINE_PREFIX_V2 = '!ts ';
const TABLE_ROW_LINE_PREFIX_V2 = '!t ';
const FIGURE_CAPTION_LINE_PREFIX_V2 = '!gcap ';
const FIGURE_IMAGE_LINE_PREFIX_V2 = '!gimg ';
const FIGURE_TOP_PLACEMENT_HINT_V2 = 't';
const DISPLAY_MATH_PLACEHOLDER_SHORT_V2 = 'MATH DISPLAY';
const DISPLAY_MATH_PLACEHOLDER_MEDIUM_V2 = 'MATH DISPLAY MEDIUM';
const DISPLAY_MATH_PLACEHOLDER_LONG_V2 = 'MATH DISPLAY LONG FORM';
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
const PACKAGE_ARTIFACT_CASE_IDS_V1 = new Set([
  'typeset_demo_hyperref_probe_v0',
  'typeset_demo_hyperref_links_probe_v0',
  'typeset_demo_package_require_probe_v0',
  'typeset_demo_usepackage_opts_multi_probe_v0',
  'typeset_demo_usepackage_multipackage_probe_v0',
  'typeset_demo_usepackage_capture_probe_v1',
  'typeset_demo_usepackage_multi_capture_probe_v1',
  'typeset_demo_usepackage_opts_normalize_probe_v1',
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

import {
  loadGalleryManifestV0,
  loadFixtureCasesV0,
} from './case_registry_v0.mjs';
import {
  buildTypedArtifactsPlaceholderV0,
  emitResourceHintsArtifactV0,
  emitEmptyResourceHintsArtifactV0,
  emitTypedArtifactsV0,
  buildResourceHintsRollupV0,
  collectInputIncludeGraphV1,
} from './typed_artifacts_v0.mjs';
import {
  resolverRequestKeyV0,
  collectResolverRequestsFromResourceHintsV0,
  collectResolverRequestsFromTypedArtifactsV0,
  computeBaselineMatchV0,
  resolveRequestsWithResolverV0,
  normalizeStoreRequestFromEntryV0,
  loadStoreRequestsV0,
  mergeStoreRequestsV0,
} from './request_mapping_v0.mjs';
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
      compileCode = ctx.compileMainTypeset();
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
  try {
    await emitTypedArtifactsV0(
      caseSpec,
      caseOutDir,
      summary.typed_artifacts,
      fixtureBytes,
      inputInclusionGraph.mounted_files,
      inputInclusionGraph.entries,
      xdvBytes,
    );
  } catch (error) {
    caseStatus = STATUS_INVALID_V0;
    summary.status = caseStatus;
    summary.typed_artifacts = buildTypedArtifactsPlaceholderV0();
    const message = error instanceof Error ? error.message : String(error);
    const typedArtifactsMessage = `typed_artifacts: ${message}`;
    errorMessage = errorMessage ? `${errorMessage}; ${typedArtifactsMessage}` : typedArtifactsMessage;
  }
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

  const unexpectedFail = summaries.find(
    (summary) => summary.status === STATUS_FAIL_V0 && summary.expected_status !== STATUS_FAIL_V0,
  );
  if (unexpectedFail) {
    throw new Error(`gallery contains unexpected FAIL case: ${unexpectedFail.case_id}`);
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
