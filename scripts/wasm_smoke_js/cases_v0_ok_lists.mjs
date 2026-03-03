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
  if (itemizeMovement.right3PositiveTotal !== 524288) {
    throw new Error(`compile_main(ok itemize text doc) expected right3PositiveTotal=524288, got ${itemizeMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK enumerate list case failed');
  const enumerateTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{enumerate}\\item A\\item B\\end{enumerate}\\end{document}',
  );
  if (addMountedFile('main.tex', enumerateTextDocBytes, 'ok_enumerate_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok enumerate text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK enumerate list case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok enumerate text doc)');
  const enumerateLogBytes = readCompileLogBytes();
  if (enumerateLogBytes.length !== 0) {
    throw new Error(`compile_main(ok enumerate text doc) expected empty log, got ${enumerateLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(enumerateLogBytes, {}, 'compile_main(ok enumerate text doc)');
  const enumerateXdvBytes = readMainXdvArtifactBytes('compile_main(ok enumerate text doc)');
  if (enumerateXdvBytes.length === 0) {
    throw new Error('compile_main(ok enumerate text doc) main.xdv expected non-empty bytes');
  }
  const enumerateMovement = countMovementOpsInTextPages(enumerateXdvBytes, 'compile_main(ok enumerate text doc)');
  if (enumerateMovement.down3 < 3) {
    throw new Error(`compile_main(ok enumerate text doc) expected down3>=3, got ${enumerateMovement.down3}`);
  }
  if (enumerateMovement.right3PositiveTotal !== 393216) {
    throw new Error(`compile_main(ok enumerate text doc) expected right3PositiveTotal=393216, got ${enumerateMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before nested list OK case failed');
  const nestedListDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{itemize}\\item A\\begin{enumerate}\\item B\\end{enumerate}\\item C\\end{itemize}\\end{document}',
  );
  if (addMountedFile('main.tex', nestedListDocBytes, 'ok_nested_list_main') !== 0) {
    throw new Error('mount_add_file(ok nested list main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for nested list OK case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok nested list)');
  const nestedListLogBytes = readCompileLogBytes();
  if (nestedListLogBytes.length !== 0) {
    throw new Error(`compile_main(ok nested list) expected empty log, got ${nestedListLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(nestedListLogBytes, {}, 'compile_main(ok nested list)');
  const nestedListXdvBytes = readMainXdvArtifactBytes('compile_main(ok nested list)');
  if (nestedListXdvBytes.length === 0) {
    throw new Error('compile_main(ok nested list) main.xdv expected non-empty bytes');
  }
  const nestedListMovement = countMovementOpsInTextPages(nestedListXdvBytes, 'compile_main(ok nested list)');
  if (nestedListMovement.right3PositiveTotal !== 589824) {
    throw new Error(
      `compile_main(ok nested list) expected right3PositiveTotal=589824, got ${nestedListMovement.right3PositiveTotal}`,
    );
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
