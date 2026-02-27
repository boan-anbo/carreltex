use super::*;
use super::utils::skip_space_tokens_v0;

pub(super) fn parse_expandafter_v0(
    tokens: &[TokenV0],
    expandafter_index: usize,
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    let first_index = skip_space_tokens_v0(tokens, expandafter_index + 1);
    let first_name = match tokens.get(first_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroExpandafterUnsupported),
    };

    let second_index = skip_space_tokens_v0(tokens, first_index + 1);
    let second_name = match tokens.get(second_index) {
        Some(TokenV0::ControlSeq(name)) => name.clone(),
        _ => return Err(InvalidInputReasonV0::MacroExpandafterUnsupported),
    };

    Ok((
        vec![TokenV0::ControlSeq(second_name), TokenV0::ControlSeq(first_name)],
        second_index + 1,
    ))
}

pub(super) fn parse_csname_v0(
    tokens: &[TokenV0],
    csname_index: usize,
) -> Result<(TokenV0, usize), InvalidInputReasonV0> {
    let mut name_bytes = Vec::<u8>::new();
    let mut index = csname_index + 1;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"endcsname" => {
                if name_bytes.is_empty() {
                    return Err(InvalidInputReasonV0::MacroCsnameUnsupported);
                }
                return Ok((TokenV0::ControlSeq(name_bytes), index + 1));
            }
            Some(TokenV0::Char(byte)) => name_bytes.push(*byte),
            _ => return Err(InvalidInputReasonV0::MacroCsnameUnsupported),
        }
        index += 1;
    }
    Err(InvalidInputReasonV0::MacroCsnameUnsupported)
}
