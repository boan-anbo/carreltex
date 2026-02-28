use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};
use carreltex_xdv::validate_dvi_v2_empty_page_v0;

fn valid_request() -> CompileRequestV0 {
    CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 4096,
    }
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
    assert!(validate_dvi_v2_empty_page_v0(&result.main_xdv_bytes));
    assert!(!result.tex_stats_json.is_empty());
}

#[test]
fn non_empty_article_doc_stays_not_implemented() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    assert!(result.main_xdv_bytes.is_empty());
}
