use std::collections::BTreeMap;

use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

pub(crate) const MAX_MACROS_V0: usize = 4096;
pub(crate) const MAX_MACRO_EXPANSIONS_V0: usize = 4096;
pub(crate) const MAX_MACRO_DEPTH_V0: usize = 64;

#[derive(Clone)]
struct MacroDefV0 {
    param_count: u8,
    body_tokens: Vec<TokenV0>,
}

pub(crate) fn expand_macros_v0(tokens: &[TokenV0]) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    let mut macro_frames = Vec::<BTreeMap<Vec<u8>, MacroDefV0>>::new();
    macro_frames.push(BTreeMap::new());
    let mut output = Vec::<TokenV0>::new();
    let mut active_macros = Vec::<Vec<u8>>::new();
    let mut expansion_count = 0usize;
    expand_stream_v0(
        tokens,
        &mut macro_frames,
        &mut output,
        &mut active_macros,
        &mut expansion_count,
        0,
    )?;
    Ok(output)
}

fn expand_stream_v0(
    tokens: &[TokenV0],
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroDefV0>>,
    out: &mut Vec<TokenV0>,
    active_macros: &mut Vec<Vec<u8>>,
    expansion_count: &mut usize,
    depth: usize,
) -> Result<(), InvalidInputReasonV0> {
    if depth > MAX_MACRO_DEPTH_V0 {
        return Err(InvalidInputReasonV0::MacroDepthExceeded);
    }

    let mut index = 0usize;
    while index < tokens.len() {
        match &tokens[index] {
            TokenV0::BeginGroup => {
                macro_frames.push(BTreeMap::new());
                push_checked_v0(out, TokenV0::BeginGroup)?;
                index += 1;
            }
            TokenV0::EndGroup => {
                if macro_frames.len() > 1 {
                    macro_frames.pop();
                }
                push_checked_v0(out, TokenV0::EndGroup)?;
                index += 1;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"def" || name.as_slice() == b"gdef" => {
                let is_global = name.as_slice() == b"gdef";
                index = parse_def_v0(tokens, index, macro_frames, is_global)?;
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"global" => {
                let next_index = index + 1;
                let is_global = match tokens.get(next_index) {
                    Some(TokenV0::ControlSeq(next_name)) if next_name.as_slice() == b"def" => true,
                    Some(TokenV0::ControlSeq(next_name)) if next_name.as_slice() == b"gdef" => true,
                    _ => return Err(InvalidInputReasonV0::MacroGlobalPrefixUnsupported),
                };
                index = parse_def_v0(tokens, next_index, macro_frames, is_global)?;
            }
            TokenV0::ControlSeq(name) => {
                if let Some(macro_def) = lookup_macro_v0(macro_frames, name) {
                    *expansion_count = expansion_count
                        .checked_add(1)
                        .ok_or(InvalidInputReasonV0::MacroExpansionsExceeded)?;
                    if *expansion_count > MAX_MACRO_EXPANSIONS_V0 {
                        return Err(InvalidInputReasonV0::MacroExpansionsExceeded);
                    }
                    if active_macros.iter().any(|active| active == name) {
                        return Err(InvalidInputReasonV0::MacroCycleFailed);
                    }
                    let (expanded_body, next_index) = match macro_def.param_count {
                        0 => (macro_def.body_tokens, index + 1),
                        1 => {
                            let (argument_tokens, argument_next_index) =
                                parse_balanced_group_payload_v0(tokens, index + 1)?;
                            let substituted_body = substitute_single_param_placeholders_v0(
                                &macro_def.body_tokens,
                                &argument_tokens,
                            )?;
                            (substituted_body, argument_next_index)
                        }
                        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
                    };
                    active_macros.push(name.clone());
                    let result = expand_stream_v0(
                        &expanded_body,
                        macro_frames,
                        out,
                        active_macros,
                        expansion_count,
                        depth + 1,
                    );
                    active_macros.pop();
                    result?;
                    index = next_index;
                } else {
                    push_checked_v0(out, tokens[index].clone())?;
                    index += 1;
                }
            }
            token => {
                push_checked_v0(out, token.clone())?;
                index += 1;
            }
        }
    }
    Ok(())
}

fn parse_def_v0(
    tokens: &[TokenV0],
    def_index: usize,
    macro_frames: &mut Vec<BTreeMap<Vec<u8>, MacroDefV0>>,
    is_global: bool,
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
        MacroDefV0 {
            param_count,
            body_tokens,
        },
    );
    Ok(next_index)
}

fn lookup_macro_v0(
    macro_frames: &[BTreeMap<Vec<u8>, MacroDefV0>],
    name: &[u8],
) -> Option<MacroDefV0> {
    for frame in macro_frames.iter().rev() {
        if let Some(macro_def) = frame.get(name) {
            return Some(macro_def.clone());
        }
    }
    None
}

fn total_macro_defs_v0(macro_frames: &[BTreeMap<Vec<u8>, MacroDefV0>]) -> usize {
    macro_frames.iter().map(|frame| frame.len()).sum()
}

fn validate_macro_body_tokens_v0(
    body_tokens: &[TokenV0],
    param_count: u8,
) -> Result<(), InvalidInputReasonV0> {
    let mut index = 0usize;
    while index < body_tokens.len() {
        match body_tokens.get(index) {
            Some(TokenV0::Char(b'#')) => match param_count {
                0 => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
                1 => match body_tokens.get(index + 1) {
                    Some(TokenV0::Char(b'1')) => index += 2,
                    _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
                },
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            },
            Some(_) => index += 1,
            None => break,
        }
    }
    Ok(())
}

fn substitute_single_param_placeholders_v0(
    body_tokens: &[TokenV0],
    argument_tokens: &[TokenV0],
) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    let mut out = Vec::<TokenV0>::new();
    let mut index = 0usize;
    while index < body_tokens.len() {
        match body_tokens.get(index) {
            Some(TokenV0::Char(b'#')) => match body_tokens.get(index + 1) {
                Some(TokenV0::Char(b'1')) => {
                    for token in argument_tokens {
                        push_checked_v0(&mut out, token.clone())?;
                    }
                    index += 2;
                }
                _ => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
            },
            Some(token) => {
                push_checked_v0(&mut out, token.clone())?;
                index += 1;
            }
            None => break,
        }
    }
    Ok(out)
}

fn parse_balanced_group_payload_v0(
    tokens: &[TokenV0],
    begin_group_index: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    if !matches!(tokens.get(begin_group_index), Some(TokenV0::BeginGroup)) {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }

    let mut depth = 1usize;
    let mut payload = Vec::<TokenV0>::new();
    let mut index = begin_group_index + 1;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::BeginGroup) => {
                depth += 1;
                payload.push(TokenV0::BeginGroup);
            }
            Some(TokenV0::EndGroup) => {
                depth -= 1;
                if depth == 0 {
                    return Ok((payload, index + 1));
                }
                payload.push(TokenV0::EndGroup);
            }
            Some(token) => payload.push(token.clone()),
            None => break,
        }
        index += 1;
    }
    Err(InvalidInputReasonV0::MacroValidationFailed)
}

fn push_checked_v0(out: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    out.push(token);
    Ok(())
}
