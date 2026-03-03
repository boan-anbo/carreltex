export function runOkThmEnvCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK theorem env case failed');
  const theoremDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{theorem}x\\end{theorem}B\\end{document}',
  );
  if (addMountedFile('main.tex', theoremDocBytes, 'ok_thm_env_theorem_main') !== 0) {
    throw new Error('mount_add_file(ok theorem env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK theorem env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok theorem env)');
  const theoremLogBytes = readCompileLogBytes();
  if (theoremLogBytes.length !== 0) {
    throw new Error(`compile_main(ok theorem env) expected empty log, got ${theoremLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    theoremLogBytes,
    { char_count: baselineStats.char_count + 17 },
    'compile_main(ok theorem env)',
  );
  const theoremXdvBytes = readMainXdvArtifactBytes('compile_main(ok theorem env)');
  if (theoremXdvBytes.length === 0) {
    throw new Error('compile_main(ok theorem env) main.xdv expected non-empty bytes');
  }
  const theoremMovement = countMovementOpsInTextPages(theoremXdvBytes, 'compile_main(ok theorem env)');
  if (theoremMovement.down3 < 2) {
    throw new Error(`compile_main(ok theorem env) expected down3>=2, got ${theoremMovement.down3}`);
  }
  if (theoremMovement.right3PositiveTotal !== 458752) {
    throw new Error(
      `compile_main(ok theorem env) expected right3PositiveTotal=458752, got ${theoremMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK proof env case failed');
  const proofDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{proof}x\\end{proof}B\\end{document}',
  );
  if (addMountedFile('main.tex', proofDocBytes, 'ok_thm_env_proof_main') !== 0) {
    throw new Error('mount_add_file(ok proof env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK proof env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok proof env)');
  const proofLogBytes = readCompileLogBytes();
  if (proofLogBytes.length !== 0) {
    throw new Error(`compile_main(ok proof env) expected empty log, got ${proofLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    proofLogBytes,
    { char_count: baselineStats.char_count + 13 },
    'compile_main(ok proof env)',
  );
  const proofXdvBytes = readMainXdvArtifactBytes('compile_main(ok proof env)');
  if (proofXdvBytes.length === 0) {
    throw new Error('compile_main(ok proof env) main.xdv expected non-empty bytes');
  }
  const proofMovement = countMovementOpsInTextPages(proofXdvBytes, 'compile_main(ok proof env)');
  if (proofMovement.down3 < 2) {
    throw new Error(`compile_main(ok proof env) expected down3>=2, got ${proofMovement.down3}`);
  }
  if (proofMovement.right3PositiveTotal !== 589824) {
    throw new Error(
      `compile_main(ok proof env) expected right3PositiveTotal=589824, got ${proofMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before nested theorem env invalid case failed');
  const nestedThmDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{theorem}\\begin{proof}X\\end{proof}\\end{theorem}\\end{document}',
  );
  if (addMountedFile('main.tex', nestedThmDocBytes, 'ok_thm_env_nested_begin_main') !== 0) {
    throw new Error('mount_add_file(nested theorem env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for nested theorem env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(nested theorem env invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before missing end theorem env invalid case failed');
  const missingEndThmDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{theorem}X\\end{document}',
  );
  if (addMountedFile('main.tex', missingEndThmDocBytes, 'ok_thm_env_missing_end_main') !== 0) {
    throw new Error('mount_add_file(missing end theorem env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for missing end theorem env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(missing end theorem env invalid)');
}
