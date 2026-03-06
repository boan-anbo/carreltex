fn is_safe_graphics_path_byte_v0(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-')
}

fn has_allowed_graphics_extension_v0(path: &[u8]) -> bool {
    let Some(last_segment) = path.rsplit(|byte| *byte == b'/').next() else {
        return false;
    };
    let Some(dot_index) = last_segment.iter().rposition(|byte| *byte == b'.') else {
        return false;
    };
    if dot_index == 0 || dot_index + 1 >= last_segment.len() {
        return false;
    }
    let ext = last_segment[dot_index + 1..]
        .iter()
        .map(u8::to_ascii_lowercase)
        .collect::<Vec<u8>>();
    matches!(ext.as_slice(), b"png" | b"jpg" | b"jpeg" | b"pdf")
}

fn parse_decimal_milli_v0(value: &[u8]) -> Option<u32> {
    if value.is_empty() {
        return None;
    }
    if value
        .iter()
        .any(|byte| matches!(byte, b'+' | b'-' | b'e' | b'E'))
    {
        return None;
    }
    let mut dot_index = None::<usize>;
    for (index, byte) in value.iter().enumerate() {
        if *byte == b'.' {
            if dot_index.is_some() {
                return None;
            }
            dot_index = Some(index);
            continue;
        }
        if !byte.is_ascii_digit() {
            return None;
        }
    }
    let (int_part, frac_part) = if let Some(index) = dot_index {
        (&value[..index], &value[index + 1..])
    } else {
        (value, &b""[..])
    };
    if int_part.is_empty() || frac_part.len() > 3 {
        return None;
    }
    let int_value = std::str::from_utf8(int_part).ok()?.parse::<u64>().ok()?;
    let mut frac_value = 0u64;
    if !frac_part.is_empty() {
        frac_value = std::str::from_utf8(frac_part).ok()?.parse::<u64>().ok()?;
        for _ in 0..(3 - frac_part.len()) {
            frac_value = frac_value.checked_mul(10)?;
        }
    }
    let milli = int_value.checked_mul(1000)?.checked_add(frac_value)?;
    if milli == 0 || milli > u32::MAX as u64 {
        return None;
    }
    Some(milli as u32)
}

fn convert_dimension_to_milli_pt_v0(value_milli: u32, unit: &[u8]) -> Option<u32> {
    let value = u64::from(value_milli);
    let converted = if unit == b"pt" {
        value
    } else if unit == b"in" {
        value.checked_mul(72)?
    } else if unit == b"cm" {
        value.checked_mul(3600)?.checked_div(127)?
    } else if unit == b"mm" {
        value.checked_mul(360)?.checked_div(127)?
    } else {
        return None;
    };
    if converted == 0 || converted > u32::MAX as u64 {
        return None;
    }
    Some(converted as u32)
}

fn parse_dimension_milli_pt_v0(value: &[u8]) -> Option<u32> {
    let trimmed = trim_horizontal_space_bytes_v0(value);
    if trimmed.is_empty() {
        return None;
    }
    if let Some(prefix) = trimmed.strip_suffix(b"\\textwidth") {
        let ratio_milli = parse_decimal_milli_v0(trim_horizontal_space_bytes_v0(prefix))?;
        return scale_dimension_milli_pt_v0(MAX_FIGURE_PLACEHOLDER_WIDTH_MPT_V0, ratio_milli);
    }
    let mut suffix_start = trimmed.len();
    while suffix_start > 0 && trimmed[suffix_start - 1].is_ascii_alphabetic() {
        suffix_start -= 1;
    }
    let unit = if suffix_start == trimmed.len() {
        b"pt".as_slice()
    } else {
        &trimmed[suffix_start..]
    };
    if !INCLUDEGRAPHICS_ALLOWED_WIDTH_UNITS_V0
        .iter()
        .any(|allowed| *allowed == unit)
    {
        return None;
    }
    let number_part = trim_horizontal_space_bytes_v0(&trimmed[..suffix_start]);
    let value_milli = parse_decimal_milli_v0(number_part)?;
    convert_dimension_to_milli_pt_v0(value_milli, unit)
}

fn scale_dimension_milli_pt_v0(base_milli: u32, scale_milli: u32) -> Option<u32> {
    let scaled = u64::from(base_milli)
        .checked_mul(u64::from(scale_milli))?
        .checked_div(1000)?;
    if scaled == 0 || scaled > u32::MAX as u64 {
        return None;
    }
    Some(scaled as u32)
}

fn scale_by_ratio_milli_pt_v0(value: u32, numerator: u32, denominator: u32) -> Option<u32> {
    let scaled = u64::from(value)
        .checked_mul(u64::from(numerator))?
        .checked_div(u64::from(denominator))?;
    if scaled == 0 || scaled > u32::MAX as u64 {
        return None;
    }
    Some(scaled as u32)
}

fn apply_figure_sizing_caps_v0(sizing: FigureSizingMptV0) -> FigureSizingMptV0 {
    FigureSizingMptV0 {
        width_mpt: sizing
            .width_mpt
            .min(MAX_FIGURE_PLACEHOLDER_WIDTH_MPT_V0)
            .max(1),
        height_mpt: sizing
            .height_mpt
            .min(MAX_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0)
            .max(1),
    }
}

#[derive(Clone)]
struct IncludeGraphicsOptionsV0 {
    sizing: FigureSizingMptV0,
    path_prefix: Option<Vec<u8>>,
    ext_override: Option<Vec<u8>>,
}

fn normalize_graphics_path_prefix_option_v0(raw_value: &[u8]) -> Option<Vec<u8>> {
    let trimmed = trim_horizontal_space_bytes_v0(raw_value);
    if trimmed.is_empty() {
        return None;
    }
    let mut without_trailing = trimmed.to_vec();
    while without_trailing.ends_with(b"/") {
        without_trailing.pop();
    }
    if without_trailing.is_empty() {
        return None;
    }
    if without_trailing.starts_with(b"/") || without_trailing.starts_with(b"\\") {
        return None;
    }
    if without_trailing.contains(&b'\\') || without_trailing.contains(&b':') {
        return None;
    }
    if !without_trailing
        .iter()
        .copied()
        .all(is_safe_graphics_path_byte_v0)
    {
        return None;
    }
    let mut normalized_segments = Vec::<Vec<u8>>::new();
    for segment in without_trailing.split(|byte| *byte == b'/') {
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
    Some(normalized)
}

fn normalize_graphics_extension_option_v0(raw_value: &[u8]) -> Option<Vec<u8>> {
    let trimmed = trim_horizontal_space_bytes_v0(raw_value);
    if trimmed.is_empty() {
        return None;
    }
    let ext = if let Some(rest) = trimmed.strip_prefix(b".") {
        rest
    } else {
        trimmed
    };
    if ext.is_empty() || !ext.iter().all(u8::is_ascii_alphanumeric) {
        return None;
    }
    let lowered = ext
        .iter()
        .map(u8::to_ascii_lowercase)
        .collect::<Vec<u8>>();
    if !matches!(lowered.as_slice(), b"png" | b"jpg" | b"jpeg" | b"pdf") {
        return None;
    }
    Some(lowered)
}

fn parse_positive_integer_option_v0(raw_value: &[u8]) -> Option<u32> {
    let trimmed = trim_horizontal_space_bytes_v0(raw_value);
    if trimmed.is_empty() || !trimmed.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let parsed = std::str::from_utf8(trimmed).ok()?.parse::<u32>().ok()?;
    if parsed == 0 {
        return None;
    }
    Some(parsed)
}

fn parse_includegraphics_options_v0(raw: &[u8]) -> Option<IncludeGraphicsOptionsV0> {
    let mut width_mpt = None::<u32>;
    let mut height_mpt = None::<u32>;
    let mut scale_milli = None::<u32>;
    let mut path_prefix = None::<Vec<u8>>;
    let mut ext_override = None::<Vec<u8>>;
    let mut saw_keepaspectratio = false;
    let mut saw_page = false;
    let mut saw_entry = false;
    for chunk in raw.split(|byte| *byte == b',') {
        let trimmed = trim_horizontal_space_bytes_v0(chunk);
        if trimmed.is_empty() {
            return None;
        }
        saw_entry = true;
        let equals_index = trimmed.iter().position(|byte| *byte == b'=');
        let (key_raw, value_opt) = if let Some(equals_index) = equals_index {
            if equals_index == 0 || equals_index + 1 >= trimmed.len() {
                return None;
            }
            (
                trim_horizontal_space_bytes_v0(&trimmed[..equals_index]),
                Some(trim_horizontal_space_bytes_v0(&trimmed[equals_index + 1..])),
            )
        } else {
            (trimmed, None)
        };
        let key = key_raw
            .iter()
            .map(u8::to_ascii_lowercase)
            .collect::<Vec<u8>>();
        match key.as_slice() {
            b"width" => {
                if width_mpt.is_some() {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                width_mpt = Some(parse_dimension_milli_pt_v0(value)?);
            }
            b"height" => {
                if height_mpt.is_some() {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                height_mpt = Some(parse_dimension_milli_pt_v0(value)?);
            }
            b"scale" => {
                if scale_milli.is_some() {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                scale_milli = Some(parse_decimal_milli_v0(value)?);
            }
            b"dir" | b"path" => {
                if path_prefix.is_some() {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                path_prefix = Some(normalize_graphics_path_prefix_option_v0(value)?);
            }
            b"ext" | b"extension" | b"type" => {
                if ext_override.is_some() {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                ext_override = Some(normalize_graphics_extension_option_v0(value)?);
            }
            b"page" => {
                if saw_page {
                    return None;
                }
                let value = value_opt?;
                if value.is_empty() {
                    return None;
                }
                parse_positive_integer_option_v0(value)?;
                saw_page = true;
            }
            b"keepaspectratio" => {
                if value_opt.is_some() || saw_keepaspectratio {
                    return None;
                }
                saw_keepaspectratio = true;
            }
            _ => return None,
        }
    }
    if !saw_entry {
        return None;
    }
    if scale_milli.is_some() && (width_mpt.is_some() || height_mpt.is_some()) {
        return None;
    }
    let default_sizing = FigureSizingMptV0 {
        width_mpt: DEFAULT_FIGURE_PLACEHOLDER_WIDTH_MPT_V0,
        height_mpt: DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0,
    };
    let resolved = if let Some(scale) = scale_milli {
        FigureSizingMptV0 {
            width_mpt: scale_dimension_milli_pt_v0(default_sizing.width_mpt, scale)?,
            height_mpt: scale_dimension_milli_pt_v0(default_sizing.height_mpt, scale)?,
        }
    } else {
        match (width_mpt, height_mpt) {
            (Some(width), Some(height)) => FigureSizingMptV0 {
                width_mpt: width,
                height_mpt: height,
            },
            (Some(width), None) => FigureSizingMptV0 {
                width_mpt: width,
                height_mpt: scale_by_ratio_milli_pt_v0(
                    width,
                    DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0,
                    DEFAULT_FIGURE_PLACEHOLDER_WIDTH_MPT_V0,
                )?,
            },
            (None, Some(height)) => FigureSizingMptV0 {
                width_mpt: scale_by_ratio_milli_pt_v0(
                    height,
                    DEFAULT_FIGURE_PLACEHOLDER_WIDTH_MPT_V0,
                    DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0,
                )?,
                height_mpt: height,
            },
            (None, None) => return None,
        }
    };
    Some(IncludeGraphicsOptionsV0 {
        sizing: apply_figure_sizing_caps_v0(resolved),
        path_prefix,
        ext_override,
    })
}

fn normalize_graphics_path_v0(raw_path: &[u8], ext_override: Option<&[u8]>) -> Option<Vec<u8>> {
    if raw_path.is_empty() {
        return None;
    }
    if raw_path.starts_with(b"/") || raw_path.starts_with(b"\\") {
        return None;
    }
    if raw_path.contains(&b'\\') || raw_path.contains(&b':') {
        return None;
    }
    if !raw_path.iter().copied().all(is_safe_graphics_path_byte_v0) {
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
    if !has_allowed_graphics_extension_v0(&normalized) {
        let has_explicit_extension = normalized
            .rsplit(|byte| *byte == b'/')
            .next()
            .map(|last: &[u8]| last.contains(&b'.'))
            .unwrap_or(false);
        if has_explicit_extension {
            return None;
        }
        if let Some(ext) = ext_override {
            normalized.push(b'.');
            normalized.extend_from_slice(ext);
        } else {
            normalized.extend_from_slice(b".png");
        }
    }
    if !has_allowed_graphics_extension_v0(&normalized) {
        return None;
    }
    Some(normalized)
}

fn normalize_graphics_prefix_v0(raw_prefix: &[u8]) -> Option<Vec<u8>> {
    if raw_prefix.is_empty() || !raw_prefix.ends_with(b"/") {
        return None;
    }
    let prefix_without_trailing = &raw_prefix[..raw_prefix.len() - 1];
    if prefix_without_trailing.is_empty() {
        return None;
    }
    if prefix_without_trailing.starts_with(b"/") || prefix_without_trailing.starts_with(b"\\") {
        return None;
    }
    if prefix_without_trailing.contains(&b'\\') || prefix_without_trailing.contains(&b':') {
        return None;
    }
    if !prefix_without_trailing
        .iter()
        .copied()
        .all(is_safe_graphics_path_byte_v0)
    {
        return None;
    }
    let mut normalized_segments = Vec::<Vec<u8>>::new();
    for segment in prefix_without_trailing.split(|byte| *byte == b'/') {
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
    Some(normalized)
}

fn resolve_includegraphics_paths_with_prefixes_v0(
    raw_path: &[u8],
    graphicspath_prefixes: &[Vec<u8>],
    ext_override: Option<&[u8]>,
) -> Option<Vec<Vec<u8>>> {
    let use_graphicspath_prefixes = !graphicspath_prefixes.is_empty()
        && !raw_path.contains(&b'/')
        && !raw_path.contains(&b'\\');
    if !use_graphicspath_prefixes {
        return Some(vec![normalize_graphics_path_v0(raw_path, ext_override)?]);
    }
    let mut candidates = Vec::<Vec<u8>>::new();
    for prefix in graphicspath_prefixes {
        let mut prefixed_path = Vec::<u8>::new();
        prefixed_path.extend_from_slice(prefix);
        prefixed_path.push(b'/');
        prefixed_path.extend_from_slice(raw_path);
        candidates.push(normalize_graphics_path_v0(&prefixed_path, ext_override)?);
    }
    if candidates.is_empty() {
        return None;
    }
    Some(candidates)
}

fn consume_includegraphics_command_v0(
    tokens: &[TokenV0],
    index: usize,
    graphicspath_prefixes: &[Vec<u8>],
) -> Option<(IncludeGraphicsCommandV0, usize)> {
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == INCLUDEGRAPHICS_CONTROL_V0
    ) {
        return None;
    }
    let mut group_index = skip_spaces(tokens, index + 1);
    let mut options = IncludeGraphicsOptionsV0 {
        sizing: FigureSizingMptV0 {
            width_mpt: DEFAULT_FIGURE_PLACEHOLDER_WIDTH_MPT_V0,
            height_mpt: DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0,
        },
        path_prefix: None,
        ext_override: None,
    };
    if matches!(tokens.get(group_index), Some(TokenV0::Char(b'['))) {
        let mut cursor = group_index + 1;
        let mut option_bytes = Vec::<u8>::new();
        let mut saw_non_space = false;
        while let Some(token) = tokens.get(cursor) {
            match token {
                TokenV0::Char(b']') => {
                    if !saw_non_space {
                        return None;
                    }
                    options = parse_includegraphics_options_v0(&option_bytes)?;
                    group_index = cursor + 1;
                    break;
                }
                TokenV0::Char(byte) => {
                    option_bytes.push(*byte);
                    if !is_horizontal_space_v0(*byte) {
                        saw_non_space = true;
                    }
                }
                TokenV0::Space => option_bytes.push(b' '),
                TokenV0::ControlSeq(name) => {
                    option_bytes.push(b'\\');
                    option_bytes.extend_from_slice(name);
                    saw_non_space = true;
                }
                _ => return None,
            }
            cursor += 1;
        }
        if !matches!(tokens.get(cursor), Some(TokenV0::Char(b']'))) {
            return None;
        }
    }
    let (group_start, group_end, next) = consume_group_bounds(tokens, group_index)?;
    let mut raw_path = Vec::<u8>::new();
    for token in &tokens[group_start..group_end] {
        match token {
            TokenV0::Char(byte) if is_safe_graphics_path_byte_v0(*byte) => raw_path.push(*byte),
            _ => return None,
        }
    }
    if raw_path.is_empty() {
        return None;
    }
    let resolved_raw_path = if let Some(prefix) = options.path_prefix.as_ref() {
        let mut prefixed = Vec::<u8>::new();
        prefixed.extend_from_slice(prefix);
        prefixed.push(b'/');
        prefixed.extend_from_slice(&raw_path);
        prefixed
    } else {
        raw_path
    };
    let mut candidates =
        resolve_includegraphics_paths_with_prefixes_v0(
            &resolved_raw_path,
            graphicspath_prefixes,
            options.ext_override.as_deref(),
        )?;
    let normalized = candidates.remove(0);
    Some((
        IncludeGraphicsCommandV0 {
            path: normalized,
            sizing: options.sizing,
        },
        next,
    ))
}

fn emit_inline_includegraphics_placeholder_v0(
    out: &mut Vec<u8>,
    image: &IncludeGraphicsCommandV0,
    figure_anchor_id: u32,
    figure_ordinal: u32,
) {
    push_paragraph_break(out);
    out.extend_from_slice(FIGURE_BOX_PREFIX_MARKER_V0);
    push_newline(out);
    out.extend_from_slice(FIGURE_IMAGE_PREFIX_MARKER_V0);
    out.extend_from_slice(figure_anchor_id.to_string().as_bytes());
    out.push(b' ');
    out.extend_from_slice(&image.path);
    out.push(b' ');
    out.extend_from_slice(image.sizing.width_mpt.to_string().as_bytes());
    out.push(b' ');
    out.extend_from_slice(image.sizing.height_mpt.to_string().as_bytes());
    push_newline(out);
    out.extend_from_slice(FIGURE_CAPTION_PREFIX_MARKER_V0);
    out.extend_from_slice(b"Figure ");
    out.extend_from_slice(figure_ordinal.to_string().as_bytes());
    out.extend_from_slice(b": Inline graphic");
    push_newline(out);
    push_paragraph_break(out);
}

fn consume_tabular_environment_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != TABULAR_ENV_V0 {
        return None;
    }

    let (align_start, align_end, next_after_align) = consume_group_bounds(tokens, cursor)?;
    let mut align_spec = Vec::<u8>::new();
    for token in &tokens[align_start..align_end] {
        match token {
            TokenV0::Char(byte) if matches!(byte, b'l' | b'c' | b'r') => align_spec.push(*byte),
            _ => return None,
        }
    }
    if align_spec.is_empty() || align_spec.len() > MAX_TABULAR_COLUMNS_V0 {
        return None;
    }
    cursor = next_after_align;

    let mut rows = Vec::<Vec<Vec<u8>>>::new();
    loop {
        cursor = skip_spaces(tokens, cursor);
        if matches!(
            tokens.get(cursor),
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0
        ) {
            let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
            if end_env.as_slice() != TABULAR_ENV_V0 || rows.is_empty() {
                return None;
            }
            push_paragraph_break(out);
            out.extend_from_slice(TABLE_SPEC_PREFIX_MARKER_V0);
            out.extend_from_slice(&align_spec);
            push_newline(out);
            for row in &rows {
                out.extend_from_slice(TABLE_ROW_PREFIX_MARKER_V0);
                for (cell_index, cell) in row.iter().enumerate() {
                    if cell_index > 0 {
                        out.extend_from_slice(b"||");
                    }
                    out.extend_from_slice(cell);
                }
                push_newline(out);
            }
            push_paragraph_break(out);
            return Some(next);
        }

        let mut row = Vec::<Vec<u8>>::new();
        let mut cell = Vec::<u8>::new();
        loop {
            match tokens.get(cursor) {
                Some(TokenV0::Char(b'&')) => {
                    trim_trailing_spaces(&mut cell);
                    if cell.windows(2).any(|window| window == b"||") {
                        return None;
                    }
                    row.push(core::mem::take(&mut cell));
                    cursor += 1;
                }
                Some(TokenV0::ControlSeq(name))
                    if is_hard_line_break_control_v0(name.as_slice()) =>
                {
                    trim_trailing_spaces(&mut cell);
                    if cell.windows(2).any(|window| window == b"||") {
                        return None;
                    }
                    row.push(core::mem::take(&mut cell));
                    cursor += 1;
                    break;
                }
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                    return None;
                }
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                    return None;
                }
                Some(_) => {
                    cursor = consume_fragment_token_v0(tokens, cursor, &mut cell, false, false)?;
                }
                None => return None,
            }
        }
        if row.len() != align_spec.len() {
            return None;
        }
        rows.push(row);
    }
}

fn consume_figure_environment_v0(
    tokens: &[TokenV0],
    index: usize,
    out: &mut Vec<u8>,
    graphicspath_prefixes: &[Vec<u8>],
    figure_anchor_id: u32,
    figure_ordinal: u32,
) -> Option<usize> {
    let (env_name, mut cursor) = consume_env_name_command_v0(tokens, index, BEGIN_CONTROL_V0)?;
    if env_name.as_slice() != FIGURE_ENV_V0 {
        return None;
    }
    let (placement_hint, after_placement) = consume_figure_placement_hint_v0(tokens, cursor)?;
    cursor = after_placement;

    let mut caption: Option<Vec<u8>> = None;
    let mut image: Option<IncludeGraphicsCommandV0> = None;
    loop {
        cursor = skip_spaces(tokens, cursor);
        match tokens.get(cursor) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == END_CONTROL_V0 => {
                let (end_env, next) = consume_env_name_command_v0(tokens, cursor, END_CONTROL_V0)?;
                if end_env.as_slice() != FIGURE_ENV_V0 {
                    return None;
                }
                let mut figure_caption = caption?;
                trim_trailing_spaces(&mut figure_caption);
                if figure_caption.is_empty() {
                    return None;
                }
                if matches!(placement_hint, FigurePlacementHintV0::Top) {
                    if !out.is_empty() && !matches!(out.last().copied(), Some(PAGE_BREAK_MARKER_V0))
                    {
                        push_page_break(out);
                    }
                } else {
                    push_paragraph_break(out);
                }
                out.extend_from_slice(FIGURE_BOX_PREFIX_MARKER_V0);
                if matches!(placement_hint, FigurePlacementHintV0::Top) {
                    out.extend_from_slice(b" t");
                }
                push_newline(out);
                if let Some(image_meta) = &image {
                    out.extend_from_slice(FIGURE_IMAGE_PREFIX_MARKER_V0);
                    out.extend_from_slice(figure_anchor_id.to_string().as_bytes());
                    out.push(b' ');
                    out.extend_from_slice(&image_meta.path);
                    out.push(b' ');
                    out.extend_from_slice(image_meta.sizing.width_mpt.to_string().as_bytes());
                    out.push(b' ');
                    out.extend_from_slice(image_meta.sizing.height_mpt.to_string().as_bytes());
                    push_newline(out);
                }
                out.extend_from_slice(FIGURE_CAPTION_PREFIX_MARKER_V0);
                out.extend_from_slice(b"Figure ");
                out.extend_from_slice(figure_ordinal.to_string().as_bytes());
                out.extend_from_slice(b": ");
                out.extend_from_slice(&figure_caption);
                push_newline(out);
                push_paragraph_break(out);
                return Some(next);
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == BEGIN_CONTROL_V0 => {
                return None;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == CAPTION_CONTROL_V0 => {
                if caption.is_some() {
                    return None;
                }
                let (group_start, group_end, next) = consume_group_bounds(tokens, cursor + 1)?;
                let mut value = Vec::<u8>::new();
                consume_fragment_range_v0(
                    tokens,
                    group_start,
                    group_end,
                    &mut value,
                    false,
                    false,
                )?;
                trim_trailing_spaces(&mut value);
                if value.is_empty() {
                    return None;
                }
                caption = Some(value);
                cursor = next;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == INCLUDEGRAPHICS_CONTROL_V0 => {
                if image.is_some() {
                    return None;
                }
                let (parsed_image, next) =
                    consume_includegraphics_command_v0(tokens, cursor, graphicspath_prefixes)?;
                image = Some(parsed_image);
                cursor = next;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"protect" || name.as_slice() == b"relax" =>
            {
                cursor += 1;
            }
            Some(TokenV0::Char(byte)) if *byte == NEWLINE_MARKER_V0 => {
                cursor += 1;
            }
            Some(TokenV0::Space) => {
                cursor += 1;
            }
            Some(_) => return None,
            None => return None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListKindV0 {
    Itemize,
    Enumerate,
}
