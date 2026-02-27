import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const wasmPath = path.join(
  rootDir,
  'target',
  'wasm32-unknown-unknown',
  'debug',
  'carreltex_wasm_smoke.wasm',
);

const bytes = await readFile(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {});

const add = instance.exports.carreltex_wasm_smoke_add;
if (typeof add !== 'function') {
  throw new Error('Missing export: carreltex_wasm_smoke_add');
}

const result = add(1, 2);
if (result !== 3) {
  throw new Error(`Unexpected result: ${result}`);
}

const { memory } = instance.exports;
if (!(memory instanceof WebAssembly.Memory)) {
  throw new Error('Missing export: memory');
}

const alloc = instance.exports.carreltex_wasm_alloc;
const dealloc = instance.exports.carreltex_wasm_dealloc;
const validate = instance.exports.carreltex_wasm_validate_main_tex;
const mountReset = instance.exports.carreltex_wasm_mount_reset;
const mountAddFile = instance.exports.carreltex_wasm_mount_add_file;
const mountFinalize = instance.exports.carreltex_wasm_mount_finalize;
const mountHasFile = instance.exports.carreltex_wasm_mount_has_file;
const mountReadFileLen = instance.exports.carreltex_wasm_mount_read_file_len_v0;
const mountReadFileCopy = instance.exports.carreltex_wasm_mount_read_file_copy_v0;
const compileMain = instance.exports.carreltex_wasm_compile_main_v0;
const compileRequestReset = instance.exports.carreltex_wasm_compile_request_reset_v0;
const compileRequestSetEntrypoint = instance.exports.carreltex_wasm_compile_request_set_entrypoint_v0;
const compileRequestSetEpoch = instance.exports.carreltex_wasm_compile_request_set_source_date_epoch_v0;
const compileRequestSetMaxLogBytes = instance.exports.carreltex_wasm_compile_request_set_max_log_bytes_v0;
const compileRun = instance.exports.carreltex_wasm_compile_run_v0;
const reportLen = instance.exports.carreltex_wasm_compile_report_len_v0;
const reportCopy = instance.exports.carreltex_wasm_compile_report_copy_v0;
const logLen = instance.exports.carreltex_wasm_compile_log_len_v0;
const logCopy = instance.exports.carreltex_wasm_compile_log_copy_v0;
const eventsLen = instance.exports.carreltex_wasm_events_len_v0;
const eventsCopy = instance.exports.carreltex_wasm_events_copy_v0;
const artifactMainXdvLen = instance.exports.carreltex_wasm_artifact_main_xdv_len_v0;
const artifactMainXdvCopy = instance.exports.carreltex_wasm_artifact_main_xdv_copy_v0;
const artifactLenByName = instance.exports.carreltex_wasm_artifact_len_v0;
const artifactCopyByName = instance.exports.carreltex_wasm_artifact_copy_v0;

for (const [name, fn] of [
  ['carreltex_wasm_alloc', alloc],
  ['carreltex_wasm_dealloc', dealloc],
  ['carreltex_wasm_validate_main_tex', validate],
  ['carreltex_wasm_mount_reset', mountReset],
  ['carreltex_wasm_mount_add_file', mountAddFile],
  ['carreltex_wasm_mount_finalize', mountFinalize],
  ['carreltex_wasm_mount_has_file', mountHasFile],
  ['carreltex_wasm_mount_read_file_len_v0', mountReadFileLen],
  ['carreltex_wasm_mount_read_file_copy_v0', mountReadFileCopy],
  ['carreltex_wasm_compile_main_v0', compileMain],
  ['carreltex_wasm_compile_request_reset_v0', compileRequestReset],
  ['carreltex_wasm_compile_request_set_entrypoint_v0', compileRequestSetEntrypoint],
  ['carreltex_wasm_compile_request_set_source_date_epoch_v0', compileRequestSetEpoch],
  ['carreltex_wasm_compile_request_set_max_log_bytes_v0', compileRequestSetMaxLogBytes],
  ['carreltex_wasm_compile_run_v0', compileRun],
  ['carreltex_wasm_compile_report_len_v0', reportLen],
  ['carreltex_wasm_compile_report_copy_v0', reportCopy],
  ['carreltex_wasm_compile_log_len_v0', logLen],
  ['carreltex_wasm_compile_log_copy_v0', logCopy],
  ['carreltex_wasm_events_len_v0', eventsLen],
  ['carreltex_wasm_events_copy_v0', eventsCopy],
  ['carreltex_wasm_artifact_main_xdv_len_v0', artifactMainXdvLen],
  ['carreltex_wasm_artifact_main_xdv_copy_v0', artifactMainXdvCopy],
  ['carreltex_wasm_artifact_len_v0', artifactLenByName],
  ['carreltex_wasm_artifact_copy_v0', artifactCopyByName],
]) {
  if (typeof fn !== 'function') {
    throw new Error(`Missing export: ${name}`);
  }
}

function allocBytes(value, field) {
  const ptr = alloc(value.byteLength);
  if (!Number.isInteger(ptr) || ptr <= 0) {
    throw new Error(`alloc failed (${field}), ptr=${ptr}`);
  }
  new Uint8Array(memory.buffer, ptr, value.byteLength).set(value);
  return ptr;
}

function callWithBytes(value, field, callback) {
  const ptr = allocBytes(value, field);
  try {
    return callback(ptr, value.byteLength);
  } finally {
    dealloc(ptr, value.byteLength);
  }
}

function addMountedFile(pathValue, dataValue, label) {
  const pathBytes = new TextEncoder().encode(pathValue);
  if (pathBytes.byteLength === 0) {
    return callWithBytes(dataValue, `${label}_data`, (dataPtr, dataLen) => {
      return mountAddFile(0, 0, dataPtr, dataLen);
    });
  }
  return callWithBytes(pathBytes, `${label}_path`, (pathPtr, pathLen) => {
    return callWithBytes(dataValue, `${label}_data`, (dataPtr, dataLen) => {
      return mountAddFile(pathPtr, pathLen, dataPtr, dataLen);
    });
  });
}

function pathBytes(pathValue) {
  return new TextEncoder().encode(pathValue);
}

function expectInvalid(value, label) {
  if (value !== 1) {
    throw new Error(`${label} expected invalid(1), got ${value}`);
  }
}

function expectNotImplemented(value, label) {
  if (value !== 2) {
    throw new Error(`${label} expected NOT_IMPLEMENTED(2), got ${value}`);
  }
}

function readCompileReportJson() {
  const jsonLen = reportLen();
  if (!Number.isInteger(jsonLen) || jsonLen <= 0 || jsonLen > 4096) {
    throw new Error(`report_len_v0 unexpected: ${jsonLen}`);
  }

  const outPtr = alloc(jsonLen);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`alloc failed for report, ptr=${outPtr}`);
  }

  try {
    const written = reportCopy(outPtr, jsonLen);
    if (written !== jsonLen) {
      throw new Error(`report_copy_v0 expected ${jsonLen}, got ${written}`);
    }
    const outBytes = new Uint8Array(memory.buffer, outPtr, jsonLen);
    const text = new TextDecoder().decode(outBytes);
    return JSON.parse(text);
  } finally {
    dealloc(outPtr, jsonLen);
  }
}

function readCompileLogBytes() {
  const bytesLen = logLen();
  if (!Number.isInteger(bytesLen) || bytesLen < 0 || bytesLen > 4096) {
    throw new Error(`compile_log_len_v0 unexpected: ${bytesLen}`);
  }
  if (bytesLen === 0) {
    return new Uint8Array();
  }

  const outPtr = alloc(bytesLen);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`alloc failed for compile log, ptr=${outPtr}`);
  }

  try {
    const written = logCopy(outPtr, bytesLen);
    if (written !== bytesLen) {
      throw new Error(`compile_log_copy_v0 expected ${bytesLen}, got ${written}`);
    }
    return new Uint8Array(memory.buffer, outPtr, bytesLen).slice();
  } finally {
    dealloc(outPtr, bytesLen);
  }
}

function readEventsBytes() {
  const bytesLen = eventsLen();
  if (!Number.isInteger(bytesLen) || bytesLen < 0 || bytesLen > 4096) {
    throw new Error(`events_len_v0 unexpected: ${bytesLen}`);
  }
  if (bytesLen === 0) {
    return new Uint8Array();
  }

  const outPtr = alloc(bytesLen);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`alloc failed for events, ptr=${outPtr}`);
  }

  try {
    const written = eventsCopy(outPtr, bytesLen);
    if (written !== bytesLen) {
      throw new Error(`events_copy_v0 expected ${bytesLen}, got ${written}`);
    }
    return new Uint8Array(memory.buffer, outPtr, bytesLen).slice();
  } finally {
    dealloc(outPtr, bytesLen);
  }
}

function readU32LE(bytes, offset) {
  return (
    bytes[offset]
    | (bytes[offset + 1] << 8)
    | (bytes[offset + 2] << 16)
    | (bytes[offset + 3] << 24)
  ) >>> 0;
}

function decodeEvents(bytes, label) {
  const events = [];
  let offset = 0;
  while (offset < bytes.length) {
    if (offset + 8 > bytes.length) {
      throw new Error(`${label}: truncated event header at offset=${offset}`);
    }
    const kind = readU32LE(bytes, offset);
    const payloadLen = readU32LE(bytes, offset + 4);
    offset += 8;
    if (offset + payloadLen > bytes.length) {
      throw new Error(`${label}: truncated event payload at offset=${offset}, len=${payloadLen}`);
    }
    events.push({
      kind,
      payload: bytes.subarray(offset, offset + payloadLen),
    });
    offset += payloadLen;
  }
  return events;
}

function assertEventsMatchLogAndStats(logBytes, expectedStatsExact, label) {
  const eventsBytes = readEventsBytes();
  const events = decodeEvents(eventsBytes, label);
  if (events.length !== 2) {
    throw new Error(`${label}: expected 2 events, got ${events.length}`);
  }
  const logEvent = events[0];
  if (logEvent.kind !== 1) {
    throw new Error(`${label}: event[0] kind expected 1, got ${logEvent.kind}`);
  }
  if (
    logEvent.payload.length !== logBytes.length
    || !logEvent.payload.every((byte, index) => byte === logBytes[index])
  ) {
    throw new Error(`${label}: event[0] payload bytes mismatch with compile log`);
  }

  const statsEvent = events[1];
  if (statsEvent.kind !== 2) {
    throw new Error(`${label}: event[1] kind expected 2, got ${statsEvent.kind}`);
  }
  let statsText;
  try {
    statsText = new TextDecoder('utf-8', { fatal: true }).decode(statsEvent.payload);
  } catch {
    throw new Error(`${label}: event[1] payload is not valid utf-8`);
  }
  if (/[ \t\r\n]/.test(statsText)) {
    throw new Error(`${label}: event[1] payload must not contain whitespace`);
  }
  if (statsText.includes('"unexpected_key"')) {
    throw new Error(`${label}: event[1] payload must not contain unexpected_key`);
  }

  let stats;
  try {
    stats = JSON.parse(statsText);
  } catch {
    throw new Error(`${label}: event[1] payload is not valid JSON`);
  }
  if (typeof stats !== 'object' || stats === null) {
    throw new Error(`${label}: event[1] JSON payload must be object`);
  }
  const statsKeys = Object.keys(stats);
  const expectedKeys = [
    'token_count',
    'control_seq_count',
    'char_count',
    'space_count',
    'begin_group_count',
    'end_group_count',
    'max_group_depth',
  ];
  if (statsKeys.length !== expectedKeys.length || !expectedKeys.every((key) => statsKeys.includes(key))) {
    throw new Error(`${label}: event[1] JSON keys mismatch`);
  }
  for (const [key, value] of Object.entries(expectedStatsExact)) {
    if (stats[key] !== value) {
      throw new Error(`${label}: event[1] ${key} expected ${value}, got ${stats[key]}`);
    }
  }
  if (!(typeof stats.token_count === 'number' && stats.token_count > 0)) {
    throw new Error(`${label}: event[1] token_count expected >0`);
  }
  if (!(typeof stats.char_count === 'number' && stats.char_count > 0)) {
    throw new Error(`${label}: event[1] char_count expected >0`);
  }
  return stats;
}

function readMountedFileBytes(pathValue, label) {
  const encodedPath = pathBytes(pathValue);
  return callWithBytes(encodedPath, `${label}_path`, (pathPtr, pathLen) => {
    const len = mountReadFileLen(pathPtr, pathLen);
    if (!Number.isInteger(len) || len < 0 || len > 4 * 1024 * 1024) {
      throw new Error(`${label}: unexpected mounted file len=${len}`);
    }
    if (len === 0) {
      return new Uint8Array();
    }
    const outPtr = alloc(len);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`${label}: alloc failed for mounted file copy, ptr=${outPtr}`);
    }
    try {
      const written = mountReadFileCopy(pathPtr, pathLen, outPtr, len);
      if (written !== len) {
        throw new Error(`${label}: mounted file copy expected ${len}, got ${written}`);
      }
      return new Uint8Array(memory.buffer, outPtr, len).slice();
    } finally {
      dealloc(outPtr, len);
    }
  });
}

function assertReadbackZero(pathValue, label) {
  const encodedPath = pathBytes(pathValue);
  const len = callWithBytes(encodedPath, `${label}_len_path`, (pathPtr, pathLen) => mountReadFileLen(pathPtr, pathLen));
  if (len !== 0) {
    throw new Error(`${label}: expected read_file_len=0, got ${len}`);
  }
  const copyNull = callWithBytes(encodedPath, `${label}_copy_null_path`, (pathPtr, pathLen) =>
    mountReadFileCopy(pathPtr, pathLen, 0, 0),
  );
  if (copyNull !== 0) {
    throw new Error(`${label}: expected read_file_copy(null,0)=0`);
  }
  const outPtr = alloc(1);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`${label}: alloc(1) failed`);
  }
  try {
    const copyOne = callWithBytes(encodedPath, `${label}_copy_one_path`, (pathPtr, pathLen) =>
      mountReadFileCopy(pathPtr, pathLen, outPtr, 1),
    );
    if (copyOne !== 0) {
      throw new Error(`${label}: expected read_file_copy(out,1)=0, got ${copyOne}`);
    }
  } finally {
    dealloc(outPtr, 1);
  }
}

function assertMainXdvArtifactEmpty(label) {
  const bytesLen = artifactMainXdvLen();
  if (bytesLen !== 0) {
    throw new Error(`${label}: expected main.xdv len=0, got ${bytesLen}`);
  }
  if (artifactMainXdvCopy(0, 0) !== 0) {
    throw new Error(`${label}: expected main.xdv copy(null,0)=0`);
  }

  const outPtr = alloc(1);
  if (!Number.isInteger(outPtr) || outPtr <= 0) {
    throw new Error(`${label}: alloc(1) failed for artifact copy check`);
  }
  try {
    if (artifactMainXdvCopy(outPtr, 1) !== 0) {
      throw new Error(`${label}: expected main.xdv copy(out,1)=0`);
    }
  } finally {
    dealloc(outPtr, 1);
  }

  const mainName = new TextEncoder().encode('main.xdv');
  const genericLen = callWithBytes(mainName, `${label}_generic_main_len`, (namePtr, nameLen) =>
    artifactLenByName(namePtr, nameLen),
  );
  if (genericLen !== 0) {
    throw new Error(`${label}: expected generic artifact_len(main.xdv)=0, got ${genericLen}`);
  }
  const genericCopyNull = callWithBytes(mainName, `${label}_generic_main_copy_null`, (namePtr, nameLen) =>
    artifactCopyByName(namePtr, nameLen, 0, 0),
  );
  if (genericCopyNull !== 0) {
    throw new Error(`${label}: expected generic artifact_copy(main.xdv,null,0)=0`);
  }
  const genericOutPtr = alloc(1);
  if (!Number.isInteger(genericOutPtr) || genericOutPtr <= 0) {
    throw new Error(`${label}: alloc(1) failed for generic artifact copy check`);
  }
  try {
    const genericCopyOne = callWithBytes(mainName, `${label}_generic_main_copy_one`, (namePtr, nameLen) =>
      artifactCopyByName(namePtr, nameLen, genericOutPtr, 1),
    );
    if (genericCopyOne !== 0) {
      throw new Error(`${label}: expected generic artifact_copy(main.xdv,out,1)=0, got ${genericCopyOne}`);
    }
  } finally {
    dealloc(genericOutPtr, 1);
  }

  const unknownName = new TextEncoder().encode('unknown.bin');
  const unknownLen = callWithBytes(unknownName, `${label}_generic_unknown_len`, (namePtr, nameLen) =>
    artifactLenByName(namePtr, nameLen),
  );
  if (unknownLen !== 0) {
    throw new Error(`${label}: expected generic artifact_len(unknown.bin)=0, got ${unknownLen}`);
  }
  const unknownCopy = callWithBytes(unknownName, `${label}_generic_unknown_copy`, (namePtr, nameLen) =>
    artifactCopyByName(namePtr, nameLen, 0, 0),
  );
  if (unknownCopy !== 0) {
    throw new Error(`${label}: expected generic artifact_copy(unknown.bin,null,0)=0`);
  }
}

function assertNoEvents(label) {
  const bytes = readEventsBytes();
  if (bytes.length !== 0) {
    throw new Error(`${label}: expected zero events for INVALID_INPUT, got ${bytes.length} bytes`);
  }
}

const mainTex = '\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n';
const mainBytes = new TextEncoder().encode(mainTex);
const expectedMainTexStatsExact = {
  control_seq_count: 3,
  begin_group_count: 3,
  end_group_count: 3,
  max_group_depth: 1,
};
const ok = callWithBytes(mainBytes, 'main_tex', (ptr, len) => validate(ptr, len));
if (ok !== 0) {
  throw new Error(`validate failed, code=${ok}`);
}

const rawNonUtf8Main = Uint8Array.from([0xff, 0x0a, 0x58]);
const rawNonUtf8Ok = callWithBytes(rawNonUtf8Main, 'main_tex_raw_non_utf8', (ptr, len) => validate(ptr, len));
if (rawNonUtf8Ok !== 0) {
  throw new Error(`validate(raw non-utf8 main.tex bytes) failed, code=${rawNonUtf8Ok}`);
}

if (mountReset() !== 0) {
  throw new Error('mount_reset failed');
}

const subTexBytes = new TextEncoder().encode('Included file.\\n');
if (addMountedFile('main.tex', mainBytes, 'main') !== 0) {
  throw new Error('mount_add_file(main.tex) failed');
}
if (addMountedFile('sub.tex', subTexBytes, 'sub') !== 0) {
  throw new Error('mount_add_file(sub.tex) failed');
}
const subBinBytes = Uint8Array.from([0xff, 0x58]);
if (addMountedFile('sub.bin', subBinBytes, 'sub_bin') !== 0) {
  throw new Error('mount_add_file(sub.bin) failed');
}

{
  const readMain = readMountedFileBytes('main.tex', 'read_main');
  if (readMain.length <= 0) {
    throw new Error('read_main: expected non-empty bytes');
  }
  if (readMain.length !== mainBytes.length || !readMain.every((byte, index) => byte === mainBytes[index])) {
    throw new Error('read_main: bytes mismatch');
  }
}

{
  const readSub = readMountedFileBytes('sub.tex', 'read_sub');
  if (readSub.length !== subTexBytes.length || !readSub.every((byte, index) => byte === subTexBytes[index])) {
    throw new Error('read_sub: bytes mismatch');
  }
}

{
  const readSubBin = readMountedFileBytes('sub.bin', 'read_sub_bin');
  if (readSubBin.length !== subBinBytes.length || !readSubBin.every((byte, index) => byte === subBinBytes[index])) {
    throw new Error('read_sub_bin: bytes mismatch');
  }
}

assertReadbackZero('missing.tex', 'read_missing');
assertReadbackZero('/abs.tex', 'read_invalid_abs');
assertReadbackZero('a\\\\b.tex', 'read_invalid_backslash');

if (mountFinalize() !== 0) {
  throw new Error('mount_finalize failed');
}

const hasMain = callWithBytes(new TextEncoder().encode('main.tex'), 'has_main_path', (ptr, len) => mountHasFile(ptr, len));
if (hasMain !== 0) {
  throw new Error(`mount_has_file(main.tex) expected 0, got ${hasMain}`);
}

const hasMissing = callWithBytes(new TextEncoder().encode('missing.tex'), 'has_missing_path', (ptr, len) => mountHasFile(ptr, len));
if (hasMissing !== 1) {
  throw new Error(`mount_has_file(missing.tex) expected 1, got ${hasMissing}`);
}

expectNotImplemented(compileMain(), 'compile_main_v0');
let baselineMainCharCount = null;
{
  const report = readCompileReportJson();
  if (report.status !== 'NOT_IMPLEMENTED') {
    throw new Error(`compile_main report.status expected NOT_IMPLEMENTED, got ${report.status}`);
  }
  if (!Array.isArray(report.missing_components) || report.missing_components.length === 0) {
    throw new Error('compile_main report.missing_components expected non-empty array');
  }
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (logBytes.length <= 0 || !logText.startsWith('NOT_IMPLEMENTED:')) {
    throw new Error('compile_main log expected non-empty NOT_IMPLEMENTED prefix');
  }
  if (logBytes.length > 1024) {
    throw new Error(`compile_main log exceeds default max_log_bytes: ${logBytes.length}`);
  }
  const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main');
  baselineMainCharCount = stats.char_count;
  assertMainXdvArtifactEmpty('compile_main');
}

if (compileRequestReset() !== 0) {
  throw new Error('compile_request_reset_v0 failed');
}
const requestEntrypoint = new TextEncoder().encode('main.tex');
const setEntrypointCode = callWithBytes(requestEntrypoint, 'compile_request_entrypoint', (ptr, len) =>
  compileRequestSetEntrypoint(ptr, len),
);
if (setEntrypointCode !== 0) {
  throw new Error(`compile_request_set_entrypoint_v0(main.tex) failed, code=${setEntrypointCode}`);
}
if (compileRequestSetEpoch(1700000000n) !== 0) {
  throw new Error('compile_request_set_source_date_epoch_v0 failed');
}
if (compileRequestSetMaxLogBytes(1024) !== 0) {
  throw new Error('compile_request_set_max_log_bytes_v0 failed');
}
expectNotImplemented(compileRun(), 'compile_run_v0(valid request)');
{
  const report = readCompileReportJson();
  if (report.status !== 'NOT_IMPLEMENTED') {
    throw new Error(`compile_run report.status expected NOT_IMPLEMENTED, got ${report.status}`);
  }
  if (!Array.isArray(report.missing_components) || report.missing_components.length === 0) {
    throw new Error('compile_run report.missing_components expected non-empty array');
  }
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (logBytes.length <= 0 || !logText.startsWith('NOT_IMPLEMENTED:')) {
    throw new Error('compile_run log expected non-empty NOT_IMPLEMENTED prefix');
  }
  if (logBytes.length > 1024) {
    throw new Error(`compile_run log exceeds max_log_bytes: ${logBytes.length}`);
  }
  assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_run(valid request)');
  assertMainXdvArtifactEmpty('compile_run(valid request)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before input expansion positive case failed');
}
const inputExpansionMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input{sub.tex}\n\\end{document}\n');
const inputExpansionSubBytes = new TextEncoder().encode('XYZ');
if (addMountedFile('main.tex', inputExpansionMainBytes, 'input_expansion_main') !== 0) {
  throw new Error('mount_add_file(input expansion main.tex) failed');
}
if (addMountedFile('sub.tex', inputExpansionSubBytes, 'input_expansion_sub') !== 0) {
  throw new Error('mount_add_file(input expansion sub.tex) failed');
}
if (mountFinalize() !== 0) {
  throw new Error('mount_finalize for input expansion positive case failed');
}
expectNotImplemented(compileMain(), 'compile_main_v0(input expansion positive)');
{
  const report = readCompileReportJson();
  if (report.status !== 'NOT_IMPLEMENTED') {
    throw new Error(`compile_main(input expansion) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
  }
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  const tracePrefix = 'INPUT_TRACE_V0:';
  const tracePrefixIndex = logText.indexOf(tracePrefix);
  if (tracePrefixIndex < 0) {
    throw new Error(`compile_main(input expansion) missing ${tracePrefix}`);
  }
  const traceJsonText = logText.slice(tracePrefixIndex + tracePrefix.length);
  const trace = JSON.parse(traceJsonText);
  if (trace.expansions !== 1) {
    throw new Error(`compile_main(input expansion) trace.expansions expected 1, got ${trace.expansions}`);
  }
  if (!Array.isArray(trace.files) || !trace.files.includes('main.tex') || !trace.files.includes('sub.tex')) {
    throw new Error(`compile_main(input expansion) trace.files missing expected paths: ${traceJsonText}`);
  }
  const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(input expansion positive)');
  if (baselineMainCharCount === null) {
    throw new Error('baselineMainCharCount not initialized');
  }
  if (stats.char_count !== baselineMainCharCount + 3) {
    throw new Error(`compile_main(input expansion) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
  }
  assertMainXdvArtifactEmpty('compile_main(input expansion positive)');
}

if (compileRequestReset() !== 0) {
  throw new Error('compile_request_reset_v0 before negative setter tests failed');
}
expectInvalid(
  callWithBytes(new TextEncoder().encode('other.tex'), 'compile_request_bad_entrypoint', (ptr, len) =>
    compileRequestSetEntrypoint(ptr, len),
  ),
  'compile_request_set_entrypoint_v0(other.tex)',
);
expectInvalid(compileRequestSetEpoch(0n), 'compile_request_set_source_date_epoch_v0(0)');
expectInvalid(compileRequestSetMaxLogBytes(0), 'compile_request_set_max_log_bytes_v0(0)');

if (compileRequestReset() !== 0) {
  throw new Error('compile_request_reset_v0 before truncation check failed');
}
const setEntrypointForTruncation = callWithBytes(
  new TextEncoder().encode('main.tex'),
  'compile_request_entrypoint_truncation',
  (ptr, len) => compileRequestSetEntrypoint(ptr, len),
);
if (setEntrypointForTruncation !== 0) {
  throw new Error('compile_request_set_entrypoint_v0(main.tex) for truncation failed');
}
if (compileRequestSetEpoch(1700000000n) !== 0) {
  throw new Error('compile_request_set_source_date_epoch_v0 for truncation failed');
}
if (compileRequestSetMaxLogBytes(8) !== 0) {
  throw new Error('compile_request_set_max_log_bytes_v0(8) failed');
}
expectNotImplemented(compileRun(), 'compile_run_v0(truncation case)');
{
  const logBytes = readCompileLogBytes();
  if (logBytes.length !== 8) {
    throw new Error(`compile_run truncated log expected 8 bytes, got ${logBytes.length}`);
  }
  const logText = new TextDecoder().decode(logBytes);
  if (logText.includes('INPUT_TRACE_V0:')) {
    throw new Error(`compile_run truncated log must omit INPUT_TRACE_V0 marker, got: ${logText}`);
  }
  assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_run(truncation case)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before invalid tokenize compile check failed');
}
const invalidTokenMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello\\');
if (addMountedFile('main.tex', invalidTokenMainBytes, 'invalid_token_main') !== 0) {
  throw new Error('mount_add_file(invalid tokenize main.tex) failed');
}
if (mountFinalize() !== 0) {
  throw new Error('mount_finalize for invalid tokenize main.tex failed');
}
expectInvalid(compileMain(), 'compile_main_v0(invalid tokenize main.tex)');
{
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenize_failed')) {
    throw new Error(`compile_main invalid tokenize log mismatch: ${logText}`);
  }
  assertNoEvents('compile_main_v0(invalid tokenize main.tex)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before whitespace-only compile check failed');
}
const whitespaceOnlyMainBytes = new TextEncoder().encode(' \n\t');
if (addMountedFile('main.tex', whitespaceOnlyMainBytes, 'invalid_whitespace_main') !== 0) {
  throw new Error('mount_add_file(whitespace-only main.tex) failed');
}
const whitespaceFinalizeCode = mountFinalize();
if (whitespaceFinalizeCode !== 0 && whitespaceFinalizeCode !== 1) {
  throw new Error(`mount_finalize(whitespace-only main.tex) unexpected code=${whitespaceFinalizeCode}`);
}
expectInvalid(compileMain(), 'compile_main_v0(whitespace-only main.tex)');
{
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('mount_finalize_failed')) {
    throw new Error(`compile_main whitespace-only log mismatch: ${logText}`);
  }
  assertNoEvents('compile_main_v0(whitespace-only main.tex)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before verb compile check failed');
}
const verbMainBytes = new TextEncoder().encode('\\verb|x|\n');
if (addMountedFile('main.tex', verbMainBytes, 'invalid_verb_main') !== 0) {
  throw new Error('mount_add_file(verb main.tex) failed');
}
const verbFinalizeCode = mountFinalize();
if (verbFinalizeCode !== 0 && verbFinalizeCode !== 1) {
  throw new Error(`mount_finalize(verb main.tex) unexpected code=${verbFinalizeCode}`);
}
expectInvalid(compileMain(), 'compile_main_v0(verb main.tex)');
{
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenize_failed')) {
    throw new Error(`compile_main verb log mismatch: ${logText}`);
  }
  assertNoEvents('compile_main_v0(verb main.tex)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before missing-input compile check failed');
}
const missingInputMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{missing.tex}\n\\end{document}\n');
if (addMountedFile('main.tex', missingInputMainBytes, 'missing_input_main') !== 0) {
  throw new Error('mount_add_file(missing input main.tex) failed');
}
const missingInputFinalizeCode = mountFinalize();
if (missingInputFinalizeCode !== 0 && missingInputFinalizeCode !== 1) {
  throw new Error(`mount_finalize(missing input main.tex) unexpected code=${missingInputFinalizeCode}`);
}
expectInvalid(compileMain(), 'compile_main_v0(missing input file)');
{
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_validation_failed')) {
    throw new Error(`compile_main missing-input log mismatch: ${logText}`);
  }
  assertNoEvents('compile_main_v0(missing input file)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset before input cycle compile check failed');
}
const cycleMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{a.tex}\n\\end{document}\n');
const cycleSubBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{main.tex}\n\\end{document}\n');
if (addMountedFile('main.tex', cycleMainBytes, 'input_cycle_main') !== 0) {
  throw new Error('mount_add_file(input cycle main.tex) failed');
}
if (addMountedFile('a.tex', cycleSubBytes, 'input_cycle_sub') !== 0) {
  throw new Error('mount_add_file(input cycle a.tex) failed');
}
const cycleFinalizeCode = mountFinalize();
if (cycleFinalizeCode !== 0 && cycleFinalizeCode !== 1) {
  throw new Error(`mount_finalize(input cycle) unexpected code=${cycleFinalizeCode}`);
}
expectInvalid(compileMain(), 'compile_main_v0(input cycle)');
{
  const logBytes = readCompileLogBytes();
  const logText = new TextDecoder().decode(logBytes);
  if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_cycle_failed')) {
    throw new Error(`compile_main input cycle log mismatch: ${logText}`);
  }
  assertNoEvents('compile_main_v0(input cycle)');
}

if (mountReset() !== 0) {
  throw new Error('mount_reset for negative cases failed');
}

expectInvalid(addMountedFile('/abs.tex', mainBytes, 'neg_abs'), 'mount_add_file(/abs.tex)');
expectInvalid(addMountedFile('../up.tex', mainBytes, 'neg_up'), 'mount_add_file(../up.tex)');
expectInvalid(addMountedFile('a/../b.tex', mainBytes, 'neg_traversal'), 'mount_add_file(a/../b.tex)');
expectInvalid(addMountedFile('a\\\\b.tex', mainBytes, 'neg_backslash'), 'mount_add_file(a\\\\b.tex)');
expectInvalid(addMountedFile('', mainBytes, 'neg_empty'), 'mount_add_file(empty)');
expectInvalid(addMountedFile('a//b.tex', mainBytes, 'neg_empty_segment'), 'mount_add_file(a//b.tex)');
expectInvalid(addMountedFile('a/b/', mainBytes, 'neg_trailing_slash'), 'mount_add_file(a/b/)');

console.log('PASS: JS loaded WASM and exercised ABI (alloc/validate/mount/compile/report)');
