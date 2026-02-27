use super::*;
use super::bindings::{snapshot_let_binding_v0, total_macro_defs_v0, MacroBindingV0};
use super::utils::skip_space_tokens_v0;

pub(super) fn parse_let_v0(
    tokens: &[TokenV0],
    let_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let alias_name = match tokens.get(let_index + 1) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    };

    let mut index = skip_space_tokens_v0(tokens, let_index + 2);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'='))) {
        index = skip_space_tokens_v0(tokens, index + 1);
    }

    let target_name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroLetUnsupported),
    };
    let resolved_binding = snapshot_let_binding_v0(&target_name, macro_frames)?;
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
    if !target_frame.contains_key(&alias_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        alias_name,
        MacroBindingV0::LetAlias {
            target_name,
            resolved_binding: Box::new(resolved_binding),
        },
    );
    Ok(index + 1)
}

pub(super) fn parse_futurelet_v0(
    tokens: &[TokenV0],
    futurelet_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
    is_global: bool,
) -> Result<usize, InvalidInputReasonV0> {
    let alias_name_index = skip_space_tokens_v0(tokens, futurelet_index + 1);
    let alias_name = match tokens.get(alias_name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
    };

    let probe_index = skip_space_tokens_v0(tokens, alias_name_index + 1);
    let _probe_name = match tokens.get(probe_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
    };

    let target_index = skip_space_tokens_v0(tokens, probe_index + 1);
    let target_name = match tokens.get(target_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroFutureletUnsupported),
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
    if !target_frame.contains_key(&alias_name) && total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(alias_name, MacroBindingV0::ControlSeqLiteral(target_name));
    Ok(probe_index)
}
