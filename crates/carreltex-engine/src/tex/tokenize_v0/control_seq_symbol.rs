use super::{ParsedControlSeqV0, TokenV0, TokenizeErrorV0};

pub(super) fn parse_control_symbol_v0(
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
    Ok(ParsedControlSeqV0 {
        token: TokenV0::ControlSeq(vec![byte]),
        next_index,
    })
}
