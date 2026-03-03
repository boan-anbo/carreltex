use crate::tex::tokenize_v0::TokenV0;

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end_limit: usize) -> usize {
    while index < end_limit && matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn is_supported_ok_char_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

fn is_supported_ok_wrapper_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"textbf"
            | b"textit"
            | b"emph"
            | b"texttt"
            | b"underline"
            | b"textrm"
            | b"textsf"
            | b"textsc"
            | b"textsl"
            | b"textmd"
            | b"textup"
            | b"textsuperscript"
            | b"textsubscript"
    )
}

fn consume_balanced_group_bounds_v0(
    tokens: &[TokenV0],
    index: usize,
    max_group_depth: usize,
    end_limit: usize,
) -> Option<(usize, usize, usize)> {
    let mut cursor = skip_spaces_until(tokens, index, end_limit);
    if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let content_start = cursor + 1;
    let mut depth = 1usize;
    cursor += 1;
    while cursor < end_limit {
        match tokens.get(cursor)? {
            TokenV0::BeginGroup => {
                depth += 1;
                if depth > max_group_depth {
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

fn consume_char_space_nested_group_non_empty_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_group_depth: usize,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (inner_start, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, max_group_depth, end)?;
    let mut scan = inner_start;
    let mut has_non_space_char = false;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(byte) => {
                if *byte != b' ' {
                    has_non_space_char = true;
                }
                scan += 1;
            }
            TokenV0::Space | TokenV0::BeginGroup | TokenV0::EndGroup => {
                scan += 1;
            }
            _ => return None,
        }
    }
    if !has_non_space_char {
        return None;
    }
    Some(next_index)
}

fn render_group_fragment_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_group_depth: usize,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (inner_start, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, max_group_depth, end)?;
    consume_bibitem_label_fragment_range_v0(
        tokens,
        inner_start,
        inner_end,
        max_group_depth,
        true,
        body,
        previous_was_space,
    )?;
    Some(next_index)
}

fn consume_bibitem_label_fragment_range_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    max_group_depth: usize,
    allow_raw_groups: bool,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<()> {
    let mut index = start;
    while index < end {
        index = match tokens.get(index)? {
            TokenV0::Space => {
                if !*previous_was_space {
                    body.push(b' ');
                    *previous_was_space = true;
                }
                index + 1
            }
            TokenV0::Char(byte) if is_supported_ok_char_v0(*byte) => {
                if *byte == b'[' || *byte == b']' {
                    return None;
                }
                body.push(*byte);
                *previous_was_space = false;
                index + 1
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::ControlSeq(name) if name.as_slice() == b"protect" => index + 1,
            TokenV0::ControlSeq(name) if is_supported_ok_wrapper_command_v0(name.as_slice()) => {
                let cursor = skip_spaces_until(tokens, index + 1, end);
                let (inner_start, inner_end, next_index) =
                    consume_balanced_group_bounds_v0(tokens, cursor, max_group_depth, end)?;
                consume_bibitem_label_fragment_range_v0(
                    tokens,
                    inner_start,
                    inner_end,
                    max_group_depth,
                    true,
                    body,
                    previous_was_space,
                )?;
                next_index
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"url" => {
                let cursor = skip_spaces_until(tokens, index + 1, end);
                let (inner_start, inner_end, next_index) =
                    consume_balanced_group_bounds_v0(tokens, cursor, max_group_depth, end)?;
                consume_bibitem_label_fragment_range_v0(
                    tokens,
                    inner_start,
                    inner_end,
                    max_group_depth,
                    true,
                    body,
                    previous_was_space,
                )?;
                next_index
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"natexlab" => {
                render_group_fragment_v0(
                    tokens,
                    index + 1,
                    end,
                    max_group_depth,
                    body,
                    previous_was_space,
                )?
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"citeauthoryear" => {
                let mut cursor = index + 1;
                cursor = render_group_fragment_v0(
                    tokens,
                    cursor,
                    end,
                    max_group_depth,
                    body,
                    previous_was_space,
                )?;
                if !*previous_was_space {
                    body.push(b' ');
                    *previous_was_space = true;
                }
                let mut discarded_group = Vec::new();
                let mut discarded_previous_was_space = false;
                cursor = render_group_fragment_v0(
                    tokens,
                    cursor,
                    end,
                    max_group_depth,
                    &mut discarded_group,
                    &mut discarded_previous_was_space,
                )?;
                render_group_fragment_v0(
                    tokens,
                    cursor,
                    end,
                    max_group_depth,
                    body,
                    previous_was_space,
                )?
            }
            TokenV0::ControlSeq(name) if name.as_slice() == b"label" => {
                consume_char_space_nested_group_non_empty_v0(tokens, index + 1, end, max_group_depth)?
            }
            TokenV0::BeginGroup if allow_raw_groups => {
                let (inner_start, inner_end, next_index) =
                    consume_balanced_group_bounds_v0(tokens, index, max_group_depth, end)?;
                consume_bibitem_label_fragment_range_v0(
                    tokens,
                    inner_start,
                    inner_end,
                    max_group_depth,
                    true,
                    body,
                    previous_was_space,
                )?;
                next_index
            }
            _ => return None,
        };
    }
    Some(())
}

pub(super) fn consume_optional_bibitem_label_fragment_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_tokens: usize,
    max_group_depth: usize,
) -> Option<(usize, Option<Vec<u8>>)> {
    let mut cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some((cursor, None));
    }
    cursor += 1;
    let content_start = cursor;
    let mut scanned = 0usize;
    let mut group_depth = 0usize;
    while cursor < end {
        scanned += 1;
        if scanned > max_tokens {
            return None;
        }
        match tokens.get(cursor)? {
            TokenV0::ControlSeq(name) if name.as_slice() == b"begin" => return None,
            TokenV0::BeginGroup => {
                group_depth += 1;
                if group_depth > max_group_depth {
                    return None;
                }
            }
            TokenV0::EndGroup => {
                if group_depth == 0 {
                    return None;
                }
                group_depth -= 1;
            }
            TokenV0::Char(b'[') if group_depth == 0 => return None,
            TokenV0::Char(b']') if group_depth == 0 => {
                let mut rendered = Vec::new();
                let mut previous_was_space = false;
                consume_bibitem_label_fragment_range_v0(
                    tokens,
                    content_start,
                    cursor,
                    max_group_depth,
                    true,
                    &mut rendered,
                    &mut previous_was_space,
                )?;
                return Some((cursor + 1, Some(rendered)));
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}
