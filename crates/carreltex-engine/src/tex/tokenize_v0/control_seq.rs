use super::caret::decode_caret_hex_v0;
use super::{TokenV0, TokenizeErrorV0};
#[path = "control_seq_symbol.rs"]
mod control_seq_symbol;
#[path = "control_seq_word.rs"]
mod control_seq_word;
use control_seq_symbol::parse_control_symbol_v0;
use control_seq_word::parse_control_word_v0;

pub(super) struct ParsedControlSeqV0 {
    pub(super) tokens: Vec<TokenV0>,
    pub(super) next_index: usize,
}

pub(super) fn parse_control_seq_v0(
    input: &[u8],
    start_index: usize,
) -> Result<ParsedControlSeqV0, TokenizeErrorV0> {
    if start_index >= input.len() {
        return Err(TokenizeErrorV0::InvalidInput);
    }

    let (next, after_next_index) = decode_caret_hex_v0(input, start_index)?;
    if next == 0 {
        return Err(TokenizeErrorV0::InvalidInput);
    }

    if next.is_ascii_alphabetic() {
        parse_control_word_v0(input, next, after_next_index)
    } else {
        parse_control_symbol_v0(input, next, after_next_index)
    }
}
