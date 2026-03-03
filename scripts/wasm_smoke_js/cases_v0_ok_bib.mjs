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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliographystyle-body baseline case failed');
  const bibStyleBodyBaselineDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\bibliographystyle{plain}\\end{document}',
  );
  if (addMountedFile('main.tex', bibStyleBodyBaselineDocBytes, 'ok_bibliographystyle_body_baseline_main') !== 0) {
    throw new Error('mount_add_file(ok bibliographystyle-body baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliographystyle-body baseline case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliographystyle-body baseline)');
  const bibStyleBodyBaselineLogBytes = readCompileLogBytes();
  if (bibStyleBodyBaselineLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliographystyle-body baseline) expected empty log, got ${bibStyleBodyBaselineLogBytes.length} bytes`,
    );
  }
  const bibStyleBodyBaselineStats = assertEventsMatchLogAndStats(
    bibStyleBodyBaselineLogBytes,
    {},
    'compile_main(ok bibliographystyle-body baseline)',
  );
  readMainXdvArtifactBytes('compile_main(ok bibliographystyle-body baseline)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliographystyle-body case failed');
  const bibStyleBodyDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\bibliographystyle{plain}B\\end{document}',
  );
  if (addMountedFile('main.tex', bibStyleBodyDocBytes, 'ok_bibliographystyle_body_main') !== 0) {
    throw new Error('mount_add_file(ok bibliographystyle-body main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliographystyle-body case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliographystyle-body)');
  const bibStyleBodyLogBytes = readCompileLogBytes();
  if (bibStyleBodyLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliographystyle-body) expected empty log, got ${bibStyleBodyLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibStyleBodyLogBytes,
    { char_count: bibStyleBodyBaselineStats.char_count + 2 },
    'compile_main(ok bibliographystyle-body)',
  );
  const bibStyleBodyXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliographystyle-body)');
  if (bibStyleBodyXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliographystyle-body) main.xdv expected non-empty bytes');
  }
  const bibStyleBodyMovement = countMovementOpsInTextPages(
    bibStyleBodyXdvBytes,
    'compile_main(ok bibliographystyle-body)',
  );
  if (bibStyleBodyMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok bibliographystyle-body) expected right3PositiveTotal=131072, got ${bibStyleBodyMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography-body baseline case failed');
  const bibliographyBodyBaselineDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\bibliography{refs}\\end{document}',
  );
  if (addMountedFile('main.tex', bibliographyBodyBaselineDocBytes, 'ok_bibliography_body_baseline_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography-body baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography-body baseline case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography-body baseline)');
  const bibliographyBodyBaselineLogBytes = readCompileLogBytes();
  if (bibliographyBodyBaselineLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography-body baseline) expected empty log, got ${bibliographyBodyBaselineLogBytes.length} bytes`,
    );
  }
  const bibliographyBodyBaselineStats = assertEventsMatchLogAndStats(
    bibliographyBodyBaselineLogBytes,
    {},
    'compile_main(ok bibliography-body baseline)',
  );
  readMainXdvArtifactBytes('compile_main(ok bibliography-body baseline)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography-body case failed');
  const bibliographyBodyDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\bibliography{refs}B\\end{document}',
  );
  if (addMountedFile('main.tex', bibliographyBodyDocBytes, 'ok_bibliography_body_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography-body main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography-body case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography-body)');
  const bibliographyBodyLogBytes = readCompileLogBytes();
  if (bibliographyBodyLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography-body) expected empty log, got ${bibliographyBodyLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibliographyBodyLogBytes,
    { char_count: bibliographyBodyBaselineStats.char_count + 2 },
    'compile_main(ok bibliography-body)',
  );
  const bibliographyBodyXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography-body)');
  if (bibliographyBodyXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography-body) main.xdv expected non-empty bytes');
  }
  const bibliographyBodyMovement = countMovementOpsInTextPages(
    bibliographyBodyXdvBytes,
    'compile_main(ok bibliography-body)',
  );
  if (bibliographyBodyMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok bibliography-body) expected right3PositiveTotal=131072, got ${bibliographyBodyMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK nocite-body baseline case failed');
  const nociteBodyBaselineDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\nocite{X,Y}\\end{document}',
  );
  if (addMountedFile('main.tex', nociteBodyBaselineDocBytes, 'ok_nocite_body_baseline_main') !== 0) {
    throw new Error('mount_add_file(ok nocite-body baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK nocite-body baseline case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok nocite-body baseline)');
  const nociteBodyBaselineLogBytes = readCompileLogBytes();
  if (nociteBodyBaselineLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok nocite-body baseline) expected empty log, got ${nociteBodyBaselineLogBytes.length} bytes`,
    );
  }
  const nociteBodyBaselineStats = assertEventsMatchLogAndStats(
    nociteBodyBaselineLogBytes,
    {},
    'compile_main(ok nocite-body baseline)',
  );
  readMainXdvArtifactBytes('compile_main(ok nocite-body baseline)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK nocite-body case failed');
  const nociteBodyDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\nocite{X,Y}B\\end{document}',
  );
  if (addMountedFile('main.tex', nociteBodyDocBytes, 'ok_nocite_body_main') !== 0) {
    throw new Error('mount_add_file(ok nocite-body main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK nocite-body case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok nocite-body)');
  const nociteBodyLogBytes = readCompileLogBytes();
  if (nociteBodyLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok nocite-body) expected empty log, got ${nociteBodyLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    nociteBodyLogBytes,
    { char_count: nociteBodyBaselineStats.char_count + 2 },
    'compile_main(ok nocite-body)',
  );
  const nociteBodyXdvBytes = readMainXdvArtifactBytes('compile_main(ok nocite-body)');
  if (nociteBodyXdvBytes.length === 0) {
    throw new Error('compile_main(ok nocite-body) main.xdv expected non-empty bytes');
  }
  const nociteBodyMovement = countMovementOpsInTextPages(
    nociteBodyXdvBytes,
    'compile_main(ok nocite-body)',
  );
  if (nociteBodyMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok nocite-body) expected right3PositiveTotal=131072, got ${nociteBodyMovement.right3PositiveTotal}`,
    );
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography natbib-label case failed');
  const bibNatbibLabelDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\protect\\citeauthoryear{A}{B}{2020}\\natexlab{a}]{K}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibNatbibLabelDocBytes, 'ok_bibliography_natbib_label_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography natbib-label main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography natbib-label case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography natbib-label)');
  const bibNatbibLabelLogBytes = readCompileLogBytes();
  if (bibNatbibLabelLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography natbib-label) expected empty log, got ${bibNatbibLabelLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibNatbibLabelLogBytes,
    { char_count: baselineStats.char_count + 42 },
    'compile_main(ok bibliography natbib-label)',
  );
  const bibNatbibLabelXdvBytes = readMainXdvArtifactBytes('compile_main(ok bibliography natbib-label)');
  if (bibNatbibLabelXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography natbib-label) main.xdv expected non-empty bytes');
  }
  const bibNatbibLabelMovement = countMovementOpsInTextPages(
    bibNatbibLabelXdvBytes,
    'compile_main(ok bibliography natbib-label)',
  );
  if (bibNatbibLabelMovement.right3PositiveTotal !== 753664) {
    throw new Error(
      `compile_main(ok bibliography natbib-label) expected right3PositiveTotal=753664, got ${bibNatbibLabelMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK bibliography natbib-label top-group case failed');
  const bibNatbibLabelTopGroupDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[{\\protect\\citeauthoryear{A}{B}{2020}\\natexlab{a}}]{K}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibNatbibLabelTopGroupDocBytes, 'ok_bibliography_natbib_label_top_group_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography natbib-label top-group main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK bibliography natbib-label top-group case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok bibliography natbib-label top-group)');
  const bibNatbibLabelTopGroupLogBytes = readCompileLogBytes();
  if (bibNatbibLabelTopGroupLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok bibliography natbib-label top-group) expected empty log, got ${bibNatbibLabelTopGroupLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    bibNatbibLabelTopGroupLogBytes,
    { char_count: baselineStats.char_count + 42 },
    'compile_main(ok bibliography natbib-label top-group)',
  );
  const bibNatbibLabelTopGroupXdvBytes = readMainXdvArtifactBytes(
    'compile_main(ok bibliography natbib-label top-group)',
  );
  if (bibNatbibLabelTopGroupXdvBytes.length === 0) {
    throw new Error('compile_main(ok bibliography natbib-label top-group) main.xdv expected non-empty bytes');
  }
  const bibNatbibLabelTopGroupMovement = countMovementOpsInTextPages(
    bibNatbibLabelTopGroupXdvBytes,
    'compile_main(ok bibliography natbib-label top-group)',
  );
  if (bibNatbibLabelTopGroupMovement.right3PositiveTotal !== 753664) {
    throw new Error(
      `compile_main(ok bibliography natbib-label top-group) expected right3PositiveTotal=753664, got ${bibNatbibLabelTopGroupMovement.right3PositiveTotal}`,
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before bibliography natbib-label missing-group invalid case failed');
  const bibNatbibLabelMissingGroupDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\citeauthoryear{A}{B}]{K}A\\end{thebibliography}\\end{document}',
  );
  if (addMountedFile('main.tex', bibNatbibLabelMissingGroupDocBytes, 'ok_bibliography_natbib_label_missing_group_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography natbib-label missing-group main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for bibliography natbib-label missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok bibliography natbib-label missing-group invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before bibliography-body missing-arg invalid case failed');
  const bibliographyBodyMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\bibliography X\\end{document}',
  );
  if (addMountedFile('main.tex', bibliographyBodyMissingArgDocBytes, 'ok_bibliography_body_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok bibliography-body missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for bibliography-body missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok bibliography-body missing-arg invalid)');
}
