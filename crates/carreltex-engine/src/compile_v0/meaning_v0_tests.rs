use super::compile_request_v0;
use carreltex_core::{CompileRequestV0, CompileStatus, Mount};

fn valid_request() -> CompileRequestV0 {
    CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 4096,
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

#[test]
fn meaning_macro_binding_emits_macro_descriptor() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\meaning\\foo\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + b"macro:foo".len() as u64);
}

#[test]
fn meaning_sees_macro_defined_via_input_expansion() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\meaning\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + b"macro:foo".len() as u64);
}

#[test]
fn meaning_alias_binding_emits_alias_descriptor() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\let\\bar=\\foo\\meaning\\bar\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + b"alias:bar->foo".len() as u64);
}

#[test]
fn meaning_undefined_binding_emits_undefined_descriptor() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\meaning\\nope\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + b"undefined:nope".len() as u64);
}

#[test]
fn meaning_with_unsupported_tokens_is_invalid() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\meaning{}").is_ok());

    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_meaning_unsupported"));
}

#[test]
fn let_uses_snapshot_semantics_not_dynamic_alias() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{X}\\let\\bar=\\foo\\def\\foo{XYZ}\\bar\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 1);
}

#[test]
fn let_uses_snapshot_semantics_across_input_boundary() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\let\\bar=\\foo\\def\\foo{A}\\bar\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn futurelet_sees_macro_defined_across_input_boundary() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\futurelet\\bar\\noop\\foo\\bar\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn csname_sees_macro_defined_across_input_boundary() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\csname foo\\endcsname\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 3);
}

#[test]
fn string_sees_macro_defined_across_input_boundary() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\string\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 4);
}

#[test]
fn expandafter_sees_macros_defined_across_input_boundary() {
    let mut baseline_mount = Mount::default();
    let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
    let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
    assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
    let baseline_char_count =
        stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\expandafter\\bar\\foo\n\\end{document}\n";
    let sub = b"\\def\\foo{XYZ}\\def\\bar{A}";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline_char_count + 4);
}
