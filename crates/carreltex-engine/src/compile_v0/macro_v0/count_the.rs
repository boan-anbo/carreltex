use super::*;
use super::utils::push_ascii_bytes_v0;

const MAX_COUNT_VALUE_V0: u32 = 1_000_000;

pub(super) fn parse_count_assignment_v0(
    tokens: &[TokenV0],
    count_index: usize,
    counters: &mut [u32; 2],
) -> Result<usize, InvalidInputReasonV0> {
    let register_index = match tokens.get(count_index + 1) {
        Some(TokenV0::Char(b'0')) => 0usize,
        Some(TokenV0::Char(b'1')) => 1usize,
        _ => return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported),
    };
    if !matches!(tokens.get(count_index + 2), Some(TokenV0::Char(b'='))) {
        return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
    }

    let mut index = count_index + 3;
    let mut value: u32 = 0;
    let mut saw_digit = false;
    while let Some(TokenV0::Char(byte)) = tokens.get(index) {
        if !byte.is_ascii_digit() {
            break;
        }
        saw_digit = true;
        value = value
            .checked_mul(10)
            .and_then(|current| current.checked_add((byte - b'0') as u32))
            .ok_or(InvalidInputReasonV0::MacroCountAssignmentUnsupported)?;
        if value > MAX_COUNT_VALUE_V0 {
            return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
        }
        index += 1;
    }
    if !saw_digit {
        return Err(InvalidInputReasonV0::MacroCountAssignmentUnsupported);
    }

    counters[register_index] = value;
    Ok(index)
}

pub(super) fn parse_the_v0(
    tokens: &[TokenV0],
    the_index: usize,
    counters: &[u32; 2],
) -> Result<(Vec<TokenV0>, usize), InvalidInputReasonV0> {
    if !matches!(tokens.get(the_index + 1), Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"count")
    {
        return Err(InvalidInputReasonV0::MacroTheUnsupported);
    }

    let register_index = match tokens.get(the_index + 2) {
        Some(TokenV0::Char(b'0')) => 0usize,
        Some(TokenV0::Char(b'1')) => 1usize,
        _ => return Err(InvalidInputReasonV0::MacroTheUnsupported),
    };

    let mut out = Vec::<TokenV0>::new();
    let digits = counters[register_index].to_string();
    push_ascii_bytes_v0(&mut out, digits.as_bytes())?;
    Ok((out, the_index + 3))
}
