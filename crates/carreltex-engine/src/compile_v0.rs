#[cfg(test)]
mod count_v0_tests;
#[cfg(test)]
mod edef_v0_tests;
mod ifnum_v0;
#[cfg(test)]
mod ifnum_v0_tests;
mod ifx_v0;
#[cfg(test)]
mod ifx_v0_tests;
mod input_expand_v0;
#[cfg(test)]
mod input_macro_v0_tests;
mod macro_expand_v0;
#[cfg(test)]
mod meaning_v0_tests_base;
#[cfg(test)]
mod meaning_v0_tests_input_guards;
#[cfg(test)]
mod newcommand_v0_tests;
mod ok_v0;
#[cfg(test)]
mod ok_v0_tests;
#[cfg(test)]
mod providecommand_v0_tests;
mod stats_v0;
mod tokenize_reason_v0;
#[cfg(test)]
mod tokenizer_textword_139_tests;
#[cfg(test)]
mod tokenizer_textword_140_tests;
#[cfg(test)]
mod tokenizer_textword_141_tests;
#[cfg(test)]
mod tokenizer_textword_142_tests;
#[cfg(test)]
mod tokenizer_textword_143_tests;
#[cfg(test)]
mod tokenizer_textword_144_tests;
#[cfg(test)]
mod tokenizer_textword_145_tests;
#[cfg(test)]
mod tokenizer_textword_147_tests;
#[cfg(test)]
mod tokenizer_textword_148_tests;
mod trace_v0;
#[cfg(test)]
mod xdef_noexpand_v0_tests;
use crate::reasons_v0::{invalid_log_bytes_v0, InvalidInputReasonV0};
use crate::tex::tokenize_v0::{tokenize_v0, TokenV0, MAX_TOKENS_V0};
use carreltex_core::{
    build_compile_result_v0, truncate_log_bytes_v0, CompileRequestV0, CompileResultV0,
    CompileStatus, Mount, DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, MAX_LOG_BYTES_V0,
};
use carreltex_xdv::{
    validate_dvi_v2_text_page_v0, write_dvi_v2_text_page_with_layout_wrap_and_paging_v0,
    DEFAULT_MAX_LINES_PER_PAGE_V0, DEFAULT_MAX_LINE_GLYPHS_V0,
};
use input_expand_v0::expand_inputs_v0;
use macro_expand_v0::expand_macros_v0;
use ok_v0::{
    extract_strict_ok_text_body_v0, MAX_OK_TEXT_BYTES_V0, OK_GLYPH_ADVANCE_SP_V0,
    OK_LINE_ADVANCE_SP_V0,
};
use stats_v0::build_tex_stats_from_tokens_v0;
use trace_v0::build_not_implemented_log_v0;
const MISSING_COMPONENTS_V0: &[&str] = &["tex-engine"];
const EMPTY_TEX_STATS_JSON: &str = "";
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
        ok_max_line_glyphs_v0: None,
        ok_max_lines_per_page_v0: None,
        ok_line_advance_sp_v0: None,
        ok_glyph_advance_sp_v0: None,
    };
    compile_request_v0(mount, &request)
}

pub fn compile_request_v0(mount: &mut Mount, req: &CompileRequestV0) -> CompileResultV0 {
    // INVALID_INPUT reason precedence SSOT: request -> finalize -> read -> tokenize -> input -> macro -> stats.
    if req.entrypoint != "main.tex" || req.source_date_epoch == 0 || req.max_log_bytes == 0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }
    if req.max_log_bytes > MAX_LOG_BYTES_V0 {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
    }
    if let Some(value) = req.ok_max_line_glyphs_v0 {
        if !(1..=256).contains(&value) {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
        }
    }
    if let Some(value) = req.ok_max_lines_per_page_v0 {
        if !(1..=200).contains(&value) {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
        }
    }
    if let Some(value) = req.ok_line_advance_sp_v0 {
        if !(1..=8_388_607).contains(&value) {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
        }
    }
    if let Some(value) = req.ok_glyph_advance_sp_v0 {
        if !(1..=8_388_607).contains(&value) {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::RequestInvalid);
        }
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
        Err(error) => {
            return invalid_result_v0(
                req.max_log_bytes,
                tokenize_reason_v0::map_tokenize_error_to_reason_v0(error),
            )
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

    let macro_expanded_tokens = match expand_macros_v0(&expanded_tokens) {
        Ok(tokens) => tokens,
        Err(reason) => return invalid_result_v0(req.max_log_bytes, reason),
    };

    let tex_stats_json = match build_tex_stats_from_tokens_v0(&macro_expanded_tokens) {
        Ok(json) => json,
        Err(_) => {
            return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
        }
    };
    if tex_stats_json.is_empty() {
        return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed);
    }
    let ok_text_bytes = match (
        extract_strict_ok_text_body_v0(&expanded_tokens),
        extract_strict_ok_text_body_v0(&macro_expanded_tokens),
    ) {
        (Some(pre_macro), Some(post_macro)) if pre_macro == post_macro => Some(post_macro),
        _ => None,
    };

    if let Some(ok_text_bytes) = ok_text_bytes {
        if ok_text_bytes.len() <= MAX_OK_TEXT_BYTES_V0 {
            let max_line_glyphs =
                req.ok_max_line_glyphs_v0
                    .unwrap_or(DEFAULT_MAX_LINE_GLYPHS_V0 as u32) as usize;
            let max_lines_per_page =
                req.ok_max_lines_per_page_v0
                    .unwrap_or(DEFAULT_MAX_LINES_PER_PAGE_V0 as u32) as usize;
            let line_advance_sp = req.ok_line_advance_sp_v0.unwrap_or(OK_LINE_ADVANCE_SP_V0);
            let glyph_advance_sp = req.ok_glyph_advance_sp_v0.unwrap_or(OK_GLYPH_ADVANCE_SP_V0);
            let xdv_bytes = match write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(
                &ok_text_bytes,
                glyph_advance_sp,
                line_advance_sp,
                max_line_glyphs,
                max_lines_per_page,
            ) {
                Some(bytes) => bytes,
                None => {
                    return invalid_result_v0(
                        req.max_log_bytes,
                        InvalidInputReasonV0::StatsBuildFailed,
                    )
                }
            };
            if !validate_dvi_v2_text_page_v0(&xdv_bytes) {
                return invalid_result_v0(
                    req.max_log_bytes,
                    InvalidInputReasonV0::StatsBuildFailed,
                );
            }
            return build_compile_result_v0(
                CompileStatus::Ok,
                &[],
                vec![],
                xdv_bytes,
                tex_stats_json,
            );
        }
    }

    let not_implemented_log =
        match build_not_implemented_log_v0(req.max_log_bytes as usize, &input_trace) {
            Some(log) => log,
            None => {
                return invalid_result_v0(req.max_log_bytes, InvalidInputReasonV0::StatsBuildFailed)
            }
        };
    build_compile_result_v0(
        CompileStatus::NotImplemented,
        MISSING_COMPONENTS_V0,
        truncate_log_bytes_v0(&not_implemented_log, req.max_log_bytes),
        vec![],
        tex_stats_json,
    )
}

#[cfg(test)]
mod tests;
