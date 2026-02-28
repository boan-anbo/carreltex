use crate::tex::tokenize_v0::TokenV0;
pub(crate) const MAX_OK_TEXT_BYTES_V0: usize = 64 * 1024;
pub(crate) const OK_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub(crate) const OK_LINE_ADVANCE_SP_V0: i32 = 786_432;

fn skip_spaces(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn consume_group_literal(tokens: &[TokenV0], mut index: usize, literal: &[u8]) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    for expected in literal {
        if !matches!(tokens.get(index), Some(TokenV0::Char(value)) if value == expected) {
            return None;
        }
        index += 1;
    }
    if !matches!(tokens.get(index), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(index + 1)
}

fn is_supported_ok_char_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte) && byte != b'\\'
}

pub(crate) fn extract_strict_ok_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let mut index = 0usize;
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return None;
    }
    index += 1;
    index = consume_group_literal(tokens, index, b"article")?;
    index = skip_spaces(tokens, index);

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin"
    ) {
        return None;
    }
    index += 1;
    index = consume_group_literal(tokens, index, b"document")?;

    let mut body = Vec::<u8>::new();
    let mut previous_was_space = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::Space) => {
                if !previous_was_space {
                    body.push(b' ');
                    previous_was_space = true;
                }
                index += 1;
            }
            Some(TokenV0::Char(0x0c)) => {
                body.push(0x0c);
                previous_was_space = false;
                index += 1;
            }
            Some(TokenV0::Char(0x0a)) => {
                body.push(0x0a);
                previous_was_space = false;
                index += 1;
            }
            Some(TokenV0::Char(byte)) if is_supported_ok_char_v0(*byte) => {
                body.push(*byte);
                previous_was_space = false;
                index += 1;
            }
            _ => break,
        }
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        return None;
    }
    index += 1;
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);
    if index != tokens.len() {
        return None;
    }
    Some(body)
}
