export function runNewcommandCases(ctx, helpers) {
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
    throw new Error('mount_reset before newcommand baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_newcommand_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro newcommand baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for newcommand baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro newcommand baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro newcommand baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro newcommand baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before newcommand zero-param case failed');
  }
  const newcommandMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\newcommand{\\foo}{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', newcommandMainBytes, 'macro_newcommand_main') !== 0) {
    throw new Error('mount_add_file(macro newcommand main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for newcommand zero-param case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro newcommand zero-param)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro newcommand zero-param)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for newcommand zero-param');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro newcommand zero-param) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro newcommand zero-param)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before newcommand single-param case failed');
  }
  const newcommandParamMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\newcommand{\\foo}[1]{#1}\\foo{A}\n\\end{document}\n');
  if (addMountedFile('main.tex', newcommandParamMainBytes, 'macro_newcommand_param_main') !== 0) {
    throw new Error('mount_add_file(macro newcommand single-param main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for newcommand single-param case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro newcommand single-param)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro newcommand single-param)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for newcommand single-param');
    }
    if (stats.char_count !== baselineCharCount + 1) {
      throw new Error(`compile_main(macro newcommand single-param) char_count delta expected +1, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro newcommand single-param)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before newcommand already-defined invalid case failed');
  }
  const newcommandAlreadyDefinedBytes = new TextEncoder().encode('\\newcommand{\\foo}{A}\\newcommand{\\foo}{B}');
  if (addMountedFile('main.tex', newcommandAlreadyDefinedBytes, 'macro_newcommand_already_defined_main') !== 0) {
    throw new Error('mount_add_file(macro newcommand already-defined main.tex) failed');
  }
  const alreadyDefinedFinalizeCode = ctx.mountFinalize();
  if (alreadyDefinedFinalizeCode !== 0 && alreadyDefinedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro newcommand already-defined) unexpected code=${alreadyDefinedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro newcommand already-defined invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_newcommand_already_defined')) {
      throw new Error(`compile_main macro newcommand already-defined invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro newcommand already-defined invalid)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before renewcommand undefined invalid case failed');
  }
  const renewcommandUndefinedBytes = new TextEncoder().encode('\\renewcommand{\\foo}{XYZ}\\foo');
  if (addMountedFile('main.tex', renewcommandUndefinedBytes, 'macro_renewcommand_undefined_main') !== 0) {
    throw new Error('mount_add_file(macro renewcommand undefined main.tex) failed');
  }
  const undefinedFinalizeCode = ctx.mountFinalize();
  if (undefinedFinalizeCode !== 0 && undefinedFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro renewcommand undefined) unexpected code=${undefinedFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro renewcommand undefined invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_renewcommand_undefined')) {
      throw new Error(`compile_main macro renewcommand undefined invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro renewcommand undefined invalid)');
  }
}
