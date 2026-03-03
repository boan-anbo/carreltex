export function runOkEnsuremathCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK ensuremath positive case failed');
  const ensuremathDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\ensuremath{x}B\\end{document}',
  );
  if (addMountedFile('main.tex', ensuremathDocBytes, 'ok_ensuremath_positive_main') !== 0) {
    throw new Error('mount_add_file(ok ensuremath positive main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK ensuremath positive case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok ensuremath positive)');
  const ensuremathLogBytes = readCompileLogBytes();
  if (ensuremathLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok ensuremath positive) expected empty log, got ${ensuremathLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    ensuremathLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok ensuremath positive)',
  );
  const ensuremathXdvBytes = readMainXdvArtifactBytes('compile_main(ok ensuremath positive)');
  const ensuremathMovement = countMovementOpsInTextPages(ensuremathXdvBytes, 'compile_main(ok ensuremath positive)');
  if (ensuremathMovement.right3 !== 9) {
    throw new Error(`compile_main(ok ensuremath positive) expected right3=9, got ${ensuremathMovement.right3}`);
  }
  if (ensuremathMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok ensuremath positive) expected right3PositiveTotal=557056, got ${ensuremathMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK ensuremath invalid case failed');
  const ensuremathInvalidDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\ensuremath xB\\end{document}',
  );
  if (addMountedFile('main.tex', ensuremathInvalidDocBytes, 'ok_ensuremath_invalid_main') !== 0) {
    throw new Error('mount_add_file(ok ensuremath invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK ensuremath invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok ensuremath invalid)');
}
