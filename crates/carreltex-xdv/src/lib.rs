const DVI_PRE: u8 = 247;
const DVI_BOP: u8 = 139;
const DVI_EOP: u8 = 140;
const DVI_POST: u8 = 248;
const DVI_POSTPOST: u8 = 249;
const DVI_ID_V2: u8 = 2;
const DVI_TRAILER_BYTE: u8 = 223;
const DVI_NUM: u32 = 25_400_000;
const DVI_DEN: u32 = 473_628_672;
const DVI_MAG: u32 = 1000;

fn push_u32_be(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn push_i32_be(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_be_bytes());
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

pub fn write_dvi_v2_empty_page_v0() -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    out.push(DVI_PRE);
    out.push(DVI_ID_V2);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    out.push(0); // comment_len

    let bop_offset = out.len() as u32;
    out.push(DVI_BOP);
    for _ in 0..10 {
        push_i32_be(&mut out, 0);
    }
    push_i32_be(&mut out, -1); // prev_bop
    out.push(DVI_EOP);

    let post_offset = out.len() as u32;
    out.push(DVI_POST);
    push_u32_be(&mut out, bop_offset);
    push_u32_be(&mut out, DVI_NUM);
    push_u32_be(&mut out, DVI_DEN);
    push_u32_be(&mut out, DVI_MAG);
    push_u32_be(&mut out, 0); // maxv
    push_u32_be(&mut out, 0); // maxh
    out.extend_from_slice(&0u16.to_be_bytes()); // maxstack
    out.extend_from_slice(&1u16.to_be_bytes()); // pages

    out.push(DVI_POSTPOST);
    push_u32_be(&mut out, post_offset);
    out.push(DVI_ID_V2);

    let trailer_padding = match out.len() % 4 {
        0 => 4,
        remainder => 8 - remainder,
    };
    for _ in 0..trailer_padding {
        out.push(DVI_TRAILER_BYTE);
    }
    out
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

#[cfg(test)]
mod tests {
    use super::{validate_dvi_v2_empty_page_v0, write_dvi_v2_empty_page_v0, DVI_PRE, DVI_TRAILER_BYTE};

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
}
