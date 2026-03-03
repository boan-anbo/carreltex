export function runOkQuoteCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK enquote text doc case failed');
  const enquoteTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\enquote{B}C\\end{document}',
  );
  if (addMountedFile('main.tex', enquoteTextDocBytes, 'ok_enquote_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok enquote text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK enquote text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok enquote text doc)');
  const enquoteLogBytes = readCompileLogBytes();
  if (enquoteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok enquote text doc) expected empty log, got ${enquoteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    enquoteLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok enquote text doc)',
  );
  const enquoteXdvBytes = readMainXdvArtifactBytes('compile_main(ok enquote text doc)');
  if (enquoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok enquote text doc) main.xdv expected non-empty bytes');
  }
  const enquoteMovement = countMovementOpsInTextPages(enquoteXdvBytes, 'compile_main(ok enquote text doc)');
  if (enquoteMovement.right3 !== 5) {
    throw new Error(`compile_main(ok enquote text doc) expected right3=5, got ${enquoteMovement.right3}`);
  }
  if (enquoteMovement.right3PositiveTotal !== 327680) {
    throw new Error(`compile_main(ok enquote text doc) expected right3PositiveTotal=327680, got ${enquoteMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before quote missing-arg invalid case failed');
  const quoteMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\quote XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', quoteMissingArgDocBytes, 'ok_quote_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok quote missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for quote missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok quote missing-arg)');
}
