export function runOkTextMathCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK text-math positive case failed');
  const textMathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\text{x}B\\end{document}',
  );
  if (addMountedFile('main.tex', textMathDocBytes, 'ok_text_math_positive_main') !== 0) {
    throw new Error('mount_add_file(ok text-math positive main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK text-math positive case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok text-math positive)');
  const textMathLogBytes = readCompileLogBytes();
  if (textMathLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok text-math positive) expected empty log, got ${textMathLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    textMathLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok text-math positive)',
  );
  const textMathXdvBytes = readMainXdvArtifactBytes('compile_main(ok text-math positive)');
  const textMathMovement = countMovementOpsInTextPages(textMathXdvBytes, 'compile_main(ok text-math positive)');
  if (textMathMovement.right3 !== 9) {
    throw new Error(`compile_main(ok text-math positive) expected right3=9, got ${textMathMovement.right3}`);
  }
  if (textMathMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok text-math positive) expected right3PositiveTotal=557056, got ${textMathMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK text-math invalid case failed');
  const textMathInvalidDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\text xB\\end{document}',
  );
  if (addMountedFile('main.tex', textMathInvalidDocBytes, 'ok_text_math_invalid_main') !== 0) {
    throw new Error('mount_add_file(ok text-math invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK text-math invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok text-math invalid)');
}
