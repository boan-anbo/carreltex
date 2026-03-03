use crate::tex::tokenize_v0::TokenV0;
pub(crate) const MAX_OK_TEXT_BYTES_V0: usize = 64 * 1024;
pub(crate) const OK_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub(crate) const OK_LINE_ADVANCE_SP_V0: i32 = 786_432;
const MAX_OK_GROUP_DEPTH_V0: usize = 64;
const MAX_OK_BRACKET_BYTES_V0: usize = 256;
const MAX_OK_MATH_SCAN_TOKENS_V0: usize = 4096;

enum ListEnvV0 {
    Itemize,
    Enumerate { next: u32 },
    Thebibliography,
    Figure,
    Table,
}

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

fn is_supported_bibliography_preamble_command(name: &[u8]) -> bool {
    matches!(name, b"bibliographystyle" | b"bibliography")
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

fn consume_char_space_nested_group_non_empty_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut scan = cursor + 1;
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

fn consume_bibliography_preamble_command(tokens: &[TokenV0], mut index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if is_supported_bibliography_preamble_command(name.as_slice())
    ) {
        return None;
    }
    index += 1;
    index = consume_char_space_nested_group_non_empty_v0(tokens, index, tokens.len())?;
    Some(skip_spaces(tokens, index))
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

fn is_supported_ok_heading_command_v0(name: &[u8]) -> bool {
    matches!(name, b"section" | b"subsection" | b"subsubsection")
}

fn is_supported_ok_style_declaration_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"bfseries"
            | b"mdseries"
            | b"itshape"
            | b"slshape"
            | b"scshape"
            | b"upshape"
            | b"rmfamily"
            | b"sffamily"
            | b"ttfamily"
            | b"em"
            | b"centering"
    )
}

fn ok_marker_command_v0(name: &[u8]) -> Option<&'static [u8]> {
    match name {
        b"cite" | b"citet" | b"citep" => Some(b"CITE"),
        b"ref" | b"autoref" | b"cref" | b"Cref" => Some(b"REF"),
        b"pageref" => Some(b"PAGEREF"),
        b"eqref" => Some(b"EQREF"),
        _ => None,
    }
}

fn consume_ok_group_fragment_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    body: &mut Vec<u8>,
    previous_was_space: &mut bool,
) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (inner_start, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut nested_list_env = None;
    consume_ok_body_range_v0(
        tokens,
        inner_start,
        inner_end,
        true,
        &mut nested_list_env,
        body,
        previous_was_space,
    )?;
    Some(next_index)
}

fn consume_ok_group_fragment_discard_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let mut scratch_body = Vec::new();
    let mut scratch_previous_was_space = false;
    consume_ok_group_fragment_v0(
        tokens,
        index,
        end,
        &mut scratch_body,
        &mut scratch_previous_was_space,
    )
}

fn consume_char_space_nested_group_v0(tokens: &[TokenV0], index: usize, end: usize) -> Option<usize> {
    let cursor = skip_spaces_until(tokens, index, end);
    let (_, inner_end, next_index) =
        consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
    let mut scan = cursor + 1;
    while scan < inner_end {
        match tokens.get(scan)? {
            TokenV0::Char(_) | TokenV0::Space | TokenV0::BeginGroup | TokenV0::EndGroup => {
                scan += 1;
            }
            _ => return None,
        }
    }
    Some(next_index)
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

fn consume_optional_simple_bracket_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    max_bytes: usize,
) -> Option<usize> {
    let mut cursor = skip_spaces_until(tokens, index, end);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut content_len = 0usize;
    while cursor < end {
        match tokens.get(cursor)? {
            TokenV0::Char(b']') => return Some(cursor + 1),
            TokenV0::Char(_) | TokenV0::Space => {
                content_len += 1;
                if content_len > max_bytes {
                    return None;
                }
                cursor += 1;
            }
            _ => return None,
        }
    }
    None
}

fn consume_math_control_span_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    close_name: &[u8],
) -> Option<usize> {
    let mut cursor = index + 1;
    let mut scanned = 0usize;
    while cursor < end {
        scanned += 1;
        if scanned > MAX_OK_MATH_SCAN_TOKENS_V0 {
            return None;
        }
        if matches!(tokens.get(cursor), Some(TokenV0::ControlSeq(name)) if name.as_slice() == close_name) {
            return Some(cursor + 1);
        }
        cursor += 1;
    }
    None
}

fn emit_ok_inline_math_marker_v0(body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(b' ');
    body.push(b'[');
    body.push(b'M');
    body.push(b'A');
    body.push(b'T');
    body.push(b'H');
    body.push(b']');
    *previous_was_space = false;
}

fn emit_ok_display_math_marker_v0(body: &mut Vec<u8>, previous_was_space: &mut bool) {
    body.push(0x0a);
    body.push(b'[');
    body.push(b'M');
    body.push(b'A');
    body.push(b'T');
    body.push(b'H');
    body.push(b']');
    body.push(0x0a);
    *previous_was_space = true;
}

fn consume_ok_body_range_v0(
    tokens: &[TokenV0],
    start: usize,
    end: usize,
    allow_nested_groups: bool,
    list_env: &mut Option<ListEnvV0>,
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
            list_env,
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
    list_env: &mut Option<ListEnvV0>,
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
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_style_declaration_v0(name.as_slice()) =>
        {
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"newline" => {
            body.push(0x0a);
            *previous_was_space = true;
            Some(index + 1)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"(" => {
            let next_index = consume_math_control_span_v0(tokens, index, end, b")")?;
            emit_ok_inline_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"[" => {
            let next_index = consume_math_control_span_v0(tokens, index, end, b"]")?;
            emit_ok_display_math_marker_v0(body, previous_was_space);
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"footnotemark" => Some(index + 1),
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == b"footnote" || name.as_slice() == b"footnotetext" =>
        {
            let mut footnote_text = Vec::new();
            let mut footnote_previous_was_space = false;
            let next_index = consume_ok_group_fragment_v0(
                tokens,
                index + 1,
                end,
                &mut footnote_text,
                &mut footnote_previous_was_space,
            )?;
            body.push(b' ');
            body.push(b'[');
            body.extend_from_slice(&footnote_text);
            body.push(b']');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"label" => {
            consume_char_space_nested_group_v0(tokens, index + 1, end)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"includegraphics" => {
            let mut cursor =
                consume_optional_simple_bracket_span_v0(tokens, index + 1, end, MAX_OK_BRACKET_BYTES_V0)?;
            cursor = consume_char_space_nested_group_non_empty_v0(tokens, cursor, end)?;
            body.push(b' ');
            body.push(b'[');
            body.push(b'I');
            body.push(b'M');
            body.push(b'G');
            body.push(b']');
            *previous_was_space = false;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"caption" => {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                return None;
            }
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
                cursor = skip_spaces_until(tokens, cursor, end);
            }
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                return None;
            }
            let mut caption_text = Vec::new();
            let mut caption_previous_was_space = false;
            cursor = consume_ok_group_fragment_v0(
                tokens,
                cursor,
                end,
                &mut caption_text,
                &mut caption_previous_was_space,
            )?;
            body.push(0x0a);
            body.extend_from_slice(&caption_text);
            body.push(0x0a);
            *previous_was_space = true;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if ok_marker_command_v0(name.as_slice()).is_some() =>
        {
            let marker = ok_marker_command_v0(name.as_slice())?;
            let next_index = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
            body.push(b' ');
            body.push(b'[');
            body.extend_from_slice(marker);
            body.push(b']');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"href" => {
            let mut scratch_body = Vec::new();
            let mut scratch_previous_was_space = *previous_was_space;
            let first_arg_end = consume_ok_group_fragment_v0(
                tokens,
                index + 1,
                end,
                &mut scratch_body,
                &mut scratch_previous_was_space,
            )?;
            consume_ok_group_fragment_v0(tokens, first_arg_end, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"url" => {
            consume_ok_group_fragment_v0(tokens, index + 1, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"textcolor" => {
            let first_arg_end = consume_ok_group_fragment_discard_v0(tokens, index + 1, end)?;
            consume_ok_group_fragment_v0(tokens, first_arg_end, end, body, previous_was_space)
        }
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"color" => {
            consume_ok_group_fragment_discard_v0(tokens, index + 1, end)
        }
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == b"enquote" || name.as_slice() == b"quote" =>
        {
            let mut quote_text = Vec::new();
            let mut quote_previous_was_space = false;
            let next_index = consume_ok_group_fragment_v0(
                tokens,
                index + 1,
                end,
                &mut quote_text,
                &mut quote_previous_was_space,
            )?;
            body.push(b'"');
            body.extend_from_slice(&quote_text);
            body.push(b'"');
            *previous_was_space = false;
            Some(next_index)
        }
        Some(TokenV0::ControlSeq(name))
            if is_supported_ok_wrapper_command_v0(name.as_slice()) =>
        {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, cursor, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
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
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
            body.push(0x0a);
            *previous_was_space = true;
            cursor = next_index;
            Some(cursor)
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"begin" =>
        {
            let cursor = skip_spaces_until(tokens, index + 1, end);
            if list_env.is_some() {
                return None;
            }
            if let Some(next_index) = consume_group_literal(tokens, cursor, b"itemize") {
                *list_env = Some(ListEnvV0::Itemize);
                return Some(next_index);
            }
            if let Some(next_index) = consume_group_literal(tokens, cursor, b"enumerate") {
                *list_env = Some(ListEnvV0::Enumerate { next: 1 });
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"figure") {
                let next_index = consume_optional_simple_bracket_span_v0(
                    tokens,
                    begin_env_end,
                    end,
                    MAX_OK_BRACKET_BYTES_V0,
                )?;
                *list_env = Some(ListEnvV0::Figure);
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"table") {
                let next_index = consume_optional_simple_bracket_span_v0(
                    tokens,
                    begin_env_end,
                    end,
                    MAX_OK_BRACKET_BYTES_V0,
                )?;
                *list_env = Some(ListEnvV0::Table);
                return Some(next_index);
            }
            if let Some(begin_env_end) = consume_group_literal(tokens, cursor, b"thebibliography") {
                let next_index =
                    consume_char_space_nested_group_non_empty_v0(tokens, begin_env_end, end)?;
                *list_env = Some(ListEnvV0::Thebibliography);
                return Some(next_index);
            }
            None
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"end" =>
        {
            let cursor = skip_spaces_until(tokens, index + 1, end);
            match list_env {
                Some(ListEnvV0::Itemize) => {
                    let next_index = consume_group_literal(tokens, cursor, b"itemize")?;
                    body.push(0x0a);
                    *previous_was_space = true;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListEnvV0::Enumerate { .. }) => {
                    let next_index = consume_group_literal(tokens, cursor, b"enumerate")?;
                    body.push(0x0a);
                    *previous_was_space = true;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListEnvV0::Thebibliography) => {
                    let next_index = consume_group_literal(tokens, cursor, b"thebibliography")?;
                    body.push(0x0a);
                    *previous_was_space = true;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListEnvV0::Figure) => {
                    let next_index = consume_group_literal(tokens, cursor, b"figure")?;
                    *list_env = None;
                    Some(next_index)
                }
                Some(ListEnvV0::Table) => {
                    let next_index = consume_group_literal(tokens, cursor, b"table")?;
                    *list_env = None;
                    Some(next_index)
                }
                None => None,
            }
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"item" =>
        {
            match list_env {
                Some(ListEnvV0::Itemize) => {
                    body.push(0x0a);
                    body.push(b'-');
                    body.push(b' ');
                    *previous_was_space = true;
                    Some(index + 1)
                }
                Some(ListEnvV0::Enumerate { next }) => {
                    body.push(0x0a);
                    for byte in next.to_string().as_bytes() {
                        body.push(*byte);
                    }
                    body.push(b'.');
                    body.push(b' ');
                    *next += 1;
                    *previous_was_space = true;
                    Some(index + 1)
                }
                Some(ListEnvV0::Thebibliography) => None,
                Some(ListEnvV0::Figure) => None,
                Some(ListEnvV0::Table) => None,
                None => None,
            }
        }
        Some(TokenV0::ControlSeq(name))
            if !allow_nested_groups && name.as_slice() == b"bibitem" =>
        {
            match list_env {
                Some(ListEnvV0::Thebibliography) => {
                    let next_index =
                        consume_char_space_nested_group_non_empty_v0(tokens, index + 1, end)?;
                    body.push(0x0a);
                    body.push(b'-');
                    body.push(b' ');
                    *previous_was_space = true;
                    Some(next_index)
                }
                _ => None,
            }
        }
        Some(TokenV0::BeginGroup) if allow_nested_groups => {
            let (inner_start, inner_end, next_index) =
                consume_balanced_group_bounds_v0(tokens, index, MAX_OK_GROUP_DEPTH_V0, end)?;
            consume_ok_body_range_v0(
                tokens,
                inner_start,
                inner_end,
                true,
                list_env,
                body,
                previous_was_space,
            )?;
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
            Some(TokenV0::ControlSeq(name))
                if is_supported_bibliography_preamble_command(name.as_slice()) =>
            {
                index = consume_bibliography_preamble_command(tokens, index)?;
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
    let mut list_env: Option<ListEnvV0> = None;
    loop {
        if list_env.is_none()
            && matches!(
                tokens.get(index),
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
            )
            && consume_group_literal(tokens, skip_spaces(tokens, index + 1), b"document").is_some()
        {
            break;
        }
        index = consume_ok_body_token_v0(
            tokens,
            index,
            tokens.len(),
            false,
            &mut list_env,
            &mut body,
            &mut previous_was_space,
        )?;
    }
    if list_env.is_some() {
        return None;
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
