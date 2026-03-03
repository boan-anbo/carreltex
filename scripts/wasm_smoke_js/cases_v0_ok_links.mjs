export function runOkLinkCases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK href text doc case failed');
  const hrefTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\href{https://example.test/path?q=1}{XYZ}\\end{document}',
  );
  if (addMountedFile('main.tex', hrefTextDocBytes, 'ok_href_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok href text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK href text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok href text doc)');
  const hrefTextLogBytes = readCompileLogBytes();
  if (hrefTextLogBytes.length !== 0) {
    throw new Error(`compile_main(ok href text doc) expected empty log, got ${hrefTextLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(hrefTextLogBytes, {}, 'compile_main(ok href text doc)');
  const hrefTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok href text doc)');
  if (hrefTextXdvBytes.length === 0) throw new Error('compile_main(ok href text doc) main.xdv expected non-empty bytes');
  const hrefMovement = countMovementOpsInTextPages(hrefTextXdvBytes, 'compile_main(ok href text doc)');
  if (hrefMovement.down3 !== 0) throw new Error(`compile_main(ok href text doc) expected down3=0, got ${hrefMovement.down3}`);
  if (hrefMovement.right3PositiveTotal !== 196608) {
    throw new Error(`compile_main(ok href text doc) expected right3PositiveTotal=196608, got ${hrefMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK url text doc case failed');
  const urlTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\url{abc}\\end{document}',
  );
  if (addMountedFile('main.tex', urlTextDocBytes, 'ok_url_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok url text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK url text doc case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok url text doc)');
  const urlTextLogBytes = readCompileLogBytes();
  if (urlTextLogBytes.length !== 0) {
    throw new Error(`compile_main(ok url text doc) expected empty log, got ${urlTextLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(urlTextLogBytes, {}, 'compile_main(ok url text doc)');
  const urlTextXdvBytes = readMainXdvArtifactBytes('compile_main(ok url text doc)');
  if (urlTextXdvBytes.length === 0) throw new Error('compile_main(ok url text doc) main.xdv expected non-empty bytes');
  const urlMovement = countMovementOpsInTextPages(urlTextXdvBytes, 'compile_main(ok url text doc)');
  if (urlMovement.right3PositiveTotal !== 196608) {
    throw new Error(`compile_main(ok url text doc) expected right3PositiveTotal=196608, got ${urlMovement.right3PositiveTotal}`);
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before href nested-group case failed');
  const hrefNestedGroupDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\href{abc}{{A}B{C}}\\end{document}',
  );
  if (addMountedFile('main.tex', hrefNestedGroupDocBytes, 'ok_href_nested_group_main') !== 0) {
    throw new Error('mount_add_file(ok href nested-group main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for href nested-group case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok href nested-group)');
  const hrefNestedGroupLogBytes = readCompileLogBytes();
  if (hrefNestedGroupLogBytes.length !== 0) {
    throw new Error(
      `compile_main(ok href nested-group) expected empty log, got ${hrefNestedGroupLogBytes.length} bytes`,
    );
  }
  assertEventsMatchLogAndStats(hrefNestedGroupLogBytes, {}, 'compile_main(ok href nested-group)');
  const hrefNestedGroupXdvBytes = readMainXdvArtifactBytes('compile_main(ok href nested-group)');
  if (hrefNestedGroupXdvBytes.length === 0) {
    throw new Error('compile_main(ok href nested-group) main.xdv expected non-empty bytes');
  }
  const hrefNestedGroupMovement = countMovementOpsInTextPages(
    hrefNestedGroupXdvBytes,
    'compile_main(ok href nested-group)',
  );
  if (hrefNestedGroupMovement.right3PositiveTotal !== 196608) {
    throw new Error(
      `compile_main(ok href nested-group) expected right3PositiveTotal=196608, got ${hrefNestedGroupMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before href missing-arg invalid case failed');
  const hrefMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\href{abc}XYZ\\end{document}',
  );
  if (addMountedFile('main.tex', hrefMissingArgDocBytes, 'ok_href_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(ok href missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for href missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok href missing-arg)');
}
