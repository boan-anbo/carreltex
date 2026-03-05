const fs = require('node:fs');
const hintsPath = process.argv[2];
const outputPath = process.argv[3];
const hints = JSON.parse(fs.readFileSync(hintsPath, 'utf8'));
const baseRequest = {
  kind: 'texmf',
  format: 'tex',
  name: 'typeset_demo_minimal_v0',
  variant: 'typeset',
};
const requests = [baseRequest, ...(Array.isArray(hints.requests) ? hints.requests : [])];
const deduped = [];
const seen = new Set();
for (const request of requests) {
  const key = `${request.kind}\u0000${request.format}\u0000${request.name}\u0000${request.variant}`;
  if (seen.has(key)) {
    continue;
  }
  seen.add(key);
  deduped.push(request);
}
const output = {
  version: 1,
  requests: deduped,
};
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
