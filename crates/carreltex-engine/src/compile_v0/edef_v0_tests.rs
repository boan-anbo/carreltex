use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};

fn valid_request() -> CompileRequestV0 {
    CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 4096,
        ok_max_line_glyphs_v0: None,
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
    rest[..digits_len].parse().ok()
}

fn baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

#[test]
fn edef_expands_body_once_at_definition_time() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\def\\bar{XYZ}\\edef\\foo{\\bar}\\foo\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn edef_snapshot_is_stable_across_input_boundary() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\edef\\foo{\\bar}\\def\\bar{A}\\foo\n\\end{document}\n";
    let sub = b"\\def\\bar{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn edef_is_snapshot_not_dynamic_after_redefinition() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\def\\bar{X}\\edef\\foo{\\bar}\\def\\bar{XYZ}\\foo\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 1);
}

#[test]
fn edef_rejects_parameterized_definition() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\edef\\foo#1{#1}").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_params_unsupported"));
}
