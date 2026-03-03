use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
use carreltex_xdv::{
    count_dvi_v2_text_movements_v0, sum_dvi_v2_positive_right3_amounts_with_layout_v0,
    validate_dvi_v2_text_page_v0,
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

fn assert_inline_math_wrapper_ok(control_word: &str) {
    let mut mount = Mount::default();
    let main = format!("\\documentclass{{article}}\\begin{{document}}A\\{control_word}{{x}}B\\end{{document}}");
    assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert_eq!(movement.0, 9);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 557_056);
}

#[test]
fn textnormal_group_emits_inline_math_marker_ok() {
    assert_inline_math_wrapper_ok("textnormal");
}

#[test]
fn mathrm_group_emits_inline_math_marker_ok() {
    assert_inline_math_wrapper_ok("mathrm");
}

#[test]
fn mathit_group_emits_inline_math_marker_ok() {
    assert_inline_math_wrapper_ok("mathit");
}

#[test]
fn math_text_wrapper_missing_group_is_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\mathrm xB\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
