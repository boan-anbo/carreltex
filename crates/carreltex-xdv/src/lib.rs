const DVI_PRE: u8 = 247;
const DVI_BOP: u8 = 139;
const DVI_EOP: u8 = 140;
const DVI_POST: u8 = 248;
const DVI_POSTPOST: u8 = 249;
const DVI_FNT_DEF1: u8 = 243;
const DVI_FNT_NUM_0: u8 = 171;
const DVI_RIGHT3: u8 = 145;
const DVI_DOWN3: u8 = 160;
const DVI_ID_V2: u8 = 2;
const DVI_TRAILER_BYTE: u8 = 223;
const DVI_NUM: u32 = 25_400_000;
const DVI_DEN: u32 = 473_628_672;
const DVI_MAG: u32 = 1000;
const FONT_ID_V0: u8 = 0;
const FONT_NAME_V0: &[u8] = b"carreltex-v0";
pub const DEFAULT_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub const DEFAULT_LINE_ADVANCE_SP_V0: i32 = 786_432;
pub const DEFAULT_MAX_LINE_GLYPHS_V0: usize = 80;
pub const DEFAULT_MAX_LINES_PER_PAGE_V0: usize = 200;

mod layout_v0;
mod pdf_v0;
pub use layout_v0::{
    plan_layout_v0, plan_layout_width_v0, recompute_line_width_sp_v0, GlyphPlanV0, LayoutPlanV0,
    LinePlanV0,
    PagePlanV0,
};
pub use pdf_v0::render_dvi_v2_text_page_to_pdf_v0;
use layout_v0::glyph_width_sp_v0;

fn push_u32_be(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn push_i32_be(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn push_i24_be(out: &mut Vec<u8>, value: i32) -> Option<()> {
    if !(-8_388_608..=8_388_607).contains(&value) {
        return None;
    }
    out.push(((value >> 16) & 0xff) as u8);
    out.push(((value >> 8) & 0xff) as u8);
    out.push((value & 0xff) as u8);
    Some(())
}

fn read_u8(bytes: &[u8], index: &mut usize) -> Option<u8> {
    let value = *bytes.get(*index)?;
    *index += 1;
    Some(value)
}

fn read_u16_be(bytes: &[u8], index: &mut usize) -> Option<u16> {
    let end = index.checked_add(2)?;
    let slice = bytes.get(*index..end)?;
    *index = end;
    Some(u16::from_be_bytes([slice[0], slice[1]]))
}

fn read_u32_be(bytes: &[u8], index: &mut usize) -> Option<u32> {
    let end = index.checked_add(4)?;
    let slice = bytes.get(*index..end)?;
    *index = end;
    Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_i32_be(bytes: &[u8], index: &mut usize) -> Option<i32> {
    let end = index.checked_add(4)?;
    let slice = bytes.get(*index..end)?;
    *index = end;
    Some(i32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_i24_be(bytes: &[u8], index: &mut usize) -> Option<i32> {
    let end = index.checked_add(3)?;
    let slice = bytes.get(*index..end)?;
    *index = end;
    let raw = ((slice[0] as i32) << 16) | ((slice[1] as i32) << 8) | (slice[2] as i32);
    if (raw & 0x80_0000) != 0 {
        Some(raw | !0x00ff_ffff)
    } else {
        Some(raw)
    }
}

fn is_supported_text_byte_v0(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

fn append_font_def_v0(out: &mut Vec<u8>) {
    out.push(DVI_FNT_DEF1);
    out.push(FONT_ID_V0);
    push_u32_be(out, 0);
    push_u32_be(out, 0);
    push_u32_be(out, 0);
    out.push(0);
    out.push(FONT_NAME_V0.len() as u8);
    out.extend_from_slice(FONT_NAME_V0);
}

fn read_and_validate_font_def_v0(bytes: &[u8], index: &mut usize) -> Option<()> {
    if read_u8(bytes, index)? != DVI_FNT_DEF1 {
        return None;
    }
    if read_u8(bytes, index)? != FONT_ID_V0 {
        return None;
    }
    if read_u32_be(bytes, index)? != 0 {
        return None;
    }
    if read_u32_be(bytes, index)? != 0 {
        return None;
    }
    if read_u32_be(bytes, index)? != 0 {
        return None;
    }
    if read_u8(bytes, index)? != 0 {
        return None;
    }
    if read_u8(bytes, index)? != FONT_NAME_V0.len() as u8 {
        return None;
    }
    let end = index.checked_add(FONT_NAME_V0.len())?;
    let name = bytes.get(*index..end)?;
    if name != FONT_NAME_V0 {
        return None;
    }
    *index = end;
    Some(())
}

fn append_trailer(out: &mut Vec<u8>) {
    let trailer_padding = match out.len() % 4 {
        0 => 4,
        remainder => 8 - remainder,
    };
    for _ in 0..trailer_padding {
        out.push(DVI_TRAILER_BYTE);
    }
}

pub fn write_dvi_v2_empty_page_v0() -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    out.push(DVI_PRE);
    out.push(DVI_ID_V2);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    out.push(0);

    let bop_offset = out.len() as u32;
    out.push(DVI_BOP);
    for _ in 0..10 {
        push_i32_be(&mut out, 0);
    }
    push_i32_be(&mut out, -1);
    out.push(DVI_EOP);

    let post_offset = out.len() as u32;
    out.push(DVI_POST);
    push_u32_be(&mut out, bop_offset);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    push_u32_be(&mut out, 0);
    push_u32_be(&mut out, 0);
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());

    out.push(DVI_POSTPOST);
    push_u32_be(&mut out, post_offset);
    out.push(DVI_ID_V2);
    append_trailer(&mut out);
    out
}

pub fn write_dvi_v2_text_page_with_layout_v0(
    text: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
) -> Option<Vec<u8>> {
    write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(
        text,
        glyph_advance_sp,
        line_advance_sp,
        DEFAULT_MAX_LINE_GLYPHS_V0,
        DEFAULT_MAX_LINES_PER_PAGE_V0,
    )
}

pub fn write_dvi_v2_text_page_with_layout_and_wrap_v0(
    text: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
    max_line_glyphs: usize,
) -> Option<Vec<u8>> {
    write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(
        text,
        glyph_advance_sp,
        line_advance_sp,
        max_line_glyphs,
        DEFAULT_MAX_LINES_PER_PAGE_V0,
    )
}

pub fn write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(
    text: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
    max_line_glyphs: usize,
    max_lines_per_page: usize,
) -> Option<Vec<u8>> {
    let layout = plan_layout_v0(
        text,
        glyph_advance_sp,
        line_advance_sp,
        max_line_glyphs,
        max_lines_per_page,
    )?;
    write_dvi_v2_text_page_from_layout_v0(&layout, line_advance_sp)
}

fn emit_line_plan_v0(out: &mut Vec<u8>, line: &LinePlanV0) -> Option<u32> {
    let expected_width = recompute_line_width_sp_v0(line)?;
    if expected_width != line.width_sp {
        return None;
    }
    let mut emitted_width = 0u32;
    for glyph in &line.glyphs {
        if !is_supported_text_byte_v0(glyph.byte) || glyph.advance_sp <= 0 {
            return None;
        }
        out.push(glyph.byte);
        out.push(DVI_RIGHT3);
        push_i24_be(out, glyph.advance_sp)?;
        emitted_width = emitted_width.checked_add(u32::try_from(glyph.advance_sp).ok()?)?;
    }
    if emitted_width != line.width_sp {
        return None;
    }
    Some(emitted_width)
}

fn emit_reset_back_v0(out: &mut Vec<u8>, mut reset_back: u32) -> Option<()> {
    const MAX_RIGHT3_STEP_V0: u32 = 8_388_607;
    while reset_back > 0 {
        let step = reset_back.min(MAX_RIGHT3_STEP_V0);
        out.push(DVI_RIGHT3);
        push_i24_be(out, -i32::try_from(step).ok()?)?;
        reset_back -= step;
    }
    Some(())
}

pub fn write_dvi_v2_text_page_from_layout_v0(
    layout: &LayoutPlanV0,
    line_advance_sp: i32,
) -> Option<Vec<u8>> {
    if line_advance_sp <= 0 || layout.pages.is_empty() {
        return None;
    }
    let mut out = Vec::<u8>::new();
    out.push(DVI_PRE);
    out.push(DVI_ID_V2);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    out.push(0);

    let mut bop_offsets = Vec::<u32>::new();
    let mut max_h = 0u32;
    let mut max_v = 0u32;
    for page in &layout.pages {
        if page.lines.is_empty() {
            return None;
        }
        let bop_offset = out.len() as u32;
        out.push(DVI_BOP);
        for _ in 0..10 {
            push_i32_be(&mut out, 0);
        }
        let prev_bop = if let Some(previous) = bop_offsets.last() {
            i32::try_from(*previous).ok()?
        } else {
            -1
        };
        push_i32_be(&mut out, prev_bop);
        append_font_def_v0(&mut out);
        out.push(DVI_FNT_NUM_0);

        let mut page_h = 0u32;
        let mut page_v = 0u32;
        let mut previous_line_h = emit_line_plan_v0(&mut out, &page.lines[0])?;
        page_h = page_h.max(previous_line_h);
        for line in page.lines.iter().skip(1) {
            if previous_line_h > 0 {
                emit_reset_back_v0(&mut out, previous_line_h)?;
            }
            out.push(DVI_DOWN3);
            push_i24_be(&mut out, line_advance_sp)?;
            page_v = page_v.checked_add(u32::try_from(line_advance_sp).ok()?)?;
            previous_line_h = emit_line_plan_v0(&mut out, line)?;
            page_h = page_h.max(previous_line_h);
        }
        max_h = max_h.max(page_h);
        max_v = max_v.max(page_v);
        out.push(DVI_EOP);
        bop_offsets.push(bop_offset);
    }
    let page_count = u16::try_from(bop_offsets.len()).ok()?;
    if page_count == 0 {
        return None;
    }

    let post_offset = out.len() as u32;
    out.push(DVI_POST);
    push_u32_be(&mut out, *bop_offsets.last()?);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    push_u32_be(&mut out, max_h);
    push_u32_be(&mut out, max_v);
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&page_count.to_be_bytes());

    out.push(DVI_POSTPOST);
    push_u32_be(&mut out, post_offset);
    out.push(DVI_ID_V2);
    append_trailer(&mut out);
    Some(out)
}

pub fn write_dvi_v2_text_page_with_advance_v0(
    text: &[u8],
    glyph_advance_sp: i32,
) -> Option<Vec<u8>> {
    write_dvi_v2_text_page_with_layout_v0(text, glyph_advance_sp, DEFAULT_LINE_ADVANCE_SP_V0)
}

pub fn write_dvi_v2_text_page_v0(text: &[u8]) -> Option<Vec<u8>> {
    write_dvi_v2_text_page_with_layout_v0(
        text,
        DEFAULT_GLYPH_ADVANCE_SP_V0,
        DEFAULT_LINE_ADVANCE_SP_V0,
    )
}

pub fn validate_dvi_v2_empty_page_v0(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return false;
    }

    let mut index = 0usize;
    if read_u8(bytes, &mut index) != Some(DVI_PRE) {
        return false;
    }
    if read_u8(bytes, &mut index) != Some(DVI_ID_V2) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_NUM) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_DEN) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_MAG) {
        return false;
    }
    if read_u8(bytes, &mut index) != Some(0) {
        return false;
    }

    let bop_offset = index;
    if read_u8(bytes, &mut index) != Some(DVI_BOP) {
        return false;
    }
    for _ in 0..10 {
        if read_i32_be(bytes, &mut index) != Some(0) {
            return false;
        }
    }
    if read_i32_be(bytes, &mut index) != Some(-1) {
        return false;
    }
    if read_u8(bytes, &mut index) != Some(DVI_EOP) {
        return false;
    }

    let post_offset = index;
    if read_u8(bytes, &mut index) != Some(DVI_POST) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(bop_offset as u32) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_NUM) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_DEN) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_MAG) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(0) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(0) {
        return false;
    }
    if read_u16_be(bytes, &mut index) != Some(0) {
        return false;
    }
    if read_u16_be(bytes, &mut index) != Some(1) {
        return false;
    }

    if read_u8(bytes, &mut index) != Some(DVI_POSTPOST) {
        return false;
    }
    if read_u32_be(bytes, &mut index) != Some(post_offset as u32) {
        return false;
    }
    if read_u8(bytes, &mut index) != Some(DVI_ID_V2) {
        return false;
    }
    let trailer_len = bytes.len().saturating_sub(index);
    if trailer_len < 4 {
        return false;
    }
    if !bytes[index..].iter().all(|byte| *byte == DVI_TRAILER_BYTE) {
        return false;
    }
    true
}

struct ParsedDviTextLayoutV0 {
    layout: LayoutPlanV0,
    right3_count: u32,
    down3_count: u32,
    page_count: u16,
}

fn parse_dvi_v2_text_page_internal_v0(
    bytes: &[u8],
    line_advance_sp: i32,
) -> Option<ParsedDviTextLayoutV0> {
    if line_advance_sp <= 0 || bytes.is_empty() || bytes.len() % 4 != 0 {
        return None;
    }

    let mut index = 0usize;
    if read_u8(bytes, &mut index) != Some(DVI_PRE) {
        return None;
    }
    if read_u8(bytes, &mut index) != Some(DVI_ID_V2) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_NUM) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_DEN) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_MAG) {
        return None;
    }
    if read_u8(bytes, &mut index) != Some(0) {
        return None;
    }

    let mut right3_count = 0u32;
    let mut down3_count = 0u32;
    let mut page_count = 0u16;
    let mut previous_bop_offset: Option<usize> = None;
    let mut last_bop_offset = 0u32;
    let mut max_h = 0u32;
    let mut max_v = 0u32;
    let mut pages = Vec::<PagePlanV0>::new();

    loop {
        let opcode = *bytes.get(index)?;
        if opcode == DVI_POST {
            break;
        }
        if opcode != DVI_BOP {
            return None;
        }
        let bop_offset = index;
        last_bop_offset = bop_offset as u32;
        index += 1;
        for _ in 0..10 {
            if read_i32_be(bytes, &mut index) != Some(0) {
                return None;
            }
        }
        let expected_prev = if let Some(previous) = previous_bop_offset {
            i32::try_from(previous).ok()?
        } else {
            -1
        };
        if read_i32_be(bytes, &mut index) != Some(expected_prev) {
            return None;
        }
        read_and_validate_font_def_v0(bytes, &mut index)?;
        if read_u8(bytes, &mut index) != Some(DVI_FNT_NUM_0) {
            return None;
        }

        let mut page_lines = Vec::<LinePlanV0>::new();
        let mut current_glyphs = Vec::<GlyphPlanV0>::new();
        let mut current_line_width = 0u32;
        let mut page_h = 0u32;
        let mut page_h_max = 0u32;
        let mut page_v = 0u32;
        let mut expect_width_right_after_char = false;
        let mut expect_down3_after_reset = false;
        let mut pending_byte = 0u8;

        while let Some(op) = bytes.get(index).copied() {
            if op == DVI_EOP {
                if expect_down3_after_reset || expect_width_right_after_char {
                    return None;
                }
                let line = LinePlanV0 {
                    glyphs: current_glyphs,
                    width_sp: current_line_width,
                };
                if recompute_line_width_sp_v0(&line)? != line.width_sp {
                    return None;
                }
                page_h_max = page_h_max.max(line.width_sp);
                page_lines.push(line);
                if page_lines.is_empty() {
                    return None;
                }
                index += 1;
                break;
            }
            if expect_width_right_after_char {
                if op != DVI_RIGHT3 {
                    return None;
                }
                right3_count = right3_count.checked_add(1)?;
                index += 1;
                let amount = read_i24_be(bytes, &mut index)?;
                if amount <= 0 {
                    return None;
                }
                let amount_u32 = u32::try_from(amount).ok()?;
                current_line_width = current_line_width.checked_add(amount_u32)?;
                page_h = page_h.checked_add(amount_u32)?;
                current_glyphs.push(GlyphPlanV0 {
                    byte: pending_byte,
                    advance_sp: amount,
                });
                expect_width_right_after_char = false;
                continue;
            }

            if op == DVI_RIGHT3 {
                right3_count = right3_count.checked_add(1)?;
                index += 1;
                let amount = read_i24_be(bytes, &mut index)?;
                if amount >= 0 {
                    return None;
                }
                let back = u32::try_from(-amount).ok()?;
                if back == 0 || back > page_h {
                    return None;
                }
                page_h -= back;
                expect_down3_after_reset = true;
                continue;
            }

            if op == DVI_DOWN3 {
                down3_count = down3_count.checked_add(1)?;
                index += 1;
                if read_i24_be(bytes, &mut index)? != line_advance_sp {
                    return None;
                }
                if page_h != 0 {
                    return None;
                }
                if expect_down3_after_reset {
                    expect_down3_after_reset = false;
                }
                page_v = page_v.checked_add(u32::try_from(line_advance_sp).ok()?)?;
                let line = LinePlanV0 {
                    glyphs: std::mem::take(&mut current_glyphs),
                    width_sp: current_line_width,
                };
                if recompute_line_width_sp_v0(&line)? != line.width_sp {
                    return None;
                }
                page_h_max = page_h_max.max(line.width_sp);
                page_lines.push(line);
                current_line_width = 0;
                continue;
            }

            if op > 127 || !is_supported_text_byte_v0(op) || expect_down3_after_reset {
                return None;
            }
            pending_byte = op;
            index += 1;
            expect_width_right_after_char = true;
        }

        if page_lines.is_empty() {
            return None;
        }
        pages.push(PagePlanV0 { lines: page_lines });
        if page_h_max > max_h {
            max_h = page_h_max;
        }
        if page_v > max_v {
            max_v = page_v;
        }
        previous_bop_offset = Some(bop_offset);
        page_count = page_count.checked_add(1)?;
    }

    if page_count == 0 {
        return None;
    }

    let post_offset = index;
    if read_u8(bytes, &mut index) != Some(DVI_POST) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(last_bop_offset) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_NUM) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_DEN) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(DVI_MAG) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(max_h) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(max_v) {
        return None;
    }
    if read_u16_be(bytes, &mut index) != Some(0) {
        return None;
    }
    if read_u16_be(bytes, &mut index) != Some(page_count) {
        return None;
    }
    if read_u8(bytes, &mut index) != Some(DVI_POSTPOST) {
        return None;
    }
    if read_u32_be(bytes, &mut index) != Some(post_offset as u32) {
        return None;
    }
    if read_u8(bytes, &mut index) != Some(DVI_ID_V2) {
        return None;
    }
    let trailer_len = bytes.len().saturating_sub(index);
    if trailer_len < 4 {
        return None;
    }
    if !bytes[index..].iter().all(|byte| *byte == DVI_TRAILER_BYTE) {
        return None;
    }

    Some(ParsedDviTextLayoutV0 {
        layout: LayoutPlanV0 { pages },
        right3_count,
        down3_count,
        page_count,
    })
}

pub fn parse_dvi_v2_text_page_to_layout_v0(bytes: &[u8], line_advance_sp: i32) -> Option<LayoutPlanV0> {
    parse_dvi_v2_text_page_internal_v0(bytes, line_advance_sp).map(|parsed| parsed.layout)
}

pub fn validate_dvi_v2_text_page_matches_layout_v0(
    bytes: &[u8],
    layout: &LayoutPlanV0,
    line_advance_sp: i32,
) -> bool {
    parse_dvi_v2_text_page_to_layout_v0(bytes, line_advance_sp)
        .map(|parsed_layout| parsed_layout == *layout)
        .unwrap_or(false)
}

pub fn count_dvi_v2_text_pages_with_layout_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
) -> Option<u16> {
    count_dvi_v2_text_movements_with_layout_v0(bytes, glyph_advance_sp, line_advance_sp)
        .map(|(_, _, _, _, page_count)| page_count)
}

pub fn count_dvi_v2_text_movements_with_layout_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
) -> Option<(u32, u32, u32, u32, u16)> {
    if glyph_advance_sp <= 0 {
        return None;
    }
    let parsed = parse_dvi_v2_text_page_internal_v0(bytes, line_advance_sp)?;
    for page in &parsed.layout.pages {
        for line in &page.lines {
            if recompute_line_width_sp_v0(line)? != line.width_sp {
                return None;
            }
            for glyph in &line.glyphs {
                if glyph.advance_sp != glyph_width_sp_v0(glyph.byte, glyph_advance_sp)? {
                    return None;
                }
            }
        }
    }
    Some((parsed.right3_count, 0, 0, parsed.down3_count, parsed.page_count))
}

pub fn sum_dvi_v2_positive_right3_amounts_with_layout_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
) -> Option<u32> {
    count_dvi_v2_text_movements_with_layout_v0(bytes, glyph_advance_sp, line_advance_sp)?;
    let mut index = 0usize;
    let mut total = 0u32;
    while index < bytes.len() {
        if bytes[index] == DVI_RIGHT3 {
            index += 1;
            let amount = read_i24_be(bytes, &mut index)?;
            if amount > 0 {
                total = total.checked_add(u32::try_from(amount).ok()?)?;
            }
        } else {
            index += 1;
        }
    }
    Some(total)
}

pub fn count_dvi_v2_text_pages_with_advance_v0(bytes: &[u8], glyph_advance_sp: i32) -> Option<u16> {
    count_dvi_v2_text_pages_with_layout_v0(bytes, glyph_advance_sp, DEFAULT_LINE_ADVANCE_SP_V0)
}

pub fn count_dvi_v2_text_movements_with_advance_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
) -> Option<(u32, u32, u32, u32, u16)> {
    count_dvi_v2_text_movements_with_layout_v0(bytes, glyph_advance_sp, DEFAULT_LINE_ADVANCE_SP_V0)
}

pub fn count_dvi_v2_text_movements_v0(bytes: &[u8]) -> Option<(u32, u32, u32, u32, u16)> {
    count_dvi_v2_text_movements_with_advance_v0(bytes, DEFAULT_GLYPH_ADVANCE_SP_V0)
}

pub fn count_dvi_v2_text_pages_v0(bytes: &[u8]) -> Option<u16> {
    count_dvi_v2_text_pages_with_advance_v0(bytes, DEFAULT_GLYPH_ADVANCE_SP_V0)
}

pub fn validate_dvi_v2_text_page_with_layout_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
    line_advance_sp: i32,
) -> bool {
    count_dvi_v2_text_movements_with_layout_v0(bytes, glyph_advance_sp, line_advance_sp).is_some()
}

pub fn validate_dvi_v2_text_page_with_advance_v0(bytes: &[u8], glyph_advance_sp: i32) -> bool {
    validate_dvi_v2_text_page_with_layout_v0(bytes, glyph_advance_sp, DEFAULT_LINE_ADVANCE_SP_V0)
}

pub fn validate_dvi_v2_text_page_v0(bytes: &[u8]) -> bool {
    validate_dvi_v2_text_page_with_layout_v0(
        bytes,
        DEFAULT_GLYPH_ADVANCE_SP_V0,
        DEFAULT_LINE_ADVANCE_SP_V0,
    )
}

#[cfg(test)]
mod tests;
