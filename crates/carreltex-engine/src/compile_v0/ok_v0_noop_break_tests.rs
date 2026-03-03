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

fn hello_world_baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

fn hello_world_baseline_right3_total() -> u32 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    sum_dvi_v2_positive_right3_amounts_with_layout_v0(&result.main_xdv_bytes, 65_536, 786_432)
        .expect("sum parser should parse")
}

fn assert_break_noop_ok(main: &[u8]) {
    let baseline = hello_world_baseline_char_count();
    let baseline_total = hello_world_baseline_right3_total();
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, baseline_total);
}

#[test]
fn newpage_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\newpage\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn linebreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\linebreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn clearpage_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\clearpage\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn nopagebreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\nopagebreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn nolinebreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\nolinebreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn newpage_with_group_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}Hello\\newpage{X}World\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
