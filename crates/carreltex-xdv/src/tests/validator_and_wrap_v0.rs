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
fn validator_rejects_missing_width_right3_after_glyph() {
    let mut bytes = write_dvi_v2_text_page_v0(b"AB").expect("writer should accept AB");
    let right_index = bytes
        .iter()
        .position(|byte| *byte == DVI_RIGHT3)
        .expect("right3 opcode should exist");
    bytes[right_index] = DVI_DOWN3;
    bytes[right_index + 1] = 0x0c;
    bytes[right_index + 2] = 0x00;
    bytes[right_index + 3] = 0x00;
    assert!(!validate_dvi_v2_text_page_v0(&bytes));
    assert!(parse_dvi_v2_text_page_to_layout_v0(&bytes, 786_432).is_none());
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
