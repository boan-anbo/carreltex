use super::compile_main_typeset_minimal_v0;
use super::typeset_minimal_v0::{
    extract_typeset_minimal_text_body_v0, normalize_typeset_minimal_tokens_v0,
    preprocess_typeset_minimal_source_v0,
};
use crate::tex::tokenize_v0::tokenize_v0;
use carreltex_core::{CompileStatus, Mount};
use carreltex_xdv::parse_dvi_v2_text_page_to_layout_v0;

fn compile_typeset(main: &[u8]) -> carreltex_core::CompileResultV0 {
    let mut mount = Mount::default();
    mount
        .add_file(b"main.tex", main)
        .expect("main.tex should mount");
    compile_main_typeset_minimal_v0(&mut mount)
}

fn extract_typeset_body(main: &[u8]) -> Vec<u8> {
    let preprocessed = preprocess_typeset_minimal_source_v0(main);
    let tokens = tokenize_v0(&preprocessed).expect("tokenize should succeed");
    let normalized = normalize_typeset_minimal_tokens_v0(&tokens);
    extract_typeset_minimal_text_body_v0(&normalized).expect("extract should succeed")
}

fn layout_lines_bytes(main: &[u8]) -> Vec<Vec<u8>> {
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    let layout =
        parse_dvi_v2_text_page_to_layout_v0(&result.main_xdv_bytes, 917_504).expect("layout parse");
    layout.pages[0]
        .lines
        .iter()
        .map(|line| line.glyphs.iter().map(|glyph| glyph.byte).collect())
        .collect()
}

#[test]
fn typeset_minimal_subset_compiles_ok() {
    let main = b"\\documentclass{article}\\title{CarrelTeX Minimal Typeset Demo}\\author{Alice \\and Bob}\\date{2026-03-04}\\begin{document}\\maketitle Hello, world. This is a paragraph with \\emph{emphasis} and \\textbf{bold}.\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(!result.main_xdv_bytes.is_empty());
    assert!(result.log_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unsupported_control_sequence() {
    let main = b"\\documentclass{article}\\begin{document}\\foo{X}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unsupported_wrapper_in_body() {
    let main = b"\\documentclass{article}\\begin{document}A\\textit{B}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_blank_line_emits_paragraph_break() {
    let main = b"\\documentclass{article}\n\\begin{document}\nFirst paragraph.\n\n% spacer comment\n   \nSecond paragraph.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("First paragraph.\n\nSecond paragraph."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_explicit_par_emits_paragraph_break() {
    let main = b"\\documentclass{article}\n\\begin{document}\nFirst paragraph.\\par Second paragraph.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("First paragraph.\n\nSecond paragraph."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_author_and_is_single_newline() {
    let main = b"\\documentclass{article}\\author{Alice\\and Bob}\\begin{document}\\maketitle\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(!text.contains("Alice\n\nBob"), "body={text:?}");
    assert!(text.contains("Alice\nBob"), "body={text:?}");
}

#[test]
fn typeset_minimal_headings_emit_bold_with_paragraph_breaks() {
    let main = b"\\documentclass{article}\\begin{document}Before.\\section{Intro}\\subsection{A \\emph{B}}After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n@S {Intro}\n\n@s {A [B]}\n\n~ After."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_toc_after_maketitle_emits_placeholder_and_entries() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{Intro}\\subsection{Detail}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("\n\n!toc\n\n@S {Intro}\n\n@s {Detail}"), "body={text:?}");
    assert!(text.contains("!toc 1 1 Intro"), "body={text:?}");
    assert!(text.contains("!toc 2 2 Detail"), "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_toc_before_maketitle() {
    let main = b"\\documentclass{article}\\begin{document}\\tableofcontents\\maketitle\\section{Intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_toc_after_body_content() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle Before.\\tableofcontents\\section{Intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_toc_with_unsupported_heading_depth() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\subsubsection{Too deep}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_toc_entries_follow_heading_order_with_stable_anchors() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{One}\\subsection{Two}\\section{Three}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    let first = text.find("!toc 1 1 One").expect("first toc entry");
    let second = text.find("!toc 2 2 Two").expect("second toc entry");
    let third = text.find("!toc 1 3 Three").expect("third toc entry");
    assert!(first < second && second < third, "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_duplicate_tableofcontents_commands() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\tableofcontents\\section{Intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_tableofcontents_inside_center_environment() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\begin{center}\\tableofcontents\\end{center}\\section{Intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_accepts_deeper_heading_without_toc() {
    let main = b"\\documentclass{article}\\begin{document}\\subsubsection{Deep heading}Body.\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(!result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_toc_without_headings_emits_placeholder_only() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("\n\n!toc"), "body={text:?}");
    assert!(!text.contains("!toc 1 "), "body={text:?}");
}

#[test]
fn typeset_minimal_toc_allows_spacing_between_maketitle_and_command() {
    let main = b"\\documentclass{article}\n\\title{T}\\author{A}\\date{D}\n\\begin{document}\n\\maketitle\n\n   % spacer\n   \\tableofcontents\n\\section{Intro}\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("!toc"), "body={text:?}");
    assert!(text.contains("!toc 1 1 Intro"), "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_toc_without_maketitle_even_with_meta() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\tableofcontents\\section{Intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_toc_metadata_coexists_with_footnotes_and_links() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{Intro}Body\\footnote{Note one} and \\href{https://example.com}{Link}.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("!toc 1 1 Intro"), "body={text:?}");
    assert!(text.contains("!f 1 Note one"), "body={text:?}");
    assert!(text.contains("!u 1 https://example.com"), "body={text:?}");
    let toc_pos = text.find("!toc 1 1 Intro").expect("toc marker");
    let footnote_pos = text.find("!f 1 Note one").expect("footnote marker");
    let href_pos = text.find("!u 1 https://example.com").expect("href marker");
    assert!(footnote_pos < href_pos && href_pos < toc_pos, "body={text:?}");
}

#[test]
fn typeset_minimal_toc_anchor_ids_ignore_non_heading_commands() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{Alpha}Body with \\footnote{N}.\\subsection{Beta}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("!toc 1 1 Alpha"), "body={text:?}");
    assert!(text.contains("!toc 2 2 Beta"), "body={text:?}");
    assert!(!text.contains("!toc 1 2"), "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_tableofcontents_inside_list_environment() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\begin{itemize}\\item Intro\\tableofcontents\\end{itemize}\\section{After}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unsupported_heading_depth_when_toc_is_active() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{Top}\\paragraph{Too deep}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_toc_placeholder_precedes_rendered_headings() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{First}\\subsection{Second}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    let toc_placeholder = text.find("!toc").expect("toc placeholder should exist");
    let first_heading = text.find("@S {First}").expect("section heading should exist");
    let second_heading = text.find("@s {Second}").expect("subsection heading should exist");
    assert!(
        toc_placeholder < first_heading && first_heading < second_heading,
        "toc placeholder and headings should preserve deterministic order: body={text:?}"
    );
}

#[test]
fn typeset_minimal_toc_preserves_duplicate_heading_titles_with_unique_anchors() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\section{Repeat}\\subsection{Repeat}\\section{Repeat}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("!toc 1 1 Repeat"), "body={text:?}");
    assert!(text.contains("!toc 2 2 Repeat"), "body={text:?}");
    assert!(text.contains("!toc 1 3 Repeat"), "body={text:?}");
}

#[test]
fn typeset_minimal_toc_anchor_ids_follow_document_order_even_when_levels_change() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\tableofcontents\\subsection{Early sub}\\section{Later section}\\subsection{Last sub}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    let first = text.find("!toc 2 1 Early sub").expect("first toc entry");
    let second = text
        .find("!toc 1 2 Later section")
        .expect("second toc entry");
    let third = text.find("!toc 2 3 Last sub").expect("third toc entry");
    assert!(first < second && second < third, "body={text:?}");
}

#[test]
fn typeset_minimal_labels_and_refs_emit_metadata_and_resolve_inline_values() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\section{Intro}\\label{sec:intro}See \\ref{sec:intro} and \\ref{sec:missing}.\\begin{figure}\\caption{Figure caption}\\end{figure}\\label{fig:cap}Figure \\ref{fig:cap}.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("See 1 and ??."), "body={text:?}");
    assert!(text.contains("Figure 2."), "body={text:?}");
    assert!(text.contains("!l sec:intro 1 heading 1 Intro"), "body={text:?}");
    assert!(text.contains("!l fig:cap 2 figure 0 -"), "body={text:?}");
    assert!(text.contains("!r sec:intro "), "body={text:?}");
    assert!(text.contains("!r sec:missing "), "body={text:?}");
    assert!(text.contains("!r fig:cap "), "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_label_not_immediately_after_heading_or_figure() {
    let main = b"\\documentclass{article}\\begin{document}\\section{Intro}Body first.\\label{sec:intro}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_duplicate_label_keys() {
    let main = b"\\documentclass{article}\\begin{document}\\section{A}\\label{dup:key}\\subsection{B}\\label{dup:key}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_bibliography_and_cites_emit_block_and_metadata() {
    let main = b"\\documentclass{article}\\begin{document}See \\cite{ref:a} and \\cite{missing}.\\begin{thebibliography}{9}\\bibitem{ref:a}Alpha source text.\\end{thebibliography}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("See [1] and [?]."), "body={text:?}");
    assert!(text.contains("@S {References}"), "body={text:?}");
    assert!(text.contains("[1] Alpha source text."), "body={text:?}");
    assert!(text.contains("!b ref:a 1 18"), "body={text:?}");
    assert!(text.contains("!c ref:a "), "body={text:?}");
    assert!(text.contains("!c missing "), "body={text:?}");
}

#[test]
fn typeset_minimal_rejects_bibliography_command_stub() {
    let main = b"\\documentclass{article}\\begin{document}\\bibliography{refs}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_bibliographystyle_command_stub() {
    let main = b"\\documentclass{article}\\begin{document}\\bibliographystyle{plain}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_thebibliography_without_bibitem() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\end{thebibliography}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_label_with_unsafe_key_bytes() {
    let main = b"\\documentclass{article}\\begin{document}\\section{A}\\label{../bad}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_heading_at_start_has_no_leading_blank_lines() {
    let main =
        b"\\documentclass{article}\\begin{document}\\paragraph{Lead in}Body text.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.starts_with("@s {Lead in}\n\n~ Body text."), "body={text:?}");
}

#[test]
fn typeset_minimal_first_paragraph_after_heading_uses_noindent_marker() {
    let main = b"\\documentclass{article}\n\\begin{document}\n\\section{Intro}First paragraph.\\par Second paragraph.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("@S {Intro}\n\n~ First paragraph.\n\nSecond paragraph."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_lists_emit_expected_itemize_and_enumerate_lines() {
    let main = b"\\documentclass{article}\\begin{document}Before.\\begin{itemize}\\item First \\emph{item}\\item Second item\\end{itemize}\\begin{enumerate}\\item One\\item Two\\end{enumerate}After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n- First [item]\n- Second item\n\n1. One\n2. Two\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_enumerate_emits_double_digit_items() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{enumerate}\\item One\\item Two\\item Three\\item Four\\item Five\\item Six\\item Seven\\item Eight\\item Nine\\item Ten\\end{enumerate}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("9. Nine"), "body={text:?}");
    assert!(text.contains("10. Ten"), "body={text:?}");
}

#[test]
fn typeset_minimal_inline_wrapper_boundaries_preserve_expected_spacing() {
    let main = b"\\documentclass{article}\\begin{document}word\\emph{mid}word word \\emph{lead} trail,\\textbf{bold}!\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("word[mid]word word [lead] trail,{bold}!"),
        "body={text:?}"
    );
    assert!(!text.contains("word [mid]word"), "body={text:?}");
    assert!(!text.contains("word[lead]"), "body={text:?}");
}

#[test]
fn typeset_minimal_accepts_single_level_nested_lists() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{itemize}\\item Outer item\\begin{enumerate}\\item Inner one\\item Inner two\\end{enumerate}\\item Outer tail\\end{itemize}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("- Outer item\n\n  1. Inner one\n  2. Inner two\n\n- Outer tail"),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_lists_nested_deeper_than_one_level() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{itemize}\\item Outer\\begin{enumerate}\\item Inner\\begin{itemize}\\item Too deep\\end{itemize}\\end{enumerate}\\end{itemize}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_quote_environment_prefixes_each_line() {
    let main = b"\\documentclass{article}\n\\begin{document}\nBefore.\n\\begin{quote}\nQuoted one\\linebreak Quoted two\n\nNew paragraph\n\\end{quote}\nAfter.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n> Quoted one\n> Quoted two\n\n> New paragraph\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_heading_list_quote_rhythm_markers_are_stable() {
    let main = b"\\documentclass{article}\\begin{document}\\section{Heading}After heading.\\begin{itemize}\\item First item\\item Second item\\end{itemize}\\begin{quote}Quoted line one\\linebreak Quoted line two\\end{quote}After quote.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("@S {Heading}\n\n~ After heading.\n\n- First item\n- Second item\n\n> Quoted line one\n> Quoted line two\n\nAfter quote."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_nested_quote_environment() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{quote}Outer\\begin{quote}Inner\\end{quote}\\end{quote}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_center_environment_prefixes_each_line() {
    let main = b"\\documentclass{article}\n\\begin{document}\nBefore.\n\\begin{center}\nCentered one\\linebreak Centered two\n\nCentered paragraph\n\\end{center}\nAfter.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n^ Centered one\n^ Centered two\n\n^ Centered paragraph\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_centerline_emits_single_centered_line() {
    let main = b"\\documentclass{article}\\begin{document}Before.\\centerline{A \\emph{B}}After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Before.\n\n^ A [B]\n\nAfter."), "body={text:?}");
}

#[test]
fn typeset_minimal_centerline_rejects_multiline_content() {
    let main =
        b"\\documentclass{article}\\begin{document}\\centerline{A\\\\B}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_flushright_environment_prefixes_each_line() {
    let main = b"\\documentclass{article}\n\\begin{document}\nBefore.\n\\begin{flushright}\nRight one\\linebreak Right two\n\nRight paragraph\n\\end{flushright}\nAfter.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n| Right one\n| Right two\n\n| Right paragraph\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rightline_emits_single_right_aligned_line() {
    let main = b"\\documentclass{article}\\begin{document}Before.\\rightline{A \\textbf{B}}After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Before.\n\n| A {B}\n\nAfter."), "body={text:?}");
}

#[test]
fn typeset_minimal_rightline_rejects_multiline_content() {
    let main = b"\\documentclass{article}\\begin{document}\\rightline{A\\\\B}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_tabular_emits_deterministic_row_markers() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{tabular}{lcr}Left & Center & Right\\\\L2 & C2 & R2\\\\\\end{tabular}\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("!t Left||Center||Right\n!t L2||C2||R2"),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_tabular_unsupported_alignment() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{tabular}{ll}A & B\\\\\\end{tabular}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_tabular_missing_row_terminator() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{tabular}{lcr}A & B & C\\end{tabular}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_figure_stub_emits_placeholder_and_caption_markers() {
    let main = b"\\documentclass{article}\\begin{document}Before.\\begin{figure}\\caption{Demo figure caption with \\emph{emphasis}.}\\end{figure}After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n!gbox\n!gcap Demo figure caption with [emphasis].\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_figure_with_includegraphics() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{figure}\\includegraphics{demo.png}\\caption{Nope}\\end{figure}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_figure_without_caption() {
    let main = b"\\documentclass{article}\\begin{document}\\begin{figure}\\end{figure}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_footnotes_and_href_emit_expected_markers() {
    let main = b"\\documentclass{article}\\begin{document}Body text\\footnote{First note}. Visit \\href{https://example.com}{example link}.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Body text^1. Visit <{example link}>."),
        "body={text:?}"
    );
    assert!(text.contains("!f 1 First note"), "body={text:?}");
    assert!(
        text.contains("!u 1 https://example.com"),
        "href url metadata line missing: body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_nested_footnote_content() {
    let main = b"\\documentclass{article}\\begin{document}A\\footnote{outer \\footnote{inner}}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_href_with_unsupported_url_tokens() {
    let main = b"\\documentclass{article}\\begin{document}\\href{\\bad}{text}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_malformed_href_missing_text_group() {
    let main = b"\\documentclass{article}\\begin{document}\\href{https://example.com}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_keeps_multiple_footnotes_in_order() {
    let main = b"\\documentclass{article}\\begin{document}A\\footnote{First note} B\\footnote{Second note}.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("A^1 B^2."),
        "inline markers should be emitted in-order: body={text:?}"
    );
    assert!(
        text.contains("!f 1 First note"),
        "first footnote line missing: body={text:?}"
    );
    assert!(
        text.contains("!f 2 Second note"),
        "second footnote line missing: body={text:?}"
    );
}

#[test]
fn typeset_minimal_long_paragraph_wraps_to_multiple_lines() {
    let main = b"\\documentclass{article}\\begin{document}This is a long paragraph that should wrap deterministically to multiple physical lines in the minimal typeset pipeline when width-based layout is enabled and the content exceeds the configured line width for the page body area.\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    let layout =
        parse_dvi_v2_text_page_to_layout_v0(&result.main_xdv_bytes, 917_504).expect("layout parse");
    assert!(
        layout.pages.first().map(|page| page.lines.len()).unwrap_or(0) >= 2,
        "expected wrapped output with multiple lines"
    );
}

#[test]
fn typeset_minimal_single_newline_collapses_to_space() {
    let main = b"\\documentclass{article}\\begin{document}Hello,\nworld.\\end{document}";
    let lines = layout_lines_bytes(main);
    assert_eq!(lines[0], b"Hello, world.");
}

#[test]
fn typeset_minimal_punctuation_does_not_cross_paragraph_break() {
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello,\n\nworld.\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Hello,\n\nworld."), "body={text:?}");
    assert!(!text.contains("Hello, world."), "body={text:?}");
}

#[test]
fn typeset_minimal_punctuation_collapses_following_spaces_to_one() {
    let main = b"\\documentclass{article}\\begin{document}Hello,    world.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Hello, world."), "body={text:?}");
    assert!(!text.contains("Hello,  world."), "body={text:?}");
}

#[test]
fn typeset_minimal_punctuation_removes_spaces_before_markers_and_punctuation() {
    let main = b"\\documentclass{article}\\begin{document}lead\\emph{core} , trail and lead\\textbf{core} ! done.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("lead[core], trail and lead{core}! done."), "body={text:?}");
    assert!(!text.contains("[core] ,"), "body={text:?}");
    assert!(!text.contains("{core} !"), "body={text:?}");
}

#[test]
fn typeset_minimal_tex_double_quotes_normalize_to_ascii_quote() {
    let main = b"\\documentclass{article}\\begin{document}``Hello''\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("\"Hello\""), "body={text:?}");
    assert!(!text.contains("``"), "body={text:?}");
    assert!(!text.contains("''"), "body={text:?}");
}

#[test]
fn typeset_minimal_tex_dashes_normalize_to_unicode_dashes() {
    let main = b"\\documentclass{article}\\begin{document}A---B--C\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("A—B–C"), "body={text:?}");
    assert!(!text.contains("---"), "body={text:?}");
    assert!(!text.contains("--"), "body={text:?}");
}

#[test]
fn typeset_minimal_tex_ellipsis_normalizes_to_unicode_ellipsis() {
    let main = b"\\documentclass{article}\\begin{document}Wait... done.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Wait… done."), "body={text:?}");
    assert!(!text.contains("..."), "body={text:?}");
}

#[test]
fn typeset_minimal_parentheses_remove_inner_spaces_same_line() {
    let main = b"\\documentclass{article}\\begin{document}( A )\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("(A)"), "body={text:?}");
    assert!(!text.contains("( A )"), "body={text:?}");
}

#[test]
fn typeset_minimal_parentheses_do_not_strip_across_hard_newline() {
    let main = b"\\documentclass{article}\n\\begin{document}\n( \\newline A )\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("(\nA)"), "body={text:?}");
}

#[test]
fn typeset_minimal_brackets_and_braces_remove_inner_spaces_same_line() {
    let main = b"\\documentclass{article}\\begin{document}\\emph{ A } \\textbf{ B }\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("[A] {B}"), "body={text:?}");
    assert!(!text.contains("[ A ]"), "body={text:?}");
    assert!(!text.contains("{ B }"), "body={text:?}");
}

#[test]
fn typeset_minimal_wrapper_markers_trim_inner_spaces_without_newlines() {
    let main = b"\\documentclass{article}\\begin{document}\\emph{   lead   } and \\textbf{  core  }\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("[lead] and {core}"), "body={text:?}");
    assert!(!text.contains("[ lead"), "body={text:?}");
    assert!(!text.contains("core }"), "body={text:?}");
}

#[test]
fn typeset_minimal_nested_wrapper_boundary_spacing_is_stable() {
    let main = b"\\documentclass{article}\\begin{document}word\\emph{\\textbf{ mid }}word and word\\textbf{\\emph{ core }} ,trail\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("word[{mid}]word and word{[core]},trail"), "body={text:?}");
    assert!(!text.contains("word [{mid}]word"), "body={text:?}");
    assert!(!text.contains("} ,trail"), "body={text:?}");
}

#[test]
fn typeset_minimal_brackets_and_braces_do_not_strip_across_hard_newline() {
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\emph{\\textbf{ \\newline A }}\n\\end{document}\n";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("[{\nA}]"), "body={text:?}");
}

#[test]
fn typeset_minimal_double_backslash_emits_hard_newline() {
    let main = b"\\documentclass{article}\\begin{document}Hello\\\\world.\\end{document}";
    let lines = layout_lines_bytes(main);
    assert!(lines.len() >= 2, "expected at least two lines, got {:?}", lines);
    assert_eq!(lines[0], b"Hello");
    assert_eq!(lines[1], b"world.");
}

#[test]
fn typeset_minimal_newline_alias_emits_hard_newline() {
    let main = b"\\documentclass{article}\n\\begin{document}\nHello\\newline world.\n\\end{document}\n";
    let lines = layout_lines_bytes(main);
    assert!(lines.len() >= 2, "expected at least two lines, got {:?}", lines);
    assert_eq!(lines[0], b"Hello");
    assert_eq!(lines[1], b"world.");
}

#[test]
fn typeset_minimal_linebreak_alias_emits_hard_newline() {
    let main = b"\\documentclass{article}\\begin{document}Hello\\linebreak world.\\end{document}";
    let lines = layout_lines_bytes(main);
    assert!(lines.len() >= 2, "expected at least two lines, got {:?}", lines);
    assert_eq!(lines[0], b"Hello");
    assert_eq!(lines[1], b"world.");
}

#[test]
fn typeset_minimal_pagebreak_alias_emits_forced_page_split() {
    let main = b"\\documentclass{article}\\begin{document}A\\pagebreak B\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    let layout =
        parse_dvi_v2_text_page_to_layout_v0(&result.main_xdv_bytes, 917_504).expect("layout parse");
    assert_eq!(layout.pages.len(), 2);
    let first_line: Vec<u8> = layout.pages[0].lines[0]
        .glyphs
        .iter()
        .map(|glyph| glyph.byte)
        .collect();
    let second_line: Vec<u8> = layout.pages[1].lines[0]
        .glyphs
        .iter()
        .map(|glyph| glyph.byte)
        .collect();
    assert_eq!(first_line, b"A");
    assert_eq!(second_line, b"B");
}

#[test]
fn typeset_minimal_inline_math_emits_placeholder_text() {
    let main = b"\\documentclass{article}\\begin{document}Before $x + y$ after.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(text.contains("Before MATH after."), "body={text:?}");
}

#[test]
fn typeset_minimal_display_math_emits_centered_placeholder_block() {
    let main =
        b"\\documentclass{article}\\begin{document}Before.\\[x+y\\]After.\\end{document}";
    let body = extract_typeset_body(main);
    let text = String::from_utf8(body).expect("body should be valid utf8");
    assert!(
        text.contains("Before.\n\n^ MATH DISPLAY\n\nAfter."),
        "body={text:?}"
    );
}

#[test]
fn typeset_minimal_rejects_inline_math_with_control_sequence_payload() {
    let main = b"\\documentclass{article}\\begin{document}$\\alpha$\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unterminated_inline_math() {
    let main = b"\\documentclass{article}\\begin{document}A $x+y\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unterminated_display_math() {
    let main = b"\\documentclass{article}\\begin{document}A \\[x+y\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
