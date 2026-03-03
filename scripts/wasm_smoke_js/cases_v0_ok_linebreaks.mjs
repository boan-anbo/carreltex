export function runOkLinebreakCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK control-symbol linebreak case failed');
  const linebreakDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\\\B\\end{document}',
  );
  if (addMountedFile('main.tex', linebreakDocBytes, 'ok_linebreak_control_symbol_main') !== 0) {
    throw new Error('mount_add_file(ok control-symbol linebreak main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK control-symbol linebreak case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok control-symbol linebreak)');
  const linebreakLogBytes = readCompileLogBytes();
  if (linebreakLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok control-symbol linebreak) expected empty log, got ${linebreakLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    linebreakLogBytes,
    { char_count: baselineStats.char_count + 2 },
    'compile_main(ok control-symbol linebreak)',
  );
  const linebreakXdvBytes = readMainXdvArtifactBytes('compile_main(ok control-symbol linebreak)');
  if (linebreakXdvBytes.length === 0) {
    throw new Error('compile_main(ok control-symbol linebreak) main.xdv expected non-empty bytes');
  }
  const linebreakMovement = countMovementOpsInTextPages(
    linebreakXdvBytes,
    'compile_main(ok control-symbol linebreak)',
  );
  if (linebreakMovement.down3 < 1) {
    throw new Error(`compile_main(ok control-symbol linebreak) expected down3>=1, got ${linebreakMovement.down3}`);
  }
  if (linebreakMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok control-symbol linebreak) expected right3PositiveTotal=131072, got ${linebreakMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK control-symbol linebreak star case failed');
  const linebreakStarDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\\\*B\\end{document}',
  );
  if (addMountedFile('main.tex', linebreakStarDocBytes, 'ok_linebreak_control_symbol_star_main') !== 0) {
    throw new Error('mount_add_file(ok control-symbol linebreak star main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK control-symbol linebreak star case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok control-symbol linebreak star)');
  const linebreakStarLogBytes = readCompileLogBytes();
  if (linebreakStarLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok control-symbol linebreak star) expected empty log, got ${linebreakStarLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    linebreakStarLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok control-symbol linebreak star)',
  );
  const linebreakStarXdvBytes = readMainXdvArtifactBytes('compile_main(ok control-symbol linebreak star)');
  if (linebreakStarXdvBytes.length === 0) {
    throw new Error('compile_main(ok control-symbol linebreak star) main.xdv expected non-empty bytes');
  }
  const linebreakStarMovement = countMovementOpsInTextPages(
    linebreakStarXdvBytes,
    'compile_main(ok control-symbol linebreak star)',
  );
  if (linebreakStarMovement.down3 < 1) {
    throw new Error(`compile_main(ok control-symbol linebreak star) expected down3>=1, got ${linebreakStarMovement.down3}`);
  }
  if (linebreakStarMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok control-symbol linebreak star) expected right3PositiveTotal=131072, got ${linebreakStarMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before control-symbol linebreak preamble invalid case failed');
  const preambleLinebreakDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\\\\\begin{document}AB\\end{document}',
  );
  if (addMountedFile('main.tex', preambleLinebreakDocBytes, 'ok_linebreak_control_symbol_preamble_main') !== 0) {
    throw new Error('mount_add_file(control-symbol linebreak preamble invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for control-symbol linebreak preamble invalid case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(control-symbol linebreak preamble invalid)');
}
