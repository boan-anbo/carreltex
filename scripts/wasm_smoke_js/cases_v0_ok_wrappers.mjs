export function runOkWrapperCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK declaration text doc case failed');
  const declarationTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\bfseries XYZ\\end{document}');
  if (addMountedFile('main.tex', declarationTextDocBytes, 'ok_declaration_text_doc_main') !== 0) throw new Error('mount_add_file(ok declaration text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK declaration text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok declaration text doc)');
  const declarationTextLogBytes = readCompileLogBytes();
  if (declarationTextLogBytes.length !== 0) throw new Error(`compile_main(ok declaration text doc) expected empty log, got ${declarationTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(declarationTextLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok declaration text doc)');
  if (readMainXdvArtifactBytes('compile_main(ok declaration text doc)').length === 0) throw new Error('compile_main(ok declaration text doc) main.xdv expected non-empty bytes');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK wrapper text doc case failed');
  const wrapperTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\textrm{A B}C\\end{document}');
  if (addMountedFile('main.tex', wrapperTextDocBytes, 'ok_wrapper_text_doc_main') !== 0) throw new Error('mount_add_file(ok wrapper text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK wrapper text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok wrapper text doc)');
  const wrapperTextLogBytes = readCompileLogBytes();
  if (wrapperTextLogBytes.length !== 0) throw new Error(`compile_main(ok wrapper text doc) expected empty log, got ${wrapperTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(wrapperTextLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok wrapper text doc)');
  const wrapperTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok wrapper text doc)');
  if (wrapperTextXdvBytes.length === 0) throw new Error('compile_main(ok wrapper text doc) main.xdv expected non-empty bytes');
  const wrapperMovement = countMovementOpsInTextPages(wrapperTextXdvBytes, 'compile_main(ok wrapper text doc)');
  if (wrapperMovement.right3 !== 4) throw new Error(`compile_main(ok wrapper text doc) expected right3=4, got ${wrapperMovement.right3}`);
  if (wrapperMovement.right3PositiveTotal !== (3 * 65536 + 32768)) throw new Error(`compile_main(ok wrapper text doc) expected right3PositiveTotal=${3 * 65536 + 32768}, got ${wrapperMovement.right3PositiveTotal}`);

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK mbox wrapper case failed');
  const mboxDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\mbox{A B}C\\end{document}');
  if (addMountedFile('main.tex', mboxDocBytes, 'ok_mbox_wrapper_main') !== 0) throw new Error('mount_add_file(ok mbox wrapper main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK mbox wrapper case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok mbox wrapper)');
  const mboxLogBytes = readCompileLogBytes();
  if (mboxLogBytes.length !== 0) throw new Error(`compile_main(ok mbox wrapper) expected empty log, got ${mboxLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(mboxLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok mbox wrapper)');
  const mboxXdvBytes = readMainXdvArtifactBytes('compile_main(ok mbox wrapper)');
  if (mboxXdvBytes.length === 0) throw new Error('compile_main(ok mbox wrapper) main.xdv expected non-empty bytes');
  const mboxMovement = countMovementOpsInTextPages(mboxXdvBytes, 'compile_main(ok mbox wrapper)');
  if (mboxMovement.right3 !== 4) throw new Error(`compile_main(ok mbox wrapper) expected right3=4, got ${mboxMovement.right3}`);
  if (mboxMovement.right3PositiveTotal !== (3 * 65536 + 32768)) throw new Error(`compile_main(ok mbox wrapper) expected right3PositiveTotal=${3 * 65536 + 32768}, got ${mboxMovement.right3PositiveTotal}`);

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK heading text doc case failed');
  const headingTextDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\section{ABC}D\\end{document}');
  if (addMountedFile('main.tex', headingTextDocBytes, 'ok_heading_text_doc_main') !== 0) throw new Error('mount_add_file(ok heading text doc main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK heading text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok heading text doc)');
  const headingTextLogBytes = readCompileLogBytes();
  if (headingTextLogBytes.length !== 0) throw new Error(`compile_main(ok heading text doc) expected empty log, got ${headingTextLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(headingTextLogBytes, { char_count: baselineStats.char_count + 4 }, 'compile_main(ok heading text doc)');
  const headingTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok heading text doc)');
  if (headingTextXdvBytes.length === 0) throw new Error('compile_main(ok heading text doc) main.xdv expected non-empty bytes');
  const headingMovement = countMovementOpsInTextPages(headingTextXdvBytes, 'compile_main(ok heading text doc)');
  if (headingMovement.down3 < 1) throw new Error(`compile_main(ok heading text doc) expected down3>=1, got ${headingMovement.down3}`);
  if (headingMovement.right3PositiveTotal !== (4 * 65536)) throw new Error(`compile_main(ok heading text doc) expected right3PositiveTotal=${4 * 65536}, got ${headingMovement.right3PositiveTotal}`);

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK maketitle emit case failed');
  const maketitleDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\end{document}',
  );
  if (addMountedFile('main.tex', maketitleDocBytes, 'ok_maketitle_emit_main') !== 0) throw new Error('mount_add_file(ok maketitle emit main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK maketitle emit case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok maketitle emit)');
  const maketitleLogBytes = readCompileLogBytes();
  if (maketitleLogBytes.length !== 0) throw new Error(`compile_main(ok maketitle emit) expected empty log, got ${maketitleLogBytes.length} bytes`);
  assertEventsMatchLogAndStats(maketitleLogBytes, { char_count: baselineStats.char_count + 3 }, 'compile_main(ok maketitle emit)');
  const maketitleXdvBytes = readMainXdvArtifactBytes('compile_main(ok maketitle emit)');
  if (maketitleXdvBytes.length === 0) throw new Error('compile_main(ok maketitle emit) main.xdv expected non-empty bytes');
  const maketitleMovement = countMovementOpsInTextPages(maketitleXdvBytes, 'compile_main(ok maketitle emit)');
  if (maketitleMovement.right3PositiveTotal !== (3 * 65536)) throw new Error(`compile_main(ok maketitle emit) expected right3PositiveTotal=${3 * 65536}, got ${maketitleMovement.right3PositiveTotal}`);

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before wrapper missing-group invalid case failed');
  const invalidWrapperDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\textbf XYZ\\end{document}');
  if (addMountedFile('main.tex', invalidWrapperDocBytes, 'ok_wrapper_missing_group_main') !== 0) throw new Error('mount_add_file(ok wrapper missing-group main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for wrapper missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok wrapper missing-group)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before heading missing-group invalid case failed');
  const invalidHeadingDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\section XYZ\\end{document}');
  if (addMountedFile('main.tex', invalidHeadingDocBytes, 'ok_heading_missing_group_main') !== 0) throw new Error('mount_add_file(ok heading missing-group main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for heading missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok heading missing-group)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before textsf missing-group invalid case failed');
  const invalidTextsfDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\textsf XYZ\\end{document}');
  if (addMountedFile('main.tex', invalidTextsfDocBytes, 'ok_textsf_missing_group_main') !== 0) throw new Error('mount_add_file(ok textsf missing-group main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for textsf missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok textsf missing-group)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before mbox missing-group invalid case failed');
  const invalidMboxDocBytes = new TextEncoder().encode('\\documentclass{article}\\begin{document}\\mbox X\\end{document}');
  if (addMountedFile('main.tex', invalidMboxDocBytes, 'ok_mbox_missing_group_main') !== 0) throw new Error('mount_add_file(ok mbox missing-group main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for mbox missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok mbox missing-group)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before title missing-group invalid case failed');
  const invalidTitleDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\title X\\begin{document}\\maketitle\\end{document}',
  );
  if (addMountedFile('main.tex', invalidTitleDocBytes, 'ok_title_missing_group_main') !== 0) throw new Error('mount_add_file(ok title missing-group main.tex) failed');
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for title missing-group invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok title missing-group)');
}
