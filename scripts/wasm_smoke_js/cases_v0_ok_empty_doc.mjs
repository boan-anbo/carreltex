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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK printable text doc case failed');
  }

  const printableTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello, world! 123\n\\end{document}\n');
  if (addMountedFile('main.tex', printableTextDocBytes, 'ok_printable_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok printable text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK printable text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok printable text doc)');
  const printableReport = readCompileReportJson();
  if (printableReport.status !== 'OK') {
    throw new Error(`compile_main(ok printable text doc) report.status expected OK, got ${printableReport.status}`);
  }
  if (!Array.isArray(printableReport.missing_components) || printableReport.missing_components.length !== 0) {
    throw new Error('compile_main(ok printable text doc) missing_components expected empty array');
  }
  const printableLogBytes = readCompileLogBytes();
  if (printableLogBytes.length !== 0) {
    throw new Error(`compile_main(ok printable text doc) expected empty log, got ${printableLogBytes.length} bytes`);
  }
  const printableStats = assertEventsMatchLogAndStats(
    printableLogBytes,
    { char_count: stats.char_count + 15 },
    'compile_main(ok printable text doc)',
  );
  if (!(typeof printableStats.token_count === 'number' && printableStats.token_count > 0)) {
    throw new Error('compile_main(ok printable text doc) token_count expected >0');
  }
  const printableXdvBytes = readMainXdvArtifactBytes('compile_main(ok printable text doc)');
  if (printableXdvBytes.length === 0) {
    throw new Error('compile_main(ok printable text doc) main.xdv expected non-empty bytes');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK whitespace text doc case failed');
  }

  const whitespaceTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nA  \n\nB\n\\end{document}\n');
  if (addMountedFile('main.tex', whitespaceTextDocBytes, 'ok_whitespace_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok whitespace text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK whitespace text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok whitespace text doc)');
  const whitespaceReport = readCompileReportJson();
  if (whitespaceReport.status !== 'OK') {
    throw new Error(`compile_main(ok whitespace text doc) report.status expected OK, got ${whitespaceReport.status}`);
  }
  const whitespaceLogBytes = readCompileLogBytes();
  if (whitespaceLogBytes.length !== 0) {
    throw new Error(`compile_main(ok whitespace text doc) expected empty log, got ${whitespaceLogBytes.length} bytes`);
  }
  const whitespaceStats = assertEventsMatchLogAndStats(
    whitespaceLogBytes,
    { char_count: stats.char_count + 2 },
    'compile_main(ok whitespace text doc)',
  );
  if (!(typeof whitespaceStats.token_count === 'number' && whitespaceStats.token_count > 0)) {
    throw new Error('compile_main(ok whitespace text doc) token_count expected >0');
  }
  const whitespaceXdvBytes = readMainXdvArtifactBytes('compile_main(ok whitespace text doc)');
  if (whitespaceXdvBytes.length === 0) {
    throw new Error('compile_main(ok whitespace text doc) main.xdv expected non-empty bytes');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK tilde text doc case failed');
  }

  const tildeTextDocBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n~\n\\end{document}\n');
  if (addMountedFile('main.tex', tildeTextDocBytes, 'ok_tilde_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok tilde text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK tilde text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok tilde text doc)');
  const tildeReport = readCompileReportJson();
  if (tildeReport.status !== 'OK') {
    throw new Error(`compile_main(ok tilde text doc) report.status expected OK, got ${tildeReport.status}`);
  }
  const tildeLogBytes = readCompileLogBytes();
  if (tildeLogBytes.length !== 0) {
    throw new Error(`compile_main(ok tilde text doc) expected empty log, got ${tildeLogBytes.length} bytes`);
  }
  const tildeStats = assertEventsMatchLogAndStats(
    tildeLogBytes,
    { char_count: stats.char_count + 1 },
    'compile_main(ok tilde text doc)',
  );
  if (!(typeof tildeStats.token_count === 'number' && tildeStats.token_count > 0)) {
    throw new Error('compile_main(ok tilde text doc) token_count expected >0');
  }
  const tildeXdvBytes = readMainXdvArtifactBytes('compile_main(ok tilde text doc)');
  if (tildeXdvBytes.length === 0) {
    throw new Error('compile_main(ok tilde text doc) main.xdv expected non-empty bytes');
  }
}
