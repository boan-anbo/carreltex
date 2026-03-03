export function runOkColorCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK textcolor text doc case failed');
  const textcolorTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\textcolor{red}{XYZ}\\end{document}',
  );
  if (addMountedFile('main.tex', textcolorTextDocBytes, 'ok_textcolor_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok textcolor text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK textcolor text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok textcolor text doc)');
  const textcolorLogBytes = readCompileLogBytes();
  if (textcolorLogBytes.length !== 0) {
    throw new Error(`compile_main(ok textcolor text doc) expected empty log, got ${textcolorLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    textcolorLogBytes,
    { char_count: baselineStats.char_count + 6 },
    'compile_main(ok textcolor text doc)',
  );
  const textcolorXdvBytes = readMainXdvArtifactBytes('compile_main(ok textcolor text doc)');
  if (textcolorXdvBytes.length === 0) {
    throw new Error('compile_main(ok textcolor text doc) main.xdv expected non-empty bytes');
  }
  const textcolorMovement = countMovementOpsInTextPages(
    textcolorXdvBytes,
    'compile_main(ok textcolor text doc)',
  );
  if (textcolorMovement.down3 !== 0) {
    throw new Error(`compile_main(ok textcolor text doc) expected down3=0, got ${textcolorMovement.down3}`);
  }
  if (textcolorMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok textcolor text doc) expected right3PositiveTotal=196608, got ${textcolorMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK color declaration text doc case failed');
  const colorDeclTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\color{red}XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', colorDeclTextDocBytes, 'ok_color_decl_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok color declaration text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK color declaration text doc case failed');
  }
  expectOk(ctx.compileMain(), 'compile_main_v0(ok color declaration text doc)');
  const colorDeclLogBytes = readCompileLogBytes();
  if (colorDeclLogBytes.length !== 0) {
    throw new Error(`compile_main(ok color declaration text doc) expected empty log, got ${colorDeclLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    colorDeclLogBytes,
    { char_count: baselineStats.char_count + 6 },
    'compile_main(ok color declaration text doc)',
  );
  const colorDeclXdvBytes = readMainXdvArtifactBytes('compile_main(ok color declaration text doc)');
  if (colorDeclXdvBytes.length === 0) {
    throw new Error('compile_main(ok color declaration text doc) main.xdv expected non-empty bytes');
  }
  const colorDeclMovement = countMovementOpsInTextPages(
    colorDeclXdvBytes,
    'compile_main(ok color declaration text doc)',
  );
  if (colorDeclMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok color declaration text doc) expected right3PositiveTotal=196608, got ${colorDeclMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before textcolor missing-arg invalid case failed');
  const textcolorMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\textcolor{red}XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', textcolorMissingArgDocBytes, 'ok_textcolor_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok textcolor missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for textcolor missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok textcolor missing-arg)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before color missing-arg invalid case failed');
  const colorMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\color XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', colorMissingArgDocBytes, 'ok_color_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok color missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for color missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok color missing-arg)');
}
