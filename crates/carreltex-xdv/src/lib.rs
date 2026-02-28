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
const PAGEBREAK_MARKER_V0: u8 = 0x0c;
const NEWLINE_MARKER_V0: u8 = 0x0a;
pub const DEFAULT_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub const DEFAULT_LINE_ADVANCE_SP_V0: i32 = 786_432;
pub const DEFAULT_MAX_LINE_GLYPHS_V0: usize = 80;
pub const DEFAULT_MAX_LINES_PER_PAGE_V0: usize = 200;

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

fn glyph_width_sp_v0(byte: u8, glyph_advance_sp: i32) -> Option<i32> {
    if glyph_advance_sp <= 0 {
        return None;
    }
    let half_em = glyph_advance_sp / 2;
    let one_and_half_em = glyph_advance_sp.checked_add(half_em)?;
    let width = match byte {
        b' ' | b'.' | b'i' => half_em,
        b'm' | b'W' => one_and_half_em,
        _ => glyph_advance_sp,
    };
    if !(1..=8_388_607).contains(&width) {
        return None;
    }
    Some(width)
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

fn emit_line_glyphs_v0(out: &mut Vec<u8>, line: &[u8], glyph_advance_sp: i32) -> Option<u32> {
    let mut line_h = 0u32;
    for byte in line {
        out.push(*byte);
        let glyph_width = glyph_width_sp_v0(*byte, glyph_advance_sp)?;
        out.push(DVI_RIGHT3);
        push_i24_be(out, glyph_width)?;
        line_h = line_h.checked_add(u32::try_from(glyph_width).ok()?)?;
    }
    Some(line_h)
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
    if glyph_advance_sp <= 0
        || line_advance_sp <= 0
        || max_line_glyphs == 0
        || max_lines_per_page == 0
    {
        return None;
    }
    let forced_pages = split_pages_v0(text)?;

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
    for forced_page in forced_pages {
        let logical_lines = split_lines_v0(forced_page);
        let mut physical_lines = Vec::<Vec<u8>>::new();
        for line in logical_lines {
            let wrapped = wrap_logical_line_v0(line, max_line_glyphs)?;
            physical_lines.extend(wrapped);
        }
        for chunk in physical_lines.chunks(max_lines_per_page) {
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
            let mut previous_line_h = emit_line_glyphs_v0(
                &mut out,
                chunk.first().map(|line| line.as_slice()).unwrap_or(&[]),
                glyph_advance_sp,
            )?;
            page_h = page_h.max(previous_line_h);
            for line in chunk.iter().skip(1) {
                if previous_line_h > 0 {
                    out.push(DVI_RIGHT3);
                    let reset_back = -i32::try_from(previous_line_h).ok()?;
                    push_i24_be(&mut out, reset_back)?;
                }
                out.push(DVI_DOWN3);
                push_i24_be(&mut out, line_advance_sp)?;
                page_v = page_v.checked_add(u32::try_from(line_advance_sp).ok()?)?;
                previous_line_h = emit_line_glyphs_v0(&mut out, line.as_slice(), glyph_advance_sp)?;
                page_h = page_h.max(previous_line_h);
            }
            max_h = max_h.max(page_h);
            max_v = max_v.max(page_v);
            out.push(DVI_EOP);
            bop_offsets.push(bop_offset);
        }
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
    if glyph_advance_sp <= 0 || line_advance_sp <= 0 {
        return None;
    }
    if bytes.is_empty() || bytes.len() % 4 != 0 {
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
    let w3_count = 0u32;
    let w0_count = 0u32;
    let mut down3_count = 0u32;
    let mut page_count = 0u16;
    let mut previous_bop_offset: Option<usize> = None;
    let mut last_bop_offset = 0u32;
    let mut max_h = 0u32;
    let mut max_v = 0u32;
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
        let mut page_h = 0u32;
        let mut page_h_max = 0u32;
        let mut page_v = 0u32;
        let mut expect_width_right_after_char = false;
        let mut expect_down3_after_reset = false;
        let mut expected_right_after_char = 0i32;
        while let Some(op) = bytes.get(index).copied() {
            if op == DVI_EOP {
                if expect_down3_after_reset || expect_width_right_after_char {
                    return None;
                }
                index += 1;
                break;
            }
            if expect_width_right_after_char {
                if op == DVI_RIGHT3 {
                    right3_count = right3_count.checked_add(1)?;
                    index += 1;
                    let amount = read_i24_be(bytes, &mut index)?;
                    if amount != expected_right_after_char {
                        return None;
                    }
                    page_h = page_h.checked_add(u32::try_from(amount).ok()?)?;
                    page_h_max = page_h_max.max(page_h);
                    expect_width_right_after_char = false;
                    continue;
                } else {
                    return None;
                }
            } else {
                if op == DVI_RIGHT3 {
                    right3_count = right3_count.checked_add(1)?;
                    index += 1;
                    let amount = read_i24_be(bytes, &mut index)?;
                    if amount >= 0 {
                        return None;
                    }
                    let back = u32::try_from(-amount).ok()?;
                    if back != page_h {
                        return None;
                    }
                    page_h = 0;
                    expect_down3_after_reset = true;
                    continue;
                } else if op == DVI_DOWN3 {
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
                    continue;
                }
                if op > 127 || !is_supported_text_byte_v0(op) {
                    return None;
                }
                if expect_down3_after_reset {
                    return None;
                }
                expected_right_after_char = glyph_width_sp_v0(op, glyph_advance_sp)?;
                index += 1;
                expect_width_right_after_char = true;
            }
        }
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
    Some((right3_count, w3_count, w0_count, down3_count, page_count))
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

pub fn validate_dvi_v2_text_page_v0(bytes: &[u8]) -> bool {
    count_dvi_v2_text_pages_v0(bytes).is_some()
}

#[cfg(test)]
mod tests;
