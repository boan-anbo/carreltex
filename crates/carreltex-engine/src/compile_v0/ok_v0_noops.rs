use crate::tex::tokenize_v0::TokenV0;

use super::consume_char_space_nested_group_v0;
use super::ok_v0_optional_brackets::consume_optional_digits_bracket_span_v0;

const MAX_OK_STATE_NAME_BYTES_V0: usize = 128;
const MAX_OK_STATE_DIGIT_BYTES_V0: usize = 128;
const MAX_OK_LENGTH_EXPR_TOKENS_V0: usize = 512;

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end: usize) -> usize {
    while index < end {
        if matches!(tokens.get(index), Some(TokenV0::Space)) {
            index += 1;
            continue;
        }
        break;
    }
    index
}

fn consume_balanced_group_bounds_with_caps_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
    max_group_depth: usize,
) -> Option<(usize, usize, usize)> {
    let cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut depth = 1usize;
    let mut scan = cursor + 1;
    let content_start = scan;
    let mut scanned_tokens = 0usize;
    while scan < end {
        scanned_tokens += 1;
        if scanned_tokens > max_tokens {
            return None;
        }
        match tokens.get(scan)? {
            TokenV0::BeginGroup => {
                depth += 1;
                if depth > max_group_depth {
                    return None;
                }
            }
            TokenV0::EndGroup => {
                depth -= 1;
                if depth == 0 {
                    return Some((content_start, scan, scan + 1));
                }
            }
            _ => {}
        }
        scan += 1;
    }
    None
}

fn consume_state_name_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) = consume_balanced_group_bounds_with_caps_v0(
        tokens,
        cursor,
        end,
        MAX_OK_STATE_NAME_BYTES_V0,
        super::MAX_OK_GROUP_DEPTH_V0,
    )?;
    let mut scan = cursor + 1;
    let mut has_non_space_char = false;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(byte) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
            }
            TokenV0::Space => {}
            _ => return None,
        }
        scan += 1;
    }
    if !has_non_space_char {
        return None;
    }
    Some(next_index)
}

fn consume_state_digits_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) = consume_balanced_group_bounds_with_caps_v0(
        tokens,
        cursor,
        end,
        MAX_OK_STATE_DIGIT_BYTES_V0,
        super::MAX_OK_GROUP_DEPTH_V0,
    )?;
    let mut scan = cursor + 1;
    let mut has_digit = false;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(byte) if byte.is_ascii_digit() => {
                has_digit = true;
            }
            TokenV0::Space => {}
            _ => return None,
        }
        scan += 1;
    }
    if !has_digit {
        return None;
    }
    Some(next_index)
}

fn consume_length_register_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut scan = cursor + 1;
    while matches!(tokens.get(scan), Some(TokenV0::Space)) {
        scan += 1;
    }
    if !matches!(tokens.get(scan), Some(TokenV0::ControlSeq(_))) {
        return None;
    }
    scan += 1;
    while matches!(tokens.get(scan), Some(TokenV0::Space)) {
        scan += 1;
    }
    if !matches!(tokens.get(scan), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(scan + 1)
}

fn consume_length_expr_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let (_, _, next_index) = consume_balanced_group_bounds_with_caps_v0(
        tokens,
        index,
        end,
        MAX_OK_LENGTH_EXPR_TOKENS_V0,
        super::MAX_OK_GROUP_DEPTH_V0,
    )?;
    Some(next_index)
}

pub(super) fn is_ok_noop_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"bibliographystyle"
            | b"bibliography"
            | b"nocite"
            | b"phantomsection"
            | b"addcontentsline"
            | b"addtocontents"
            | b"markboth"
            | b"markright"
            | b"thispagestyle"
            | b"pagestyle"
            | b"smallskip"
            | b"medskip"
            | b"bigskip"
            | b"hfill"
            | b"vfill"
            | b"newpage"
            | b"clearpage"
            | b"pagebreak"
            | b"nopagebreak"
            | b"linebreak"
            | b"nolinebreak"
            | b"goodbreak"
            | b"filbreak"
            | b"samepage"
            | b"nobreak"
            | b"break"
            | b"vspace"
            | b"hspace"
            | b"setcounter"
            | b"addtocounter"
            | b"stepcounter"
            | b"refstepcounter"
            | b"setlength"
            | b"addtolength"
    )
}

pub(super) fn consume_ok_noop_command_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    name: &[u8],
) -> Option<usize> {
    match name {
        b"phantomsection"
        | b"smallskip"
        | b"medskip"
        | b"bigskip"
        | b"hfill"
        | b"vfill"
        | b"newpage"
        | b"clearpage"
        | b"goodbreak"
        | b"filbreak"
        | b"samepage"
        | b"nobreak"
        | b"break" => Some(index + 1),
        b"pagebreak" | b"nopagebreak" | b"linebreak" | b"nolinebreak" => {
            consume_optional_digits_bracket_span_v0(tokens, index + 1, end, 8)
        }
        b"vspace" | b"hspace" => {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
                cursor = skip_spaces_until(tokens, cursor, end);
            }
            consume_char_space_nested_group_v0(tokens, cursor, end)
        }
        b"bibliographystyle"
        | b"bibliography"
        | b"nocite"
        | b"markright"
        | b"thispagestyle"
        | b"pagestyle" => consume_char_space_nested_group_v0(tokens, index + 1, end),
        b"addtocontents" | b"markboth" => {
            let next = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
            consume_char_space_nested_group_v0(tokens, next, end)
        }
        b"addcontentsline" => {
            let next = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
            let next = consume_char_space_nested_group_v0(tokens, next, end)?;
            consume_char_space_nested_group_v0(tokens, next, end)
        }
        b"setcounter" | b"addtocounter" => {
            let next = consume_state_name_group_v0(tokens, index + 1, end)?;
            consume_state_digits_group_v0(tokens, next, end)
        }
        b"stepcounter" | b"refstepcounter" => {
            consume_state_name_group_v0(tokens, index + 1, end)
        }
        b"setlength" | b"addtolength" => {
            let next = consume_length_register_group_v0(tokens, index + 1, end)?;
            consume_length_expr_group_v0(tokens, next, end)
        }
        _ => None,
    }
}
