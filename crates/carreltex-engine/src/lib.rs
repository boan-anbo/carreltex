pub mod tex;

use crate::tex::tokenize_v0::{tokenize_v0, TokenV0};
use carreltex_core::{
    build_compile_result_v0, build_tex_stats_json_v0, truncate_log_bytes_v0, CompileRequestV0,
    CompileResultV0, CompileStatus, Mount, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, MAX_LOG_BYTES_V0,
};

const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const NOT_IMPLEMENTED_LOG_BYTES: &[u8] =
    b"NOT_IMPLEMENTED: tex-engine compile pipeline is not wired yet";
const EMPTY_TEX_STATS_JSON: &str = "";

enum InvalidInputReasonV0 {
    MountFinalizeFailed,
    RequestInvalid,
    EntrypointMissing,
    TokenizeFailed,
    StatsBuildFailed,
}

fn invalid_log_bytes_v0(reason: InvalidInputReasonV0) -> &'static [u8] {
    match reason {
        InvalidInputReasonV0::MountFinalizeFailed => b"INVALID_INPUT: mount_finalize_failed",
        InvalidInputReasonV0::RequestInvalid => b"INVALID_INPUT: request_invalid",
        InvalidInputReasonV0::EntrypointMissing => b"INVALID_INPUT: entrypoint_missing",
        InvalidInputReasonV0::TokenizeFailed => b"INVALID_INPUT: tokenize_failed",
        InvalidInputReasonV0::StatsBuildFailed => b"INVALID_INPUT: stats_build_failed",
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
    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }

    let entry_bytes = match mount.read_file_by_bytes_v0(req.entrypoint.as_bytes()) {
        Ok(Some(bytes)) => bytes.to_vec(),
        _ => return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::EntrypointMissing),
    };
    if mount.finalize().is_err() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::MountFinalizeFailed);
    }
    let tokens = match tokenize_v0(&entry_bytes) {
        Ok(tokens) => tokens,
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::TokenizeFailed)
        }
    };
    let tex_stats_json = match build_tex_stats_from_tokens_v0(&tokens) {
        Ok(json) => json,
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
        }
    };
    if tex_stats_json.is_empty() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
    }

    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(NOT_IMPLEMENTED_LOG_BYTES, req.max_log_bytes),
        vec![],
        tex_stats_json,
    )
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

    #[test]
    fn compile_requires_valid_mount() {
        let mut mount = Mount::default();
        let result = compile_main_v0(&mut mount);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.log_bytes.starts_with(b"INVALID_INPUT:"));
        assert!(result.log_bytes.ends_with(b"entrypoint_missing"));
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
        assert!(result.main_xdv_bytes.is_empty());
        assert!(!result.tex_stats_json.is_empty());
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
        assert!(result.log_bytes.ends_with(b"entrypoint_missing"));
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
}
