use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_ok_group_fragment_discard_v0, consume_ok_group_fragment_v0, skip_spaces,
};
use super::super::ok_v0_title_state::OkTitleStateV0;

pub(super) fn is_supported_meta_preamble_command(name: &[u8]) -> bool {
    matches!(
        name,
        b"title"
            | b"author"
            | b"date"
            | b"thanks"
            | b"subtitle"
            | b"institute"
            | b"affiliation"
            | b"address"
            | b"email"
            | b"homepage"
            | b"keywords"
            | b"subject"
            | b"titlehead"
            | b"authorrunning"
            | b"titlerunning"
            | b"publishers"
            | b"dedication"
            | b"extratitle"
            | b"extrainfo"
            | b"uppertitleback"
            | b"lowertitleback"
    )
}

pub(super) fn consume_meta_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice()) => {
            name.as_slice()
        }
        _ => return None,
    };
    let mut cursor = skip_spaces(tokens, index + 1);
    if matches!(name, b"title" | b"author" | b"date" | b"subtitle")
        && matches!(tokens.get(cursor), Some(TokenV0::Char(b'*')))
    {
        cursor += 1;
        cursor = skip_spaces(tokens, cursor);
    }
    if matches!(name, b"title" | b"author" | b"date") {
        let mut fragment = Vec::new();
        let mut fragment_previous_was_space = false;
        let next_index = consume_ok_group_fragment_v0(
            tokens,
            cursor,
            tokens.len(),
            title_state,
            &mut fragment,
            &mut fragment_previous_was_space,
        )?;
        title_state.set_field(name, fragment);
        return Some(skip_spaces(tokens, next_index));
    }
    let next_index = consume_ok_group_fragment_discard_v0(tokens, cursor, tokens.len(), title_state)?;
    Some(skip_spaces(tokens, next_index))
}

fn consume_single_named_controlseq_group_v0(
    tokens: &[TokenV0],
    index: usize,
    allowed_names: &[&[u8]],
) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    let name = match tokens.get(cursor) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if !allowed_names.iter().any(|allowed| *allowed == name) {
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

pub(super) fn consume_renewcommand_and_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"renewcommand"
    ) {
        return None;
    }
    let mut cursor = skip_spaces(tokens, index + 1);
    cursor = consume_single_named_controlseq_group_v0(tokens, cursor, &[b"and"])?;
    cursor = skip_spaces(tokens, cursor);
    cursor = consume_ok_group_fragment_discard_v0(tokens, cursor, tokens.len(), title_state)?;
    Some(skip_spaces(tokens, cursor))
}
