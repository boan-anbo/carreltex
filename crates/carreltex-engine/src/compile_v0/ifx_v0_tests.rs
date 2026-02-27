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

fn baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

#[test]
fn ifx_undefined_equals_undefined_keeps_then_branch() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\ifx\\foo\\bar XYZ\\else AAA\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifx_alias_equals_alias_keeps_then_branch() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\let\\a=\\foo\\let\\b=\\foo\\ifx\\a\\b XYZ\\else AAA\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifx_macro_equals_macro_keeps_then_branch() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\def\\a{XYZ}\\def\\b{XYZ}\\ifx\\a\\b XYZ\\else AAA\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifx_macro_not_equal_macro_keeps_else_branch() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\def\\a{X}\\def\\b{XYZ}\\ifx\\a\\b AAA\\else XYZ\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifx_duplicate_else_is_invalid() {
    let mut mount = Mount::default();
    assert!(
        mount
            .add_file(b"main.tex", b"\\ifx\\foo\\bar X\\else Y\\else Z\\fi")
            .is_ok()
    );
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_ifx_else_duplicate"));
}

#[test]
fn ifx_else_without_if_is_invalid() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\ifx\\foo\\bar X\\fi\\else").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_ifx_else_without_if"));
}

#[test]
fn ifx_let_snapshot_not_equal_after_redefine_keeps_else_branch() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{X}\\let\\bar=\\foo\\def\\foo{XYZ}\\ifx\\bar\\foo AAA\\else XYZ\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifx_let_to_undefined_is_equal_to_undefined_control_sequence() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\let\\a=\\b\\ifx\\a\\b XYZ\\else AAA\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}
