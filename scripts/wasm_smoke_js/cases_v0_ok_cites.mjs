export function runOkCiteCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK cite text doc case failed');
  const citeTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\cite{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', citeTextDocBytes, 'ok_cite_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok cite text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK cite text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok cite text doc)');
  const citeLogBytes = readCompileLogBytes();
  if (citeLogBytes.length !== 0) {
    throw new Error(`compile_main(ok cite text doc) expected empty log, got ${citeLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(citeLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok cite text doc)');
  const citeXdvBytes = readMainXdvArtifactBytes('compile_main(ok cite text doc)');
  if (citeXdvBytes.length === 0) {
    throw new Error('compile_main(ok cite text doc) main.xdv expected non-empty bytes');
  }
  const citeMovement = countMovementOpsInTextPages(citeXdvBytes, 'compile_main(ok cite text doc)');
  if (citeMovement.right3 !== 9) {
    throw new Error(`compile_main(ok cite text doc) expected right3=9, got ${citeMovement.right3}`);
  }
  if (citeMovement.right3PositiveTotal !== 557056) {
    throw new Error(`compile_main(ok cite text doc) expected right3PositiveTotal=557056, got ${citeMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before cite missing-arg invalid case failed');
  const citeMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\cite X\\end{document}',
  );
  if (addMountedFile('main.tex', citeMissingArgDocBytes, 'ok_cite_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok cite missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for cite missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok cite missing-arg)');
}
