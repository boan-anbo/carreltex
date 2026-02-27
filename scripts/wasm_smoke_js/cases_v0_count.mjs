export function runCountCases(ctx, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before count baseline case failed');
  }
  const countBaselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', countBaselineMainBytes, 'macro_count_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro count baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for count baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro count baseline)');
  let countBaselineCharCount = null;
  {
    const baselineLogBytes = readCompileLogBytes();
    const baselineStats = assertEventsMatchLogAndStats(baselineLogBytes, {}, 'compile_main(macro count baseline)');
    countBaselineCharCount = baselineStats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro count baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before count0 assignment + the case failed');
  }
  const count0MainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\count0=12\\the\\count0\n\\end{document}\n');
  if (addMountedFile('main.tex', count0MainBytes, 'macro_count0_main') !== 0) {
    throw new Error('mount_add_file(macro count0 main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro count0 case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro count0=12 + the)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro count0=12 + the)');
    if (countBaselineCharCount === null) {
      throw new Error('countBaselineCharCount not initialized');
    }
    if (stats.char_count !== countBaselineCharCount + 2) {
      throw new Error(`compile_main(macro count0=12 + the) char_count delta expected +2, got baseline=${countBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro count0=12 + the)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before the count1 default case failed');
  }
  const count1DefaultMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\the\\count1\n\\end{document}\n');
  if (addMountedFile('main.tex', count1DefaultMainBytes, 'macro_count1_default_main') !== 0) {
    throw new Error('mount_add_file(macro count1 default main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro count1 default case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro the count1 default)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro the count1 default)');
    if (countBaselineCharCount === null) {
      throw new Error('countBaselineCharCount not initialized');
    }
    if (stats.char_count !== countBaselineCharCount + 1) {
      throw new Error(`compile_main(macro the count1 default) char_count delta expected +1, got baseline=${countBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro the count1 default)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before count assignment invalid case failed');
  }
  const countInvalidMainBytes = new TextEncoder().encode('\\count0=-1');
  if (addMountedFile('main.tex', countInvalidMainBytes, 'macro_count_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro count invalid main.tex) failed');
  }
  const countInvalidFinalizeCode = ctx.mountFinalize();
  if (countInvalidFinalizeCode !== 0 && countInvalidFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro count invalid) unexpected code=${countInvalidFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro count assignment invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_count_assignment_unsupported')) {
      throw new Error(`compile_main macro count invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro count assignment invalid)');
  }
}
