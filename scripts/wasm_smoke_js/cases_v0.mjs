import { runMeaningCases } from './cases_v0_meaning.mjs';
import { runCountCases } from './cases_v0_count.mjs';
import { runEdefCases } from './cases_v0_edef.mjs';
import { runXdefNoexpandCases } from './cases_v0_xdef_noexpand.mjs';
import { runIfnumCases } from './cases_v0_ifnum.mjs';
import { runIfxCases } from './cases_v0_ifx.mjs';
import { runTokenizerCases } from './cases_v0_tokenizer.mjs';
import { runTokenizerTextwordLeaf133Cases } from './cases_v0_tokenizer_textword_133.mjs';
import { runTokenizerTextwordLeaf134Cases } from './cases_v0_tokenizer_textword_134.mjs';
import { runTokenizerTextwordLeaf135Cases } from './cases_v0_tokenizer_textword_135.mjs';
import { runTokenizerTextwordLeaf137Cases } from './cases_v0_tokenizer_textword_137.mjs';
import { runTokenizerTextwordLeaf138Cases } from './cases_v0_tokenizer_textword_138.mjs';
import { runTokenizerTextwordLeaf139Cases } from './cases_v0_tokenizer_textword_139.mjs';
import { runTokenizerTextwordLeaf140Cases } from './cases_v0_tokenizer_textword_140.mjs';
import { runTokenizerTextwordLeaf141Cases } from './cases_v0_tokenizer_textword_141.mjs';
import { runTokenizerTextwordLeaf142Cases } from './cases_v0_tokenizer_textword_142.mjs';
import { runTokenizerTextwordLeaf143Cases } from './cases_v0_tokenizer_textword_143.mjs';
import { runMacroCases } from './cases_v0_macro.mjs';

export function runCasesV0(ctx, mem, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectNotImplemented,
    readCompileReportJson,
    readCompileLogBytes,
    readEventsBytes,
    decodeEvents,
    assertEventsMatchLogAndStats,
    readMountedFileBytes,
    assertReadbackZero,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  } = helpers;

  const mainTex = '\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n';
  const mainBytes = new TextEncoder().encode(mainTex);
  const expectedMainTexStatsExact = {
    control_seq_count: 3,
    begin_group_count: 3,
    end_group_count: 3,
    max_group_depth: 1,
  };
  const ok = mem.callWithBytes(mainBytes, 'main_tex', (ptr, len) => ctx.validate(ptr, len));
  if (ok !== 0) {
    throw new Error(`validate failed, code=${ok}`);
  }

  const rawNonUtf8Main = Uint8Array.from([0xff, 0x0a, 0x58]);
  const rawNonUtf8Ok = mem.callWithBytes(rawNonUtf8Main, 'main_tex_raw_non_utf8', (ptr, len) => ctx.validate(ptr, len));
  if (rawNonUtf8Ok !== 0) {
    throw new Error(`validate(raw non-utf8 main.tex bytes) failed, code=${rawNonUtf8Ok}`);
  }

  if (ctx.mountReset() !== 0) {
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

  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize failed');
  }

  const hasMain = mem.callWithBytes(new TextEncoder().encode('main.tex'), 'has_main_path', (ptr, len) => ctx.mountHasFile(ptr, len));
  if (hasMain !== 0) {
    throw new Error(`mount_has_file(main.tex) expected 0, got ${hasMain}`);
  }

  const hasMissing = mem.callWithBytes(new TextEncoder().encode('missing.tex'), 'has_missing_path', (ptr, len) => ctx.mountHasFile(ptr, len));
  if (hasMissing !== 1) {
    throw new Error(`mount_has_file(missing.tex) expected 1, got ${hasMissing}`);
  }

  expectNotImplemented(ctx.compileMain(), 'compile_main_v0');
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

  if (ctx.compileRequestReset() !== 0) {
    throw new Error('compile_request_reset_v0 failed');
  }
  const requestEntrypoint = new TextEncoder().encode('main.tex');
  const setEntrypointCode = mem.callWithBytes(requestEntrypoint, 'compile_request_entrypoint', (ptr, len) =>
    ctx.compileRequestSetEntrypoint(ptr, len),
  );
  if (setEntrypointCode !== 0) {
    throw new Error(`compile_request_set_entrypoint_v0(main.tex) failed, code=${setEntrypointCode}`);
  }
  if (ctx.compileRequestSetEpoch(1700000000n) !== 0) {
    throw new Error('compile_request_set_source_date_epoch_v0 failed');
  }
  if (ctx.compileRequestSetMaxLogBytes(1024) !== 0) {
    throw new Error('compile_request_set_max_log_bytes_v0 failed');
  }
  expectNotImplemented(ctx.compileRun(), 'compile_run_v0(valid request)');
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

  if (ctx.mountReset() !== 0) {
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
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input expansion positive case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input expansion positive)');
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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before unbraced input expansion case failed');
  }
  const unbracedInputMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input sub\\foo\n\\end{document}\n');
  const unbracedInputSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', unbracedInputMainBytes, 'input_unbraced_main') !== 0) {
    throw new Error('mount_add_file(input unbraced main.tex) failed');
  }
  if (addMountedFile('sub.tex', unbracedInputSubBytes, 'input_unbraced_sub') !== 0) {
    throw new Error('mount_add_file(input unbraced sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input unbraced case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input unbraced + default .tex)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(input unbraced) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(input unbraced + default .tex)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(input unbraced) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    const logText = new TextDecoder().decode(logBytes);
    const tracePrefix = 'INPUT_TRACE_V0:';
    const tracePrefixIndex = logText.indexOf(tracePrefix);
    if (tracePrefixIndex < 0) {
      throw new Error(`compile_main(input unbraced) missing ${tracePrefix}`);
    }
    const traceJsonText = logText.slice(tracePrefixIndex + tracePrefix.length);
    const trace = JSON.parse(traceJsonText);
    if (!Array.isArray(trace.files) || !trace.files.includes('sub.tex')) {
      throw new Error(`compile_main(input unbraced) trace.files missing resolved sub.tex: ${traceJsonText}`);
    }
    assertMainXdvArtifactEmpty('compile_main(input unbraced + default .tex)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before braced input default-extension case failed');
  }
  const bracedNoExtMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input{sub}\\foo\n\\end{document}\n');
  const bracedNoExtSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', bracedNoExtMainBytes, 'input_braced_no_ext_main') !== 0) {
    throw new Error('mount_add_file(input braced no-ext main.tex) failed');
  }
  if (addMountedFile('sub.tex', bracedNoExtSubBytes, 'input_braced_no_ext_sub') !== 0) {
    throw new Error('mount_add_file(input braced no-ext sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input braced no-ext case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input braced default .tex)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(input braced no-ext) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(input braced default .tex)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(input braced no-ext) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(input braced default .tex)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before unbraced explicit .tex + control-seq case failed');
  }
  const unbracedExplicitTexMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input sub.tex\\foo\n\\end{document}\n');
  const unbracedExplicitTexSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', unbracedExplicitTexMainBytes, 'input_unbraced_explicit_tex_main') !== 0) {
    throw new Error('mount_add_file(input unbraced explicit .tex main.tex) failed');
  }
  if (addMountedFile('sub.tex', unbracedExplicitTexSubBytes, 'input_unbraced_explicit_tex_sub') !== 0) {
    throw new Error('mount_add_file(input unbraced explicit .tex sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input unbraced explicit .tex + control-seq case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input unbraced explicit .tex + control-seq)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(input unbraced explicit .tex + control-seq) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(input unbraced explicit .tex + control-seq)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(input unbraced explicit .tex + control-seq) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    const logText = new TextDecoder().decode(logBytes);
    const tracePrefix = 'INPUT_TRACE_V0:';
    const tracePrefixIndex = logText.indexOf(tracePrefix);
    if (tracePrefixIndex < 0) {
      throw new Error(`compile_main(input unbraced explicit .tex + control-seq) missing ${tracePrefix}`);
    }
    const traceJsonText = logText.slice(tracePrefixIndex + tracePrefix.length);
    const trace = JSON.parse(traceJsonText);
    if (!Array.isArray(trace.files) || !trace.files.includes('sub.tex')) {
      throw new Error(`compile_main(input unbraced explicit .tex + control-seq) trace.files missing resolved sub.tex: ${traceJsonText}`);
    }
    assertMainXdvArtifactEmpty('compile_main(input unbraced explicit .tex + control-seq)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before unbraced dash filename case failed');
  }
  const unbracedDashMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input sub-1\\foo\n\\end{document}\n');
  const unbracedDashSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', unbracedDashMainBytes, 'input_unbraced_dash_main') !== 0) {
    throw new Error('mount_add_file(input unbraced dash main.tex) failed');
  }
  if (addMountedFile('sub-1.tex', unbracedDashSubBytes, 'input_unbraced_dash_sub') !== 0) {
    throw new Error('mount_add_file(input unbraced dash sub-1.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input unbraced dash filename case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input unbraced dash filename)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(input unbraced dash filename) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(input unbraced dash filename)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(input unbraced dash filename) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    const logText = new TextDecoder().decode(logBytes);
    const tracePrefix = 'INPUT_TRACE_V0:';
    const tracePrefixIndex = logText.indexOf(tracePrefix);
    if (tracePrefixIndex < 0) {
      throw new Error(`compile_main(input unbraced dash filename) missing ${tracePrefix}`);
    }
    const traceJsonText = logText.slice(tracePrefixIndex + tracePrefix.length);
    const trace = JSON.parse(traceJsonText);
    if (!Array.isArray(trace.files) || !trace.files.includes('sub-1.tex')) {
      throw new Error(`compile_main(input unbraced dash filename) trace.files missing resolved sub-1.tex: ${traceJsonText}`);
    }
    assertMainXdvArtifactEmpty('compile_main(input unbraced dash filename)');
  }

  const { gdefBaselineCharCount } = runMacroCases(
    ctx,
    {
      addMountedFile,
      expectInvalid,
      expectNotImplemented,
      readCompileReportJson,
      readCompileLogBytes,
      readEventsBytes,
      decodeEvents,
      assertEventsMatchLogAndStats,
      assertMainXdvArtifactEmpty,
      assertNoEvents,
    },
    baselineMainCharCount,
  );

  runMeaningCases(ctx, {
    addMountedFile,
    expectInvalid,
    expectNotImplemented,
    readCompileReportJson,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
    gdefBaselineCharCount,
  });

  if (gdefBaselineCharCount === null) {
    throw new Error('gdefBaselineCharCount not initialized before count cases');
  }
  runCountCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runEdefCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runXdefNoexpandCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runIfnumCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runIfxCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runTokenizerCases(ctx, { addMountedFile, expectInvalid, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty, assertNoEvents });
  runTokenizerTextwordLeaf133Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf134Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf135Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf137Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf138Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf139Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf140Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf141Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf142Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });
  runTokenizerTextwordLeaf143Cases(ctx, { addMountedFile, expectNotImplemented, readCompileLogBytes, assertEventsMatchLogAndStats, assertMainXdvArtifactEmpty });

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  if (addMountedFile('main.tex', mainBytes, 'compile_request_base_main') !== 0) {
    throw new Error('mount_add_file(main.tex) before compile_request negative setter tests failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize before compile_request negative setter tests failed');
  }

  if (ctx.compileRequestReset() !== 0) {
    throw new Error('compile_request_reset_v0 before negative setter tests failed');
  }
  expectInvalid(
    mem.callWithBytes(new TextEncoder().encode('other.tex'), 'compile_request_bad_entrypoint', (ptr, len) =>
      ctx.compileRequestSetEntrypoint(ptr, len),
    ),
    'compile_request_set_entrypoint_v0(other.tex)',
  );
  expectInvalid(ctx.compileRequestSetEpoch(0n), 'compile_request_set_source_date_epoch_v0(0)');
  expectInvalid(ctx.compileRequestSetMaxLogBytes(0), 'compile_request_set_max_log_bytes_v0(0)');

  if (ctx.compileRequestReset() !== 0) {
    throw new Error('compile_request_reset_v0 before truncation check failed');
  }
  const setEntrypointForTruncation = mem.callWithBytes(
    new TextEncoder().encode('main.tex'),
    'compile_request_entrypoint_truncation',
    (ptr, len) => ctx.compileRequestSetEntrypoint(ptr, len),
  );
  if (setEntrypointForTruncation !== 0) {
    throw new Error('compile_request_set_entrypoint_v0(main.tex) for truncation failed');
  }
  if (ctx.compileRequestSetEpoch(1700000000n) !== 0) {
    throw new Error('compile_request_set_source_date_epoch_v0 for truncation failed');
  }
  if (ctx.compileRequestSetMaxLogBytes(8) !== 0) {
    throw new Error('compile_request_set_max_log_bytes_v0(8) failed');
  }
  expectNotImplemented(ctx.compileRun(), 'compile_run_v0(truncation case)');
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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before invalid tokenize compile check failed');
  }
  const invalidTokenMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello\\');
  if (addMountedFile('main.tex', invalidTokenMainBytes, 'invalid_token_main') !== 0) {
    throw new Error('mount_add_file(invalid tokenize main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for invalid tokenize main.tex failed');
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(invalid tokenize main.tex)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenize_failed')) {
      throw new Error(`compile_main invalid tokenize log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(invalid tokenize main.tex)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before whitespace-only compile check failed');
  }
  const whitespaceOnlyMainBytes = new TextEncoder().encode(' \n\t');
  if (addMountedFile('main.tex', whitespaceOnlyMainBytes, 'invalid_whitespace_main') !== 0) {
    throw new Error('mount_add_file(whitespace-only main.tex) failed');
  }
  const whitespaceFinalizeCode = ctx.mountFinalize();
  if (whitespaceFinalizeCode !== 0 && whitespaceFinalizeCode !== 1) {
    throw new Error(`mount_finalize(whitespace-only main.tex) unexpected code=${whitespaceFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(whitespace-only main.tex)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('mount_finalize_failed')) {
      throw new Error(`compile_main whitespace-only log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(whitespace-only main.tex)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before verb compile check failed');
  }
  const verbMainBytes = new TextEncoder().encode('\\verb|x|\n');
  if (addMountedFile('main.tex', verbMainBytes, 'invalid_verb_main') !== 0) {
    throw new Error('mount_add_file(verb main.tex) failed');
  }
  const verbFinalizeCode = ctx.mountFinalize();
  if (verbFinalizeCode !== 0 && verbFinalizeCode !== 1) {
    throw new Error(`mount_finalize(verb main.tex) unexpected code=${verbFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(verb main.tex)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenize_failed')) {
      throw new Error(`compile_main verb log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(verb main.tex)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before missing-input compile check failed');
  }
  const missingInputMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{missing.tex}\n\\end{document}\n');
  if (addMountedFile('main.tex', missingInputMainBytes, 'missing_input_main') !== 0) {
    throw new Error('mount_add_file(missing input main.tex) failed');
  }
  const missingInputFinalizeCode = ctx.mountFinalize();
  if (missingInputFinalizeCode !== 0 && missingInputFinalizeCode !== 1) {
    throw new Error(`mount_finalize(missing input main.tex) unexpected code=${missingInputFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(missing input file)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_validation_failed')) {
      throw new Error(`compile_main missing-input log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(missing input file)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before missing-input-unbraced compile check failed');
  }
  const missingUnbracedInputMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input missing\n\\end{document}\n');
  if (addMountedFile('main.tex', missingUnbracedInputMainBytes, 'missing_unbraced_input_main') !== 0) {
    throw new Error('mount_add_file(missing unbraced input main.tex) failed');
  }
  const missingUnbracedInputFinalizeCode = ctx.mountFinalize();
  if (missingUnbracedInputFinalizeCode !== 0 && missingUnbracedInputFinalizeCode !== 1) {
    throw new Error(`mount_finalize(missing unbraced input main.tex) unexpected code=${missingUnbracedInputFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(missing unbraced input file)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_validation_failed')) {
      throw new Error(`compile_main missing-unbraced-input log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(missing unbraced input file)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before invalid-unbraced-input syntax compile check failed');
  }
  const invalidUnbracedInputMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', invalidUnbracedInputMainBytes, 'invalid_unbraced_input_main') !== 0) {
    throw new Error('mount_add_file(invalid unbraced input syntax main.tex) failed');
  }
  const invalidUnbracedInputFinalizeCode = ctx.mountFinalize();
  if (invalidUnbracedInputFinalizeCode !== 0 && invalidUnbracedInputFinalizeCode !== 1) {
    throw new Error(`mount_finalize(invalid unbraced input syntax main.tex) unexpected code=${invalidUnbracedInputFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(invalid unbraced input syntax)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_validation_failed')) {
      throw new Error(`compile_main invalid-unbraced-input log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(invalid unbraced input syntax)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before invalid-unbraced-brace-boundary compile check failed');
  }
  const invalidUnbracedBraceBoundaryMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input sub{}\n\\end{document}\n');
  if (addMountedFile('main.tex', invalidUnbracedBraceBoundaryMainBytes, 'invalid_unbraced_brace_boundary_main') !== 0) {
    throw new Error('mount_add_file(invalid unbraced brace-boundary main.tex) failed');
  }
  if (addMountedFile('sub.tex', new TextEncoder().encode('\\def\\foo{XYZ}'), 'invalid_unbraced_brace_boundary_sub') !== 0) {
    throw new Error('mount_add_file(invalid unbraced brace-boundary sub.tex) failed');
  }
  const invalidUnbracedBraceBoundaryFinalizeCode = ctx.mountFinalize();
  if (invalidUnbracedBraceBoundaryFinalizeCode !== 0 && invalidUnbracedBraceBoundaryFinalizeCode !== 1) {
    throw new Error(`mount_finalize(invalid unbraced brace-boundary main.tex) unexpected code=${invalidUnbracedBraceBoundaryFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(invalid unbraced brace-boundary)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_validation_failed')) {
      throw new Error(`compile_main invalid-unbraced-brace-boundary log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(invalid unbraced brace-boundary)');
  }

  if (ctx.mountReset() !== 0) {
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
  const cycleFinalizeCode = ctx.mountFinalize();
  if (cycleFinalizeCode !== 0 && cycleFinalizeCode !== 1) {
    throw new Error(`mount_finalize(input cycle) unexpected code=${cycleFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(input cycle)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('input_cycle_failed')) {
      throw new Error(`compile_main input cycle log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(input cycle)');
  }

  if (ctx.mountReset() !== 0) {
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
}
