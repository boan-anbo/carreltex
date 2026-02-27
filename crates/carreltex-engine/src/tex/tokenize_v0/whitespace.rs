use super::caret::decode_caret_hex_v0;
use super::TokenizeErrorV0;

pub(super) fn is_whitespace_v0(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

pub(super) fn consume_whitespace_run_v0(
    input: &[u8],
    mut index: usize,
) -> Result<usize, TokenizeErrorV0> {
    while index < input.len() {
        let (next_byte, following_index) = decode_caret_hex_v0(input, index)?;
        if !is_whitespace_v0(next_byte) {
            break;
        }
        index = following_index;
    }
    Ok(index)
}
