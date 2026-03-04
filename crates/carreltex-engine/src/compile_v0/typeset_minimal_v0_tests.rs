use super::compile_main_typeset_minimal_v0;
use carreltex_core::{CompileStatus, Mount};

fn compile_typeset(main: &[u8]) -> carreltex_core::CompileResultV0 {
    let mut mount = Mount::default();
    mount
        .add_file(b"main.tex", main)
        .expect("main.tex should mount");
    compile_main_typeset_minimal_v0(&mut mount)
}

#[test]
fn typeset_minimal_subset_compiles_ok() {
    let main = b"\\documentclass{article}\\title{CarrelTeX Minimal Typeset Demo}\\author{Alice \\and Bob}\\date{2026-03-04}\\begin{document}\\maketitle Hello, world. This is a paragraph with \\emph{emphasis} and \\textbf{bold}.\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::Ok);
    assert!(!result.main_xdv_bytes.is_empty());
    assert!(result.log_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unsupported_control_sequence() {
    let main = b"\\documentclass{article}\\begin{document}\\section{X}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}

#[test]
fn typeset_minimal_rejects_unsupported_wrapper_in_body() {
    let main = b"\\documentclass{article}\\begin{document}A\\textit{B}\\end{document}";
    let result = compile_typeset(main);
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
