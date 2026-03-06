#[test]
fn pdf_renderer_accepts_bibliography_and_cite_metadata_lines_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Body cite [1] and unresolved [?].\n\n!b ref:a 1 Alpha source text.\n!c ref:a 1 1\n!c missing 1 0",
    )
    .expect("writer should accept bibliography marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Body cite "),
        "body prefix text should render: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(1) Tj"),
        "resolved cite token should render: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(?) Tj"),
        "unresolved cite token should render: {pdf_text}"
    );
    assert!(
        !pdf_text.contains("!b ref:a"),
        "bibliography metadata prefix should be hidden in pdf output: {pdf_text}"
    );
    assert!(
        !pdf_text.contains("!c ref:a"),
        "cite metadata prefix should be hidden in pdf output: {pdf_text}"
    );
}

#[test]
fn pdf_renderer_rejects_malformed_cite_metadata_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Body text.\n\n!c ref:a 0 1")
        .expect("writer should accept marker text bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on malformed cite metadata line"
    );
}

#[test]
fn pdf_renderer_table_rows_use_stable_column_x_offsets_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before.\n\n!ts lcr\n!t Alpha||Beta||Gamma\n!t Delta||Epsilon||Zeta\n\nAfter.",
    )
    .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        !pdf_text.contains("!t Alpha||Beta||Gamma"),
        "table marker should be hidden in pdf output: {pdf_text}"
    );

    let epsilon_pt = 0.05f32;
    let alpha_x = *tm_xs_for_segment_text_v0(&pdf, "Alpha")
        .first()
        .expect("alpha x");
    let delta_x = *tm_xs_for_segment_text_v0(&pdf, "Delta")
        .first()
        .expect("delta x");
    let beta_x = *tm_xs_for_segment_text_v0(&pdf, "Beta")
        .first()
        .expect("beta x");
    let epsilon_x = *tm_xs_for_segment_text_v0(&pdf, "Epsilon")
        .first()
        .expect("epsilon x");
    let gamma_x = *tm_xs_for_segment_text_v0(&pdf, "Gamma")
        .first()
        .expect("gamma x");
    let zeta_x = *tm_xs_for_segment_text_v0(&pdf, "Zeta")
        .first()
        .expect("zeta x");

    assert!(
        (alpha_x - delta_x).abs() <= epsilon_pt,
        "column 1 x drift: {alpha_x} vs {delta_x}"
    );
    assert!(
        (beta_x - epsilon_x).abs() <= 2.0,
        "column 2 x drift too large: {beta_x} vs {epsilon_x}"
    );
    assert!(
        (gamma_x - zeta_x).abs() <= 3.0,
        "column 3 x drift too large: {gamma_x} vs {zeta_x}"
    );
    assert!(
        alpha_x < beta_x && beta_x < gamma_x,
        "column order mismatch"
    );
}

#[test]
fn pdf_renderer_table_cells_stay_within_column_bounds_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before.\n\n!ts lcr\n!t A||WideMiddle||9.9\n!t LongLeft||B||123.45\n\nAfter.",
    )
    .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let left_margin_pt = 72.0f32;
    let cell_padding_pt = 7.0f32;
    let epsilon_pt = 0.05f32;
    let col1_width_pt = segment_width_pt_v0(b"LongLeft");
    let col2_width_pt = segment_width_pt_v0(b"WideMiddle");
    let col3_width_pt = segment_width_pt_v0(b"123.45");
    let col1_content_left_pt = left_margin_pt + cell_padding_pt;
    let col2_content_left_pt = left_margin_pt + col1_width_pt + (cell_padding_pt * 3.0);
    let col3_content_left_pt = col2_content_left_pt + col2_width_pt + (cell_padding_pt * 2.0);

    let a_x = *tm_xs_for_segment_text_v0(&pdf, "A").first().expect("A x");
    let longleft_x = *tm_xs_for_segment_text_v0(&pdf, "LongLeft")
        .first()
        .expect("LongLeft x");
    let wide_x = *tm_xs_for_segment_text_v0(&pdf, "WideMiddle")
        .first()
        .expect("WideMiddle x");
    let b_x = *tm_xs_for_segment_text_v0(&pdf, "B").first().expect("B x");
    let nine_x = *tm_xs_for_segment_text_v0(&pdf, "9.9")
        .first()
        .expect("9.9 x");
    let one_two_three_x = *tm_xs_for_segment_text_v0(&pdf, "123.45")
        .first()
        .expect("123.45 x");

    assert!(
        (a_x - col1_content_left_pt).abs() <= epsilon_pt,
        "column 1 left-aligned start mismatch: {a_x} vs {col1_content_left_pt}"
    );
    assert!(
        (longleft_x - col1_content_left_pt).abs() <= epsilon_pt,
        "column 1 left-aligned start mismatch: {longleft_x} vs {col1_content_left_pt}"
    );
    let expected_wide_x =
        col2_content_left_pt + ((col2_width_pt - segment_width_pt_v0(b"WideMiddle")) * 0.5);
    let expected_b_x = col2_content_left_pt + ((col2_width_pt - segment_width_pt_v0(b"B")) * 0.5);
    assert!(
        (wide_x - expected_wide_x).abs() <= epsilon_pt,
        "column 2 centered start mismatch: {wide_x} vs {expected_wide_x}"
    );
    assert!(
        (b_x - expected_b_x).abs() <= epsilon_pt,
        "column 2 centered start mismatch: {b_x} vs {expected_b_x}"
    );
    let expected_nine_x = col3_content_left_pt + (col3_width_pt - segment_width_pt_v0(b"9.9"));
    let expected_one_two_three_x =
        col3_content_left_pt + (col3_width_pt - segment_width_pt_v0(b"123.45"));
    assert!(
        (nine_x - expected_nine_x).abs() <= epsilon_pt,
        "column 3 right-aligned start mismatch: {nine_x} vs {expected_nine_x}"
    );
    assert!(
        (one_two_three_x - expected_one_two_three_x).abs() <= epsilon_pt,
        "column 3 right-aligned start mismatch: {one_two_three_x} vs {expected_one_two_three_x}"
    );

    let a_right_x = a_x + segment_width_pt_v0(b"A");
    let longleft_right_x = longleft_x + segment_width_pt_v0(b"LongLeft");
    let wide_right_x = wide_x + segment_width_pt_v0(b"WideMiddle");
    let b_right_x = b_x + segment_width_pt_v0(b"B");
    let nine_right_x = nine_x + segment_width_pt_v0(b"9.9");
    let one_two_three_right_x = one_two_three_x + segment_width_pt_v0(b"123.45");
    assert!(
        a_x >= col1_content_left_pt - epsilon_pt
            && a_right_x <= col1_content_left_pt + col1_width_pt + epsilon_pt,
        "row 1 col 1 text should remain within column bounds"
    );
    assert!(
        longleft_x >= col1_content_left_pt - epsilon_pt
            && longleft_right_x <= col1_content_left_pt + col1_width_pt + epsilon_pt,
        "row 2 col 1 text should remain within column bounds"
    );
    assert!(
        wide_x >= col2_content_left_pt - epsilon_pt
            && wide_right_x <= col2_content_left_pt + col2_width_pt + epsilon_pt,
        "row 1 col 2 text should remain within column bounds"
    );
    assert!(
        b_x >= col2_content_left_pt - epsilon_pt
            && b_right_x <= col2_content_left_pt + col2_width_pt + epsilon_pt,
        "row 2 col 2 text should remain within column bounds"
    );
    assert!(
        nine_x >= col3_content_left_pt - epsilon_pt
            && nine_right_x <= col3_content_left_pt + col3_width_pt + epsilon_pt,
        "row 1 col 3 text should remain within column bounds"
    );
    assert!(
        one_two_three_x >= col3_content_left_pt - epsilon_pt
            && one_two_three_right_x <= col3_content_left_pt + col3_width_pt + epsilon_pt,
        "row 2 col 3 text should remain within column bounds"
    );
}

#[test]
fn pdf_renderer_table_colspec_alignment_respects_lcr_per_column_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before.\n\n!ts rcll\n!t 9||Mid||Tail||End\n!t 10||More||Tail2||Ending\n\nAfter.",
    )
    .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nine_x = *tm_xs_for_segment_text_v0(&pdf, "9").first().expect("9 x");
    let ten_x = *tm_xs_for_segment_text_v0(&pdf, "10").first().expect("10 x");
    let nine_right = nine_x + segment_width_pt_v0(b"9");
    let ten_right = ten_x + segment_width_pt_v0(b"10");
    assert!(
        (nine_right - ten_right).abs() <= 0.1,
        "right-aligned first column should share right edge: {nine_right} vs {ten_right}"
    );

    let mid_x = *tm_xs_for_segment_text_v0(&pdf, "Mid")
        .first()
        .expect("mid x");
    let more_x = *tm_xs_for_segment_text_v0(&pdf, "More")
        .first()
        .expect("more x");
    let mid_center = mid_x + (segment_width_pt_v0(b"Mid") * 0.5);
    let more_center = more_x + (segment_width_pt_v0(b"More") * 0.5);
    assert!(
        (mid_center - more_center).abs() <= 0.2,
        "center-aligned second column should share center: {mid_center} vs {more_center}"
    );

    let tail_x = *tm_xs_for_segment_text_v0(&pdf, "Tail")
        .first()
        .expect("tail x");
    let tail2_x = *tm_xs_for_segment_text_v0(&pdf, "Tail2")
        .first()
        .expect("tail2 x");
    assert!(
        (tail_x - tail2_x).abs() <= 0.1,
        "left-aligned third column should share left edge: {tail_x} vs {tail2_x}"
    );

    let end_x = *tm_xs_for_segment_text_v0(&pdf, "End")
        .first()
        .expect("end x");
    let ending_x = *tm_xs_for_segment_text_v0(&pdf, "Ending")
        .first()
        .expect("ending x");
    assert!(
        (end_x - ending_x).abs() <= 0.1,
        "left-aligned fourth column should share left edge: {end_x} vs {ending_x}"
    );
}

#[test]
fn pdf_renderer_table_grid_lines_render_deterministically_v1() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!ts lcr\n!t A||B||C\n!t D||E||F\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains(" re S"),
        "expected table outer border rectangle path command"
    );
    let separator_line_count = pdf_text.matches(" l S").count();
    assert!(
        separator_line_count >= 3,
        "expected deterministic row/column separator lines, got {separator_line_count}"
    );
}

#[test]
fn pdf_renderer_table_row_height_is_stable_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!ts lcr\n!t A||B||C\n!t D||E||F\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (_, row1_y) = tm_position_for_line_containing_text_v0(&pdf, "(A)").expect("row 1 y");
    let (_, row2_y) = tm_position_for_line_containing_text_v0(&pdf, "(D)").expect("row 2 y");
    let delta = row1_y - row2_y;
    assert!(
        (delta - 15.0).abs() <= 0.2,
        "table row height should remain stable at TABLE_ROW_LEADING_PT_V0: {delta}"
    );
}

#[test]
fn pdf_renderer_rejects_table_rows_without_spec_line_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!t A||B||C\n!t D||E||F\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when !t rows are missing !ts spec line"
    );
}

#[test]
fn pdf_renderer_rejects_invalid_table_spec_letters_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!ts lxp\n!t A||B||C\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on invalid table spec bytes"
    );
}

#[test]
fn pdf_renderer_rejects_table_row_column_mismatch_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!ts lc\n!t A||B||C\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when table row cell count mismatches !ts col_count"
    );
}

#[test]
fn pdf_renderer_rejects_table_row_with_empty_cell_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!ts lcr\n!t A||||C\n\nAfter.")
        .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on empty table cell content"
    );
}

#[test]
fn pdf_renderer_rejects_table_width_overflow_v0() {
    let mut row = Vec::<u8>::new();
    row.extend_from_slice(b"!ts l\n!t ");
    row.extend_from_slice("W".repeat(400).as_bytes());
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(&row, 65_536, 786_432, 5_000)
        .expect("writer should accept table marker line");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed for table overflow"
    );
}

#[test]
fn pdf_renderer_figure_block_spacing_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before paragraph.\n\n!gbox\n!gimg 7 figures/demo.png\n!gcap Figure 1: Figure caption text.\n\nAfter paragraph.",
    )
    .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        !pdf_text.contains("!gbox"),
        "figure marker should be hidden in pdf output: {pdf_text}"
    );
    assert!(
        pdf_text.contains("([ Figure placeholder: figures/demo.png ]) Tj"),
        "graphics placeholder text should render with normalized path label: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(Figure 1: Figure caption text.) Tj"),
        "caption text should render: {pdf_text}"
    );

    let epsilon_pt = 0.05f32;
    let left_margin_pt = 72.0f32;
    let left_margin_epsilon = 0.5f32;
    let (before_x, before_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Before paragraph.)").expect("before");
    let (placeholder_x, placeholder_y) =
        tm_position_for_line_containing_text_v0(&pdf, "([ Figure placeholder: figures/demo.png ])")
            .expect("placeholder");
    let (caption_x, caption_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Figure 1: Figure caption text.)")
            .expect("caption");
    let (after_x, after_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After paragraph.)").expect("after");

    assert!(
        placeholder_x > left_margin_pt + left_margin_epsilon,
        "placeholder should be visually offset from body margin"
    );
    assert!(
        caption_x > left_margin_pt + left_margin_epsilon,
        "caption should be visually offset from body margin"
    );
    assert!(
        before_x >= left_margin_pt - epsilon_pt,
        "before paragraph should stay in body column"
    );
    assert!(
        after_x >= left_margin_pt - epsilon_pt,
        "after paragraph should stay in body column"
    );
    assert!(
        before_y > placeholder_y,
        "placeholder should render below body"
    );
    assert!(
        placeholder_y > caption_y,
        "caption should render below placeholder"
    );
    assert!(
        caption_y > after_y,
        "after paragraph should render below caption"
    );
    assert!(
        (placeholder_y - caption_y - 122.0).abs() <= epsilon_pt,
        "placeholder->caption gap should be stable and readable: placeholder_y={placeholder_y}, caption_y={caption_y}"
    );
    assert!(
        (caption_y - after_y - 14.0).abs() <= epsilon_pt,
        "caption->paragraph transition gap should remain stable after figure blocks: caption_y={caption_y}, after_y={after_y}"
    );
}

#[test]
fn pdf_renderer_figure_metadata_width_affects_placeholder_alignment_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"!gbox\n!gimg 1 figures/narrow.png 120000 80000\n!gcap Figure 1: Narrow caption.\n\n!gbox\n!gimg 2 figures/wide.png 360000 240000\n!gcap Figure 2: Wide caption.",
    )
    .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (narrow_x, _) = tm_position_for_line_containing_text_v0(
        &pdf,
        "([ Figure placeholder: figures/narrow.png ])",
    )
    .expect("narrow placeholder position");
    let (wide_x, _) =
        tm_position_for_line_containing_text_v0(&pdf, "([ Figure placeholder: figures/wide.png ])")
            .expect("wide placeholder position");
    assert!(
        wide_x + 1.0 < narrow_x,
        "wider placeholder should start further left: narrow_x={narrow_x}, wide_x={wide_x}"
    );
}

#[test]
fn pdf_renderer_accepts_figure_top_placement_marker_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!gbox t\n!gcap Figure 1: Top caption.")
        .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Figure 1: Top caption.) Tj"),
        "caption text should render for top-placement marker: {pdf_text}"
    );
    assert!(
        pdf_text.contains("([ Figure placeholder ]) Tj"),
        "placeholder text should render for top-placement marker: {pdf_text}"
    );
}

#[test]
fn pdf_renderer_rejects_figure_image_metadata_width_overflow_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"!gbox\n!gimg 1 figures/demo.png 500000 120000\n!gcap Figure 1: Caption.",
    )
    .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed for figure placeholder width overflow"
    );
}

#[test]
fn pdf_renderer_rejects_malformed_figure_image_metadata_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!gbox\n!gimg demo.png\n!gcap Caption")
        .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on malformed !gimg metadata"
    );
}

#[test]
fn pdf_renderer_rejects_malformed_figure_box_placement_marker_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!gbox h\n!gcap Figure 1: Caption.")
        .expect("writer should accept figure marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on malformed !gbox placement hint"
    );
}
