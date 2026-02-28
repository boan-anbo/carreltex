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

fn hello_baseline_char_count() -> u64 {
    let mut mount = Mount::default();
    let main = b"\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n";
    assert!(mount.add_file(b"main.tex", main).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count")
}

fn assert_control_word_delta(control_word: &str, expected_delta: u64) {
    let baseline = hello_baseline_char_count();
    let mut mount = Mount::default();
    let main = format!(
        "\\documentclass{{article}}\n\\begin{{document}}\nHello.\\{control_word} XYZ\n\\end{{document}}\n"
    );
    assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());
    let result = compile_request_v0(&mut mount, &valid_request());
    assert_eq!(result.status, CompileStatus::Ok);
    let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
    assert_eq!(char_count, baseline + expected_delta);
}

#[test]
fn control_words_leaf_141_are_counted_with_expected_deltas() {
    for (control_word, expected_delta) in [
        ("textonehalf", 6),
        ("textonequarter", 6),
        ("textthreequarters", 6),
        ("texttimes", 4),
        ("textdiv", 4),
        ("textpm", 5),
        ("textdag", 4),
        ("textbardbl", 5),
        ("textasciiacute", 4),
        ("textasciidblquote", 4),
    ] {
        assert_control_word_delta(control_word, expected_delta);
    }
}
