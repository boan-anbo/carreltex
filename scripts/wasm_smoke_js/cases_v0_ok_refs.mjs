export function runOkRefCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK ref text doc case failed');
  const refTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\ref{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', refTextDocBytes, 'ok_ref_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok ref text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK ref text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok ref text doc)');
  const refLogBytes = readCompileLogBytes();
  if (refLogBytes.length !== 0) {
    throw new Error(`compile_main(ok ref text doc) expected empty log, got ${refLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(refLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok ref text doc)');
  const refXdvBytes = readMainXdvArtifactBytes('compile_main(ok ref text doc)');
  if (refXdvBytes.length === 0) throw new Error('compile_main(ok ref text doc) main.xdv expected non-empty bytes');
  const refMovement = countMovementOpsInTextPages(refXdvBytes, 'compile_main(ok ref text doc)');
  if (refMovement.right3 !== 8) {
    throw new Error(`compile_main(ok ref text doc) expected right3=8, got ${refMovement.right3}`);
  }
  if (refMovement.right3PositiveTotal !== 491520) {
    throw new Error(
      `compile_main(ok ref text doc) expected right3PositiveTotal=491520, got ${refMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK ref optional-note text doc case failed');
  const refOptionalNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\ref[see]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', refOptionalNoteDocBytes, 'ok_ref_optional_note_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok ref optional-note text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK ref optional-note text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok ref optional-note text doc)');
  const refOptionalNoteLogBytes = readCompileLogBytes();
  if (refOptionalNoteLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok ref optional-note text doc) expected empty log, got ${refOptionalNoteLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    refOptionalNoteLogBytes,
    { char_count: baselineStats.char_count + 8 },
    'compile_main(ok ref optional-note text doc)',
  );
  const refOptionalNoteXdvBytes = readMainXdvArtifactBytes('compile_main(ok ref optional-note text doc)');
  if (refOptionalNoteXdvBytes.length === 0) {
    throw new Error('compile_main(ok ref optional-note text doc) main.xdv expected non-empty bytes');
  }
  const refOptionalNoteMovement = countMovementOpsInTextPages(
    refOptionalNoteXdvBytes,
    'compile_main(ok ref optional-note text doc)',
  );
  if (refOptionalNoteMovement.right3PositiveTotal !== 491520) {
    throw new Error(
      `compile_main(ok ref optional-note text doc) expected right3PositiveTotal=491520, got ${refOptionalNoteMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK eqref optional-note text doc case failed');
  const eqrefOptionalNotesDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\eqref[see][p.1]{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', eqrefOptionalNotesDocBytes, 'ok_eqref_optional_notes_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok eqref optional-notes text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK eqref optional-note text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok eqref optional-notes text doc)');
  const eqrefOptionalNotesLogBytes = readCompileLogBytes();
  if (eqrefOptionalNotesLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok eqref optional-notes text doc) expected empty log, got ${eqrefOptionalNotesLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(eqrefOptionalNotesLogBytes, {}, 'compile_main(ok eqref optional-notes text doc)');
  const eqrefOptionalNotesXdvBytes = readMainXdvArtifactBytes('compile_main(ok eqref optional-notes text doc)');
  if (eqrefOptionalNotesXdvBytes.length === 0) {
    throw new Error('compile_main(ok eqref optional-notes text doc) main.xdv expected non-empty bytes');
  }
  const eqrefOptionalNotesMovement = countMovementOpsInTextPages(
    eqrefOptionalNotesXdvBytes,
    'compile_main(ok eqref optional-notes text doc)',
  );
  if (eqrefOptionalNotesMovement.right3PositiveTotal !== 622592) {
    throw new Error(
      `compile_main(ok eqref optional-notes text doc) expected right3PositiveTotal=622592, got ${eqrefOptionalNotesMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK label text doc case failed');
  const labelTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\label{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', labelTextDocBytes, 'ok_label_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok label text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK label text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok label text doc)');
  const labelLogBytes = readCompileLogBytes();
  if (labelLogBytes.length !== 0) {
    throw new Error(`compile_main(ok label text doc) expected empty log, got ${labelLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(labelLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok label text doc)');
  const labelXdvBytes = readMainXdvArtifactBytes('compile_main(ok label text doc)');
  if (labelXdvBytes.length === 0) {
    throw new Error('compile_main(ok label text doc) main.xdv expected non-empty bytes');
  }
  const labelMovement = countMovementOpsInTextPages(labelXdvBytes, 'compile_main(ok label text doc)');
  if (labelMovement.right3 !== 2) {
    throw new Error(`compile_main(ok label text doc) expected right3=2, got ${labelMovement.right3}`);
  }
  if (labelMovement.right3PositiveTotal !== 131072) {
    throw new Error(
      `compile_main(ok label text doc) expected right3PositiveTotal=131072, got ${labelMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK pageref text doc case failed');
  const pagerefTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\pageref{X}B\\end{document}',
  );
  if (addMountedFile('main.tex', pagerefTextDocBytes, 'ok_pageref_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok pageref text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK pageref text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok pageref text doc)');
  const pagerefLogBytes = readCompileLogBytes();
  if (pagerefLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok pageref text doc) expected empty log, got ${pagerefLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(
    pagerefLogBytes,
    { char_count: baselineStats.char_count + 3 },
    'compile_main(ok pageref text doc)',
  );
  const pagerefXdvBytes = readMainXdvArtifactBytes('compile_main(ok pageref text doc)');
  if (pagerefXdvBytes.length === 0) {
    throw new Error('compile_main(ok pageref text doc) main.xdv expected non-empty bytes');
  }
  const pagerefMovement = countMovementOpsInTextPages(
    pagerefXdvBytes,
    'compile_main(ok pageref text doc)',
  );
  if (pagerefMovement.right3 !== 12) {
    throw new Error(`compile_main(ok pageref text doc) expected right3=12, got ${pagerefMovement.right3}`);
  }
  if (pagerefMovement.right3PositiveTotal !== 753664) {
    throw new Error(
      `compile_main(ok pageref text doc) expected right3PositiveTotal=753664, got ${pagerefMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before ref missing-arg invalid case failed');
  const refMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\ref X\\end{document}',
  );
  if (addMountedFile('main.tex', refMissingArgDocBytes, 'ok_ref_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok ref missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for ref missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok ref missing-arg)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before ref unclosed-note invalid case failed');
  const refUnclosedNoteDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\ref[see{X}\\end{document}',
  );
  if (addMountedFile('main.tex', refUnclosedNoteDocBytes, 'ok_ref_unclosed_note_main') !== 0) {
    throw new Error('mount_add_file(ok ref unclosed-note main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for ref unclosed-note invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok ref unclosed-note)');
}
