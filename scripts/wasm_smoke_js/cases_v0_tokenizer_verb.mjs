export function runTokenizerVerbCases(ctx, helpers) {
  const {
    addMountedFile,
    expectInvalid,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  } = helpers;

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer verb baseline case failed');
  }
  const baselineMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\end{document}\n');
  if (addMountedFile('main.tex', baselineMainBytes, 'tokenizer_verb_baseline_main') !== 0) {
    throw new Error('mount_add_file(tokenizer verb baseline main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer verb baseline case failed');
  }
  expectOk(ctx.compileMain(), 'compile_main_v0(tokenizer verb baseline)');
  let baselineCharCount = null;
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer verb baseline)');
    baselineCharCount = stats.char_count;
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer verb positive case failed');
  }
  const verbMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\verb|abc|\n\\end{document}\n');
  if (addMountedFile('main.tex', verbMainBytes, 'tokenizer_verb_positive_main') !== 0) {
    throw new Error('mount_add_file(tokenizer verb positive main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer verb positive case failed');
  }
  expectOk(ctx.compileMain(), 'compile_main_v0(tokenizer verb positive)');
  {
    const logBytes = readCompileLogBytes();
    const stats = assertEventsMatchLogAndStats(logBytes, {}, 'compile_main(tokenizer verb positive)');
    if (baselineCharCount === null) {
      throw new Error('baselineCharCount not initialized for tokenizer verb positive case');
    }
    if (stats.char_count !== baselineCharCount + 3) {
      throw new Error(
        `compile_main(tokenizer verb positive) char_count delta expected +3, got baseline=${baselineCharCount}, current=${stats.char_count}`,
      );
    }
    readMainXdvArtifactBytes('compile_main(tokenizer verb positive)');
  }

  if (ctx.mountReset() !== 0) {
    throw new Error('mount_reset before tokenizer verb invalid case failed');
  }
  const invalidVerbMainBytes = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\n\\verb|abc\n\\end{document}\n');
  if (addMountedFile('main.tex', invalidVerbMainBytes, 'tokenizer_verb_invalid_main') !== 0) {
    throw new Error('mount_add_file(tokenizer verb invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) {
    throw new Error('mount_finalize for tokenizer verb invalid case failed');
  }
  expectInvalid(ctx.compileMain(), 'compile_main_v0(tokenizer verb invalid)');
  {
    const logBytes = readCompileLogBytes();
    const logText = new TextDecoder().decode(logBytes);
    if (!logText.startsWith('INVALID_INPUT:') || !logText.endsWith('tokenizer_verb_not_supported')) {
      throw new Error(`compile_main(tokenizer verb invalid) log mismatch: ${logText}`);
    }
    assertMainXdvArtifactEmpty('compile_main(tokenizer verb invalid)');
    assertNoEvents('compile_main(tokenizer verb invalid)');
  }
}
