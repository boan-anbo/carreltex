export function runOkFootnoteCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK footnote text doc case failed');
  const footnoteTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\footnote{B}C\\end{document}',
  );
  if (addMountedFile('main.tex', footnoteTextDocBytes, 'ok_footnote_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok footnote text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK footnote text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok footnote text doc)');
  const footnoteLogBytes = readCompileLogBytes();
  if (footnoteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok footnote text doc) expected empty log, got ${footnoteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    footnoteLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok footnote text doc)',
  );
  const footnoteXdvBytes = readMainXdvArtifactBytes('compile_main(ok footnote text doc)');
  if (footnoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok footnote text doc) main.xdv expected non-empty bytes');
  }
  const footnoteMovement = countMovementOpsInTextPages(footnoteXdvBytes, 'compile_main(ok footnote text doc)');
  if (footnoteMovement.right3 !== 6) {
    throw new Error(`compile_main(ok footnote text doc) expected right3=6, got ${footnoteMovement.right3}`);
  }
  if (footnoteMovement.right3PositiveTotal !== 360448) {
    throw new Error(`compile_main(ok footnote text doc) expected right3PositiveTotal=360448, got ${footnoteMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK footnotemark text doc case failed');
  const footnotemarkTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\footnotemark B\\end{document}',
  );
  if (addMountedFile('main.tex', footnotemarkTextDocBytes, 'ok_footnotemark_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok footnotemark text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK footnotemark text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok footnotemark text doc)');
  const footnotemarkLogBytes = readCompileLogBytes();
  if (footnotemarkLogBytes.length !== 0) {
    throw new Error(`compile_main(ok footnotemark text doc) expected empty log, got ${footnotemarkLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    footnotemarkLogBytes,
    { char_count: baselineStats.char_count + 2 },
    'compile_main(ok footnotemark text doc)',
  );
  const footnotemarkXdvBytes = readMainXdvArtifactBytes('compile_main(ok footnotemark text doc)');
  if (footnotemarkXdvBytes.length === 0) {
    throw new Error('compile_main(ok footnotemark text doc) main.xdv expected non-empty bytes');
  }
  const footnotemarkMovement = countMovementOpsInTextPages(
    footnotemarkXdvBytes,
    'compile_main(ok footnotemark text doc)',
  );
  if (footnotemarkMovement.right3 !== 2) {
    throw new Error(`compile_main(ok footnotemark text doc) expected right3=2, got ${footnotemarkMovement.right3}`);
  }
  if (footnotemarkMovement.right3PositiveTotal !== 131072) {
    throw new Error(`compile_main(ok footnotemark text doc) expected right3PositiveTotal=131072, got ${footnotemarkMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK footnotetext text doc case failed');
  const footnotetextTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\footnotetext{B}C\\end{document}',
  );
  if (addMountedFile('main.tex', footnotetextTextDocBytes, 'ok_footnotetext_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok footnotetext text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK footnotetext text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok footnotetext text doc)');
  const footnotetextLogBytes = readCompileLogBytes();
  if (footnotetextLogBytes.length !== 0) {
    throw new Error(`compile_main(ok footnotetext text doc) expected empty log, got ${footnotetextLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    footnotetextLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok footnotetext text doc)',
  );
  const footnotetextXdvBytes = readMainXdvArtifactBytes('compile_main(ok footnotetext text doc)');
  if (footnotetextXdvBytes.length === 0) {
    throw new Error('compile_main(ok footnotetext text doc) main.xdv expected non-empty bytes');
  }
  const footnotetextMovement = countMovementOpsInTextPages(
    footnotetextXdvBytes,
    'compile_main(ok footnotetext text doc)',
  );
  if (footnotetextMovement.right3 !== 6) {
    throw new Error(`compile_main(ok footnotetext text doc) expected right3=6, got ${footnotetextMovement.right3}`);
  }
  if (footnotetextMovement.right3PositiveTotal !== 360448) {
    throw new Error(`compile_main(ok footnotetext text doc) expected right3PositiveTotal=360448, got ${footnotetextMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before footnote missing-arg invalid case failed');
  const footnoteMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\footnote XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', footnoteMissingArgDocBytes, 'ok_footnote_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok footnote missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for footnote missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok footnote missing-arg)');
}
