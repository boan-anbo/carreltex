use crate::tex::tokenize_v0::TokenV0;
pub(crate) const MAX_OK_TEXT_BYTES_V0: usize = 64 * 1024;
pub(crate) const OK_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub(crate) const OK_LINE_ADVANCE_SP_V0: i32 = 786_432;
const MAX_OK_GROUP_DEPTH_V0: usize = 64;

fn skip_spaces(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end_limit: usize) -> usize {
    while index < end_limit && matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn consume_group_literal(tokens: &[TokenV0], mut index: usize, literal: &[u8]) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    for expected in literal {
        if !matches!(tokens.get(index), Some(TokenV0::Char(value)) if value == expected) {
            return None;
        }
        index += 1;
    }
    if !matches!(tokens.get(index), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(index + 1)
}

fn consume_char_space_group_non_empty(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    let mut has_non_space_char = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::EndGroup) if has_non_space_char => return Some(index + 1),
            Some(TokenV0::EndGroup) => return None,
            Some(TokenV0::Space) => {
                index += 1;
            }
            Some(TokenV0::Char(byte)) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                index += 1;
            }
            _ => return None,
        }
    }
}

fn consume_bracket_options_non_empty(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        return None;
    }
    index += 1;
    let mut has_non_space_char = false;
    loop {
        match tokens.get(index) {
            Some(TokenV0::Char(b']')) if has_non_space_char => return Some(index + 1),
            Some(TokenV0::Char(b']')) => return None,
            Some(TokenV0::Space) => {
                index += 1;
            }
            Some(TokenV0::Char(byte)) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                index += 1;
            }
            _ => return None,
        }
    }
}

fn consume_usepackage_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        index = consume_bracket_options_non_empty(tokens, index)?;
        index = skip_spaces(tokens, index);
    }
    index = consume_char_space_group_non_empty(tokens, index)?;
    Some(skip_spaces(tokens, index))
}

fn is_supported_meta_preamble_command(name: &[u8]) -> bool {
    matches!(name, b"title" | b"author" | b"date")
}

fn consume_meta_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice())
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_char_space_group_non_empty(tokens, index)?;
    Some(skip_spaces(tokens, index))
}

fn is_supported_ok_char_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

fn is_supported_ok_wrapper_command_v0(name: &[u8]) -> bool {
    matches!(name, b"textbf" | b"textit" | b"emph" | b"texttt" | b"underline")
}

fn is_supported_ok_heading_command_v0(name: &[u8]) -> bool {
    matches!(name, b"section" | b"subsection" | b"subsubsection")
}

fn consume_balanced_group_bounds_v0(
    tokens: &[TokenV0],
    index: usize,
    depth_cap: usize,
    end_limit: usize,
) -> Option<(usize, usize, usize)> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut depth = 1usize;
    let mut cursor = index + 1;
    let content_start = cursor;
    while cursor < end_limit {
        let token = tokens.get(cursor)?;
        match token {
            TokenV0::BeginGroup => {
                depth += 1;
                if depth > depth_cap {
                    return None;
                }
            }
            TokenV0::EndGroup => {
                depth -= 1;
                if depth == 0 {
                    return Some((content_start, cursor, cursor + 1));
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn consume_ok_body_range_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    allow_nested_groups: bool,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<()> {
    let mut index = start;
    while index < end {
        index = consume_ok_body_token_v0(
            tokens,
            index,
            end,
            allow_nested_groups,
            body,
            previous_was_space,
        )?;
    }
    Some(())
}

fn consume_ok_body_token_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    allow_nested_groups: bool,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    if index >= end {
        return None;
    }
    match tokens.get(index) {
        Some(TokenV0::Space) => {
            if !*previous_was_space {
                body.push(b' ');
                *previous_was_space = true;
            }
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"par" => {
            if !*previous_was_space {
                body.push(b' ');
                *previous_was_space = true;
            }
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"maketitle" => Some(index + 1),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"noindent" => Some(index + 1),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"newline" => {
            body.push(0x0a);
            *previous_was_space = true;
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_wrapper_command_v0(name.as_slice()) =>
        {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(tokens, inner_start, inner_end, true, body, previous_was_space)?;
            cursor = next_index;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_heading_command_v0(name.as_slice()) =>
        {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
            body.push(0x0a);
            *previous_was_space = true;
            consume_ok_body_range_v0(tokens, inner_start, inner_end, true, body, previous_was_space)?;
            body.push(0x0a);
            *previous_was_space = true;
            cursor = next_index;
            Some(cursor)
        }
        Some(TokenV0::BeginGroup) if allow_nested_groups => {
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, index, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(tokens, inner_start, inner_end, true, body, previous_was_space)?;
            Some(next_index)
        }
        Some(TokenV0::Char(0x0c)) => {
            body.push(0x0c);
            *previous_was_space = false;
            Some(index + 1)
        }
        Some(TokenV0::Char(0x0a)) => {
            body.push(0x0a);
            *previous_was_space = true;
            Some(index + 1)
        }
        Some(TokenV0::Char(byte)) if is_supported_ok_char_v0(*byte) => {
            body.push(*byte);
            *previous_was_space = false;
            Some(index + 1)
        }
        _ => None,
    }
}

pub(crate) fn extract_strict_ok_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let mut index = 0usize;
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        index = consume_bracket_options_non_empty(tokens, index)?;
    }
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"article")?;
    index = skip_spaces(tokens, index);
    loop {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage" => {
                index = consume_usepackage_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_meta_preamble_command(name.as_slice()) =>
            {
                index = consume_meta_preamble_command(tokens, index)?;
                continue;
            }
            _ => {}
        }
        break;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);

    let mut body = Vec::<u8>::new();
    let mut previous_was_space = false;
    while !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        index = consume_ok_body_token_v0(
            tokens,
            index,
            tokens.len(),
            false,
            &mut body,
            &mut previous_was_space,
        )?;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);
    if index != tokens.len() {
        return None;
    }
    Some(body)
}
