use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileResultV0, CompileStatus, Mount};
use carreltex_xdv::{sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_text_page_v0};

fn valid_request() -> CompileRequestV0 {
    CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 4096,
        ok_max_line_glyphs_v0: None,
        ok_max_lines_per_page_v0: None,
        ok_line_advance_sp_v0: None,
        ok_glyph_advance_sp_v0: None,
    }
}

fn compile_main(main: &[u8]) -> CompileResultV0 {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    compile_request_v0(&mut mount, &valid_request())
}

fn right3_positive_total(result: &CompileResultV0) -> u32 {
    sum_dvi_v2_positive_right3_amounts_with_layout_v0(&result.main_xdv_bytes, 65_536, 786_432)
        .expect("sum parser should parse")
}

#[test]
fn newtheorem_preamble_is_ok_and_output_matches_without_it() {
    let with_newtheorem = compile_main(
        b"\\documentclass{article}\\newtheorem{theorem}{Theorem}\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}",
    );
    let without_newtheorem =
        compile_main(b"\\documentclass{article}\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}");
    assert_eq!(with_newtheorem.status, CompileStatus::Ok);
    assert_eq!(without_newtheorem.status, CompileStatus::Ok);
    assert!(with_newtheorem.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_newtheorem.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_newtheorem),
        right3_positive_total(&without_newtheorem)
    );
}

#[test]
fn setlength_preamble_is_ok_and_output_matches_without_it() {
    let with_setlength = compile_main(
        b"\\documentclass{article}\\setlength{\\parindent}{1em}\\begin{document}HelloWorld\\end{document}",
    );
    let without_setlength =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_setlength.status, CompileStatus::Ok);
    assert_eq!(without_setlength.status, CompileStatus::Ok);
    assert!(with_setlength.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_setlength.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_setlength),
        right3_positive_total(&without_setlength)
    );
}

#[test]
fn style_declaration_in_preamble_is_ok() {
    let with_style =
        compile_main(b"\\documentclass{article}\\centering\\begin{document}HelloWorld\\end{document}");
    let without_style =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_style.status, CompileStatus::Ok);
    assert_eq!(without_style.status, CompileStatus::Ok);
    assert!(with_style.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_style.main_xdv_bytes));
    assert_eq!(right3_positive_total(&with_style), right3_positive_total(&without_style));
}

#[test]
fn newtheorem_prefix_optional_bracket_is_ok_and_output_matches_without_it() {
    let with_optional = compile_main(
        b"\\documentclass{article}\\newtheorem{theorem}[section]{Theorem}\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}",
    );
    let without_optional =
        compile_main(b"\\documentclass{article}\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}");
    assert_eq!(with_optional.status, CompileStatus::Ok);
    assert_eq!(without_optional.status, CompileStatus::Ok);
    assert!(with_optional.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_optional.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_optional),
        right3_positive_total(&without_optional)
    );
}

#[test]
fn newtheorem_suffix_optional_bracket_is_ok_and_output_matches_without_it() {
    let with_optional = compile_main(
        b"\\documentclass{article}\\newtheorem{theorem}{Theorem}[section]\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}",
    );
    let without_optional =
        compile_main(b"\\documentclass{article}\\begin{document}\\begin{theorem}X\\end{theorem}\\end{document}");
    assert_eq!(with_optional.status, CompileStatus::Ok);
    assert_eq!(without_optional.status, CompileStatus::Ok);
    assert!(with_optional.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_optional.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_optional),
        right3_positive_total(&without_optional)
    );
}

#[test]
fn newtheorem_star_with_optional_bracket_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\newtheorem*{theorem}[section]{Theorem}\\begin{document}X\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
