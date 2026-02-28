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
    throw new Error('mount_reset before tokenizer control-symbol-bang-noop case failed');
  }
  const bangMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\!XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', bangMainBytes, 'tokenizer_control_symbol_bang_noop_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-bang-noop main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-bang-noop case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-bang-noop)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-bang-noop)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-bang-noop case');
    }
    if (stats.char_count !== helloBaselineCharCount + 3) {
      throw new Error(`compile_main(tokenizer control-symbol-bang-noop) char_count delta expected +3, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-bang-noop)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-semicolon case failed');
  }
  const semicolonMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\;XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', semicolonMainBytes, 'tokenizer_control_symbol_semicolon_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-semicolon main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-semicolon case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-semicolon)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-semicolon)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-semicolon case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-semicolon) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-semicolon)');
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
    throw new Error('mount_reset before tokenizer control-symbol-underscore case failed');
  }
  const underscoreMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\_XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', underscoreMainBytes, 'tokenizer_control_symbol_underscore_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-underscore main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-underscore case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-underscore)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-underscore)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-underscore case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-underscore) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-underscore)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-hash case failed');
  }
  const hashMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\#XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', hashMainBytes, 'tokenizer_control_symbol_hash_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-hash main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-hash case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-hash)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-hash)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-hash case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-hash) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-hash)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-dollar case failed');
  }
  const dollarMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\$XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', dollarMainBytes, 'tokenizer_control_symbol_dollar_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-dollar main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-dollar case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-dollar)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-dollar)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-dollar case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-dollar) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-dollar)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-ampersand case failed');
  }
  const ampersandMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\&XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', ampersandMainBytes, 'tokenizer_control_symbol_ampersand_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-ampersand main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-ampersand case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-ampersand)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-ampersand)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-ampersand case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-ampersand) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-ampersand)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-lbrace case failed');
  }
  const lbraceMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\{XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', lbraceMainBytes, 'tokenizer_control_symbol_lbrace_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-lbrace main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-lbrace case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-lbrace)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-lbrace)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-lbrace case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-lbrace) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-lbrace)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-symbol-rbrace case failed');
  }
  const rbraceMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\}XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', rbraceMainBytes, 'tokenizer_control_symbol_rbrace_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-symbol-rbrace main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-symbol-rbrace case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-symbol-rbrace)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-symbol-rbrace)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-symbol-rbrace case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-symbol-rbrace) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-symbol-rbrace)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-word-textbackslash case failed');
  }
  const textbackslashMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\textbackslash XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', textbackslashMainBytes, 'tokenizer_control_word_textbackslash_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-word-textbackslash main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-word-textbackslash case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-word-textbackslash)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-word-textbackslash)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-word-textbackslash case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-word-textbackslash) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-word-textbackslash)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-word-textasciitilde case failed');
  }
  const textasciitildeMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\textasciitilde XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', textasciitildeMainBytes, 'tokenizer_control_word_textasciitilde_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-word-textasciitilde main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-word-textasciitilde case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-word-textasciitilde)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-word-textasciitilde)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-word-textasciitilde case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-word-textasciitilde) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-word-textasciitilde)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-word-textasciicircum case failed');
  }
  const textasciicircumMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\textasciicircum XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', textasciicircumMainBytes, 'tokenizer_control_word_textasciicircum_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-word-textasciicircum main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-word-textasciicircum case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-word-textasciicircum)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-word-textasciicircum)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-word-textasciicircum case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-word-textasciicircum) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-word-textasciicircum)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-word-textquotedbl case failed');
  }
  const textquotedblMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\textquotedbl XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', textquotedblMainBytes, 'tokenizer_control_word_textquotedbl_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-word-textquotedbl main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-word-textquotedbl case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-word-textquotedbl)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-word-textquotedbl)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-word-textquotedbl case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer control-word-textquotedbl) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-word-textquotedbl)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer control-word-par case failed');
  }
  const parMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\par XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', parMainBytes, 'tokenizer_control_word_par_main') !== 0) {
    throw new Error('mount_add_file(tokenizer control-word-par main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer control-word-par case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer control-word-par)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer control-word-par)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer control-word-par case');
    }
    if (stats.char_count !== helloBaselineCharCount + 3) {
      throw new Error(`compile_main(tokenizer control-word-par) char_count delta expected +3, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer control-word-par)');
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

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer braced-accent-passthrough case failed');
  }
  const bracedAccentMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\~{a}XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', bracedAccentMainBytes, 'tokenizer_braced_accent_passthrough_main') !== 0) {
    throw new Error('mount_add_file(tokenizer braced-accent-passthrough main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer braced-accent-passthrough case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer braced-accent-passthrough)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer braced-accent-passthrough)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer braced-accent-passthrough case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer braced-accent-passthrough) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer braced-accent-passthrough)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer braced-accent control-symbol payload case failed');
  }
  const bracedAccentControlSymbolMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\\~{\\%}XYZ\n\\end{document}\n');
  if (addMountedFile('main.tex', bracedAccentControlSymbolMainBytes, 'tokenizer_braced_accent_control_symbol_main') !== 0) {
    throw new Error('mount_add_file(tokenizer braced-accent control-symbol payload main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer braced-accent control-symbol payload case failed');
  }
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(tokenizer braced-accent control-symbol payload)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer braced-accent control-symbol payload)');
    if (helloBaselineCharCount === null) {
      throw new Error('helloBaselineCharCount not initialized for tokenizer braced-accent control-symbol payload case');
    }
    if (stats.char_count !== helloBaselineCharCount + 4) {
      throw new Error(`compile_main(tokenizer braced-accent control-symbol payload) char_count delta expected +4, got baseline=${helloBaselineCharCount}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer braced-accent control-symbol payload)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer accent-not-supported case failed');
  }
  if (addMountedFile('main.tex', new TextEncoder().encode('\\~a'), 'tokenizer_accent_not_supported_main') !== 0) {
    throw new Error('mount_add_file(tokenizer accent-not-supported main.tex) failed');
  }
  const accentFinalizeCode = ctx.mountFinalize();
  if (accentFinalizeCode !== 0 && accentFinalizeCode !== 1) {
    throw new Error(`mount_finalize(tokenizer accent-not-supported) unexpected code=${accentFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(tokenizer accent-not-supported)');
  {
    const logText = new TextDecoder().decode(readCompileLogBytes());
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenizer_accent_not_supported')) {
      throw new Error(`compile_main tokenizer accent-not-supported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(tokenizer accent-not-supported)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer accent-control-word-not-supported case failed');
  }
  if (addMountedFile('main.tex', new TextEncoder().encode('\\~{\\par}'), 'tokenizer_accent_control_word_not_supported_main') !== 0) {
    throw new Error('mount_add_file(tokenizer accent-control-word-not-supported main.tex) failed');
  }
  const accentControlWordFinalizeCode = ctx.mountFinalize();
  if (accentControlWordFinalizeCode !== 0 && accentControlWordFinalizeCode !== 1) {
    throw new Error(`mount_finalize(tokenizer accent-control-word-not-supported) unexpected code=${accentControlWordFinalizeCode}`);
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(tokenizer accent-control-word-not-supported)');
  {
    const logText = new TextDecoder().decode(readCompileLogBytes());
    if (!logText.startsWith('INVALID_INPUT:') || !logText.includes('tokenizer_accent_not_supported')) {
      throw new Error(`compile_main tokenizer accent-control-word-not-supported log mismatch: ${logText}`);
    }
    assertNoEvents('compile_main_v0(tokenizer accent-control-word-not-supported)');
  }
}
