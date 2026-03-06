fn is_safe_bib_resource_path_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-')
}

fn normalize_bib_resource_name_v0(raw_path: &[u8]) -> Option<Vec<u8>> {
    if raw_path.is_empty() {
        return None;
    }
    if raw_path.starts_with(b"/") || raw_path.starts_with(b"\\") {
        return None;
    }
    if raw_path.contains(&b'\\') || raw_path.contains(&b':') {
        return None;
    }
    if !raw_path
        .iter()
        .copied()
        .all(is_safe_bib_resource_path_byte_v0)
    {
        return None;
    }

    let mut normalized_segments = Vec::<Vec<u8>>::new();
    for segment in raw_path.split(|byte| *byte == b'/') {
        if segment.is_empty() || segment == b"." || segment == b".." {
            return None;
        }
        normalized_segments.push(segment.to_vec());
    }
    if normalized_segments.is_empty() {
        return None;
    }

    let mut normalized = Vec::<u8>::new();
    for (segment_index, segment) in normalized_segments.iter().enumerate() {
        if segment_index > 0 {
            normalized.push(b'/');
        }
        normalized.extend_from_slice(segment);
    }
    let has_explicit_extension = normalized
        .rsplit(|byte| *byte == b'/')
        .next()
        .map(|last: &[u8]| last.contains(&b'.'))
        .unwrap_or(false);
    if has_explicit_extension {
        if !normalized.ends_with(b".bib") {
            return None;
        }
    } else {
        normalized.extend_from_slice(b".bib");
    }
    Some(normalized)
}

fn split_bibliography_group_values_v0(raw_group: &[u8]) -> Option<Vec<Vec<u8>>> {
    let mut values = Vec::<Vec<u8>>::new();
    let mut segment_start = 0usize;
    for (index, byte) in raw_group.iter().enumerate() {
        if *byte == b',' {
            let raw_segment = trim_horizontal_space_bytes_v0(&raw_group[segment_start..index]);
            if raw_segment.is_empty() {
                return None;
            }
            values.push(normalize_bib_resource_name_v0(raw_segment)?);
            segment_start = index + 1;
        }
    }
    let raw_tail = trim_horizontal_space_bytes_v0(&raw_group[segment_start..]);
    if raw_tail.is_empty() {
        return None;
    }
    values.push(normalize_bib_resource_name_v0(raw_tail)?);
    Some(values)
}

fn parse_bibliography_resource_command_v0(
    tokens: &[TokenV0],
    index: usize,
) -> Option<(Vec<Vec<u8>>, usize)> {
    let command = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    if command != ADDBIBRESOURCE_CONTROL_V0 && command != BIBLIOGRAPHY_CONTROL_V0 {
        return None;
    }
    let mut cursor = index + 1;
    if command == ADDBIBRESOURCE_CONTROL_V0 {
        cursor = consume_simple_bracket_non_empty(tokens, cursor)?;
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, cursor)?;
    let raw_group = parse_char_space_group_trimmed_v0(tokens, group_start, group_end)?;
    let resources = if command == ADDBIBRESOURCE_CONTROL_V0 {
        vec![normalize_bib_resource_name_v0(&raw_group)?]
    } else {
        split_bibliography_group_values_v0(&raw_group)?
    };
    Some((resources, next))
}

fn skip_horizontal_space_bytes_v0(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && is_horizontal_space_v0(bytes[index]) {
        index += 1;
    }
    index
}

fn normalize_bib_value_text_v0(raw: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::<u8>::new();
    let mut saw_non_space = false;
    let mut pending_space = false;
    for byte in raw {
        if matches!(*byte, b'{' | b'}' | b'"') {
            continue;
        }
        if byte.is_ascii_whitespace() {
            pending_space = saw_non_space;
            continue;
        }
        if !byte.is_ascii_graphic() && !byte.is_ascii_alphanumeric() {
            return None;
        }
        if pending_space {
            out.push(b' ');
            pending_space = false;
        }
        out.push(*byte);
        saw_non_space = true;
    }
    trim_trailing_spaces(&mut out);
    if out.is_empty() {
        return None;
    }
    Some(out)
}

fn extract_bib_title_field_v0(entry_body: &[u8]) -> Option<Vec<u8>> {
    let mut index = 0usize;
    while index < entry_body.len() {
        if !entry_body[index].is_ascii_alphabetic() {
            index += 1;
            continue;
        }
        let field_start = index;
        while index < entry_body.len() && entry_body[index].is_ascii_alphabetic() {
            index += 1;
        }
        let field_name = entry_body[field_start..index]
            .iter()
            .map(u8::to_ascii_lowercase)
            .collect::<Vec<u8>>();
        index = skip_horizontal_space_bytes_v0(entry_body, index);
        if index >= entry_body.len() || entry_body[index] != b'=' {
            continue;
        }
        index += 1;
        index = skip_horizontal_space_bytes_v0(entry_body, index);
        if field_name != b"title" {
            while index < entry_body.len() && entry_body[index] != b',' {
                index += 1;
            }
            continue;
        }
        if index >= entry_body.len() {
            return None;
        }
        if entry_body[index] == b'{' {
            let mut depth = 1usize;
            let mut cursor = index + 1;
            while cursor < entry_body.len() {
                match entry_body[cursor] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return normalize_bib_value_text_v0(&entry_body[index + 1..cursor]);
                        }
                    }
                    _ => {}
                }
                cursor += 1;
            }
            return None;
        }
        if entry_body[index] == b'"' {
            let mut cursor = index + 1;
            while cursor < entry_body.len() {
                if entry_body[cursor] == b'"' && entry_body[cursor.saturating_sub(1)] != b'\\' {
                    return normalize_bib_value_text_v0(&entry_body[index + 1..cursor]);
                }
                cursor += 1;
            }
            return None;
        }
        let mut cursor = index;
        while cursor < entry_body.len() && entry_body[cursor] != b',' {
            cursor += 1;
        }
        return normalize_bib_value_text_v0(&entry_body[index..cursor]);
    }
    None
}

pub(crate) fn parse_minimal_bib_entries_v0(bib_bytes: &[u8]) -> Option<BTreeMap<Vec<u8>, Vec<u8>>> {
    let mut entries = BTreeMap::<Vec<u8>, Vec<u8>>::new();
    let mut index = 0usize;

    while index < bib_bytes.len() {
        if bib_bytes[index] == b'%' {
            while index < bib_bytes.len() && bib_bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if bib_bytes[index] != b'@' {
            index += 1;
            continue;
        }
        index += 1;
        index = skip_horizontal_space_bytes_v0(bib_bytes, index);
        let type_start = index;
        while index < bib_bytes.len() && bib_bytes[index].is_ascii_alphabetic() {
            index += 1;
        }
        if index == type_start {
            return None;
        }
        index = skip_horizontal_space_bytes_v0(bib_bytes, index);
        if !matches!(bib_bytes.get(index), Some(b'{')) {
            return None;
        }
        index += 1;
        index = skip_horizontal_space_bytes_v0(bib_bytes, index);
        let key_start = index;
        while index < bib_bytes.len() && !matches!(bib_bytes[index], b',' | b'}') {
            index += 1;
        }
        if index == key_start || !matches!(bib_bytes.get(index), Some(b',')) {
            return None;
        }
        let key_raw = trim_horizontal_space_bytes_v0(&bib_bytes[key_start..index]);
        if key_raw.is_empty()
            || !key_raw.iter().copied().all(is_safe_label_key_byte_v0)
            || key_raw.starts_with(b"/")
            || key_raw.windows(2).any(|window| window == b"..")
        {
            return None;
        }
        index += 1;
        let body_start = index;
        let mut depth = 1usize;
        while index < bib_bytes.len() {
            match bib_bytes[index] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            index += 1;
        }
        if depth != 0 {
            return None;
        }
        let entry_body = &bib_bytes[body_start..index];
        let title = extract_bib_title_field_v0(entry_body).unwrap_or_else(|| key_raw.to_vec());
        if title.is_empty() {
            return None;
        }
        if entries.insert(key_raw.to_vec(), title).is_some() {
            return None;
        }
        index += 1;
    }

    Some(entries)
}

pub(crate) fn collect_bibliography_resource_names_v0(tokens: &[TokenV0]) -> Option<Vec<Vec<u8>>> {
    let mut resources = BTreeMap::<Vec<u8>, ()>::new();
    let mut index = 0usize;
    while index < tokens.len() {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == ADDBIBRESOURCE_CONTROL_V0
                    || name.as_slice() == BIBLIOGRAPHY_CONTROL_V0 =>
            {
                let (resource_names, next) = parse_bibliography_resource_command_v0(tokens, index)?;
                for resource in resource_names {
                    resources.insert(resource, ());
                }
                index = next;
            }
            _ => index += 1,
        }
    }
    Some(resources.into_keys().collect())
}

fn emit_bibliography_block_v0(out: &mut Vec<u8>, items: &[BibItemMetaV0]) {
    push_paragraph_break(out);
    out.extend_from_slice(SECTION_HEADING_PREFIX_MARKER_V0);
    out.push(BOLD_START_MARKER_V0);
    out.extend_from_slice(b"References");
    out.push(BOLD_END_MARKER_V0);
    push_paragraph_break(out);
    for (ordinal_index, item) in items.iter().enumerate() {
        let ordinal = ordinal_index + 1;
        out.push(b'[');
        out.extend_from_slice(ordinal.to_string().as_bytes());
        out.extend_from_slice(b"] ");
        out.extend_from_slice(&item.text);
        push_newline(out);
    }
    push_paragraph_break(out);
}

fn consume_thebibliography_environment_v0(
    tokens: &[TokenV0],
    index: usize,
    bibitems: &mut Vec<BibItemMetaV0>,
) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != THEBIBLIOGRAPHY_ENV_V0 {
        return None;
    }

    let (width_group_start, width_group_end, width_group_next) =
        consume_group_bounds(tokens, cursor)?;
    let width_hint = parse_char_space_group_trimmed_v0(tokens, width_group_start, width_group_end)?;
    if width_hint.is_empty() {
        return None;
    }
    cursor = width_group_next;

    let mut local_items = Vec::<BibItemMetaV0>::new();
    loop {
        cursor = skip_spaces(tokens, cursor);
        if matches!(
            tokens.get(cursor),
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0
        ) {
            let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
            if end_env.as_slice() != THEBIBLIOGRAPHY_ENV_V0 || local_items.is_empty() {
                return None;
            }
            bibitems.extend(local_items);
            return Some(next);
        }

        if !matches!(
            tokens.get(cursor),
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BIBITEM_CONTROL_V0
        ) {
            return None;
        }

        let (key, next_after_key) = parse_label_or_ref_key_group_v0(tokens, cursor)?;
        if local_items.iter().any(|item| item.key == key)
            || bibitems.iter().any(|item| item.key == key)
        {
            return None;
        }
        cursor = next_after_key;

        let mut item_text = Vec::<u8>::new();
        loop {
            match tokens.get(cursor) {
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == BIBITEM_CONTROL_V0 => break,
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => break,
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                    return None
                }
                Some(_) => {
                    cursor =
                        consume_fragment_token_v0(tokens, cursor, &mut item_text, false, true)?;
                }
                None => return None,
            }
        }
        trim_trailing_spaces(&mut item_text);
        if item_text.is_empty() {
            return None;
        }
        let text_len = u32::try_from(item_text.len()).ok()?;
        local_items.push(BibItemMetaV0 {
            key,
            text: item_text,
            text_len,
        });
    }
}
