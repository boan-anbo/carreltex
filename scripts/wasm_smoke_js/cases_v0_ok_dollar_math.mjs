export function runOkDollarMathCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before dollar display math case failed');
  const dollarDisplayMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$$x$$B\\end{document}',
  );
  if (addMountedFile('main.tex', dollarDisplayMathDocBytes, 'ok_dollar_display_math_main') !== 0) {
    throw new Error('mount_add_file(ok dollar display math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for dollar display math case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok dollar display math)');
  const dollarDisplayMathLogBytes = readCompileLogBytes();
  if (dollarDisplayMathLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok dollar display math) expected empty log, got ${dollarDisplayMathLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    dollarDisplayMathLogBytes,
    { char_count: baselineStats.char_count + 7 },
    'compile_main(ok dollar display math)',
  );
  const dollarDisplayMathXdvBytes = readMainXdvArtifactBytes('compile_main(ok dollar display math)');
  if (dollarDisplayMathXdvBytes.length === 0) {
    throw new Error('compile_main(ok dollar display math) main.xdv expected non-empty bytes');
  }
  const dollarDisplayMathMovement = countMovementOpsInTextPages(
    dollarDisplayMathXdvBytes,
    'compile_main(ok dollar display math)',
  );
  if (dollarDisplayMathMovement.down3 < 2) {
    throw new Error(
      `compile_main(ok dollar display math) expected down3>=2, got ${dollarDisplayMathMovement.down3}`,
    );
  }
  if (dollarDisplayMathMovement.right3PositiveTotal !== 524288) {
    throw new Error(
      `compile_main(ok dollar display math) expected right3PositiveTotal=524288, got ${dollarDisplayMathMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before inline dollar math case failed');
  const inlineDollarMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$x$B\\end{document}',
  );
  if (addMountedFile('main.tex', inlineDollarMathDocBytes, 'ok_inline_dollar_math_main') !== 0) {
    throw new Error('mount_add_file(ok inline dollar math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for inline dollar math case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok inline dollar math)');
  const inlineDollarMathLogBytes = readCompileLogBytes();
  if (inlineDollarMathLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok inline dollar math) expected empty log, got ${inlineDollarMathLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    inlineDollarMathLogBytes,
    { char_count: baselineStats.char_count + 5 },
    'compile_main(ok inline dollar math)',
  );
  const inlineDollarMathXdvBytes = readMainXdvArtifactBytes('compile_main(ok inline dollar math)');
  if (inlineDollarMathXdvBytes.length === 0) {
    throw new Error('compile_main(ok inline dollar math) main.xdv expected non-empty bytes');
  }
  const inlineDollarMathMovement = countMovementOpsInTextPages(
    inlineDollarMathXdvBytes,
    'compile_main(ok inline dollar math)',
  );
  if (inlineDollarMathMovement.right3 !== 9) {
    throw new Error(`compile_main(ok inline dollar math) expected right3=9, got ${inlineDollarMathMovement.right3}`);
  }
  if (inlineDollarMathMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok inline dollar math) expected right3PositiveTotal=557056, got ${inlineDollarMathMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before inline numeric dollar math case failed');
  const inlineNumericDollarMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$1$B\\end{document}',
  );
  if (addMountedFile('main.tex', inlineNumericDollarMathDocBytes, 'ok_inline_numeric_dollar_math_main') !== 0) {
    throw new Error('mount_add_file(ok inline numeric dollar math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for inline numeric dollar math case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok inline numeric dollar math)');
  const inlineNumericDollarMathLogBytes = readCompileLogBytes();
  if (inlineNumericDollarMathLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok inline numeric dollar math) expected empty log, got ${inlineNumericDollarMathLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    inlineNumericDollarMathLogBytes,
    { char_count: baselineStats.char_count + 5 },
    'compile_main(ok inline numeric dollar math)',
  );
  const inlineNumericDollarMathXdvBytes = readMainXdvArtifactBytes('compile_main(ok inline numeric dollar math)');
  if (inlineNumericDollarMathXdvBytes.length === 0) {
    throw new Error('compile_main(ok inline numeric dollar math) main.xdv expected non-empty bytes');
  }
  const inlineNumericDollarMathMovement = countMovementOpsInTextPages(
    inlineNumericDollarMathXdvBytes,
    'compile_main(ok inline numeric dollar math)',
  );
  if (inlineNumericDollarMathMovement.right3 !== 9) {
    throw new Error(
      `compile_main(ok inline numeric dollar math) expected right3=9, got ${inlineNumericDollarMathMovement.right3}`,
    );
  }
  if (inlineNumericDollarMathMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok inline numeric dollar math) expected right3PositiveTotal=557056, got ${inlineNumericDollarMathMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before unclosed dollar display math invalid case failed');
  const unclosedDollarDisplayMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$$xB\\end{document}',
  );
  if (addMountedFile('main.tex', unclosedDollarDisplayMathDocBytes, 'ok_unclosed_dollar_display_math_main') !== 0) {
    throw new Error('mount_add_file(ok unclosed dollar display math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for unclosed dollar display math invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok unclosed dollar display math)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before unclosed inline dollar math invalid case failed');
  const unclosedInlineDollarMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$xB\\end{document}',
  );
  if (addMountedFile('main.tex', unclosedInlineDollarMathDocBytes, 'ok_unclosed_inline_dollar_math_main') !== 0) {
    throw new Error('mount_add_file(ok unclosed inline dollar math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for unclosed inline dollar math invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok unclosed inline dollar math)');
}
