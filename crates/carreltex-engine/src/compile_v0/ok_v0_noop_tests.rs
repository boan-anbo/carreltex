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

fn assert_noop_command_delta_two_chars_ok(command: &[u8]) {
    let mut baseline_mount = Mount::default();
    let mut baseline_main = b"\\documentclass{article}\\begin{document}".to_vec();
    baseline_main.extend_from_slice(command);
    baseline_main.extend_from_slice(b"\\end{document}");
    assert!(baseline_mount.add_file(b"main.tex", &baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let mut main = b"\\documentclass{article}\\begin{document}A".to_vec();
    main.extend_from_slice(command);
    main.extend_from_slice(b"B\\end{document}");
    assert!(mount.add_file(b"main.tex", &main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 2);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 131_072);
}

#[test]
fn phantomsection_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\phantomsection ");
}

#[test]
fn bibliographystyle_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\bibliographystyle{plain}");
}

#[test]
fn bibliography_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\bibliography{refs}");
}

#[test]
fn nocite_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\nocite{X,Y}");
}

#[test]
fn addcontentsline_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\addcontentsline{toc}{section}{Foo}");
}

#[test]
fn addtocontents_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\addtocontents{toc}{Foo}");
}

#[test]
fn markboth_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\markboth{L}{R}");
}

#[test]
fn markright_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\markright{R}");
}

#[test]
fn thispagestyle_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\thispagestyle{plain}");
}

#[test]
fn pagestyle_in_body_is_noop_ok() {
    assert_noop_command_delta_two_chars_ok(b"\\pagestyle{plain}");
}

#[test]
fn markright_missing_arg_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\markright X\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
