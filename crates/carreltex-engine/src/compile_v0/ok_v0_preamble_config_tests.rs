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
fn hypersetup_preamble_is_ok_and_output_matches_without_it() {
    let with_hypersetup = compile_main(
        b"\\documentclass{article}\\hypersetup{colorlinks=true}\\begin{document}HelloWorld\\end{document}",
    );
    let without_hypersetup =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_hypersetup.status, CompileStatus::Ok);
    assert_eq!(without_hypersetup.status, CompileStatus::Ok);
    assert!(with_hypersetup.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_hypersetup.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_hypersetup),
        right3_positive_total(&without_hypersetup)
    );
}

#[test]
fn graphicspath_preamble_is_ok_and_output_matches_without_it() {
    let with_graphicspath = compile_main(
        b"\\documentclass{article}\\graphicspath{{fig/}{img/}}\\begin{document}HelloWorld\\end{document}",
    );
    let without_graphicspath =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_graphicspath.status, CompileStatus::Ok);
    assert_eq!(without_graphicspath.status, CompileStatus::Ok);
    assert!(with_graphicspath.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_graphicspath.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_graphicspath),
        right3_positive_total(&without_graphicspath)
    );
}

#[test]
fn setlist_with_optional_bracket_preamble_is_ok_and_output_matches_without_it() {
    let with_setlist = compile_main(
        b"\\documentclass{article}\\setlist[itemize]{label=--}\\begin{document}HelloWorld\\end{document}",
    );
    let without_setlist =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_setlist.status, CompileStatus::Ok);
    assert_eq!(without_setlist.status, CompileStatus::Ok);
    assert!(with_setlist.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_setlist.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_setlist),
        right3_positive_total(&without_setlist)
    );
}

#[test]
fn setlist_with_unclosed_bracket_in_preamble_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\setlist[itemize{label=--}\\begin{document}X\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn hypersetup_missing_group_in_preamble_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\hypersetup XYZ\\begin{document}X\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
