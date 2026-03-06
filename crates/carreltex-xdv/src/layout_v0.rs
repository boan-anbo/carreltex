const PAGEBREAK_MARKER_V0: u8 = 0x0c;
const NEWLINE_MARKER_V0: u8 = 0x0a;
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';
const LINK_START_MARKER_V0: u8 = b'<';
const LINK_END_MARKER_V0: u8 = b'>';

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutPlanV0 {
    pub pages: Vec<PagePlanV0>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PagePlanV0 {
    pub lines: Vec<LinePlanV0>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinePlanV0 {
    pub glyphs: Vec<GlyphPlanV0>,
    pub width_sp: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphPlanV0 {
    pub byte: u8,
    pub advance_sp: i32,
}

fn is_supported_text_byte_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

pub(crate) fn is_style_marker_byte_v0(byte: u8) -> bool {
    matches!(
        byte,
        ITALIC_START_MARKER_V0
            | ITALIC_END_MARKER_V0
            | BOLD_START_MARKER_V0
            | BOLD_END_MARKER_V0
            | LINK_START_MARKER_V0
            | LINK_END_MARKER_V0
    )
}

fn glyph_width_ratio_v0(byte: u8) -> (u32, u32) {
    match byte {
        b' ' => (1, 2),
        b'.' | b',' | b';' | b':' | b'!' | b'?' | b'\'' | b'"' => (1, 2),
        b'i' | b'l' | b'I' | b'|' => (1, 2),
        b'm' | b'w' | b'M' | b'W' => (3, 2),
        _ => (1, 1),
    }
}

pub(crate) fn glyph_width_sp_v0(byte: u8, glyph_advance_sp: i32) -> Option<i32> {
    if glyph_advance_sp <= 0 {
        return None;
    }
    if is_style_marker_byte_v0(byte) {
        return Some(0);
    }
    let (num, den) = glyph_width_ratio_v0(byte);
    let glyph_advance = i64::from(glyph_advance_sp);
    let width = (glyph_advance
        .checked_mul(i64::from(num))?
        .checked_add(i64::from(den / 2))?)
    .checked_div(i64::from(den))?;
    if !(1..=8_388_607).contains(&width) {
        return None;
    }
    i32::try_from(width).ok()
}

pub fn recompute_line_width_sp_v0(line: &LinePlanV0) -> Option<u32> {
    let mut total = 0u32;
    for glyph in &line.glyphs {
        if glyph.advance_sp < 0 || glyph.advance_sp > 8_388_607 {
            return None;
        }
        if glyph.advance_sp == 0 {
            if !is_style_marker_byte_v0(glyph.byte) {
                return None;
            }
            continue;
        }
        total = total.checked_add(u32::try_from(glyph.advance_sp).ok()?)?;
    }
    Some(total)
}

fn split_pages_v0(text: &[u8]) -> Option<Vec<&[u8]>> {
    if text.iter().any(|byte| {
        !is_supported_text_byte_v0(*byte)
            && *byte != PAGEBREAK_MARKER_V0
            && *byte != NEWLINE_MARKER_V0
    }) {
        return None;
    }
    let mut pages = Vec::<&[u8]>::new();
    let mut start = 0usize;
    for (index, byte) in text.iter().enumerate() {
        if *byte == PAGEBREAK_MARKER_V0 {
            pages.push(&text[start..index]);
            start = index + 1;
        }
    }
    pages.push(&text[start..]);
    Some(pages)
}

fn split_lines_v0(page: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::<&[u8]>::new();
    let mut start = 0usize;
    for (index, byte) in page.iter().enumerate() {
        if *byte == NEWLINE_MARKER_V0 {
            lines.push(&page[start..index]);
            start = index + 1;
        }
    }
    lines.push(&page[start..]);
    lines
}

fn wrap_logical_line_v0(line: &[u8], max_line_glyphs: usize) -> Option<Vec<Vec<u8>>> {
    if max_line_glyphs == 0 {
        return None;
    }
    if line.is_empty() {
        return Some(vec![Vec::new()]);
    }
    let mut wrapped = Vec::<Vec<u8>>::new();
    let mut start = 0usize;
    while start < line.len() {
        if line.len() - start <= max_line_glyphs {
            wrapped.push(line[start..].to_vec());
            break;
        }
        let limit = start + max_line_glyphs;
        let mut break_at = None::<usize>;
        for index in (start..limit).rev() {
            if line[index] == b' ' {
                break_at = Some(index);
                break;
            }
        }
        if let Some(space_index) = break_at {
            if space_index > start {
                wrapped.push(line[start..space_index].to_vec());
            } else {
                wrapped.push(Vec::new());
            }
            start = space_index + 1;
            while start < line.len() && line[start] == b' ' {
                start += 1;
            }
        } else {
            wrapped.push(line[start..limit].to_vec());
            start = limit;
        }
    }
    Some(wrapped)
}

const WRAP_BALANCE_REMAINDER_CHARS_MAX_V12: usize = 12;
const WRAP_BALANCE_MIN_FILL_NUM_V12: u32 = 11;
const WRAP_BALANCE_MIN_FILL_DEN_V12: u32 = 20;
const INLINE_MATH_PLACEHOLDER_TOKEN_V15: &[u8] = b"MATH";

fn is_wrap_unfriendly_leading_byte_v12(byte: u8) -> bool {
    matches!(
        byte,
        b'.' | b',' | b';' | b':' | b'!' | b'?' | b')' | b']' | b'}'
    )
}

fn is_token_delimiter_byte_v15(byte: u8) -> bool {
    byte == b' '
        || matches!(
            byte,
            b'.'
                | b','
                | b';'
                | b':'
                | b'!'
                | b'?'
                | b'('
                | b')'
                | b'['
                | b']'
                | b'{'
                | b'}'
        )
}

fn next_visible_byte_after_v12(line: &[u8], start: usize) -> Option<u8> {
    let mut cursor = start;
    while cursor < line.len() {
        let byte = line[cursor];
        if byte == b' ' || is_style_marker_byte_v0(byte) {
            cursor += 1;
            continue;
        }
        return Some(byte);
    }
    None
}

fn remaining_visible_count_v12(line: &[u8], start: usize) -> usize {
    line[start..]
        .iter()
        .copied()
        .filter(|byte| *byte != b' ' && !is_style_marker_byte_v0(*byte))
        .count()
}

fn next_visible_token_is_inline_math_v15(line: &[u8], start: usize) -> bool {
    let mut cursor = start;
    while cursor < line.len() {
        let byte = line[cursor];
        if is_style_marker_byte_v0(byte) || is_token_delimiter_byte_v15(byte) {
            cursor += 1;
            continue;
        }
        break;
    }
    if cursor >= line.len() {
        return false;
    }
    let mut token = Vec::<u8>::new();
    while cursor < line.len() {
        let byte = line[cursor];
        if is_token_delimiter_byte_v15(byte) {
            break;
        }
        if !is_style_marker_byte_v0(byte) {
            token.push(byte);
            if token.len() > INLINE_MATH_PLACEHOLDER_TOKEN_V15.len() {
                return false;
            }
        }
        cursor += 1;
    }
    token == INLINE_MATH_PLACEHOLDER_TOKEN_V15
}

fn previous_visible_token_is_inline_math_v15(line: &[u8], end_exclusive: usize) -> bool {
    if end_exclusive == 0 {
        return false;
    }
    let mut cursor = end_exclusive;
    while cursor > 0 {
        let byte = line[cursor - 1];
        if is_style_marker_byte_v0(byte) || is_token_delimiter_byte_v15(byte) {
            cursor -= 1;
            continue;
        }
        break;
    }
    if cursor == 0 {
        return false;
    }
    let token_end = cursor;
    while cursor > 0 {
        let byte = line[cursor - 1];
        if is_token_delimiter_byte_v15(byte) {
            break;
        }
        cursor -= 1;
    }
    let mut token = Vec::<u8>::new();
    for byte in line[cursor..token_end].iter().copied() {
        if is_style_marker_byte_v0(byte) {
            continue;
        }
        token.push(byte);
        if token.len() > INLINE_MATH_PLACEHOLDER_TOKEN_V15.len() {
            return false;
        }
    }
    token == INLINE_MATH_PLACEHOLDER_TOKEN_V15
}

fn wrap_logical_line_by_width_v0(
    line: &[u8],
    glyph_advance_sp: i32,
    max_line_width_sp: u32,
) -> Option<Vec<Vec<u8>>> {
    if max_line_width_sp == 0 {
        return None;
    }
    if line.is_empty() {
        return Some(vec![Vec::new()]);
    }
    let mut wrapped = Vec::<Vec<u8>>::new();
    let mut start = 0usize;
    while start < line.len() {
        let mut cursor = start;
        let mut width = 0u32;
        let mut space_candidates = Vec::<(usize, u32)>::new();
        while cursor < line.len() {
            let advance_sp =
                u32::try_from(glyph_width_sp_v0(line[cursor], glyph_advance_sp)?).ok()?;
            if width.checked_add(advance_sp)? > max_line_width_sp {
                break;
            }
            width = width.checked_add(advance_sp)?;
            if line[cursor] == b' ' {
                space_candidates.push((cursor, width.saturating_sub(advance_sp)));
            }
            cursor += 1;
        }

        if cursor == line.len() {
            wrapped.push(line[start..].to_vec());
            break;
        }

        if cursor == start {
            let hard_break = start.checked_add(1)?;
            wrapped.push(line[start..hard_break].to_vec());
            start = hard_break;
            continue;
        }

        if !space_candidates.is_empty() {
            let mut chosen_space_candidate = space_candidates.len() - 1;
            while chosen_space_candidate > 0 {
                let (candidate_space_index, candidate_width_sp) =
                    space_candidates[chosen_space_candidate];
                let next_line_start = candidate_space_index + 1;
                let short_remainder = remaining_visible_count_v12(line, next_line_start)
                    <= WRAP_BALANCE_REMAINDER_CHARS_MAX_V12;
                let punctuation_leading_remainder =
                    next_visible_byte_after_v12(line, next_line_start)
                        .map(is_wrap_unfriendly_leading_byte_v12)
                        .unwrap_or(false);
                let inline_math_leading_remainder =
                    next_visible_token_is_inline_math_v15(line, next_line_start);
                let inline_math_trailing_token =
                    previous_visible_token_is_inline_math_v15(line, candidate_space_index);
                let min_fill_sp = max_line_width_sp
                    .checked_mul(WRAP_BALANCE_MIN_FILL_NUM_V12)?
                    .checked_div(WRAP_BALANCE_MIN_FILL_DEN_V12)?;
                let should_backtrack_for_inline_math =
                    inline_math_leading_remainder || inline_math_trailing_token;
                let should_backtrack_for_short_or_punctuation = candidate_width_sp >= min_fill_sp
                    && (short_remainder || punctuation_leading_remainder);
                if should_backtrack_for_short_or_punctuation || should_backtrack_for_inline_math {
                    chosen_space_candidate -= 1;
                    continue;
                }
                break;
            }
            let (space_index, _) = space_candidates[chosen_space_candidate];
            if space_index > start {
                wrapped.push(line[start..space_index].to_vec());
            } else {
                wrapped.push(Vec::new());
            }
            start = space_index + 1;
            while start < line.len() && line[start] == b' ' {
                start += 1;
            }
        } else {
            wrapped.push(line[start..cursor].to_vec());
            start = cursor;
        }
    }
    Some(wrapped)
}

fn build_line_plan_v0(line: &[u8], glyph_advance_sp: i32) -> Option<LinePlanV0> {
    let mut glyphs = Vec::<GlyphPlanV0>::new();
    for byte in line {
        if !is_supported_text_byte_v0(*byte) {
            return None;
        }
        let advance_sp = glyph_width_sp_v0(*byte, glyph_advance_sp)?;
        glyphs.push(GlyphPlanV0 {
            byte: *byte,
            advance_sp,
        });
    }
    let mut line_plan = LinePlanV0 {
        glyphs,
        width_sp: 0,
    };
    line_plan.width_sp = recompute_line_width_sp_v0(&line_plan)?;
    Some(line_plan)
}

pub fn plan_layout_v0(
    text: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
    max_line_glyphs: usize,
    max_lines_per_page: usize,
) -> Option<LayoutPlanV0> {
    if glyph_advance_sp <= 0
        || line_advance_sp <= 0
        || max_line_glyphs == 0
        || max_lines_per_page == 0
    {
        return None;
    }

    let forced_pages = split_pages_v0(text)?;
    let mut pages = Vec::<PagePlanV0>::new();
    for forced_page in forced_pages {
        let logical_lines = split_lines_v0(forced_page);
        let mut physical_lines = Vec::<LinePlanV0>::new();
        for logical_line in logical_lines {
            let wrapped = wrap_logical_line_v0(logical_line, max_line_glyphs)?;
            for wrapped_line in wrapped {
                let line_plan = build_line_plan_v0(&wrapped_line, glyph_advance_sp)?;
                physical_lines.push(line_plan);
            }
        }
        for chunk in physical_lines.chunks(max_lines_per_page) {
            pages.push(PagePlanV0 {
                lines: chunk.to_vec(),
            });
        }
    }
    if pages.is_empty() {
        return None;
    }
    Some(LayoutPlanV0 { pages })
}

pub fn plan_layout_width_v0(
    text: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
    max_line_width_sp: u32,
    max_lines_per_page: usize,
) -> Option<LayoutPlanV0> {
    if glyph_advance_sp <= 0
        || line_advance_sp <= 0
        || max_line_width_sp == 0
        || max_lines_per_page == 0
    {
        return None;
    }

    let forced_pages = split_pages_v0(text)?;
    let mut pages = Vec::<PagePlanV0>::new();
    for forced_page in forced_pages {
        let logical_lines = split_lines_v0(forced_page);
        let mut physical_lines = Vec::<LinePlanV0>::new();
        for logical_line in logical_lines {
            let wrapped =
                wrap_logical_line_by_width_v0(logical_line, glyph_advance_sp, max_line_width_sp)?;
            for wrapped_line in wrapped {
                let line_plan = build_line_plan_v0(&wrapped_line, glyph_advance_sp)?;
                physical_lines.push(line_plan);
            }
        }
        for chunk in physical_lines.chunks(max_lines_per_page) {
            pages.push(PagePlanV0 {
                lines: chunk.to_vec(),
            });
        }
    }
    if pages.is_empty() {
        return None;
    }
    Some(LayoutPlanV0 { pages })
}
