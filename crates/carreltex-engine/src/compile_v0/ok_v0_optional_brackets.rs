use crate::tex::tokenize_v0::TokenV0;

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end_limit: usize) -> usize {
    while index < end_limit && matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

pub(super) fn consume_optional_simple_bracket_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_bytes: usize,
) -> Option<usize> {
    let mut cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut content_len = 0usize;
    while cursor < end {
        match tokens.get(cursor)? {
            TokenV0::Char(b']') => return Some(cursor + 1),
            TokenV0::Char(_) | TokenV0::Space => {
                content_len += 1;
                if content_len > max_bytes {
                    return None;
                }
                cursor += 1;
            }
            _ => return None,
        }
    }
    None
}

pub(super) fn consume_optional_digits_bracket_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_bytes: usize,
) -> Option<usize> {
    let mut cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut content_len = 0usize;
    let mut has_digit = false;
    while cursor < end {
        match tokens.get(cursor)? {
            TokenV0::Char(b']') if has_digit => return Some(cursor + 1),
            TokenV0::Char(b']') => return None,
            TokenV0::Space => {
                content_len += 1;
                if content_len > max_bytes {
                    return None;
                }
                cursor += 1;
            }
            TokenV0::Char(byte) if byte.is_ascii_digit() => {
                content_len += 1;
                if content_len > max_bytes {
                    return None;
                }
                has_digit = true;
                cursor += 1;
            }
            _ => return None,
        }
    }
    None
}

pub(super) fn consume_optional_heading_short_title_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
    max_group_depth: usize,
) -> Option<usize> {
    let mut cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut scanned = 0usize;
    let mut group_depth = 0usize;
    while cursor < end {
        scanned += 1;
        if scanned > max_tokens {
            return None;
        }
        match tokens.get(cursor)? {
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::BeginGroup => {
                group_depth += 1;
                if group_depth > max_group_depth {
                    return None;
                }
            }
            TokenV0::EndGroup => {
                if group_depth == 0 {
                    return None;
                }
                group_depth -= 1;
            }
            TokenV0::Char(b']') if group_depth == 0 => return Some(cursor + 1),
            _ => {}
        }
        cursor += 1;
    }
    None
}
