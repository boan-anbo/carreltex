export function runTokenizerTextwordLeaf144Cases(ctx, helpers) {
  const {
    addMountedFile,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    assertMainXdvArtifactEmpty,
  } = helpers;

  const compileAndAssertDelta = (label, controlWord, expectedDelta) => {
    if (ctx.mountReset() !== 0) {
      throw new Error(`mount_reset before ${label} baseline case failed`);
    }
    const baselineMain = new TextEncoder().encode('\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n');
    if (addMountedFile('main.tex', baselineMain, `${label}_baseline_main`) !== 0) {
      throw new Error(`mount_add_file(${label} baseline main.tex) failed`);
    }
    if (ctx.mountFinalize() !== 0) {
      throw new Error(`mount_finalize for ${label} baseline case failed`);
    }
    expectNotImplemented(ctx.compileMain(), `compile_main_v0(${label} baseline)`);
    const baselineStats = assertEventsMatchLogAndStats(readCompileLogBytes(), {}, `compile_main(${label} baseline)`);
    if (ctx.mountReset() !== 0) {
      throw new Error(`mount_reset before ${label} case failed`);
    }
    const mainBytes = new TextEncoder().encode(`\\documentclass{article}\n\\begin{document}\nHello.\\${controlWord} XYZ\n\\end{document}\n`);
    if (addMountedFile('main.tex', mainBytes, `${label}_main`) !== 0) {
      throw new Error(`mount_add_file(${label} main.tex) failed`);
    }
    if (ctx.mountFinalize() !== 0) {
      throw new Error(`mount_finalize for ${label} case failed`);
    }
    expectNotImplemented(ctx.compileMain(), `compile_main_v0(${label})`);
    const stats = assertEventsMatchLogAndStats(readCompileLogBytes(), {}, `compile_main(${label})`);
    if (stats.char_count !== baselineStats.char_count + expectedDelta) {
      throw new Error(`compile_main(${label}) char_count delta expected +${expectedDelta}, got baseline=${baselineStats.char_count}, current=${stats.char_count}`);
    }
    assertMainXdvArtifactEmpty(`compile_main(${label})`);
  };

  compileAndAssertDelta('tokenizer control-word-textfractionsolidus', 'textfractionsolidus', 4);
  compileAndAssertDelta('tokenizer control-word-textasterisklow', 'textasterisklow', 4);
  compileAndAssertDelta('tokenizer control-word-textdoublepipe', 'textdoublepipe', 5);
  compileAndAssertDelta('tokenizer control-word-textasciicomma', 'textasciicomma', 4);
  compileAndAssertDelta('tokenizer control-word-textasciiperiod', 'textasciiperiod', 4);
  compileAndAssertDelta('tokenizer control-word-textasciicolon', 'textasciicolon', 4);
  compileAndAssertDelta('tokenizer control-word-textasciiplus', 'textasciiplus', 4);
  compileAndAssertDelta('tokenizer control-word-textasciiminus', 'textasciiminus', 4);
  compileAndAssertDelta('tokenizer control-word-textasciiequal', 'textasciiequal', 4);
  compileAndAssertDelta('tokenizer control-word-textasciislash', 'textasciislash', 4);
}
