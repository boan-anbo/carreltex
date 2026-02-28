use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
use carreltex_xdv::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0, validate_dvi_v2_text_page_v0,
};

fn valid_request() -> CompileRequestV0 {
    CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 4096,
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

#[test]
fn strict_empty_article_doc_returns_ok_with_valid_xdv() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.tex_stats_json.is_empty());
}

#[test]
fn simple_text_article_doc_returns_ok_with_valid_xdv() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn printable_ascii_text_body_returns_ok_with_valid_xdv() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello, world! 123\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 15);
}

#[test]
fn printable_ascii_tilde_char_is_supported() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n~\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 1);
}

#[test]
fn whitespace_runs_are_normalized_to_single_spaces_in_ok_subset() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nA  \n\nB\tC\r\nD\rE\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 5);
}

#[test]
fn pagebreak_marker_emits_two_pages() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nAB\\pagebreak CD\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(result.main_xdv_bytes.len() > 0);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(count_dvi_v2_text_pages_v0(&result.main_xdv_bytes), Some(2));
}

#[test]
fn ok_text_emits_deterministic_movement_sequence() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}ABCDE\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes),
        Some((2, 1, 1, 0, 1))
    );
}

#[test]
fn ok_text_two_chars_emits_single_right_move_only() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}AB\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes),
        Some((1, 0, 0, 0, 1))
    );
}

#[test]
fn ok_newline_control_word_emits_down3_and_stays_single_page() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\newline B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(count_dvi_v2_text_pages_v0(&result.main_xdv_bytes), Some(1));
    assert_eq!(
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes),
        Some((0, 0, 0, 1, 1))
    );
}

#[test]
fn ok_multichar_newline_control_word_uses_reset_sequence() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}AB\\newline C\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(count_dvi_v2_text_pages_v0(&result.main_xdv_bytes), Some(1));
    assert_eq!(
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes),
        Some((2, 0, 0, 1, 1))
    );
}

#[test]
fn unsupported_char_backslash_in_body_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nA\\textbackslash B\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn control_sequence_in_body_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\foo\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn non_printable_body_char_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n^^1f\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
