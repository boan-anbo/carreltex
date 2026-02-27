export function runXdefNoexpandCases(ctx, helpers) {
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
    throw new Error('mount_reset before xdef/noexpand baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'macro_xdef_noexpand_baseline_main') !== 0) {
    throw new Error('mount_add_file(macro xdef/noexpand baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for xdef/noexpand baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro xdef/noexpand baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro xdef/noexpand baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(macro xdef/noexpand baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before xdef positive case failed');
  }
  const xdefMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n{\\def\\bar{XYZ}\\xdef\\foo{\\bar}}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', xdefMainBytes, 'macro_xdef_positive_main') !== 0) {
    throw new Error('mount_add_file(macro xdef positive main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for xdef positive case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro xdef positive)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro xdef positive)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for xdef positive case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro xdef positive) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro xdef positive)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before noexpand dynamic case failed');
  }
  const xdefInputSnapshotMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}{\\xdef\\foo{\\bar}}\\def\\bar{A}\\foo\n\\end{document}\n');
  const xdefInputSnapshotSubBytes = new TextEncoder().encode('\\def\\bar{XYZ}');
  if (addMountedFile('main.tex', xdefInputSnapshotMainBytes, 'macro_xdef_input_snapshot_main') !== 0) {
    throw new Error('mount_add_file(macro xdef input snapshot main.tex) failed');
  }
  if (addMountedFile('sub.tex', xdefInputSnapshotSubBytes, 'macro_xdef_input_snapshot_sub') !== 0) {
    throw new Error('mount_add_file(macro xdef input snapshot sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for xdef input snapshot case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro xdef input snapshot)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro xdef input snapshot)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for xdef input snapshot case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro xdef input snapshot) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro xdef input snapshot)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before noexpand dynamic case failed');
  }
  const noexpandMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\def\\bar{X}\\edef\\foo{\\noexpand\\bar}\\def\\bar{XYZ}\\foo\n\\end{document}\n');
  if (addMountedFile('main.tex', noexpandMainBytes, 'macro_noexpand_dynamic_main') !== 0) {
    throw new Error('mount_add_file(macro noexpand dynamic main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for noexpand dynamic case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro noexpand dynamic)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro noexpand dynamic)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for noexpand dynamic case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(macro noexpand dynamic) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro noexpand dynamic)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before noexpand input snapshot case failed');
  }
  const noexpandInputSnapshotMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\edef\\foo{\\noexpand\\bar}\\def\\bar{A}\\foo\n\\end{document}\n');
  const noexpandInputSnapshotSubBytes = new TextEncoder().encode('\\def\\bar{XYZ}');
  if (addMountedFile('main.tex', noexpandInputSnapshotMainBytes, 'macro_noexpand_input_snapshot_main') !== 0) {
    throw new Error('mount_add_file(macro noexpand input snapshot main.tex) failed');
  }
  if (addMountedFile('sub.tex', noexpandInputSnapshotSubBytes, 'macro_noexpand_input_snapshot_sub') !== 0) {
    throw new Error('mount_add_file(macro noexpand input snapshot sub.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for noexpand input snapshot case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(macro noexpand input snapshot)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(macro noexpand input snapshot)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for noexpand input snapshot case');
    }
    if (stats.char_count !== baselineCharCount + 1) {
      throw new Error(`compile_main(macro noexpand input snapshot) char_count delta expected +1, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(macro noexpand input snapshot)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before noexpand invalid case failed');
  }
  const noexpandInvalidMainBytes = new TextEncoder().encode('\\edef\\foo{\\noexpand}');
  if (addMountedFile('main.tex', noexpandInvalidMainBytes, 'macro_noexpand_invalid_main') !== 0) {
    throw new Error('mount_add_file(macro noexpand invalid main.tex) failed');
  }
  const invalidFinalizeCode = ctx.mountFinalize();
  if (invalidFinalizeCode !== 0 && invalidFinalizeCode !== 1) {
    throw new Error(`mount_finalize(macro noexpand invalid) unexpected code=${invalidFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro noexpand invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_noexpand_unsupported')) {
      throw new Error(`compile_main macro noexpand invalid log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro noexpand invalid)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before macro begingroup-depth-exceeded case failed');
  }
  const macroBegingroupDepthExceededMainBytes = new TextEncoder().encode(`${'\\begingroup'.repeat(1025)}X`);
  if (addMountedFile('main.tex', macroBegingroupDepthExceededMainBytes, 'macro_begingroup_depth_exceeded_main') !== 0) {
    throw new Error('mount_add_file(macro begingroup-depth-exceeded main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for macro begingroup-depth-exceeded case failed');
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(macro begingroup-depth-exceeded)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('macro_group_depth_exceeded')) {
      throw new Error(`compile_main macro begingroup-depth-exceeded log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(macro begingroup-depth-exceeded)');
  }
}
