import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';

const STATUS_OK_V0 = 'OK';
const STATUS_NI_V0 = 'NI';
const STATUS_INVALID_V0 = 'INVALID';
const STATUS_FAIL_V0 = 'FAIL';
const TYPED_ARTIFACT_KEYS_V0 = ['toc', 'labels', 'refs', 'pageref', 'bib', 'cite', 'bibitems', 'cites', 'hyperref', 'pkgopt', 'packages', 'graphics', 'float', 'input', 'math', 'table'];
const TYPED_ARTIFACTS_VERSION_V0 = 1;
const RESOURCE_HINTS_V0_VERSION = 1;
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
const MAX_TABLE_CELL_BYTES_V1 = 512;
const MAX_TABLE_ENTRIES_V1 = 32;
const MAX_TABLE_COLUMNS_V1 = 16;
const MAX_TABLE_ROWS_V1 = 256;
const MAX_TABLE_TOTAL_WIDTH_PT_V1 = 468;
const MAX_TABLE_LINE_BYTES_V1 = 4096;
const MAX_HYPERREF_ENTRIES_V0 = 128;
const MAX_MATH_ENTRIES_V2 = 64;
const MAX_FLOAT_TEXT_BYTES_V0 = 256;
const MAX_INPUT_PATH_BYTES_V1 = 160;
const MAX_INPUT_GRAPH_DEPTH_V1 = 16;
const MAX_INPUT_GRAPH_NODES_V1 = 256;
const MAX_MATH_PAYLOAD_BYTES_V2 = 512;
const DISPLAY_MATH_SHORT_MAX_PAYLOAD_BYTES_V2 = 24;
const DISPLAY_MATH_MEDIUM_MAX_PAYLOAD_BYTES_V2 = 72;
const DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2 = 180;
const DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2 = 120;
const MAX_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2 = 468;
const MAX_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2 = 288;
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

function resolverRequestKeyV0(request) {
  return `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
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
        throw new Error(`toc_v2 title exceeds cap ${MAX_TOC_TITLE_BYTES_V0}`);
      }
      const anchorId = `h${entries.length + 1}`;
      entries.push({
        level,
        title: titleGroup.value,
        anchor_id: anchorId,
        source_span: buildSourceSpanV0(sourceBytes, index, titleGroup.next, 'toc_v2'),
      });
      if (entries.length > MAX_TOC_ENTRIES_V0) {
        throw new Error(`toc_v2 entries exceed cap ${MAX_TOC_ENTRIES_V0}`);
      }
    }

    index = titleGroup.next;
  }
  return entries;
}

function parseTocAnchorIdTagV2(anchorId) {
  if (typeof anchorId !== 'string') {
    return null;
  }
  const match = /^h([1-9]\d*)$/.exec(anchorId);
  if (!match) {
    return null;
  }
  const parsed = Number.parseInt(match[1], 10);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    return null;
  }
  return parsed;
}

function normalizeTocLinkWrappedTitleV2(title) {
  if (typeof title !== 'string') {
    return '';
  }
  const trimmed = title.trim();
  if (trimmed.startsWith('<') && trimmed.endsWith('>') && trimmed.length >= 2) {
    return trimmed.slice(1, -1).trim();
  }
  return trimmed;
}

function isStyleMarkerByteV2(byte) {
  return byte === 0x5b
    || byte === 0x5d
    || byte === 0x7b
    || byte === 0x7d
    || byte === 0x3c
    || byte === 0x3e;
}

function readU8V2(bytes, state) {
  if (state.index >= bytes.length) {
    throw new Error('toc_v2 dvi parse overflow (u8)');
  }
  const value = bytes[state.index];
  state.index += 1;
  return value;
}

function readI24V2(bytes, state) {
  if (state.index + 3 > bytes.length) {
    throw new Error('toc_v2 dvi parse overflow (i24)');
  }
  const b0 = bytes[state.index];
  const b1 = bytes[state.index + 1];
  const b2 = bytes[state.index + 2];
  state.index += 3;
  const raw = (b0 << 16) | (b1 << 8) | b2;
  return (raw & 0x80_0000) !== 0 ? (raw | ~0x00ff_ffff) : raw;
}

function readI32V2(bytes, state) {
  if (state.index + 4 > bytes.length) {
    throw new Error('toc_v2 dvi parse overflow (i32)');
  }
  const value = (bytes[state.index] << 24)
    | (bytes[state.index + 1] << 16)
    | (bytes[state.index + 2] << 8)
    | bytes[state.index + 3];
  state.index += 4;
  return value;
}

function readU32V2(bytes, state) {
  if (state.index + 4 > bytes.length) {
    throw new Error('toc_v2 dvi parse overflow (u32)');
  }
  const value = ((bytes[state.index] * 0x1000000) >>> 0)
    + (bytes[state.index + 1] << 16)
    + (bytes[state.index + 2] << 8)
    + bytes[state.index + 3];
  state.index += 4;
  return value >>> 0;
}

function expectByteV2(bytes, state, expected, context) {
  const got = readU8V2(bytes, state);
  if (got !== expected) {
    throw new Error(`toc_v2 dvi parse expected ${context}=${expected}, got ${got}`);
  }
}

function expectU32V2(bytes, state, expected, context) {
  const got = readU32V2(bytes, state);
  if (got !== expected) {
    throw new Error(`toc_v2 dvi parse expected ${context}=${expected}, got ${got}`);
  }
}

function parseDviTextPagesForTocV2(xdvBytes) {
  const bytes = toUint8ArrayV0(xdvBytes);
  if (bytes.length === 0 || bytes.length % 4 !== 0) {
    throw new Error('toc_v2 requires non-empty 4-byte aligned main.xdv');
  }
  const state = { index: 0 };
  expectByteV2(bytes, state, DVI_PRE_OPCODE_V2, 'PRE');
  expectByteV2(bytes, state, DVI_ID_V2, 'DVI_ID');
  expectU32V2(bytes, state, DVI_NUM_V2, 'NUM');
  expectU32V2(bytes, state, DVI_DEN_V2, 'DEN');
  expectU32V2(bytes, state, DVI_MAG_V2, 'MAG');
  const commentLen = readU8V2(bytes, state);
  if (commentLen !== 0) {
    throw new Error('toc_v2 expects empty DVI comment');
  }

  const pages = [];
  let previousBopOffset = null;
  let pageCount = 0;
  let lastBopOffset = 0;

  while (state.index < bytes.length) {
    const opcode = bytes[state.index];
    if (opcode === DVI_POST_OPCODE_V2) {
      break;
    }
    if (opcode !== DVI_BOP_OPCODE_V2) {
      throw new Error(`toc_v2 dvi parse expected BOP, got opcode=${opcode}`);
    }
    const bopOffset = state.index;
    lastBopOffset = bopOffset >>> 0;
    state.index += 1;
    for (let counter = 0; counter < 10; counter += 1) {
      if (readI32V2(bytes, state) !== 0) {
        throw new Error('toc_v2 dvi parse expects zero bop counters');
      }
    }
    const prevBop = readI32V2(bytes, state);
    if (previousBopOffset === null) {
      if (prevBop !== -1) {
        throw new Error(`toc_v2 dvi parse expected first prev_bop=-1, got ${prevBop}`);
      }
    } else if (prevBop !== previousBopOffset) {
      throw new Error(`toc_v2 dvi parse expected prev_bop=${previousBopOffset}, got ${prevBop}`);
    }

    expectByteV2(bytes, state, DVI_FNT_DEF1_OPCODE_V2, 'FNT_DEF1');
    expectByteV2(bytes, state, 0, 'FONT_ID');
    expectU32V2(bytes, state, 0, 'FONT_CHECKSUM');
    expectU32V2(bytes, state, 0, 'FONT_SCALE');
    expectU32V2(bytes, state, 0, 'FONT_DESIGN');
    expectByteV2(bytes, state, 0, 'FONT_AREA_LEN');
    const fontNameLen = readU8V2(bytes, state);
    if (fontNameLen !== DVI_FONT_NAME_V2.length) {
      throw new Error(`toc_v2 dvi parse expected font name length ${DVI_FONT_NAME_V2.length}`);
    }
    if (state.index + fontNameLen > bytes.length) {
      throw new Error('toc_v2 dvi parse overflow (font name)');
    }
    const fontName = Buffer.from(bytes.slice(state.index, state.index + fontNameLen)).toString('utf8');
    state.index += fontNameLen;
    if (fontName !== DVI_FONT_NAME_V2) {
      throw new Error(`toc_v2 dvi parse expected font ${DVI_FONT_NAME_V2}, got ${fontName}`);
    }
    expectByteV2(bytes, state, DVI_FNT_NUM_0_OPCODE_V2, 'FNT_NUM_0');

    const lines = [];
    let currentLine = [];
    let expectWidthRightAfterChar = false;
    let expectDownAfterReset = false;
    let pendingByte = 0;
    let pageH = 0;

    while (state.index < bytes.length) {
      const op = bytes[state.index];
      if (op === DVI_EOP_OPCODE_V2) {
        if (expectWidthRightAfterChar || expectDownAfterReset) {
          throw new Error('toc_v2 dvi parse reached EOP with pending line state');
        }
        lines.push(currentLine);
        state.index += 1;
        break;
      }
      if (expectWidthRightAfterChar) {
        if (op !== DVI_RIGHT3_OPCODE_V2) {
          throw new Error(`toc_v2 dvi parse expected RIGHT3 after char, got ${op}`);
        }
        state.index += 1;
        const amount = readI24V2(bytes, state);
        if (amount < 0) {
          throw new Error('toc_v2 dvi parse found negative RIGHT3 width after char');
        }
        if (isStyleMarkerByteV2(pendingByte)) {
          if (amount !== 0) {
            throw new Error('toc_v2 dvi parse style marker must have zero advance');
          }
        } else if (amount === 0) {
          throw new Error('toc_v2 dvi parse printable glyph must have non-zero advance');
        } else {
          pageH += amount;
        }
        currentLine.push(pendingByte);
        expectWidthRightAfterChar = false;
        continue;
      }
      if (op === DVI_RIGHT3_OPCODE_V2) {
        state.index += 1;
        const amount = readI24V2(bytes, state);
        if (amount >= 0) {
          throw new Error('toc_v2 dvi parse expects negative RIGHT3 reset');
        }
        const back = -amount;
        if (back <= 0 || back > pageH) {
          throw new Error('toc_v2 dvi parse invalid RIGHT3 reset amount');
        }
        pageH -= back;
        expectDownAfterReset = true;
        continue;
      }
      if (op === DVI_DOWN3_OPCODE_V2) {
        state.index += 1;
        const amount = readI24V2(bytes, state);
        if (amount <= 0) {
          throw new Error('toc_v2 dvi parse DOWN3 must be positive');
        }
        if (pageH !== 0) {
          throw new Error('toc_v2 dvi parse DOWN3 requires zero horizontal cursor');
        }
        if (expectDownAfterReset) {
          expectDownAfterReset = false;
        }
        lines.push(currentLine);
        currentLine = [];
        continue;
      }
      if (op > 127 || op < 0x20 || op > 0x7e || expectDownAfterReset) {
        throw new Error(`toc_v2 dvi parse found unsupported opcode/text byte ${op}`);
      }
      pendingByte = op;
      state.index += 1;
      expectWidthRightAfterChar = true;
    }

    if (lines.length === 0) {
      throw new Error('toc_v2 dvi parse produced empty page');
    }
    pages.push(lines.map((line) => Uint8Array.from(line)));
    previousBopOffset = bopOffset;
    pageCount += 1;
  }

  if (pageCount <= 0) {
    throw new Error('toc_v2 dvi parse found no pages');
  }
  expectByteV2(bytes, state, DVI_POST_OPCODE_V2, 'POST');
  expectU32V2(bytes, state, lastBopOffset >>> 0, 'POST_LAST_BOP');
  expectU32V2(bytes, state, DVI_NUM_V2, 'POST_NUM');
  expectU32V2(bytes, state, DVI_DEN_V2, 'POST_DEN');
  expectU32V2(bytes, state, DVI_MAG_V2, 'POST_MAG');
  readU32V2(bytes, state); // max_h
  readU32V2(bytes, state); // max_v
  const stackDepth = (readU8V2(bytes, state) << 8) | readU8V2(bytes, state);
  if (stackDepth !== 0) {
    throw new Error(`toc_v2 dvi parse expected stack depth 0, got ${stackDepth}`);
  }
  const declaredPages = (readU8V2(bytes, state) << 8) | readU8V2(bytes, state);
  if (declaredPages !== pageCount) {
    throw new Error(`toc_v2 dvi parse page count mismatch ${declaredPages} vs ${pageCount}`);
  }
  expectByteV2(bytes, state, DVI_POSTPOST_OPCODE_V2, 'POSTPOST');
  const postPointer = readU32V2(bytes, state);
  if (postPointer >= bytes.length) {
    throw new Error('toc_v2 dvi parse post pointer out of bounds');
  }
  expectByteV2(bytes, state, DVI_ID_V2, 'POSTPOST_DVI_ID');
  if (bytes.length - state.index < 4) {
    throw new Error('toc_v2 dvi parse trailer too short');
  }
  for (let index = state.index; index < bytes.length; index += 1) {
    if (bytes[index] !== DVI_TRAILER_BYTE_V2) {
      throw new Error('toc_v2 dvi parse trailer byte mismatch');
    }
  }

  return pages;
}

function lineStartsWithAsciiV2(lineBytes, prefix) {
  const prefixBytes = Buffer.from(prefix, 'ascii');
  if (!lineBytes || lineBytes.length < prefixBytes.length) {
    return false;
  }
  for (let index = 0; index < prefixBytes.length; index += 1) {
    if (lineBytes[index] !== prefixBytes[index]) {
      return false;
    }
  }
  return true;
}

function lineEqualsAsciiV2(lineBytes, value) {
  const valueBytes = Buffer.from(value, 'ascii');
  if (!lineBytes || lineBytes.length !== valueBytes.length) {
    return false;
  }
  for (let index = 0; index < valueBytes.length; index += 1) {
    if (lineBytes[index] !== valueBytes[index]) {
      return false;
    }
  }
  return true;
}

function hasFigureBoxMarkerPrefixV2(lineBytes) {
  return lineStartsWithAsciiV2(lineBytes, FIGURE_BOX_LINE_V2);
}

function parseFigurePlacementHintFromFigureBoxLineV0(lineBytes) {
  if (lineEqualsAsciiV2(lineBytes, FIGURE_BOX_LINE_V2)) {
    return 'inline';
  }
  const topMarker = `${FIGURE_BOX_LINE_V2} ${FIGURE_TOP_PLACEMENT_HINT_V2}`;
  if (lineEqualsAsciiV2(lineBytes, topMarker)) {
    return FIGURE_TOP_PLACEMENT_HINT_V2;
  }
  return null;
}

function detectListPrefixLineV2(lineBytes) {
  let leading = 0;
  while (leading < lineBytes.length && lineBytes[leading] === 0x20) {
    leading += 1;
  }
  if (leading + 2 <= lineBytes.length && lineBytes[leading] === 0x2d && lineBytes[leading + 1] === 0x20) {
    return true;
  }
  let cursor = leading;
  let sawDigit = false;
  while (cursor < lineBytes.length && lineBytes[cursor] >= 0x30 && lineBytes[cursor] <= 0x39) {
    sawDigit = true;
    cursor += 1;
  }
  return sawDigit
    && cursor + 1 < lineBytes.length
    && lineBytes[cursor] === 0x2e
    && lineBytes[cursor + 1] === 0x20;
}

function isStructuredNonTitleLineV2(lineBytes) {
  return lineStartsWithAsciiV2(lineBytes, SECTION_HEADING_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, SUBSECTION_HEADING_PREFIX_V2)
    || detectListPrefixLineV2(lineBytes)
    || lineStartsWithAsciiV2(lineBytes, '> ')
    || lineStartsWithAsciiV2(lineBytes, '^ ')
    || lineStartsWithAsciiV2(lineBytes, '| ')
    || lineStartsWithAsciiV2(lineBytes, '~ ')
    || lineStartsWithAsciiV2(lineBytes, TABLE_SPEC_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, TABLE_ROW_LINE_PREFIX_V2)
    || hasFigureBoxMarkerPrefixV2(lineBytes)
    || lineStartsWithAsciiV2(lineBytes, FIGURE_CAPTION_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, FIGURE_IMAGE_LINE_PREFIX_V2)
    || lineEqualsAsciiV2(lineBytes, TOC_PLACEHOLDER_MARKER_V2)
    || lineStartsWithAsciiV2(lineBytes, TOC_ENTRY_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, FOOTNOTE_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, HREF_URL_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, LABEL_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, REF_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, PAGEREF_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, REF_ANCHOR_LINK_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, PAGEREF_PAGE_LINK_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, EQUATION_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, BIBITEM_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, CITE_LINE_PREFIX_V2);
}

function detectTitleBlockLenV2(lines) {
  let index = 0;
  while (index < lines.length && lines[index].length > 0 && !isStructuredNonTitleLineV2(lines[index])) {
    index += 1;
  }
  return index > 0 && index < lines.length ? index : 0;
}

function hasMetadataPrefixLineV2(lineBytes) {
  return lineStartsWithAsciiV2(lineBytes, FOOTNOTE_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, HREF_URL_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, TOC_ENTRY_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, LABEL_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, REF_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, PAGEREF_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, REF_ANCHOR_LINK_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, PAGEREF_PAGE_LINK_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, EQUATION_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, BIBITEM_LINE_PREFIX_V2)
    || lineStartsWithAsciiV2(lineBytes, CITE_LINE_PREFIX_V2);
}

function splitBodyAndMetadataLinesV2(pages) {
  const bodyPages = pages.map(() => []);
  const metadataLines = [];
  let inMetadata = false;
  for (let pageIndex = 0; pageIndex < pages.length; pageIndex += 1) {
    const lines = pages[pageIndex];
    for (const lineBytes of lines) {
      if (hasMetadataPrefixLineV2(lineBytes)) {
        inMetadata = true;
      }
      if (inMetadata) {
        metadataLines.push(lineBytes);
      } else {
        bodyPages[pageIndex].push(lineBytes);
      }
    }
  }
  while (bodyPages.length > 1 && bodyPages[bodyPages.length - 1].length === 0) {
    bodyPages.pop();
  }
  return { bodyPages, metadataLines };
}

function isDisplayMathPlaceholderLineV2(lineBytes) {
  if (!lineStartsWithAsciiV2(lineBytes, '^ ')) {
    return false;
  }
  const payload = Buffer.from(lineBytes.slice(2)).toString('ascii');
  return payload === DISPLAY_MATH_PLACEHOLDER_SHORT_V2
    || payload === DISPLAY_MATH_PLACEHOLDER_MEDIUM_V2
    || payload === DISPLAY_MATH_PLACEHOLDER_LONG_V2;
}

function parseTocEntriesFromMetadataLinesV2(metadataLines) {
  const entries = [];
  const seenAnchors = new Set();
  for (const lineBytes of metadataLines) {
    if (!lineStartsWithAsciiV2(lineBytes, TOC_ENTRY_LINE_PREFIX_V2)) {
      continue;
    }
    const raw = Buffer.from(lineBytes).toString('utf8');
    const match = /^!toc ([12]) ([1-9]\d*) (.+)$/.exec(raw);
    if (!match) {
      throw new Error(`toc_v2 encountered malformed toc metadata line: ${raw}`);
    }
    const level = Number.parseInt(match[1], 10);
    const anchorId = Number.parseInt(match[2], 10);
    const title = match[3].trim();
    if (!Number.isInteger(level) || (level !== 1 && level !== 2)) {
      throw new Error(`toc_v2 encountered unsupported level in metadata: ${raw}`);
    }
    if (!Number.isInteger(anchorId) || anchorId <= 0) {
      throw new Error(`toc_v2 encountered invalid anchor id in metadata: ${raw}`);
    }
    if (title.length === 0) {
      throw new Error(`toc_v2 encountered empty title in metadata: ${raw}`);
    }
    if (seenAnchors.has(anchorId)) {
      throw new Error(`toc_v2 encountered duplicate toc anchor metadata: ${anchorId}`);
    }
    seenAnchors.add(anchorId);
    entries.push({ level, anchor_id: anchorId, title });
  }
  entries.sort((left, right) => left.anchor_id - right.anchor_id);
  return entries;
}

function collectNominalAnchorPageNumbersFromBodyPagesV2(bodyPages) {
  const pageNumbersByAnchorId = new Map();
  let nextAnchorId = 1;
  for (let pageIndex = 0; pageIndex < bodyPages.length; pageIndex += 1) {
    const lines = bodyPages[pageIndex];
    const titleBlockLen = pageIndex === 0 ? detectTitleBlockLenV2(lines) : 0;
    for (let lineIndex = 0; lineIndex < lines.length; lineIndex += 1) {
      if (lineIndex < titleBlockLen) {
        continue;
      }
      const lineBytes = lines[lineIndex];
      const figurePlacementHint = parseFigurePlacementHintFromFigureBoxLineV0(lineBytes);
      if (hasFigureBoxMarkerPrefixV2(lineBytes) && figurePlacementHint === null) {
        throw new Error(`toc_v2 malformed figure marker line: ${Buffer.from(lineBytes).toString('utf8')}`);
      }
      const hasAnchor = figurePlacementHint !== null
        || isDisplayMathPlaceholderLineV2(lineBytes)
        || lineStartsWithAsciiV2(lineBytes, SECTION_HEADING_PREFIX_V2)
        || lineStartsWithAsciiV2(lineBytes, SUBSECTION_HEADING_PREFIX_V2);
      if (!hasAnchor) {
        continue;
      }
      if (pageNumbersByAnchorId.has(nextAnchorId)) {
        throw new Error(`toc_v2 encountered duplicate nominal anchor id ${nextAnchorId}`);
      }
      pageNumbersByAnchorId.set(nextAnchorId, pageIndex + 1);
      nextAnchorId += 1;
    }
  }
  return pageNumbersByAnchorId;
}

function collectTocOutputSnapshotV2(xdvBytes) {
  const pages = parseDviTextPagesForTocV2(xdvBytes);
  const { bodyPages, metadataLines } = splitBodyAndMetadataLinesV2(pages);
  if (bodyPages.length <= 0) {
    throw new Error('toc_v2 requires at least one body page');
  }
  const tocEntries = parseTocEntriesFromMetadataLinesV2(metadataLines);
  const pageNumbersByAnchorId = collectNominalAnchorPageNumbersFromBodyPagesV2(bodyPages);
  for (const entry of tocEntries) {
    const pageNo = pageNumbersByAnchorId.get(entry.anchor_id);
    if (!Number.isInteger(pageNo)) {
      throw new Error(`toc_v2 missing destination for anchor ${entry.anchor_id}`);
    }
    if (pageNo <= 0 || pageNo > bodyPages.length) {
      throw new Error(`toc_v2 destination page out of range for anchor ${entry.anchor_id}: ${pageNo}`);
    }
  }
  return {
    tocEntries,
    pageNumbersByAnchorId,
    pageCount: bodyPages.length,
  };
}

function isMathPayloadWhitespaceByteV0(byte) {
  return byte === 0x20 || byte === 0x09 || byte === 0x0a || byte === 0x0d;
}

function isAsciiAlphaByteV2(byte) {
  return (byte >= 0x41 && byte <= 0x5a) || (byte >= 0x61 && byte <= 0x7a);
}

function isSafeMathV2LiteralByte(byte) {
  return isAsciiAlphaByteV2(byte)
    || (byte >= 0x30 && byte <= 0x39)
    || byte === 0x2b // +
    || byte === 0x2d // -
    || byte === 0x2f // /
    || byte === 0x2a // *
    || byte === 0x3d // =
    || byte === 0x28 // (
    || byte === 0x29 // )
    || byte === 0x5e // ^
    || byte === 0x5f // _
    || byte === 0x7b // {
    || byte === 0x7d; // }
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

function sanitizeMathPayloadPreviewV2(payloadBytes) {
  const normalized = Buffer.from(payloadBytes).toString('utf8').replace(/\s+/g, ' ').trim();
  const normalizedBytes = Buffer.from(normalized, 'utf8');
  if (normalizedBytes.length <= MAX_MATH_PAYLOAD_PREVIEW_BYTES_V2) {
    return normalized;
  }
  return `${normalizedBytes.subarray(0, MAX_MATH_PAYLOAD_PREVIEW_BYTES_V2).toString('utf8')}...`;
}

function addMathEntryV2(entries, payloadBytes, sourceBytes, startByte, endByte) {
  trimMathPayloadTrailingSpaceV0(payloadBytes);
  if (payloadBytes.length === 0) {
    throw new Error('math_v2 payload must be non-empty');
  }
  if (payloadBytes.length > MAX_MATH_PAYLOAD_BYTES_V0) {
    throw new Error(`math_v2 payload exceeds cap ${MAX_MATH_PAYLOAD_BYTES_V0}`);
  }
  const ordinal = entries.length + 1;
  entries.push({
    ordinal,
    payload_preview: sanitizeMathPayloadPreviewV2(payloadBytes),
    anchor_id: `eq${ordinal}`,
    source_span: buildSourceSpanV0(sourceBytes, startByte, endByte, 'math_v2'),
  });
  if (entries.length > MAX_MATH_ENTRIES_V0) {
    throw new Error(`math_v2 entries exceed cap ${MAX_MATH_ENTRIES_V0}`);
  }
}

function extractMathEntriesFromSourceV2(sourceBytes) {
  const entries = [];
  let index = 0;
  while (index < sourceBytes.length) {
    const byte = sourceBytes[index];

    if (byte === 0x5c && index + 1 < sourceBytes.length && sourceBytes[index + 1] === 0x5b) {
      const startByte = index;
      const payloadBytes = [];
      let cursor = index + 2;
      let closed = false;
      while (cursor < sourceBytes.length) {
        if (sourceBytes[cursor] === 0x5c && cursor + 1 < sourceBytes.length && sourceBytes[cursor + 1] === 0x5d) {
          addMathEntryV2(entries, payloadBytes, sourceBytes, startByte, cursor + 2);
          cursor += 2;
          index = cursor;
          closed = true;
          break;
        }
        const current = sourceBytes[cursor];
        if (isMathPayloadWhitespaceByteV0(current)) {
          pushMathPayloadSpaceV0(payloadBytes);
          cursor += 1;
          continue;
        }
        if (current === 0x5c) {
          const commandStart = cursor + 1;
          let commandEnd = commandStart;
          while (commandEnd < sourceBytes.length && isAsciiAlphaByteV2(sourceBytes[commandEnd])) {
            commandEnd += 1;
          }
          if (commandEnd === commandStart) {
            throw new Error('math_v2 display payload has invalid backslash command');
          }
          payloadBytes.push(0x5c);
          for (let i = commandStart; i < commandEnd; i += 1) {
            payloadBytes.push(sourceBytes[i]);
          }
          cursor = commandEnd;
          continue;
        }
        if (!isSafeMathV2LiteralByte(current)) {
          throw new Error(`math_v2 display payload has unsupported byte 0x${current.toString(16).padStart(2, '0')}`);
        }
        payloadBytes.push(current);
        cursor += 1;
      }
      if (!closed) {
        throw new Error('math_v2 display payload missing closing \\] delimiter');
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
    throw new Error(`table_v2 invalid row count ${rawRows.length}`);
  }
  const rows = [];
  for (const rawRow of rawRows) {
    const cells = rawRow.split('&').map(normalizeTableCellTextV0);
    if (cells.length !== expectedColumnCount) {
      throw new Error(`table_v2 row column count mismatch: expected ${expectedColumnCount}, got ${cells.length}`);
    }
    if (cells.some((cell) => cell.length === 0)) {
      throw new Error('table_v2 row contains empty cell');
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
      throw new Error('table_v2 tabular align spec missing closing }');
    }
    const alignSpec = Buffer.from(sourceBytes.slice(alignStart, alignEnd)).toString('utf8');
    if (
      alignSpec.length === 0
      || alignSpec.length > MAX_TABLE_COLS_PER_ENTRY_V0
      || !/^[lcr]+$/.test(alignSpec)
    ) {
      throw new Error(`table_v2 unsupported align spec '${alignSpec}'`);
    }
    const bodyStart = alignEnd + 1;
    const endIndex = indexOfSubarrayV0(sourceBytes, endMarker, bodyStart);
    if (endIndex < 0) {
      throw new Error('table_v2 tabular missing end marker');
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
      source_span: buildSourceSpanV0(sourceBytes, beginIndex, endIndex + endMarker.length, 'table_v2'),
    });
    if (entries.length > MAX_TABLE_ENTRIES_V0) {
      throw new Error(`table_v2 entries exceed cap ${MAX_TABLE_ENTRIES_V0}`);
    }
    index = endIndex + endMarker.length;
  }
  return entries;
}

function normalizeFloatCaptionSummaryV0(rawCaption) {
  const normalized = rawCaption.replace(/\s+/g, ' ').trim();
  const normalizedBytes = Buffer.from(normalized, 'utf8');
  if (normalizedBytes.length <= MAX_FLOAT_CAPTION_SUMMARY_BYTES_V0) {
    return normalized;
  }
  return `${normalizedBytes.subarray(0, MAX_FLOAT_CAPTION_SUMMARY_BYTES_V0).toString('utf8')}...`;
}

function parseFigurePlacementHintOptionV0(rawOption) {
  const normalized = rawOption.replace(/\s+/g, '');
  if (normalized.length === 0) {
    throw new Error('float_v0 figure placement option must be non-empty');
  }
  if (normalized === FIGURE_TOP_PLACEMENT_HINT_V2) {
    return FIGURE_TOP_PLACEMENT_HINT_V2;
  }
  throw new Error(`float_v0 unsupported figure placement option '${rawOption.trim()}'`);
}

function extractFloatEntriesFromSourceV0(sourceBytes) {
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
    if (command !== 'begin') {
      index = commandIndex;
      continue;
    }

    const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
    if (!envGroup.ok) {
      throw new Error('float_v0 malformed \\begin group');
    }
    if (envGroup.value.trim() !== 'figure') {
      index = envGroup.next;
      continue;
    }

    const beginIndex = index;
    let cursor = skipSpacesV0(sourceBytes, envGroup.next);
    let placementHint = 'inline';
    const placementGroup = readBracketGroupV0(sourceBytes, cursor);
    if (placementGroup.ok) {
      placementHint = parseFigurePlacementHintOptionV0(placementGroup.value);
      cursor = placementGroup.next;
    } else if (cursor < sourceBytes.length && sourceBytes[cursor] === 0x5b) {
      throw new Error('float_v0 malformed figure placement option');
    }

    let captionSummary = null;
    let endIndex = -1;
    while (cursor < sourceBytes.length) {
      if (sourceBytes[cursor] !== 0x5c) {
        cursor += 1;
        continue;
      }
      let innerCommandIndex = cursor + 1;
      while (innerCommandIndex < sourceBytes.length && isAsciiLetterByteV0(sourceBytes[innerCommandIndex])) {
        innerCommandIndex += 1;
      }
      if (innerCommandIndex === cursor + 1) {
        cursor += 1;
        continue;
      }
      const innerCommand = Buffer.from(sourceBytes.slice(cursor + 1, innerCommandIndex)).toString('ascii');
      if (innerCommand === 'begin') {
        throw new Error('float_v0 nested \\begin inside figure is unsupported');
      }
      if (innerCommand === 'end') {
        const endGroup = readBracedGroupV0(sourceBytes, innerCommandIndex);
        if (!endGroup.ok || endGroup.value.trim() !== 'figure') {
          throw new Error('float_v0 malformed \\end{figure}');
        }
        endIndex = endGroup.next;
        break;
      }
      if (innerCommand === 'caption') {
        if (captionSummary !== null) {
          throw new Error('float_v0 duplicate \\caption inside figure');
        }
        const captionGroup = readBracedGroupV0(sourceBytes, innerCommandIndex);
        if (!captionGroup.ok) {
          throw new Error('float_v0 malformed \\caption group');
        }
        const normalizedCaption = normalizeFloatCaptionSummaryV0(captionGroup.value);
        if (normalizedCaption.length === 0) {
          throw new Error('float_v0 caption summary must be non-empty');
        }
        captionSummary = normalizedCaption;
        cursor = captionGroup.next;
        continue;
      }
      cursor = innerCommandIndex;
    }

    if (endIndex < 0) {
      throw new Error('float_v0 missing \\end{figure}');
    }
    if (captionSummary === null) {
      throw new Error('float_v0 figure requires caption summary');
    }
    entries.push({
      float_id: `flt${entries.length + 1}`,
      figure_ordinal: entries.length + 1,
      placement_hint: placementHint,
      caption_summary: captionSummary,
      source_span: buildSourceSpanV0(sourceBytes, beginIndex, endIndex, 'float_v0'),
    });
    if (entries.length > MAX_FLOAT_ENTRIES_V0) {
      throw new Error(`float_v0 entries exceed cap ${MAX_FLOAT_ENTRIES_V0}`);
    }
    index = endIndex;
  }

  return entries;
}

function collectFloatOutputSnapshotV0(xdvBytes) {
  const pages = parseDviTextPagesForTocV2(xdvBytes);
  const { bodyPages } = splitBodyAndMetadataLinesV2(pages);
  if (bodyPages.length <= 0) {
    throw new Error('float_v0 requires at least one body page');
  }
  const figureEntries = [];
  let nextAnchorId = 1;
  for (let pageIndex = 0; pageIndex < bodyPages.length; pageIndex += 1) {
    const lines = bodyPages[pageIndex];
    const titleBlockLen = pageIndex === 0 ? detectTitleBlockLenV2(lines) : 0;
    for (let lineIndex = 0; lineIndex < lines.length; lineIndex += 1) {
      if (lineIndex < titleBlockLen) {
        continue;
      }
      const lineBytes = lines[lineIndex];
      const figurePlacementHint = parseFigurePlacementHintFromFigureBoxLineV0(lineBytes);
      if (hasFigureBoxMarkerPrefixV2(lineBytes) && figurePlacementHint === null) {
        throw new Error(`float_v0 malformed figure marker line: ${Buffer.from(lineBytes).toString('utf8')}`);
      }
      const hasAnchor = figurePlacementHint !== null
        || isDisplayMathPlaceholderLineV2(lineBytes)
        || lineStartsWithAsciiV2(lineBytes, SECTION_HEADING_PREFIX_V2)
        || lineStartsWithAsciiV2(lineBytes, SUBSECTION_HEADING_PREFIX_V2);
      if (!hasAnchor) {
        continue;
      }
      const anchorId = nextAnchorId;
      nextAnchorId += 1;
      if (figurePlacementHint !== null) {
        figureEntries.push({
          anchor_id: anchorId,
          placement_hint: figurePlacementHint,
          page_no: pageIndex + 1,
        });
      }
    }
  }
  return {
    entries: figureEntries,
    page_count: bodyPages.length,
  };
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

function normalizePackageOptionTokenV1(rawValue, context) {
  const trimmed = rawValue.trim();
  if (trimmed.length === 0) {
    throw new Error(`${context} has empty option entry`);
  }
  if (trimmed.includes('{') || trimmed.includes('}') || trimmed.includes('\\')) {
    throw new Error(`${context} has unsupported option token '${trimmed}'`);
  }
  const normalized = trimmed.replace(/\s*=\s*/g, '=');
  if (!/^[A-Za-z0-9._:-]+(?:=[A-Za-z0-9._:/-]+)?$/.test(normalized)) {
    throw new Error(`${context} has unsupported option token '${trimmed}'`);
  }
  return normalized;
}

function splitPackageOptionsStrictV1(rawValue, context) {
  const values = [];
  for (const chunk of rawValue.split(',')) {
    values.push(normalizePackageOptionTokenV1(chunk, context));
  }
  return dedupeValuesPreserveOrderV0(values);
}

function ensureDefaultExtensionV0(value, extension) {
  if (value.includes('.')) {
    return value;
  }
  return `${value}.${extension}`;
}

function normalizeIncludegraphicsCandidatePathV1(rawValue, extOption) {
  const trimmed = `${rawValue}`.trim();
  if (trimmed.length === 0) {
    throw new Error('resource_hints_v0 includegraphics path is empty');
  }
  const slashIndex = Math.max(trimmed.lastIndexOf('/'), trimmed.lastIndexOf('\\'));
  const lastSegment = trimmed.slice(slashIndex + 1);
  const hasAnyDot = lastSegment.includes('.');
  let resolved = trimmed;
  if (extOption.length > 0) {
    resolved = ensureDefaultExtensionV0(resolved, extOption);
  } else if (!hasAnyDot) {
    resolved = ensureDefaultExtensionV0(resolved, 'png');
  }

  const resolvedSlashIndex = Math.max(resolved.lastIndexOf('/'), resolved.lastIndexOf('\\'));
  const resolvedLastSegment = resolved.slice(resolvedSlashIndex + 1);
  const dotIndex = resolvedLastSegment.lastIndexOf('.');
  if (dotIndex <= 0 || dotIndex === resolvedLastSegment.length - 1) {
    throw new Error(`resource_hints_v0 includegraphics rejects extension '${resolved}'`);
  }
  const ext = resolvedLastSegment.slice(dotIndex + 1).toLowerCase();
  if (!['png', 'jpg', 'jpeg', 'pdf'].includes(ext)) {
    throw new Error(`resource_hints_v0 includegraphics rejects extension '${ext}'`);
  }
  return resolved;
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
    if (value.length === 0) {
      throw new Error(`resource_hints_v0 graphicspath rejects empty prefix '${rawValue}'`);
    }
    values.push(value);
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

function splitPackageNamesStrictV1(rawValue, context) {
  const values = [];
  for (const chunk of rawValue.split(',')) {
    const trimmed = chunk.trim();
    if (trimmed.length === 0) {
      throw new Error(`${context} has empty package entry`);
    }
    values.push(trimmed);
  }
  return dedupeValuesPreserveOrderV0(values);
}

function addPackagesEntryV1(entries, entry) {
  const nameBytes = Buffer.from(entry.name, 'utf8');
  if (nameBytes.length > MAX_PACKAGES_NAME_BYTES_V1) {
    throw new Error(`packages_v1 package exceeds cap ${MAX_PACKAGES_NAME_BYTES_V1}`);
  }
  if (!Array.isArray(entry.options) || entry.options.length > MAX_PACKAGES_OPTIONS_PER_ENTRY_V1) {
    throw new Error(`packages_v1 options exceed cap ${MAX_PACKAGES_OPTIONS_PER_ENTRY_V1}`);
  }
  for (const option of entry.options) {
    const optionBytes = Buffer.from(option, 'utf8');
    if (optionBytes.length > MAX_PACKAGES_OPTION_BYTES_V1) {
      throw new Error(`packages_v1 option exceeds cap ${MAX_PACKAGES_OPTION_BYTES_V1}`);
    }
  }
  entries.push(entry);
  if (entries.length > MAX_PACKAGES_ENTRIES_V1) {
    throw new Error(`packages_v1 entries exceed cap ${MAX_PACKAGES_ENTRIES_V1}`);
  }
}

function extractPackagesEntriesFromSourceV1(sourceBytes) {
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
    if (command !== 'usepackage' && command !== 'RequirePackage') {
      index = commandIndex;
      continue;
    }

    let options = [];
    let next = commandIndex;
    const optionsGroup = readBracketGroupV0(sourceBytes, next);
    if (optionsGroup.ok) {
      options = splitPackageOptionsStrictV1(optionsGroup.value, 'packages_v1 usepackage options');
      next = optionsGroup.next;
    }

    const packageGroup = readBracedGroupV0(sourceBytes, next);
    if (!packageGroup.ok || packageGroup.value.length === 0) {
      throw new Error('packages_v1 usepackage command missing package group');
    }
    if (packageGroup.value.includes('{') || packageGroup.value.includes('}')) {
      throw new Error(`packages_v1 rejects nested braces in package group '${packageGroup.value}'`);
    }
    const packageNames = splitPackageNamesStrictV1(packageGroup.value, 'packages_v1 usepackage packages');
    for (const packageNameRaw of packageNames) {
      const normalizedPackage = normalizePathHintTokenV0(packageNameRaw, 'package_file');
      if (!normalizedPackage) {
        throw new Error(`packages_v1 package name '${packageNameRaw}' normalized empty`);
      }
      addPackagesEntryV1(entries, {
        command,
        name: ensureDefaultExtensionV0(normalizedPackage, 'sty'),
        options,
        source_span: buildSourceSpanV0(sourceBytes, index, packageGroup.next, 'packages_v1'),
      });
    }
    index = packageGroup.next;
  }
  return entries;
}

function parsePositiveDecimalStrictV2(rawValue, context) {
  const trimmed = `${rawValue}`.trim();
  if (!/^[0-9]+(?:\.[0-9]+)?$/.test(trimmed)) {
    throw new Error(`${context} has malformed decimal '${rawValue}'`);
  }
  const parsed = Number.parseFloat(trimmed);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`${context} must be positive '${rawValue}'`);
  }
  return parsed;
}

function parseLengthPtStrictV2(rawValue, context) {
  const trimmed = `${rawValue}`.trim();
  const match = /^([0-9]+(?:\.[0-9]+)?)(pt|mm|cm|in)?$/i.exec(trimmed);
  if (!match) {
    throw new Error(`${context} has malformed length '${rawValue}'`);
  }
  const numeric = parsePositiveDecimalStrictV2(match[1], context);
  const unit = (match[2] ?? 'pt').toLowerCase();
  if (unit === 'pt') {
    return numeric;
  }
  if (unit === 'in') {
    return numeric * 72.0;
  }
  if (unit === 'cm') {
    return (numeric * 72.0) / 2.54;
  }
  if (unit === 'mm') {
    return (numeric * 72.0) / 25.4;
  }
  throw new Error(`${context} has unsupported unit '${unit}'`);
}

function clampGraphicsDimensionV2(valuePt, maxPt) {
  return Math.max(1.0, Math.min(maxPt, valuePt));
}

function parseGraphicsSizingOptionsStrictV2(rawValue) {
  const entries = rawValue.split(',');
  const options = new Map();
  for (const chunk of entries) {
    const trimmed = chunk.trim();
    if (trimmed.length === 0) {
      throw new Error('graphics_v2 includegraphics options has empty entry');
    }
    const equalsIndex = trimmed.indexOf('=');
    if (equalsIndex <= 0 || equalsIndex === trimmed.length - 1) {
      throw new Error(`graphics_v2 includegraphics option '${trimmed}' is malformed`);
    }
    const key = trimmed.slice(0, equalsIndex).trim().toLowerCase();
    const value = trimmed.slice(equalsIndex + 1).trim();
    if (!['width', 'height', 'scale'].includes(key)) {
      throw new Error(`graphics_v2 includegraphics option '${key}' is unsupported`);
    }
    if (options.has(key)) {
      throw new Error(`graphics_v2 includegraphics option '${key}' is duplicated`);
    }
    options.set(key, value);
  }
  if (options.size === 0) {
    throw new Error('graphics_v2 includegraphics options must be non-empty');
  }
  if (options.has('scale') && (options.has('width') || options.has('height'))) {
    throw new Error('graphics_v2 includegraphics scale cannot be combined with width/height');
  }

  let widthPt = DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2;
  let heightPt = DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2;
  let scale = null;
  if (options.has('scale')) {
    scale = parsePositiveDecimalStrictV2(options.get('scale'), 'graphics_v2 scale');
    widthPt *= scale;
    heightPt *= scale;
  } else if (options.has('width') && options.has('height')) {
    widthPt = parseLengthPtStrictV2(options.get('width'), 'graphics_v2 width');
    heightPt = parseLengthPtStrictV2(options.get('height'), 'graphics_v2 height');
  } else if (options.has('width')) {
    widthPt = parseLengthPtStrictV2(options.get('width'), 'graphics_v2 width');
    heightPt = widthPt * (DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2 / DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2);
  } else if (options.has('height')) {
    heightPt = parseLengthPtStrictV2(options.get('height'), 'graphics_v2 height');
    widthPt = heightPt * (DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2 / DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2);
  }
  widthPt = clampGraphicsDimensionV2(widthPt, MAX_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2);
  heightPt = clampGraphicsDimensionV2(heightPt, MAX_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2);
  return {
    width_pt: Number(widthPt.toFixed(3)),
    height_pt: Number(heightPt.toFixed(3)),
    scale: scale === null ? null : Number(scale.toFixed(6)),
  };
}

function extractGraphicsEntriesFromSourceV2(sourceBytes) {
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
    const sizingOpts = optGroup.ok
      ? parseGraphicsSizingOptionsStrictV2(optGroup.value)
      : {
          width_pt: DEFAULT_GRAPHICS_PLACEHOLDER_WIDTH_PT_V2,
          height_pt: DEFAULT_GRAPHICS_PLACEHOLDER_HEIGHT_PT_V2,
          scale: null,
        };
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
      throw new Error(`graphics_v2 unsupported extension '${extension}'`);
    }

    const pathBytes = Buffer.from(pathAsWritten, 'utf8');
    if (pathBytes.length > MAX_GRAPHICS_PATH_BYTES_V0) {
      throw new Error(`graphics_v2 path exceeds cap ${MAX_GRAPHICS_PATH_BYTES_V0}`);
    }
    entries.push({
      command,
      path: pathAsWritten,
      resolver_path: resolverPath,
      opts: sizingOpts,
      source_span: buildSourceSpanV0(sourceBytes, index, pathGroup.next, 'graphics_v2'),
    });
    if (entries.length > MAX_GRAPHICS_ENTRIES_V0) {
      throw new Error(`graphics_v2 entries exceed cap ${MAX_GRAPHICS_ENTRIES_V0}`);
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
    if (command !== 'input' && command !== 'include' && command !== 'includeonly') {
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
        hint_type: command === 'input'
          ? 'tex_input'
          : command === 'include'
            ? 'tex_include'
            : 'tex_includeonly',
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

      if (!resolved || parsedPaths.has(directive.value) || directive.command === 'includeonly') {
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
        splitPackageOptionsStrictV1(optionsGroup.value, 'resource_hints_v0 usepackage options');
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
            candidates.push(normalizeIncludegraphicsCandidatePathV1(withDir, extNormalized));
          }
        } else {
          const withDir = dirNormalized ? `${dirNormalized}/${value}` : value;
          candidates.push(normalizeIncludegraphicsCandidatePathV1(withDir, extNormalized));
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
        if (!rawValue.endsWith('/')) {
          throw new Error(`resource_hints_v0 graphicspath entry must end with '/': '${rawValue}'`);
        }
        const withoutTrailingSlash = rawValue.slice(0, -1).trim();
        if (withoutTrailingSlash.length === 0) {
          throw new Error(`resource_hints_v0 graphicspath rejects empty prefix '${rawValue}'`);
        }
        const normalized = normalizePathHintTokenV0(withoutTrailingSlash, 'graphics_path');
        if (!normalized || normalized.length === 0) {
          throw new Error(`resource_hints_v0 graphicspath normalized empty prefix '${rawValue}'`);
        }
        prefixes.push(normalized);
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

function traversePagerefSourceV2(sourcePath, sourceBytes, sourcesByPath, state, pagerefByKey) {
  if (state.visiting.has(sourcePath)) {
    throw new Error(`pageref_v2 include cycle at ${sourcePath}`);
  }
  state.visiting.add(sourcePath);
  try {
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
      if (command === 'begin') {
        const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!envGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        const envName = envGroup.value.trim();
        if (envName === 'figure') {
          state.inFigure = true;
        }
        state.pendingLabelTarget = null;
        index = envGroup.next;
        continue;
      }
      if (command === 'end') {
        const envGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!envGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        const envName = envGroup.value.trim();
        if (envName === 'figure') {
          state.inFigure = false;
        }
        state.pendingLabelTarget = null;
        index = envGroup.next;
        continue;
      }
      if (command === 'section' || command === 'subsection') {
        const titleGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!titleGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        state.pendingLabelTarget = {
          anchor_id: state.nextAnchorId,
          page_no: state.currentPageNo,
        };
        state.nextAnchorId += 1;
        index = titleGroup.next;
        continue;
      }
      if (command === 'caption' && state.inFigure) {
        const captionGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!captionGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        state.pendingLabelTarget = {
          anchor_id: state.nextAnchorId,
          page_no: state.currentPageNo,
        };
        state.nextAnchorId += 1;
        index = captionGroup.next;
        continue;
      }
      if (command === 'label') {
        const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!keyGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        const key = keyGroup.value.trim();
        if (state.pendingLabelTarget && isSafeLabelRefKeyValueV1(key) && !state.labelsByKey.has(key)) {
          const keyBytes = Buffer.from(key, 'utf8');
          if (keyBytes.length > MAX_LABEL_VALUE_BYTES_V0) {
            throw new Error(`pageref_v2 label key exceeds cap ${MAX_LABEL_VALUE_BYTES_V0}`);
          }
          state.labelsByKey.set(key, {
            anchor_id: state.pendingLabelTarget.anchor_id,
            page_no: state.pendingLabelTarget.page_no,
          });
        }
        state.pendingLabelTarget = null;
        index = keyGroup.next;
        continue;
      }
      if (command === 'pageref') {
        const keyGroup = readBracedGroupV0(sourceBytes, commandIndex);
        if (!keyGroup.ok) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        const key = keyGroup.value.trim();
        if (isSafeLabelRefKeyValueV1(key)) {
          let entry = pagerefByKey.get(key);
          if (!entry) {
            entry = {
              key,
              resolved: false,
              anchor_id: null,
              page_no: null,
              source_path: sourcePath,
              source_span: buildSourceSpanV0(sourceBytes, index, keyGroup.next, 'pageref_v2'),
              occurrences: [],
            };
            pagerefByKey.set(key, entry);
          }
          entry.occurrences.push({
            source_path: sourcePath,
            line_index: lineIndexForByteOffsetV1(sourceBytes, index),
            page_no: null,
          });
          if (entry.occurrences.length > MAX_PAGEREF_OCCURRENCES_PER_KEY_V2) {
            throw new Error(`pageref_v2 occurrences exceed cap ${MAX_PAGEREF_OCCURRENCES_PER_KEY_V2} for key ${key}`);
          }
        }
        state.pendingLabelTarget = null;
        index = keyGroup.next;
        continue;
      }
      if (command === 'input' || command === 'include') {
        const group = readBracedGroupV0(sourceBytes, commandIndex);
        if (!group.ok || group.value.length === 0) {
          index = commandIndex;
          state.pendingLabelTarget = null;
          continue;
        }
        const values = splitCommaValuesV0(group.value);
        for (const rawValue of values) {
          const mountPath = normalizeInputIncludeMountPathV1(rawValue);
          if (!mountPath) {
            continue;
          }
          const nestedBytes = sourcesByPath.get(mountPath);
          if (!nestedBytes) {
            continue;
          }
          if (command === 'include') {
            state.currentPageNo += 1;
          }
          traversePagerefSourceV2(mountPath, nestedBytes, sourcesByPath, state, pagerefByKey);
        }
        state.pendingLabelTarget = null;
        index = group.next;
        continue;
      }

      state.pendingLabelTarget = null;
      index = commandIndex;
    }
  } finally {
    state.visiting.delete(sourcePath);
  }
}

function extractPagerefEntriesFromSourcesV2(mainSourceBytes, mountedFiles = []) {
  const sourcesByPath = new Map();
  sourcesByPath.set('main.tex', toUint8ArrayV0(mainSourceBytes));
  for (const [mountPath, mountBytes] of mountedFiles) {
    sourcesByPath.set(mountPath, toUint8ArrayV0(mountBytes));
  }

  const state = {
    nextAnchorId: 1,
    currentPageNo: 1,
    inFigure: false,
    pendingLabelTarget: null,
    labelsByKey: new Map(),
    visiting: new Set(),
  };
  const pagerefByKey = new Map();
  traversePagerefSourceV2('main.tex', toUint8ArrayV0(mainSourceBytes), sourcesByPath, state, pagerefByKey);

  const entries = [...pagerefByKey.values()].sort((left, right) => left.key.localeCompare(right.key));
  for (const entry of entries) {
    const resolved = state.labelsByKey.get(entry.key);
    if (!resolved) {
      continue;
    }
    entry.resolved = true;
    entry.anchor_id = resolved.anchor_id;
    entry.page_no = resolved.page_no;
    for (const occurrence of entry.occurrences) {
      occurrence.page_no = resolved.page_no;
    }
  }
  if (entries.length > MAX_PAGEREF_ENTRIES_V2) {
    throw new Error(`pageref_v2 entries exceed cap ${MAX_PAGEREF_ENTRIES_V2}`);
  }
  return entries;
}

async function augmentPagerefMountedFilesWithFixtureSourceV2(caseOutDir, mainSourceBytes, mountedFiles = []) {
  const mountedByPath = new Map();
  for (const [mountPath, mountBytes] of mountedFiles) {
    mountedByPath.set(mountPath, toUint8ArrayV0(mountBytes));
  }
  const fixtureSourceTexRoot = path.join(`${path.dirname(caseOutDir)}_fixture_source_v0`, 'xetex', 'tex');
  const sourceBytesByPath = new Map();
  sourceBytesByPath.set('main.tex', toUint8ArrayV0(mainSourceBytes));
  for (const [mountPath, mountBytes] of mountedByPath.entries()) {
    sourceBytesByPath.set(mountPath, mountBytes);
  }

  const queue = ['main.tex'];
  const visited = new Set();
  while (queue.length > 0) {
    const sourcePath = queue.shift();
    if (visited.has(sourcePath)) {
      continue;
    }
    visited.add(sourcePath);
    const sourceBytes = sourceBytesByPath.get(sourcePath);
    if (!(sourceBytes instanceof Uint8Array)) {
      continue;
    }
    const directives = extractInputIncludeDirectivesFromSourceV1(sourceBytes, sourcePath);
    for (const directive of directives) {
      const mountPath = directive.value;
      if (mountedByPath.has(mountPath)) {
        if (!visited.has(mountPath)) {
          queue.push(mountPath);
        }
        continue;
      }
      const aliasPath = mountPath.replaceAll('/', '__');
      const candidatePaths = aliasPath === mountPath
        ? [mountPath]
        : [aliasPath, mountPath];
      let loadedBytes = null;
      for (const candidatePath of candidatePaths) {
        const candidateFile = path.join(fixtureSourceTexRoot, candidatePath);
        try {
          loadedBytes = toUint8ArrayV0(await readFile(candidateFile));
          break;
        } catch {
          // ignore and continue trying candidate paths
        }
      }
      if (!(loadedBytes instanceof Uint8Array)) {
        continue;
      }
      mountedByPath.set(mountPath, loadedBytes);
      sourceBytesByPath.set(mountPath, loadedBytes);
      queue.push(mountPath);
    }
  }

  return [...mountedByPath.entries()].sort(([leftPath], [rightPath]) => leftPath.localeCompare(rightPath));
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

async function emitPagerefTypedArtifactV2(caseOutDir, fixtureBytes, mountedFiles) {
  const pagerefMountedFiles = await augmentPagerefMountedFilesWithFixtureSourceV2(
    caseOutDir,
    fixtureBytes,
    mountedFiles,
  );
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'pageref_v2',
    entries: extractPagerefEntriesFromSourcesV2(fixtureBytes, pagerefMountedFiles),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'pageref_v2.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitTocTypedArtifactV2(caseOutDir, fixtureBytes, xdvBytes) {
  const caseId = path.basename(caseOutDir);
  const sourceEntries = extractTocEntriesFromSourceV0(fixtureBytes);
  let outputSnapshot;
  try {
    outputSnapshot = collectTocOutputSnapshotV2(xdvBytes);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`toc_v2 case=${caseId} xdv_bytes=${xdvBytes.length}: ${message}`);
  }
  const outputByAnchorId = new Map();
  for (const entry of outputSnapshot.tocEntries) {
    outputByAnchorId.set(entry.anchor_id, entry);
  }

  const sourceAnchorIds = new Set();
  for (const sourceEntry of sourceEntries) {
    const anchorId = parseTocAnchorIdTagV2(sourceEntry.anchor_id);
    if (!anchorId) {
      throw new Error(`toc_v2 invalid source anchor tag: ${sourceEntry.anchor_id}`);
    }
    sourceAnchorIds.add(anchorId);
  }

  if (sourceAnchorIds.size !== outputByAnchorId.size) {
    throw new Error(
      `toc_v2 source/output anchor count mismatch (${sourceAnchorIds.size} vs ${outputByAnchorId.size})`,
    );
  }

  const entries = [];
  for (const sourceEntry of sourceEntries) {
    const anchorId = parseTocAnchorIdTagV2(sourceEntry.anchor_id);
    if (!anchorId) {
      throw new Error(`toc_v2 invalid source anchor tag: ${sourceEntry.anchor_id}`);
    }
    const outputEntry = outputByAnchorId.get(anchorId);
    if (!outputEntry) {
      throw new Error(`toc_v2 output metadata missing anchor ${anchorId}`);
    }
    const expectedTitle = normalizeTocLinkWrappedTitleV2(sourceEntry.title);
    const actualTitle = normalizeTocLinkWrappedTitleV2(outputEntry.title);
    if (sourceEntry.level !== outputEntry.level || expectedTitle !== actualTitle) {
      throw new Error(`toc_v2 source/output metadata mismatch for anchor ${anchorId}`);
    }
    const pageNo = outputSnapshot.pageNumbersByAnchorId.get(anchorId);
    if (!Number.isInteger(pageNo) || pageNo <= 0 || pageNo > outputSnapshot.pageCount) {
      throw new Error(`toc_v2 invalid page_no for anchor ${anchorId}: ${pageNo}`);
    }
    entries.push({
      ...sourceEntry,
      page_no: pageNo,
    });
  }

  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'toc_v2',
    entries,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'toc_v2.json';
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

async function emitPackagesTypedArtifactV1(caseOutDir, fixtureBytes) {
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'packages_v1',
    entries: extractPackagesEntriesFromSourceV1(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'packages_v1.json';
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
    schema: 'graphics_v2',
    entries: extractGraphicsEntriesFromSourceV2(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'graphics_v2.json';
  const fullPath = path.join(caseOutDir, relpath);
  await writeFile(fullPath, bytes);
  return {
    present: true,
    items: payload.entries.length,
    artifact_relpath: relpath,
    artifact_sha256: sha256HexV0(bytes),
  };
}

async function emitFloatTypedArtifactV0(caseOutDir, fixtureBytes, xdvBytes) {
  const sourceEntries = extractFloatEntriesFromSourceV0(fixtureBytes);
  const outputSnapshot = collectFloatOutputSnapshotV0(xdvBytes);
  if (sourceEntries.length !== outputSnapshot.entries.length) {
    throw new Error(
      `float_v0 source/output figure count mismatch (${sourceEntries.length} vs ${outputSnapshot.entries.length})`,
    );
  }
  const entries = sourceEntries.map((sourceEntry, index) => {
    const outputEntry = outputSnapshot.entries[index];
    if (!outputEntry || !Number.isInteger(outputEntry.anchor_id) || outputEntry.anchor_id <= 0) {
      throw new Error(`float_v0 invalid output anchor for entry ${index + 1}`);
    }
    if (sourceEntry.placement_hint !== outputEntry.placement_hint) {
      throw new Error(
        `float_v0 placement mismatch for entry ${index + 1}: ${sourceEntry.placement_hint} vs ${outputEntry.placement_hint}`,
      );
    }
    if (!Number.isInteger(outputEntry.page_no) || outputEntry.page_no <= 0 || outputEntry.page_no > outputSnapshot.page_count) {
      throw new Error(`float_v0 invalid page_no for entry ${index + 1}: ${outputEntry.page_no}`);
    }
    return {
      ...sourceEntry,
      anchor_id: outputEntry.anchor_id,
      page_no: outputEntry.page_no,
    };
  });
  const payload = {
    version: TYPED_ARTIFACTS_VERSION_V0,
    schema: 'float_v0',
    entries,
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'float_v0.json';
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
    schema: 'math_v2',
    entries: extractMathEntriesFromSourceV2(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'math_v2.json';
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
    schema: 'table_v2',
    entries: extractTableEntriesFromSourceV1(fixtureBytes),
  };
  const bytes = Buffer.from(`${JSON.stringify(payload, null, 2)}\n`, 'utf8');
  const relpath = 'table_v2.json';
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
  mountedFiles = [],
  inputEntries = [],
  xdvBytes = new Uint8Array(),
) {
  if (caseSpec.id === 'typeset_demo_toc_probe_v0') {
    typedArtifacts.toc = await emitTocTypedArtifactV2(caseOutDir, fixtureBytes, xdvBytes);
  }
  if (caseSpec.id === 'typeset_demo_labels_probe_v0') {
    typedArtifacts.labels = await emitLabelsTypedArtifactV0(caseOutDir, fixtureBytes);
    typedArtifacts.refs = await emitRefsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (
    caseSpec.id === 'typeset_demo_pageref_probe_v2'
    || caseSpec.id === 'typeset_demo_pageref_include_probe_v2'
    || caseSpec.id === 'typeset_demo_pageref_unresolved_probe_v2'
  ) {
    typedArtifacts.pageref = await emitPagerefTypedArtifactV2(
      caseOutDir,
      fixtureBytes,
      mountedFiles,
    );
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
  if (PACKAGE_ARTIFACT_CASE_IDS_V1.has(caseSpec.id)) {
    typedArtifacts.packages = await emitPackagesTypedArtifactV1(caseOutDir, fixtureBytes);
  }
  if (
    caseSpec.id === 'typeset_demo_graphics_probe_v0'
    || caseSpec.id === 'typeset_demo_graphics_width_probe_v0'
    || caseSpec.id === 'typeset_demo_graphics_scale_probe_v0'
  ) {
    typedArtifacts.graphics = await emitGraphicsTypedArtifactV0(caseOutDir, fixtureBytes);
  }
  if (caseSpec.id === 'typeset_demo_float_probe_v0') {
    typedArtifacts.float = await emitFloatTypedArtifactV0(caseOutDir, fixtureBytes, xdvBytes);
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


export {
  buildTypedArtifactsPlaceholderV0,
  emitResourceHintsArtifactV0,
  emitEmptyResourceHintsArtifactV0,
  emitTypedArtifactsV0,
  buildResourceHintsRollupV0,
  collectInputIncludeGraphV1,
  ensureDefaultExtensionV0,
  isSafeResolverTokenV0,
};
