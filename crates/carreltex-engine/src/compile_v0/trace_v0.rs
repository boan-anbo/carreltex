use carreltex_core::validate_input_trace_json_v0;

pub(crate) const INPUT_TRACE_PREFIX_BYTES: &[u8] = b"\nINPUT_TRACE_V0:";
const INPUT_TRACE_MAX_FILES_V0: usize = 32;

#[derive(Default)]
pub(crate) struct InputTraceV0 {
    pub(crate) expansions: u64,
    pub(crate) max_depth: u64,
    pub(crate) unique_files: u64,
    pub(crate) files: Vec<String>,
}

impl InputTraceV0 {
    pub(crate) fn new() -> Self {
        let mut trace = Self::default();
        trace.record_file("main.tex");
        trace
    }

    pub(crate) fn record_file(&mut self, path: &str) {
        if self.files.iter().any(|existing| existing == path) {
            return;
        }
        self.unique_files = self.unique_files.saturating_add(1);
        if self.files.len() < INPUT_TRACE_MAX_FILES_V0 {
            self.files.push(path.to_owned());
        }
    }

    pub(crate) fn record_depth(&mut self, depth: usize) {
        let depth_u64 = depth as u64;
        if depth_u64 > self.max_depth {
            self.max_depth = depth_u64;
        }
    }
}

pub(crate) fn build_input_trace_json_v0(trace: &InputTraceV0) -> String {
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

pub(crate) fn build_not_implemented_log_v0(
    max_log_bytes: usize,
    trace: &InputTraceV0,
) -> Option<Vec<u8>> {
    let trace_json = build_input_trace_json_v0(trace);
    if validate_input_trace_json_v0(&trace_json).is_err() {
        return None;
    }

    let mut trace_line = Vec::new();
    trace_line.extend_from_slice(INPUT_TRACE_PREFIX_BYTES);
    trace_line.extend_from_slice(trace_json.as_bytes());

    let mut not_implemented_log =
        b"NOT_IMPLEMENTED: tex-engine compile pipeline is not wired yet".to_vec();
    if not_implemented_log
        .len()
        .checked_add(trace_line.len())
        .is_some_and(|total| total <= max_log_bytes)
    {
        not_implemented_log.extend_from_slice(&trace_line);
    }
    Some(not_implemented_log)
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
