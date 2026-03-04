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
fn preamble_noarg_page_style_bundle_is_ok_and_output_matches_without_commands() {
    let with_commands = compile_main(
        b"\\documentclass{article}\\makeindex\\frontmatter\\mainmatter\\backmatter\\pagenumbering\\pagestyle\\thispagestyle\\begin{document}HelloWorld\\end{document}",
    );
    let without_commands =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_commands.status, CompileStatus::Ok);
    assert_eq!(without_commands.status, CompileStatus::Ok);
    assert!(with_commands.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_commands.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_commands),
        right3_positive_total(&without_commands)
    );
}

#[test]
fn preamble_page_style_arg_form_is_accepted_for_ok() {
    let with_command = compile_main(
        b"\\documentclass{article}\\pagestyle{plain}\\begin{document}HelloWorld\\end{document}",
    );
    let without_command =
        compile_main(b"\\documentclass{article}\\begin{document}HelloWorld\\end{document}");
    assert_eq!(with_command.status, CompileStatus::Ok);
    assert_eq!(without_command.status, CompileStatus::Ok);
    assert!(with_command.log_bytes.is_empty());
    assert!(validate_dvi_v2_text_page_v0(&with_command.main_xdv_bytes));
    assert_eq!(
        right3_positive_total(&with_command),
        right3_positive_total(&without_command)
    );
}
