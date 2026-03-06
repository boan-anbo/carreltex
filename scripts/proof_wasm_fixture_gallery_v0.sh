#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
ONDEMAND_OUT_DIR="${OUT_DIR}_ondemand_v1"
STORE_DIR="${OUT_DIR}_store"
HINT_STORE_DIR_A="${OUT_DIR}_store_from_hints_a"
HINT_STORE_DIR_B="${OUT_DIR}_store_from_hints_b"
BASELINE_ROOT="${OUT_DIR}_baseline"
BASELINE_DIR_A="$BASELINE_ROOT/run1"
BASELINE_DIR_B="$BASELINE_ROOT/run2"
BASELINE_PACKS_ROOT="${BASELINE_ROOT}/packs"
BASELINE_AUTO_PACK_DIR="${BASELINE_PACKS_ROOT}/auto_pack"
REQUEST_LIST="$OUT_DIR/requests.json"
HINT_REQUEST_LIST_A="$OUT_DIR/request_list_from_hints_a.json"
HINT_REQUEST_LIST_B="$OUT_DIR/request_list_from_hints_b.json"
COMBINED_REQUEST_LIST_A="$OUT_DIR/requests_combined_from_hints_a.json"
COMBINED_REQUEST_LIST_B="$OUT_DIR/requests_combined_from_hints_b.json"
FIXTURE_SOURCE_DIR="${OUT_DIR}_fixture_source_v0"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export SOURCE_DATE_EPOCH
export TZ=UTC

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

rm -rf "$OUT_DIR" "$ONDEMAND_OUT_DIR" "$STORE_DIR" "$HINT_STORE_DIR_A" "$HINT_STORE_DIR_B" "$FIXTURE_SOURCE_DIR" "$BASELINE_ROOT"
mkdir -p \
  "$OUT_DIR" \
  "$FIXTURE_SOURCE_DIR/xetex/tex" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/sections" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/chapters" \
  "$FIXTURE_SOURCE_DIR/xetex/tex/appendices" \
  "$FIXTURE_SOURCE_DIR/xetex/bib" \
  "$FIXTURE_SOURCE_DIR/xetex/bst" \
  "$FIXTURE_SOURCE_DIR/xetex/png" \
  "$FIXTURE_SOURCE_DIR/xetex/pdf" \
  "$FIXTURE_SOURCE_DIR/xetex/sty" \
  "$FIXTURE_SOURCE_DIR/xetex/cls" \
  "$FIXTURE_SOURCE_DIR/fontconfig/public" \
  "$BASELINE_ROOT" \
  "$BASELINE_PACKS_ROOT"

printf 'fixture-bytes-for-typeset-minimal-v0\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_minimal_v0"
printf 'fixture-bytes-for-ondemand-input-probe-main\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_ondemand_input_probe_v0"
printf 'fixture-bytes-for-ondemand-include-probe-main\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/typeset_demo_ondemand_include_probe_v0"
printf 'fixture-bytes-for-chapter-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_intro.tex"
printf 'fixture-bytes-for-chapter-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapter_appendix.tex"
printf 'fixture-bytes-for-chapters-intro\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__intro.tex"
printf 'fixture-bytes-for-chapters-appendix\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__appendix.tex"
printf 'fixture-bytes-for-sections-intro-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/intro.tex"
printf 'fixture-bytes-for-chapters-ch1-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters/ch1.tex"
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/one.tex" <<'EOF'
\section{Included One}
\label{sec:one}
Included section body.
EOF
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/two.tex" <<'EOF'
\section{Included Two}
\label{sec:two}
\begin{figure}
\caption{Included figure}
\end{figure}
\label{fig:two}
\[x+y\]
\label{eq:two}
EOF
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections/toc_headings.tex" <<'EOF'
\section{Input Section}
\subsection{Input Detail}
EOF
printf 'fixture-bytes-for-sections-intro-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__intro.tex"
printf 'fixture-bytes-for-chapters-ch1-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/chapters__ch1.tex"
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__one.tex" <<'EOF'
\section{Included One}
\label{sec:one}
Included section body.
EOF
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__two.tex" <<'EOF'
\section{Included Two}
\label{sec:two}
\begin{figure}
\caption{Included figure}
\end{figure}
\label{fig:two}
\[x+y\]
\label{eq:two}
EOF
cat > "$FIXTURE_SOURCE_DIR/xetex/tex/sections__toc_headings.tex" <<'EOF'
\section{Input Section}
\subsection{Input Detail}
EOF
printf 'fixture-bytes-for-ondemand-extra-section-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand/extra_section.tex"
printf 'fixture-bytes-for-ondemand-chapter-one-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand/chapter_one.tex"
printf 'fixture-bytes-for-ondemand-extra-section-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand__extra_section.tex"
printf 'fixture-bytes-for-ondemand-chapter-one-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/ondemand__chapter_one.tex"
printf 'fixture-bytes-for-appendices-apx-a-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices/apx_a.tex"
printf 'fixture-bytes-for-appendices-apx-b-nested\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices/apx_b.tex"
printf 'fixture-bytes-for-appendices-apx-a-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices__apx_a.tex"
printf 'fixture-bytes-for-appendices-apx-b-normalized\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/appendices__apx_b.tex"
printf '\\include{cycles/b}\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/cycles__a.tex"
printf '\\input{cycles/a}\n' > "$FIXTURE_SOURCE_DIR/xetex/tex/cycles__b.tex"
printf 'fixture-bytes-for-demo-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/demo.png"
printf 'fixture-bytes-for-probe-figure-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/probe-figure.png"
printf 'fixture-bytes-for-figs-demo-graphic-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/figs__demo_graphic.png"
printf 'fixture-bytes-for-plots-demo-graphic-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/plots__demo_graphic.png"
printf 'fixture-bytes-for-figs-banner-graphic-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/figs__banner_graphic.png"
printf 'fixture-bytes-for-figs-sub-banner-graphic-png\n' > "$FIXTURE_SOURCE_DIR/xetex/png/figs__sub__banner_graphic.png"
printf 'fixture-bytes-for-figs-diagram-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/figs__diagram.pdf"
printf 'fixture-bytes-for-assets-figs-multi-probe-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__figs__multi_probe.pdf"
printf 'fixture-bytes-for-assets-plots-multi-probe-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__plots__multi_probe.pdf"
printf 'fixture-bytes-for-assets-hires-chart-pdf\n' > "$FIXTURE_SOURCE_DIR/xetex/pdf/assets__hires__chart.pdf"
printf 'fixture-bytes-for-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/refs.bib"
printf 'fixture-bytes-for-styleprobe-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/styleprobe_refs.bib"
printf 'fixture-bytes-for-multiadd-refs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multiadd_refs.bib"
printf 'fixture-bytes-for-multibib-a-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multibib_a.bib"
printf 'fixture-bytes-for-multibib-b-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/multibib_b.bib"
printf 'fixture-bytes-for-legacyrefs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/legacyrefs.bib"
printf 'fixture-bytes-for-bib-deep-refs-local-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/bib__deep__refs-local.bib"
printf 'fixture-bytes-for-legacy-deeprefs-bib\n' > "$FIXTURE_SOURCE_DIR/xetex/bib/legacy__deeprefs.bib"
printf 'fixture-bytes-for-plain-bst\n' > "$FIXTURE_SOURCE_DIR/xetex/bst/plain.bst"
printf 'fixture-bytes-for-xcolor-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/xcolor.sty"
printf 'fixture-bytes-for-hyperref-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/hyperref.sty"
printf 'fixture-bytes-for-graphicx-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/graphicx.sty"
printf 'fixture-bytes-for-foo-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/foo.sty"
printf 'fixture-bytes-for-foo-bar-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/foo__bar.sty"
printf 'fixture-bytes-for-fooopts-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/fooopts.sty"
printf 'fixture-bytes-for-baropts-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/baropts.sty"
printf 'fixture-bytes-for-pkgoptsdemo-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/pkgoptsdemo.sty"
printf 'fixture-bytes-for-packmulti-a-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__a.sty"
printf 'fixture-bytes-for-packmulti-b-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__b.sty"
printf 'fixture-bytes-for-packmulti-c-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/packmulti__c.sty"
printf 'fixture-bytes-for-natbib-sty\n' > "$FIXTURE_SOURCE_DIR/xetex/sty/natbib.sty"
printf 'fixture-bytes-for-memoir-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/memoir.cls"
printf 'fixture-bytes-for-classoptsdemo-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/classoptsdemo.cls"
printf 'fixture-bytes-for-classoptsmulti-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/classoptsmulti.cls"
printf 'fixture-bytes-for-memoirplus-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/memoirplus.cls"
printf 'fixture-bytes-for-article-cls\n' > "$FIXTURE_SOURCE_DIR/xetex/cls/article.cls"
printf 'fixture-bytes-for-found-sans\n' > "$FIXTURE_SOURCE_DIR/fontconfig/public/FoundSans"

cat > "$REQUEST_LIST" <<'JSON'
{
  "version": 1,
  "requests": [
    { "kind": "texmf", "format": "tex", "name": "typeset_demo_minimal_v0", "variant": "typeset" },
    { "kind": "texmf", "format": "tex", "name": "missing_demo_case", "variant": "ok" }
  ]
}
JSON

TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$REQUEST_LIST" "$STORE_DIR"

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$STORE_DIR" \
node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"

node "$ROOT_DIR/scripts/texlive_smoke/request_list_from_hints_v0.mjs" \
  "$OUT_DIR/report.json" \
  "$HINT_REQUEST_LIST_A"
node "$ROOT_DIR/scripts/texlive_smoke/request_list_from_hints_v0.mjs" \
  "$OUT_DIR/report.json" \
  "$HINT_REQUEST_LIST_B"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-request-list \
  "$HINT_REQUEST_LIST_A" \
  "$HINT_REQUEST_LIST_B"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  write-combined-request-list \
  "$HINT_REQUEST_LIST_A" \
  "$COMBINED_REQUEST_LIST_A"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  write-combined-request-list \
  "$HINT_REQUEST_LIST_B" \
  "$COMBINED_REQUEST_LIST_B"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-first-run \
  "$OUT_DIR" \
  "$BASELINE_ROOT"

TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$COMBINED_REQUEST_LIST_A" "$HINT_STORE_DIR_A"
TEXLIVE_RESOLVER_BACKEND_V0=fixture_dir_v0 \
TEXLIVE_STORE_SOURCE_DIR_V0="$FIXTURE_SOURCE_DIR" \
node "$ROOT_DIR/scripts/texlive_store_gen_v0.mjs" "$COMBINED_REQUEST_LIST_B" "$HINT_STORE_DIR_B"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-hint-store \
  "$HINT_STORE_DIR_A" \
  "$HINT_STORE_DIR_B"

TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
TEXLIVE_STORE_DIR_V0="$HINT_STORE_DIR_A" \
node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"

node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_A"
node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_DIR_B"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-baseline-generator \
  "$BASELINE_DIR_A" \
  "$BASELINE_DIR_B" \
  "$OUT_DIR"

node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/generate_v0.mjs" "$OUT_DIR" "$BASELINE_AUTO_PACK_DIR"

rm -rf "$STORE_DIR"
cp -R "$HINT_STORE_DIR_A" "$STORE_DIR"

WASM_FIXTURE_GALLERY_SKIP_PROOF_V0=1 \
WASM_FIXTURE_GALLERY_NO_OPEN_V0=1 \
WASM_FIXTURE_GALLERY_AUTO_BASELINE_PACK_V0=1 \
TEXLIVE_BASELINE_PACKS_DIR_V0="$BASELINE_PACKS_ROOT" \
"$ROOT_DIR/scripts/open_wasm_fixture_gallery_v0.sh" "$OUT_DIR"

if [[ ! -s "$OUT_DIR/report.json" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/report.json" >&2
  exit 1
fi

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-second-run \
  "$OUT_DIR"

node "$ROOT_DIR/scripts/wasm_fixture_gallery_v1/proof_runner_v0.mjs" \
  assert-ondemand-integration \
  "$ROOT_DIR" \
  "$ONDEMAND_OUT_DIR" \
  "$STORE_DIR" \
  "$FIXTURE_SOURCE_DIR"

echo "PASS: wasm fixture gallery artifacts $OUT_DIR"
echo "PASS: wasm fixture gallery proof"
