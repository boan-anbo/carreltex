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

#[test]
fn bibliography_env_emits_bullet_markers_and_newline() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem{X}ABC\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 35);
    let movement = count_dvi_v2_text_movements_v0(&result.main_xdv_bytes).expect("movement summary");
    assert!(movement.3 >= 1);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 294_912);
}

#[test]
fn bibitem_outside_env_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\bibitem{X}A\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn bibliography_end_mismatch_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem{X}A\\end{itemize}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn nested_bibliography_env_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\begin{itemize}\\item A\\end{itemize}\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn bibitem_optional_form_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[X]{Y}A\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 36);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 393_216);
}

#[test]
fn bibitem_label_fragment_with_wrappers_and_url_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\textbf{X}\\url{Y}]{K}A\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 37);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 458_752);
}

#[test]
fn bibitem_label_fragment_with_begin_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem[\\begin{itemize}\\item X\\end{itemize}]{K}A\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn bibliography_newblock_is_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}\\begin{thebibliography}{9}\\bibitem{X}A\\newblock B\\end{thebibliography}\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 34);
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, 262_144);
}

#[test]
fn newblock_outside_bibliography_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}A\\newblock B\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn bibliography_preamble_commands_are_accepted_for_ok() {
    let mut baseline_mount = Mount::default();
    let baseline_main =
        b"\\documentclass{article}\\bibliographystyle{plain}\\bibliography{refs}\\begin{document}\\end{document}";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::Ok);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\\bibliographystyle{plain}\\bibliography{refs}\\begin{document}XYZ\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn bibliography_preamble_missing_arg_falls_back_to_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\bibliography\\begin{document}X\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
