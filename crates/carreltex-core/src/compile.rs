use crate::mount::Error;

pub const MAX_LOG_BYTES_V0: u32 = 1024 * 1024;
pub const MAX_ARTIFACT_BYTES_V0: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileStatus {
    Ok = 0,
    InvalidInput = 1,
    NotImplemented = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileRequestV0 {
    pub entrypoint: String,
    pub source_date_epoch: u64,
    pub max_log_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileResultV0 {
    pub status: CompileStatus,
    pub report_json: String,
    pub log_bytes: Vec<u8>,
    pub main_xdv_bytes: Vec<u8>,
}

pub fn build_compile_result_v0(
    status: CompileStatus,
    missing_components: &[&str],
    log_bytes: Vec<u8>,
    main_xdv_bytes: Vec<u8>,
) -> CompileResultV0 {
    let report_json = build_compile_report_json(status, missing_components);
    CompileResultV0 {
        status,
        report_json,
        log_bytes,
        main_xdv_bytes,
    }
}

pub fn truncate_log_bytes_v0(log_bytes: &[u8], max_log_bytes: u32) -> Vec<u8> {
    let max = max_log_bytes as usize;
    if log_bytes.len() <= max {
        return log_bytes.to_vec();
    }
    log_bytes[..max].to_vec()
}

pub fn artifact_bytes_within_cap_v0(bytes: &[u8]) -> bool {
    bytes.len() <= MAX_ARTIFACT_BYTES_V0
}

fn build_compile_report_json(status: CompileStatus, missing_components: &[&str]) -> String {
    let status_str = match status {
        CompileStatus::Ok => "OK",
        CompileStatus::InvalidInput => "INVALID_INPUT",
        CompileStatus::NotImplemented => "NOT_IMPLEMENTED",
    };

    let mut out = String::new();
    out.push_str("{\"status\":\"");
    out.push_str(status_str);
    out.push_str("\",\"missing_components\":[");
    for (index, component) in missing_components.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&escape_json_string(component));
        out.push('"');
    }
    out.push_str("]}");
    out
}

fn escape_json_string(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                use core::fmt::Write;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

pub fn validate_compile_report_json(report_json: &str) -> Result<(), Error> {
    if report_json.trim().is_empty() {
        return Err(Error::InvalidInput);
    }
    if !report_json.contains("\"missing_components\"") {
        return Err(Error::InvalidInput);
    }

    let status_tokens = [
        "\"status\":\"OK\"",
        "\"status\":\"INVALID_INPUT\"",
        "\"status\":\"NOT_IMPLEMENTED\"",
    ];
    let status_match_count = status_tokens
        .iter()
        .filter(|token| report_json.contains(**token))
        .count();
    if status_match_count != 1 {
        return Err(Error::InvalidInput);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        artifact_bytes_within_cap_v0, build_compile_result_v0, truncate_log_bytes_v0,
        validate_compile_report_json, CompileRequestV0, CompileStatus, MAX_ARTIFACT_BYTES_V0,
        MAX_LOG_BYTES_V0,
    };

    #[test]
    fn compile_request_struct_accepts_v0_fields() {
        let request = CompileRequestV0 {
            entrypoint: "main.tex".to_owned(),
            source_date_epoch: 1_700_000_000,
            max_log_bytes: 1024,
        };
        assert_eq!(request.entrypoint, "main.tex");
        assert_eq!(request.source_date_epoch, 1_700_000_000);
        assert_eq!(request.max_log_bytes, 1024);
    }

    #[test]
    fn compile_result_builder_uses_canonical_key_order() {
        let result = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["tex-engine"],
            b"NOT_IMPLEMENTED: missing tex-engine".to_vec(),
            vec![],
        );
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
    }

    #[test]
    fn compile_result_builder_escapes_json_string_content() {
        let result = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["a\"b\\c"],
            vec![0xff, b'\n', b'X'],
            vec![1, 2, 3],
        );
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"a\\\"b\\\\c\"]}"
        );
        assert_eq!(result.log_bytes, vec![0xff, b'\n', b'X']);
        assert_eq!(result.main_xdv_bytes, vec![1, 2, 3]);
    }

    #[test]
    fn validate_compile_report_json_rejects_missing_keys_or_unknown_status() {
        assert!(validate_compile_report_json("{\"status\":\"OK\"}").is_err());
        assert!(validate_compile_report_json("{\"missing_components\":[],\"status\":\"UNKNOWN\"}").is_err());
        assert!(validate_compile_report_json("{\"missing_components\":[]}").is_err());
        assert!(validate_compile_report_json("").is_err());
    }

    #[test]
    fn validate_compile_report_json_rejects_multiple_status_tokens() {
        let bad = "{\"status\":\"OK\",\"status\":\"INVALID_INPUT\",\"missing_components\":[]}";
        assert!(validate_compile_report_json(bad).is_err());
    }

    #[test]
    fn validate_compile_report_json_accepts_single_known_status() {
        assert!(validate_compile_report_json("{\"status\":\"OK\",\"missing_components\":[]}").is_ok());
        assert!(
            validate_compile_report_json(
                "{\"status\":\"INVALID_INPUT\",\"missing_components\":[]}"
            )
            .is_ok()
        );
        assert!(
            validate_compile_report_json(
                "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
            )
            .is_ok()
        );
    }

    #[test]
    fn max_log_bytes_constant_is_non_zero() {
        assert!(MAX_LOG_BYTES_V0 > 0);
    }

    #[test]
    fn truncate_log_bytes_enforces_max() {
        let bytes = b"NOT_IMPLEMENTED: tex-engine missing".to_vec();
        let truncated = truncate_log_bytes_v0(&bytes, 16);
        assert_eq!(truncated.len(), 16);
        assert_eq!(truncated, bytes[..16].to_vec());
    }

    #[test]
    fn report_json_stays_stable_with_different_log_bytes() {
        let a = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["tex-engine"],
            b"NOT_IMPLEMENTED: A".to_vec(),
            vec![0x01],
        );
        let b = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["tex-engine"],
            b"NOT_IMPLEMENTED: B".to_vec(),
            vec![0x02, 0x03],
        );
        assert_eq!(
            a.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
        assert_eq!(a.report_json, b.report_json);
    }

    #[test]
    fn compile_result_builder_keeps_artifact_bytes_exact() {
        let artifact = vec![0xde, 0xad, 0xbe, 0xef];
        let result = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["tex-engine"],
            b"NOT_IMPLEMENTED: log".to_vec(),
            artifact.clone(),
        );
        assert_eq!(result.main_xdv_bytes, artifact);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
    }

    #[test]
    fn artifact_bytes_within_cap_honors_limit() {
        let bytes = vec![0u8; MAX_ARTIFACT_BYTES_V0];
        assert!(artifact_bytes_within_cap_v0(&bytes));

        let bytes = vec![0u8; MAX_ARTIFACT_BYTES_V0 + 1];
        assert!(!artifact_bytes_within_cap_v0(&bytes));
    }
}
