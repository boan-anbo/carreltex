export function runIfnumCases(ctx, helpers) {
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
    throw new Error('mount_reset before ifnum baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_ifnum_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro ifnum baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifnum baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifnum baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifnum baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro ifnum baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifnum true case failed');
  }
  const trueMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\count0=1\\count1=2\\ifnum\\count0<\\count1 XYZ\\fi\n\\end{document}\n');
  if (addMountedFile('main.tex', trueMainBytes, 'macro_ifnum_true_main') !== 0) {
    throw new Error('mount_add_file(macro ifnum true main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifnum true case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifnum true)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifnum true)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for ifnum true case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro ifnum true) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro ifnum true)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifnum false case failed');
  }
  const falseMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\count0=2\\count1=1\\ifnum\\count0<\\count1 AAA\\else XYZ\\fi\n\\end{document}\n');
  if (addMountedFile('main.tex', falseMainBytes, 'macro_ifnum_false_main') !== 0) {
    throw new Error('mount_add_file(macro ifnum false main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifnum false case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifnum false)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifnum false)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for ifnum false case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro ifnum false) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro ifnum false)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifnum input-counts case failed');
  }
  const inputCountsMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\ifnum\\count0<\\count1 XYZ\\else AAA\\fi\n\\end{document}\n');
  const inputCountsSubBytes = new TextEncoder().encode('\\count0=1\\count1=2');
  if (addMountedFile('main.tex', inputCountsMainBytes, 'macro_ifnum_input_counts_main') !== 0) {
    throw new Error('mount_add_file(macro ifnum input-counts main.tex) failed');
  }
  if (addMountedFile('sub.tex', inputCountsSubBytes, 'macro_ifnum_input_counts_sub') !== 0) {
    throw new Error('mount_add_file(macro ifnum input-counts sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for ifnum input-counts case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro ifnum input-counts)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro ifnum input-counts)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for ifnum input-counts case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro ifnum input-counts) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro ifnum input-counts)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before ifnum else invalid case failed');
  }
  const invalidMainBytes = new TextEncoder().encode('\\ifnum\\count0<\\count1 X\\else Y\\else Z\\fi');
  if (addMountedFile('main.tex', invalidMainBytes, 'macro_ifnum_else_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro ifnum else invalid main.tex) failed');
  }
  const finalizeCode = ctx.mountFinalize();
  if (finalizeCode !== 0 && finalizeCode !== 1) {
    throw new Error(`mount_finalize(macro ifnum else invalid) unexpected code=${finalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro ifnum else invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_if_else_duplicate')) {
      throw new Error(`compile_main macro ifnum else invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro ifnum else invalid)');
  }
}
