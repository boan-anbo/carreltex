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

fn assert_maketitle_ok(main: &[u8], expected_char_delta: u64, expected_right3_total: u32) {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
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
    assert_eq!(total, expected_right3_total);
}

#[test]
fn maketitle_emits_title_author_date_ok() {
    let main = b"\\documentclass{article}\\title{T}\\author{A}\\date{D}\\begin{document}\\maketitle\\end{document}";
    assert_maketitle_ok(main, 3, 196_608);
}

#[test]
fn maketitle_emits_only_title_ok() {
    let main = b"\\documentclass{article}\\title{T}\\begin{document}\\maketitle\\end{document}";
    assert_maketitle_ok(main, 1, 65_536);
}

#[test]
fn maketitle_emits_only_author_ok() {
    let main = b"\\documentclass{article}\\author{A}\\begin{document}\\maketitle\\end{document}";
    assert_maketitle_ok(main, 1, 65_536);
}

#[test]
fn maketitle_emits_only_date_ok() {
    let main = b"\\documentclass{article}\\date{D}\\begin{document}\\maketitle\\end{document}";
    assert_maketitle_ok(main, 1, 65_536);
}

#[test]
fn title_missing_group_is_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\title X\\begin{document}\\maketitle\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
