export function runOkTableCases(ctx, helpers, baselineStats) {
  const {
    addMountedFile,
    expectOk,
    expectNotImplemented,
    readCompileLogBytes,
    assertEventsMatchLogAndStats,
    readMainXdvArtifactBytes,
    countMovementOpsInTextPages,
  } = helpers;

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK tabular env case failed');
  const tabularDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{tabular}{c}x\\end{tabular}B\\end{document}',
  );
  if (addMountedFile('main.tex', tabularDocBytes, 'ok_table_env_tabular_main') !== 0) {
    throw new Error('mount_add_file(ok tabular env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK tabular env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok tabular env)');
  const tabularLogBytes = readCompileLogBytes();
  if (tabularLogBytes.length !== 0) {
    throw new Error(`compile_main(ok tabular env) expected empty log, got ${tabularLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    tabularLogBytes,
    { char_count: baselineStats.char_count + 18 },
    'compile_main(ok tabular env)',
  );
  const tabularXdvBytes = readMainXdvArtifactBytes('compile_main(ok tabular env)');
  if (tabularXdvBytes.length === 0) {
    throw new Error('compile_main(ok tabular env) main.xdv expected non-empty bytes');
  }
  const tabularMovement = countMovementOpsInTextPages(tabularXdvBytes, 'compile_main(ok tabular env)');
  if (tabularMovement.down3 < 2) {
    throw new Error(`compile_main(ok tabular env) expected down3>=2, got ${tabularMovement.down3}`);
  }
  if (tabularMovement.right3PositiveTotal !== 589824) {
    throw new Error(
      `compile_main(ok tabular env) expected right3PositiveTotal=589824, got ${tabularMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before OK longtable env case failed');
  const longtableDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}A\\begin{longtable}\\end{longtable}B\\end{document}',
  );
  if (addMountedFile('main.tex', longtableDocBytes, 'ok_table_env_longtable_main') !== 0) {
    throw new Error('mount_add_file(ok longtable env main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for OK longtable env case failed');
  expectOk(ctx.compileMain(), 'compile_main_v0(ok longtable env)');
  const longtableLogBytes = readCompileLogBytes();
  if (longtableLogBytes.length !== 0) {
    throw new Error(`compile_main(ok longtable env) expected empty log, got ${longtableLogBytes.length} bytes`);
  }
  assertEventsMatchLogAndStats(
    longtableLogBytes,
    { char_count: baselineStats.char_count + 20 },
    'compile_main(ok longtable env)',
  );
  const longtableXdvBytes = readMainXdvArtifactBytes('compile_main(ok longtable env)');
  if (longtableXdvBytes.length === 0) {
    throw new Error('compile_main(ok longtable env) main.xdv expected non-empty bytes');
  }
  const longtableMovement = countMovementOpsInTextPages(
    longtableXdvBytes,
    'compile_main(ok longtable env)',
  );
  if (longtableMovement.down3 < 2) {
    throw new Error(`compile_main(ok longtable env) expected down3>=2, got ${longtableMovement.down3}`);
  }
  if (longtableMovement.right3PositiveTotal !== 589824) {
    throw new Error(
      `compile_main(ok longtable env) expected right3PositiveTotal=589824, got ${longtableMovement.right3PositiveTotal}`,
    );
  }

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before nested table env invalid case failed');
  const nestedTableEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{tabular}\\begin{center}X\\end{center}\\end{tabular}\\end{document}',
  );
  if (addMountedFile('main.tex', nestedTableEnvDocBytes, 'ok_table_env_nested_begin_main') !== 0) {
    throw new Error('mount_add_file(nested table env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for nested table env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(nested table env invalid)');

  if (ctx.mountReset() !== 0) throw new Error('mount_reset before missing end table env invalid case failed');
  const missingEndTableEnvDocBytes = new TextEncoder().encode(
    '\\documentclass{article}\\begin{document}\\begin{tabular}X\\end{document}',
  );
  if (addMountedFile('main.tex', missingEndTableEnvDocBytes, 'ok_table_env_missing_end_main') !== 0) {
    throw new Error('mount_add_file(missing end table env invalid main.tex) failed');
  }
  if (ctx.mountFinalize() !== 0) throw new Error('mount_finalize for missing end table env invalid case failed');
  expectNotImplemented(ctx.compileMain(), 'compile_main_v0(missing end table env invalid)');
}
