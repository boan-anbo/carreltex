export function runTokenizerTextwordLeaf145Cases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
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
    expectOk(ctx.compileMain(), `compile_main_v0(${label} baseline)`);
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
    expectOk(ctx.compileMain(), `compile_main_v0(${label})`);
    const stats = assertEventsMatchLogAndStats(readCompileLogBytes(), {}, `compile_main(${label})`);
    if (stats.char_count !== baselineStats.char_count + expectedDelta) {
      throw new Error(`compile_main(${label}) char_count delta expected +${expectedDelta}, got baseline=${baselineStats.char_count}, current=${stats.char_count}`);
    }
    readMainXdvArtifactBytes(`compile_main(${label})`);
  };

  compileAndAssertDelta('tokenizer control-word-textmu', 'textmu', 4);
  compileAndAssertDelta('tokenizer control-word-textohm', 'textohm', 4);
  compileAndAssertDelta('tokenizer control-word-textmho', 'textmho', 4);
  compileAndAssertDelta('tokenizer control-word-textcelsius', 'textcelsius', 4);
  compileAndAssertDelta('tokenizer control-word-textnaira', 'textnaira', 4);
  compileAndAssertDelta('tokenizer control-word-textpeso', 'textpeso', 4);
  compileAndAssertDelta('tokenizer control-word-textwon', 'textwon', 4);
  compileAndAssertDelta('tokenizer control-word-textrupee', 'textrupee', 4);
  compileAndAssertDelta('tokenizer control-word-textbaht', 'textbaht', 4);
  compileAndAssertDelta('tokenizer control-word-textflorin', 'textflorin', 4);
  compileAndAssertDelta('tokenizer control-word-textcolonmonetary', 'textcolonmonetary', 4);
  compileAndAssertDelta('tokenizer control-word-textdong', 'textdong', 4);
  compileAndAssertDelta('tokenizer control-word-textlira', 'textlira', 4);
  compileAndAssertDelta('tokenizer control-word-textestimated', 'textestimated', 4);
  compileAndAssertDelta('tokenizer control-word-textrecipe', 'textrecipe', 4);
  compileAndAssertDelta('tokenizer control-word-textservicemark', 'textservicemark', 5);
  compileAndAssertDelta('tokenizer control-word-textcopyleft', 'textcopyleft', 5);
  compileAndAssertDelta('tokenizer control-word-textinterrobang', 'textinterrobang', 5);
}
