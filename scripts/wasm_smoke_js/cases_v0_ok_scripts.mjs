export function runOkScriptCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK superscript text doc case failed');
  const superscriptTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\textsuperscript{B}C\\end{document}',
  );
  if (addMountedFile('main.tex', superscriptTextDocBytes, 'ok_superscript_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok superscript text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK superscript text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok superscript text doc)');
  const superscriptLogBytes = readCompileLogBytes();
  if (superscriptLogBytes.length !== 0) {
    throw new Error(`compile_main(ok superscript text doc) expected empty log, got ${superscriptLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    superscriptLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok superscript text doc)',
  );
  const superscriptXdvBytes = readMainXdvArtifactBytes('compile_main(ok superscript text doc)');
  if (superscriptXdvBytes.length === 0) {
    throw new Error('compile_main(ok superscript text doc) main.xdv expected non-empty bytes');
  }
  const superscriptMovement = countMovementOpsInTextPages(
    superscriptXdvBytes,
    'compile_main(ok superscript text doc)',
  );
  if (superscriptMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok superscript text doc) expected right3PositiveTotal=196608, got ${superscriptMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before subscript missing-arg invalid case failed');
  const subscriptMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\textsubscript XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', subscriptMissingArgDocBytes, 'ok_subscript_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok subscript missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for subscript missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok subscript missing-arg)');
}
