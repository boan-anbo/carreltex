use carreltex_core::{
    build_compile_result_v0, truncate_log_bytes_v0, CompileRequestV0, CompileResultV0,
    CompileStatus, Mount, MAX_LOG_BYTES_V0,
};

const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const NOT_IMPLEMENTED_LOG_BYTES: &[u8] = b"NOT_IMPLEMENTED: tex-engine compile pipeline is not wired yet";
const INVALID_INPUT_LOG_BYTES: &[u8] = b"";

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
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
        );
    }

    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
        );
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return build_compile_result_v0(
            CompileStatus::InvalidInput,
            &[],
            truncate_log_bytes_v0(INVALID_INPUT_LOG_BYTES, req.max_log_bytes),
        );
    }

    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(NOT_IMPLEMENTED_LOG_BYTES, req.max_log_bytes),
    )
}

#[cfg(test)]
mod tests {
    use super::{compile_main_v0, compile_request_v0};
    use carreltex_core::{CompileRequestV0, CompileStatus, Mount, MAX_LOG_BYTES_V0};

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
        assert!(!result.log_bytes.is_empty());
        assert!(result.log_bytes.starts_with(b"NOT_IMPLEMENTED:"));
        assert!(result.log_bytes.len() <= valid_request().max_log_bytes as usize);
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
    }
}
