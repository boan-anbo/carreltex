fn is_supported_literal_char_v0(byte: u8) -> bool {
    if !(0x20..=0x7e).contains(&byte) {
        return false;
    }
    !matches!(
        byte,
        ITALIC_START_MARKER_V0
            | ITALIC_END_MARKER_V0
            | BOLD_START_MARKER_V0
            | BOLD_END_MARKER_V0
            | LINK_START_MARKER_V0
            | LINK_END_MARKER_V0
    )
}

fn is_safe_math_payload_char_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
        && !matches!(
            byte,
            b'$' | b'\\'
                | ITALIC_START_MARKER_V0
                | ITALIC_END_MARKER_V0
                | BOLD_START_MARKER_V0
                | BOLD_END_MARKER_V0
                | LINK_START_MARKER_V0
                | LINK_END_MARKER_V0
        )
}

fn is_ascii_alpha_bytes_v0(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes
            .iter()
            .copied()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_uppercase())
}

fn is_safe_display_math_payload_char_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'+' | b'-' | b'/' | b'*' | b'=' | b'(' | b')' | b'^' | b'_' | b'{' | b'}'
        )
}

fn display_math_placeholder_for_payload_v0(payload: &[u8]) -> &'static [u8] {
    if payload.len() <= DISPLAY_MATH_SHORT_MAX_PAYLOAD_BYTES_V0 {
        DISPLAY_MATH_PLACEHOLDER_SHORT_V0
    } else if payload.len() <= DISPLAY_MATH_MEDIUM_MAX_PAYLOAD_BYTES_V0 {
        DISPLAY_MATH_PLACEHOLDER_MEDIUM_V0
    } else {
        DISPLAY_MATH_PLACEHOLDER_LONG_V0
    }
}

fn consume_inline_math_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::Char(b'$'))) {
        return None;
    }
    let mut cursor = index + 1;
    let mut payload = Vec::<u8>::new();
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::Char(b'$')) => {
                trim_trailing_spaces(&mut payload);
                if payload.is_empty() {
                    return None;
                }
                out.extend_from_slice(INLINE_MATH_PLACEHOLDER_V0);
                return Some(cursor + 1);
            }
            Some(TokenV0::Char(byte)) if *byte == NEWLINE_MARKER_V0 => {
                push_space(&mut payload);
                cursor += 1;
            }
            Some(TokenV0::Char(byte)) if is_safe_math_payload_char_v0(*byte) => {
                payload.push(*byte);
                cursor += 1;
            }
            Some(TokenV0::Space) => {
                push_space(&mut payload);
                cursor += 1;
            }
            _ => return None,
        }
    }
}

fn consume_display_math_command_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"["
    ) {
        return None;
    }
    let mut cursor = index + 1;
    let mut payload = Vec::<u8>::new();
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"]" => {
                trim_trailing_spaces(&mut payload);
                if payload.is_empty() {
                    return None;
                }
                push_paragraph_break(out);
                out.extend_from_slice(b"^ ");
                out.extend_from_slice(display_math_placeholder_for_payload_v0(&payload));
                push_paragraph_break(out);
                return Some(cursor + 1);
            }
            Some(TokenV0::ControlSeq(name)) if is_ascii_alpha_bytes_v0(name.as_slice()) => {
                payload.push(b'\\');
                payload.extend_from_slice(name.as_slice());
                cursor += 1;
            }
            Some(TokenV0::Char(byte)) if *byte == NEWLINE_MARKER_V0 => {
                push_space(&mut payload);
                cursor += 1;
            }
            Some(TokenV0::Char(byte)) if is_safe_display_math_payload_char_v0(*byte) => {
                payload.push(*byte);
                cursor += 1;
            }
            Some(TokenV0::Space) => {
                push_space(&mut payload);
                cursor += 1;
            }
            _ => return None,
        }
    }
}
