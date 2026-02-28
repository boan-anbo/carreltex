export function runOkEmptyDocCases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    readCompileReportJson,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
  } = helpers;

  const countPagesInDviV2 = (bytes, label) => {
    let pageCount = 0;
    for (const byte of bytes) {
      if (byte === 139) {
        pageCount += 1;
      }
    }
    if (pageCount <= 0) {
      throw new Error(`${label} expected at least one BOP opcode`);
    }
    return pageCount;
  };

  const countMovementOpsInTextPages = (bytes, label) => {
    const DVI_PRE = 247;
    const DVI_BOP = 139;
    const DVI_EOP = 140;
    const DVI_POST = 248;
    const DVI_FNT_DEF1 = 243;
    const DVI_FNT_NUM_0 = 171;
    const DVI_RIGHT3 = 145;
    const DVI_W0 = 147;
    const DVI_W3 = 150;
    const DVI_DOWN3 = 160;
    let index = 0;
    if (bytes[index++] !== DVI_PRE) {
      throw new Error(`${label} expected DVI preamble`);
    }
    index += 14;
    let right3 = 0;
    let w3 = 0;
    let w0 = 0;
    let down3 = 0;
    while (index < bytes.length) {
      const opcode = bytes[index];
      if (opcode === DVI_POST) {
        break;
      }
      if (opcode !== DVI_BOP) {
        throw new Error(`${label} expected BOP opcode before page stream`);
      }
      index += 1 + 44;
      if (bytes[index] !== DVI_FNT_DEF1) {
        throw new Error(`${label} expected font definition`);
      }
      const areaLen = bytes[index + 14];
      const nameLen = bytes[index + 15];
      index += 16 + areaLen + nameLen;
      if (bytes[index] !== DVI_FNT_NUM_0) {
        throw new Error(`${label} expected font select`);
      }
      index += 1;
      while (index < bytes.length && bytes[index] !== DVI_EOP) {
        if (bytes[index] === DVI_RIGHT3) {
          right3 += 1;
          index += 4;
        } else if (bytes[index] === DVI_W3) {
          w3 += 1;
          index += 4;
        } else if (bytes[index] === DVI_W0) {
          w0 += 1;
          index += 1;
        } else if (bytes[index] === DVI_DOWN3) {
          down3 += 1;
          index += 4;
        } else {
          index += 1;
        }
      }
      if (bytes[index] !== DVI_EOP) {
        throw new Error(`${label} expected EOP opcode`);
      }
      index += 1;
    }
    return { right3, w3, w0, down3 };
  };

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
  const textMovement = countMovementOpsInTextPages(textXdvBytes, 'compile_main(ok text doc)');
  if (textMovement.right3 < 1) {
    throw new Error('compile_main(ok text doc) expected at least one RIGHT3 opcode');
  }
  if (textMovement.w3 < 1) {
    throw new Error('compile_main(ok text doc) expected at least one W3 opcode');
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
  const printableMovement = countMovementOpsInTextPages(
    printableXdvBytes,
    'compile_main(ok printable text doc)',
  );
  if (printableMovement.w0 <= 0) {
    throw new Error('compile_main(ok printable text doc) expected at least one W0 opcode');
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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK pagebreak text doc case failed');
  }

  const pagebreakTextDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\n\\begin{document}\nAB\\pagebreak CD\n\\end{document}\n',
  );
  if (addMountedFile('main.tex', pagebreakTextDocBytes, 'ok_pagebreak_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok pagebreak text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK pagebreak text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok pagebreak text doc)');
  const pagebreakReport = readCompileReportJson();
  if (pagebreakReport.status !== 'OK') {
    throw new Error(`compile_main(ok pagebreak text doc) report.status expected OK, got ${pagebreakReport.status}`);
  }
  const pagebreakLogBytes = readCompileLogBytes();
  if (pagebreakLogBytes.length !== 0) {
    throw new Error(`compile_main(ok pagebreak text doc) expected empty log, got ${pagebreakLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(pagebreakLogBytes, {}, 'compile_main(ok pagebreak text doc)');
  const pagebreakXdvBytes = readMainXdvArtifactBytes('compile_main(ok pagebreak text doc)');
  if (pagebreakXdvBytes.length === 0) {
    throw new Error('compile_main(ok pagebreak text doc) main.xdv expected non-empty bytes');
  }
  const pageCount = countPagesInDviV2(pagebreakXdvBytes, 'compile_main(ok pagebreak text doc)');
  if (pageCount !== 2) {
    throw new Error(`compile_main(ok pagebreak text doc) expected 2 pages, got ${pageCount}`);
  }
  const pagebreakMovement = countMovementOpsInTextPages(
    pagebreakXdvBytes,
    'compile_main(ok pagebreak text doc)',
  );
  if (pagebreakMovement.right3 < 1) {
    throw new Error('compile_main(ok pagebreak text doc) expected at least one RIGHT3 opcode');
  }

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
  const newlineReport = readCompileReportJson();
  if (newlineReport.status !== 'OK') {
    throw new Error(`compile_main(ok newline text doc) report.status expected OK, got ${newlineReport.status}`);
  }
  const newlineLogBytes = readCompileLogBytes();
  if (newlineLogBytes.length !== 0) {
    throw new Error(`compile_main(ok newline text doc) expected empty log, got ${newlineLogBytes.length} bytes`);
  }
  const newlineStats = assertEventsMatchLogAndStats(
    newlineLogBytes,
    { char_count: stats.char_count + 3 },
    'compile_main(ok newline text doc)',
  );
  if (!(typeof newlineStats.token_count === 'number' && newlineStats.token_count > 0)) {
    throw new Error('compile_main(ok newline text doc) token_count expected >0');
  }
  const newlineXdvBytes = readMainXdvArtifactBytes('compile_main(ok newline text doc)');
  if (newlineXdvBytes.length === 0) {
    throw new Error('compile_main(ok newline text doc) main.xdv expected non-empty bytes');
  }
  const newlinePages = countPagesInDviV2(newlineXdvBytes, 'compile_main(ok newline text doc)');
  if (newlinePages !== 1) {
    throw new Error(`compile_main(ok newline text doc) expected 1 page, got ${newlinePages}`);
  }
  const newlineMovement = countMovementOpsInTextPages(newlineXdvBytes, 'compile_main(ok newline text doc)');
  if (newlineMovement.down3 !== 1) {
    throw new Error(`compile_main(ok newline text doc) expected exactly one DOWN3 opcode, got ${newlineMovement.down3}`);
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before OK wrapped text doc case failed');
  }

  const wrappedTextBody = 'A'.repeat(81);
  const wrappedTextDocBytes = new TextEncoder().encode(
    `\\documentclass{article}\\begin{document}${wrappedTextBody}\\end{document}`,
  );
  if (addMountedFile('main.tex', wrappedTextDocBytes, 'ok_wrapped_text_doc_main') !== 0) {
    throw new Error('mount_add_file(ok wrapped text doc main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for OK wrapped text doc case failed');
  }

  expectOk(ctx.compileMain(), 'compile_main_v0(ok wrapped text doc)');
  const wrappedReport = readCompileReportJson();
  if (wrappedReport.status !== 'OK') {
    throw new Error(`compile_main(ok wrapped text doc) report.status expected OK, got ${wrappedReport.status}`);
  }
  const wrappedLogBytes = readCompileLogBytes();
  if (wrappedLogBytes.length !== 0) {
    throw new Error(`compile_main(ok wrapped text doc) expected empty log, got ${wrappedLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    wrappedLogBytes,
    { char_count: stats.char_count + 81 },
    'compile_main(ok wrapped text doc)',
  );
  const wrappedXdvBytes = readMainXdvArtifactBytes('compile_main(ok wrapped text doc)');
  if (wrappedXdvBytes.length === 0) {
    throw new Error('compile_main(ok wrapped text doc) main.xdv expected non-empty bytes');
  }
  const wrappedPages = countPagesInDviV2(wrappedXdvBytes, 'compile_main(ok wrapped text doc)');
  if (wrappedPages !== 1) {
    throw new Error(`compile_main(ok wrapped text doc) expected 1 page, got ${wrappedPages}`);
  }
  const wrappedMovement = countMovementOpsInTextPages(wrappedXdvBytes, 'compile_main(ok wrapped text doc)');
  if (wrappedMovement.down3 < 1) {
    throw new Error('compile_main(ok wrapped text doc) expected at least one DOWN3 opcode from auto-wrap');
  }
}
