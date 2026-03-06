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
    let total =
        sum_dvi_v2_positive_right3_amounts_with_layout_v0(&bytes, glyph_advance_sp, 786_432)
            .expect("sum parser should parse");
    assert_eq!(total, (65_536 * 5 / 2) as u32);
}

#[test]
fn text_writer_accepts_glyph_advance_one_for_half_em_glyphs() {
    let bytes = write_dvi_v2_text_page_with_layout_v0(b"i. ", 1, 786_432)
        .expect("writer should accept glyph advance 1");
    assert!(validate_dvi_v2_text_page_with_layout_v0(&bytes, 1, 786_432));
    let total = sum_dvi_v2_positive_right3_amounts_with_layout_v0(&bytes, 1, 786_432)
        .expect("sum parser should parse");
    assert_eq!(total, 3);
}

#[test]
fn planner_wrap_and_paging_shape_is_deterministic() {
    let plan = plan_layout_v0(b"ABCD\nEFGHIJKL", 65_536, 786_432, 4, 2).expect("layout plan");
    assert_eq!(plan.pages.len(), 2);
    assert_eq!(plan.pages[0].lines.len(), 2);
    assert_eq!(plan.pages[1].lines.len(), 1);
    assert_eq!(plan.pages[0].lines[0].width_sp, 4 * 65_536);
    assert_eq!(plan.pages[0].lines[1].width_sp, 4 * 65_536);
    assert!(plan.pages[1].lines[0].width_sp < 4 * 65_536);
}

#[test]
fn planner_forced_pagebreak_and_wrap_interaction_shape() {
    let plan =
        plan_layout_v0(b"ABCDEFGH\x0cIJKLMNOP", 65_536, 786_432, 4, 10).expect("layout plan");
    assert_eq!(plan.pages.len(), 2);
    assert_eq!(plan.pages[0].lines.len(), 2);
    assert_eq!(plan.pages[1].lines.len(), 2);
    assert_eq!(plan.pages[1].lines[0].glyphs[0].byte, b'I');
    assert_eq!(plan.pages[1].lines[1].glyphs[0].byte, b'M');
}

#[test]
fn planner_propagates_wi_dot_glyph_advances_and_line_width() {
    let plan = plan_layout_v0(b"Wi.", 65_536, 786_432, 80, 200).expect("layout plan");
    assert_eq!(plan.pages.len(), 1);
    assert_eq!(plan.pages[0].lines.len(), 1);
    let line = &plan.pages[0].lines[0];
    assert_eq!(line.glyphs.len(), 3);
    assert_eq!(line.glyphs[0].advance_sp, 98_304);
    assert_eq!(line.glyphs[1].advance_sp, 32_768);
    assert_eq!(line.glyphs[2].advance_sp, 32_768);
    assert_eq!(line.width_sp, 163_840);
    assert_eq!(recompute_line_width_sp_v0(line), Some(line.width_sp));
}

#[test]
fn planner_space_and_punctuation_advances_are_stable_v0() {
    let plan =
        plan_layout_width_v0(b"A ,.!? B", 65_536, 786_432, 10_000_000, 200).expect("layout plan");
    assert_eq!(plan.pages.len(), 1);
    assert_eq!(plan.pages[0].lines.len(), 1);
    let line = &plan.pages[0].lines[0];
    let glyph_bytes: Vec<u8> = line.glyphs.iter().map(|glyph| glyph.byte).collect();
    let glyph_advances: Vec<i32> = line.glyphs.iter().map(|glyph| glyph.advance_sp).collect();
    assert_eq!(glyph_bytes, b"A ,.!? B");
    assert_eq!(
        glyph_advances,
        vec![65_536, 32_768, 32_768, 32_768, 32_768, 32_768, 32_768, 65_536]
    );
    let recomputed = recompute_line_width_sp_v0(line).expect("line width should recompute");
    let summed = glyph_advances
        .iter()
        .copied()
        .fold(0u32, |acc, value| acc + value as u32);
    assert_eq!(recomputed, summed);
    assert_eq!(line.width_sp, summed);
}

#[test]
fn planner_variable_width_ratio_categories_are_stable_v0() {
    let plan =
        plan_layout_width_v0(b"Wm i|.M", 65_536, 786_432, 10_000_000, 200).expect("layout plan");
    assert_eq!(plan.pages.len(), 1);
    assert_eq!(plan.pages[0].lines.len(), 1);
    let line = &plan.pages[0].lines[0];
    let glyph_bytes: Vec<u8> = line.glyphs.iter().map(|glyph| glyph.byte).collect();
    let glyph_advances: Vec<i32> = line.glyphs.iter().map(|glyph| glyph.advance_sp).collect();
    assert_eq!(glyph_bytes, b"Wm i|.M");
    assert_eq!(
        glyph_advances,
        vec![98_304, 98_304, 32_768, 32_768, 32_768, 32_768, 98_304]
    );
    assert_eq!(recompute_line_width_sp_v0(line), Some(line.width_sp));
}

#[test]
fn planner_width_wraps_long_line_at_spaces() {
    let plan = plan_layout_width_v0(b"aaaa bbbb cccc", 65_536, 786_432, 327_680, 200)
        .expect("layout plan");
    assert_eq!(plan.pages.len(), 1);
    assert_eq!(plan.pages[0].lines.len(), 3);
    assert_eq!(
        plan.pages[0].lines[0]
            .glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<_>>(),
        b"aaaa"
    );
    assert_eq!(
        plan.pages[0].lines[1]
            .glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<_>>(),
        b"bbbb"
    );
    assert_eq!(
        plan.pages[0].lines[2]
            .glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<_>>(),
        b"cccc"
    );
}

#[test]
fn planner_width_uses_variable_space_width_v0() {
    let plan = plan_layout_width_v0(b"A A", 100, 786_432, 250, 200).expect("layout plan");
    assert_eq!(plan.pages.len(), 1);
    assert_eq!(plan.pages[0].lines.len(), 1);
    assert_eq!(
        plan.pages[0].lines[0]
            .glyphs
            .iter()
            .map(|glyph| glyph.byte)
            .collect::<Vec<_>>(),
        b"A A"
    );
}

#[test]
fn style_markers_have_zero_advance_and_roundtrip_v0() {
    let layout =
        plan_layout_width_v0(b"A [B] {C}", 65_536, 786_432, 10_000_000, 200).expect("layout plan");
    assert_eq!(layout.pages.len(), 1);
    assert_eq!(layout.pages[0].lines.len(), 1);
    let line = &layout.pages[0].lines[0];

    for glyph in &line.glyphs {
        if matches!(glyph.byte, b'[' | b']' | b'{' | b'}') {
            assert_eq!(glyph.advance_sp, 0);
        } else {
            assert!(glyph.advance_sp > 0);
        }
    }

    let xdv = write_dvi_v2_text_page_from_layout_v0(&layout, 786_432).expect("xdv bytes");
    let parsed = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("parsed layout");
    assert_eq!(parsed, layout);
    assert!(validate_dvi_v2_text_page_matches_layout_v0(
        &xdv, &layout, 786_432
    ));
    assert!(validate_dvi_v2_text_page_with_layout_v0(
        &xdv, 65_536, 786_432
    ));
}
