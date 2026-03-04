import path from 'node:path';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { Worker } from 'node:worker_threads';

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function createEngineWorker(wrapperPath, engineJsPath) {
  const defaultTimeoutMs = Number.parseInt(process.env.CARRELTEX_WASM_TYPESET_TIMEOUT_MS || '900000', 10);
  const worker = new Worker(wrapperPath, {
    workerData: { engineJsPath },
  });

  let readyResolve;
  let readyReject;
  const ready = new Promise((resolve, reject) => {
    readyResolve = resolve;
    readyReject = reject;
  });

  const inbox = [];
  worker.on('message', (msg) => {
    inbox.push(msg);
    if (msg && msg.result === 'ok' && msg.cmd === undefined && msg.status === undefined && msg.log === undefined) {
      readyResolve();
    }
  });
  worker.on('error', (err) => readyReject(err));
  worker.on('exit', (code) => {
    if (code !== 0) readyReject(new Error(`worker exit code ${code}`));
  });

  async function waitFor(predicate, timeoutMs = defaultTimeoutMs) {
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
      for (let i = 0; i < inbox.length; i++) {
        const msg = inbox[i];
        if (predicate(msg)) {
          inbox.splice(i, 1);
          return msg;
        }
      }
      await new Promise((r) => setTimeout(r, 10));
    }
    throw new Error('timeout waiting for worker message');
  }

  function sendOnly(cmd) {
    worker.postMessage(cmd);
  }

  async function sendAndWait(cmd, predicate) {
    worker.postMessage(cmd);
    return await waitFor(predicate);
  }

  return { worker, ready, sendOnly, sendAndWait };
}

async function ensureXetexFormatCache({ xetex, fmtCachePath }) {
  // Generate a format file compatible with this SwiftLaTeX XeTeX WASM build.
  // The engine will later request `swiftlatexxetex.fmt` via kpsewhich/XHR, which our worker shim
  // serves from `CARRELTEX_SWIFTLATEX_FMT_CACHE_PATH`.
  await mkdir(path.dirname(fmtCachePath), { recursive: true });
  const res = await xetex.sendAndWait(
    { cmd: 'compileformat' },
    (m) => m && m.cmd === 'compile' && (m.result === 'ok' || m.result === 'failed'),
  );
  if (res.result !== 'ok') {
    const log = typeof res.log === 'string' ? res.log : JSON.stringify(res.log);
    throw new Error(`xetex compileformat failed: status=${res.status} log=${log}`);
  }
  const fmtBytes = new Uint8Array(res.pdf);
  await writeFile(fmtCachePath, fmtBytes);
}

async function compileTexToXdvBytes({ xetex, mainTexBytes, entrypoint = 'main.tex' }) {
  // SwiftLaTeX workers do not acknowledge `flushcache` or `setmainfile`.
  // Messages are processed sequentially, so send them without waiting.
  xetex.sendOnly({ cmd: 'flushcache' });
  xetex.sendOnly({ cmd: 'setmainfile', url: entrypoint });
  await xetex.sendAndWait({ cmd: 'writefile', url: entrypoint, src: mainTexBytes }, (m) => m && m.cmd === 'writefile');

  const res = await xetex.sendAndWait(
    { cmd: 'compilelatex' },
    (m) => m && m.cmd === 'compile' && (m.result === 'ok' || m.result === 'failed'),
  );
  if (res.result !== 'ok') {
    const log = typeof res.log === 'string' ? res.log : JSON.stringify(res.log);
    throw new Error(`xetex compile failed: status=${res.status} log=${log}`);
  }
  return new Uint8Array(res.pdf);
}

async function convertXdvToPdfBytes({ dvipdfm, xdvBytes, entrypoint = 'main.tex' }) {
  const xdvName = entrypoint.replace(/\\.tex$/i, '.xdv');
  // dvipdfm worker expects main entry to match the XDV filename.
  dvipdfm.sendOnly({ cmd: 'flushcache' });
  dvipdfm.sendOnly({ cmd: 'setmainfile', url: xdvName });
  await dvipdfm.sendAndWait({ cmd: 'writefile', url: xdvName, src: xdvBytes }, (m) => m && m.cmd === 'writefile');

  const res = await dvipdfm.sendAndWait(
    { cmd: 'compilepdf' },
    (m) => m && m.cmd === 'compile' && (m.result === 'ok' || m.result === 'failed'),
  );
  if (res.result !== 'ok') {
    const log = typeof res.log === 'string' ? res.log : JSON.stringify(res.log);
    throw new Error(`dvipdfm compile failed: status=${res.status} log=${log}`);
  }
  return new Uint8Array(res.pdf);
}

async function runOne({ fixturePath, outDir, xetex, dvipdfm }) {
  const caseName = path.basename(fixturePath, '.tex');
  const caseDir = path.join(outDir, caseName);
  await mkdir(caseDir, { recursive: true });

  const mainTexBytes = new Uint8Array(await readFile(fixturePath));
  const xdvBytes = await compileTexToXdvBytes({ xetex, mainTexBytes });
  // dvipdfmx uses kpathsea for opening the input file; seed a stable path the worker shim can serve.
  const xdvCachePath = path.join(outDir, '.swiftlatex_cache', 'main.xdv');
  await mkdir(path.dirname(xdvCachePath), { recursive: true });
  await writeFile(xdvCachePath, xdvBytes);
  const pdfBytes = await convertXdvToPdfBytes({ dvipdfm, xdvBytes });

  await writeFile(path.join(caseDir, 'main.xdv'), xdvBytes);
  await writeFile(path.join(caseDir, 'main.pdf'), pdfBytes);
}

async function main() {
  const [texliveBackend, swiftlatexDistDir, outDir, fixtureA, fixtureB] = process.argv.slice(2);
  assert(
    texliveBackend && swiftlatexDistDir && outDir && fixtureA && fixtureB,
    'usage: <texliveBackend> <distDir> <outDir> <fixtureA.tex> <fixtureB.tex>',
  );

  const wrapperPath = path.resolve(path.dirname(new URL(import.meta.url).pathname), 'wasm_typeset_swiftlatex_worker_v0.cjs');

  const fmtCachePath = path.join(outDir, '.swiftlatex_cache', 'swiftlatexxetex.fmt');
  process.env.CARRELTEX_SWIFTLATEX_FMT_CACHE_PATH = fmtCachePath;
  process.env.CARRELTEX_SWIFTLATEX_XDV_CACHE_PATH = path.join(outDir, '.swiftlatex_cache', 'main.xdv');
  process.env.CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND = texliveBackend;

  const xetexEngineJs = path.join(swiftlatexDistDir, 'swiftlatexxetex.js');
  const dvipdfmEngineJs = path.join(swiftlatexDistDir, 'swiftlatexdvipdfm.js');
  assert(await fileExists(xetexEngineJs), `missing ${xetexEngineJs}`);
  assert(await fileExists(dvipdfmEngineJs), `missing ${dvipdfmEngineJs}`);

  const xetex = createEngineWorker(wrapperPath, xetexEngineJs);
  const dvipdfm = createEngineWorker(wrapperPath, dvipdfmEngineJs);

  await xetex.ready;
  await dvipdfm.ready;

  try {
    await ensureXetexFormatCache({ xetex, fmtCachePath });
    await runOne({ fixturePath: fixtureA, outDir, xetex, dvipdfm });
    await runOne({ fixturePath: fixtureB, outDir, xetex, dvipdfm });
  } finally {
    xetex.worker.terminate();
    dvipdfm.worker.terminate();
  }
}

async function fileExists(p) {
  try {
    await readFile(p);
    return true;
  } catch {
    return false;
  }
}

await main();
