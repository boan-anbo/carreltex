export function runOkOptionalBracketCases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK caption[short] case failed');
  const captionShortDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\caption[short]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', captionShortDocBytes, 'ok_optional_caption_short_main') !== 0) {
    throw new Error('mount_add_file(ok caption[short] main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK caption[short] case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok caption[short])');
  const captionShortLogBytes = readCompileLogBytes();
  if (captionShortLogBytes.length !== 0) {
    throw new Error(`compile_main(ok caption[short]) expected empty log, got ${captionShortLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(captionShortLogBytes, {}, 'compile_main(ok caption[short])');
  const captionShortXdvBytes = readMainXdvArtifactBytes('compile_main(ok caption[short])');
  if (captionShortXdvBytes.length === 0) {
    throw new Error('compile_main(ok caption[short]) main.xdv expected non-empty bytes');
  }
  const captionShortMovement = countMovementOpsInTextPages(captionShortXdvBytes, 'compile_main(ok caption[short])');
  if (captionShortMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok caption[short]) expected right3PositiveTotal=196608, got ${captionShortMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK caption*[short] case failed');
  const captionStarShortDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\caption*[short]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', captionStarShortDocBytes, 'ok_optional_caption_star_short_main') !== 0) {
    throw new Error('mount_add_file(ok caption*[short] main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK caption*[short] case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok caption*[short])');
  const captionStarShortLogBytes = readCompileLogBytes();
  if (captionStarShortLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok caption*[short]) expected empty log, got ${captionStarShortLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(captionStarShortLogBytes, {}, 'compile_main(ok caption*[short])');
  const captionStarShortXdvBytes = readMainXdvArtifactBytes('compile_main(ok caption*[short])');
  if (captionStarShortXdvBytes.length === 0) {
    throw new Error('compile_main(ok caption*[short]) main.xdv expected non-empty bytes');
  }
  const captionStarShortMovement = countMovementOpsInTextPages(
    captionStarShortXdvBytes,
    'compile_main(ok caption*[short])',
  );
  if (captionStarShortMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok caption*[short]) expected right3PositiveTotal=196608, got ${captionStarShortMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK footnote[1] case failed');
  const footnoteNumDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\footnote[1]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', footnoteNumDocBytes, 'ok_optional_footnote_num_main') !== 0) {
    throw new Error('mount_add_file(ok footnote[1] main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK footnote[1] case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok footnote[1])');
  const footnoteNumLogBytes = readCompileLogBytes();
  if (footnoteNumLogBytes.length !== 0) {
    throw new Error(`compile_main(ok footnote[1]) expected empty log, got ${footnoteNumLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(footnoteNumLogBytes, {}, 'compile_main(ok footnote[1])');
  const footnoteNumXdvBytes = readMainXdvArtifactBytes('compile_main(ok footnote[1])');
  if (footnoteNumXdvBytes.length === 0) {
    throw new Error('compile_main(ok footnote[1]) main.xdv expected non-empty bytes');
  }
  const footnoteNumMovement = countMovementOpsInTextPages(footnoteNumXdvBytes, 'compile_main(ok footnote[1])');
  if (footnoteNumMovement.right3PositiveTotal !== 360448) {
    throw new Error(
      `compile_main(ok footnote[1]) expected right3PositiveTotal=360448, got ${footnoteNumMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK footnotemark[2] case failed');
  const footnotemarkNumDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\footnotemark[2]B\\end{document}',
  );
  if (addMountedFile('main.tex', footnotemarkNumDocBytes, 'ok_optional_footnotemark_num_main') !== 0) {
    throw new Error('mount_add_file(ok footnotemark[2] main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK footnotemark[2] case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok footnotemark[2])');
  const footnotemarkNumLogBytes = readCompileLogBytes();
  if (footnotemarkNumLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok footnotemark[2]) expected empty log, got ${footnotemarkNumLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(footnotemarkNumLogBytes, {}, 'compile_main(ok footnotemark[2])');
  const footnotemarkNumXdvBytes = readMainXdvArtifactBytes('compile_main(ok footnotemark[2])');
  if (footnotemarkNumXdvBytes.length === 0) {
    throw new Error('compile_main(ok footnotemark[2]) main.xdv expected non-empty bytes');
  }
  const footnotemarkNumMovement = countMovementOpsInTextPages(
    footnotemarkNumXdvBytes,
    'compile_main(ok footnotemark[2])',
  );
  if (footnotemarkNumMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok footnotemark[2]) expected right3PositiveTotal=131072, got ${footnotemarkNumMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before invalid footnote[ab] case failed');
  const invalidFootnoteNumDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\footnote[ab]{X}\\end{document}',
  );
  if (addMountedFile('main.tex', invalidFootnoteNumDocBytes, 'ok_optional_footnote_invalid_num_main') !== 0) {
    throw new Error('mount_add_file(ok footnote[ab] main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for invalid footnote[ab] case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok footnote[ab] invalid)');
}
