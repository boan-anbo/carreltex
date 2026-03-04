use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_balanced_group_bounds_v0, consume_bracket_options_non_empty, consume_char_space_group_non_empty,
    skip_spaces,
};

fn consume_named_controlseq_group_v0(tokens: &[TokenV0], index: usize, name: &[u8]) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    if !matches!(tokens.get(cursor), Some(TokenV0::ControlSeq(control)) if control.as_slice() == name) {
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

fn consume_skip_footins_group_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    if !matches!(tokens.get(cursor), Some(TokenV0::ControlSeq(control)) if control.as_slice() == b"skip") {
        return None;
    }
    cursor += 1;
    while matches!(tokens.get(cursor), Some(TokenV0::Space)) {
        cursor += 1;
    }
    if !matches!(tokens.get(cursor), Some(TokenV0::ControlSeq(control)) if control.as_slice() == b"footins") {
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

fn consume_balanced_group_discard_non_empty_v0(tokens: &[TokenV0], index: usize, max_tokens: usize) -> Option<usize> {
    let (inner_start, inner_end, next_index) = consume_balanced_group_bounds_v0(
        tokens,
        skip_spaces(tokens, index),
        super::super::MAX_OK_GROUP_DEPTH_V0,
        tokens.len(),
    )?;
    if inner_end <= inner_start || inner_end - inner_start > max_tokens {
        return None;
    }
    let mut has_non_space = false;
    for token in &tokens[inner_start..inner_end] {
        if matches!(token, TokenV0::ControlSeq(name) if matches!(name.as_slice(), b"begin" | b"end")) {
            return None;
        }
        if !matches!(token, TokenV0::Space) {
            has_non_space = true;
        }
    }
    has_non_space.then_some(skip_spaces(tokens, next_index))
}

pub(super) fn consume_caption_footnote_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"captionsetup" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"floatname" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"renewcommand" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_named_controlseq_group_v0(tokens, cursor, b"footnoterule")?;
            cursor = consume_balanced_group_discard_non_empty_v0(tokens, cursor, 2048)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"setlength" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_skip_footins_group_v0(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
