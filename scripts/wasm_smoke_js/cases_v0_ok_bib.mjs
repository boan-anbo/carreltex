export function runOkBibCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography env case failed');
  const bibEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem{X}ABC\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibEnvDocBytes, 'ok_bibliography_env_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography env)');
  const bibEnvLogBytes = readCompileLogBytes();
  if (bibEnvLogBytes.length !== 0) {
    throw new Error(`compile_main(ok bibliography env) expected empty log, got ${bibEnvLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    bibEnvLogBytes,
    { char_count: baselineStats.char_count + 35 },
    'compile_main(ok bibliography env)',
  );
  const bibEnvXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography env)');
  if (bibEnvXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography env) main.xdv expected non-empty bytes');
  }
  const bibEnvMovement = countMovementOpsInTextPages(bibEnvXdvBytes, 'compile_main(ok bibliography env)');
  if (bibEnvMovement.down3 < 1) {
    throw new Error(`compile_main(ok bibliography env) expected down3>=1, got ${bibEnvMovement.down3}`);
  }
  if (bibEnvMovement.right3PositiveTotal !== 294912) {
    throw new Error(
      `compile_main(ok bibliography env) expected right3PositiveTotal=294912, got ${bibEnvMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography preamble baseline case failed');
  const bibPreambleBaselineBytes = new TextEncoder().encode(
    '\\documentclass{article}\\bibliographystyle{plain}\\bibliography{refs}\\begin{document}\\end{document}',
  );
  if (addMountedFile('main.tex', bibPreambleBaselineBytes, 'ok_bibliography_preamble_baseline_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography preamble baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography preamble baseline case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography preamble baseline)');
  const bibPreambleBaselineLogBytes = readCompileLogBytes();
  if (bibPreambleBaselineLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography preamble baseline) expected empty log, got ${bibPreambleBaselineLogBytes.length} bytes`,
    );
  }
  const bibPreambleBaselineStats = assertEventsMatchLogAndStats(
    bibPreambleBaselineLogBytes,
    {},
    'compile_main(ok bibliography preamble baseline)',
  );
  readMainXdvArtifactBytes('compile_main(ok bibliography preamble baseline)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography preamble case failed');
  const bibPreambleDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\bibliographystyle{plain}\\bibliography{refs}\\begin{document}XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', bibPreambleDocBytes, 'ok_bibliography_preamble_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography preamble main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography preamble case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography preamble)');
  const bibPreambleLogBytes = readCompileLogBytes();
  if (bibPreambleLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography preamble) expected empty log, got ${bibPreambleLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibPreambleLogBytes,
    { char_count: bibPreambleBaselineStats.char_count + 3 },
    'compile_main(ok bibliography preamble)',
  );
  const bibPreambleXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography preamble)');
  if (bibPreambleXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography preamble) main.xdv expected non-empty bytes');
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before bibitem outside env invalid case failed');
  const bibitemOutsideEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\bibitem{X}A\\end{document}',
  );
  if (addMountedFile('main.tex', bibitemOutsideEnvDocBytes, 'ok_bibitem_outside_env_main') !== 0) {
    throw new Error('mount_add_file(ok bibitem outside env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for bibitem outside env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok bibitem outside env)');
}
