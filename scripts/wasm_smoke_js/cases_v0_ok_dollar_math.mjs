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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before unclosed dollar display math invalid case failed');
  const unclosedDollarDisplayMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A$$xB\\end{document}',
  );
  if (addMountedFile('main.tex', unclosedDollarDisplayMathDocBytes, 'ok_unclosed_dollar_display_math_main') !== 0) {
    throw new Error('mount_add_file(ok unclosed dollar display math main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for unclosed dollar display math invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok unclosed dollar display math)');
}
