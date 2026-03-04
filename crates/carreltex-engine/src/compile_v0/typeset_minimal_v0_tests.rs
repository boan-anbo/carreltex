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
    let main = b"\\documentclass{article}\\begin{document}\\section{X}\\end{document}";
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
