export function runOkEnvCiteCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation cite case failed');
  const equationCiteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\cite{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationCiteDocBytes, 'ok_env_cite_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation cite main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation cite case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation cite)');
  const equationCiteLogBytes = readCompileLogBytes();
  if (equationCiteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation cite) expected empty log, got ${equationCiteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationCiteLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok equation cite)',
  );
  const equationCiteXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation cite)');
  if (equationCiteXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation cite) main.xdv expected non-empty bytes');
  }
  const equationCiteMovement = countMovementOpsInTextPages(
    equationCiteXdvBytes,
    'compile_main(ok equation cite)',
  );
  if (equationCiteMovement.right3PositiveTotal !== 950272) {
    throw new Error(
      `compile_main(ok equation cite) expected right3PositiveTotal=950272, got ${equationCiteMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK theorem cite case failed');
  const theoremCiteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{theorem}x\\citep{r}\\end{theorem}B\\end{document}',
  );
  if (addMountedFile('main.tex', theoremCiteDocBytes, 'ok_env_cite_theorem_main') !== 0) {
    throw new Error('mount_add_file(ok theorem cite main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK theorem cite case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok theorem cite)');
  const theoremCiteLogBytes = readCompileLogBytes();
  if (theoremCiteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok theorem cite) expected empty log, got ${theoremCiteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    theoremCiteLogBytes,
    { char_count: baselineStats.char_count + 18 },
    'compile_main(ok theorem cite)',
  );
  const theoremCiteXdvBytes = readMainXdvArtifactBytes('compile_main(ok theorem cite)');
  if (theoremCiteXdvBytes.length === 0) {
    throw new Error('compile_main(ok theorem cite) main.xdv expected non-empty bytes');
  }
  const theoremCiteMovement = countMovementOpsInTextPages(theoremCiteXdvBytes, 'compile_main(ok theorem cite)');
  if (theoremCiteMovement.right3PositiveTotal !== 884736) {
    throw new Error(
      `compile_main(ok theorem cite) expected right3PositiveTotal=884736, got ${theoremCiteMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK top-level cite case failed');
  const citeOutsideEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\cite{r}B\\end{document}',
  );
  if (addMountedFile('main.tex', citeOutsideEnvDocBytes, 'ok_env_cite_outside_main') !== 0) {
    throw new Error('mount_add_file(ok top-level cite main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK top-level cite case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok top-level cite)');
  const citeOutsideLogBytes = readCompileLogBytes();
  if (citeOutsideLogBytes.length !== 0) {
    throw new Error(`compile_main(ok top-level cite) expected empty log, got ${citeOutsideLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    citeOutsideLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok top-level cite)',
  );
  const citeOutsideXdvBytes = readMainXdvArtifactBytes('compile_main(ok top-level cite)');
  if (citeOutsideXdvBytes.length === 0) {
    throw new Error('compile_main(ok top-level cite) main.xdv expected non-empty bytes');
  }
  const citeOutsideMovement = countMovementOpsInTextPages(
    citeOutsideXdvBytes,
    'compile_main(ok top-level cite)',
  );
  if (citeOutsideMovement.right3PositiveTotal !== 557056) {
    throw new Error(
      `compile_main(ok top-level cite) expected right3PositiveTotal=557056, got ${citeOutsideMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation cite optional-notes case failed');
  const equationCiteOptionalNotesDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\cite[see][p.1]{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationCiteOptionalNotesDocBytes, 'ok_env_cite_equation_optional_notes_main') !== 0) {
    throw new Error('mount_add_file(ok equation cite optional-notes main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation cite optional-notes case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation cite optional-notes)');
  const equationCiteOptionalNotesLogBytes = readCompileLogBytes();
  if (equationCiteOptionalNotesLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok equation cite optional-notes) expected empty log, got ${equationCiteOptionalNotesLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    equationCiteOptionalNotesLogBytes,
    {},
    'compile_main(ok equation cite optional-notes)',
  );
  const equationCiteOptionalNotesXdvBytes = readMainXdvArtifactBytes(
    'compile_main(ok equation cite optional-notes)',
  );
  if (equationCiteOptionalNotesXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation cite optional-notes) main.xdv expected non-empty bytes');
  }
  const equationCiteOptionalNotesMovement = countMovementOpsInTextPages(
    equationCiteOptionalNotesXdvBytes,
    'compile_main(ok equation cite optional-notes)',
  );
  if (equationCiteOptionalNotesMovement.right3PositiveTotal !== 950272) {
    throw new Error(
      `compile_main(ok equation cite optional-notes) expected right3PositiveTotal=950272, got ${equationCiteOptionalNotesMovement.right3PositiveTotal}`,
    );
  }
}
