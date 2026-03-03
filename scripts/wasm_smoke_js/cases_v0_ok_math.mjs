export function runOkMathCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before inline math case failed');
  const inlineMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\(X\\)B\\end{document}',
  );
  if (addMountedFile('main.tex', inlineMathDocBytes, 'ok_inline_math_main') !== 0) {
    throw new Error('mount_add_file(ok inline math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for inline math case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok inline math)');
  const inlineLogBytes = readCompileLogBytes();
  if (inlineLogBytes.length !== 0) {
    throw new Error(`compile_main(ok inline math) expected empty log, got ${inlineLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    inlineLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok inline math)',
  );
  const inlineXdvBytes = readMainXdvArtifactBytes('compile_main(ok inline math)');
  if (inlineXdvBytes.length === 0) {
    throw new Error('compile_main(ok inline math) main.xdv expected non-empty bytes');
  }
  const inlineMovement = countMovementOpsInTextPages(inlineXdvBytes, 'compile_main(ok inline math)');
  if (inlineMovement.right3 !== 9) {
    throw new Error(`compile_main(ok inline math) expected right3=9, got ${inlineMovement.right3}`);
  }
  if (inlineMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok inline math) expected right3PositiveTotal=557056, got ${inlineMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before display math case failed');
  const displayMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\[X\\]B\\end{document}',
  );
  if (addMountedFile('main.tex', displayMathDocBytes, 'ok_display_math_main') !== 0) {
    throw new Error('mount_add_file(ok display math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for display math case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok display math)');
  const displayLogBytes = readCompileLogBytes();
  if (displayLogBytes.length !== 0) {
    throw new Error(`compile_main(ok display math) expected empty log, got ${displayLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    displayLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok display math)',
  );
  const displayXdvBytes = readMainXdvArtifactBytes('compile_main(ok display math)');
  if (displayXdvBytes.length === 0) {
    throw new Error('compile_main(ok display math) main.xdv expected non-empty bytes');
  }
  const displayMovement = countMovementOpsInTextPages(displayXdvBytes, 'compile_main(ok display math)');
  if (displayMovement.down3 < 2) {
    throw new Error(`compile_main(ok display math) expected down3>=2, got ${displayMovement.down3}`);
  }
  if (displayMovement.right3PositiveTotal !== 524288) {
    throw new Error(
      `compile_main(ok display math) expected right3PositiveTotal=524288, got ${displayMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before unclosed control inline math invalid case failed');
  const unclosedControlInlineMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\(X\\end{document}',
  );
  if (addMountedFile('main.tex', unclosedControlInlineMathDocBytes, 'ok_unclosed_control_inline_math_main') !== 0) {
    throw new Error('mount_add_file(ok unclosed control inline math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for unclosed control inline math invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok unclosed control inline math)');
}
