export function runOkMathFontWrapperCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK math-font wrapper case failed');
  const wrapperDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\mathbf{x}B\\end{document}',
  );
  if (addMountedFile('main.tex', wrapperDocBytes, 'ok_math_font_wrapper_main') !== 0) {
    throw new Error('mount_add_file(ok math-font wrapper main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK math-font wrapper case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok math-font wrapper)');
  const wrapperLogBytes = readCompileLogBytes();
  if (wrapperLogBytes.length !== 0) {
    throw new Error(`compile_main(ok math-font wrapper) expected empty log, got ${wrapperLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    wrapperLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok math-font wrapper)',
  );
  const wrapperXdvBytes = readMainXdvArtifactBytes('compile_main(ok math-font wrapper)');
  const wrapperMovement = countMovementOpsInTextPages(wrapperXdvBytes, 'compile_main(ok math-font wrapper)');
  if (wrapperMovement.right3 !== 9) {
    throw new Error(`compile_main(ok math-font wrapper) expected right3=9, got ${wrapperMovement.right3}`);
  }
  if (wrapperMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok math-font wrapper) expected right3PositiveTotal=557056, got ${wrapperMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before invalid math-font wrapper case failed');
  const invalidWrapperDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\mathbf xB\\end{document}',
  );
  if (addMountedFile('main.tex', invalidWrapperDocBytes, 'ok_math_font_wrapper_invalid_main') !== 0) {
    throw new Error('mount_add_file(ok math-font wrapper invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for invalid math-font wrapper case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok math-font wrapper invalid)');
}
