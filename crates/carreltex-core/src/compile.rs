use crate::mount::{Error, Mount};

pub const MAX_LOG_BYTES_V0: u32 = 1024 * 1024;
const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];

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

pub fn compile_main_v0(mount: &mut Mount) -> CompileResultV0 {
    let request = CompileRequestV0 {
        entrypoint: "main.tex".to_owned(),
        source_date_epoch: 1,
        max_log_bytes: 1024,
    };
    compile_request_v0(mount, &request)
}

pub fn compile_request_v0(mount: &mut Mount, req: &CompileRequestV0) -> CompileResultV0 {
    if mount.finalize().is_err() {
        return compile_result_v0(CompileStatus::InvalidInput, &[]);
    }

    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return compile_result_v0(CompileStatus::InvalidInput, &[]);
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return compile_result_v0(CompileStatus::InvalidInput, &[]);
    }

    compile_result_v0(CompileStatus::NotImplemented, MISSING_COMPONENTS_V0)
}

fn compile_result_v0(status: CompileStatus, missing_components: &[&str]) -> CompileResultV0 {
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
        compile_main_v0, compile_request_v0, CompileRequestV0, CompileStatus, MAX_LOG_BYTES_V0,
    };
    use crate::mount::Mount;

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
    fn compile_request_returns_not_implemented_when_valid() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let result = compile_request_v0(&mut mount, &valid_request());
        assert_eq!(result.status, CompileStatus::NotImplemented);
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        );
    }

    #[test]
    fn compile_request_rejects_invalid_entrypoint() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.entrypoint = "other.tex".to_owned();
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
    }

    #[test]
    fn compile_request_rejects_zero_epoch_or_log_cap() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.source_date_epoch = 0;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);

        request = valid_request();
        request.max_log_bytes = 0;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
    }

    #[test]
    fn compile_request_rejects_log_cap_above_limit() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", valid_main()).is_ok());

        let mut request = valid_request();
        request.max_log_bytes = MAX_LOG_BYTES_V0 + 1;
        let result = compile_request_v0(&mut mount, &request);
        assert_eq!(result.status, CompileStatus::InvalidInput);
    }
}
