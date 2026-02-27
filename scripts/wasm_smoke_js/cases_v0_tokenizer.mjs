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
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer baseline)');
    baselineCharCount = stats.char_count;
    assertMainXdvArtifactEmpty('compile_main(tokenizer baseline)');
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
}
