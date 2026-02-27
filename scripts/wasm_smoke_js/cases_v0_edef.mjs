export function runEdefCases(ctx, helpers) {
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
    throw new Error('mount_reset before edef baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_edef_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro edef baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for edef baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro edef baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro edef baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro edef baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before edef positive case failed');
  }
  const edefPositiveMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\def\\bar{XYZ}\\edef\\foo{\\bar}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', edefPositiveMainBytes, 'macro_edef_positive_main') !== 0) {
    throw new Error('mount_add_file(macro edef positive main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for edef positive case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro edef positive)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro edef positive)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for edef positive case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro edef positive) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro edef positive)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before edef snapshot case failed');
  }
  const edefSnapshotMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\def\\bar{X}\\edef\\foo{\\bar}\\def\\bar{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', edefSnapshotMainBytes, 'macro_edef_snapshot_main') !== 0) {
    throw new Error('mount_add_file(macro edef snapshot main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for edef snapshot case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro edef snapshot)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro edef snapshot)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for edef snapshot case');
    }
    if (stats.char_count !== baselineCharCount + 1) {
      throw new Error(`compile_main(macro edef snapshot) char_count delta expected +1, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro edef snapshot)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before edef invalid case failed');
  }
  const edefInvalidMainBytes = new TextEncoder().encode('\\edef\\foo#1{#1}');
  if (addMountedFile('main.tex', edefInvalidMainBytes, 'macro_edef_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro edef invalid main.tex) failed');
  }
  const invalidFinalizeCode = ctx.mountFinalize();
  if (invalidFinalizeCode !== 0 && invalidFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro edef invalid) unexpected code=${invalidFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro edef invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_params_unsupported')) {
      throw new Error(`compile_main macro edef invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro edef invalid)');
  }
}
