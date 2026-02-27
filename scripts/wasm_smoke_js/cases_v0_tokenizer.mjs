export function runTokenizerCases(ctx, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'tokenizer_baseline_main') !== 0) {
    throw new Error('mount_add_file(tokenizer baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer baseline)');
  let baselineCharCount = null;
  let helloBaselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(tokenizer baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer hello baseline case failed');
  }
  const helloBaselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n');
  if (addMountedFile('main.tex', helloBaselineMainBytes, 'tokenizer_hello_baseline_main') !== 0) {
    throw new Error('mount_add_file(tokenizer hello baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer hello baseline case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer hello baseline)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer hello baseline)');
    helloBaselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(tokenizer hello baseline)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-comma case failed');
  }
  const commaMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\,XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', commaMainBytes, 'tokenizer_control_symbol_comma_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-comma main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-comma case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-comma)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-comma)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-comma case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-comma) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-comma)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer caret-hex decode case failed');
  }
  const decodeMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nA^^4AB\n\\end{document}\n');
  if (addMountedFile('main.tex', decodeMainBytes, 'tokenizer_caret_hex_main') !== 0) {
    throw new Error('mount_add_file(tokenizer caret-hex decode main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer caret-hex decode case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer caret-hex decode)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer caret-hex decode)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for tokenizer caret-hex decode case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(tokenizer caret-hex decode) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer caret-hex decode)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-percent case failed');
  }
  const percentMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\%XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', percentMainBytes, 'tokenizer_control_symbol_percent_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-percent main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-percent case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-percent)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-percent)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-percent case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-percent) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-percent)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer CRLF normalization case failed');
  }
  const crlfMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nA\r\nB\n\\end{document}\n');
  if (addMountedFile('main.tex', crlfMainBytes, 'tokenizer_crlf_normalization_main') !== 0) {
    throw new Error('mount_add_file(tokenizer CRLF normalization main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer CRLF normalization case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer CRLF normalization)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer CRLF normalization)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for tokenizer CRLF normalization case');
    }
    if (stats.char_count !== baselineCharCount + 2) {
      throw new Error(`compile_main(tokenizer CRLF normalization) char_count delta expected +2, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer CRLF normalization)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer lone-CR normalization case failed');
  }
  const loneCrMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nA\rB\n\\end{document}\n');
  if (addMountedFile('main.tex', loneCrMainBytes, 'tokenizer_lone_cr_normalization_main') !== 0) {
    throw new Error('mount_add_file(tokenizer lone-CR normalization main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer lone-CR normalization case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer lone-CR normalization)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer lone-CR normalization)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for tokenizer lone-CR normalization case');
    }
    if (stats.char_count !== baselineCharCount + 2) {
      throw new Error(`compile_main(tokenizer lone-CR normalization) char_count delta expected +2, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer lone-CR normalization)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer caret-in-comment case failed');
  }
  const commentMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n% ^^ZZ\nXYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', commentMainBytes, 'tokenizer_caret_in_comment_main') !== 0) {
    throw new Error('mount_add_file(tokenizer caret-in-comment main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer caret-in-comment case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer caret-in-comment)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer caret-in-comment)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for tokenizer caret-in-comment case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(`compile_main(tokenizer caret-in-comment) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer caret-in-comment)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer unsupported-caret case failed');
  }
  if (addMountedFile('main.tex', new TextEncoder().encode('A^^ZZB'), 'tokenizer_caret_unsupported_main') !== 0) {
    throw new Error('mount_add_file(tokenizer unsupported-caret main.tex) failed');
  }
  const finalizeCode = ctx.mountFinalize();
  if (finalizeCode !== 0 && finalizeCode !== 1) {
    throw new Error(`mount_finalize(tokenizer unsupported-caret) unexpected code=${finalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(tokenizer unsupported-caret)');
  {
    const logText = new TextDecoder().decode(readCompileLogBytes());
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenizer_caret_not_supported')) {
      throw new Error(`compile_main tokenizer unsupported-caret log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(tokenizer unsupported-caret)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer non-ascii control-seq case failed');
  }
  if (addMountedFile('main.tex', new TextEncoder().encode('\\def\\^^ff{XYZ}'), 'tokenizer_non_ascii_controlseq_main') !== 0) {
    throw new Error('mount_add_file(tokenizer non-ascii control-seq main.tex) failed');
  }
  const nonAsciiFinalizeCode = ctx.mountFinalize();
  if (nonAsciiFinalizeCode !== 0 && nonAsciiFinalizeCode !== 1) {
    throw new Error(`mount_finalize(tokenizer non-ascii control-seq) unexpected code=${nonAsciiFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(tokenizer non-ascii control-seq)');
  {
    const logText = new TextDecoder().decode(readCompileLogBytes());
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenizer_control_seq_non_ascii')) {
      throw new Error(`compile_main tokenizer non-ascii control-seq log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(tokenizer non-ascii control-seq)');
  }
}
