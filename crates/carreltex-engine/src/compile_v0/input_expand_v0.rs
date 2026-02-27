use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{tokenize_v0, TokenV0, MAX_TOKENS_V0};
use carreltex_core::{normalize_path_v0, Mount};

use super::trace_v0::InputTraceV0;

pub(crate) const MAX_INPUT_DEPTH_V0: usize = 32;
pub(crate) const MAX_INPUT_EXPANSIONS_V0: usize = 1024;

pub(crate) fn expand_inputs_v0(
    tokens: &[TokenV0],
    mount: &Mount,
) -> Result<(Vec<TokenV0>, InputTraceV0), InvalidInputReasonV0> {
    let mut active_stack = vec!["main.tex".to_owned()];
    let mut expansion_count = 0usize;
    let mut trace = InputTraceV0::new();
    let expanded = expand_inputs_inner_v0(
        tokens,
        mount,
        0,
        &mut active_stack,
        &mut expansion_count,
        &mut trace,
    )?;
    Ok((expanded, trace))
}

fn parse_input_path_group_v0(
    tokens: &[TokenV0],
    input_index: usize,
) -> Result<(String, usize), InvalidInputReasonV0> {
    if !matches!(
        tokens.get(input_index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"input"
    ) {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }

    let mut index = input_index + 1;
    let mut path_bytes = Vec::new();
    if matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        index += 1;
        loop {
            match tokens.get(index) {
                Some(TokenV0::Char(byte)) => {
                    path_bytes.push(*byte);
                    index += 1;
                }
                Some(TokenV0::EndGroup) => {
                    index += 1;
                    break;
                }
                Some(TokenV0::Space)
                | Some(TokenV0::ControlSeq(_))
                | Some(TokenV0::BeginGroup)
                | None => {
                    return Err(InvalidInputReasonV0::InputValidationFailed);
                }
            }
        }
    } else {
        while let Some(TokenV0::Char(byte)) = tokens.get(index) {
            path_bytes.push(*byte);
            index += 1;
        }
    }

    if path_bytes.is_empty() {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }

    let mut normalized =
        normalize_path_v0(&path_bytes).map_err(|_| InvalidInputReasonV0::InputValidationFailed)?;
    if !normalized.ends_with(".tex") {
        normalized.push_str(".tex");
    }
    Ok((normalized, index))
}

fn push_token_checked(out: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    out.push(token);
    Ok(())
}

fn extend_tokens_checked(
    out: &mut Vec<TokenV0>,
    tokens: &[TokenV0],
) -> Result<(), InvalidInputReasonV0> {
    let total = out
        .len()
        .checked_add(tokens.len())
        .ok_or(InvalidInputReasonV0::InputValidationFailed)?;
    if total > MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::InputValidationFailed);
    }
    out.extend_from_slice(tokens);
    Ok(())
}

fn expand_inputs_inner_v0(
    tokens: &[TokenV0],
    mount: &Mount,
    depth: usize,
    active_stack: &mut Vec<String>,
    expansion_count: &mut usize,
    trace: &mut InputTraceV0,
) -> Result<Vec<TokenV0>, InvalidInputReasonV0> {
    if depth > MAX_INPUT_DEPTH_V0 {
        return Err(InvalidInputReasonV0::InputDepthExceeded);
    }

    let mut out = Vec::new();
    let mut index = 0usize;
    while index < tokens.len() {
        match &tokens[index] {
            TokenV0::ControlSeq(name) if name.as_slice() == b"input" => {
                *expansion_count = expansion_count
                    .checked_add(1)
                    .ok_or(InvalidInputReasonV0::InputExpansionsExceeded)?;
                if *expansion_count > MAX_INPUT_EXPANSIONS_V0 {
                    return Err(InvalidInputReasonV0::InputExpansionsExceeded);
                }
                trace.expansions = *expansion_count as u64;

                let (normalized_path, next_index) = parse_input_path_group_v0(tokens, index)?;
                if active_stack.iter().any(|path| path == &normalized_path) {
                    return Err(InvalidInputReasonV0::InputCycleFailed);
                }
                trace.record_file(&normalized_path);
                trace.record_depth(depth + 1);

                let included_bytes = match mount.read_file_by_bytes_v0(normalized_path.as_bytes()) {
                    Ok(Some(bytes)) => bytes,
                    _ => return Err(InvalidInputReasonV0::InputValidationFailed),
                };
                let included_tokens = tokenize_v0(included_bytes)
                    .map_err(|_| InvalidInputReasonV0::InputValidationFailed)?;

                active_stack.push(normalized_path);
                let expanded = expand_inputs_inner_v0(
                    &included_tokens,
                    mount,
                    depth + 1,
                    active_stack,
                    expansion_count,
                    trace,
                )?;
                active_stack.pop();

                extend_tokens_checked(&mut out, &expanded)?;
                index = next_index;
            }
            token => {
                push_token_checked(&mut out, token.clone())?;
                index += 1;
            }
        }
    }

    Ok(out)
}
