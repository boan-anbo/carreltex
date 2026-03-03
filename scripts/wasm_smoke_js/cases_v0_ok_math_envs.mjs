export function runOkMathEnvCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation env case failed');
  const equationDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationDocBytes, 'ok_math_env_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation env)');
  const equationLogBytes = readCompileLogBytes();
  if (equationLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation env) expected empty log, got ${equationLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationLogBytes,
    { char_count: baselineStats.char_count + 19 },
    'compile_main(ok equation env)',
  );
  const equationXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation env)');
  if (equationXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation env) main.xdv expected non-empty bytes');
  }
  const equationMovement = countMovementOpsInTextPages(equationXdvBytes, 'compile_main(ok equation env)');
  if (equationMovement.down3 < 2) {
    throw new Error(`compile_main(ok equation env) expected down3>=2, got ${equationMovement.down3}`);
  }
  if (equationMovement.right3PositiveTotal !== 524288) {
    throw new Error(
      `compile_main(ok equation env) expected right3PositiveTotal=524288, got ${equationMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation* env case failed');
  const equationStarDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation*}x\\end{equation*}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationStarDocBytes, 'ok_math_env_equation_star_main') !== 0) {
    throw new Error('mount_add_file(ok equation* env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation* env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation* env)');
  const equationStarLogBytes = readCompileLogBytes();
  if (equationStarLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation* env) expected empty log, got ${equationStarLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationStarLogBytes,
    { char_count: baselineStats.char_count + 21 },
    'compile_main(ok equation* env)',
  );
  const equationStarXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation* env)');
  if (equationStarXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation* env) main.xdv expected non-empty bytes');
  }
  const equationStarMovement = countMovementOpsInTextPages(
    equationStarXdvBytes,
    'compile_main(ok equation* env)',
  );
  if (equationStarMovement.down3 < 2) {
    throw new Error(`compile_main(ok equation* env) expected down3>=2, got ${equationStarMovement.down3}`);
  }
  if (equationStarMovement.right3PositiveTotal !== 524288) {
    throw new Error(
      `compile_main(ok equation* env) expected right3PositiveTotal=524288, got ${equationStarMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before nested math env invalid case failed');
  const nestedMathEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{equation}\\begin{align}x\\end{align}\\end{equation}\\end{document}',
  );
  if (addMountedFile('main.tex', nestedMathEnvDocBytes, 'ok_math_env_nested_begin_main') !== 0) {
    throw new Error('mount_add_file(nested math env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for nested math env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(nested math env invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before missing end math env invalid case failed');
  const missingEndMathEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{equation}x\\end{document}',
  );
  if (addMountedFile('main.tex', missingEndMathEnvDocBytes, 'ok_math_env_missing_end_main') !== 0) {
    throw new Error('mount_add_file(missing end math env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for missing end math env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(missing end math env invalid)');
}
