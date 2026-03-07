use super::super::*;

#[test]
fn pdf_renderer_itemize_bullet_and_body_x_offsets_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- alpha\ncontinuation\n- beta\ncontinuationtwo")
        .expect("writer should accept list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(- alpha) Tj".len())
            .any(|w| w == b"(- alpha) Tj"),
        "prefix should be split from body"
    );

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let alpha_xs = tm_xs_for_segment_text_v0(&pdf, "alpha");
    let beta_xs = tm_xs_for_segment_text_v0(&pdf, "beta");
    let continuation_xs = tm_xs_for_segment_text_v0(&pdf, "continuation");
    let continuation_two_xs = tm_xs_for_segment_text_v0(&pdf, "continuationtwo");

    assert_eq!(
        bullet_xs.len(),
        2,
        "expected two bullet renders: {bullet_xs:?}"
    );
    assert_eq!(alpha_xs.len(), 1, "expected alpha render");
    assert_eq!(beta_xs.len(), 1, "expected beta render");
    assert_eq!(continuation_xs.len(), 1, "expected continuation render");
    assert_eq!(
        continuation_two_xs.len(),
        1,
        "expected continuationtwo render"
    );

    let epsilon_pt = 0.02f32;
    for x in [
        alpha_xs[0],
        beta_xs[0],
        continuation_xs[0],
        continuation_two_xs[0],
    ] {
        assert!((x - 96.0).abs() <= epsilon_pt, "item body x mismatch: {x}");
    }
    let target_gap_pt = 8.0f32;
    for (bullet_x, body_x) in bullet_xs.iter().zip([alpha_xs[0], beta_xs[0]]) {
        let marker_gap = body_x - (*bullet_x + segment_width_pt_v0(b"-"));
        assert!(
            (marker_gap - target_gap_pt).abs() <= 0.25,
            "itemize marker/body gap should stay tight and stable: marker_gap={marker_gap}, body_x={body_x}, bullet_x={bullet_x}"
        );
    }
}

#[test]
fn pdf_renderer_enumerate_number_column_alignment_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"9. nine\n10. ten")
        .expect("writer should accept enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(9. nine) Tj".len())
            .any(|w| w == b"(9. nine) Tj"),
        "prefix should be split from body"
    );

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let nine_body_x = tm_xs_for_segment_text_v0(&pdf, "nine");
    let ten_body_x = tm_xs_for_segment_text_v0(&pdf, "ten");

    assert_eq!(nine_number_x.len(), 1, "expected 9. number render");
    assert_eq!(ten_number_x.len(), 1, "expected 10. number render");
    assert_eq!(nine_body_x.len(), 1, "expected nine body render");
    assert_eq!(ten_body_x.len(), 1, "expected ten body render");

    let epsilon_pt = 0.02f32;
    assert!(
        nine_number_x[0] > ten_number_x[0],
        "single-digit number should start further right: nine={:?}, ten={:?}",
        nine_number_x,
        ten_number_x
    );
    assert!(
        (nine_body_x[0] - 96.0).abs() <= epsilon_pt,
        "nine body x mismatch: {}",
        nine_body_x[0]
    );
    assert!(
        (ten_body_x[0] - 96.0).abs() <= epsilon_pt,
        "ten body x mismatch: {}",
        ten_body_x[0]
    );
    let min_gap_pt = 7.5f32;
    let nine_number_right = nine_number_x[0] + segment_width_pt_v0(b"9.");
    let ten_number_right = ten_number_x[0] + segment_width_pt_v0(b"10.");
    assert!(
        nine_body_x[0] - nine_number_right >= min_gap_pt,
        "enumerate gap for 9. should remain readable: body_x={}, number_right={}",
        nine_body_x[0],
        nine_number_right
    );
    assert!(
        ten_body_x[0] - ten_number_right >= min_gap_pt,
        "enumerate gap for 10. should remain readable: body_x={}, number_right={}",
        ten_body_x[0],
        ten_number_right
    );
}

#[test]
fn pdf_renderer_enumerate_number_column_alignment_across_wraps_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"9. [NINESTART] item with enough repeated words to force wrapping and keep deterministic body indent for continuation lines before token [WRAPNINE]\n10. [TENSTART] item with enough repeated words to force wrapping and keep deterministic body indent for continuation lines before token [WRAPTEN]")
        .expect("writer should accept long enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let nine_start_x = tm_xs_for_segment_text_v0(&pdf, "NINESTART");
    let ten_start_x = tm_xs_for_segment_text_v0(&pdf, "TENSTART");
    let nine_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "WRAPNINE");
    let ten_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "WRAPTEN");

    assert_eq!(nine_number_x.len(), 1, "expected 9. number render");
    assert_eq!(ten_number_x.len(), 1, "expected 10. number render");
    assert_eq!(nine_start_x.len(), 1, "expected NINESTART render");
    assert_eq!(ten_start_x.len(), 1, "expected TENSTART render");
    assert_eq!(nine_wrap_x.len(), 1, "expected WRAPNINE render");
    assert_eq!(ten_wrap_x.len(), 1, "expected WRAPTEN render");

    let epsilon_pt = 0.02f32;
    assert!(
        nine_number_x[0] > ten_number_x[0],
        "single-digit number should start further right: nine={:?}, ten={:?}",
        nine_number_x,
        ten_number_x
    );
    assert!(
        (nine_start_x[0] - 96.0).abs() <= epsilon_pt,
        "start body x mismatch for 9.: {}",
        nine_start_x[0]
    );
    assert!(
        (ten_start_x[0] - 96.0).abs() <= epsilon_pt,
        "start body x mismatch for 10.: {}",
        ten_start_x[0]
    );
    assert!(
        (nine_wrap_x[0] - nine_start_x[0]).abs() <= epsilon_pt,
        "wrap body x mismatch for 9.: start={}, wrap={}",
        nine_start_x[0],
        nine_wrap_x[0]
    );
    assert!(
        (ten_wrap_x[0] - ten_start_x[0]).abs() <= epsilon_pt,
        "wrap body x mismatch for 10.: start={}, wrap={}",
        ten_start_x[0],
        ten_wrap_x[0]
    );
    let min_gap_pt = 7.5f32;
    let nine_number_right = nine_number_x[0] + segment_width_pt_v0(b"9.");
    let ten_number_right = ten_number_x[0] + segment_width_pt_v0(b"10.");
    assert!(
        nine_start_x[0] - nine_number_right >= min_gap_pt,
        "wrapped enumerate gap for 9. should remain readable: body_x={}, number_right={}",
        nine_start_x[0],
        nine_number_right
    );
    assert!(
        ten_start_x[0] - ten_number_right >= min_gap_pt,
        "wrapped enumerate gap for 10. should remain readable: body_x={}, number_right={}",
        ten_start_x[0],
        ten_number_right
    );
}

#[test]
fn pdf_renderer_mixed_list_block_transition_and_marker_spacing_invariants_v20() {
    let xdv = write_dvi_v2_text_page_v0(b"\nParagraph before lists.\n\n- BULLETSTART alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha BULLETWRAP\n\n10. ENUMSTART beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta ENUMWRAP\n\nParagraph after lists.")
        .expect("writer should accept mixed list transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_lists_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before lists.)")
            .expect("before list paragraph");
    let (bullet_start_x, bullet_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BULLETSTART").expect("bullet start");
    let (bullet_wrap_x, bullet_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "BULLETWRAP").expect("bullet wrap");
    let (enum_start_x, enum_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ENUMSTART").expect("enum start");
    let (enum_wrap_x, enum_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "ENUMWRAP").expect("enum wrap");
    let (_, after_lists_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after lists.)")
        .expect("after list paragraph");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let enum_number_xs = tm_xs_for_segment_text_v0(&pdf, "10.");
    assert_eq!(bullet_xs.len(), 1, "expected one bullet marker: {bullet_xs:?}");
    assert_eq!(
        enum_number_xs.len(),
        1,
        "expected one enumerate marker: {enum_number_xs:?}"
    );

    let epsilon_pt = 0.2f32;
    assert!(
        (before_lists_y - bullet_start_y - 24.0).abs() <= epsilon_pt,
        "paragraph->list transition should remain tightened: before_lists_y={before_lists_y}, bullet_start_y={bullet_start_y}"
    );
    assert!(
        (bullet_wrap_y - enum_start_y - 24.0).abs() <= epsilon_pt,
        "list->list block transition should remain even: bullet_wrap_y={bullet_wrap_y}, enum_start_y={enum_start_y}"
    );
    assert!(
        (enum_wrap_y - after_lists_y - 24.0).abs() <= epsilon_pt,
        "list->paragraph transition should remain tightened: enum_wrap_y={enum_wrap_y}, after_lists_y={after_lists_y}"
    );
    assert!(
        (bullet_start_y - bullet_wrap_y - 13.0).abs() <= epsilon_pt,
        "itemize wrapped-line rhythm mismatch: bullet_start_y={bullet_start_y}, bullet_wrap_y={bullet_wrap_y}"
    );
    assert!(
        (enum_start_y - enum_wrap_y - 13.0).abs() <= epsilon_pt,
        "enumerate wrapped-line rhythm mismatch: enum_start_y={enum_start_y}, enum_wrap_y={enum_wrap_y}"
    );

    assert!(
        (bullet_start_x - enum_start_x).abs() <= 0.02,
        "mixed list body columns should stay aligned: bullet_start_x={bullet_start_x}, enum_start_x={enum_start_x}"
    );
    assert!(
        (bullet_wrap_x - bullet_start_x).abs() <= 0.02
            && (enum_wrap_x - enum_start_x).abs() <= 0.02,
        "wrapped list continuation x should stay aligned: bullet_start_x={bullet_start_x}, bullet_wrap_x={bullet_wrap_x}, enum_start_x={enum_start_x}, enum_wrap_x={enum_wrap_x}"
    );
    let bullet_gap = bullet_start_x - (bullet_xs[0] + segment_width_pt_v0(b"-"));
    let enum_gap = enum_start_x - (enum_number_xs[0] + segment_width_pt_v0(b"10."));
    assert!(
        (bullet_gap - enum_gap).abs() <= 0.25,
        "mixed itemize/enumerate marker/body gap drift: bullet_gap={bullet_gap}, enum_gap={enum_gap}"
    );
}

#[test]
fn pdf_renderer_nested_mixed_width_enumerate_wrap_alignment_invariants_v20() {
    let xdv = write_dvi_v2_text_page_v0(b"- Outer item.\n  9. NINESTART gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma NINEWRAP\n  10. TENSTART delta delta delta delta delta delta delta delta delta delta delta delta delta delta delta delta TENWRAP")
        .expect("writer should accept nested mixed-width enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nine_number_x = tm_xs_for_segment_text_v0(&pdf, "9.");
    let ten_number_x = tm_xs_for_segment_text_v0(&pdf, "10.");
    let (nine_start_x, nine_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "NINESTART").expect("nine start");
    let (nine_wrap_x, nine_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "NINEWRAP").expect("nine wrap");
    let (ten_start_x, ten_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "TENSTART").expect("ten start");
    let (ten_wrap_x, ten_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "TENWRAP").expect("ten wrap");

    assert_eq!(nine_number_x.len(), 1, "expected one 9. marker");
    assert_eq!(ten_number_x.len(), 1, "expected one 10. marker");

    let epsilon_pt = 0.2f32;
    assert!(
        (nine_start_x - ten_start_x).abs() <= 0.02,
        "nested enumerate body column should stay aligned across mixed-width markers: nine_start_x={nine_start_x}, ten_start_x={ten_start_x}"
    );
    assert!(
        (nine_start_x - nine_wrap_x).abs() <= 0.02 && (ten_start_x - ten_wrap_x).abs() <= 0.02,
        "nested wrapped continuation x should remain stable: nine_start_x={nine_start_x}, nine_wrap_x={nine_wrap_x}, ten_start_x={ten_start_x}, ten_wrap_x={ten_wrap_x}"
    );
    let nine_gap = nine_start_x - (nine_number_x[0] + segment_width_pt_v0(b"9."));
    let ten_gap = ten_start_x - (ten_number_x[0] + segment_width_pt_v0(b"10."));
    assert!(
        (nine_gap - ten_gap).abs() <= 0.25,
        "nested enumerate marker/body gap drift across mixed-width markers: nine_gap={nine_gap}, ten_gap={ten_gap}"
    );
    assert!(
        (nine_start_y - nine_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested 9. wrapped-line rhythm mismatch: nine_start_y={nine_start_y}, nine_wrap_y={nine_wrap_y}"
    );
    assert!(
        (ten_start_y - ten_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested 10. wrapped-line rhythm mismatch: ten_start_y={ten_start_y}, ten_wrap_y={ten_wrap_y}"
    );
    assert!(
        (nine_wrap_y - ten_start_y - 13.0).abs() <= epsilon_pt,
        "nested enumerate entry-to-entry rhythm mismatch: nine_wrap_y={nine_wrap_y}, ten_start_y={ten_start_y}"
    );
}

#[test]
fn pdf_renderer_figure_caption_and_adjacent_table_transition_rhythm_v21() {
    let xdv = write_dvi_v2_text_page_v0(b"\nParagraph before blocks.\n\n!gbox\n!gcap Figure 1: CAPSTART [dense], {styled} caption text with punctuation, continuity check.\n\n!ts ll\n!t ROWONEA||ROWONEB\n!t ROWTWOA||ROWTWOB\n\nParagraph after blocks.")
        .expect("writer should accept figure-table transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, caption_y) =
        tm_position_for_segment_substring_v0(&pdf, "CAPSTART").expect("caption start");
    let (_, row_one_y) = tm_position_for_segment_substring_v0(&pdf, "ROWONEA").expect("row one");
    let (_, row_two_y) = tm_position_for_segment_substring_v0(&pdf, "ROWTWOA").expect("row two");
    let (_, after_blocks_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after blocks.)")
            .expect("paragraph after blocks");

    let epsilon_pt = 0.2f32;
    assert!(
        (caption_y - row_one_y - 37.0).abs() <= epsilon_pt,
        "figure-caption->table transition gap should stay tightened: caption_y={caption_y}, row_one_y={row_one_y}"
    );
    assert!(
        (row_one_y - row_two_y - 13.0).abs() <= epsilon_pt,
        "table row leading should remain stable after caption transition: row_one_y={row_one_y}, row_two_y={row_two_y}"
    );
    assert!(
        (row_two_y - after_blocks_y - 24.0).abs() <= epsilon_pt,
        "table->paragraph transition should stay tightened: row_two_y={row_two_y}, after_blocks_y={after_blocks_y}"
    );
    let max_caption_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CAPSTART")
        .expect("caption line should render and expose tm gaps");
    assert!(
        max_caption_tm_gap <= 18.0,
        "caption styled seam spacing should remain bounded: max_caption_tm_gap={max_caption_tm_gap}"
    );
}

#[test]
fn pdf_renderer_applies_quote_indent_and_hides_prefix_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n> quoted line\ncontinuation line")
        .expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(!pdf
        .windows(b"(> quoted line) Tj".len())
        .any(|w| w == b"(> quoted line) Tj"));
    assert!(pdf
        .windows(b"(quoted line) Tj".len())
        .any(|w| w == b"(quoted line) Tj"));

    let pdf_text = String::from_utf8_lossy(&pdf);
    let mut xs = Vec::<f32>::new();
    for line in pdf_text.lines() {
        if !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        if let Ok(x_pt) = fields[4].parse::<f32>() {
            xs.push(x_pt);
        }
    }
    assert!(
        xs.len() >= 2,
        "expected at least two rendered lines, got {xs:?}"
    );
    assert!(
        (xs[0] - 104.0).abs() <= 0.02,
        "quote line indent should be stable and deeper than body indent: {xs:?}"
    );
    assert!(
        (xs[0] - xs[1]).abs() <= 0.02,
        "quote continuation should keep indent: {xs:?}"
    );
}

#[test]
fn pdf_renderer_quote_indent_and_paragraph_break_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n> quote first line\n> quote continuation\n\n> second paragraph line\n> second continuation",
    )
    .expect("writer should accept quote text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let text = String::from_utf8_lossy(&pdf);
    assert!(
        !text.contains("(> quote first line) Tj"),
        "quote prefix should be hidden"
    );
    let (x1, y1) =
        tm_position_for_line_containing_text_v0(&pdf, "(quote first line)").expect("line 1");
    let (x2, y2) =
        tm_position_for_line_containing_text_v0(&pdf, "(quote continuation)").expect("line 2");
    let (x3, y3) =
        tm_position_for_line_containing_text_v0(&pdf, "(second paragraph line)").expect("line 3");
    let (x4, y4) =
        tm_position_for_line_containing_text_v0(&pdf, "(second continuation)").expect("line 4");

    let epsilon_pt = 0.02f32;
    assert!(
        (x1 - 104.0).abs() <= epsilon_pt,
        "quote indent baseline mismatch: {x1}"
    );
    assert!(
        (x1 - x2).abs() <= epsilon_pt,
        "quote line x drift: {x1} vs {x2}"
    );
    assert!(
        (x1 - x3).abs() <= epsilon_pt,
        "quote paragraph x drift: {x1} vs {x3}"
    );
    assert!(
        (x1 - x4).abs() <= epsilon_pt,
        "quote line x drift: {x1} vs {x4}"
    );
    assert!(
        (y1 - y2 - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch"
    );
    assert!(
        (y2 - y3 - 26.5).abs() <= epsilon_pt,
        "quote paragraph gap mismatch"
    );
    assert!(
        (y3 - y4 - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch"
    );
}

#[test]
fn pdf_renderer_paragraph_quote_transition_spacing_polish_v7() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\nParagraph before quote.\n\n> QUOTESTART quote quote quote quote quote quote quote quote quote quote quote quote QUOTEWRAP\n\nParagraph after quote.",
    )
    .expect("writer should accept paragraph-quote transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before quote.)")
            .expect("before quote paragraph");
    let (quote_start_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote start");
    let (quote_wrap_x, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAP").expect("quote wrap");
    let (_, after_quote_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after quote.)")
        .expect("after quote paragraph");

    let epsilon_pt = 0.02f32;
    assert!(
        (before_quote_y - quote_start_y - 23.0).abs() <= epsilon_pt,
        "paragraph->quote transition gap mismatch: before_quote_y={before_quote_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_wrap_y - 12.5).abs() <= epsilon_pt,
        "quote wrapped-line rhythm mismatch: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
    assert!(
        (quote_wrap_y - after_quote_y - 23.0).abs() <= epsilon_pt,
        "quote->paragraph transition gap mismatch: quote_wrap_y={quote_wrap_y}, after_quote_y={after_quote_y}"
    );
    assert!(
        (quote_start_x - quote_wrap_x).abs() <= epsilon_pt,
        "wrapped quote continuation should preserve quote indent: quote_start_x={quote_start_x}, quote_wrap_x={quote_wrap_x}"
    );
}

#[test]
fn pdf_renderer_mixed_paragraph_quote_list_transition_spacing_polish_v19() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\nParagraph before quote.\n\n> QUOTESTART quote quote quote quote quote quote quote quote quote quote quote quote QUOTEWRAP\n\n- List after quote line.\n- List continuation line.\n\nParagraph after list.",
    )
    .expect("writer should accept mixed paragraph/quote/list transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before quote.)")
            .expect("before quote paragraph");
    let (quote_start_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote start");
    let (quote_wrap_x, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAP").expect("quote wrap");
    let (list_start_x, list_start_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List after quote line.)")
            .expect("list after quote");
    let (_, list_next_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List continuation line.)")
            .expect("list continuation");
    let (_, after_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after list.)")
            .expect("after list paragraph");

    let epsilon_pt = 0.05f32;
    assert!(
        (before_quote_y - quote_start_y - 23.0).abs() <= epsilon_pt,
        "paragraph->quote transition gap mismatch: before_quote_y={before_quote_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_wrap_y - 12.5).abs() <= epsilon_pt,
        "quote wrapped-line rhythm mismatch: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
    assert!(
        (quote_wrap_y - list_start_y - 23.0).abs() <= epsilon_pt,
        "quote->list transition gap mismatch: quote_wrap_y={quote_wrap_y}, list_start_y={list_start_y}"
    );
    assert!(
        (list_start_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm mismatch: list_start_y={list_start_y}, list_next_y={list_next_y}"
    );
    assert!(
        (list_next_y - after_list_y - 24.0).abs() <= epsilon_pt,
        "list->paragraph transition gap mismatch: list_next_y={list_next_y}, after_list_y={after_list_y}"
    );
    assert!(
        (quote_start_x - quote_wrap_x).abs() <= epsilon_pt,
        "wrapped quote continuation should preserve quote indent: quote_start_x={quote_start_x}, quote_wrap_x={quote_wrap_x}"
    );
    assert!(
        quote_start_x >= list_start_x + 6.0,
        "quote indent should remain visibly deeper than list body indent: quote_start_x={quote_start_x}, list_start_x={list_start_x}"
    );
}

#[test]
fn pdf_renderer_hides_center_prefix_and_centers_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ centered line")
        .expect("writer should accept centered text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let centered_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| width_sp_for_prefixed_rendered_line_v0(line, [b'^', b' ']).is_some())
        .expect("center-prefixed line");
    let expected_x = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(centered_line, [b'^', b' '])
            .expect("prefixed width"),
    );
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(!pdf
        .windows(b"(^ centered line) Tj".len())
        .any(|w| w == b"(^ centered line) Tj"));
    assert!(pdf
        .windows(b"(centered line) Tj".len())
        .any(|w| w == b"(centered line) Tj"));
    let x_pt =
        tm_x_for_line_containing_text_v0(&pdf, "(centered line)").expect("centered Tm position");
    let epsilon_pt = 0.02f32;
    assert!(
        (x_pt - expected_x).abs() <= epsilon_pt,
        "center line x mismatch: actual={x_pt}, expected={expected_x}"
    );
}

#[test]
fn pdf_renderer_single_line_quote_and_list_styled_seams_use_v31_profile() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n- LISTLINEV31 alpha [LISTITALICV31] tail.\n\n> QUOTELINEV31 beta {QUOTEBOLDV31} tail.",
    )
    .expect("writer should accept single-line quote/list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let list_line = pdf_text
        .lines()
        .find(|line| line.contains("(LISTITALICV31) Tj"))
        .expect("single-line list styled segment should render");
    let quote_line = pdf_text
        .lines()
        .find(|line| line.contains("(QUOTEBOLDV31) Tj"))
        .expect("single-line quote styled segment should render");

    assert!(
        list_line.contains("97 Tz") && list_line.contains("(LISTITALICV31) Tj 100 Tz"),
        "single-line list styled segment should use indented seam compensation"
    );
    assert!(
        quote_line.contains("95 Tz") && quote_line.contains("(QUOTEBOLDV31) Tj 100 Tz"),
        "single-line quote styled segment should use indented seam compensation"
    );
}

#[test]
fn pdf_renderer_single_line_quote_and_list_pre_style_gaps_are_tightened_v33() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n- LISTPREV33 with [LISTITALICPREV33] tail.\n\n> QUOTEPREV33 with {QUOTEBOLDPREV33} tail.",
    )
    .expect("writer should accept single-line quote/list v33 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let list_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(LISTITALICPREV33)",
        "(LISTPREV33 with )",
    )
    .expect("list prefix x");
    let list_italic_x =
        tm_x_for_segment_substring_v0(&pdf, "(LISTITALICPREV33)", "(LISTITALICPREV33)")
            .expect("list italic x");
    let quote_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(QUOTEBOLDPREV33)",
        "(QUOTEPREV33 with )",
    )
    .expect("quote prefix x");
    let quote_bold_x =
        tm_x_for_segment_substring_v0(&pdf, "(QUOTEBOLDPREV33)", "(QUOTEBOLDPREV33)")
            .expect("quote bold x");

    let expected_list_gap = segment_width_pt_v0(b"LISTPREV33 with ") - (12.0 * 0.12);
    let expected_quote_gap = segment_width_pt_v0(b"QUOTEPREV33 with ") - (12.0 * 0.15);
    let epsilon_pt = 0.3f32;
    assert!(
        ((list_italic_x - list_prefix_x) - expected_list_gap).abs() <= epsilon_pt,
        "single-line list pre-style seam should trim the preceding space-bounded gap: prefix_x={list_prefix_x}, italic_x={list_italic_x}, expected_gap={expected_list_gap}"
    );
    assert!(
        ((quote_bold_x - quote_prefix_x) - expected_quote_gap).abs() <= epsilon_pt,
        "single-line quote pre-style seam should trim the preceding space-bounded gap: prefix_x={quote_prefix_x}, bold_x={quote_bold_x}, expected_gap={expected_quote_gap}"
    );
}

#[test]
fn pdf_renderer_quote_and_list_continuation_pre_style_short_gaps_are_tightened_v36() {
    let xdv = write_dvi_v2_text_page_v0(
        b"- LISTSTART36 alpha alpha alpha alpha alpha\nwith [LISTWRAPPREV36] tail.\n\n> QUOTESTART36 beta beta beta beta beta\nwith {QUOTEWRAPPREV36} tail.",
    )
    .expect("writer should accept quote/list continuation v36 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (list_start_x, list_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTSTART36").expect("list start position");
    let (list_style_x, list_style_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTWRAPPREV36").expect("list style position");
    let list_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(LISTWRAPPREV36)",
        "(with )",
    )
    .expect("list wrapped prefix x");

    let (quote_start_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART36").expect("quote start position");
    let (quote_style_x, quote_style_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAPPREV36").expect("quote style position");
    let quote_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(QUOTEWRAPPREV36)",
        "(with )",
    )
    .expect("quote wrapped prefix x");

    let with_gap = segment_width_pt_v0(b"with ");
    let expected_list_gap = with_gap - (12.0f32 * 0.12f32).min(with_gap * 0.25f32);
    let expected_quote_gap = with_gap - (12.0f32 * 0.15f32).min(with_gap * 0.25f32);
    let epsilon_pt = 0.75f32;
    assert!(
        list_start_y > list_style_y && quote_start_y > quote_style_y,
        "fixtures should continue onto a later indented line before the styled tokens: list_start_y={list_start_y}, list_style_y={list_style_y}, quote_start_y={quote_start_y}, quote_style_y={quote_style_y}"
    );
    assert!(
        list_style_x >= list_start_x && ((list_style_x - list_prefix_x) - expected_list_gap).abs() <= epsilon_pt,
        "list continuation pre-style short seam should trim the preceding gap: prefix_x={list_prefix_x}, style_x={list_style_x}, expected_gap={expected_list_gap}"
    );
    assert!(
        quote_style_x >= quote_start_x && ((quote_style_x - quote_prefix_x) - expected_quote_gap).abs() <= epsilon_pt,
        "quote continuation pre-style short seam should trim the preceding gap: prefix_x={quote_prefix_x}, style_x={quote_style_x}, expected_gap={expected_quote_gap}"
    );
}

#[test]
fn pdf_renderer_live_list_long_prefix_pre_style_gaps_are_tightened_v37() {
    let xdv = write_dvi_v2_text_page_v0(
        b"- First bullet with [emphasis] and a deliberately long sentence to force wrapping so body alignment remains stable across continuation lines in the preview renderer while keeping punctuation,wrapper boundaries near deterministic wrap points.\n- Second bullet with {bold} plus another long continuation sentence that should wrap without shifting the bullet column and should still preserve deterministic hanging indentation between wrapped lines.\n  - Another nested bullet with [styled words] near punctuation,like this, to stress inline wrappers inside list content.",
    )
    .expect("writer should accept live list seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let list_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(emphasis)", "(First bullet with )")
            .expect("list prefix x");
    let list_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(emphasis)", "(emphasis)").expect("list style x");
    let bold_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold)", "(Second bullet with )")
            .expect("bold prefix x");
    let bold_style_x = tm_x_for_segment_substring_v0(&pdf, "(bold)", "(bold)")
        .expect("bold style x");

    assert!(
        list_style_x - list_prefix_x <= 14.0,
        "list long-prefix pre-style seam should stay tightened: prefix_x={list_prefix_x}, style_x={list_style_x}"
    );
    assert!(
        bold_style_x - bold_prefix_x <= 15.0,
        "bold long-prefix pre-style seam should stay tightened: prefix_x={bold_prefix_x}, style_x={bold_style_x}"
    );
}

#[test]
fn pdf_renderer_live_nested_list_very_long_prefix_gap_is_tightened_v38() {
    let xdv = write_dvi_v2_text_page_v0(
        b"  - Another nested bullet with [styled words] near punctuation,like this, to stress inline wrappers inside list content.",
    )
    .expect("writer should accept live nested seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let nested_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(styled words)", "(Another nested bullet with )")
            .expect("nested prefix x");
    let nested_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(styled words)", "(styled words)")
            .expect("nested style x");

    assert!(
        nested_style_x - nested_prefix_x <= 150.0,
        "nested long-prefix pre-style seam should stay tightened: prefix_x={nested_prefix_x}, style_x={nested_style_x}"
    );
}

#[test]
fn pdf_renderer_live_quote_very_long_prefix_gap_is_tightened_v39() {
    let xdv = write_dvi_v2_text_page_v0(
        b"> This quoted paragraph is intentionally long so width-based wrapping produces continuation lines in the preview while preserving a deterministic left indent for each logical quote line with [inline emphasis] and {bold emphasis}.",
    )
    .expect("writer should accept live quote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let quote_line_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "inline emphasis").expect("quote tm gap");

    assert!(
        quote_line_tm_gap <= 315.0,
        "quote very-long-prefix pre-style seam should stay tightened: tm_gap={quote_line_tm_gap}"
    );
}

#[test]
fn pdf_renderer_live_quote_medium_prefix_gap_is_tightened_v45() {
    let xdv = write_dvi_v2_text_page_v0(
        b"> Quote prefix with [inline words] and compact trailing text.",
    )
    .expect("writer should accept medium quote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let quote_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(Quote prefix with )")
            .expect("medium quote prefix x");
    let quote_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium quote style x");

    assert!(
        quote_style_x - quote_prefix_x <= 88.0,
        "quote medium-prefix pre-style seam should stay tightened: prefix_x={quote_prefix_x}, style_x={quote_style_x}"
    );
}

#[test]
fn pdf_renderer_live_quote_medium_bold_prefix_gap_is_tightened_v47() {
    let xdv = write_dvi_v2_text_page_v0(
        b"> Quote prefix with {bold words} and compact trailing text.",
    )
    .expect("writer should accept medium bold quote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let quote_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(Quote prefix with )")
            .expect("medium bold quote prefix x");
    let quote_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold quote style x");

    assert!(
        quote_style_x - quote_prefix_x <= 90.0,
        "quote medium-bold-prefix seam should stay tightened: prefix_x={quote_prefix_x}, style_x={quote_style_x}"
    );
}

#[test]
fn pdf_renderer_live_quote_medium_bold_prefix_gap_is_tightened_v89() {
    let xdv = write_dvi_v2_text_page_v0(
        b"> Quote prefix with {bold words} and compact trailing text.",
    )
    .expect("writer should accept medium bold quote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let quote_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(Quote prefix with )")
            .expect("medium bold quote prefix x");
    let quote_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold quote style x");

    assert!(
        quote_style_x - quote_prefix_x <= 89.0,
        "quote medium-bold-prefix seam should stay slightly tighter after v89: prefix_x={quote_prefix_x}, style_x={quote_style_x}"
    );
}

#[test]
fn pdf_renderer_live_quote_medium_inline_prefix_gap_is_tightened_v92() {
    let xdv = write_dvi_v2_text_page_v0(
        b"> Quote prefix with [inline words] and compact trailing text.",
    )
    .expect("writer should accept medium quote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let quote_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(Quote prefix with )")
            .expect("medium quote prefix x");
    let quote_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium quote style x");

    assert!(
        quote_style_x - quote_prefix_x <= 87.0,
        "quote medium-prefix seam should stay slightly tighter after v92: prefix_x={quote_prefix_x}, style_x={quote_style_x}"
    );
}

#[test]
fn pdf_renderer_live_list_medium_prefix_gap_is_tightened_v46() {
    let xdv = write_dvi_v2_text_page_v0(
        b"- List prefix with [inline words] and compact trailing text.",
    )
    .expect("writer should accept medium list seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let list_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(List prefix with )")
            .expect("medium list prefix x");
    let list_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium list style x");

    assert!(
        list_style_x - list_prefix_x <= 84.0,
        "list medium-prefix pre-style seam should stay tightened: prefix_x={list_prefix_x}, style_x={list_style_x}"
    );
}

#[test]
fn pdf_renderer_live_list_medium_bold_prefix_gap_is_tightened_v48() {
    let xdv = write_dvi_v2_text_page_v0(
        b"- List prefix with {bold words} and compact trailing text.",
    )
    .expect("writer should accept medium bold list seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let list_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(List prefix with )")
            .expect("medium bold list prefix x");
    let list_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold list style x");

    assert!(
        list_style_x - list_prefix_x <= 86.0,
        "list medium-bold-prefix seam should stay tightened: prefix_x={list_prefix_x}, style_x={list_style_x}"
    );
}

#[test]
fn pdf_renderer_live_list_medium_bold_prefix_gap_is_tightened_v90() {
    let xdv = write_dvi_v2_text_page_v0(
        b"- List prefix with {bold words} and compact trailing text.",
    )
    .expect("writer should accept medium bold list seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let list_prefix_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(List prefix with )")
            .expect("medium bold list prefix x");
    let list_style_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold list style x");

    assert!(
        list_style_x - list_prefix_x <= 85.0,
        "list medium-bold-prefix seam should stay slightly tighter after v90: prefix_x={list_prefix_x}, style_x={list_style_x}"
    );
}
