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

fn assert_state_noop_ok(main: &[u8]) {
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

fn assert_state_noop_not_implemented(main: &[u8]) {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn setcounter_is_noop_ok() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\setcounter{enumi}{2}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn addtocounter_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}Hello\\addtocounter{enumi}{3}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn stepcounter_is_noop_ok() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\stepcounter{enumi}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn refstepcounter_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}Hello\\refstepcounter{equation}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn setlength_is_noop_ok() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\setlength{\\parskip}{1em}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn addtolength_is_noop_ok() {
    let main = b"\\documentclass{article}\\begin{document}Hello\\addtolength{\\parindent}{2em}World\\end{document}";
    assert_state_noop_ok(main);
}

#[test]
fn setcounter_missing_value_is_not_implemented() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\setcounter{enumi}World\\end{document}";
    assert_state_noop_not_implemented(main);
}

#[test]
fn setcounter_non_digit_value_is_not_implemented() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\setcounter{enumi}{ab}World\\end{document}";
    assert_state_noop_not_implemented(main);
}

#[test]
fn setlength_without_controlseq_register_is_not_implemented() {
    let main =
        b"\\documentclass{article}\\begin{document}Hello\\setlength{parskip}{1em}World\\end{document}";
    assert_state_noop_not_implemented(main);
}
