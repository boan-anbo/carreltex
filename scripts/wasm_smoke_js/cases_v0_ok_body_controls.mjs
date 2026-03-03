export function runOkBodyControlCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countPagesInDviV2,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK begin/end with space text doc case failed');
  const beginEndSpaceTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin {document}\nXYZ\n\\end {document}\n');
  if (addMountedFile('main.tex', beginEndSpaceTextDocBytes, 'ok_begin_end_space_text_doc_main') !== 0) throw new Error('mount_add_file(ok begin/end with space text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK begin/end with space text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok begin/end with space text doc)');
  const beginEndSpaceTextLogBytes = readCompileLogBytes();
  if (beginEndSpaceTextLogBytes.length !== 0) throw new Error(`compile_main(ok begin/end with space text doc) expected empty log, got ${beginEndSpaceTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(beginEndSpaceTextLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok begin/end with space text doc)');
  if (readMainXdvArtifactBytes('compile_main(ok begin/end with space text doc)').length === 0) throw new Error('compile_main(ok begin/end with space text doc) main.xdv expected non-empty bytes');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK begin trailing-space text doc case failed');
  const beginTrailingSpaceTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document} \nXYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', beginTrailingSpaceTextDocBytes, 'ok_begin_trailing_space_text_doc_main') !== 0) throw new Error('mount_add_file(ok begin trailing-space text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK begin trailing-space text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok begin trailing-space text doc)');
  const beginTrailingSpaceTextLogBytes = readCompileLogBytes();
  if (beginTrailingSpaceTextLogBytes.length !== 0) throw new Error(`compile_main(ok begin trailing-space text doc) expected empty log, got ${beginTrailingSpaceTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(beginTrailingSpaceTextLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok begin trailing-space text doc)');
  if (readMainXdvArtifactBytes('compile_main(ok begin trailing-space text doc)').length === 0) throw new Error('compile_main(ok begin trailing-space text doc) main.xdv expected non-empty bytes');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK par control-seq text doc case failed');
  const parControlSeqTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}A\\par B\\end{document}');
  if (addMountedFile('main.tex', parControlSeqTextDocBytes, 'ok_par_control_seq_text_doc_main') !== 0) throw new Error('mount_add_file(ok par control-seq text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK par control-seq text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok par control-seq text doc)');
  const parControlSeqTextLogBytes = readCompileLogBytes();
  if (parControlSeqTextLogBytes.length !== 0) throw new Error(`compile_main(ok par control-seq text doc) expected empty log, got ${parControlSeqTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(parControlSeqTextLogBytes, { char_count: baselineStats.char_count + 2 }, 'compile_main(ok par control-seq text doc)');
  const parControlSeqTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok par control-seq text doc)');
  if (parControlSeqTextXdvBytes.length === 0) throw new Error('compile_main(ok par control-seq text doc) main.xdv expected non-empty bytes');
  if (countMovementOpsInTextPages(parControlSeqTextXdvBytes, 'compile_main(ok par control-seq text doc)').right3 !== 3) throw new Error('compile_main(ok par control-seq text doc) expected right3=3 for A space B');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK noindent text doc case failed');
  const noindentTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\noindent XYZ\\end{document}');
  if (addMountedFile('main.tex', noindentTextDocBytes, 'ok_noindent_text_doc_main') !== 0) throw new Error('mount_add_file(ok noindent text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK noindent text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok noindent text doc)');
  const noindentTextLogBytes = readCompileLogBytes();
  if (noindentTextLogBytes.length !== 0) throw new Error(`compile_main(ok noindent text doc) expected empty log, got ${noindentTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(noindentTextLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok noindent text doc)');
  if (readMainXdvArtifactBytes('compile_main(ok noindent text doc)').length === 0) throw new Error('compile_main(ok noindent text doc) main.xdv expected non-empty bytes');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK verb payload text doc case failed');
  const verbPayloadTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}A\\verb|b c|D\\end{document}');
  if (addMountedFile('main.tex', verbPayloadTextDocBytes, 'ok_verb_payload_text_doc_main') !== 0) throw new Error('mount_add_file(ok verb payload text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK verb payload text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok verb payload text doc)');
  const verbPayloadTextLogBytes = readCompileLogBytes();
  if (verbPayloadTextLogBytes.length !== 0) throw new Error(`compile_main(ok verb payload text doc) expected empty log, got ${verbPayloadTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(verbPayloadTextLogBytes, { char_count: baselineStats.char_count + 5 }, 'compile_main(ok verb payload text doc)');
  const verbPayloadTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok verb payload text doc)');
  if (verbPayloadTextXdvBytes.length === 0) throw new Error('compile_main(ok verb payload text doc) main.xdv expected non-empty bytes');
  if (countMovementOpsInTextPages(verbPayloadTextXdvBytes, 'compile_main(ok verb payload text doc)').right3 !== 5) throw new Error('compile_main(ok verb payload text doc) expected right3=5 for A b<space>c D');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK newline control-seq text doc case failed');
  const newlineControlSeqTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}A\\csname newline\\endcsname B\\end{document}');
  if (addMountedFile('main.tex', newlineControlSeqTextDocBytes, 'ok_newline_control_seq_text_doc_main') !== 0) throw new Error('mount_add_file(ok newline control-seq text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK newline control-seq text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok newline control-seq text doc)');
  const newlineControlSeqTextLogBytes = readCompileLogBytes();
  if (newlineControlSeqTextLogBytes.length !== 0) throw new Error(`compile_main(ok newline control-seq text doc) expected empty log, got ${newlineControlSeqTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(newlineControlSeqTextLogBytes, { char_count: baselineStats.char_count + 2 }, 'compile_main(ok newline control-seq text doc)');
  const newlineControlSeqTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok newline control-seq text doc)');
  if (newlineControlSeqTextXdvBytes.length === 0) throw new Error('compile_main(ok newline control-seq text doc) main.xdv expected non-empty bytes');
  const newlineMovement = countMovementOpsInTextPages(newlineControlSeqTextXdvBytes, 'compile_main(ok newline control-seq text doc)');
  if (newlineMovement.right3 !== 3) throw new Error('compile_main(ok newline control-seq text doc) expected right3=3 (A width + reset + B width)');
  if (newlineMovement.right3PositiveAmounts.length !== 2) throw new Error(`compile_main(ok newline control-seq text doc) expected exactly 2 positive right3 amounts (A and B), got ${newlineMovement.right3PositiveAmounts.length}`);
  if (newlineMovement.down3 !== 1) throw new Error('compile_main(ok newline control-seq text doc) expected exactly one DOWN3');
  if (countPagesInDviV2(newlineControlSeqTextXdvBytes, 'compile_main(ok newline control-seq text doc)') !== 1) throw new Error('compile_main(ok newline control-seq text doc) expected one page');

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK newline text doc case failed');
  }
  const newlineTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\newline B\\end{document}',
  );
  if (addMountedFile('main.tex', newlineTextDocBytes, 'ok_newline_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok newline text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK newline text doc case failed');
  }
  expectOk(ctx.compileMain(), 'compile_main_v0(ok newline text doc)');
  const newlineLogBytes = readCompileLogBytes();
  if (newlineLogBytes.length !== 0) {
    throw new Error(`compile_main(ok newline text doc) expected empty log, got ${newlineLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    newlineLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok newline text doc)',
  );
  const newlineXdvBytes = readMainXdvArtifactBytes('compile_main(ok newline text doc)');
  if (newlineXdvBytes.length === 0) {
    throw new Error('compile_main(ok newline text doc) main.xdv expected non-empty bytes');
  }
  const newlinePages = countPagesInDviV2(newlineXdvBytes, 'compile_main(ok newline text doc)');
  if (newlinePages !== 1) {
    throw new Error(`compile_main(ok newline text doc) expected 1 page, got ${newlinePages}`);
  }
  const newlineTextMovement = countMovementOpsInTextPages(newlineXdvBytes, 'compile_main(ok newline text doc)');
  if (newlineTextMovement.down3 !== 1) {
    throw new Error(`compile_main(ok newline text doc) expected exactly one DOWN3 opcode, got ${newlineTextMovement.down3}`);
  }
}
