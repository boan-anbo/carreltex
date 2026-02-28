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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK text doc case failed');
  }

  const strictTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nXYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', strictTextDocBytes, 'ok_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok text doc)');
  const textReport = readCompileReportJson();
  if (textReport.status !== 'OK') {
    throw new Error(`compile_main(ok text doc) report.status expected OK, got ${textReport.status}`);
  }
  if (!Array.isArray(textReport.missing_components) || textReport.missing_components.length !== 0) {
    throw new Error('compile_main(ok text doc) missing_components expected empty array');
  }
  const textLogBytes = readCompileLogBytes();
  if (textLogBytes.length !== 0) {
    throw new Error(`compile_main(ok text doc) expected empty log, got ${textLogBytes.length} bytes`);
  }
  const textStats = assertEventsMatchLogAndStats(
    textLogBytes,
    { char_count: stats.char_count + 3 },
    'compile_main(ok text doc)',
  );
  if (!(typeof textStats.token_count === 'number' && textStats.token_count > 0)) {
    throw new Error('compile_main(ok text doc) token_count expected >0');
  }
  const textXdvBytes = readMainXdvArtifactBytes('compile_main(ok text doc)');
  if (textXdvBytes.length === 0) {
    throw new Error('compile_main(ok text doc) main.xdv expected non-empty bytes');
  }
}
