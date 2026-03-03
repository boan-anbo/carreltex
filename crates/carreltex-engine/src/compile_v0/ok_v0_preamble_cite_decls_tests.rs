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
fn bibpunct_preamble_is_ok_and_output_matches_without_it() {
    let with_bibpunct = compile_main(
        b"\\documentclass{article}\\bibpunct{(}{)}{;}{a}{,}{,}\\begin{document}\\cite{X}\\end{document}",
    );
    let without_bibpunct =
        compile_main(b"\\documentclass{article}\\begin{document}\\cite{X}\\end{document}");
    assert_eq!(with_bibpunct.status, CompileStatus::Ok);
    assert_eq!(without_bibpunct.status, CompileStatus::Ok);
    assert!(with_bibpunct.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_bibpunct.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_bibpunct),
        right3_positive_total(&without_bibpunct)
    );
}

#[test]
fn crefname_preamble_is_ok_and_output_matches_without_it() {
    let with_crefname = compile_main(
        b"\\documentclass{article}\\crefname{equation}{Eq.}{Eqs.}\\begin{document}\\begin{equation}x\\cref{k}\\end{equation}\\end{document}",
    );
    let without_crefname = compile_main(
        b"\\documentclass{article}\\begin{document}\\begin{equation}x\\cref{k}\\end{equation}\\end{document}",
    );
    assert_eq!(with_crefname.status, CompileStatus::Ok);
    assert_eq!(without_crefname.status, CompileStatus::Ok);
    assert!(with_crefname.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_crefname.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_crefname),
        right3_positive_total(&without_crefname)
    );
}

#[test]
fn setcitestyle_preamble_is_ok_and_output_matches_without_it() {
    let with_setcitestyle = compile_main(
        b"\\documentclass{article}\\setcitestyle{authoryear{,}}\\begin{document}\\cite{X}\\end{document}",
    );
    let without_setcitestyle =
        compile_main(b"\\documentclass{article}\\begin{document}\\cite{X}\\end{document}");
    assert_eq!(with_setcitestyle.status, CompileStatus::Ok);
    assert_eq!(without_setcitestyle.status, CompileStatus::Ok);
    assert!(with_setcitestyle.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_setcitestyle.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_setcitestyle),
        right3_positive_total(&without_setcitestyle)
    );
}

#[test]
fn bibpunct_missing_required_groups_is_not_implemented() {
    let result =
        compile_main(b"\\documentclass{article}\\bibpunct{(}{)}\\begin{document}X\\end{document}");
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
