use crate::mount::{Error, Mount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileStatus {
    Ok = 0,
    InvalidInput = 1,
    NotImplemented = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileReport {
    pub status: CompileStatus,
    pub missing_components: &'static [&'static str],
}

impl CompileReport {
    pub fn to_canonical_json(&self) -> String {
        // Manual canonical JSON (stable key order, no serde dependency).
        // Keys are always: status, missing_components.
        let status_str = match self.status {
            CompileStatus::Ok => "OK",
            CompileStatus::InvalidInput => "INVALID_INPUT",
            CompileStatus::NotImplemented => "NOT_IMPLEMENTED",
        };

        let mut out = String::new();
        out.push('{');
        out.push_str("\"missing_components\":[");
        for (idx, comp) in self.missing_components.iter().enumerate() {
            if idx != 0 {
                out.push(',');
            }
            out.push('"');
            out.push_str(&escape_json_string(comp));
            out.push('"');
        }
        out.push_str("],\"status\":\"");
        out.push_str(status_str);
        out.push_str("\"}");
        out
    }
}

pub fn compile_main_v0(mount: &mut Mount) -> (CompileStatus, CompileReport) {
    if mount.finalize().is_err() {
        return (
            CompileStatus::InvalidInput,
            CompileReport {
                status: CompileStatus::InvalidInput,
                missing_components: &[],
            },
        );
    }

    (
        CompileStatus::NotImplemented,
        CompileReport {
            status: CompileStatus::NotImplemented,
            missing_components: &["tex-engine"],
        },
    )
}

fn escape_json_string(value: &str) -> String {
    // Minimal, fail-closed escaping: reject control chars by escaping them.
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
    // Very small guard: ensure it is non-empty UTF-8 and contains required keys.
    // The full JSON parsing contract is enforced by the JS proof (Leaf 24).
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
    use super::{compile_main_v0, CompileStatus};
    use crate::mount::Mount;

    #[test]
    fn compile_requires_valid_mount() {
        let mut mount = Mount::default();
        let (status, report) = compile_main_v0(&mut mount);
        assert_eq!(status, CompileStatus::InvalidInput);
        assert_eq!(report.status, CompileStatus::InvalidInput);
    }

    #[test]
    fn compile_returns_not_implemented_when_mount_valid() {
        let mut mount = Mount::default();
        let main = b"\\documentclass{article}\n\\begin{document}\nHi\n\\end{document}\n";
        assert!(mount.add_file(b"main.tex", main).is_ok());

        let (status, report) = compile_main_v0(&mut mount);
        assert_eq!(status, CompileStatus::NotImplemented);
        let json = report.to_canonical_json();
        assert!(json.contains("\"status\":\"NOT_IMPLEMENTED\""));
        assert!(json.contains("\"missing_components\""));
        assert!(json.contains("tex-engine"));
    }
}

