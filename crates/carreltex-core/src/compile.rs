use crate::mount::Error;

pub const MAX_LOG_BYTES_V0: u32 = 1024 * 1024;

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
}

pub fn build_compile_result_v0(status: CompileStatus, missing_components: &[&str]) -> CompileResultV0 {
    let report_json = build_compile_report_json(status, missing_components);
    CompileResultV0 { status, report_json }
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
    if !report_json.contains("\"status\"") {
        return Err(Error::InvalidInput);
    }
    if !report_json.contains("\"missing_components\"") {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        build_compile_result_v0, validate_compile_report_json, CompileRequestV0, CompileStatus,
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
        let result = build_compile_result_v0(CompileStatus::NotImplemented, &["tex-engine"]);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
    }

    #[test]
    fn compile_result_builder_escapes_json_string_content() {
        let result = build_compile_result_v0(CompileStatus::NotImplemented, &["a\"b\\c"]);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"a\\\"b\\\\c\"]}"
        );
    }

    #[test]
    fn validate_compile_report_json_rejects_missing_keys() {
        assert!(validate_compile_report_json("{\"status\":\"OK\"}").is_err());
        assert!(validate_compile_report_json("{\"missing_components\":[]}").is_err());
        assert!(validate_compile_report_json("").is_err());
    }

    #[test]
    fn max_log_bytes_constant_is_non_zero() {
        assert!(MAX_LOG_BYTES_V0 > 0);
    }
}
