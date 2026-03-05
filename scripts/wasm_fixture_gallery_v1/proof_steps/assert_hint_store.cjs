const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const storeDirA = process.argv[2];
const storeDirB = process.argv[3];
const sha256 = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');

const indexAPath = path.join(storeDirA, 'index.json');
const indexBPath = path.join(storeDirB, 'index.json');
const summaryAPath = path.join(storeDirA, 'summary.json');
const summaryBPath = path.join(storeDirB, 'summary.json');
const indexASha = sha256(fs.readFileSync(indexAPath));
const indexBSha = sha256(fs.readFileSync(indexBPath));
if (indexASha !== indexBSha) {
  console.error('FAIL: texlive_store_gen_v0 from hints must be deterministic');
  process.exit(1);
}
const summaryA = JSON.parse(fs.readFileSync(summaryAPath, 'utf8'));
const summaryB = JSON.parse(fs.readFileSync(summaryBPath, 'utf8'));
if (summaryA.index_sha256 !== summaryB.index_sha256 || summaryA.found_count !== summaryB.found_count) {
  console.error('FAIL: texlive_store_gen_v0 hint summaries must match across reruns');
  process.exit(1);
}
const indexA = JSON.parse(fs.readFileSync(indexAPath, 'utf8'));
const entries = Array.isArray(indexA?.entries) ? indexA.entries : [];
const hasEntry = (kind, format, name, variant) => entries.some(
  (entry) => entry.kind === kind && entry.format === format && entry.name === name && entry.variant === variant,
);
const requiredEntries = [
  ['texmf', 'tex', 'typeset_demo_minimal_v0', 'typeset'],
  ['texmf', 'tex', 'chapter_intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapter_appendix.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__appendix.tex', 'typeset'],
  ['texmf', 'tex', 'sections__intro.tex', 'typeset'],
  ['texmf', 'tex', 'chapters__ch1.tex', 'typeset'],
  ['texmf', 'tex', 'appendices__apx_a.tex', 'typeset'],
  ['texmf', 'tex', 'appendices__apx_b.tex', 'typeset'],
  ['texmf', 'sty', 'xcolor.sty', 'typeset'],
  ['texmf', 'sty', 'foo__bar.sty', 'typeset'],
  ['texmf', 'sty', 'fooopts.sty', 'typeset'],
  ['texmf', 'sty', 'baropts.sty', 'typeset'],
  ['texmf', 'sty', 'pkgoptsdemo.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__a.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__b.sty', 'typeset'],
  ['texmf', 'sty', 'packmulti__c.sty', 'typeset'],
  ['texmf', 'cls', 'classoptsdemo.cls', 'typeset'],
  ['texmf', 'cls', 'classoptsmulti.cls', 'typeset'],
  ['texmf', 'cls', 'memoir.cls', 'typeset'],
  ['texmf', 'cls', 'memoirplus.cls', 'typeset'],
  ['texmf', 'bib', 'refs.bib', 'typeset'],
  ['texmf', 'bib', 'styleprobe_refs.bib', 'typeset'],
  ['texmf', 'bib', 'multiadd_refs.bib', 'typeset'],
  ['texmf', 'bib', 'multibib_a.bib', 'typeset'],
  ['texmf', 'bib', 'multibib_b.bib', 'typeset'],
  ['texmf', 'bib', 'legacyrefs.bib', 'typeset'],
  ['texmf', 'bib', 'bib__deep__refs-local.bib', 'typeset'],
  ['texmf', 'bib', 'legacy__deeprefs.bib', 'typeset'],
  ['texmf', 'bst', 'plain.bst', 'typeset'],
  ['texmf', 'sty', 'natbib.sty', 'typeset'],
  ['texmf', 'png', 'demo.png', 'typeset'],
  ['texmf', 'png', 'probe-figure.png', 'typeset'],
  ['texmf', 'pdf', 'figs__diagram.pdf', 'typeset'],
  ['texmf', 'pdf', 'figs__demo_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'plots__demo_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'figs__banner_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'figs__sub__banner_graphic.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__figs__multi_probe.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__plots__multi_probe.pdf', 'typeset'],
  ['texmf', 'pdf', 'assets__hires__chart.pdf', 'typeset'],
  ['fontconfig', 'name', 'FoundSans', 'public'],
];
for (const [kind, format, name, variant] of requiredEntries) {
  if (!hasEntry(kind, format, name, variant)) {
    console.error(`FAIL: expected hint-driven store entry ${kind}/${format}/${variant}/${name}`);
    process.exit(1);
  }
}
if (!(summaryA.found_count >= requiredEntries.length && summaryA.missing_count >= 1)) {
  console.error(`FAIL: expected hint-driven store found>=${requiredEntries.length} and missing>=1, got found=${summaryA.found_count} missing=${summaryA.missing_count}`);
  process.exit(1);
}
console.log(`PASS: texlive_store_gen_v0 from hints deterministic index_sha256 ${indexASha}`);
console.log(`PASS: texlive_store_gen_v0 from hints found=${summaryA.found_count} missing=${summaryA.missing_count}`);
