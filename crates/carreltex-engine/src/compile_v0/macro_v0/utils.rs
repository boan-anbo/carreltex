use super::*;

pub(super) fn skip_space_tokens_v0(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

pub(super) fn push_checked_v0(
    out: &mut Vec<TokenV0>,
    token: TokenV0,
) -> Result<(), InvalidInputReasonV0> {
    if out.len() >= MAX_TOKENS_V0 {
        return Err(InvalidInputReasonV0::MacroValidationFailed);
    }
    out.push(token);
    Ok(())
}

pub(super) fn push_ascii_bytes_v0(
    out: &mut Vec<TokenV0>,
    bytes: &[u8],
) -> Result<(), InvalidInputReasonV0> {
    for byte in bytes {
        push_checked_v0(out, TokenV0::Char(*byte))?;
    }
    Ok(())
}

pub(super) fn parse_balanced_group_payload_v0(
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

pub(super) fn validate_macro_body_tokens_v0(
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

pub(super) fn substitute_single_param_placeholders_v0(
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
