use crate::tex::tokenize_v0::TokenV0;

pub(super) fn consume_math_control_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    close_name: &[u8],
    max_tokens: usize,
) -> Option<usize> {
    let mut cursor = index + 1;
    let mut scanned = 0usize;
    while cursor < end {
        scanned += 1;
        if scanned > max_tokens {
            return None;
        }
        if matches!(tokens.get(cursor), Some(TokenV0::ControlSeq(name)) if name.as_slice() == close_name) {
            return Some(cursor + 1);
        }
        cursor += 1;
    }
    None
}

pub(super) fn consume_ensuremath_group_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
    max_group_depth: usize,
) -> Option<usize> {
    if !matches!(tokens.get(index + 1), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut cursor = index + 2;
    let mut scanned = 0usize;
    let mut group_depth = 1usize;
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
                group_depth -= 1;
                if group_depth == 0 {
                    return Some(cursor + 1);
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}
