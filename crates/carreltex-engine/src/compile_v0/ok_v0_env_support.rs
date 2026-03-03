use crate::tex::tokenize_v0::TokenV0;

const MAX_OK_ENV_SCAN_TOKENS_V0: usize = 8192;
const MAX_OK_ENV_DEPTH_V0: usize = 64;

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end_limit: usize) -> usize {
    while index < end_limit && matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn consume_char_only_group_payload_v0(
    tokens: &[TokenV0],
    index: usize,
    end_limit: usize,
    max_bytes: usize,
) -> Option<(Vec<u8>, usize)> {
    let mut cursor = skip_spaces_until(tokens, index, end_limit);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    let mut bytes = Vec::new();
    while cursor < end_limit {
        match tokens.get(cursor)? {
            TokenV0::EndGroup => {
                if bytes.is_empty() {
                    return None;
                }
                return Some((bytes, cursor + 1));
            }
            TokenV0::Char(byte) => {
                bytes.push(*byte);
                if bytes.len() > max_bytes {
                    return None;
                }
                cursor += 1;
            }
            _ => return None,
        }
    }
    None
}

pub(super) fn is_supported_display_math_env_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"equation"
            | b"equation*"
            | b"align"
            | b"align*"
            | b"gather"
            | b"gather*"
            | b"multline"
            | b"multline*"
    )
}

pub(super) fn is_supported_ok_block_env_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"center"
            | b"flushleft"
            | b"flushright"
            | b"abstract"
            | b"quote"
            | b"quotation"
            | b"verbatim"
    )
}

pub(super) fn is_supported_ok_table_stub_env_v0(name: &[u8]) -> bool {
    matches!(name, b"tabular" | b"tabular*" | b"tabularx" | b"longtable")
}

pub(super) fn ok_thm_stub_marker_v0(name: &[u8]) -> Option<&'static [u8]> {
    match name {
        b"theorem"
        | b"lemma"
        | b"proposition"
        | b"corollary"
        | b"definition"
        | b"remark"
        | b"example" => Some(b"THM"),
        b"proof" => Some(b"PROOF"),
        _ => None,
    }
}

pub(super) fn consume_named_environment_span_v0(
    tokens: &[TokenV0],
    begin_index: usize,
    end_limit: usize,
) -> Option<(Vec<u8>, usize, usize, usize)> {
    if !matches!(tokens.get(begin_index), Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin")
    {
        return None;
    }
    let (env_name, mut cursor) =
        consume_char_only_group_payload_v0(tokens, begin_index + 1, end_limit, 64)?;
    let inner_start = cursor;
    let mut env_stack = vec![env_name.clone()];
    let mut scanned = 0usize;
    while cursor < end_limit {
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin" => {
                let (nested_name, next_index) =
                    consume_char_only_group_payload_v0(tokens, cursor + 1, end_limit, 64)?;
                scanned += next_index - cursor;
                if scanned > MAX_OK_ENV_SCAN_TOKENS_V0 || env_stack.len() >= MAX_OK_ENV_DEPTH_V0 {
                    return None;
                }
                env_stack.push(nested_name);
                cursor = next_index;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end" => {
                let (end_name, next_index) =
                    consume_char_only_group_payload_v0(tokens, cursor + 1, end_limit, 64)?;
                scanned += next_index - cursor;
                if scanned > MAX_OK_ENV_SCAN_TOKENS_V0 {
                    return None;
                }
                if !matches!(env_stack.last(), Some(top) if top == &end_name) {
                    return None;
                }
                env_stack.pop();
                if env_stack.is_empty() {
                    return Some((env_name, inner_start, cursor, next_index));
                }
                cursor = next_index;
            }
            _ => {
                scanned += 1;
                if scanned > MAX_OK_ENV_SCAN_TOKENS_V0 {
                    return None;
                }
                cursor += 1;
            }
        }
    }
    None
}
