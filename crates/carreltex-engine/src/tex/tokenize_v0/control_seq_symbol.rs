use super::super::caret::decode_caret_hex_v0;
use super::{ParsedControlSeqV0, TokenV0, TokenizeErrorV0};

pub(super) fn parse_control_symbol_v0(
    input: &[u8],
    byte: u8,
    next_index: usize,
) -> Result<ParsedControlSeqV0, TokenizeErrorV0> {
    if !byte.is_ascii() {
        return Err(TokenizeErrorV0::ControlSeqNonAscii);
    }
    if byte == b',' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b' '),
            next_index,
        });
    }
    if byte == b'%' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'%'),
            next_index,
        });
    }
    if byte == b'_' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'_'),
            next_index,
        });
    }
    if byte == b'#' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'#'),
            next_index,
        });
    }
    if byte == b'$' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'$'),
            next_index,
        });
    }
    if byte == b'&' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'&'),
            next_index,
        });
    }
    if byte == b'{' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'{'),
            next_index,
        });
    }
    if byte == b'}' {
        return Ok(ParsedControlSeqV0 {
            token: TokenV0::Char(b'}'),
            next_index,
        });
    }
    if byte == b'~' || byte == b'^' || byte == b'"' {
        return parse_braced_accent_passthrough_v0(input, next_index);
    }
    Ok(ParsedControlSeqV0 {
        token: TokenV0::ControlSeq(vec![byte]),
        next_index,
    })
}

fn parse_braced_accent_passthrough_v0(
    input: &[u8],
    start_index: usize,
) -> Result<ParsedControlSeqV0, TokenizeErrorV0> {
    if input.get(start_index) != Some(&b'{') {
        return Err(TokenizeErrorV0::AccentNotSupported);
    }
    let payload_index = start_index + 1;
    let (payload, close_index) = decode_for_accent_passthrough_v0(input, payload_index)?;
    if payload == 0 {
        return Err(TokenizeErrorV0::InvalidInput);
    }
    if payload == b'\\' || payload == b'{' || payload == b'}' {
        return Err(TokenizeErrorV0::AccentNotSupported);
    }
    if input.get(close_index) != Some(&b'}') {
        return Err(TokenizeErrorV0::AccentNotSupported);
    }
    Ok(ParsedControlSeqV0 {
        token: TokenV0::Char(payload),
        next_index: close_index + 1,
    })
}

fn decode_for_accent_passthrough_v0(
    input: &[u8],
    index: usize,
) -> Result<(u8, usize), TokenizeErrorV0> {
    if index >= input.len() {
        return Err(TokenizeErrorV0::AccentNotSupported);
    }
    decode_caret_hex_v0(input, index).map_err(|_| TokenizeErrorV0::AccentNotSupported)
}
