use crate::tex::tokenize_v0::TokenV0;

pub(super) fn consume_display_math_dollar_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::Char(b'$')))
        || !matches!(tokens.get(index + 1), Some(TokenV0::Char(b'$')))
    {
        return None;
    }
    let mut cursor = index + 2;
    let mut scanned = 0usize;
    while cursor + 1 < end {
        scanned += 1;
        if scanned > max_tokens {
            return None;
        }
        match tokens.get(cursor)? {
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::Char(b'$') if matches!(tokens.get(cursor + 1), Some(TokenV0::Char(b'$'))) => {
                return Some(cursor + 2)
            }
            _ => {
                cursor += 1;
            }
        }
    }
    None
}

pub(super) fn consume_inline_math_dollar_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::Char(b'$')))
        || matches!(tokens.get(index + 1), Some(TokenV0::Char(b'$')))
    {
        return None;
    }
    let mut cursor = index + 1;
    let mut scanned = 0usize;
    while cursor < end {
        scanned += 1;
        if scanned > max_tokens {
            return None;
        }
        match tokens.get(cursor)? {
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::Char(b'$') => return Some(cursor + 1),
            _ => {
                cursor += 1;
            }
        }
    }
    None
}
