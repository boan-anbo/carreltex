const DVI_PRE: u8 = 247;
const DVI_BOP: u8 = 139;
const DVI_EOP: u8 = 140;
const DVI_POST: u8 = 248;
const DVI_POSTPOST: u8 = 249;
const DVI_FNT_DEF1: u8 = 243;
const DVI_FNT_NUM_0: u8 = 171;
const DVI_W0: u8 = 147;
const DVI_W3: u8 = 150;
const DVI_RIGHT3: u8 = 145;
const DVI_ID_V2: u8 = 2;
const DVI_TRAILER_BYTE: u8 = 223;
const DVI_NUM: u32 = 25_400_000;
const DVI_DEN: u32 = 473_628_672;
const DVI_MAG: u32 = 1000;
const FONT_ID_V0: u8 = 0;
const FONT_NAME_V0: &[u8] = b"carreltex-v0";
const PAGEBREAK_MARKER_V0: u8 = 0x0c;
pub const DEFAULT_GLYPH_ADVANCE_SP_V0: i32 = 65_536;

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

fn split_pages_v0(text: &[u8]) -> Option<Vec<&[u8]>> {
    if text
        .iter()
        .any(|byte| !is_supported_text_byte_v0(*byte) && *byte != PAGEBREAK_MARKER_V0)
    {
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

pub fn write_dvi_v2_text_page_with_advance_v0(text: &[u8], glyph_advance_sp: i32) -> Option<Vec<u8>> {
    if glyph_advance_sp <= 0 {
        return None;
    }
    let pages = split_pages_v0(text)?;
    let page_count = u16::try_from(pages.len()).ok()?;
    if page_count == 0 {
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
    for page in pages {
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
        let mut w_defined = false;
        for (index, byte) in page.iter().enumerate() {
            if index > 0 {
                if index % 2 == 1 {
                    out.push(DVI_RIGHT3);
                    push_i24_be(&mut out, glyph_advance_sp)?;
                } else if !w_defined {
                    out.push(DVI_W3);
                    push_i24_be(&mut out, glyph_advance_sp)?;
                    w_defined = true;
                } else {
                    out.push(DVI_W0);
                }
                page_h = page_h.checked_add(u32::try_from(glyph_advance_sp).ok()?)?;
            }
            out.push(*byte);
        }
        max_h = max_h.max(page_h);
        out.push(DVI_EOP);
        bop_offsets.push(bop_offset);
    }

    let post_offset = out.len() as u32;
    out.push(DVI_POST);
    push_u32_be(&mut out, *bop_offsets.last()?);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    push_u32_be(&mut out, max_h);
    push_u32_be(&mut out, 0);
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&page_count.to_be_bytes());

    out.push(DVI_POSTPOST);
    push_u32_be(&mut out, post_offset);
    out.push(DVI_ID_V2);
    append_trailer(&mut out);
    Some(out)
}

pub fn write_dvi_v2_text_page_v0(text: &[u8]) -> Option<Vec<u8>> {
    write_dvi_v2_text_page_with_advance_v0(text, DEFAULT_GLYPH_ADVANCE_SP_V0)
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

pub fn count_dvi_v2_text_pages_with_advance_v0(bytes: &[u8], glyph_advance_sp: i32) -> Option<u16> {
    if glyph_advance_sp <= 0 {
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

    let mut page_count = 0u16;
    let mut previous_bop_offset: Option<usize> = None;
    let mut last_bop_offset = 0u32;
    let mut max_h = 0u32;
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
        if read_and_validate_font_def_v0(bytes, &mut index).is_none() {
            return None;
        }
        if read_u8(bytes, &mut index) != Some(DVI_FNT_NUM_0) {
            return None;
        }
        let mut page_h = 0u32;
        let mut expect_move_after_char = false;
        let mut saw_page_char = false;
        let mut w_defined = false;
        while let Some(opcode) = bytes.get(index).copied() {
            if opcode == DVI_EOP {
                if !saw_page_char && expect_move_after_char {
                    return None;
                }
                index += 1;
                break;
            }
            if expect_move_after_char {
                if opcode == DVI_RIGHT3 {
                    index += 1;
                    if read_i24_be(bytes, &mut index)? != glyph_advance_sp {
                        return None;
                    }
                } else if opcode == DVI_W3 {
                    index += 1;
                    if read_i24_be(bytes, &mut index)? != glyph_advance_sp {
                        return None;
                    }
                    w_defined = true;
                } else if opcode == DVI_W0 {
                    if !w_defined {
                        return None;
                    }
                    index += 1;
                } else {
                    return None;
                }
                page_h = page_h.checked_add(u32::try_from(glyph_advance_sp).ok()?)?;
                let next = *bytes.get(index)?;
                if next > 127 || !is_supported_text_byte_v0(next) {
                    return None;
                }
                index += 1;
                saw_page_char = true;
                expect_move_after_char = true;
            } else {
                if opcode > 127 || !is_supported_text_byte_v0(opcode) {
                    return None;
                }
                index += 1;
                saw_page_char = true;
                expect_move_after_char = true;
            }
        }
        if page_h > max_h {
            max_h = page_h;
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
    if read_u32_be(bytes, &mut index) != Some(0) {
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
    Some(page_count)
}

pub fn count_dvi_v2_text_movements_with_advance_v0(
    bytes: &[u8],
    glyph_advance_sp: i32,
) -> Option<(u32, u32, u32)> {
    if glyph_advance_sp <= 0 {
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
    let mut w3_count = 0u32;
    let mut w0_count = 0u32;
    loop {
        let opcode = *bytes.get(index)?;
        if opcode == DVI_POST {
            break;
        }
        if opcode != DVI_BOP {
            return None;
        }
        index += 1;
        for _ in 0..10 {
            read_i32_be(bytes, &mut index)?;
        }
        read_i32_be(bytes, &mut index)?;
        read_and_validate_font_def_v0(bytes, &mut index)?;
        if read_u8(bytes, &mut index) != Some(DVI_FNT_NUM_0) {
            return None;
        }
        let mut expect_move_after_char = false;
        let mut w_defined = false;
        while let Some(op) = bytes.get(index).copied() {
            if op == DVI_EOP {
                index += 1;
                break;
            }
            if expect_move_after_char {
                if op == DVI_RIGHT3 {
                    right3_count = right3_count.checked_add(1)?;
                    index += 1;
                    if read_i24_be(bytes, &mut index)? != glyph_advance_sp {
                        return None;
                    }
                } else if op == DVI_W3 {
                    w3_count = w3_count.checked_add(1)?;
                    index += 1;
                    if read_i24_be(bytes, &mut index)? != glyph_advance_sp {
                        return None;
                    }
                    w_defined = true;
                } else if op == DVI_W0 {
                    if !w_defined {
                        return None;
                    }
                    w0_count = w0_count.checked_add(1)?;
                    index += 1;
                } else {
                    return None;
                }
                let next = *bytes.get(index)?;
                if next > 127 || !is_supported_text_byte_v0(next) {
                    return None;
                }
                index += 1;
                expect_move_after_char = true;
            } else {
                if op > 127 || !is_supported_text_byte_v0(op) {
                    return None;
                }
                index += 1;
                expect_move_after_char = true;
            }
        }
    }
    Some((right3_count, w3_count, w0_count))
}

pub fn count_dvi_v2_text_movements_v0(bytes: &[u8]) -> Option<(u32, u32, u32)> {
    count_dvi_v2_text_movements_with_advance_v0(bytes, DEFAULT_GLYPH_ADVANCE_SP_V0)
}

pub fn count_dvi_v2_text_pages_v0(bytes: &[u8]) -> Option<u16> {
    count_dvi_v2_text_pages_with_advance_v0(bytes, DEFAULT_GLYPH_ADVANCE_SP_V0)
}

pub fn validate_dvi_v2_text_page_v0(bytes: &[u8]) -> bool {
    count_dvi_v2_text_pages_v0(bytes).is_some()
}

#[cfg(test)]
mod tests {
    use super::{
        count_dvi_v2_text_pages_v0, count_dvi_v2_text_pages_with_advance_v0,
        count_dvi_v2_text_movements_v0,
        validate_dvi_v2_empty_page_v0, validate_dvi_v2_text_page_v0, write_dvi_v2_empty_page_v0,
        write_dvi_v2_text_page_v0, write_dvi_v2_text_page_with_advance_v0, DVI_EOP, DVI_FNT_DEF1,
        DVI_PRE, DVI_RIGHT3, DVI_TRAILER_BYTE, DVI_W0, DVI_W3,
    };

    #[test]
    fn writer_output_validates() {
        let bytes = write_dvi_v2_empty_page_v0();
        assert!(validate_dvi_v2_empty_page_v0(&bytes));
        assert_eq!(bytes.first().copied(), Some(DVI_PRE));
        assert_eq!(bytes.last().copied(), Some(DVI_TRAILER_BYTE));
    }

    #[test]
    fn writer_output_is_non_empty() {
        let bytes = write_dvi_v2_empty_page_v0();
        assert!(!bytes.is_empty());
        assert_eq!(bytes.len() % 4, 0);
    }

    #[test]
    fn text_writer_output_validates() {
        let bytes = write_dvi_v2_text_page_v0(b"XYZ").expect("writer should accept XYZ");
        assert!(validate_dvi_v2_text_page_v0(&bytes));
        assert_eq!(count_dvi_v2_text_pages_v0(&bytes), Some(1));
        assert_eq!(bytes.first().copied(), Some(DVI_PRE));
        assert_eq!(bytes.last().copied(), Some(DVI_TRAILER_BYTE));
    }

    #[test]
    fn text_writer_allows_empty_text_body() {
        let bytes = write_dvi_v2_text_page_v0(b"").expect("writer should accept empty text");
        assert!(validate_dvi_v2_text_page_v0(&bytes));
        assert_eq!(count_dvi_v2_text_pages_v0(&bytes), Some(1));
        assert_eq!(bytes.first().copied(), Some(DVI_PRE));
        assert_eq!(bytes.last().copied(), Some(DVI_TRAILER_BYTE));
    }

    #[test]
    fn text_writer_pagebreak_emits_multiple_pages() {
        let bytes = write_dvi_v2_text_page_v0(b"AB\x0cCD").expect("writer should accept pagebreak");
        assert!(validate_dvi_v2_text_page_v0(&bytes));
        assert_eq!(count_dvi_v2_text_pages_v0(&bytes), Some(2));
    }

    #[test]
    fn text_writer_emits_right_and_w_movement_ops() {
        let bytes = write_dvi_v2_text_page_v0(b"ABCDE").expect("writer should accept text");
        assert!(validate_dvi_v2_text_page_v0(&bytes));
        let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
        assert_eq!(movement, (2, 1, 1));
    }

    #[test]
    fn text_writer_rejects_non_positive_advance() {
        assert!(write_dvi_v2_text_page_with_advance_v0(b"ABC", 0).is_none());
        assert!(write_dvi_v2_text_page_with_advance_v0(b"ABC", -1).is_none());
    }

    #[test]
    fn text_writer_rejects_out_of_range_bytes() {
        assert!(write_dvi_v2_text_page_v0(&[0x1f]).is_none());
        assert!(write_dvi_v2_text_page_v0(&[0x7f]).is_none());
    }

    #[test]
    fn validator_rejects_missing_font_definition() {
        let mut bytes = write_dvi_v2_text_page_v0(b"XYZ").expect("writer should accept XYZ");
        let font_def_index = bytes
            .iter()
            .position(|byte| *byte == DVI_FNT_DEF1)
            .expect("font def opcode should exist");
        bytes[font_def_index] = DVI_EOP;
        assert!(!validate_dvi_v2_text_page_v0(&bytes));
    }

    #[test]
    fn validator_rejects_set_char_before_font_select() {
        let mut bytes = write_dvi_v2_text_page_v0(b"XYZ").expect("writer should accept XYZ");
        let font_def_index = bytes
            .iter()
            .position(|byte| *byte == DVI_FNT_DEF1)
            .expect("font def opcode should exist");
        let font_select_index = font_def_index + 27;
        bytes[font_select_index] = b'X';
        assert!(!validate_dvi_v2_text_page_v0(&bytes));
    }

    #[test]
    fn validator_rejects_w0_without_w_definition() {
        let mut bytes = write_dvi_v2_text_page_v0(b"AB").expect("writer should accept AB");
        let right_index = bytes
            .iter()
            .position(|byte| *byte == DVI_RIGHT3)
            .expect("right3 opcode should exist");
        bytes[right_index] = DVI_W0;
        assert!(!validate_dvi_v2_text_page_v0(&bytes));
    }

    #[test]
    fn validator_rejects_wrong_movement_amount() {
        let mut bytes = write_dvi_v2_text_page_v0(b"ABCD").expect("writer should accept ABCD");
        let w3_index = bytes
            .iter()
            .position(|byte| *byte == DVI_W3)
            .expect("w3 opcode should exist");
        let amount_start = w3_index + 1;
        bytes[amount_start] = 0x00;
        bytes[amount_start + 1] = 0x00;
        bytes[amount_start + 2] = 0x01;
        assert!(!validate_dvi_v2_text_page_v0(&bytes));
    }

    #[test]
    fn count_rejects_mismatched_advance_parameter() {
        let bytes = write_dvi_v2_text_page_with_advance_v0(b"ABC", 1024).expect("writer should accept");
        assert_eq!(count_dvi_v2_text_pages_with_advance_v0(&bytes, 1024), Some(1));
        assert_eq!(count_dvi_v2_text_pages_with_advance_v0(&bytes, 2048), None);
    }
}
