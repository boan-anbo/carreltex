use super::*;
use super::utils::push_checked_v0;

pub(super) fn parse_noexpand_v0(
    tokens: &[TokenV0],
    noexpand_index: usize,
    out: &mut Vec<TokenV0>,
) -> Result<usize, InvalidInputReasonV0> {
    let next = tokens
        .get(noexpand_index + 1)
        .ok_or(InvalidInputReasonV0::MacroNoexpandUnsupported)?
        .clone();
    push_checked_v0(out, next)?;
    Ok(noexpand_index + 2)
}
