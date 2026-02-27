use super::TokenizeErrorV0;

pub(super) fn decode_caret_hex_v0(
    input: &[u8],
    index: usize,
) -> Result<(u8, usize), TokenizeErrorV0> {
    let byte = input[index];
    if byte != b'^' || index + 1 >= input.len() || input[index + 1] != b'^' {
        return Ok((byte, index + 1));
    }
    if index + 3 >= input.len() {
        return Err(TokenizeErrorV0::CaretNotSupported);
    }
    let high = parse_hex_nibble_v0(input[index + 2])?;
    let low = parse_hex_nibble_v0(input[index + 3])?;
    Ok((((high << 4) | low), index + 4))
}

fn parse_hex_nibble_v0(byte: u8) -> Result<u8, TokenizeErrorV0> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(TokenizeErrorV0::CaretNotSupported),
    }
}
