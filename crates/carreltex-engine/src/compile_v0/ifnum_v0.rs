use crate::reasons_v0::InvalidInputReasonV0;
use crate::tex::tokenize_v0::{TokenV0, MAX_TOKENS_V0};

pub(crate) const MAX_IF_DEPTH_V0: usize = 64;

pub(crate) fn parse_ifnum_v0(
    tokens: &[TokenV0],
    ifnum_index: usize,
    counters: &[u32; 2],
    if_depth: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    if if_depth >= MAX_IF_DEPTH_V0 {
        return Err(InvalidInputReasonV0::MacroIfDepthExceeded);
    }

    let mut index = skip_space_tokens_v0(tokens, ifnum_index + 1);
    let left = parse_ifnum_count_operand_v0(tokens, &mut index, counters)?;
    index = skip_space_tokens_v0(tokens, index);
    let operator = match tokens.get(index) {
        Some(TokenV0::Char(b'<')) => b'<',
        Some(TokenV0::Char(b'=')) => b'=',
        Some(TokenV0::Char(b'>')) => b'>',
        _ => return Err(InvalidInputReasonV0::MacroIfnumUnsupported),
    };
    index += 1;
    index = skip_space_tokens_v0(tokens, index);
    let right = parse_ifnum_count_operand_v0(tokens, &mut index, counters)?;
    let condition = match operator {
        b'<' => left < right,
        b'=' => left == right,
        b'>' => left > right,
        _ => return Err(InvalidInputReasonV0::MacroIfnumUnsupported),
    };

    let mut out = Vec::<TokenV0>::new();
    let mut in_else = false;
    let mut saw_else = false;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"ifnum" => {
                let (nested_tokens, next_index) = parse_ifnum_v0(tokens, index, counters, if_depth + 1)?;
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
                    return Err(InvalidInputReasonV0::MacroIfElseDuplicate);
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

    Err(InvalidInputReasonV0::MacroIfMissingFi)
}

fn parse_ifnum_count_operand_v0(
    tokens: &[TokenV0],
    index: &mut usize,
    counters: &[u32; 2],
) -> Result<u32, InvalidInputReasonV0> {
    if !matches!(tokens.get(*index), Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"count")
    {
        return Err(InvalidInputReasonV0::MacroIfnumUnsupported);
    }
    *index += 1;
    match tokens.get(*index) {
        Some(TokenV0::Char(b'0')) => {
            *index += 1;
            Ok(counters[0])
        }
        Some(TokenV0::Char(b'1')) => {
            *index += 1;
            Ok(counters[1])
        }
        _ => Err(InvalidInputReasonV0::MacroIfnumUnsupported),
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
