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
fn body_protect_is_transparent_for_ok_output() {
    let with_protect =
        compile_main(b"\\documentclass{article}\\begin{document}A\\protect\\cite{X}B\\end{document}");
    let without_protect =
        compile_main(b"\\documentclass{article}\\begin{document}A\\cite{X}B\\end{document}");
    assert_eq!(with_protect.status, CompileStatus::Ok);
    assert_eq!(without_protect.status, CompileStatus::Ok);
    assert!(with_protect.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_protect.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_protect),
        right3_positive_total(&without_protect)
    );
}

#[test]
fn body_relax_is_transparent_for_ok_output() {
    let with_relax =
        compile_main(b"\\documentclass{article}\\begin{document}A\\relax B\\end{document}");
    let without_relax = compile_main(b"\\documentclass{article}\\begin{document}AB\\end{document}");
    assert_eq!(with_relax.status, CompileStatus::Ok);
    assert_eq!(without_relax.status, CompileStatus::Ok);
    assert!(with_relax.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_relax.main_xdv_bytes));
    assert_eq!(right3_positive_total(&with_relax), right3_positive_total(&without_relax));
}

#[test]
fn preamble_protect_relax_are_transparent_for_ok_output() {
    let with_noops = compile_main(
        b"\\documentclass{article}\\protect\\relax\\begin{document}World\\end{document}",
    );
    let without_noops = compile_main(b"\\documentclass{article}\\begin{document}World\\end{document}");
    assert_eq!(with_noops.status, CompileStatus::Ok);
    assert_eq!(without_noops.status, CompileStatus::Ok);
    assert!(with_noops.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_noops.main_xdv_bytes));
    assert_eq!(right3_positive_total(&with_noops), right3_positive_total(&without_noops));
}

#[test]
fn protect_with_group_stays_not_implemented() {
    let result = compile_main(b"\\documentclass{article}\\begin{document}\\protect{X}\\end{document}");
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
