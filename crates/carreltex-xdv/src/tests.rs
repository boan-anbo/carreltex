use super::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0,
    count_dvi_v2_text_pages_with_advance_v0, validate_dvi_v2_empty_page_v0,
    sum_dvi_v2_positive_right3_amounts_with_layout_v0,
    validate_dvi_v2_text_page_v0, write_dvi_v2_empty_page_v0, write_dvi_v2_text_page_v0,
    write_dvi_v2_text_page_with_advance_v0, write_dvi_v2_text_page_with_layout_and_wrap_v0,
    write_dvi_v2_text_page_with_layout_v0, write_dvi_v2_text_page_with_layout_wrap_and_paging_v0,
    DVI_DOWN3, DVI_EOP, DVI_FNT_DEF1, DVI_PRE, DVI_RIGHT3, DVI_TRAILER_BYTE,
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
fn text_writer_emits_right3_movement_ops_only() {
    let bytes = write_dvi_v2_text_page_v0(b"ABCDE").expect("writer should accept text");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
    assert_eq!(movement, (5, 0, 0, 0, 1));
}

#[test]
fn text_writer_newline_emits_down3_and_keeps_single_page() {
    let bytes = write_dvi_v2_text_page_v0(b"A\nB").expect("writer should accept newline");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
    assert_eq!(movement, (3, 0, 0, 1, 1));
    assert!(bytes.contains(&DVI_DOWN3));
}

#[test]
fn text_writer_multichar_newline_reset_validates() {
    let bytes = write_dvi_v2_text_page_v0(b"AB\nC").expect("writer should accept newline");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
    assert_eq!(movement, (4, 0, 0, 1, 1));
}

#[test]
fn text_writer_uses_per_glyph_metrics_for_right3_amounts() {
    let glyph_advance_sp = 65_536;
    let bytes = write_dvi_v2_text_page_with_layout_v0(b"Wi.", glyph_advance_sp, 786_432)
        .expect("writer should accept Wi.");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
    assert_eq!(movement, (3, 0, 0, 0, 1));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(&bytes, glyph_advance_sp, 786_432)
        .expect("sum parser should parse");
    assert_eq!(total, (65_536 * 5 / 2) as u32);
}

#[test]
fn text_writer_wraps_long_line_with_down3() {
    let mut line = Vec::<u8>::new();
    for _ in 0..50 {
        line.extend_from_slice(b"A ");
    }
    let bytes = write_dvi_v2_text_page_v0(&line).expect("writer should accept wrapped line");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let movement = count_dvi_v2_text_movements_v0(&bytes).expect("movement summary should parse");
    assert_eq!(movement.4, 1);
    assert!(movement.3 >= 1);
}

#[test]
fn text_writer_rejects_non_positive_advance() {
    assert!(write_dvi_v2_text_page_with_advance_v0(b"ABC", 0).is_none());
    assert!(write_dvi_v2_text_page_with_advance_v0(b"ABC", -1).is_none());
    assert!(write_dvi_v2_text_page_with_layout_v0(b"ABC", 1024, 0).is_none());
    assert!(write_dvi_v2_text_page_with_layout_v0(b"ABC", 1024, -1).is_none());
    assert!(write_dvi_v2_text_page_with_layout_and_wrap_v0(b"ABC", 1024, 2048, 0).is_none());
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
fn validator_rejects_positive_right_without_preceding_char() {
    let mut bytes = write_dvi_v2_text_page_v0(b"AB").expect("writer should accept AB");
    let right_index = bytes
        .iter()
        .position(|byte| *byte == DVI_RIGHT3)
        .expect("right3 opcode should exist");
    bytes[right_index - 1] = DVI_RIGHT3;
    bytes[right_index] = 0x00;
    bytes[right_index + 1] = 0x00;
    bytes[right_index + 2] = 0x01;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
}

#[test]
fn validator_rejects_wrong_movement_amount() {
    let mut bytes = write_dvi_v2_text_page_v0(b"ABCD").expect("writer should accept ABCD");
    let right_index = bytes
        .iter()
        .position(|byte| *byte == DVI_RIGHT3)
        .expect("right3 opcode should exist");
    let amount_start = right_index + 1;
    bytes[amount_start] = 0x00;
    bytes[amount_start + 1] = 0x00;
    bytes[amount_start + 2] = 0x01;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
}

#[test]
fn validator_rejects_wrong_down3_amount() {
    let mut bytes = write_dvi_v2_text_page_v0(b"A\nB").expect("writer should accept newline");
    let down3_index = bytes
        .iter()
        .position(|byte| *byte == DVI_DOWN3)
        .expect("down3 opcode should exist");
    let amount_start = down3_index + 1;
    bytes[amount_start] = 0x00;
    bytes[amount_start + 1] = 0x00;
    bytes[amount_start + 2] = 0x01;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
}

#[test]
fn validator_rejects_wrong_reset_amount_before_down3() {
    let mut bytes = write_dvi_v2_text_page_v0(b"AB\nC").expect("writer should accept newline");
    let down3_index = bytes
        .iter()
        .position(|byte| *byte == DVI_DOWN3)
        .expect("down3 opcode should exist");
    let reset_index = bytes[..down3_index]
        .iter()
        .rposition(|byte| *byte == DVI_RIGHT3)
        .expect("reset right3 opcode should exist");
    let amount_start = reset_index + 1;
    bytes[amount_start] = 0xff;
    bytes[amount_start + 1] = 0xff;
    bytes[amount_start + 2] = 0xff;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
}

#[test]
fn validator_rejects_wrong_reset_amount_in_wrapped_output() {
    let mut line = Vec::<u8>::new();
    for _ in 0..50 {
        line.extend_from_slice(b"A ");
    }
    let mut bytes = write_dvi_v2_text_page_v0(&line).expect("writer should accept wrapped line");
    let down3_index = bytes
        .iter()
        .position(|byte| *byte == DVI_DOWN3)
        .expect("down3 opcode should exist");
    let reset_index = bytes[..down3_index]
        .iter()
        .rposition(|byte| *byte == DVI_RIGHT3)
        .expect("reset right3 opcode should exist");
    let amount_start = reset_index + 1;
    bytes[amount_start] = 0x00;
    bytes[amount_start + 1] = 0x00;
    bytes[amount_start + 2] = 0x01;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
}

#[test]
fn count_rejects_mismatched_advance_parameter() {
    let bytes = write_dvi_v2_text_page_with_advance_v0(b"ABC", 1024).expect("writer should accept");
    assert_eq!(
        count_dvi_v2_text_pages_with_advance_v0(&bytes, 1024),
        Some(1)
    );
    assert_eq!(count_dvi_v2_text_pages_with_advance_v0(&bytes, 2048), None);
}

#[test]
fn write_with_small_wrap_cap_increases_down3_count() {
    let text = b"word word word word word word word word word word";
    let wide = write_dvi_v2_text_page_with_layout_and_wrap_v0(text, 65_536, 786_432, 80)
        .expect("writer should accept wide cap");
    let narrow = write_dvi_v2_text_page_with_layout_and_wrap_v0(text, 65_536, 786_432, 10)
        .expect("writer should accept narrow cap");
    assert!(validate_dvi_v2_text_page_v0(&wide));
    assert!(validate_dvi_v2_text_page_v0(&narrow));
    let wide_down3 = count_dvi_v2_text_movements_v0(&wide)
        .expect("wide movement summary should parse")
        .3;
    let narrow_down3 = count_dvi_v2_text_movements_v0(&narrow)
        .expect("narrow movement summary should parse")
        .3;
    assert!(narrow_down3 > wide_down3);
}

#[test]
fn write_with_wrap_cap_one_hard_breaks_each_glyph() {
    let bytes = write_dvi_v2_text_page_with_layout_and_wrap_v0(b"AB", 65_536, 786_432, 1)
        .expect("writer should accept wrap cap 1");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let down3_count = count_dvi_v2_text_movements_v0(&bytes)
        .expect("movement summary should parse")
        .3;
    assert_eq!(down3_count, 1);
}

#[test]
fn write_with_paging_limit_splits_into_multiple_pages() {
    let bytes = write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(
        b"line one line two line three line four line five line six",
        65_536,
        786_432,
        8,
        2,
    )
    .expect("writer should accept paging parameters");
    assert!(validate_dvi_v2_text_page_v0(&bytes));
    let pages = count_dvi_v2_text_pages_v0(&bytes).expect("page count");
    assert!(pages >= 2);
}
