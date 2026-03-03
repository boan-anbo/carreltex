use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
use carreltex_xdv::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0,
    plan_layout_v0, validate_dvi_v2_text_page_matches_layout_v0,
    sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_text_page_v0,
    validate_dvi_v2_text_page_with_layout_v0,
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
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
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
    let main =
        b"\\documentclass{article}\n\\begin{document}\nA  \n\nB\tC\r\nD\rE\n\\end{document}\n";
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
        Some((5, 0, 0, 0, 1))
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
        Some((2, 0, 0, 0, 1))
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
        Some((3, 0, 0, 1, 1))
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
        Some((4, 0, 0, 1, 1))
    );
}

#[test]
fn ok_wi_dot_uses_scaled_per_glyph_widths() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}Wi.\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, (65_536 * 5 / 2) as u32);
}

#[test]
fn ok_glyph_advance_one_remains_ok_for_half_em_glyphs() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}i.\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_glyph_advance_sp_v0 = Some(1);
    let result = compile_request_v0(&mut mount, &request);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_with_layout_v0(
        &result.main_xdv_bytes,
        1,
        786_432,
    ));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(&result.main_xdv_bytes, 1, 786_432)
        .expect("sum parser should parse");
    assert_eq!(total, 2);
}

#[test]
fn ok_long_text_without_newline_wraps_automatically() {
    let mut mount = Mount::default();
    let long_body = "A".repeat(81);
    let main = format!("\\documentclass{{article}}\\begin{{document}}{long_body}\\end{{document}}");
    assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert_eq!(count_dvi_v2_text_pages_v0(&result.main_xdv_bytes), Some(1));
    let movement =
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert!(movement.3 >= 1);
}

#[test]
fn printable_backslash_in_body_is_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nA\\textbackslash B\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn macro_expanded_ok_body_returns_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\\begin{document}\\def\\foo{XYZ}\\foo\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn usepackage_without_options_in_preamble_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main =
        b"\\documentclass{article}\n\\usepackage{amsmath}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\usepackage{amsmath}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn usepackage_with_options_in_preamble_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\usepackage[utf8]{inputenc}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\usepackage[utf8]{inputenc}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn usepackage_with_control_sequence_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\usepackage\\foo{bar}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn documentclass_with_options_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main =
        b"\\documentclass[11pt]{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass[11pt]{article}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn title_author_date_in_preamble_are_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\title{T}\n\\author{A}\n\\date{D}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\title{T}\n\\author{A}\n\\date{D}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn title_with_control_sequence_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\title\\foo{bar}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn maketitle_in_body_is_ignored_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\title{T}\n\\author{A}\n\\date{D}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\title{T}\n\\author{A}\n\\date{D}\n\\begin{document}\n\\maketitle\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn make_title_wrong_case_in_body_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\title{T}\n\\author{A}\n\\date{D}\n\\begin{document}\n\\makeTitle\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn noindent_in_body_is_ignored_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\noindent XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn newline_control_sequence_in_body_is_treated_as_linefeed() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\csname newline\\endcsname B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 2);
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert_eq!(movement.0, 3);
    assert_eq!(movement.3, 1);
    assert_eq!(movement.4, 1);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536 * 2);
}

#[test]
fn wrapper_control_sequence_in_body_is_transparent_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\textbf{B C}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert_eq!(movement.0, 4);
    assert_eq!(movement.3, 0);
    assert_eq!(movement.4, 1);
}

#[test]
fn heading_control_sequence_emits_newline_markers_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\section{ABC}D\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 4);
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert!(movement.3 >= 1);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536 * 4);
}

#[test]
fn wrapper_without_braced_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\textbf XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn bfseries_declaration_is_ignored_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\bfseries XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn em_declaration_is_ignored_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\em XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn textrm_wrapper_is_transparent_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\textrm{A B}C\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert_eq!(movement.0, 4);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 65_536 * 3 + 32_768);
}

#[test]
fn href_wrapper_extracts_only_text_for_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\href{https://example.test/path?q=1}{XYZ}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert_eq!(movement.3, 0);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 196_608);
}

#[test]
fn url_wrapper_extracts_url_text_for_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\url{abc}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 196_608);
}

#[test]
fn href_missing_text_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\href{abc}XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn href_nested_group_text_is_supported_for_ok() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\href{abc}{{A}B{C}}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 196_608);
}

#[test]
fn textsf_without_braced_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\textsf XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn heading_without_braced_argument_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\section XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn begin_end_with_space_before_group_is_accepted() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin {document}\n\n\\end {document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin {document}\nXYZ\n\\end {document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn begin_document_trailing_space_is_accepted() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document} \n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document} \nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn par_control_sequence_in_body_is_treated_as_space() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\csname par\\endcsname B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 2);
    assert_eq!(
        count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary").0,
        3,
    );
}

#[test]
fn partial_control_sequence_in_body_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\partial\n\\end{document}\n";
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

#[test]
fn ok_request_wrap_cap_10_increases_down3_vs_80() {
    let main = b"\\documentclass{article}\\begin{document}word word word word word word word word word word\\end{document}";

    let mut wide_mount = Mount::default();
    assert!(wide_mount.add_file(b"main.tex", main).is_ok());
    let mut wide_request = valid_request();
    wide_request.ok_max_line_glyphs_v0 = Some(80);
    let wide_result = compile_request_v0(&mut wide_mount, &wide_request);
    assert_eq!(wide_result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&wide_result.main_xdv_bytes));
    let wide_down3 = count_dvi_v2_text_movements_v0(&wide_result.main_xdv_bytes)
        .expect("wide movement summary")
        .3;

    let mut narrow_mount = Mount::default();
    assert!(narrow_mount.add_file(b"main.tex", main).is_ok());
    let mut narrow_request = valid_request();
    narrow_request.ok_max_line_glyphs_v0 = Some(10);
    let narrow_result = compile_request_v0(&mut narrow_mount, &narrow_request);
    assert_eq!(narrow_result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&narrow_result.main_xdv_bytes));
    let narrow_down3 = count_dvi_v2_text_movements_v0(&narrow_result.main_xdv_bytes)
        .expect("narrow movement summary")
        .3;

    assert!(narrow_down3 > wide_down3);
}

#[test]
fn ok_request_wrap_cap_one_hard_breaks_text() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}AB\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(1);
    let result = compile_request_v0(&mut mount, &request);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let down3_count = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes)
        .expect("movement summary")
        .3;
    assert_eq!(down3_count, 1);
}

#[test]
fn request_wrap_cap_out_of_range_is_invalid_input() {
    let main = b"\\documentclass{article}\\begin{document}AB\\end{document}";

    let mut low_mount = Mount::default();
    assert!(low_mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(0);
    let result = compile_request_v0(&mut low_mount, &request);
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.main_xdv_bytes.is_empty());
    assert!(result.log_bytes.ends_with(b"request_invalid"));

    let mut high_mount = Mount::default();
    assert!(high_mount.add_file(b"main.tex", main).is_ok());
    request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(257);
    let result = compile_request_v0(&mut high_mount, &request);
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.main_xdv_bytes.is_empty());
    assert!(result.log_bytes.ends_with(b"request_invalid"));
}

#[test]
fn request_max_lines_per_page_one_splits_wrapped_output_into_multiple_pages() {
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\\begin{document}word word word word word word word word word word\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(10);
    request.ok_max_lines_per_page_v0 = Some(1);
    let result = compile_request_v0(&mut mount, &request);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let pages = count_dvi_v2_text_pages_v0(&result.main_xdv_bytes).expect("page count");
    assert!(pages >= 2);
}

#[test]
fn ok_layout_plan_wrapped_and_paged_output_is_valid() {
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\\begin{document}word word word word word word word word word word\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(10);
    request.ok_max_lines_per_page_v0 = Some(1);
    let result = compile_request_v0(&mut mount, &request);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(!result.main_xdv_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let pages = count_dvi_v2_text_pages_v0(&result.main_xdv_bytes).expect("page count");
    assert!(pages >= 2);
}

#[test]
fn ok_output_matches_planned_layout_exactly() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}word word word word word word word word word word\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let mut request = valid_request();
    request.ok_max_line_glyphs_v0 = Some(10);
    request.ok_max_lines_per_page_v0 = Some(1);
    request.ok_line_advance_sp_v0 = Some(786_432);
    request.ok_glyph_advance_sp_v0 = Some(65_536);
    let result = compile_request_v0(&mut mount, &request);
    assert_eq!(result.status, CompileStatus::Ok);
    let planned = plan_layout_v0(
        b"word word word word word word word word word word",
        65_536,
        786_432,
        10,
        1,
    )
    .expect("layout plan");
    assert!(validate_dvi_v2_text_page_matches_layout_v0(
        &result.main_xdv_bytes,
        &planned,
        786_432,
    ));
}
