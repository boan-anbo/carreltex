use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
use carreltex_xdv::{
    sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_text_page_v0,
};

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

fn right3_total(bytes: &[u8]) -> u32 {
    sum_dvi_v2_positive_right3_amounts_with_layout_v0(bytes, 65_536, 786_432)
        .expect("sum parser should parse")
}

#[test]
fn caption_short_title_optional_arg_is_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\caption[short]{X}B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(right3_total(&result.main_xdv_bytes), 196_608);
}

#[test]
fn caption_star_short_title_optional_arg_is_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\caption*[short]{X}B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(right3_total(&result.main_xdv_bytes), 196_608);
}

#[test]
fn footnote_optional_number_is_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\footnote[1]{X}B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(right3_total(&result.main_xdv_bytes), 360_448);
}

#[test]
fn footnotemark_optional_number_is_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\footnotemark[2]B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(right3_total(&result.main_xdv_bytes), 131_072);
}

#[test]
fn footnote_optional_number_rejects_non_digits() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\footnote[ab]{X}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
}
