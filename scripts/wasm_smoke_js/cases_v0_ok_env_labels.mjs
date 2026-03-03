export function runOkEnvLabelCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation label case failed');
  const equationLabelDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\label{a}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationLabelDocBytes, 'ok_env_label_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation label main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation label case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation label)');
  const equationLabelLogBytes = readCompileLogBytes();
  if (equationLabelLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok equation label) expected empty log, got ${equationLabelLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    equationLabelLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok equation label)',
  );
  const equationLabelXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation label)');
  if (equationLabelXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation label) main.xdv expected non-empty bytes');
  }
  const equationLabelMovement = countMovementOpsInTextPages(
    equationLabelXdvBytes,
    'compile_main(ok equation label)',
  );
  if (equationLabelMovement.right3PositiveTotal !== 819200) {
    throw new Error(
      `compile_main(ok equation label) expected right3PositiveTotal=819200, got ${equationLabelMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK theorem label case failed');
  const theoremLabelDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{theorem}x\\label{a}\\end{theorem}B\\end{document}',
  );
  if (addMountedFile('main.tex', theoremLabelDocBytes, 'ok_env_label_theorem_main') !== 0) {
    throw new Error('mount_add_file(ok theorem label main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK theorem label case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok theorem label)');
  const theoremLabelLogBytes = readCompileLogBytes();
  if (theoremLabelLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok theorem label) expected empty log, got ${theoremLabelLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    theoremLabelLogBytes,
    { char_count: baselineStats.char_count + 18 },
    'compile_main(ok theorem label)',
  );
  const theoremLabelXdvBytes = readMainXdvArtifactBytes('compile_main(ok theorem label)');
  if (theoremLabelXdvBytes.length === 0) {
    throw new Error('compile_main(ok theorem label) main.xdv expected non-empty bytes');
  }
  const theoremLabelMovement = countMovementOpsInTextPages(
    theoremLabelXdvBytes,
    'compile_main(ok theorem label)',
  );
  if (theoremLabelMovement.right3PositiveTotal !== 819200) {
    throw new Error(
      `compile_main(ok theorem label) expected right3PositiveTotal=819200, got ${theoremLabelMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK top-level label case failed');
  const labelOutsideEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\label{a}B\\end{document}',
  );
  if (addMountedFile('main.tex', labelOutsideEnvDocBytes, 'ok_env_label_outside_main') !== 0) {
    throw new Error('mount_add_file(ok top-level label main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK top-level label case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok top-level label)');
  const labelOutsideLogBytes = readCompileLogBytes();
  if (labelOutsideLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok top-level label) expected empty log, got ${labelOutsideLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    labelOutsideLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok top-level label)',
  );
  const labelOutsideXdvBytes = readMainXdvArtifactBytes('compile_main(ok top-level label)');
  if (labelOutsideXdvBytes.length === 0) {
    throw new Error('compile_main(ok top-level label) main.xdv expected non-empty bytes');
  }
  const labelOutsideMovement = countMovementOpsInTextPages(
    labelOutsideXdvBytes,
    'compile_main(ok top-level label)',
  );
  if (labelOutsideMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok top-level label) expected right3PositiveTotal=131072, got ${labelOutsideMovement.right3PositiveTotal}`,
    );
  }
}
