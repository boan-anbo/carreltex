const PAGEBREAK_MARKER_V0: u8 = 0x0c;
const NEWLINE_MARKER_V0: u8 = 0x0a;

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
        if glyph.advance_sp <= 0 || glyph.advance_sp > 8_388_607 {
            return None;
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
        let mut last_space = None::<usize>;
        while cursor < line.len() {
            let advance_sp = u32::try_from(glyph_width_sp_v0(line[cursor], glyph_advance_sp)?).ok()?;
            if width.checked_add(advance_sp)? > max_line_width_sp {
                break;
            }
            width = width.checked_add(advance_sp)?;
            if line[cursor] == b' ' {
                last_space = Some(cursor);
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

        if let Some(space_index) = last_space {
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
    let mut line_plan = LinePlanV0 { glyphs, width_sp: 0 };
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
