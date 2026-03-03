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

#[test]
fn inline_dollar_math_emits_inline_marker_ok() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A$x$B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 5);
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
fn unclosed_inline_dollar_math_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A$xB\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
