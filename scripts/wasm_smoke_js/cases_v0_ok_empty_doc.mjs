export function runOkEmptyDocCases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    readCompileReportJson,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK empty doc case failed');
  }

  const strictEmptyDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\end{document}\n');
  if (addMountedFile('main.tex', strictEmptyDocBytes, 'ok_empty_doc_main') !== 0) {
    throw new Error('mount_add_file(ok empty doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK empty doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok empty doc)');
  const report = readCompileReportJson();
  if (report.status !== 'OK') {
    throw new Error(`compile_main(ok empty doc) report.status expected OK, got ${report.status}`);
  }
  if (!Array.isArray(report.missing_components) || report.missing_components.length !== 0) {
    throw new Error('compile_main(ok empty doc) missing_components expected empty array');
  }

  const logBytes = readCompileLogBytes();
  if (logBytes.length !== 0) {
    throw new Error(`compile_main(ok empty doc) expected empty log, got ${logBytes.length} bytes`);
  }
  const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(ok empty doc)');
  if (!(typeof stats.token_count === 'number' && stats.token_count > 0)) {
    throw new Error('compile_main(ok empty doc) token_count expected >0');
  }

  const xdvBytes = readMainXdvArtifactBytes('compile_main(ok empty doc)');
  if (xdvBytes[0] !== 247) {
    throw new Error(`compile_main(ok empty doc) main.xdv first byte expected 247, got ${xdvBytes[0]}`);
  }
  let trailerCount = 0;
  for (let index = xdvBytes.length - 1; index >= 0 && xdvBytes[index] === 223; index -= 1) {
    trailerCount += 1;
  }
  if (trailerCount < 4) {
    throw new Error(`compile_main(ok empty doc) trailer byte count expected >=4, got ${trailerCount}`);
  }
}
