use super::*;
use super::bindings::{lookup_macro_binding_v0, total_macro_defs_v0, MacroBindingV0, MacroDefV0};
use super::utils::{parse_balanced_group_payload_v0, validate_macro_body_tokens_v0};

pub(super) fn parse_providecommand_v0(
    tokens: &[TokenV0],
    command_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroBindingV0>>,
) -> Result<usize, InvalidInputReasonV0> {
    let unsupported_reason = InvalidInputReasonV0::MacroProvidecommandUnsupported;
    let name_group_index = command_index + 1;
    let (macro_name, mut index) =
        parse_braced_control_seq_name_v0(tokens, name_group_index).ok_or(unsupported_reason)?;

    if matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }

    let mut param_count = 0u8;
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        if !matches!(
            (tokens.get(index + 1), tokens.get(index + 2)),
            (Some(TokenV0::Char(b'1')), Some(TokenV0::Char(b']')))
        ) {
            return Err(unsupported_reason);
        }
        param_count = 1;
        index += 3;
        if matches!(tokens.get(index), Some(TokenV0::Space)) {
            index += 1;
        }
    }

    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return Err(unsupported_reason);
    }
    let (body_tokens, next_index) =
        parse_balanced_group_payload_v0(tokens, index).map_err(|_| unsupported_reason)?;
    validate_macro_body_tokens_v0(&body_tokens, param_count)?;

    if lookup_macro_binding_v0(macro_frames, &macro_name).is_some() {
        return Ok(next_index);
    }

    let target_frame_index = macro_frames
        .len()
        .checked_sub(1)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    let total_macro_defs = total_macro_defs_v0(macro_frames);
    let target_frame = macro_frames
        .get_mut(target_frame_index)
        .ok_or(InvalidInputReasonV0::MacroValidationFailed)?;
    if total_macro_defs >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    target_frame.insert(
        macro_name,
        MacroBindingV0::Macro(MacroDefV0 {
            param_count,
            body_tokens,
        }),
    );
    Ok(next_index)
}

fn parse_braced_control_seq_name_v0(tokens: &[TokenV0], index: usize) -> Option<(Vec<u8>, usize)> {
    let (name_group_tokens, next_index) = parse_balanced_group_payload_v0(tokens, index).ok()?;
    if name_group_tokens.len() != 1 {
        return None;
    }
    match &name_group_tokens[0] {
        TokenV0::ControlSeq(name) => Some((name.clone(), next_index)),
        _ => None,
    }
}
