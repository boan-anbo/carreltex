import { readFile } from 'node:fs/promises';
import path from 'node:path';

export async function createCtx(rootDir) {
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
  const addResult = add(1, 2);
  if (addResult !== 3) {
    throw new Error(`Unexpected result: ${addResult}`);
  }

  const { memory } = instance.exports;
  if (!(memory instanceof WebAssembly.Memory)) {
    throw new Error('Missing export: memory');
  }

  const ctx = {
    rootDir,
    memory,
    alloc: instance.exports.carreltex_wasm_alloc,
    dealloc: instance.exports.carreltex_wasm_dealloc,
    validate: instance.exports.carreltex_wasm_validate_main_tex,
    mountReset: instance.exports.carreltex_wasm_mount_reset,
    mountAddFile: instance.exports.carreltex_wasm_mount_add_file,
    mountFinalize: instance.exports.carreltex_wasm_mount_finalize,
    mountHasFile: instance.exports.carreltex_wasm_mount_has_file,
    mountReadFileLen: instance.exports.carreltex_wasm_mount_read_file_len_v0,
    mountReadFileCopy: instance.exports.carreltex_wasm_mount_read_file_copy_v0,
    compileMain: instance.exports.carreltex_wasm_compile_main_v0,
    compileRequestReset: instance.exports.carreltex_wasm_compile_request_reset_v0,
    compileRequestSetEntrypoint: instance.exports.carreltex_wasm_compile_request_set_entrypoint_v0,
    compileRequestSetEpoch: instance.exports.carreltex_wasm_compile_request_set_source_date_epoch_v0,
    compileRequestSetMaxLogBytes: instance.exports.carreltex_wasm_compile_request_set_max_log_bytes_v0,
    compileRequestSetOkMaxLineGlyphs: instance.exports.carreltex_wasm_compile_request_set_ok_max_line_glyphs_v0,
    compileRequestSetOkMaxLinesPerPage: instance.exports.carreltex_wasm_compile_request_set_ok_max_lines_per_page_v0,
    compileRequestSetOkLineAdvanceSp: instance.exports.carreltex_wasm_compile_request_set_ok_line_advance_sp_v0,
    compileRequestSetOkGlyphAdvanceSp: instance.exports.carreltex_wasm_compile_request_set_ok_glyph_advance_sp_v0,
    compileRun: instance.exports.carreltex_wasm_compile_run_v0,
    reportLen: instance.exports.carreltex_wasm_compile_report_len_v0,
    reportCopy: instance.exports.carreltex_wasm_compile_report_copy_v0,
    logLen: instance.exports.carreltex_wasm_compile_log_len_v0,
    logCopy: instance.exports.carreltex_wasm_compile_log_copy_v0,
    eventsLen: instance.exports.carreltex_wasm_events_len_v0,
    eventsCopy: instance.exports.carreltex_wasm_events_copy_v0,
    artifactMainXdvLen: instance.exports.carreltex_wasm_artifact_main_xdv_len_v0,
    artifactMainXdvCopy: instance.exports.carreltex_wasm_artifact_main_xdv_copy_v0,
    artifactLenByName: instance.exports.carreltex_wasm_artifact_len_v0,
    artifactCopyByName: instance.exports.carreltex_wasm_artifact_copy_v0,
  };

  for (const [name, fn] of [
    ['carreltex_wasm_alloc', ctx.alloc],
    ['carreltex_wasm_dealloc', ctx.dealloc],
    ['carreltex_wasm_validate_main_tex', ctx.validate],
    ['carreltex_wasm_mount_reset', ctx.mountReset],
    ['carreltex_wasm_mount_add_file', ctx.mountAddFile],
    ['carreltex_wasm_mount_finalize', ctx.mountFinalize],
    ['carreltex_wasm_mount_has_file', ctx.mountHasFile],
    ['carreltex_wasm_mount_read_file_len_v0', ctx.mountReadFileLen],
    ['carreltex_wasm_mount_read_file_copy_v0', ctx.mountReadFileCopy],
    ['carreltex_wasm_compile_main_v0', ctx.compileMain],
    ['carreltex_wasm_compile_request_reset_v0', ctx.compileRequestReset],
    ['carreltex_wasm_compile_request_set_entrypoint_v0', ctx.compileRequestSetEntrypoint],
    ['carreltex_wasm_compile_request_set_source_date_epoch_v0', ctx.compileRequestSetEpoch],
    ['carreltex_wasm_compile_request_set_max_log_bytes_v0', ctx.compileRequestSetMaxLogBytes],
    ['carreltex_wasm_compile_request_set_ok_max_line_glyphs_v0', ctx.compileRequestSetOkMaxLineGlyphs],
    ['carreltex_wasm_compile_request_set_ok_max_lines_per_page_v0', ctx.compileRequestSetOkMaxLinesPerPage],
    ['carreltex_wasm_compile_request_set_ok_line_advance_sp_v0', ctx.compileRequestSetOkLineAdvanceSp],
    ['carreltex_wasm_compile_request_set_ok_glyph_advance_sp_v0', ctx.compileRequestSetOkGlyphAdvanceSp],
    ['carreltex_wasm_compile_run_v0', ctx.compileRun],
    ['carreltex_wasm_compile_report_len_v0', ctx.reportLen],
    ['carreltex_wasm_compile_report_copy_v0', ctx.reportCopy],
    ['carreltex_wasm_compile_log_len_v0', ctx.logLen],
    ['carreltex_wasm_compile_log_copy_v0', ctx.logCopy],
    ['carreltex_wasm_events_len_v0', ctx.eventsLen],
    ['carreltex_wasm_events_copy_v0', ctx.eventsCopy],
    ['carreltex_wasm_artifact_main_xdv_len_v0', ctx.artifactMainXdvLen],
    ['carreltex_wasm_artifact_main_xdv_copy_v0', ctx.artifactMainXdvCopy],
    ['carreltex_wasm_artifact_len_v0', ctx.artifactLenByName],
    ['carreltex_wasm_artifact_copy_v0', ctx.artifactCopyByName],
  ]) {
    if (typeof fn !== 'function') {
      throw new Error(`Missing export: ${name}`);
    }
  }

  return ctx;
}
