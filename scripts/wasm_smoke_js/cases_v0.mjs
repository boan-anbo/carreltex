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
    throw new Error('mount_reset before macro expansion positive case failed');
  }
  const macroExpansionMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\def\\foo{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', macroExpansionMainBytes, 'macro_expansion_main') !== 0) {
    throw new Error('mount_add_file(macro expansion main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro expansion positive case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro expansion positive)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro expansion) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(macro expansion positive)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(macro expansion) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro expansion positive)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro cycle compile check failed');
  }
  const macroCycleMainBytes = new TextEncoder().encode('\\def\\foo{\\foo}\\foo');
  if (addMountedFile('main.tex', macroCycleMainBytes, 'macro_cycle_main') !== 0) {
    throw new Error('mount_add_file(macro cycle main.tex) failed');
  }
  const macroCycleFinalizeCode = ctx.mountFinalize();
  if (macroCycleFinalizeCode !== 0 && macroCycleFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro cycle) unexpected code=${macroCycleFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro cycle)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_cycle_failed')) {
      throw new Error(`compile_main macro cycle log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro cycle)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro single-param positive case failed');
  }
  const macroSingleParamMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\def\\foo#1{#1}\\foo{XYZ}\n\\end{document}\n');
  if (addMountedFile('main.tex', macroSingleParamMainBytes, 'macro_single_param_main') !== 0) {
    throw new Error('mount_add_file(macro single-param main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro single-param positive case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro single-param positive)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro single-param) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, expectedMainTexStatsExact, 'compile_main(macro single-param positive)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(macro single-param) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro single-param positive)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro params unsupported check failed');
  }
  const macroParamsUnsupportedMainBytes = new TextEncoder().encode('\\def\\foo#2{A}\\foo{X}');
  if (addMountedFile('main.tex', macroParamsUnsupportedMainBytes, 'macro_params_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro params unsupported main.tex) failed');
  }
  const macroParamsUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroParamsUnsupportedFinalizeCode !== 0 && macroParamsUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro params unsupported) unexpected code=${macroParamsUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro params unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_params_unsupported')) {
      throw new Error(`compile_main macro params unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro params unsupported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro scoped-no-leak baseline case failed');
  }
  const macroScopedBaselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', macroScopedBaselineMainBytes, 'macro_scoped_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro scoped baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro scoped-no-leak baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro scoped-no-leak baseline)');
  let scopedNoLeakBaselineCharCount = null;
  {
    const baselineLogBytes = readCompileLogBytes();
    const baselineEvents = decodeEvents(readEventsBytes(), 'compile_main(macro scoped-no-leak baseline)');
    if (baselineEvents.length !== 2 || baselineEvents[0].kind !== 1 || baselineEvents[1].kind !== 2) {
      throw new Error('compile_main(macro scoped-no-leak baseline): event shape mismatch');
    }
    if (
      baselineEvents[0].payload.length !== baselineLogBytes.length
      || !baselineEvents[0].payload.every((byte, index) => byte === baselineLogBytes[index])
    ) {
      throw new Error('compile_main(macro scoped-no-leak baseline): event[0] payload mismatch');
    }
    const statsText = new TextDecoder('utf-8', { fatal: true }).decode(baselineEvents[1].payload);
    const stats = JSON.parse(statsText);
    scopedNoLeakBaselineCharCount = stats.char_count;
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro scoped-no-leak case failed');
  }
  const macroScopedNoLeakMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n{\\def\\foo{XYZ}}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', macroScopedNoLeakMainBytes, 'macro_scoped_no_leak_main') !== 0) {
    throw new Error('mount_add_file(macro scoped-no-leak main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro scoped-no-leak case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro scoped-no-leak)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro scoped-no-leak) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const eventsBytes = readEventsBytes();
    const events = decodeEvents(eventsBytes, 'compile_main(macro scoped-no-leak)');
    if (events.length !== 2 || events[0].kind !== 1 || events[1].kind !== 2) {
      throw new Error('compile_main(macro scoped-no-leak): event shape mismatch');
    }
    if (
      events[0].payload.length !== logBytes.length
      || !events[0].payload.every((byte, index) => byte === logBytes[index])
    ) {
      throw new Error('compile_main(macro scoped-no-leak): event[0] payload mismatch');
    }
    const statsText = new TextDecoder('utf-8', { fatal: true }).decode(events[1].payload);
    if (/[ \t\r\n]/.test(statsText) || statsText.includes('"unexpected_key"')) {
      throw new Error(`compile_main(macro scoped-no-leak): invalid stats json text: ${statsText}`);
    }
    const stats = JSON.parse(statsText);
    if (typeof stats !== 'object' || stats === null || typeof stats.char_count !== 'number') {
      throw new Error('compile_main(macro scoped-no-leak): invalid stats json object');
    }
    if (scopedNoLeakBaselineCharCount === null) {
      throw new Error('scopedNoLeakBaselineCharCount not initialized');
    }
    if (stats.char_count !== scopedNoLeakBaselineCharCount) {
      throw new Error(`compile_main(macro scoped-no-leak) char_count delta expected +0, got baseline=${scopedNoLeakBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro scoped-no-leak)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro gdef-global baseline case failed');
  }
  const macroGdefBaselineMainBytes = new TextEncoder().encode('{}\\foo');
  if (addMountedFile('main.tex', macroGdefBaselineMainBytes, 'macro_gdef_global_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro gdef-global baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro gdef-global baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro gdef-global baseline)');
  let gdefBaselineCharCount = null;
  {
    const baselineLogBytes = readCompileLogBytes();
    const baselineEvents = decodeEvents(readEventsBytes(), 'compile_main(macro gdef-global baseline)');
    if (baselineEvents.length !== 2 || baselineEvents[0].kind !== 1 || baselineEvents[1].kind !== 2) {
      throw new Error('compile_main(macro gdef-global baseline): event shape mismatch');
    }
    if (
      baselineEvents[0].payload.length !== baselineLogBytes.length
      || !baselineEvents[0].payload.every((byte, index) => byte === baselineLogBytes[index])
    ) {
      throw new Error('compile_main(macro gdef-global baseline): event[0] payload mismatch');
    }
    const baselineStatsText = new TextDecoder('utf-8', { fatal: true }).decode(baselineEvents[1].payload);
    if (/[ \t\r\n]/.test(baselineStatsText) || baselineStatsText.includes('\"unexpected_key\"')) {
      throw new Error(`compile_main(macro gdef-global baseline): invalid stats json text: ${baselineStatsText}`);
    }
    const baselineStats = JSON.parse(baselineStatsText);
    if (typeof baselineStats !== 'object' || baselineStats === null || typeof baselineStats.char_count !== 'number') {
      throw new Error('compile_main(macro gdef-global baseline): invalid stats json object');
    }
    gdefBaselineCharCount = baselineStats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro gdef-global baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro gdef-global case failed');
  }
  const macroGdefGlobalMainBytes = new TextEncoder().encode('{\\gdef\\foo{XYZ}}\\foo');
  if (addMountedFile('main.tex', macroGdefGlobalMainBytes, 'macro_gdef_global_main') !== 0) {
    throw new Error('mount_add_file(macro gdef-global main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro gdef-global case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro gdef-global)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro gdef-global) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const events = decodeEvents(readEventsBytes(), 'compile_main(macro gdef-global)');
    if (events.length !== 2 || events[0].kind !== 1 || events[1].kind !== 2) {
      throw new Error('compile_main(macro gdef-global): event shape mismatch');
    }
    if (
      events[0].payload.length !== logBytes.length
      || !events[0].payload.every((byte, index) => byte === logBytes[index])
    ) {
      throw new Error('compile_main(macro gdef-global): event[0] payload mismatch');
    }
    const statsText = new TextDecoder('utf-8', { fatal: true }).decode(events[1].payload);
    if (/[ \t\r\n]/.test(statsText) || statsText.includes('\"unexpected_key\"')) {
      throw new Error(`compile_main(macro gdef-global): invalid stats json text: ${statsText}`);
    }
    const stats = JSON.parse(statsText);
    if (typeof stats !== 'object' || stats === null || typeof stats.char_count !== 'number') {
      throw new Error('compile_main(macro gdef-global): invalid stats json object');
    }
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro gdef-global) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro gdef-global)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro global-def case failed');
  }
  const macroGlobalDefMainBytes = new TextEncoder().encode('{\\global\\def\\foo{XYZ}}\\foo');
  if (addMountedFile('main.tex', macroGlobalDefMainBytes, 'macro_global_def_main') !== 0) {
    throw new Error('mount_add_file(macro global-def main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro global-def case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro global-def)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro global-def) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro global-def)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro global-def) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro global-def)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro global-prefix invalid case failed');
  }
  const macroGlobalStackedDefMainBytes = new TextEncoder().encode('{\\global\\global\\def\\foo{XYZ}}\\foo');
  if (addMountedFile('main.tex', macroGlobalStackedDefMainBytes, 'macro_global_stacked_def_main') !== 0) {
    throw new Error('mount_add_file(macro stacked global-def main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro stacked global-def case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro stacked global-def)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro stacked global-def) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro stacked global-def)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro stacked global-def) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro stacked global-def)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro global-prefix invalid case failed');
  }
  const macroGlobalPrefixInvalidMainBytes = new TextEncoder().encode('\\global\\foo');
  if (addMountedFile('main.tex', macroGlobalPrefixInvalidMainBytes, 'macro_global_prefix_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro global-prefix invalid main.tex) failed');
  }
  const macroGlobalPrefixInvalidFinalizeCode = ctx.mountFinalize();
  if (macroGlobalPrefixInvalidFinalizeCode !== 0 && macroGlobalPrefixInvalidFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro global-prefix invalid) unexpected code=${macroGlobalPrefixInvalidFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro global-prefix invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_global_prefix_unsupported')) {
      throw new Error(`compile_main macro global-prefix invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro global-prefix invalid)');
  }

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
