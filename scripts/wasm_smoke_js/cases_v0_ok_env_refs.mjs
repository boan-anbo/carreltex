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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation ref optional-note case failed');
  const equationRefOptionalNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\ref[see]{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationRefOptionalNoteDocBytes, 'ok_env_ref_equation_optional_note_main') !== 0) {
    throw new Error('mount_add_file(ok equation ref optional-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation ref optional-note case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation ref optional-note)');
  const equationRefOptionalNoteLogBytes = readCompileLogBytes();
  if (equationRefOptionalNoteLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok equation ref optional-note) expected empty log, got ${equationRefOptionalNoteLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(equationRefOptionalNoteLogBytes, {}, 'compile_main(ok equation ref optional-note)');
  const equationRefOptionalNoteXdvBytes = readMainXdvArtifactBytes(
    'compile_main(ok equation ref optional-note)',
  );
  if (equationRefOptionalNoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation ref optional-note) main.xdv expected non-empty bytes');
  }
  const equationRefOptionalNoteMovement = countMovementOpsInTextPages(
    equationRefOptionalNoteXdvBytes,
    'compile_main(ok equation ref optional-note)',
  );
  if (equationRefOptionalNoteMovement.right3PositiveTotal !== 1015808) {
    throw new Error(
      `compile_main(ok equation ref optional-note) expected right3PositiveTotal=1015808, got ${equationRefOptionalNoteMovement.right3PositiveTotal}`,
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation cref case failed');
  const equationCrefDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\cref{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationCrefDocBytes, 'ok_env_cref_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation cref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation cref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation cref)');
  const equationCrefLogBytes = readCompileLogBytes();
  if (equationCrefLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation cref) expected empty log, got ${equationCrefLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationCrefLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok equation cref)',
  );
  const equationCrefXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation cref)');
  if (equationCrefXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation cref) main.xdv expected non-empty bytes');
  }
  const equationCrefMovement = countMovementOpsInTextPages(
    equationCrefXdvBytes,
    'compile_main(ok equation cref)',
  );
  if (equationCrefMovement.right3PositiveTotal !== 1015808) {
    throw new Error(
      `compile_main(ok equation cref) expected right3PositiveTotal=1015808, got ${equationCrefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK theorem Cref case failed');
  const theoremCrefDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{theorem}x\\Cref{r}\\end{theorem}B\\end{document}',
  );
  if (addMountedFile('main.tex', theoremCrefDocBytes, 'ok_env_cref_theorem_main') !== 0) {
    throw new Error('mount_add_file(ok theorem Cref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK theorem Cref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok theorem Cref)');
  const theoremCrefLogBytes = readCompileLogBytes();
  if (theoremCrefLogBytes.length !== 0) {
    throw new Error(`compile_main(ok theorem Cref) expected empty log, got ${theoremCrefLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    theoremCrefLogBytes,
    { char_count: baselineStats.char_count + 18 },
    'compile_main(ok theorem Cref)',
  );
  const theoremCrefXdvBytes = readMainXdvArtifactBytes('compile_main(ok theorem Cref)');
  if (theoremCrefXdvBytes.length === 0) {
    throw new Error('compile_main(ok theorem Cref) main.xdv expected non-empty bytes');
  }
  const theoremCrefMovement = countMovementOpsInTextPages(
    theoremCrefXdvBytes,
    'compile_main(ok theorem Cref)',
  );
  if (theoremCrefMovement.right3PositiveTotal !== 1015808) {
    throw new Error(
      `compile_main(ok theorem Cref) expected right3PositiveTotal=1015808, got ${theoremCrefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation pageref case failed');
  const equationPagerefDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\pageref{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationPagerefDocBytes, 'ok_env_pageref_equation_main') !== 0) {
    throw new Error('mount_add_file(ok equation pageref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation pageref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation pageref)');
  const equationPagerefLogBytes = readCompileLogBytes();
  if (equationPagerefLogBytes.length !== 0) {
    throw new Error(`compile_main(ok equation pageref) expected empty log, got ${equationPagerefLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    equationPagerefLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok equation pageref)',
  );
  const equationPagerefXdvBytes = readMainXdvArtifactBytes('compile_main(ok equation pageref)');
  if (equationPagerefXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation pageref) main.xdv expected non-empty bytes');
  }
  const equationPagerefMovement = countMovementOpsInTextPages(
    equationPagerefXdvBytes,
    'compile_main(ok equation pageref)',
  );
  if (equationPagerefMovement.right3PositiveTotal !== 1146880) {
    throw new Error(
      `compile_main(ok equation pageref) expected right3PositiveTotal=1146880, got ${equationPagerefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK equation pageref optional-note case failed');
  const equationPagerefOptionalNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{equation}x\\pageref[see]{r}\\end{equation}B\\end{document}',
  );
  if (addMountedFile('main.tex', equationPagerefOptionalNoteDocBytes, 'ok_env_pageref_equation_optional_note_main') !== 0) {
    throw new Error('mount_add_file(ok equation pageref optional-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK equation pageref optional-note case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok equation pageref optional-note)');
  const equationPagerefOptionalNoteLogBytes = readCompileLogBytes();
  if (equationPagerefOptionalNoteLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok equation pageref optional-note) expected empty log, got ${equationPagerefOptionalNoteLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    equationPagerefOptionalNoteLogBytes,
    {},
    'compile_main(ok equation pageref optional-note)',
  );
  const equationPagerefOptionalNoteXdvBytes = readMainXdvArtifactBytes(
    'compile_main(ok equation pageref optional-note)',
  );
  if (equationPagerefOptionalNoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok equation pageref optional-note) main.xdv expected non-empty bytes');
  }
  const equationPagerefOptionalNoteMovement = countMovementOpsInTextPages(
    equationPagerefOptionalNoteXdvBytes,
    'compile_main(ok equation pageref optional-note)',
  );
  if (equationPagerefOptionalNoteMovement.right3PositiveTotal !== 1146880) {
    throw new Error(
      `compile_main(ok equation pageref optional-note) expected right3PositiveTotal=1146880, got ${equationPagerefOptionalNoteMovement.right3PositiveTotal}`,
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK top-level pageref case failed');
  const pagerefOutsideEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\pageref{r}B\\end{document}',
  );
  if (addMountedFile('main.tex', pagerefOutsideEnvDocBytes, 'ok_env_ref_pageref_outside_main') !== 0) {
    throw new Error('mount_add_file(ok top-level pageref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK top-level pageref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok top-level pageref)');
  const pagerefOutsideLogBytes = readCompileLogBytes();
  if (pagerefOutsideLogBytes.length !== 0) {
    throw new Error(`compile_main(ok top-level pageref) expected empty log, got ${pagerefOutsideLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    pagerefOutsideLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok top-level pageref)',
  );
  const pagerefOutsideXdvBytes = readMainXdvArtifactBytes('compile_main(ok top-level pageref)');
  if (pagerefOutsideXdvBytes.length === 0) {
    throw new Error('compile_main(ok top-level pageref) main.xdv expected non-empty bytes');
  }
  const pagerefOutsideMovement = countMovementOpsInTextPages(
    pagerefOutsideXdvBytes,
    'compile_main(ok top-level pageref)',
  );
  if (pagerefOutsideMovement.right3PositiveTotal !== 753664) {
    throw new Error(
      `compile_main(ok top-level pageref) expected right3PositiveTotal=753664, got ${pagerefOutsideMovement.right3PositiveTotal}`,
    );
  }
}
