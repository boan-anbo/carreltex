const { execFileSync } = require('node:child_process');
const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { parentPort, workerData } = require('node:worker_threads');

global.self = global;
// Node 18+ provides a global `fetch`. Emscripten's Node path may incorrectly
// attempt `fetch("/abs/path/to/file.wasm")`, which fails because it's not a URL.
// Force the synchronous fs-based loader path instead.
global.fetch = undefined;

function postMessage(msg, transferList) {
  try {
    parentPort.postMessage(msg, transferList);
  } catch {
    parentPort.postMessage(msg);
  }
}

self.postMessage = postMessage;
parentPort.on('message', (data) => {
  if (typeof self.onmessage === 'function') self.onmessage({ data });
});

class XMLHttpRequest {
  constructor() {
    this.timeout = 0;
    this.responseType = '';
    this.status = 0;
    this.response = null;
    this._method = null;
    this._url = null;
    this._async = false;
    this._headers = '';
  }

  open(method, url, async) {
    this._method = method;
    this._url = url;
    this._async = Boolean(async);
  }

  getResponseHeader(name) {
    const key = String(name).toLowerCase();
    const lines = this._headers.split(/\r?\n/);
    for (const line of lines) {
      const idx = line.indexOf(':');
      if (idx < 0) continue;
      const k = line.slice(0, idx).trim().toLowerCase();
      if (k === key) return line.slice(idx + 1).trim();
    }
    return null;
  }

  send() {
    if (this._async) {
      throw new Error('XMLHttpRequest async=true not supported in this worker');
    }

    const local = readLocalTexLiveBlobIfApplicable(this._url);
    if (local) {
      this.status = local.status;
      this._headers = local.headers;
      if (this.responseType === 'arraybuffer') {
        this.response = local.body.buffer.slice(local.body.byteOffset, local.body.byteOffset + local.body.byteLength);
      } else {
        this.response = local.body.toString('utf8');
      }
      return;
    }

    const headerPath = path.join(os.tmpdir(), `swiftlatex_hdr_${process.pid}_${Date.now()}.txt`);
    const bodyPath = path.join(os.tmpdir(), `swiftlatex_body_${process.pid}_${Date.now()}.bin`);
    try {
      const maxTimeSecs = this.timeout > 0 ? Math.ceil(this.timeout / 1000) : 150;
      const code = execFileSync('curl', [
        '-sS',
        '-L',
        '--max-time',
        String(maxTimeSecs),
        '-D',
        headerPath,
        '-o',
        bodyPath,
        '-w',
        '%{http_code}',
        this._url,
      ]).toString();
      this.status = Number.parseInt(code, 10) || 0;
      // SwiftLaTeX engines treat HTTP 301 as "not found" for TeX Live blobs.
      // Map 404 to 301 to avoid repeated attempts that never get cached.
      if (this.status === 404) this.status = 301;
      this._headers = fs.readFileSync(headerPath, 'utf8');
      const body = fs.readFileSync(bodyPath);
      if (this.responseType === 'arraybuffer') {
        this.response = body.buffer.slice(body.byteOffset, body.byteOffset + body.byteLength);
      } else {
        this.response = body.toString('utf8');
      }
    } finally {
      try {
        fs.unlinkSync(headerPath);
      } catch {}
      try {
        fs.unlinkSync(bodyPath);
      } catch {}
    }
  }
}

global.XMLHttpRequest = XMLHttpRequest;

require(workerData.engineJsPath);

function readLocalTexLiveBlobIfApplicable(url) {
  if (!url || typeof url !== 'string') return null;
  // Default SwiftLaTeX endpoint: https://texlive2.swiftlatex.com/xetex/<format>/<filename>
  // That endpoint is an opaque TeX Live blob store. For a deterministic local dev vertical-slice,
  // we can satisfy these requests from a local TeX Live installation via kpsewhich.
  //
  // Enabled by default when the URL targets `texlive2.swiftlatex.com`. Can be disabled by
  // setting `CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND=remote`.
  const backend = process.env.CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND || 'local';
  if (backend !== 'local') return null;

  let parsed;
  try {
    parsed = new URL(url);
  } catch {
    return null;
  }
  if (parsed.hostname !== 'texlive2.swiftlatex.com') return null;
  const parts = parsed.pathname.split('/').filter(Boolean);
  if (parts.length < 3) return null;
  if (parts[0] !== 'xetex') return null;

  const formatNum = Number.parseInt(parts[1], 10);
  const filename = parts.slice(2).join('/');
  if (!filename || filename.includes('/')) {
    return { status: 301, headers: '', body: Buffer.alloc(0) };
  }

  const resolvedPath = resolveTexLivePath(filename, Number.isFinite(formatNum) ? formatNum : null);
  if (!resolvedPath) {
    return { status: 301, headers: '', body: Buffer.alloc(0) };
  }

  const body = fs.readFileSync(resolvedPath);
  const fileid = makeStableFileId(filename, resolvedPath);
  const headers = `fileid: ${fileid}\r\n`;
  return { status: 200, headers, body };
}

function resolveTexLivePath(filename, formatNum) {
  if (filename === 'swiftlatexxetex.fmt') {
    const overridePath = process.env.CARRELTEX_SWIFTLATEX_FMT_CACHE_PATH;
    if (overridePath && fs.existsSync(overridePath)) return overridePath;
    // No compatible local override exists until the orchestrator generates it via `compileformat`.
    return null;
  }

  if (filename === 'main.xdv') {
    const overridePath = process.env.CARRELTEX_SWIFTLATEX_XDV_CACHE_PATH;
    if (overridePath && fs.existsSync(overridePath)) return overridePath;
  }

  const strippedName = filename.replace(/^\[(.*)\]$/, '$1');

  const kpseFormat = kpseFormatName(formatNum);
  const tryKpse = (name) => {
    try {
      const args = ['--engine=xetex'];
      if (kpseFormat) args.push('-format', kpseFormat);
      args.push(name);
      const out = execFileSync('kpsewhich', args, { encoding: 'utf8' }).trim();
      return out.length > 0 ? out : null;
    } catch {
      return null;
    }
  };

  // Primary attempt: request name as-is.
  let resolved = tryKpse(filename) || (strippedName !== filename ? tryKpse(strippedName) : null);
  if (resolved) return resolved;

  // Heuristic: some engine requests omit extensions (notably for OpenType fonts).
  // Try common font extensions when the base name is available.
  const fontCandidates = [
    `${strippedName}.otf`,
    `${strippedName}.ttf`,
    `${strippedName}.ttc`,
    `${strippedName}.pfb`,
  ];
  for (const candidate of fontCandidates) {
    resolved = tryKpse(candidate);
    if (resolved) return resolved;
  }

  return null;
}

function sha256Hex(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function kpseFormatName(formatNum) {
  if (formatNum === null || formatNum === undefined) return null;
  // SwiftLaTeX passes through kpathsea format numbers. We only map the ones we
  // have observed in practice; unknown formats fall back to kpsewhich defaults.
  //
  // Known values:
  // - 10: fmt (format file)
  // - 26: tex (plain LaTeX sources: .ini/.ltx/.cfg/.tex, etc)
  // - 3:  tfm (font metrics; e.g. `cmr10`)
  // - 11: map (pdftex.map)
  if (formatNum === 10) return 'fmt';
  if (formatNum === 26) return 'tex';
  if (formatNum === 3) return 'tfm';
  if (formatNum === 11) return 'map';
  return null;
}

function makeStableFileId(filename, resolvedPath) {
  // `fileid` becomes part of the in-worker filesystem path `/tex/<fileid>`.
  // Prefer preserving upstream basenames so XDV/PDF stages can refer to the same paths.
  // We still special-case a few files to keep initex jobnames stable.
  if (filename === 'xelatex.ini') return 'xelatex.ini';
  if (filename === 'main.xdv') return 'main.xdv';
  return path.basename(resolvedPath).replace(/[\\/]/g, '_');
}
