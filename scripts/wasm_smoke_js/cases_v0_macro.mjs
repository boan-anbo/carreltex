export function runMacroCases(ctx, helpers, baselineMainCharCount) {
  const {
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
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro expansion positive case failed');
  }
  const inputMacroMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\input{sub.tex}\\foo\n\\end{document}\n');
  const inputMacroSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', inputMacroMainBytes, 'input_macro_main') !== 0) {
    throw new Error('mount_add_file(input-macro main.tex) failed');
  }
  if (addMountedFile('sub.tex', inputMacroSubBytes, 'input_macro_sub') !== 0) {
    throw new Error('mount_add_file(input-macro sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for input-macro interplay case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(input->macro interplay)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(input->macro interplay) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(input->macro interplay)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(input->macro interplay) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(input->macro interplay)');
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
  let gdefBaselineCharCount = null;
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro expansion) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro expansion positive)');
    if (baselineMainCharCount === null) {
      throw new Error('baselineMainCharCount not initialized');
    }
    if (stats.char_count !== baselineMainCharCount + 3) {
      throw new Error(`compile_main(macro expansion) char_count delta expected +3, got baseline=${baselineMainCharCount}, current=${stats.char_count}`);
    }
    gdefBaselineCharCount = baselineMainCharCount;
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
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro single-param positive)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro single-param) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
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
    throw new Error('mount_reset before macro global-gdef case failed');
  }
  const macroGlobalGdefMainBytes = new TextEncoder().encode('{\\global\\gdef\\foo{XYZ}}\\foo');
  if (addMountedFile('main.tex', macroGlobalGdefMainBytes, 'macro_global_gdef_main') !== 0) {
    throw new Error('mount_add_file(macro global-gdef main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro global-gdef case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro global-gdef)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro global-gdef) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro global-gdef)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro global-gdef) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro global-gdef)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro stacked global-def case failed');
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
  const macroLetAliasMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\let\\bar=\\foo\\bar');
  if (addMountedFile('main.tex', macroLetAliasMainBytes, 'macro_let_alias_main') !== 0) {
    throw new Error('mount_add_file(macro let-alias main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro let-alias case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro let-alias)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro let-alias) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro let-alias)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro let-alias) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro let-alias)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro futurelet case failed');
  }
  const macroInputLetSnapshotMainBytes = new TextEncoder().encode('\\input{sub.tex}\\let\\bar=\\foo\\def\\foo{A}\\bar');
  const macroInputLetSnapshotSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', macroInputLetSnapshotMainBytes, 'macro_input_let_snapshot_main') !== 0) {
    throw new Error('mount_add_file(macro input let-snapshot main.tex) failed');
  }
  if (addMountedFile('sub.tex', macroInputLetSnapshotSubBytes, 'macro_input_let_snapshot_sub') !== 0) {
    throw new Error('mount_add_file(macro input let-snapshot sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro input let-snapshot case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro input let-snapshot)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro input let-snapshot) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro input let-snapshot)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro input let-snapshot) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro input let-snapshot)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro futurelet case failed');
  }
  const macroInputFutureletMainBytes = new TextEncoder().encode('\\input{sub.tex}\\futurelet\\bar\\noop\\foo\\bar');
  const macroInputFutureletSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', macroInputFutureletMainBytes, 'macro_input_futurelet_main') !== 0) {
    throw new Error('mount_add_file(macro input futurelet main.tex) failed');
  }
  if (addMountedFile('sub.tex', macroInputFutureletSubBytes, 'macro_input_futurelet_sub') !== 0) {
    throw new Error('mount_add_file(macro input futurelet sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro input futurelet case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro input futurelet)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro input futurelet) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro input futurelet)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro input futurelet) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro input futurelet)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro futurelet case failed');
  }
  const macroFutureletMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\futurelet\\bar\\noop\\foo\\bar');
  if (addMountedFile('main.tex', macroFutureletMainBytes, 'macro_futurelet_main') !== 0) {
    throw new Error('mount_add_file(macro futurelet main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro futurelet case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro futurelet)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro futurelet) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro futurelet)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro futurelet) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro futurelet)');
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
    throw new Error('mount_reset before macro let unsupported case failed');
  }
  const macroLetUnsupportedMainBytes = new TextEncoder().encode('\\let\\a=Z');
  if (addMountedFile('main.tex', macroLetUnsupportedMainBytes, 'macro_let_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro let unsupported main.tex) failed');
  }
  const macroLetUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroLetUnsupportedFinalizeCode !== 0 && macroLetUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro let unsupported) unexpected code=${macroLetUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro let unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_let_unsupported')) {
      throw new Error(`compile_main macro let unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro let unsupported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro futurelet unsupported case failed');
  }
  const macroFutureletUnsupportedMainBytes = new TextEncoder().encode('\\futurelet\\a Z \\b');
  if (addMountedFile('main.tex', macroFutureletUnsupportedMainBytes, 'macro_futurelet_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro futurelet unsupported main.tex) failed');
  }
  const macroFutureletUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroFutureletUnsupportedFinalizeCode !== 0 && macroFutureletUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro futurelet unsupported) unexpected code=${macroFutureletUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro futurelet unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_futurelet_unsupported')) {
      throw new Error(`compile_main macro futurelet unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro futurelet unsupported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  const macroExpandafterMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\def\\bar{A}\\expandafter\\bar\\foo');
  if (addMountedFile('main.tex', macroExpandafterMainBytes, 'macro_expandafter_main') !== 0) {
    throw new Error('mount_add_file(macro expandafter main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro expandafter case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro expandafter)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro expandafter) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro expandafter)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 4) {
      throw new Error(`compile_main(macro expandafter) char_count delta expected +4, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro expandafter)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro input expandafter case failed');
  }
  const macroInputExpandafterMainBytes = new TextEncoder().encode('\\input{sub.tex}\\expandafter\\bar\\foo');
  const macroInputExpandafterSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\def\\bar{A}');
  if (addMountedFile('main.tex', macroInputExpandafterMainBytes, 'macro_input_expandafter_main') !== 0) {
    throw new Error('mount_add_file(macro input expandafter main.tex) failed');
  }
  if (addMountedFile('sub.tex', macroInputExpandafterSubBytes, 'macro_input_expandafter_sub') !== 0) {
    throw new Error('mount_add_file(macro input expandafter sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro input expandafter case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro input expandafter)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro input expandafter) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro input expandafter)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 4) {
      throw new Error(`compile_main(macro input expandafter) char_count delta expected +4, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro input expandafter)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro expandafter unsupported case failed');
  }
  const macroExpandafterUnsupportedMainBytes = new TextEncoder().encode('\\expandafter{}');
  if (addMountedFile('main.tex', macroExpandafterUnsupportedMainBytes, 'macro_expandafter_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro expandafter unsupported main.tex) failed');
  }
  const macroExpandafterUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroExpandafterUnsupportedFinalizeCode !== 0 && macroExpandafterUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro expandafter unsupported) unexpected code=${macroExpandafterUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro expandafter unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_expandafter_unsupported')) {
      throw new Error(`compile_main macro expandafter unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro expandafter unsupported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  const macroInputCsnameMainBytes = new TextEncoder().encode('\\input{sub.tex}\\csname foo\\endcsname');
  const macroInputCsnameSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', macroInputCsnameMainBytes, 'macro_input_csname_main') !== 0) {
    throw new Error('mount_add_file(macro input csname main.tex) failed');
  }
  if (addMountedFile('sub.tex', macroInputCsnameSubBytes, 'macro_input_csname_sub') !== 0) {
    throw new Error('mount_add_file(macro input csname sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro input csname case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro input csname)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro input csname) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro input csname)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro input csname) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro input csname)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  const macroCsnameMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\csname foo\\endcsname');
  if (addMountedFile('main.tex', macroCsnameMainBytes, 'macro_csname_main') !== 0) {
    throw new Error('mount_add_file(macro csname main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro csname case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro csname)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro csname) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro csname)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 3) {
      throw new Error(`compile_main(macro csname) char_count delta expected +3, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro csname)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro csname unsupported case failed');
  }
  const macroCsnameUnsupportedMainBytes = new TextEncoder().encode('\\csname\\foo\\endcsname');
  if (addMountedFile('main.tex', macroCsnameUnsupportedMainBytes, 'macro_csname_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro csname unsupported main.tex) failed');
  }
  const macroCsnameUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroCsnameUnsupportedFinalizeCode !== 0 && macroCsnameUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro csname unsupported) unexpected code=${macroCsnameUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro csname unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_csname_unsupported')) {
      throw new Error(`compile_main macro csname unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro csname unsupported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  const macroInputStringMainBytes = new TextEncoder().encode('\\input{sub.tex}\\string\\foo');
  const macroInputStringSubBytes = new TextEncoder().encode('\\def\\foo{XYZ}');
  if (addMountedFile('main.tex', macroInputStringMainBytes, 'macro_input_string_main') !== 0) {
    throw new Error('mount_add_file(macro input string main.tex) failed');
  }
  if (addMountedFile('sub.tex', macroInputStringSubBytes, 'macro_input_string_sub') !== 0) {
    throw new Error('mount_add_file(macro input string sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro input string case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro input string)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro input string) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro input string)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 4) {
      throw new Error(`compile_main(macro input string) char_count delta expected +4, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro input string)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before compile_request negative setter tests failed');
  }
  const macroStringMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\string\\foo');
  if (addMountedFile('main.tex', macroStringMainBytes, 'macro_string_main') !== 0) {
    throw new Error('mount_add_file(macro string main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro string case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro string)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro string) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro string)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 4) {
      throw new Error(`compile_main(macro string) char_count delta expected +4, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro string)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro string unsupported case failed');
  }
  const macroStringUnsupportedMainBytes = new TextEncoder().encode('\\string{}');
  if (addMountedFile('main.tex', macroStringUnsupportedMainBytes, 'macro_string_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro string unsupported main.tex) failed');
  }
  const macroStringUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroStringUnsupportedFinalizeCode !== 0 && macroStringUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro string unsupported) unexpected code=${macroStringUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro string unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_string_unsupported')) {
      throw new Error(`compile_main macro string unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro string unsupported)');
  }

  return {
    gdefBaselineCharCount,
  };
}
