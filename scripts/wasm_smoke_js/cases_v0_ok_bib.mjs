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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography optional-bibitem case failed');
  const bibOptionalDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[X]{Y}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibOptionalDocBytes, 'ok_bibliography_optional_bibitem_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography optional-bibitem main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography optional-bibitem case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography optional-bibitem)');
  const bibOptionalLogBytes = readCompileLogBytes();
  if (bibOptionalLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography optional-bibitem) expected empty log, got ${bibOptionalLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibOptionalLogBytes,
    { char_count: baselineStats.char_count + 36 },
    'compile_main(ok bibliography optional-bibitem)',
  );
  const bibOptionalXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography optional-bibitem)');
  if (bibOptionalXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography optional-bibitem) main.xdv expected non-empty bytes');
  }
  const bibOptionalMovement = countMovementOpsInTextPages(
    bibOptionalXdvBytes,
    'compile_main(ok bibliography optional-bibitem)',
  );
  if (bibOptionalMovement.right3PositiveTotal !== 393216) {
    throw new Error(
      `compile_main(ok bibliography optional-bibitem) expected right3PositiveTotal=393216, got ${bibOptionalMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography label-fragment case failed');
  const bibLabelFragmentDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\textbf{X}\\url{Y}]{K}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibLabelFragmentDocBytes, 'ok_bibliography_label_fragment_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography label-fragment main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography label-fragment case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography label-fragment)');
  const bibLabelFragmentLogBytes = readCompileLogBytes();
  if (bibLabelFragmentLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography label-fragment) expected empty log, got ${bibLabelFragmentLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibLabelFragmentLogBytes,
    { char_count: baselineStats.char_count + 37 },
    'compile_main(ok bibliography label-fragment)',
  );
  const bibLabelFragmentXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography label-fragment)');
  if (bibLabelFragmentXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography label-fragment) main.xdv expected non-empty bytes');
  }
  const bibLabelFragmentMovement = countMovementOpsInTextPages(
    bibLabelFragmentXdvBytes,
    'compile_main(ok bibliography label-fragment)',
  );
  if (bibLabelFragmentMovement.right3PositiveTotal !== 458752) {
    throw new Error(
      `compile_main(ok bibliography label-fragment) expected right3PositiveTotal=458752, got ${bibLabelFragmentMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography newblock case failed');
  const bibNewblockDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem{X}A\\newblock B\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibNewblockDocBytes, 'ok_bibliography_newblock_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography newblock main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography newblock case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography newblock)');
  const bibNewblockLogBytes = readCompileLogBytes();
  if (bibNewblockLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography newblock) expected empty log, got ${bibNewblockLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibNewblockLogBytes,
    { char_count: baselineStats.char_count + 34 },
    'compile_main(ok bibliography newblock)',
  );
  const bibNewblockXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography newblock)');
  if (bibNewblockXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography newblock) main.xdv expected non-empty bytes');
  }
  const bibNewblockMovement = countMovementOpsInTextPages(
    bibNewblockXdvBytes,
    'compile_main(ok bibliography newblock)',
  );
  if (bibNewblockMovement.right3PositiveTotal !== 262144) {
    throw new Error(
      `compile_main(ok bibliography newblock) expected right3PositiveTotal=262144, got ${bibNewblockMovement.right3PositiveTotal}`,
    );
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before newblock outside bibliography invalid case failed');
  const newblockOutsideBibDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\newblock B\\end{document}',
  );
  if (addMountedFile('main.tex', newblockOutsideBibDocBytes, 'ok_newblock_outside_bib_main') !== 0) {
    throw new Error('mount_add_file(ok newblock outside bibliography main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for newblock outside bibliography invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok newblock outside bibliography)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before bibliography label-fragment begin invalid case failed');
  const bibLabelBeginInvalidDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\begin{itemize}\\item X\\end{itemize}]{K}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibLabelBeginInvalidDocBytes, 'ok_bibliography_label_begin_invalid_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography label-fragment begin invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for bibliography label-fragment begin invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok bibliography label-fragment begin invalid)');
}
