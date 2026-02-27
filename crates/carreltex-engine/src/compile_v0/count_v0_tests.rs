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
fn count0_assignment_then_the_emits_decimal_chars() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\n\\count0=12\\the\\count0\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 2);
}

#[test]
fn the_count1_without_assignment_defaults_to_zero() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\the\\count1\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 1);
}

#[test]
fn count_assignment_rejects_negative_values() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\count0=-1").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_count_assignment_unsupported"));
}

#[test]
fn the_rejects_unsupported_form() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\the{}").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_the_unsupported"));
}
