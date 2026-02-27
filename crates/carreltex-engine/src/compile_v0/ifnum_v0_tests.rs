use super::compile_request_v0;
use crate::compile_v0::ifnum_v0::MAX_IF_DEPTH_V0;
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
fn ifnum_true_branch_keeps_tokens() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\count0=1\\count1=2\\ifnum\\count0<\\count1 XYZ\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifnum_false_branch_drops_tokens() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\count0=2\\count1=1\\ifnum\\count0<\\count1 AAA\\else XYZ\\fi\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn ifnum_else_is_invalid() {
    let mut mount = Mount::default();
    assert!(
        mount
            .add_file(b"main.tex", b"\\ifnum\\count0<\\count1 X\\else Y\\else Z\\fi")
            .is_ok()
    );
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_if_else_duplicate"));
}

#[test]
fn ifnum_missing_fi_is_invalid() {
    let mut mount = Mount::default();
    assert!(
        mount
            .add_file(b"main.tex", b"\\ifnum\\count0<\\count1 X")
            .is_ok()
    );
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_if_missing_fi"));
}

#[test]
fn ifnum_else_without_if_is_invalid() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\else").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_if_else_without_if"));
}

#[test]
fn ifnum_depth_cap_is_invalid() {
    let mut main = String::new();
    for _ in 0..=MAX_IF_DEPTH_V0 {
        main.push_str("\\ifnum\\count0=\\count0 ");
    }
    main.push('X');
    for _ in 0..=MAX_IF_DEPTH_V0 {
        main.push_str("\\fi");
    }
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"macro_if_depth_exceeded"));
}

#[test]
fn caret_hex_uppercase_decode_in_document_body_is_counted_in_stats() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nA^^4AB\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn unsupported_caret_form_maps_to_tokenizer_caret_reason() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"A^^ZZB").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"tokenizer_caret_not_supported"));
}

#[test]
fn non_ascii_control_sequence_byte_maps_to_tokenize_failed() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\def\\^^ff{XYZ}").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"tokenize_failed"));
}
