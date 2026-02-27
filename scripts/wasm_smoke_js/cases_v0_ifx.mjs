export function runIfxCases(ctx, helpers) {
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
    throw new Error('mount_reset before ifx baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_ifx_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro ifx baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifx baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifx baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifx baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro ifx baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifx alias==alias case failed');
  }
  const aliasEqualMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\let\\a=\\foo\\let\\b=\\foo\\ifx\\a\\b XYZ\\else AAA\\fi\n\\end{document}\n');
  if (addMountedFile('main.tex', aliasEqualMainBytes, 'macro_ifx_alias_equal_main') !== 0) {
    throw new Error('mount_add_file(macro ifx alias==alias main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifx alias==alias case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifx alias==alias)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifx alias==alias)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for ifx alias==alias case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro ifx alias==alias) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro ifx alias==alias)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifx macro!=macro case failed');
  }
  const notEqualMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\def\\a{X}\\def\\b{XYZ}\\ifx\\a\\b AAA\\else XYZ\\fi\n\\end{document}\n');
  if (addMountedFile('main.tex', notEqualMainBytes, 'macro_ifx_not_equal_main') !== 0) {
    throw new Error('mount_add_file(macro ifx macro!=macro main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifx macro!=macro case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifx macro!=macro)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifx macro!=macro)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for ifx macro!=macro case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro ifx macro!=macro) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro ifx macro!=macro)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifx duplicate else invalid case failed');
  }
  const invalidMainBytes = new TextEncoder().encode('\\ifx\\foo\\bar X\\else Y\\else Z\\fi');
  if (addMountedFile('main.tex', invalidMainBytes, 'macro_ifx_else_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro ifx duplicate else invalid main.tex) failed');
  }
  const finalizeCode = ctx.mountFinalize();
  if (finalizeCode !== 0 && finalizeCode !== 1) {
    throw new Error(`mount_finalize(macro ifx duplicate else invalid) unexpected code=${finalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro ifx duplicate else invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_ifx_else_duplicate')) {
      throw new Error(`compile_main macro ifx duplicate else invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro ifx duplicate else invalid)');
  }
}
