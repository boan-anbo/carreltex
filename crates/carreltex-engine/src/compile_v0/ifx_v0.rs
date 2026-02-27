use super::ifnum_v0::parse_ifnum_v0;
use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

pub(crate) const MAX_IFX_DEPTH_V0: usize = 64;

pub(crate) fn parse_ifx_v0<F>(
    tokens: &[TokenV0],
    ifx_index: usize,
    counters: &[u32; 2],
    if_depth: usize,
    compare_control_seq: &mut F,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0>
where
    F: FnMut(&[u8], &[u8]) -> bool,
{
    if if_depth >= MAX_IFX_DEPTH_V0 {
        return Err(InvalidInputReasonV0::MacroIfxDepthExceeded);
    }

    let mut index = skip_space_tokens_v0(tokens, ifx_index + 1);
    let left = parse_ifx_operand_v0(tokens, &mut index)?;
    index = skip_space_tokens_v0(tokens, index);
    let right = parse_ifx_operand_v0(tokens, &mut index)?;
    let condition = compare_control_seq(&left, &right);

    let mut out = Vec::<TokenV0>::new();
    let mut in_else = false;
    let mut saw_else = false;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"ifx" => {
                let (nested_tokens, next_index) =
                    parse_ifx_v0(tokens, index, counters, if_depth + 1, compare_control_seq)?;
                if branch_selected_v0(condition, in_else) {
                    for token in nested_tokens {
                        push_checked_v0(&mut out, token)?;
                    }
                }
                index = next_index;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"ifnum" => {
                let (nested_tokens, next_index) = parse_ifnum_v0(tokens, index, counters, 0)?;
                if branch_selected_v0(condition, in_else) {
                    for token in nested_tokens {
                        push_checked_v0(&mut out, token)?;
                    }
                }
                index = next_index;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"fi" => {
                return Ok((out, index + 1));
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"else" => {
                if saw_else {
                    return Err(InvalidInputReasonV0::MacroIfxElseDuplicate);
                }
                saw_else = true;
                in_else = true;
                index += 1;
            }
            Some(token) => {
                if branch_selected_v0(condition, in_else) {
                    push_checked_v0(&mut out, token.clone())?;
                }
                index += 1;
            }
            None => break,
        }
    }

    Err(InvalidInputReasonV0::MacroIfxMissingFi)
}

fn parse_ifx_operand_v0(
    tokens: &[TokenV0],
    index: &mut usize,
) -> Result<Vec<u8>, InvalidInputReasonV0> {
    match tokens.get(*index) {
        Some(TokenV0::ControlSeq(name)) => {
            *index += 1;
            Ok(name.clone())
        }
        _ => Err(InvalidInputReasonV0::MacroIfxUnsupported),
    }
}

fn branch_selected_v0(condition: bool, in_else: bool) -> bool {
    (condition && !in_else) || (!condition && in_else)
}

fn skip_space_tokens_v0(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn push_checked_v0(out: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    out.push(token);
    Ok(())
}
