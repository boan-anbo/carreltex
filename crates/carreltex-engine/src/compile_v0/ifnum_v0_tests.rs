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

fn hello_baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n";
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
fn ifnum_uses_counts_defined_via_input() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\\ifnum\\count0<\\count1 XYZ\\else AAA\\fi\n\\end{document}\n";
    let sub = b"\\count0=1\\count1=2";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    assert!(mount.add_file(b"sub.tex", sub).is_ok());
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
fn unsupported_accent_control_symbol_maps_to_tokenizer_accent_reason() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\~a").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"tokenizer_accent_not_supported"));
}

#[test]
fn non_ascii_control_sequence_byte_maps_to_specific_reason_token() {
    let mut mount = Mount::default();
    assert!(mount.add_file(b"main.tex", b"\\def\\^^ff{XYZ}").is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::InvalidInput);
    assert!(result.log_bytes.ends_with(b"tokenizer_control_seq_non_ascii"));
}

#[test]
fn unsupported_caret_inside_comment_does_not_fail_and_body_counts_chars() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\n% ^^ZZ\nXYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}

#[test]
fn crlf_in_body_is_normalized_as_single_whitespace_run() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nA\r\nB\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 2);
}

#[test]
fn lone_cr_in_body_is_normalized_as_single_whitespace_run() {
    let baseline = baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nA\rB\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 2);
}

#[test]
fn control_symbol_comma_is_counted_as_space_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\,XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_percent_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\%XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_underscore_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\_XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_hash_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\#XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_dollar_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\$XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_ampersand_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\&XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_lbrace_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\{XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_symbol_rbrace_is_counted_as_literal_char() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\}XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_word_textbackslash_is_counted_as_literal_char_and_space_is_swallowed() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\textbackslash XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_word_textasciitilde_is_counted_as_literal_char_and_space_is_swallowed() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\textasciitilde XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_word_textasciicircum_is_counted_as_literal_char_and_space_is_swallowed() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\textasciicircum XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_word_textquotedbl_is_counted_as_literal_char_and_space_is_swallowed() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main =
        b"\\documentclass{article}\n\\begin{document}\nHello.\\textquotedbl XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 4);
}

#[test]
fn control_word_par_is_counted_as_single_space_with_no_extra_whitespace() {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\\par XYZ\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::NotImplemented);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + 3);
}
