export function runOkHeadingCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK section-star case failed');
  const sectionStarDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\section*{X}\\end{document}',
  );
  if (addMountedFile('main.tex', sectionStarDocBytes, 'ok_heading_section_star_main') !== 0) {
    throw new Error('mount_add_file(ok heading section-star main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK section-star case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok heading section-star)');
  const sectionStarLogBytes = readCompileLogBytes();
  if (sectionStarLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok heading section-star) expected empty log, got ${sectionStarLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    sectionStarLogBytes,
    { char_count: baselineStats.char_count + 2 },
    'compile_main(ok heading section-star)',
  );
  const sectionStarXdvBytes = readMainXdvArtifactBytes('compile_main(ok heading section-star)');
  if (sectionStarXdvBytes.length === 0) {
    throw new Error('compile_main(ok heading section-star) main.xdv expected non-empty bytes');
  }
  const sectionStarMovement = countMovementOpsInTextPages(
    sectionStarXdvBytes,
    'compile_main(ok heading section-star)',
  );
  if (sectionStarMovement.right3PositiveTotal !== 65536) {
    throw new Error(
      `compile_main(ok heading section-star) expected right3PositiveTotal=65536, got ${sectionStarMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK section-short case failed');
  const sectionShortDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\section[short]{X}\\end{document}',
  );
  if (addMountedFile('main.tex', sectionShortDocBytes, 'ok_heading_section_short_main') !== 0) {
    throw new Error('mount_add_file(ok heading section-short main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK section-short case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok heading section-short)');
  const sectionShortLogBytes = readCompileLogBytes();
  if (sectionShortLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok heading section-short) expected empty log, got ${sectionShortLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    sectionShortLogBytes,
    { char_count: baselineStats.char_count + 8 },
    'compile_main(ok heading section-short)',
  );
  const sectionShortXdvBytes = readMainXdvArtifactBytes('compile_main(ok heading section-short)');
  if (sectionShortXdvBytes.length === 0) {
    throw new Error('compile_main(ok heading section-short) main.xdv expected non-empty bytes');
  }
  const sectionShortMovement = countMovementOpsInTextPages(
    sectionShortXdvBytes,
    'compile_main(ok heading section-short)',
  );
  if (sectionShortMovement.right3PositiveTotal !== 65536) {
    throw new Error(
      `compile_main(ok heading section-short) expected right3PositiveTotal=65536, got ${sectionShortMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK paragraph-star case failed');
  const paragraphStarDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\paragraph*{X}\\end{document}',
  );
  if (addMountedFile('main.tex', paragraphStarDocBytes, 'ok_heading_paragraph_star_main') !== 0) {
    throw new Error('mount_add_file(ok heading paragraph-star main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK paragraph-star case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok heading paragraph-star)');
  const paragraphStarLogBytes = readCompileLogBytes();
  if (paragraphStarLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok heading paragraph-star) expected empty log, got ${paragraphStarLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    paragraphStarLogBytes,
    { char_count: baselineStats.char_count + 2 },
    'compile_main(ok heading paragraph-star)',
  );
  const paragraphStarXdvBytes = readMainXdvArtifactBytes('compile_main(ok heading paragraph-star)');
  if (paragraphStarXdvBytes.length === 0) {
    throw new Error('compile_main(ok heading paragraph-star) main.xdv expected non-empty bytes');
  }
  const paragraphStarMovement = countMovementOpsInTextPages(
    paragraphStarXdvBytes,
    'compile_main(ok heading paragraph-star)',
  );
  if (paragraphStarMovement.right3PositiveTotal !== 65536) {
    throw new Error(
      `compile_main(ok heading paragraph-star) expected right3PositiveTotal=65536, got ${paragraphStarMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before heading short missing bracket case failed');
  const invalidShortDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\section[short{X}\\end{document}',
  );
  if (addMountedFile('main.tex', invalidShortDocBytes, 'ok_heading_short_missing_bracket_main') !== 0) {
    throw new Error('mount_add_file(ok heading short missing bracket main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for heading short missing bracket case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok heading short missing bracket)');
}
