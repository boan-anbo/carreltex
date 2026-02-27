use std::collections::BTreeMap;

use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

pub(crate) const MAX_MACROS_V0: usize = 4096;
pub(crate) const MAX_MACRO_EXPANSIONS_V0: usize = 4096;
pub(crate) const MAX_MACRO_DEPTH_V0: usize = 64;

pub(crate) fn expand_macros_v0(tokens: &[TokenV0]) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    let mut macros = BTreeMap::<Vec<u8>, Vec<TokenV0>>::new();
    let mut output = Vec::<TokenV0>::new();
    let mut active_macros = Vec::<Vec<u8>>::new();
    let mut expansion_count = 0usize;
    expand_stream_v0(
        tokens,
        &mut macros,
        &mut output,
        &mut active_macros,
        &mut expansion_count,
        0,
    )?;
    Ok(output)
}

fn expand_stream_v0(
    tokens: &[TokenV0],
    macros: &mut BTreeMap<Vec<u8>, Vec<TokenV0>>,
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
            TokenV0::ControlSeq(name) if name.as_slice() == b"def" => {
                index = parse_def_v0(tokens, index, macros)?;
            }
            TokenV0::ControlSeq(name) => {
                if let Some(body) = macros.get(name).cloned() {
                    *expansion_count = expansion_count
                        .checked_add(1)
                        .ok_or(InvalidInputReasonV0::MacroExpansionsExceeded)?;
                    if *expansion_count > MAX_MACRO_EXPANSIONS_V0 {
                        return Err(InvalidInputReasonV0::MacroExpansionsExceeded);
                    }
                    if active_macros.iter().any(|active| active == name) {
                        return Err(InvalidInputReasonV0::MacroCycleFailed);
                    }
                    active_macros.push(name.clone());
                    let result = expand_stream_v0(
                        &body,
                        macros,
                        out,
                        active_macros,
                        expansion_count,
                        depth + 1,
                    );
                    active_macros.pop();
                    result?;
                } else {
                    push_checked_v0(out, tokens[index].clone())?;
                }
                index += 1;
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
    macros: &mut BTreeMap<Vec<u8>, Vec<TokenV0>>,
) -> Result<usize, InvalidInputReasonV0> {
    let name_index = def_index + 1;
    let macro_name = match tokens.get(name_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    };

    let body_start_index = name_index + 1;
    match tokens.get(body_start_index) {
        Some(TokenV0::Char(b'#')) => return Err(InvalidInputReasonV0::MacroParamsUnsupported),
        Some(TokenV0::BeginGroup) => {}
        _ => return Err(InvalidInputReasonV0::MacroValidationFailed),
    }

    let (body_tokens, next_index) = parse_balanced_group_payload_v0(tokens, body_start_index)?;
    if body_tokens
        .iter()
        .any(|token| matches!(token, TokenV0::Char(b'#')))
    {
        return Err(InvalidInputReasonV0::MacroParamsUnsupported);
    }
    if !macros.contains_key(&macro_name) && macros.len() >= MAX_MACROS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    macros.insert(macro_name, body_tokens);
    Ok(next_index)
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
