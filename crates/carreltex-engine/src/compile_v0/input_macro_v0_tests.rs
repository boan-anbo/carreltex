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
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

#[test]
fn input_then_macro_expansion_order_is_stable() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\input{sub.tex}\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn input_unbraced_without_extension_loads_tex_file() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\input sub\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn input_unbraced_with_explicit_tex_extension_stops_at_control_seq() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\input sub.tex\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn input_unbraced_with_dash_defaults_to_tex_extension() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\input sub-1\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub-1.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn input_braced_without_extension_loads_tex_file() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\input{sub}\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn input_unbraced_missing_tex_file_is_invalid() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input sub\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
    assert!(result.log_bytes.ends_with(b"input_validation_failed"));
}

#[test]
fn input_unbraced_requires_char_only_filename_tokens() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input\\foo\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
    assert!(result.log_bytes.ends_with(b"input_validation_failed"));
}

#[test]
fn input_unbraced_rejects_brace_boundary_after_filename() {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input sub{}\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", b"\\def\\foo{XYZ}").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
    assert!(result.log_bytes.ends_with(b"input_validation_failed"));
}
