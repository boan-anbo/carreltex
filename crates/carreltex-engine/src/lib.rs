pub mod tex;

use crate::tex::tokenize_v0::{tokenize_v0, TokenV0};
use carreltex_core::{
    build_compile_result_v0, truncate_log_bytes_v0, CompileRequestV0, CompileResultV0,
    CompileStatus, Mount, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, MAX_LOG_BYTES_V0,
    MAX_TEX_STATS_JSON_BYTES_V0,
};

const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const NOT_IMPLEMENTED_LOG_BYTES: &[u8] =
    b"NOT_IMPLEMENTED: tex-engine compile pipeline is not wired yet";
const INVALID_INPUT_LOG_BYTES: &[u8] = b"";
const EMPTY_TEX_STATS_JSON: &str = "";

pub fn compile_main_v0(mount: &mut Mount) -> CompileResultV0 {
    let request = CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0,
    };
    compile_request_v0(mount, &request)
}

pub fn compile_request_v0(mount: &mut Mount, req: &CompileRequestV0) -> CompileResultV0 {
    if mount.finalize().is_err() {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
            vec![],
            EMPTY_TEX_STATS_JSON.to_owned(),
        );
    }

    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
            vec![],
            EMPTY_TEX_STATS_JSON.to_owned(),
        );
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
            vec![],
            EMPTY_TEX_STATS_JSON.to_owned(),
        );
    }

    let entry_bytes = match mount.read_file_by_bytes_v0(req.entrypoint.as_bytes()) {
        Ok(Some(bytes)) => bytes,
        _ => {
            return build_compile_result_v0(
                CompileStatus::InvalidInput,
                &[],
                truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
                vec![],
                EMPTY_TEX_STATS_JSON.to_owned(),
            );
        }
    };
    let tokens = match tokenize_v0(entry_bytes) {
        Ok(tokens) => tokens,
        Err(_) => {
            return build_compile_result_v0(
                CompileStatus::InvalidInput,
                &[],
                truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
                vec![],
                EMPTY_TEX_STATS_JSON.to_owned(),
            );
        }
    };
    let tex_stats_json = match build_tex_stats_json_v0(&tokens) {
        Ok(json) => json,
        Err(_) => {
            return build_compile_result_v0(
                CompileStatus::InvalidInput,
                &[],
                truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
                vec![],
                EMPTY_TEX_STATS_JSON.to_owned(),
            );
        }
    };
    if tex_stats_json.is_empty() || tex_stats_json.len() > MAX_TEX_STATS_JSON_BYTES_V0 {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
            vec![],
            EMPTY_TEX_STATS_JSON.to_owned(),
        );
    }

    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(NOT_IMPLEMENTED_LOG_BYTES, req.max_log_bytes),
        vec![],
        tex_stats_json,
    )
}

fn build_tex_stats_json_v0(tokens: &[TokenV0]) -> Result<String, ()> {
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
    let out = format!(
        "{{\"token_count\":{token_count},\"control_seq_count\":{control_seq_count},\"char_count\":{char_count},\"space_count\":{space_count},\"begin_group_count\":{begin_group_count},\"end_group_count\":{end_group_count},\"max_group_depth\":{max_depth}}}"
    );
    if out.len() > MAX_TEX_STATS_JSON_BYTES_V0 {
        return Err(());
    }
    Ok(out)
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

        request = valid_request();
        request.max_log_bytes = 0;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
        assert!(result.main_xdv_bytes.is_empty());
        assert!(result.tex_stats_json.is_empty());
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
