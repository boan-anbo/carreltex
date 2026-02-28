export function runTokenizerTextwordLeaf138Cases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
  } = helpers;

  const compileAndAssertDeltaPlus4 = (label, controlWord) => {
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
    if (stats.char_count !== baselineStats.char_count + 4) {
      throw new Error(`compile_main(${label}) char_count delta expected +4, got baseline=${baselineStats.char_count}, current=${stats.char_count}`);
    }
    readMainXdvArtifactBytes(`compile_main(${label})`);
  };

  compileAndAssertDeltaPlus4('tokenizer control-word-textbrokenbar', 'textbrokenbar');
  compileAndAssertDeltaPlus4('tokenizer control-word-textcurrency', 'textcurrency');
  compileAndAssertDeltaPlus4('tokenizer control-word-textexclamdown', 'textexclamdown');
  compileAndAssertDeltaPlus4('tokenizer control-word-textquestiondown', 'textquestiondown');
  compileAndAssertDeltaPlus4('tokenizer control-word-textguillemotleft', 'textguillemotleft');
  compileAndAssertDeltaPlus4('tokenizer control-word-textguillemotright', 'textguillemotright');
  compileAndAssertDeltaPlus4('tokenizer control-word-textquoteleft', 'textquoteleft');
  compileAndAssertDeltaPlus4('tokenizer control-word-textquoteright', 'textquoteright');
  compileAndAssertDeltaPlus4('tokenizer control-word-textquotedblbase', 'textquotedblbase');
  compileAndAssertDeltaPlus4('tokenizer control-word-textquotesinglbase', 'textquotesinglbase');
}
