use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_char_space_group_non_empty, consume_ok_group_fragment_discard_v0, skip_spaces,
};
use super::super::ok_v0_title_state::OkTitleStateV0;

fn consume_named_controlseq_group_v0(tokens: &[TokenV0], index: usize, names: &[&[u8]]) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    let control_name = match tokens.get(cursor) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !names.iter().any(|name| *name == control_name) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    if !matches!(tokens.get(cursor), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(cursor + 1)
}

pub(super) fn consume_fancyhdr_glue_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"leftmark" | b"rightmark" => Some(skip_spaces(tokens, index + 1)),
        b"nouppercase" | b"MakeUppercase" | b"MakeLowercase" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_ok_group_fragment_discard_v0(tokens, cursor, tokens.len(), title_state)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"renewcommand" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_named_controlseq_group_v0(tokens, cursor, &[b"headrulewidth", b"footrulewidth"])?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"setlength" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_named_controlseq_group_v0(tokens, cursor, &[b"headheight", b"footskip"])?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
