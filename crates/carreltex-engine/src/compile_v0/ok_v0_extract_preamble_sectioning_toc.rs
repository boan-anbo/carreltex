use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_balanced_group_bounds_v0, consume_ok_group_fragment_discard_v0, skip_spaces,
};
use super::super::ok_v0_title_state::OkTitleStateV0;

fn consume_char_space_group_trimmed_v0(tokens: &[TokenV0], index: usize) -> Option<(Vec<u8>, usize)> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    cursor += 1;
    let mut bytes = Vec::new();
    let mut has_non_space = false;
    loop {
        match tokens.get(cursor)? {
            TokenV0::EndGroup if has_non_space => break,
            TokenV0::EndGroup => return None,
            TokenV0::Space => bytes.push(b' '),
            TokenV0::Char(byte) => {
                has_non_space = true;
                bytes.push(*byte);
            }
            _ => return None,
        }
        cursor += 1;
    }
    while matches!(bytes.first(), Some(b' ')) {
        bytes.remove(0);
    }
    while matches!(bytes.last(), Some(b' ')) {
        bytes.pop();
    }
    if bytes.is_empty() {
        return None;
    }
    Some((bytes, cursor + 1))
}

fn is_signed_int_up_to_6_v0(value: &[u8]) -> bool {
    let digits = if let Some(stripped) = value.strip_prefix(b"-") { stripped } else { value };
    (1..=6).contains(&digits.len()) && digits.iter().all(|byte| byte.is_ascii_digit())
}

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

pub(super) fn consume_sectioning_toc_preamble_command(
    tokens: &[TokenV0],
    index: usize,
    title_state: &mut OkTitleStateV0,
) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"setcounter" => {
            let (counter_name, mut cursor) = consume_char_space_group_trimmed_v0(tokens, index + 1)?;
            cursor = skip_spaces(tokens, cursor);
            let (counter_value, next_index) = consume_char_space_group_trimmed_v0(tokens, cursor)?;
            if matches!(counter_name.as_slice(), b"secnumdepth" | b"tocdepth")
                && !is_signed_int_up_to_6_v0(&counter_value)
            {
                return None;
            }
            Some(skip_spaces(tokens, next_index))
        }
        b"renewcommand" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_named_controlseq_group_v0(
                tokens,
                cursor,
                &[b"contentsname", b"listfigurename", b"listtablename"],
            )?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_ok_group_fragment_discard_v0(tokens, cursor, tokens.len(), title_state)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"addto" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            match tokens.get(cursor) {
                Some(TokenV0::ControlSeq(target))
                    if matches!(target.as_slice(), b"captionsenglish" | b"captionsngerman") =>
                {
                    cursor += 1;
                }
                _ => return None,
            }
            cursor = consume_balanced_group_discard_non_empty_v0(tokens, cursor, 2048)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
