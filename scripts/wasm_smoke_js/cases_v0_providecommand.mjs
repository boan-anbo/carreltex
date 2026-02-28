export function runProvidecommandCases(ctx, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before providecommand baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_providecommand_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro providecommand baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for providecommand baseline case failed');
  }
  expectOk(ctx.compileMain(), 'compile_main_v0(macro providecommand baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro providecommand baseline)');
    baselineCharCount = stats.char_count;
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before providecommand define case failed');
  }
  const defineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\providecommand{\\foo}{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', defineMainBytes, 'macro_providecommand_define_main') !== 0) {
    throw new Error('mount_add_file(macro providecommand define main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for providecommand define case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro providecommand define)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro providecommand define)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for providecommand define');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro providecommand define) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro providecommand define)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before providecommand no-op case failed');
  }
  const noopMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\newcommand{\\foo}{A}\\providecommand{\\foo}{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', noopMainBytes, 'macro_providecommand_noop_main') !== 0) {
    throw new Error('mount_add_file(macro providecommand no-op main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for providecommand no-op case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro providecommand no-op)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro providecommand no-op)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for providecommand no-op');
    }
    if (stats.char_count !== baselineCharCount + 1) {
      throw new Error(`compile_main(macro providecommand no-op) char_count delta expected +1, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro providecommand no-op)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before providecommand single-param case failed');
  }
  const singleParamMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\providecommand{\\foo}[1]{#1}\\foo{A}\n\\end{document}\n');
  if (addMountedFile('main.tex', singleParamMainBytes, 'macro_providecommand_single_param_main') !== 0) {
    throw new Error('mount_add_file(macro providecommand single-param main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for providecommand single-param case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro providecommand single-param)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro providecommand single-param)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for providecommand single-param');
    }
    if (stats.char_count !== baselineCharCount + 1) {
      throw new Error(`compile_main(macro providecommand single-param) char_count delta expected +1, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro providecommand single-param)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before providecommand invalid case failed');
  }
  const invalidMainBytes = new TextEncoder().encode('\\providecommand\\foo{XYZ}');
  if (addMountedFile('main.tex', invalidMainBytes, 'macro_providecommand_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro providecommand invalid main.tex) failed');
  }
  const invalidFinalizeCode = ctx.mountFinalize();
  if (invalidFinalizeCode !== 0 && invalidFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro providecommand invalid) unexpected code=${invalidFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro providecommand invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_providecommand_unsupported')) {
      throw new Error(`compile_main macro providecommand invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro providecommand invalid)');
  }
}
