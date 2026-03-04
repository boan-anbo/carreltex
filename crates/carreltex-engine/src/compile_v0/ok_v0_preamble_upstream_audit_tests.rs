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
fn preamble_fancyhdr_canonical_bundle_is_ok_and_output_matches_baseline() {
    let with_forms = compile_main(
        b"\\documentclass{article}\\usepackage{fancyhdr}\\newcommand{\\headrulewidth}{0pt}\\newcommand{\\footrulewidth}{0pt}\\pagestyle{fancy}\\fancyhead[LE,RO]{\\leftmark}\\fancyfoot[CE]{\\thepage}\\fancypagestyle{plain}{\\fancyhf{}\\fancyhead[RO]{X}}\\renewcommand{\\headrulewidth}{0pt}\\setlength{\\headheight}{14pt}\\begin{document}HelloWorld\\end{document}",
    );
    let baseline = compile_main(
        b"\\documentclass{article}\\usepackage{fancyhdr}\\newcommand{\\headrulewidth}{0pt}\\newcommand{\\footrulewidth}{0pt}\\begin{document}HelloWorld\\end{document}",
    );
    assert_eq!(with_forms.status, CompileStatus::Ok);
    assert_eq!(baseline.status, CompileStatus::Ok);
    assert!(with_forms.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_forms.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_forms),
        right3_positive_total(&baseline)
    );
}

#[test]
fn preamble_fancypagestyle_star_form_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\fancypagestyle*{plain}{\\fancyhf{}}\\begin{document}HelloWorld\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn preamble_fancyhead_missing_group_is_not_implemented() {
    let result = compile_main(
        b"\\documentclass{article}\\fancyhead[LE,RO]\\begin{document}HelloWorld\\end{document}",
    );
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
