export function runOkEnvCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK center env case failed');
  const centerEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{center}B\\end{center}C\\end{document}',
  );
  if (addMountedFile('main.tex', centerEnvDocBytes, 'ok_center_env_main') !== 0) {
    throw new Error('mount_add_file(ok center env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK center env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok center env)');
  const centerLogBytes = readCompileLogBytes();
  if (centerLogBytes.length !== 0) {
    throw new Error(`compile_main(ok center env) expected empty log, got ${centerLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    centerLogBytes,
    { char_count: baselineStats.char_count + 15 },
    'compile_main(ok center env)',
  );
  const centerXdvBytes = readMainXdvArtifactBytes('compile_main(ok center env)');
  if (centerXdvBytes.length === 0) {
    throw new Error('compile_main(ok center env) main.xdv expected non-empty bytes');
  }
  const centerMovement = countMovementOpsInTextPages(centerXdvBytes, 'compile_main(ok center env)');
  if (centerMovement.down3 < 2) {
    throw new Error(`compile_main(ok center env) expected down3>=2, got ${centerMovement.down3}`);
  }
  if (centerMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok center env) expected right3PositiveTotal=196608, got ${centerMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK verbatim env case failed');
  const verbatimEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{verbatim}x\\end{verbatim}B\\end{document}',
  );
  if (addMountedFile('main.tex', verbatimEnvDocBytes, 'ok_verbatim_env_main') !== 0) {
    throw new Error('mount_add_file(ok verbatim env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK verbatim env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok verbatim env)');
  const verbatimLogBytes = readCompileLogBytes();
  if (verbatimLogBytes.length !== 0) {
    throw new Error(`compile_main(ok verbatim env) expected empty log, got ${verbatimLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    verbatimLogBytes,
    { char_count: baselineStats.char_count + 19 },
    'compile_main(ok verbatim env)',
  );
  const verbatimXdvBytes = readMainXdvArtifactBytes('compile_main(ok verbatim env)');
  if (verbatimXdvBytes.length === 0) {
    throw new Error('compile_main(ok verbatim env) main.xdv expected non-empty bytes');
  }
  const verbatimMovement = countMovementOpsInTextPages(verbatimXdvBytes, 'compile_main(ok verbatim env)');
  if (verbatimMovement.down3 < 2) {
    throw new Error(`compile_main(ok verbatim env) expected down3>=2, got ${verbatimMovement.down3}`);
  }
  if (verbatimMovement.right3PositiveTotal !== 786432) {
    throw new Error(
      `compile_main(ok verbatim env) expected right3PositiveTotal=786432, got ${verbatimMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before nested begin env invalid case failed');
  const nestedBeginDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{center}\\begin{quote}X\\end{quote}\\end{center}\\end{document}',
  );
  if (addMountedFile('main.tex', nestedBeginDocBytes, 'ok_env_nested_begin_main') !== 0) {
    throw new Error('mount_add_file(nested begin env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for nested begin env invalid case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(nested begin env)');
  const nestedBeginLogBytes = readCompileLogBytes();
  if (nestedBeginLogBytes.length !== 0) {
    throw new Error(`compile_main(nested begin env) expected empty log, got ${nestedBeginLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(nestedBeginLogBytes, {}, 'compile_main(nested begin env)');
  const nestedBeginXdvBytes = readMainXdvArtifactBytes('compile_main(nested begin env)');
  if (nestedBeginXdvBytes.length === 0) {
    throw new Error('compile_main(nested begin env) main.xdv expected non-empty bytes');
  }
  const nestedBeginMovement = countMovementOpsInTextPages(nestedBeginXdvBytes, 'compile_main(nested begin env)');
  if (nestedBeginMovement.right3PositiveTotal !== 65536) {
    throw new Error(
      `compile_main(nested begin env) expected right3PositiveTotal=65536, got ${nestedBeginMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before missing end env invalid case failed');
  const missingEndDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{center}X\\end{document}',
  );
  if (addMountedFile('main.tex', missingEndDocBytes, 'ok_env_missing_end_main') !== 0) {
    throw new Error('mount_add_file(missing end env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for missing end env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(missing end env invalid)');
}
