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
const compileMain = instance.exports.carreltex_wasm_compile_main_v0;
const reportLen = instance.exports.carreltex_wasm_compile_report_len_v0;
const reportCopy = instance.exports.carreltex_wasm_compile_report_copy_v0;

for (const [name, fn] of [
  ['carreltex_wasm_alloc', alloc],
  ['carreltex_wasm_dealloc', dealloc],
  ['carreltex_wasm_validate_main_tex', validate],
  ['carreltex_wasm_mount_reset', mountReset],
  ['carreltex_wasm_mount_add_file', mountAddFile],
  ['carreltex_wasm_mount_finalize', mountFinalize],
  ['carreltex_wasm_mount_has_file', mountHasFile],
  ['carreltex_wasm_compile_main_v0', compileMain],
  ['carreltex_wasm_compile_report_len_v0', reportLen],
  ['carreltex_wasm_compile_report_copy_v0', reportCopy],
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

function expectInvalid(value, label) {
  if (value !== 1) {
    throw new Error(`${label} expected invalid(1), got ${value}`);
  }
}

const mainTex = '\\documentclass{article}\\n\\\\begin{document}\\nHello.\\n\\\\end{document}\\n';
const mainBytes = new TextEncoder().encode(mainTex);
const ok = callWithBytes(mainBytes, 'main_tex', (ptr, len) => validate(ptr, len));
if (ok !== 0) {
  throw new Error(`validate failed, code=${ok}`);
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

const compileCode = compileMain();
if (compileCode !== 2) {
  throw new Error(`compile_main_v0 expected NOT_IMPLEMENTED(2), got ${compileCode}`);
}

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
  const report = JSON.parse(text);
  if (report.status !== 'NOT_IMPLEMENTED') {
    throw new Error(`report.status expected NOT_IMPLEMENTED, got ${report.status}`);
  }
  if (!Array.isArray(report.missing_components) || report.missing_components.length === 0) {
    throw new Error('report.missing_components expected non-empty array');
  }
} finally {
  dealloc(outPtr, jsonLen);
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
