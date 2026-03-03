export function runOkListCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK itemize list case failed');
  const itemizeTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{itemize}\\item ABC\\item D\\end{itemize}X\\end{document}',
  );
  if (addMountedFile('main.tex', itemizeTextDocBytes, 'ok_itemize_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok itemize text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK itemize list case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok itemize text doc)');
  const itemizeLogBytes = readCompileLogBytes();
  if (itemizeLogBytes.length !== 0) {
    throw new Error(`compile_main(ok itemize text doc) expected empty log, got ${itemizeLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(itemizeLogBytes, { char_count: baselineStats.char_count + 19 }, 'compile_main(ok itemize text doc)');
  const itemizeXdvBytes = readMainXdvArtifactBytes('compile_main(ok itemize text doc)');
  if (itemizeXdvBytes.length === 0) {
    throw new Error('compile_main(ok itemize text doc) main.xdv expected non-empty bytes');
  }
  const itemizeMovement = countMovementOpsInTextPages(itemizeXdvBytes, 'compile_main(ok itemize text doc)');
  if (itemizeMovement.down3 < 3) {
    throw new Error(`compile_main(ok itemize text doc) expected down3>=3, got ${itemizeMovement.down3}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before item outside list invalid case failed');
  const itemOutsideDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\item A\\end{document}',
  );
  if (addMountedFile('main.tex', itemOutsideDocBytes, 'ok_item_outside_list_main') !== 0) {
    throw new Error('mount_add_file(ok item outside list main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for item outside list invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok item outside list)');
}
