use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
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

fn stats_u64_field(stats_json: &str, field: &str) -> Option<u64> {
    let marker = format!("\"{field}\":");
    let start = stats_json.find(&marker)? + marker.len();
    let rest = &stats_json[start..];
    let digits_len = rest
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digits_len == 0 {
        return None;
    }
    rest[..digits_len].parse::<u64>().ok()
}

fn baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

fn assert_wrapper_ok(main: &[u8], expected_char_delta: u64, expected_total: u32) {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + expected_char_delta);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, expected_total);
}

#[test]
fn mbox_preserves_inner_text_ok() {
    let main = b"\\documentclass{article}\\begin{document}\\mbox{A B}C\\end{document}";
    assert_wrapper_ok(main, 3, 229_376);
}

#[test]
fn fbox_preserves_inner_text_ok() {
    let main = b"\\documentclass{article}\\begin{document}\\fbox{AB}\\end{document}";
    assert_wrapper_ok(main, 2, 131_072);
}

#[test]
fn makebox_with_optional_brackets_ok() {
    let main = b"\\documentclass{article}\\begin{document}\\makebox[1cm]{A}B\\end{document}";
    assert_wrapper_ok(main, 7, 131_072);
}

#[test]
fn framebox_with_two_optional_brackets_ok() {
    let main = b"\\documentclass{article}\\begin{document}\\framebox[1cm][c]{A}B\\end{document}";
    assert_wrapper_ok(main, 10, 131_072);
}

#[test]
fn hbox_immediate_group_ok() {
    let main = b"\\documentclass{article}\\begin{document}\\hbox{AB}\\end{document}";
    assert_wrapper_ok(main, 2, 131_072);
}

#[test]
fn mbox_missing_group_is_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\mbox X\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
