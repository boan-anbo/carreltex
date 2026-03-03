export function runOkFloatCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before includegraphics marker case failed');
  const includeGraphicsDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\includegraphics{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', includeGraphicsDocBytes, 'ok_includegraphics_marker_main') !== 0) {
    throw new Error('mount_add_file(ok includegraphics marker main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for includegraphics marker case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok includegraphics marker)');
  const includeGraphicsLogBytes = readCompileLogBytes();
  if (includeGraphicsLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok includegraphics marker) expected empty log, got ${includeGraphicsLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    includeGraphicsLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok includegraphics marker)',
  );
  const includeGraphicsXdvBytes = readMainXdvArtifactBytes('compile_main(ok includegraphics marker)');
  if (includeGraphicsXdvBytes.length === 0) {
    throw new Error('compile_main(ok includegraphics marker) main.xdv expected non-empty bytes');
  }
  const includeGraphicsMovement = countMovementOpsInTextPages(
    includeGraphicsXdvBytes,
    'compile_main(ok includegraphics marker)',
  );
  if (includeGraphicsMovement.right3 !== 8) {
    throw new Error(
      `compile_main(ok includegraphics marker) expected right3=8, got ${includeGraphicsMovement.right3}`,
    );
  }
  if (includeGraphicsMovement.right3PositiveTotal !== 491520) {
    throw new Error(
      `compile_main(ok includegraphics marker) expected right3PositiveTotal=491520, got ${includeGraphicsMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before figure+caption+ref case failed');
  const floatDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{figure}[ht]\\caption{A}\\includegraphics[width=10]{IMG}\\label{k}\\ref{k}\\end{figure}B\\end{document}',
  );
  if (addMountedFile('main.tex', floatDocBytes, 'ok_float_caption_ref_main') !== 0) {
    throw new Error('mount_add_file(ok float caption ref main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for figure+caption+ref case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok float caption ref)');
  const floatLogBytes = readCompileLogBytes();
  if (floatLogBytes.length !== 0) {
    throw new Error(`compile_main(ok float caption ref) expected empty log, got ${floatLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(floatLogBytes, {}, 'compile_main(ok float caption ref)');
  const floatXdvBytes = readMainXdvArtifactBytes('compile_main(ok float caption ref)');
  if (floatXdvBytes.length === 0) {
    throw new Error('compile_main(ok float caption ref) main.xdv expected non-empty bytes');
  }
  const floatMovement = countMovementOpsInTextPages(floatXdvBytes, 'compile_main(ok float caption ref)');
  if (floatMovement.right3 !== 15) {
    throw new Error(`compile_main(ok float caption ref) expected right3=15, got ${floatMovement.right3}`);
  }
  if (floatMovement.down3 !== 2) {
    throw new Error(`compile_main(ok float caption ref) expected down3=2, got ${floatMovement.down3}`);
  }
  if (floatMovement.right3PositiveTotal !== 851968) {
    throw new Error(
      `compile_main(ok float caption ref) expected right3PositiveTotal=851968, got ${floatMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before missing-bracket invalid case failed');
  const badPlacementDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{figure}[ht\\caption{A}\\end{figure}\\end{document}',
  );
  if (addMountedFile('main.tex', badPlacementDocBytes, 'ok_float_bad_placement_main') !== 0) {
    throw new Error('mount_add_file(ok float bad placement main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for missing-bracket invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok float missing bracket)');
}
