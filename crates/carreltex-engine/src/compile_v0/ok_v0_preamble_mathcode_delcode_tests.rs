use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileResultV0, CompileStatus, Mount};
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

fn compile_main(main: &[u8]) -> CompileResultV0 {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    compile_request_v0(&mut mount, &valid_request())
}

fn right3_positive_total(result: &CompileResultV0) -> u32 {
    sum_dvi_v2_positive_right3_amounts_with_layout_v0(&result.main_xdv_bytes, 65_536, 786_432)
        .expect("sum parser should parse")
}

#[test]
fn mathcode_delcode_preamble_assignments_are_ok_and_output_matches_without_them() {
    let with_assignments = compile_main(
        b"\\documentclass{article}\\mathcode A = 123\\delcode B = 456\\begin{document}HelloWorld\\end{document}",
    );
    let without_assignments =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_assignments.status, CompileStatus::Ok);
    assert_eq!(without_assignments.status, CompileStatus::Ok);
    assert!(with_assignments.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_assignments.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_assignments),
        right3_positive_total(&without_assignments)
    );
}

#[test]
fn mathcode_missing_equals_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\mathcode A 123\\begin{document}HelloWorld\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn delcode_non_digit_assignment_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\delcode B = X\\begin{document}HelloWorld\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
