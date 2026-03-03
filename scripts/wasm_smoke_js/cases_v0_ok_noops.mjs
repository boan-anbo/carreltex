export function runOkNoopCases(ctx, helpers) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
  } = helpers;

  const countMovementOpsInTextPages = (bytes, label) => {
    const DVI_PRE = 247;
    const DVI_BOP = 139;
    const DVI_EOP = 140;
    const DVI_POST = 248;
    const DVI_FNT_DEF1 = 243;
    const DVI_FNT_NUM_0 = 171;
    const DVI_RIGHT3 = 145;
    let index = 0;
    if (bytes[index++] !== DVI_PRE) {
      throw new Error(`${label} expected DVI preamble`);
    }
    index += 14;
    let right3PositiveTotal = 0;
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
          const amount =
            ((bytes[index + 1] << 16) | (bytes[index + 2] << 8) | bytes[index + 3]) << 8 >> 8;
          if (amount > 0) {
            right3PositiveTotal += amount;
          }
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
    return { right3PositiveTotal };
  };

  const runNoopCase = (commandSource, tag) => {
    if (ctx.mountReset() !== 0) throw new Error(`mount_reset before ${tag} baseline failed`);
    const baselineDocBytes = new TextEncoder().encode(
      `\\documentclass{article}\\begin{document}${commandSource}\\end{document}`,
    );
    if (addMountedFile('main.tex', baselineDocBytes, `${tag}_baseline_main`) !== 0) {
      throw new Error(`mount_add_file(${tag} baseline main.tex) failed`);
    }
    if (ctx.mountFinalize() !== 0) throw new Error(`mount_finalize for ${tag} baseline failed`);
    expectOk(ctx.compileMain(), `compile_main_v0(${tag} baseline)`);
    const baselineLogBytes = readCompileLogBytes();
    if (baselineLogBytes.length !== 0) {
      throw new Error(`compile_main(${tag} baseline) expected empty log, got ${baselineLogBytes.length} bytes`);
    }
    const baselineStats = assertEventsMatchLogAndStats(
      baselineLogBytes,
      {},
      `compile_main(${tag} baseline)`,
    );
    readMainXdvArtifactBytes(`compile_main(${tag} baseline)`);

    if (ctx.mountReset() !== 0) throw new Error(`mount_reset before ${tag} case failed`);
    const docBytes = new TextEncoder().encode(
      `\\documentclass{article}\\begin{document}A${commandSource}B\\end{document}`,
    );
    if (addMountedFile('main.tex', docBytes, `${tag}_main`) !== 0) {
      throw new Error(`mount_add_file(${tag} main.tex) failed`);
    }
    if (ctx.mountFinalize() !== 0) throw new Error(`mount_finalize for ${tag} case failed`);
    expectOk(ctx.compileMain(), `compile_main_v0(${tag})`);
    const logBytes = readCompileLogBytes();
    if (logBytes.length !== 0) {
      throw new Error(`compile_main(${tag}) expected empty log, got ${logBytes.length} bytes`);
    }
    assertEventsMatchLogAndStats(
      logBytes,
      { char_count: baselineStats.char_count + 2 },
      `compile_main(${tag})`,
    );
    const xdvBytes = readMainXdvArtifactBytes(`compile_main(${tag})`);
    if (xdvBytes.length === 0) {
      throw new Error(`compile_main(${tag}) main.xdv expected non-empty bytes`);
    }
    const movement = countMovementOpsInTextPages(xdvBytes, `compile_main(${tag})`);
    if (movement.right3PositiveTotal !== 131072) {
      throw new Error(
        `compile_main(${tag}) expected right3PositiveTotal=131072, got ${movement.right3PositiveTotal}`,
      );
    }
  };

  runNoopCase('\\phantomsection ', 'ok_noop_phantomsection');
  runNoopCase('\\bibliographystyle{plain}', 'ok_noop_bibliographystyle');
  runNoopCase('\\bibliography{refs}', 'ok_noop_bibliography');
  runNoopCase('\\nocite{X,Y}', 'ok_noop_nocite');
  runNoopCase('\\addcontentsline{toc}{section}{Foo}', 'ok_noop_addcontentsline');
  runNoopCase('\\markboth{L}{R}', 'ok_noop_markboth');
  runNoopCase('\\vspace{1em}', 'ok_noop_vspace');
  runNoopCase('\\vspace*{1em}', 'ok_noop_vspace_star');
  runNoopCase('\\hspace{1em}', 'ok_noop_hspace');
  runNoopCase('\\hspace*{1em}', 'ok_noop_hspace_star');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before noop missing-arg invalid case failed');
  const missingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\markright X\\end{document}',
  );
  if (addMountedFile('main.tex', missingArgDocBytes, 'ok_noop_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(noop missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for noop missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok noop missing-arg invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before noop vspace missing-arg invalid case failed');
  const vspaceMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\vspace X\\end{document}',
  );
  if (addMountedFile('main.tex', vspaceMissingArgDocBytes, 'ok_noop_vspace_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(noop vspace missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for noop vspace missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok noop vspace missing-arg invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before noop hspace missing-arg invalid case failed');
  const hspaceMissingArgDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\hspace X\\end{document}',
  );
  if (addMountedFile('main.tex', hspaceMissingArgDocBytes, 'ok_noop_hspace_missing_arg_main') !== 0) {
    throw new Error('mount_add_file(noop hspace missing-arg main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for noop hspace missing-arg invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(ok noop hspace missing-arg invalid)');
}
