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

console.log('PASS: JS loaded WASM and called export (1+2=3)');

