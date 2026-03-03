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
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count")
}

#[test]
fn section_star_heading_is_accepted_for_ok() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\section*{X}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 2);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536);
}

#[test]
fn section_short_title_optional_arg_is_accepted_for_ok() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\section[short]{X}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 8);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536);
}

#[test]
fn paragraph_star_heading_is_accepted_for_ok() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\paragraph*{X}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 2);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536);
}

#[test]
fn heading_missing_short_title_closing_bracket_is_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\section[short{X}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
}
