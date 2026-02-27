export function runMeaningCases(ctx, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectNotImplemented,
    readCompileReportJson,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
    gdefBaselineCharCount,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro meaning case failed');
  }
  const macroMeaningMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\meaning\\foo');
  if (addMountedFile('main.tex', macroMeaningMainBytes, 'macro_meaning_main') !== 0) {
    throw new Error('mount_add_file(macro meaning main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro meaning case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro meaning)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro meaning) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro meaning)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 9) {
      throw new Error(`compile_main(macro meaning) char_count delta expected +9, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro meaning)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro meaning alias case failed');
  }
  const macroMeaningAliasMainBytes = new TextEncoder().encode('\\def\\foo{XYZ}\\let\\bar=\\foo\\meaning\\bar');
  if (addMountedFile('main.tex', macroMeaningAliasMainBytes, 'macro_meaning_alias_main') !== 0) {
    throw new Error('mount_add_file(macro meaning alias main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro meaning alias case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro meaning alias)');
  {
    const report = readCompileReportJson();
    if (report.status !== 'NOT_IMPLEMENTED') {
      throw new Error(`compile_main(macro meaning alias) report.status expected NOT_IMPLEMENTED, got ${report.status}`);
    }
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro meaning alias)');
    if (gdefBaselineCharCount === null) {
      throw new Error('gdefBaselineCharCount not initialized');
    }
    if (stats.char_count !== gdefBaselineCharCount + 14) {
      throw new Error(`compile_main(macro meaning alias) char_count delta expected +14, got baseline=${gdefBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro meaning alias)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro meaning unsupported case failed');
  }
  const macroMeaningUnsupportedMainBytes = new TextEncoder().encode('\\meaning{}');
  if (addMountedFile('main.tex', macroMeaningUnsupportedMainBytes, 'macro_meaning_unsupported_main') !== 0) {
    throw new Error('mount_add_file(macro meaning unsupported main.tex) failed');
  }
  const macroMeaningUnsupportedFinalizeCode = ctx.mountFinalize();
  if (macroMeaningUnsupportedFinalizeCode !== 0 && macroMeaningUnsupportedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro meaning unsupported) unexpected code=${macroMeaningUnsupportedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro meaning unsupported)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_meaning_unsupported')) {
      throw new Error(`compile_main macro meaning unsupported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro meaning unsupported)');
  }
}
