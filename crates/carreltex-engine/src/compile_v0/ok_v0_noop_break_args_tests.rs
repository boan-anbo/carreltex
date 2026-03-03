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
    let baseline_total = hello_world_baseline_right3_total();
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, baseline_total);
}

fn assert_break_noop_not_implemented(main: &[u8]) {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn pagebreak_bracket_digit_is_ok() {
    let baseline_total = hello_world_baseline_right3_total();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\pagebreak[2]\\end{document}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(result.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&result.main_xdv_bytes));
    assert!(!result.main_xdv_bytes.is_empty());
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(
        &result.main_xdv_bytes,
        65_536,
        786_432,
    )
    .expect("sum parser should parse");
    assert_eq!(total, baseline_total);
}

#[test]
fn nopagebreak_bracket_digit_is_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\nopagebreak[3]\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn linebreak_bracket_digit_is_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\linebreak[4]\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn nolinebreak_bracket_digit_is_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\nolinebreak[1]\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn goodbreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\goodbreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn filbreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\filbreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn samepage_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\samepage\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn nobreak_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\nobreak\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn break_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\break\\end{document}";
    assert_break_noop_ok(main);
}

#[test]
fn linebreak_non_digit_bracket_is_not_implemented() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\linebreak[ab]\\end{document}";
    assert_break_noop_not_implemented(main);
}

#[test]
fn linebreak_unclosed_bracket_is_not_implemented() {
    let main = b"\\documentclass{article}\\begin{document}HelloWorld\\linebreak[3\\end{document}";
    assert_break_noop_not_implemented(main);
}

#[test]
fn nopagebreak_too_long_bracket_is_not_implemented() {
    let main =
        b"\\documentclass{article}\\begin{document}HelloWorld\\nopagebreak[123456789]\\end{document}";
    assert_break_noop_not_implemented(main);
}
