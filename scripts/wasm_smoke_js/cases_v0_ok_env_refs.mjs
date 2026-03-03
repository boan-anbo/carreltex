export function runOkEnvRefCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation ref case failed');
  const equationRefDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\ref{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationRefDocBytes, 'ok_env_ref_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation ref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation ref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation ref)');
  const equationRefLogBytes = readCompileLogBytes();
  if (equationRefLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation ref) expected empty log, got ${equationRefLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationRefLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok equation ref)',
  );
  const equationRefXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation ref)');
  if (equationRefXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation ref) main.xdv expected non-empty bytes');
  }
  const equationRefMovement = countMovementOpsInTextPages(
    equationRefXdvBytes,
    'compile_main(ok equation ref)',
  );
  if (equationRefMovement.right3PositiveTotal !== 1015808) {
    throw new Error(
      `compile_main(ok equation ref) expected right3PositiveTotal=1015808, got ${equationRefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK theorem ref case failed');
  const theoremRefDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{theorem}x\\autoref{r}\\end{theorem}B\\end{document}',
  );
  if (addMountedFile('main.tex', theoremRefDocBytes, 'ok_env_ref_theorem_main') !== 0) {
    throw new Error('mount_add_file(ok theorem ref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK theorem ref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok theorem ref)');
  const theoremRefLogBytes = readCompileLogBytes();
  if (theoremRefLogBytes.length !== 0) {
    throw new Error(`compile_main(ok theorem ref) expected empty log, got ${theoremRefLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    theoremRefLogBytes,
    { char_count: baselineStats.char_count + 18 },
    'compile_main(ok theorem ref)',
  );
  const theoremRefXdvBytes = readMainXdvArtifactBytes('compile_main(ok theorem ref)');
  if (theoremRefXdvBytes.length === 0) {
    throw new Error('compile_main(ok theorem ref) main.xdv expected non-empty bytes');
  }
  const theoremRefMovement = countMovementOpsInTextPages(theoremRefXdvBytes, 'compile_main(ok theorem ref)');
  if (theoremRefMovement.right3PositiveTotal !== 1015808) {
    throw new Error(
      `compile_main(ok theorem ref) expected right3PositiveTotal=1015808, got ${theoremRefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK top-level ref case failed');
  const refOutsideEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\ref{r}B\\end{document}',
  );
  if (addMountedFile('main.tex', refOutsideEnvDocBytes, 'ok_env_ref_outside_main') !== 0) {
    throw new Error('mount_add_file(ok top-level ref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK top-level ref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok top-level ref)');
  const refOutsideLogBytes = readCompileLogBytes();
  if (refOutsideLogBytes.length !== 0) {
    throw new Error(`compile_main(ok top-level ref) expected empty log, got ${refOutsideLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    refOutsideLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok top-level ref)',
  );
  const refOutsideXdvBytes = readMainXdvArtifactBytes('compile_main(ok top-level ref)');
  if (refOutsideXdvBytes.length === 0) {
    throw new Error('compile_main(ok top-level ref) main.xdv expected non-empty bytes');
  }
  const refOutsideMovement = countMovementOpsInTextPages(
    refOutsideXdvBytes,
    'compile_main(ok top-level ref)',
  );
  if (refOutsideMovement.right3PositiveTotal !== 491520) {
    throw new Error(
      `compile_main(ok top-level ref) expected right3PositiveTotal=491520, got ${refOutsideMovement.right3PositiveTotal}`,
    );
  }
}
