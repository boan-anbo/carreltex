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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK cite one-note case failed');
  const citeOneNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\cite[see]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', citeOneNoteDocBytes, 'ok_cite_one_note_main') !== 0) {
    throw new Error('mount_add_file(ok cite one-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK cite one-note case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok cite one-note)');
  const citeOneNoteLogBytes = readCompileLogBytes();
  if (citeOneNoteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok cite one-note) expected empty log, got ${citeOneNoteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(citeOneNoteLogBytes, { char_count: baselineStats.char_count + 8 }, 'compile_main(ok cite one-note)');
  const citeOneNoteXdvBytes = readMainXdvArtifactBytes('compile_main(ok cite one-note)');
  if (citeOneNoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok cite one-note) main.xdv expected non-empty bytes');
  }
  const citeOneNoteMovement = countMovementOpsInTextPages(citeOneNoteXdvBytes, 'compile_main(ok cite one-note)');
  if (citeOneNoteMovement.right3 !== 9) {
    throw new Error(`compile_main(ok cite one-note) expected right3=9, got ${citeOneNoteMovement.right3}`);
  }
  if (citeOneNoteMovement.right3PositiveTotal !== 557056) {
    throw new Error(`compile_main(ok cite one-note) expected right3PositiveTotal=557056, got ${citeOneNoteMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK cite two-note case failed');
  const citeTwoNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\cite[see][p.1]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', citeTwoNoteDocBytes, 'ok_cite_two_note_main') !== 0) {
    throw new Error('mount_add_file(ok cite two-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK cite two-note case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok cite two-note)');
  const citeTwoNoteLogBytes = readCompileLogBytes();
  if (citeTwoNoteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok cite two-note) expected empty log, got ${citeTwoNoteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(citeTwoNoteLogBytes, { char_count: baselineStats.char_count + 13 }, 'compile_main(ok cite two-note)');
  const citeTwoNoteXdvBytes = readMainXdvArtifactBytes('compile_main(ok cite two-note)');
  if (citeTwoNoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok cite two-note) main.xdv expected non-empty bytes');
  }
  const citeTwoNoteMovement = countMovementOpsInTextPages(citeTwoNoteXdvBytes, 'compile_main(ok cite two-note)');
  if (citeTwoNoteMovement.right3 !== 9) {
    throw new Error(`compile_main(ok cite two-note) expected right3=9, got ${citeTwoNoteMovement.right3}`);
  }
  if (citeTwoNoteMovement.right3PositiveTotal !== 557056) {
    throw new Error(`compile_main(ok cite two-note) expected right3PositiveTotal=557056, got ${citeTwoNoteMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK citep-star case failed');
  const citepStarDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\citep*{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', citepStarDocBytes, 'ok_citep_star_main') !== 0) {
    throw new Error('mount_add_file(ok citep-star main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK citep-star case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok citep-star)');
  const citepStarLogBytes = readCompileLogBytes();
  if (citepStarLogBytes.length !== 0) {
    throw new Error(`compile_main(ok citep-star) expected empty log, got ${citepStarLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(citepStarLogBytes, { char_count: baselineStats.char_count + 4 }, 'compile_main(ok citep-star)');
  const citepStarXdvBytes = readMainXdvArtifactBytes('compile_main(ok citep-star)');
  if (citepStarXdvBytes.length === 0) {
    throw new Error('compile_main(ok citep-star) main.xdv expected non-empty bytes');
  }
  const citepStarMovement = countMovementOpsInTextPages(citepStarXdvBytes, 'compile_main(ok citep-star)');
  if (citepStarMovement.right3 !== 9) {
    throw new Error(`compile_main(ok citep-star) expected right3=9, got ${citepStarMovement.right3}`);
  }
  if (citepStarMovement.right3PositiveTotal !== 557056) {
    throw new Error(`compile_main(ok citep-star) expected right3PositiveTotal=557056, got ${citepStarMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK parencite case failed');
  const parenciteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\parencite{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', parenciteDocBytes, 'ok_parencite_main') !== 0) {
    throw new Error('mount_add_file(ok parencite main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK parencite case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok parencite)');
  const parenciteLogBytes = readCompileLogBytes();
  if (parenciteLogBytes.length !== 0) {
    throw new Error(`compile_main(ok parencite) expected empty log, got ${parenciteLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(parenciteLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok parencite)');
  const parenciteXdvBytes = readMainXdvArtifactBytes('compile_main(ok parencite)');
  if (parenciteXdvBytes.length === 0) {
    throw new Error('compile_main(ok parencite) main.xdv expected non-empty bytes');
  }
  const parenciteMovement = countMovementOpsInTextPages(parenciteXdvBytes, 'compile_main(ok parencite)');
  if (parenciteMovement.right3 !== 9) {
    throw new Error(`compile_main(ok parencite) expected right3=9, got ${parenciteMovement.right3}`);
  }
  if (parenciteMovement.right3PositiveTotal !== 557056) {
    throw new Error(`compile_main(ok parencite) expected right3PositiveTotal=557056, got ${parenciteMovement.right3PositiveTotal}`);
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

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before cite unclosed-note invalid case failed');
  const citeUnclosedNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\cite[see{X}\\end{document}',
  );
  if (addMountedFile('main.tex', citeUnclosedNoteDocBytes, 'ok_cite_unclosed_note_main') !== 0) {
    throw new Error('mount_add_file(ok cite unclosed-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for cite unclosed-note invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok cite unclosed-note)');
}
