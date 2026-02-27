use super::*;
use super::bindings::{total_macro_defs_v0, MacroBindingV0, MacroDefV0};
use super::utils::{parse_balanced_group_payload_v0, validate_macro_body_tokens_v0};

pub(super) fn parse_def_v0(
    tokens: &[TokenV0],
    def_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    is_global: bool,
    expand_body: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let name_index = def_index + 1;
    let macro_name = match tokens.get(name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    };

    let mut param_count = 0u8;
    let mut body_start_index = name_index + 1;
    match tokens.get(body_start_index) {
        Some(TokenV0::BeginGroup) => {}
        Some(TokenV0::Char(b'#')) => {
            if expand_body {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            let placeholder_digit = match tokens.get(body_start_index + 1) {
                Some(TokenV0::Char(digit)) => *digit,
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            };
            if placeholder_digit != b'1' {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            param_count = 1;
            body_start_index += 2;
            if matches!(tokens.get(body_start_index), Some(TokenV0::Char(b'#'))) {
                return Err(InvalidInputReasonV0::MacroParamsUnsupported);
            }
            if !matches!(tokens.get(body_start_index), Some(TokenV0::BeginGroup)) {
                return Err(InvalidInputReasonV0::MacroValidationFailed);
            }
        }
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    }

    let (body_tokens, next_index) = parse_balanced_group_payload_v0(tokens, body_start_index)?;
    validate_macro_body_tokens_v0(&body_tokens, param_count)?;
    let final_body_tokens = if expand_body {
        if param_count != 0 {
            return Err(InvalidInputReasonV0::MacroParamsUnsupported);
        }
        let mut expanded = Vec::<TokenV0>::new();
        let mut active_macros = Vec::<Vec<u8>>::new();
        let mut expansion_count = 0usize;
        super::expand_stream_v0(
            &body_tokens,
            macro_frames,
            counters,
            &mut expanded,
            &mut active_macros,
            &mut expansion_count,
            0,
        )?;
        expanded
    } else {
        body_tokens
    };
    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&macro_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    if param_count > 1 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        macro_name,
        MacroBindingV0::Macro(MacroDefV0 {
            param_count: if expand_body { 0 } else { param_count },
            body_tokens: final_body_tokens,
        }),
    );
    Ok(next_index)
}

pub(super) fn parse_xdef_v0(
    tokens: &[TokenV0],
    xdef_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    counters: &mut [u32; 2],
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let name_index = xdef_index + 1;
    let macro_name = match tokens.get(name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroXdefUnsupported),
    };
    let body_start_index = name_index + 1;
    if !matches!(tokens.get(body_start_index), Some(TokenV0::BeginGroup)) {
        return Err(InvalidInputReasonV0::MacroXdefUnsupported);
    }
    let (body_tokens, next_index) = parse_balanced_group_payload_v0(tokens, body_start_index)
        .map_err(|_| InvalidInputReasonV0::MacroXdefUnsupported)?;
    if body_tokens
        .iter()
        .any(|token| matches!(token, TokenV0::Char(b'#')))
    {
        return Err(InvalidInputReasonV0::MacroXdefUnsupported);
    }

    let mut expanded = Vec::<TokenV0>::new();
    let mut active_macros = Vec::<Vec<u8>>::new();
    let mut expansion_count = 0usize;
    super::expand_stream_v0(
        &body_tokens,
        macro_frames,
        counters,
        &mut expanded,
        &mut active_macros,
        &mut expansion_count,
        0,
    )?;

    let target_frame_index = if is_global {
        0usize
    } else {
        macro_frames
            .len()
            .checked_sub(1)
            .ok_or(InvalidInputReasonV0::MacroValidationFailed)?
    };
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if !target_frame.contains_key(&macro_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        macro_name,
        MacroBindingV0::Macro(MacroDefV0 {
            param_count: 0,
            body_tokens: expanded,
        }),
    );
    Ok(next_index)
}
