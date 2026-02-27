use crate::mount::Error;

pub const MAX_LOG_BYTES_V0: u32 = 1024 * 1024;
pub const DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0: u32 = 1024;
pub const MAX_ARTIFACT_BYTES_V0: usize = 32 * 1024 * 1024;
pub const MAX_WASM_ALLOC_BYTES_V0: usize = MAX_ARTIFACT_BYTES_V0;
pub const EVENT_KIND_LOG_BYTES_V0: u32 = 1;
pub const EVENT_KIND_TEX_STATS_JSON_V0: u32 = 2;
pub const MAX_TEX_STATS_JSON_BYTES_V0: usize = 4096;
pub const MAX_EVENTS_BYTES_V0: usize =
    (MAX_LOG_BYTES_V0 as usize) + 8 + MAX_TEX_STATS_JSON_BYTES_V0 + 8;

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
    pub tex_stats_json: String,
}

pub fn build_compile_result_v0(
    status: CompileStatus,
    missing_components: &[&str],
    log_bytes: Vec<u8>,
    main_xdv_bytes: Vec<u8>,
    tex_stats_json: String,
) -> CompileResultV0 {
    let report_json = build_compile_report_json(status, missing_components);
    CompileResultV0 {
        status,
        report_json,
        log_bytes,
        main_xdv_bytes,
        tex_stats_json,
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

pub fn report_json_has_status_token_v0(status: CompileStatus, report_json: &str) -> bool {
    let token = match status {
        CompileStatus::Ok => "\"status\":\"OK\"",
        CompileStatus::InvalidInput => "\"status\":\"INVALID_INPUT\"",
        CompileStatus::NotImplemented => "\"status\":\"NOT_IMPLEMENTED\"",
    };
    report_json.contains(token)
}

pub fn report_json_missing_components_is_empty_v0(report_json: &str) -> Option<bool> {
    if report_json.contains("\"missing_components\":[]") {
        return Some(true);
    }
    if report_json.contains("\"missing_components\":[") {
        return Some(false);
    }
    None
}

pub fn append_event_v0(out: &mut Vec<u8>, kind: u32, payload: &[u8]) -> Result<(), Error> {
    let payload_len_u32: u32 = match payload.len().try_into() {
        Ok(value) => value,
        Err(_) => return Err(Error::InvalidInput),
    };

    let required = 8usize
        .checked_add(payload.len())
        .ok_or(Error::InvalidInput)?;
    let total = out.len().checked_add(required).ok_or(Error::InvalidInput)?;
    if total > MAX_EVENTS_BYTES_V0 {
        return Err(Error::InvalidInput);
    }

    out.extend_from_slice(&kind.to_le_bytes());
    out.extend_from_slice(&payload_len_u32.to_le_bytes());
    out.extend_from_slice(payload);
    Ok(())
}

pub fn build_tex_stats_json_v0(
    token_count: u64,
    control_seq_count: u64,
    char_count: u64,
    space_count: u64,
    begin_group_count: u64,
    end_group_count: u64,
    max_group_depth: u64,
) -> Result<String, Error> {
    let out = format!(
        "{{\"token_count\":{token_count},\"control_seq_count\":{control_seq_count},\"char_count\":{char_count},\"space_count\":{space_count},\"begin_group_count\":{begin_group_count},\"end_group_count\":{end_group_count},\"max_group_depth\":{max_group_depth}}}"
    );
    if out.len() > MAX_TEX_STATS_JSON_BYTES_V0 {
        return Err(Error::InvalidInput);
    }
    Ok(out)
}

pub fn validate_tex_stats_json_v0(text: &str) -> Result<(), Error> {
    if text.is_empty() || text.len() > MAX_TEX_STATS_JSON_BYTES_V0 {
        return Err(Error::InvalidInput);
    }
    if text
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
    {
        return Err(Error::InvalidInput);
    }

    let keys = [
        "token_count",
        "control_seq_count",
        "char_count",
        "space_count",
        "begin_group_count",
        "end_group_count",
        "max_group_depth",
    ];
    let bytes = text.as_bytes();
    let mut index = 0usize;
    if bytes.get(index) != Some(&b'{') {
        return Err(Error::InvalidInput);
    }
    index += 1;

    for (position, key) in keys.iter().enumerate() {
        if bytes.get(index) != Some(&b'"') {
            return Err(Error::InvalidInput);
        }
        index += 1;
        let key_bytes = key.as_bytes();
        let key_end = index
            .checked_add(key_bytes.len())
            .ok_or(Error::InvalidInput)?;
        if bytes.get(index..key_end) != Some(key_bytes) {
            return Err(Error::InvalidInput);
        }
        index = key_end;
        if bytes.get(index) != Some(&b'"') {
            return Err(Error::InvalidInput);
        }
        index += 1;
        if bytes.get(index) != Some(&b':') {
            return Err(Error::InvalidInput);
        }
        index += 1;

        let value_start = index;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
        }
        if value_start == index {
            return Err(Error::InvalidInput);
        }

        if position + 1 < keys.len() {
            if bytes.get(index) != Some(&b',') {
                return Err(Error::InvalidInput);
            }
            index += 1;
        }
    }

    if bytes.get(index) != Some(&b'}') {
        return Err(Error::InvalidInput);
    }
    index += 1;
    if index != bytes.len() {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

pub fn validate_input_trace_json_v0(text: &str) -> Result<(), Error> {
    if text.is_empty() {
        return Err(Error::InvalidInput);
    }
    if text
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
    {
        return Err(Error::InvalidInput);
    }

    let bytes = text.as_bytes();
    let mut index = 0usize;
    consume_byte(bytes, &mut index, b'{')?;
    consume_key(bytes, &mut index, "expansions")?;
    consume_u64_digits(bytes, &mut index)?;
    consume_byte(bytes, &mut index, b',')?;
    consume_key(bytes, &mut index, "max_depth")?;
    consume_u64_digits(bytes, &mut index)?;
    consume_byte(bytes, &mut index, b',')?;
    consume_key(bytes, &mut index, "unique_files")?;
    consume_u64_digits(bytes, &mut index)?;
    consume_byte(bytes, &mut index, b',')?;
    consume_key(bytes, &mut index, "files")?;
    consume_byte(bytes, &mut index, b'[')?;
    if bytes.get(index) != Some(&b']') {
        loop {
            consume_json_string(bytes, &mut index)?;
            if bytes.get(index) == Some(&b',') {
                index += 1;
                continue;
            }
            break;
        }
    }
    consume_byte(bytes, &mut index, b']')?;
    consume_byte(bytes, &mut index, b'}')?;
    if index != bytes.len() {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn consume_byte(bytes: &[u8], index: &mut usize, expected: u8) -> Result<(), Error> {
    if bytes.get(*index) != Some(&expected) {
        return Err(Error::InvalidInput);
    }
    *index += 1;
    Ok(())
}

fn consume_key(bytes: &[u8], index: &mut usize, key: &str) -> Result<(), Error> {
    consume_byte(bytes, index, b'"')?;
    let key_bytes = key.as_bytes();
    let key_end = index
        .checked_add(key_bytes.len())
        .ok_or(Error::InvalidInput)?;
    if bytes.get(*index..key_end) != Some(key_bytes) {
        return Err(Error::InvalidInput);
    }
    *index = key_end;
    consume_byte(bytes, index, b'"')?;
    consume_byte(bytes, index, b':')?;
    Ok(())
}

fn consume_u64_digits(bytes: &[u8], index: &mut usize) -> Result<(), Error> {
    let start = *index;
    while matches!(bytes.get(*index), Some(b'0'..=b'9')) {
        *index += 1;
    }
    if start == *index {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn consume_json_string(bytes: &[u8], index: &mut usize) -> Result<(), Error> {
    consume_byte(bytes, index, b'"')?;
    loop {
        let byte = match bytes.get(*index) {
            Some(value) => *value,
            None => return Err(Error::InvalidInput),
        };
        *index += 1;
        match byte {
            b'"' => return Ok(()),
            b'\\' => {
                let escaped = match bytes.get(*index) {
                    Some(value) => *value,
                    None => return Err(Error::InvalidInput),
                };
                *index += 1;
                match escaped {
                    b'\\' | b'"' | b'n' | b'r' | b't' | b'b' | b'f' => {}
                    b'u' => {
                        for _ in 0..4 {
                            match bytes.get(*index) {
                                Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => *index += 1,
                                _ => return Err(Error::InvalidInput),
                            }
                        }
                    }
                    _ => return Err(Error::InvalidInput),
                }
            }
            0x00..=0x1f => return Err(Error::InvalidInput),
            _ => {}
        }
    }
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
        append_event_v0, artifact_bytes_within_cap_v0, build_compile_result_v0,
        build_tex_stats_json_v0, report_json_has_status_token_v0,
        report_json_missing_components_is_empty_v0, truncate_log_bytes_v0,
        validate_compile_report_json, validate_input_trace_json_v0, validate_tex_stats_json_v0,
        CompileRequestV0, CompileStatus, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0,
        EVENT_KIND_LOG_BYTES_V0, EVENT_KIND_TEX_STATS_JSON_V0, MAX_ARTIFACT_BYTES_V0,
        MAX_EVENTS_BYTES_V0, MAX_LOG_BYTES_V0, MAX_TEX_STATS_JSON_BYTES_V0,
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
            "{\"token_count\":1}".to_owned(),
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
            "{\"token_count\":2}".to_owned(),
        );
        assert_eq!(
            result.report_json,
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"a\\\"b\\\\c\"]}"
        );
        assert_eq!(result.log_bytes, vec![0xff, b'\n', b'X']);
        assert_eq!(result.main_xdv_bytes, vec![1, 2, 3]);
        assert_eq!(result.tex_stats_json, "{\"token_count\":2}");
    }

    #[test]
    fn validate_compile_report_json_rejects_missing_keys_or_unknown_status() {
        assert!(validate_compile_report_json("{\"status\":\"OK\"}").is_err());
        assert!(
            validate_compile_report_json("{\"missing_components\":[],\"status\":\"UNKNOWN\"}")
                .is_err()
        );
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
        assert!(
            validate_compile_report_json("{\"status\":\"OK\",\"missing_components\":[]}").is_ok()
        );
        assert!(validate_compile_report_json(
            "{\"status\":\"INVALID_INPUT\",\"missing_components\":[]}"
        )
        .is_ok());
        assert!(validate_compile_report_json(
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        )
        .is_ok());
    }

    #[test]
    fn max_log_bytes_constant_is_non_zero() {
        assert!(MAX_LOG_BYTES_V0 > 0);
    }

    #[test]
    fn default_compile_main_log_bytes_constant_is_1024() {
        assert_eq!(DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, 1024);
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
            "{\"token_count\":11}".to_owned(),
        );
        let b = build_compile_result_v0(
            CompileStatus::NotImplemented,
            &["tex-engine"],
            b"NOT_IMPLEMENTED: B".to_vec(),
            vec![0x02, 0x03],
            "{\"token_count\":22}".to_owned(),
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
            "{\"token_count\":9}".to_owned(),
        );
        assert_eq!(result.main_xdv_bytes, artifact);
        assert_eq!(result.tex_stats_json, "{\"token_count\":9}");
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

    #[test]
    fn report_json_has_status_token_checks_exact_status() {
        let ok = "{\"status\":\"OK\",\"missing_components\":[]}";
        assert!(report_json_has_status_token_v0(CompileStatus::Ok, ok));
        assert!(!report_json_has_status_token_v0(
            CompileStatus::InvalidInput,
            ok
        ));
        assert!(!report_json_has_status_token_v0(
            CompileStatus::NotImplemented,
            ok
        ));

        let invalid = "{\"status\":\"INVALID_INPUT\",\"missing_components\":[]}";
        assert!(report_json_has_status_token_v0(
            CompileStatus::InvalidInput,
            invalid
        ));
        assert!(!report_json_has_status_token_v0(CompileStatus::Ok, invalid));
        assert!(!report_json_has_status_token_v0(
            CompileStatus::NotImplemented,
            invalid
        ));

        let not_impl = "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}";
        assert!(report_json_has_status_token_v0(
            CompileStatus::NotImplemented,
            not_impl
        ));
        assert!(!report_json_has_status_token_v0(
            CompileStatus::Ok,
            not_impl
        ));
        assert!(!report_json_has_status_token_v0(
            CompileStatus::InvalidInput,
            not_impl
        ));
    }

    #[test]
    fn report_json_missing_components_empty_detection() {
        assert_eq!(
            report_json_missing_components_is_empty_v0(
                "{\"status\":\"OK\",\"missing_components\":[]}"
            ),
            Some(true)
        );
        assert_eq!(
            report_json_missing_components_is_empty_v0(
                "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
            ),
            Some(false)
        );
        assert_eq!(
            report_json_missing_components_is_empty_v0("{\"status\":\"OK\"}"),
            None
        );
    }

    #[test]
    fn append_event_encodes_header_little_endian() {
        let mut out = Vec::new();
        assert!(append_event_v0(&mut out, EVENT_KIND_LOG_BYTES_V0, b"ab").is_ok());
        let expected = vec![
            1, 0, 0, 0, // kind
            2, 0, 0, 0, // len
            b'a', b'b',
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn append_event_rejects_when_exceeds_max_events_bytes() {
        let mut out = vec![0u8; MAX_EVENTS_BYTES_V0 - 1];
        assert!(append_event_v0(&mut out, EVENT_KIND_LOG_BYTES_V0, b"x").is_err());
    }

    #[test]
    fn event_kind_tex_stats_json_constant_is_two() {
        assert_eq!(EVENT_KIND_TEX_STATS_JSON_V0, 2);
    }

    #[test]
    fn max_tex_stats_json_bytes_constant_is_4096() {
        assert_eq!(MAX_TEX_STATS_JSON_BYTES_V0, 4096);
    }

    #[test]
    fn max_events_bytes_allows_log_and_stats_events() {
        assert_eq!(
            MAX_EVENTS_BYTES_V0,
            (MAX_LOG_BYTES_V0 as usize) + 8 + MAX_TEX_STATS_JSON_BYTES_V0 + 8
        );
    }

    #[test]
    fn build_tex_stats_json_builder_emits_exact_canonical_output() {
        let out = build_tex_stats_json_v0(22, 3, 14, 2, 3, 3, 1).expect("builder should succeed");
        assert_eq!(
            out,
            "{\"token_count\":22,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3,\"max_group_depth\":1}"
        );
    }

    #[test]
    fn validate_tex_stats_json_accepts_builder_output() {
        let out = build_tex_stats_json_v0(22, 3, 14, 2, 3, 3, 1).expect("builder should succeed");
        assert!(validate_tex_stats_json_v0(&out).is_ok());
    }

    #[test]
    fn validate_tex_stats_json_rejects_extra_key() {
        let bad = "{\"token_count\":22,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3,\"max_group_depth\":1,\"unexpected_key\":1}";
        assert!(validate_tex_stats_json_v0(bad).is_err());
    }

    #[test]
    fn validate_tex_stats_json_rejects_missing_key() {
        let bad = "{\"token_count\":22,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3}";
        assert!(validate_tex_stats_json_v0(bad).is_err());
    }

    #[test]
    fn validate_tex_stats_json_rejects_whitespace() {
        let bad = "{\"token_count\":22,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3,\"max_group_depth\":1}\n";
        assert!(validate_tex_stats_json_v0(bad).is_err());
    }

    #[test]
    fn validate_tex_stats_json_rejects_negative_or_non_digit_or_empty() {
        let negative = "{\"token_count\":-22,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3,\"max_group_depth\":1}";
        assert!(validate_tex_stats_json_v0(negative).is_err());
        let non_digit = "{\"token_count\":x,\"control_seq_count\":3,\"char_count\":14,\"space_count\":2,\"begin_group_count\":3,\"end_group_count\":3,\"max_group_depth\":1}";
        assert!(validate_tex_stats_json_v0(non_digit).is_err());
        assert!(validate_tex_stats_json_v0("").is_err());
    }

    #[test]
    fn validate_input_trace_json_accepts_known_good_sample() {
        let sample =
            "{\"expansions\":1,\"max_depth\":1,\"unique_files\":2,\"files\":[\"main.tex\",\"sub.tex\"]}";
        assert!(validate_input_trace_json_v0(sample).is_ok());
    }

    #[test]
    fn validate_input_trace_json_rejects_whitespace() {
        let bad = "{\"expansions\":1, \"max_depth\":1,\"unique_files\":2,\"files\":[\"main.tex\"]}";
        assert!(validate_input_trace_json_v0(bad).is_err());
    }

    #[test]
    fn validate_input_trace_json_rejects_wrong_key_order() {
        let bad = "{\"max_depth\":1,\"expansions\":1,\"unique_files\":2,\"files\":[\"main.tex\"]}";
        assert!(validate_input_trace_json_v0(bad).is_err());
    }

    #[test]
    fn validate_input_trace_json_rejects_missing_or_extra_key() {
        let missing = "{\"expansions\":1,\"max_depth\":1,\"files\":[\"main.tex\"]}";
        assert!(validate_input_trace_json_v0(missing).is_err());

        let extra = "{\"expansions\":1,\"max_depth\":1,\"unique_files\":2,\"files\":[\"main.tex\"],\"extra\":1}";
        assert!(validate_input_trace_json_v0(extra).is_err());
    }

    #[test]
    fn validate_input_trace_json_rejects_bad_escape() {
        let bad = "{\"expansions\":1,\"max_depth\":1,\"unique_files\":2,\"files\":[\"a\\x\"]}";
        assert!(validate_input_trace_json_v0(bad).is_err());
    }

    #[test]
    fn validate_input_trace_json_rejects_non_digit_number() {
        let bad = "{\"expansions\":-1,\"max_depth\":1,\"unique_files\":2,\"files\":[\"main.tex\"]}";
        assert!(validate_input_trace_json_v0(bad).is_err());
    }
}
