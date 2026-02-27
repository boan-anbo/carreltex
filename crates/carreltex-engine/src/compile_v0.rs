use crate::reasons_v0::{invalid_log_bytes_v0, InvalidInputReasonV0};
use crate::tex::tokenize_v0::{tokenize_v0, TokenV0, MAX_TOKENS_V0};
use carreltex_core::{
    build_compile_result_v0, build_tex_stats_json_v0, normalize_path_v0, truncate_log_bytes_v0,
    validate_input_trace_json_v0, CompileRequestV0, CompileResultV0, CompileStatus, Mount,
    DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, MAX_LOG_BYTES_V0,
};

const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const NOT_IMPLEMENTED_LOG_BYTES: &[u8] =
    b"NOT_IMPLEMENTED: tex-engine compile pipeline is not wired yet";
const EMPTY_TEX_STATS_JSON: &str = "";
const MAX_INPUT_DEPTH_V0: usize = 32;
const MAX_INPUT_EXPANSIONS_V0: usize = 1024;
const INPUT_TRACE_MAX_FILES_V0: usize = 32;
const INPUT_TRACE_PREFIX_BYTES: &[u8] = b"\nINPUT_TRACE_V0:";

#[derive(Default)]
struct InputTraceV0 {
    expansions: u64,
    max_depth: u64,
    unique_files: u64,
    files: Vec<String>,
}

impl InputTraceV0 {
    fn new() -> Self {
        let mut trace = Self::default();
        trace.record_file("main.tex");
        trace
    }

    fn record_file(&mut self, path: &str) {
        if self.files.iter().any(|existing| existing == path) {
            return;
        }
        self.unique_files = self.unique_files.saturating_add(1);
        if self.files.len() < INPUT_TRACE_MAX_FILES_V0 {
            self.files.push(path.to_owned());
        }
    }

    fn record_depth(&mut self, depth: usize) {
        let depth_u64 = depth as u64;
        if depth_u64 > self.max_depth {
            self.max_depth = depth_u64;
        }
    }
}

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
    // INVALID_INPUT reason precedence SSOT (A-F):
    // A) request validation
    // B) mount finalize
    // C) entrypoint read
    // D) tokenize
    // E) stats build/group-balance
    // F) input validation / expansion
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
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::TokenizeFailed)
        }
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

    let tex_stats_json = match build_tex_stats_from_tokens_v0(&expanded_tokens) {
        Ok(json) => json,
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
        }
    };
    if tex_stats_json.is_empty() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
    }
    let trace_json = build_input_trace_json_v0(&input_trace);
    if validate_input_trace_json_v0(&trace_json).is_err() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
    }
    let mut not_implemented_log = NOT_IMPLEMENTED_LOG_BYTES.to_vec();
    let mut trace_line = Vec::new();
    trace_line.extend_from_slice(INPUT_TRACE_PREFIX_BYTES);
    trace_line.extend_from_slice(trace_json.as_bytes());
    let allowed = req.max_log_bytes as usize;
    if not_implemented_log
        .len()
        .checked_add(trace_line.len())
        .is_some_and(|total| total <= allowed)
    {
        not_implemented_log.extend_from_slice(&trace_line);
    }
    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(&not_implemented_log, req.max_log_bytes),
        vec![],
        tex_stats_json,
    )
}

fn build_input_trace_json_v0(trace: &InputTraceV0) -> String {
    let mut out = String::new();
    out.push_str("{\"expansions\":");
    out.push_str(&trace.expansions.to_string());
    out.push_str(",\"max_depth\":");
    out.push_str(&trace.max_depth.to_string());
    out.push_str(",\"unique_files\":");
    out.push_str(&trace.unique_files.to_string());
    out.push_str(",\"files\":[");
    for (index, file) in trace.files.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&escape_json_string_v0(file));
        out.push('"');
    }
    out.push_str("]}");
    out
}

fn escape_json_string_v0(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => {
                let code = ch as u32;
                escaped.push_str(&format!("\\u{code:04x}"));
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn parse_input_path_group_v0(
    tokens: &[TokenV0],
    input_index: usize,
) -> Result<(String, usize), InvalidInputReasonV0> {
    if !matches!(
        tokens.get(input_index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"input"
    ) {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }

    let mut index = input_index + 1;
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    index += 1;

    let mut path_bytes = Vec::new();
    loop {
        match tokens.get(index) {
            Some(TokenV0::Char(byte)) => {
                path_bytes.push(*byte);
                index += 1;
            }
            Some(TokenV0::EndGroup) => {
                index += 1;
                break;
            }
            Some(TokenV0::Space)
            | Some(TokenV0::ControlSeq(_))
            | Some(TokenV0::BeginGroup)
            | None => {
                return Err(InvalidInputReasonV0::InputValidationFailed);
            }
        }
    }

    if path_bytes.is_empty() {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }

    let normalized =
        normalize_path_v0(&path_bytes).map_err(|_| InvalidInputReasonV0::InputValidationFailed)?;
    if !normalized.ends_with(".tex") {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    Ok((normalized, index))
}

fn push_token_checked(out: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    out.push(token);
    Ok(())
}

fn extend_tokens_checked(
    out: &mut Vec<TokenV0>,
    tokens: &[TokenV0],
) -> Result<(), InvalidInputReasonV0> {
    let total = out
        .len()
        .checked_add(tokens.len())
        .ok_or(InvalidInputReasonV0::InputValidationFailed)?;
    if total > MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    out.extend_from_slice(tokens);
    Ok(())
}

fn expand_inputs_v0(
    tokens: &[TokenV0],
    mount: &Mount,
) -> Result<(Vec<TokenV0>, InputTraceV0), InvalidInputReasonV0> {
    let mut active_stack = vec!["main.tex".to_owned()];
    let mut expansion_count = 0usize;
    let mut trace = InputTraceV0::new();
    let expanded = expand_inputs_inner_v0(
        tokens,
        mount,
        0,
        &mut active_stack,
        &mut expansion_count,
        &mut trace,
    )?;
    Ok((expanded, trace))
}

fn expand_inputs_inner_v0(
    tokens: &[TokenV0],
    mount: &Mount,
    depth: usize,
    active_stack: &mut Vec<String>,
    expansion_count: &mut usize,
    trace: &mut InputTraceV0,
) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    if depth > MAX_INPUT_DEPTH_V0 {
        return Err(InvalidInputReasonV0::InputDepthExceeded);
    }

    let mut out = Vec::new();
    let mut index = 0usize;
    while index < tokens.len() {
        match &tokens[index] {
            TokenV0::ControlSeq(name) if name.as_slice() == b"input" => {
                *expansion_count = expansion_count
                    .checked_add(1)
                    .ok_or(InvalidInputReasonV0::InputExpansionsExceeded)?;
                if *expansion_count > MAX_INPUT_EXPANSIONS_V0 {
                    return Err(InvalidInputReasonV0::InputExpansionsExceeded);
                }
                trace.expansions = *expansion_count as u64;

                let (normalized_path, next_index) = parse_input_path_group_v0(tokens, index)?;
                if active_stack.iter().any(|path| path == &normalized_path) {
                    return Err(InvalidInputReasonV0::InputCycleFailed);
                }
                trace.record_file(&normalized_path);
                trace.record_depth(depth + 1);

                let included_bytes = match mount.read_file_by_bytes_v0(normalized_path.as_bytes()) {
                    Ok(Some(bytes)) => bytes,
                    _ => return Err(InvalidInputReasonV0::InputValidationFailed),
                };
                let included_tokens = tokenize_v0(included_bytes)
                    .map_err(|_| InvalidInputReasonV0::InputValidationFailed)?;

                active_stack.push(normalized_path);
                let expanded = expand_inputs_inner_v0(
                    &included_tokens,
                    mount,
                    depth + 1,
                    active_stack,
                    expansion_count,
                    trace,
                )?;
                active_stack.pop();

                extend_tokens_checked(&mut out, &expanded)?;
                index = next_index;
            }
            token => {
                push_token_checked(&mut out, token.clone())?;
                index += 1;
            }
        }
    }

    Ok(out)
}

fn build_tex_stats_from_tokens_v0(tokens: &[TokenV0]) -> Result<String, ()> {
    let mut depth: u64 = 0;
    let mut max_depth: u64 = 0;
    let mut control_seq_count: u64 = 0;
    let mut char_count: u64 = 0;
    let mut space_count: u64 = 0;
    let mut begin_group_count: u64 = 0;
    let mut end_group_count: u64 = 0;

    for token in tokens {
        match token {
            TokenV0::ControlSeq(_) => {
                control_seq_count = control_seq_count.checked_add(1).ok_or(())?;
            }
            TokenV0::Char(_) => {
                char_count = char_count.checked_add(1).ok_or(())?;
            }
            TokenV0::Space => {
                space_count = space_count.checked_add(1).ok_or(())?;
            }
            TokenV0::BeginGroup => {
                begin_group_count = begin_group_count.checked_add(1).ok_or(())?;
                depth = depth.checked_add(1).ok_or(())?;
                max_depth = max_depth.max(depth);
            }
            TokenV0::EndGroup => {
                if depth == 0 {
                    return Err(());
                }
                end_group_count = end_group_count.checked_add(1).ok_or(())?;
                depth -= 1;
            }
        }
    }

    if depth != 0 {
        return Err(());
    }

    let token_count: u64 = tokens.len().try_into().map_err(|_| ())?;
    build_tex_stats_json_v0(
        token_count,
        control_seq_count,
        char_count,
        space_count,
        begin_group_count,
        end_group_count,
        max_depth,
    )
    .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{compile_main_v0, compile_request_v0, MAX_INPUT_DEPTH_V0, MAX_INPUT_EXPANSIONS_V0};
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
}
