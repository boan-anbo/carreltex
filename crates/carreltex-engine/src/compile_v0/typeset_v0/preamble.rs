fn consume_group_bounds(tokens: &[TokenV0], index: usize) -> Option<(usize, usize, usize)> {
    let start = skip_spaces(tokens, index);
    if !matches!(tokens.get(start), Some(TokenV0::BeginGroup)) {
        return None;
    }
    let mut depth = 1usize;
    let mut cursor = start + 1;
    while let Some(token) = tokens.get(cursor) {
        match token {
            TokenV0::BeginGroup => depth += 1,
            TokenV0::EndGroup => {
                depth -= 1;
                if depth == 0 {
                    return Some((start + 1, cursor, cursor + 1));
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn consume_simple_bracket_non_empty(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut saw_non_space = false;
    while let Some(token) = tokens.get(cursor) {
        match token {
            TokenV0::Char(b']') => {
                return if saw_non_space {
                    Some(cursor + 1)
                } else {
                    None
                }
            }
            TokenV0::Char(_) => saw_non_space = true,
            TokenV0::Space => {}
            _ => return None,
        }
        cursor += 1;
    }
    None
}

fn consume_simple_bracket_options_non_empty_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some(cursor);
    }
    cursor += 1;
    let mut raw = Vec::<u8>::new();
    while let Some(token) = tokens.get(cursor) {
        match token {
            TokenV0::Char(b']') => {
                let trimmed = trim_horizontal_space_bytes_v0(&raw);
                validate_non_empty_comma_values_v0(trimmed)?;
                return Some(cursor + 1);
            }
            TokenV0::Char(byte) => raw.push(*byte),
            TokenV0::Space => raw.push(b' '),
            _ => return None,
        }
        cursor += 1;
    }
    None
}

fn is_safe_package_or_class_name_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-')
}

fn normalize_package_or_class_name_v0(raw_value: &[u8]) -> Option<Vec<u8>> {
    let value = trim_horizontal_space_bytes_v0(raw_value);
    if value.is_empty()
        || value.starts_with(b"/")
        || value.starts_with(b"\\")
        || value.windows(2).any(|window| window == b"..")
        || !value
            .iter()
            .copied()
            .all(is_safe_package_or_class_name_byte_v0)
    {
        return None;
    }
    Some(value.to_vec())
}

fn validate_non_empty_comma_values_v0(raw_value: &[u8]) -> Option<()> {
    let mut saw_value = false;
    for raw_segment in raw_value.split(|byte| *byte == b',') {
        if trim_horizontal_space_bytes_v0(raw_segment).is_empty() {
            return None;
        }
        saw_value = true;
    }
    if !saw_value {
        return None;
    }
    Some(())
}

fn validate_package_or_class_list_v0(raw_group: &[u8]) -> Option<()> {
    let mut saw_target = false;
    for raw_segment in raw_group.split(|byte| *byte == b',') {
        normalize_package_or_class_name_v0(raw_segment)?;
        saw_target = true;
    }
    if !saw_target {
        return None;
    }
    Some(())
}

fn consume_documentclass_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == DOCUMENTCLASS_CONTROL_V0
    ) {
        return None;
    }
    let mut cursor = consume_simple_bracket_options_non_empty_v0(tokens, index + 1)?;
    let (group_start, group_end, next) = consume_group_bounds(tokens, cursor)?;
    let class_bytes = parse_char_space_group_trimmed_v0(tokens, group_start, group_end)?;
    normalize_package_or_class_name_v0(&class_bytes)?;
    cursor = next;
    Some(cursor)
}

fn consume_graphicspath_declaration_v0(
    tokens: &[TokenV0],
    index: usize,
) -> Option<(usize, Vec<Vec<u8>>)> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == GRAPHICSPATH_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut prefixes = Vec::<Vec<u8>>::new();
    let mut cursor = group_start;
    while cursor < group_end {
        cursor = skip_spaces(tokens, cursor);
        if cursor >= group_end {
            break;
        }
        if !matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
            return None;
        }
        let (entry_start, entry_end, entry_next) = consume_group_bounds(tokens, cursor)?;
        let raw_entry = parse_char_space_group_trimmed_v0(tokens, entry_start, entry_end)?;
        let normalized = normalize_graphics_prefix_v0(&raw_entry)?;
        prefixes.push(normalized);
        cursor = entry_next;
    }
    if prefixes.is_empty() {
        return None;
    }
    Some((next, prefixes))
}

fn consume_env_name_command_v0(
    tokens: &[TokenV0],
    index: usize,
    command_name: &[u8],
) -> Option<(Vec<u8>, usize)> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(control)) if control.as_slice() == command_name
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let mut env_bytes = Vec::new();
    for token in &tokens[group_start..group_end] {
        match token {
            TokenV0::Char(byte) => env_bytes.push(*byte),
            TokenV0::Space => env_bytes.push(b' '),
            _ => return None,
        }
    }
    while matches!(env_bytes.first(), Some(b' ')) {
        env_bytes.remove(0);
    }
    while matches!(env_bytes.last(), Some(b' ')) {
        env_bytes.pop();
    }
    if env_bytes.is_empty() {
        return None;
    }
    Some((env_bytes, next))
}

fn consume_figure_placement_hint_v0(
    tokens: &[TokenV0],
    index: usize,
) -> Option<(FigurePlacementHintV0, usize)> {
    let mut cursor = skip_spaces(tokens, index);
    if !matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
        return Some((FigurePlacementHintV0::Inline, cursor));
    }
    cursor += 1;
    let mut saw_non_space = false;
    let mut saw_top = false;
    loop {
        match tokens.get(cursor) {
            Some(TokenV0::Char(b']')) => {
                if !saw_non_space || !saw_top {
                    return None;
                }
                return Some((FigurePlacementHintV0::Top, cursor + 1));
            }
            Some(TokenV0::Space) => {}
            Some(TokenV0::Char(byte)) if is_horizontal_space_v0(*byte) => {}
            Some(TokenV0::Char(b't')) if !saw_top => {
                saw_non_space = true;
                saw_top = true;
            }
            Some(TokenV0::Char(_)) => return None,
            _ => return None,
        }
        cursor += 1;
    }
}

fn consume_document_env_command_v0(tokens: &[TokenV0], index: usize, name: &[u8]) -> Option<usize> {
    let (env_name, next) = consume_env_name_command_v0(tokens, index, name)?;
    if env_name.as_slice() != DOCUMENT_ENV_V0 {
        return None;
    }
    Some(next)
}

fn consume_bibliographystyle_command_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == BIBLIOGRAPHYSTYLE_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let style = parse_char_space_group_trimmed_v0(tokens, group_start, group_end)?;
    if style.is_empty() {
        return None;
    }
    Some(next)
}

fn consume_package_declaration_noop_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == USEPACKAGE_CONTROL_V0 || name.as_slice() == REQUIREPACKAGE_CONTROL_V0
    ) {
        return None;
    }
    let mut cursor = consume_simple_bracket_options_non_empty_v0(tokens, index + 1)?;
    let (group_start, group_end, next) = consume_group_bounds(tokens, cursor)?;
    let raw_group = parse_char_space_group_trimmed_v0(tokens, group_start, group_end)?;
    validate_package_or_class_list_v0(&raw_group)?;
    cursor = next;
    Some(cursor)
}

fn consume_requirepackage_with_options_declaration_noop_v0(
    tokens: &[TokenV0],
    index: usize,
) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == REQUIREPACKAGEWITHOPTIONS_CONTROL_V0
    ) {
        return None;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, index + 1)?;
    let raw_group = parse_char_space_group_trimmed_v0(tokens, group_start, group_end)?;
    validate_package_or_class_list_v0(&raw_group)?;
    Some(next)
}

fn consume_pass_options_declaration_noop_v0(tokens: &[TokenV0], index: usize) -> Option<usize> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name))
            if name.as_slice() == PASSOPTIONSTOPACKAGE_CONTROL_V0
                || name.as_slice() == PASSOPTIONSTOCLASS_CONTROL_V0
    ) {
        return None;
    }
    let (option_start, option_end, option_next) = consume_group_bounds(tokens, index + 1)?;
    let option_group = parse_char_space_group_trimmed_v0(tokens, option_start, option_end)?;
    validate_non_empty_comma_values_v0(&option_group)?;
    let (target_start, target_end, next) = consume_group_bounds(tokens, option_next)?;
    let raw_group = parse_char_space_group_trimmed_v0(tokens, target_start, target_end)?;
    validate_package_or_class_list_v0(&raw_group)?;
    Some(next)
}
