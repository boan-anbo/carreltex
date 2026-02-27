mod input_expand_v0;
mod ifnum_v0;
mod ifx_v0;
mod macro_expand_v0;
mod stats_v0;
mod trace_v0;
#[cfg(test)]
mod count_v0_tests;
#[cfg(test)]
mod edef_v0_tests;
#[cfg(test)]
mod xdef_noexpand_v0_tests;
#[cfg(test)]
mod ifnum_v0_tests;
#[cfg(test)]
mod ifx_v0_tests;
#[cfg(test)]
mod meaning_v0_tests;
use crate::reasons_v0::{invalid_log_bytes_v0, InvalidInputReasonV0};
use crate::tex::tokenize_v0::{tokenize_v0, TokenV0, TokenizeErrorV0, MAX_TOKENS_V0};
use carreltex_core::{
    build_compile_result_v0, truncate_log_bytes_v0, CompileRequestV0, CompileResultV0,
    CompileStatus, Mount, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, MAX_LOG_BYTES_V0,
};
use input_expand_v0::expand_inputs_v0;
use macro_expand_v0::expand_macros_v0;
use stats_v0::build_tex_stats_from_tokens_v0;
use trace_v0::build_not_implemented_log_v0;
const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const EMPTY_TEX_STATS_JSON: &str = "";

fn invalid_result_v0(max_log_bytes: u32, reason: InvalidInputReasonV0) -> CompileResultV0 {
    build_compile_result_v0(
        CompileStatus::InvalidInput,
        &[],
        truncate_log_bytes_v0(invalid_log_bytes_v0(reason), max_log_bytes),
        vec![],
        EMPTY_TEX_STATS_JSON.to_owned(),
    )
}

pub fn compile_main_v0(mount: &mut Mount) -> CompileResultV0 {
    let request = CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0,
    };
    compile_request_v0(mount, &request)
}

pub fn compile_request_v0(mount: &mut Mount, req: &CompileRequestV0) -> CompileResultV0 {
    // INVALID_INPUT reason precedence SSOT (A-G):
    // A) request validation
    // B) mount finalize
    // C) entrypoint read
    // D) tokenize
    // E) input validation / expansion
    // F) macro validation / expansion
    // G) stats build/group-balance
    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }

    if mount.finalize().is_err() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::MountFinalizeFailed);
    }
    let entry_bytes = match mount.read_file_by_bytes_v0(req.entrypoint.as_bytes()) {
        Ok(Some(bytes)) => bytes.to_vec(),
        _ => return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::EntrypointMissing),
    };
    let tokens = match tokenize_v0(&entry_bytes) {
        Ok(tokens) => tokens,
        Err(TokenizeErrorV0::CaretNotSupported) => return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::TokenizerCaretNotSupported),
        Err(TokenizeErrorV0::ControlSeqNonAscii) => return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::TokenizerControlSeqNonAscii),
        Err(_) => return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::TokenizeFailed),
    };
    let (expanded_tokens, input_trace) = match expand_inputs_v0(&tokens, mount) {
        Ok(result) => result,
        Err(reason) => return invalid_result_v0(req.max_log_bytes, reason),
    };
    if expanded_tokens.len() > MAX_TOKENS_V0 {
        return invalid_result_v0(
            req.max_log_bytes,
            InvalidInputReasonV0::InputValidationFailed,
        );
    }
    if expanded_tokens
        .iter()
        .any(|token| matches!(token, TokenV0::ControlSeq(name) if name.as_slice() == b"input"))
    {
        return invalid_result_v0(
            req.max_log_bytes,
            InvalidInputReasonV0::InputValidationFailed,
        );
    }

    let macro_expanded_tokens = match expand_macros_v0(&expanded_tokens) {
        Ok(tokens) => tokens,
        Err(reason) => return invalid_result_v0(req.max_log_bytes, reason),
    };

    let tex_stats_json = match build_tex_stats_from_tokens_v0(&macro_expanded_tokens) {
        Ok(json) => json,
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
        }
    };
    if tex_stats_json.is_empty() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
    }
    let not_implemented_log =
        match build_not_implemented_log_v0(req.max_log_bytes as usize, &input_trace) {
            Some(log) => log,
            None => {
                return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed)
            }
        };
    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(&not_implemented_log, req.max_log_bytes),
        vec![],
        tex_stats_json,
    )
}

#[cfg(test)]
mod tests {
    use super::input_expand_v0::{MAX_INPUT_DEPTH_V0, MAX_INPUT_EXPANSIONS_V0};
    use super::macro_expand_v0::MAX_MACRO_EXPANSIONS_V0;
    use super::{compile_main_v0, compile_request_v0};
    use carreltex_core::{
        CompileRequestV0, CompileStatus, Mount, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0,
        MAX_LOG_BYTES_V0,
    };

    fn valid_main() -> &'static [u8] {
        b"\\documentclass{article}\n\\begin{document}\nHi\n\\end{document}\n"
    }

    fn valid_request() -> CompileRequestV0 {
        CompileRequestV0 {
            entrypoint: "main.tex".to_owned(),
            source_date_epoch: 1_700_000_000,
            max_log_bytes: 1024,
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
        rest[..digits_len].parse::<u64>().ok()
    }

    fn trace_u64_field(log_text: &str, field: &str) -> Option<u64> {
        let trace_start = log_text.find("INPUT_TRACE_V0:")?;
        let trace_json = &log_text[(trace_start + "INPUT_TRACE_V0:".len())..];
        let marker = format!("\"{field}\":");
        let start = trace_json.find(&marker)? + marker.len();
        let rest = &trace_json[start..];
        let digits_len = rest
            .bytes()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digits_len == 0 {
            return None;
        }
        rest[..digits_len].parse::<u64>().ok()
    }

    #[test]
    fn compile_requires_valid_mount() {
        let mut mount = Mount::default();
        let result = compile_main_v0(&mut mount);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"mount_finalize_failed"));
    }

    #[test]
    fn compile_main_uses_default_log_cap_and_not_implemented() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_main_v0(&mut mount);
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert!(result.log_bytes.len() <= DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0 as usize);
    }

    #[test]
    fn compile_request_returns_not_implemented_when_valid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
        assert!(!result.log_bytes.is_empty());
        assert!(result.log_bytes.starts_with(b"NOT_IMPLEMENTED:"));
        assert!(result.log_bytes.len() <= valid_request().max_log_bytes as usize);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(!result.tex_stats_json.is_empty());
    }

    #[test]
    fn compile_request_rejects_invalid_entrypoint() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.entrypoint = "other.tex".to_owned();
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"request_invalid"));
    }

    #[test]
    fn compile_request_rejects_zero_epoch_or_log_cap() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.source_date_epoch = 0;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.ends_with(b"request_invalid"));

        request = valid_request();
        request.max_log_bytes = 0;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.is_empty());
    }

    #[test]
    fn compile_request_rejects_log_cap_above_limit() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.max_log_bytes = MAX_LOG_BYTES_V0 + 1;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.ends_with(b"request_invalid"));
    }

    #[test]
    fn compile_request_log_is_truncated_by_max_log_bytes() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.max_log_bytes = 8;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert_eq!(result.log_bytes.len(), 8);
        assert_eq!(result.log_bytes, b"NOT_IMPL".to_vec());
        assert!(!String::from_utf8_lossy(&result.log_bytes).contains("INPUT_TRACE_V0:"));
        assert!(result.main_xdv_bytes.is_empty());
        assert!(!result.tex_stats_json.is_empty());
    }

    #[test]
    fn compile_request_trace_is_emitted_when_log_budget_allows() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let log = String::from_utf8_lossy(&result.log_bytes);
        assert!(log.starts_with("NOT_IMPLEMENTED:"));
        assert!(log.contains("INPUT_TRACE_V0:"));
    }

    #[test]
    fn compile_request_rejects_trailing_backslash_in_main_tex() {
        let mut mount = Mount::default();
        let trailing_backslash_main = b"\\documentclass{article}\n\\begin{document}\nHello\\";
        assert!(mount.add_file(b"main.tex", trailing_backslash_main).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert_eq!(
            result.report_json,
            "{\"status\":\"INVALID_INPUT\",\"missing_components\":[]}"
        );
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"tokenize_failed"));
    }

    #[test]
    fn compile_request_still_not_implemented_when_tokenization_succeeds() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
        assert!(result.log_bytes.starts_with(b"NOT_IMPLEMENTED:"));
        assert!(!result.tex_stats_json.is_empty());
    }

    #[test]
    fn compile_request_rejects_unbalanced_groups() {
        let mut mount = Mount::default();
        let unbalanced = b"\\documentclass{article}\n\\begin{document}\n{Hello\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", unbalanced).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.tex_stats_json.is_empty());
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"stats_build_failed"));
    }

    #[test]
    fn compile_request_missing_main_tex_reports_entrypoint_missing_reason() {
        let mut mount = Mount::default();
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"mount_finalize_failed"));
    }

    #[test]
    fn compile_request_invalid_main_content_reports_mount_finalize_failed_reason() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b" \n\t").is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"mount_finalize_failed"));
    }

    #[test]
    fn compile_request_missing_entrypoint_reports_request_invalid_reason() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());
        let mut request = valid_request();
        request.entrypoint = "missing.tex".to_owned();
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"request_invalid"));
    }

    #[test]
    fn compile_request_precedence_request_invalid_over_mount_finalize_failed() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b" \n\t").is_ok());
        let mut request = valid_request();
        request.entrypoint = "missing.tex".to_owned();
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"request_invalid"));
    }

    #[test]
    fn compile_request_stats_json_contains_expected_fields() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert!(result.tex_stats_json.contains("\"token_count\":"));
        assert!(result.tex_stats_json.contains("\"control_seq_count\":"));
        assert!(result.tex_stats_json.contains("\"char_count\":"));
        assert!(result.tex_stats_json.contains("\"space_count\":"));
        assert!(result.tex_stats_json.contains("\"begin_group_count\":"));
        assert!(result.tex_stats_json.contains("\"end_group_count\":"));
        assert!(result.tex_stats_json.contains("\"max_group_depth\":"));
    }

    #[test]
    fn input_valid_when_file_exists() {
        let mut mount = Mount::default();
        let main_with_input =
            b"\\documentclass{article}\n\\begin{document}\n\\input{sub.tex}\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main_with_input).is_ok());
        assert!(mount.add_file(b"sub.tex", b"Sub content\n").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert!(result.log_bytes.starts_with(b"NOT_IMPLEMENTED:"));
    }

    #[test]
    fn input_expands_tokens_from_subfile() {
        let mut mount_without_input = Mount::default();
        let main_without_input =
            b"\\documentclass{article}\n\\begin{document}\nHi\n\\end{document}\n";
        assert!(mount_without_input
            .add_file(b"main.tex", main_without_input)
            .is_ok());
        let result_without_input = compile_request_v0(&mut mount_without_input, &valid_request());
        assert_eq!(result_without_input.status, CompileStatus::NotImplemented);
        let char_count_without_input =
            stats_u64_field(&result_without_input.tex_stats_json, "char_count")
                .expect("char_count should exist");

        let mut mount_with_input = Mount::default();
        let main_with_input =
            b"\\documentclass{article}\n\\begin{document}\nHi\\input{sub.tex}\n\\end{document}\n";
        assert!(mount_with_input
            .add_file(b"main.tex", main_with_input)
            .is_ok());
        assert!(mount_with_input.add_file(b"sub.tex", b"ABC").is_ok());
        let result_with_input = compile_request_v0(&mut mount_with_input, &valid_request());
        assert_eq!(result_with_input.status, CompileStatus::NotImplemented);
        let log_with_input = String::from_utf8_lossy(&result_with_input.log_bytes);
        assert!(
            log_with_input.contains("INPUT_TRACE_V0:"),
            "missing trace in log: {log_with_input}"
        );
        assert_eq!(
            trace_u64_field(&log_with_input, "expansions"),
            Some(1),
            "trace log: {log_with_input}"
        );
        let char_count_with_input =
            stats_u64_field(&result_with_input.tex_stats_json, "char_count")
                .expect("char_count should exist");
        assert_eq!(char_count_with_input, char_count_without_input + 3);
    }

    #[test]
    fn input_missing_file_is_invalid() {
        let mut mount = Mount::default();
        let main_with_missing_input =
            b"\\documentclass{article}\n\\begin{document}\n\\input{missing.tex}\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main_with_missing_input).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"input_validation_failed"));
    }

    #[test]
    fn input_invalid_syntax_is_invalid() {
        let cases: [&[u8]; 3] = [
            b"\\documentclass{article}\n\\begin{document}\n\\input sub.tex\n\\end{document}\n",
            b"\\documentclass{article}\n\\begin{document}\n\\input{}\n\\end{document}\n",
            b"\\documentclass{article}\n\\begin{document}\n\\input{a b.tex}\n\\end{document}\n",
        ];
        for main in cases {
            let mut mount = Mount::default();
            assert!(mount.add_file(b"main.tex", main).is_ok());
            let result = compile_request_v0(&mut mount, &valid_request());
            assert_eq!(result.status, CompileStatus::InvalidInput);
            assert!(result.log_bytes.ends_with(b"input_validation_failed"));
        }
    }

    #[test]
    fn input_cycle_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount
            .add_file(
                b"main.tex",
                b"\\documentclass{article}\n\\begin{document}\n\\input{a.tex}\n\\end{document}\n",
            )
            .is_ok());
        assert!(mount
            .add_file(
                b"a.tex",
                b"\\documentclass{article}\n\\begin{document}\n\\input{main.tex}\n\\end{document}\n",
            )
            .is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"input_cycle_failed"));
        assert!(!String::from_utf8_lossy(&result.log_bytes).contains("INPUT_TRACE_V0:"));
    }

    #[test]
    fn input_depth_cap_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount
            .add_file(
                b"main.tex",
                b"\\documentclass{article}\n\\begin{document}\n\\input{d1.tex}\n\\end{document}\n",
            )
            .is_ok());

        for depth in 1..=(MAX_INPUT_DEPTH_V0 + 1) {
            let file_name = format!("d{depth}.tex");
            let file_contents = if depth == MAX_INPUT_DEPTH_V0 + 1 {
                b"X".to_vec()
            } else {
                format!("\\input{{d{}.tex}}", depth + 1).into_bytes()
            };
            assert!(mount
                .add_file(file_name.as_bytes(), file_contents.as_slice())
                .is_ok());
        }

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"input_depth_exceeded"));
    }

    #[test]
    fn input_expansions_cap_is_invalid() {
        let mut mount = Mount::default();
        let mut main = String::from("\\documentclass{article}\n\\begin{document}\n");
        for _ in 0..=MAX_INPUT_EXPANSIONS_V0 {
            main.push_str("\\input{a.tex}\n");
        }
        main.push_str("\\end{document}\n");
        assert!(mount.add_file(b"a.tex", b"X").is_ok());
        assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"input_expansions_exceeded"));
    }

    #[test]
    fn macro_expansion_positive_increases_char_count() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut macro_mount = Mount::default();
        let macro_main = b"\\documentclass{article}\n\\begin{document}\nHello.\\def\\foo{XYZ}\\foo\n\\end{document}\n";
        assert!(macro_mount.add_file(b"main.tex", macro_main).is_ok());
        let macro_result = compile_request_v0(&mut macro_mount, &valid_request());
        assert_eq!(macro_result.status, CompileStatus::NotImplemented);
        let macro_char_count =
            stats_u64_field(&macro_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(macro_char_count, baseline_char_count + 3);
    }

    #[test]
    fn macro_single_param_positive_increases_char_count() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut macro_mount = Mount::default();
        let macro_main = b"\\documentclass{article}\n\\begin{document}\nHello.\\def\\foo#1{#1}\\foo{XYZ}\n\\end{document}\n";
        assert!(macro_mount.add_file(b"main.tex", macro_main).is_ok());
        let macro_result = compile_request_v0(&mut macro_mount, &valid_request());
        assert_eq!(macro_result.status, CompileStatus::NotImplemented);
        let macro_char_count =
            stats_u64_field(&macro_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(macro_char_count, baseline_char_count + 3);
    }

    #[test]
    fn macro_cycle_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount
            .add_file(b"main.tex", b"\\def\\foo{\\foo}\\foo")
            .is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_cycle_failed"));
    }

    #[test]
    fn macro_params_unsupported_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount
            .add_file(b"main.tex", b"\\def\\foo#2{A}\\foo{X}")
            .is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_params_unsupported"));
    }

    #[test]
    fn macro_single_param_missing_arg_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount
            .add_file(b"main.tex", b"\\def\\foo#1{#1}\\foo")
            .is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_validation_failed"));
    }

    #[test]
    fn macro_defs_inside_group_do_not_leak_outside() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut scoped_mount = Mount::default();
        let scoped_main =
            b"\\documentclass{article}\n\\begin{document}\n{\\def\\foo{XYZ}}\\foo\n\\end{document}\n";
        assert!(scoped_mount.add_file(b"main.tex", scoped_main).is_ok());
        let scoped_result = compile_request_v0(&mut scoped_mount, &valid_request());
        assert_eq!(scoped_result.status, CompileStatus::NotImplemented);
        let scoped_char_count =
            stats_u64_field(&scoped_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(scoped_char_count, baseline_char_count);
    }

    #[test]
    fn gdef_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut gdef_mount = Mount::default();
        let gdef_main =
            b"\\documentclass{article}\n\\begin{document}\n{\\gdef\\foo{XYZ}}\\foo\n\\end{document}\n";
        assert!(gdef_mount.add_file(b"main.tex", gdef_main).is_ok());
        let gdef_result = compile_request_v0(&mut gdef_mount, &valid_request());
        assert_eq!(gdef_result.status, CompileStatus::NotImplemented);
        let gdef_char_count =
            stats_u64_field(&gdef_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(gdef_char_count, baseline_char_count + 3);
    }

    #[test]
    fn global_def_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut global_def_mount = Mount::default();
        let global_def_main =
            b"\\documentclass{article}\n\\begin{document}\n{\\global\\def\\foo{XYZ}}\\foo\n\\end{document}\n";
        assert!(global_def_mount
            .add_file(b"main.tex", global_def_main)
            .is_ok());
        let global_def_result = compile_request_v0(&mut global_def_mount, &valid_request());
        assert_eq!(global_def_result.status, CompileStatus::NotImplemented);
        let global_def_char_count =
            stats_u64_field(&global_def_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(global_def_char_count, baseline_char_count + 3);
    }

    #[test]
    fn global_gdef_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n{\\global\\gdef\\foo{XYZ}}\\foo\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 3);
    }

    #[test]
    fn stacked_global_def_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n{}\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n{\\global\\global\\def\\foo{XYZ}}\\foo\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 3);
    }

    #[test]
    fn let_alias_expands_control_sequence() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\\bar\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\let\\bar=\\foo\\bar\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 3);
    }

    #[test]
    fn global_let_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n{}\\bar\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main = b"\\documentclass{article}\n\\begin{document}\n{\\def\\foo{A}\\global\\let\\bar=\\foo}\\bar\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 1);
    }

    #[test]
    fn let_to_non_control_sequence_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\let\\a=Z").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_let_unsupported"));
    }

    #[test]
    fn futurelet_alias_expands_control_sequence() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\\bar\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\futurelet\\bar\\noop\\foo\\bar\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 3);
    }

    #[test]
    fn global_futurelet_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n{}\\bar\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main = b"\\documentclass{article}\n\\begin{document}\n{\\def\\foo{A}\\global\\futurelet\\bar\\noop\\foo}\\bar\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 1);
    }

    #[test]
    fn futurelet_with_non_control_sequence_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\futurelet\\a Z \\b").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_futurelet_unsupported"));
    }

    #[test]
    fn expandafter_reorders_two_control_sequences() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main = b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\def\\bar{A}\\expandafter\\bar\\foo\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 4);
    }

    #[test]
    fn expandafter_with_unsupported_tokens_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\expandafter{}").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_expandafter_unsupported"));
    }

    #[test]
    fn csname_generates_control_sequence_for_macro_lookup() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\csname foo\\endcsname\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 3);
    }

    #[test]
    fn csname_with_invalid_inner_tokens_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\csname\\foo\\endcsname").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_csname_unsupported"));
    }

    #[test]
    fn string_control_sequence_produces_literal_chars() {
        let mut baseline_mount = Mount::default();
        let baseline_main = b"\\documentclass{article}\n\\begin{document}\n\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut mount = Mount::default();
        let main =
            b"\\documentclass{article}\n\\begin{document}\n\\def\\foo{XYZ}\\string\\foo\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());
        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        let char_count = stats_u64_field(&result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(char_count, baseline_char_count + 4);
    }

    #[test]
    fn string_with_unsupported_tokens_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\string{}").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_string_unsupported"));
    }

    #[test]
    fn global_def_single_param_inside_group_leaks_globally() {
        let mut baseline_mount = Mount::default();
        let baseline_main =
            b"\\documentclass{article}\n\\begin{document}\n\\foo\n\\end{document}\n";
        assert!(baseline_mount.add_file(b"main.tex", baseline_main).is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut global_def_mount = Mount::default();
        let global_def_main =
            b"\\documentclass{article}\n\\begin{document}\n{\\global\\def\\foo#1{#1}}\\foo{X}\n\\end{document}\n";
        assert!(global_def_mount
            .add_file(b"main.tex", global_def_main)
            .is_ok());
        let global_def_result = compile_request_v0(&mut global_def_mount, &valid_request());
        assert_eq!(global_def_result.status, CompileStatus::NotImplemented);
        let global_def_char_count =
            stats_u64_field(&global_def_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(global_def_char_count, baseline_char_count + 1);
    }

    #[test]
    fn global_prefix_without_def_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\global\\foo").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_global_prefix_unsupported"));
    }

    #[test]
    fn stacked_global_prefix_without_def_is_invalid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b"\\global\\global\\foo").is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_global_prefix_unsupported"));
    }

    #[test]
    fn macro_defs_can_override_inside_group_without_leaking() {
        let mut baseline_mount = Mount::default();
        assert!(baseline_mount.add_file(b"main.tex", b"AB").is_ok());
        let baseline_result = compile_request_v0(&mut baseline_mount, &valid_request());
        assert_eq!(baseline_result.status, CompileStatus::NotImplemented);
        let baseline_char_count =
            stats_u64_field(&baseline_result.tex_stats_json, "char_count").expect("char_count");

        let mut scoped_mount = Mount::default();
        assert!(scoped_mount
            .add_file(b"main.tex", b"\\def\\foo{A}{\\def\\foo{B}\\foo}\\foo")
            .is_ok());
        let scoped_result = compile_request_v0(&mut scoped_mount, &valid_request());
        assert_eq!(scoped_result.status, CompileStatus::NotImplemented);
        let scoped_char_count =
            stats_u64_field(&scoped_result.tex_stats_json, "char_count").expect("char_count");
        assert_eq!(scoped_char_count, baseline_char_count);
    }

    #[test]
    fn macro_expansions_cap_is_invalid() {
        let mut mount = Mount::default();
        let mut main = String::from("\\def\\foo{A}");
        for _ in 0..=MAX_MACRO_EXPANSIONS_V0 {
            main.push_str("\\foo");
        }
        assert!(mount.add_file(b"main.tex", main.as_bytes()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.ends_with(b"macro_expansions_exceeded"));
    }
}
