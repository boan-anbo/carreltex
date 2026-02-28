use super::super::caret::decode_caret_hex_v0;
use super::super::whitespace::{consume_whitespace_run_v0, is_whitespace_v0};
use super::{ParsedControlSeqV0, TokenV0, TokenizeErrorV0};

pub(super) fn parse_control_word_v0(
    input: &[u8],
    first_byte: u8,
    mut index: usize,
) -> Result<ParsedControlSeqV0, TokenizeErrorV0> {
    let mut control_word = Vec::<u8>::new();
    control_word.push(first_byte);
    while index < input.len() {
        let (word_byte, following_index) = decode_caret_hex_v0(input, index)?;
        if word_byte == 0 {
            return Err(TokenizeErrorV0::InvalidInput);
        }
        if !word_byte.is_ascii_alphabetic() {
            break;
        }
        control_word.push(word_byte);
        index = following_index;
    }
    if control_word.as_slice() == b"verb" {
        return Err(TokenizeErrorV0::InvalidInput);
    }
    if !control_word.iter().all(|byte| byte.is_ascii()) {
        return Err(TokenizeErrorV0::ControlSeqNonAscii);
    }
    if index < input.len() {
        let (space_probe, _) = decode_caret_hex_v0(input, index)?;
        if !is_whitespace_v0(space_probe) && !space_probe.is_ascii() {
            return Err(TokenizeErrorV0::ControlSeqNonAscii);
        }
        if is_whitespace_v0(space_probe) {
            index = consume_whitespace_run_v0(input, index)?;
        }
    }
    let token = if control_word.as_slice() == b"textbackslash" {
        TokenV0::Char(b'\\')
    } else if control_word.as_slice() == b"textasciitilde" {
        TokenV0::Char(b'~')
    } else if control_word.as_slice() == b"textasciicircum" {
        TokenV0::Char(b'^')
    } else if control_word.as_slice() == b"textquotedbl" {
        TokenV0::Char(b'"')
    } else if control_word.as_slice() == b"textless" {
        TokenV0::Char(b'<')
    } else if control_word.as_slice() == b"textgreater" {
        TokenV0::Char(b'>')
    } else if control_word.as_slice() == b"textbar" {
        TokenV0::Char(b'|')
    } else if control_word.as_slice() == b"textbraceleft" {
        TokenV0::Char(b'{')
    } else if control_word.as_slice() == b"textbraceright" {
        TokenV0::Char(b'}')
    } else if control_word.as_slice() == b"textunderscore" {
        TokenV0::Char(b'_')
    } else if control_word.as_slice() == b"textquotesingle" {
        TokenV0::Char(b'\'')
    } else if control_word.as_slice() == b"textasciigrave" {
        TokenV0::Char(b'`')
    } else if control_word.as_slice() == b"textquotedblleft" {
        TokenV0::Char(b'"')
    } else if control_word.as_slice() == b"textquotedblright" {
        TokenV0::Char(b'"')
    } else if control_word.as_slice() == b"par" {
        TokenV0::Space
    } else {
        TokenV0::ControlSeq(control_word)
    };
    Ok(ParsedControlSeqV0 {
        token: Some(token),
        next_index: index,
    })
}
