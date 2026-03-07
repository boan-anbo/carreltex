use super::super::*;

#[test]
fn pdf_renderer_table_to_figure_caption_separation_rhythm_v21() {
    let xdv = write_dvi_v2_text_page_v0(b"\n!ts ll\n!t TROWONEA||TROWONEB\n!t TROWTWOA||TROWTWOB\n\n!gbox\n!gcap Figure 1: FIGCAPSTART compact caption text.\n\nParagraph after figure.")
        .expect("writer should accept table-figure transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, row_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "TROWTWOA").expect("table row two");
    let (_, caption_y) =
        tm_position_for_segment_substring_v0(&pdf, "FIGCAPSTART").expect("figure caption");
    let (_, after_figure_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after figure.)")
            .expect("paragraph after figure");

    let epsilon_pt = 0.2f32;
    assert!(
        (row_two_y - caption_y - 156.0).abs() <= epsilon_pt,
        "table->figure transition should keep stable block/caption separation: row_two_y={row_two_y}, caption_y={caption_y}"
    );
    assert!(
        (caption_y - after_figure_y - 24.0).abs() <= epsilon_pt,
        "figure-caption->paragraph transition should stay tightened: caption_y={caption_y}, after_figure_y={after_figure_y}"
    );
}

#[test]
fn pdf_renderer_bibliography_entries_use_hanging_indent_and_stable_rhythm_v14() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] ALPHASTART alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha ALPHAWRAP\n[12] BETASTART beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta beta BETAWRAP",
    )
    .expect("writer should accept bibliography-style lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (references_x, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references heading");
    let (alpha_start_x, alpha_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHASTART").expect("alpha start");
    let (alpha_wrap_x, alpha_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHAWRAP").expect("alpha wrap");
    let (beta_start_x, beta_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BETASTART").expect("beta start");
    let (beta_wrap_x, beta_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "BETAWRAP").expect("beta wrap");

    let one_label_x = *tm_xs_for_segment_text_v0(&pdf, "1")
        .first()
        .expect("label [1] x");
    let twelve_label_x = *tm_xs_for_segment_text_v0(&pdf, "12")
        .first()
        .expect("label [12] x");
    let one_label_right = one_label_x + segment_width_pt_v0(b"1");
    let twelve_label_right = twelve_label_x + segment_width_pt_v0(b"12");

    let epsilon_pt = 0.2f32;
    assert!(
        (references_y - alpha_start_y - 12.0).abs() <= epsilon_pt,
        "references heading -> first bibliography entry gap should be tightened and stable: references_y={references_y}, alpha_start_y={alpha_start_y}"
    );
    assert!(
        (alpha_start_y - alpha_wrap_y - 12.5).abs() <= epsilon_pt,
        "bibliography wrapped line rhythm should be stable: alpha_start_y={alpha_start_y}, alpha_wrap_y={alpha_wrap_y}"
    );
    assert!(
        (alpha_wrap_y - beta_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography entry-to-entry rhythm should be stable: alpha_wrap_y={alpha_wrap_y}, beta_start_y={beta_start_y}"
    );
    assert!(
        (beta_start_y - beta_wrap_y - 12.5).abs() <= epsilon_pt,
        "bibliography wrapped line rhythm should be stable for later entries: beta_start_y={beta_start_y}, beta_wrap_y={beta_wrap_y}"
    );
    assert!(
        (alpha_start_x - beta_start_x).abs() <= epsilon_pt,
        "bibliography body column should remain stable across mixed-width ordinals: alpha_start_x={alpha_start_x}, beta_start_x={beta_start_x}"
    );
    assert!(
        (alpha_start_x - alpha_wrap_x).abs() <= epsilon_pt
            && (beta_start_x - beta_wrap_x).abs() <= epsilon_pt,
        "bibliography wrapped continuation lines should keep hanging-indent column"
    );
    assert!(
        (one_label_right - twelve_label_right).abs() <= epsilon_pt,
        "bibliography ordinal label right edge should stay aligned: one_label_right={one_label_right}, twelve_label_right={twelve_label_right}"
    );
    assert!(
        references_x >= 72.0,
        "references heading should remain inside printable area"
    );
}

#[test]
fn pdf_renderer_bibliography_styled_seams_use_indented_profile_v32() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] BIBSTART [ITALICBIBV32] with {BOLDBIBV32} tail.",
    )
    .expect("writer should accept bibliography seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICBIBV32) Tj"))
        .expect("bibliography italic line should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDBIBV32) Tj"))
        .expect("bibliography bold line should render");

    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICBIBV32) Tj 100 Tz"),
        "bibliography italic seam should use indented seam compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDBIBV32) Tj 100 Tz"),
        "bibliography bold seam should use indented seam compensation"
    );
}

#[test]
fn pdf_renderer_live_bibliography_long_prefix_gaps_are_tightened_v42() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] First bibliography entry with [inline emphasis] and deliberately dense source wording so the opening styled seam remains visually controlled in the preview renderer.\n[12] Second bibliography entry with {bold emphasis} plus additional source wording to keep the mixed-width ordinal column stable while preserving compact wrapped bibliography rhythm.",
    )
    .expect("writer should accept live bibliography seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let inline_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(inline emphasis)",
        "(First bibliography entry with )",
    )
    .expect("bibliography inline prefix x");
    let inline_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline emphasis)", "(inline emphasis)")
            .expect("bibliography inline style x");
    let bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(bold emphasis)",
        "(Second bibliography entry with )",
    )
    .expect("bibliography bold prefix x");
    let bold_x = tm_x_for_segment_substring_v0(&pdf, "(bold emphasis)", "(bold emphasis)")
        .expect("bibliography bold style x");

    assert!(
        inline_x - inline_prefix_x <= 215.0,
        "bibliography long-prefix italic seam should stay tightened: prefix_x={inline_prefix_x}, style_x={inline_x}"
    );
    assert!(
        bold_x - bold_prefix_x <= 225.0,
        "bibliography long-prefix bold seam should stay tightened: prefix_x={bold_prefix_x}, style_x={bold_x}"
    );
}

#[test]
fn pdf_renderer_live_bibliography_medium_prefix_gaps_are_tightened_v44() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] Bibliography prefix with [inline words].\n[12] Second source prefix with {bold words}.",
    )
    .expect("writer should accept medium bibliography seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let inline_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(inline words)",
        "(Bibliography prefix with )",
    )
    .expect("medium bibliography inline prefix x");
    let inline_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium bibliography inline style x");
    let bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(bold words)",
        "(Second source prefix with )",
    )
    .expect("medium bibliography bold prefix x");
    let bold_x = tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
        .expect("medium bibliography bold style x");

    assert!(
        inline_x - inline_prefix_x <= 128.0,
        "bibliography medium-prefix italic seam should stay tightened: prefix_x={inline_prefix_x}, style_x={inline_x}"
    );
    assert!(
        bold_x - bold_prefix_x <= 136.0,
        "bibliography medium-prefix bold seam should stay tightened: prefix_x={bold_prefix_x}, style_x={bold_x}"
    );
}

#[test]
fn pdf_renderer_body_to_bibliography_opening_gap_is_tightened_v17() {
    let xdv = write_dvi_v2_text_page_v0(
        b"~ Body before references.\n\n@S {References}\n\n[1] ALPHASTART alpha source text.",
    )
    .expect("writer should accept bibliography transition bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body before references.)")
        .expect("body before references");
    let (_, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references heading");
    let (_, alpha_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHASTART").expect("first bibliography body");

    let epsilon_pt = 0.05f32;
    assert!(
        (body_y - references_y - 24.0).abs() <= epsilon_pt,
        "body->bibliography heading transition should stay tightened: body_y={body_y}, references_y={references_y}"
    );
    assert!(
        (references_y - alpha_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography heading->first entry gap should remain stable: references_y={references_y}, alpha_start_y={alpha_start_y}"
    );
}

#[test]
fn pdf_renderer_mixed_surface_quote_table_list_bibliography_flow_rhythm_v25() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Front Matter Title\nAuthor Name\n2026-03-05\n\n> QUOTESTART quote opening line.\n> QUOTECONT quote continuation line.\n\n!ts ll\n!t TABSTARTA||TABSTARTB\n!t TABNEXTA||TABNEXTB\n\n- LISTSTART list opening line.\n- LISTNEXT list continuation line.\n\n@S {References}\n\n[1] BIBSTART alpha source text.",
    )
    .expect("writer should accept mixed-surface v25 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) = tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)")
        .expect("date");
    let (quote_x, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("quote opening");
    let (_, quote_cont_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTECONT").expect("quote continuation");
    let (_, table_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "TABSTARTA").expect("table opening");
    let (_, table_next_y) =
        tm_position_for_segment_substring_v0(&pdf, "TABNEXTA").expect("table continuation");
    let (list_x, list_start_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(LISTSTART list opening line.)")
            .expect("list opening");
    let (_, list_next_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(LISTNEXT list continuation line.)")
            .expect("list continuation");
    let (_, references_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(References)").expect("references");
    let (_, bib_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "BIBSTART").expect("bibliography opening");

    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - quote_start_y - 37.0).abs() <= epsilon_pt,
        "front-matter date->quote transition should stay tightened: date_y={date_y}, quote_start_y={quote_start_y}"
    );
    assert!(
        (quote_start_y - quote_cont_y - 12.5).abs() <= epsilon_pt,
        "quote internal rhythm should remain stable: quote_start_y={quote_start_y}, quote_cont_y={quote_cont_y}"
    );
    assert!(
        (quote_cont_y - table_start_y - 23.0).abs() <= epsilon_pt,
        "quote->table transition should stay tightened in mixed-surface pages: quote_cont_y={quote_cont_y}, table_start_y={table_start_y}"
    );
    assert!(
        (table_start_y - table_next_y - 13.0).abs() <= epsilon_pt,
        "table internal rhythm should remain stable: table_start_y={table_start_y}, table_next_y={table_next_y}"
    );
    assert!(
        (table_next_y - list_start_y - 24.0).abs() <= epsilon_pt,
        "table->list transition should stay tightened in mixed-surface pages: table_next_y={table_next_y}, list_start_y={list_start_y}"
    );
    assert!(
        (list_start_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm should remain stable after table transition: list_start_y={list_start_y}, list_next_y={list_next_y}"
    );
    assert!(
        (list_next_y - references_y - 24.0).abs() <= epsilon_pt,
        "list->bibliography opening should stay tightened: list_next_y={list_next_y}, references_y={references_y}"
    );
    assert!(
        (references_y - bib_start_y - 12.0).abs() <= epsilon_pt,
        "bibliography heading->first entry gap should remain stable: references_y={references_y}, bib_start_y={bib_start_y}"
    );
    assert!(
        quote_x >= list_x + 6.0,
        "quote indent should remain visibly deeper than list body indent on mixed-surface pages: quote_x={quote_x}, list_x={list_x}"
    );
}

#[test]
fn pdf_renderer_nested_list_indentation_and_wrap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- [OUTERSTART] item with enough repeated words to force wrapping in the first list level before token [OUTERWRAPTOKEN]\n  - [NESTEDSTART] item with enough repeated words to force wrapping in the second list level before token [NESTEDWRAPTOKEN]")
        .expect("writer should accept nested list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let outer_start_x = tm_xs_for_segment_text_v0(&pdf, "OUTERSTART");
    let (_, outer_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "OUTERSTART").expect("outer start y");
    let outer_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "OUTERWRAPTOKEN");
    let (_, outer_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "OUTERWRAPTOKEN").expect("outer wrap y");
    let nested_start_x = tm_xs_for_segment_text_v0(&pdf, "NESTEDSTART");
    let (_, nested_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "NESTEDSTART").expect("nested start y");
    let nested_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "NESTEDWRAPTOKEN");
    let (_, nested_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "NESTEDWRAPTOKEN").expect("nested wrap y");

    assert_eq!(bullet_xs.len(), 2, "expected two bullets: {bullet_xs:?}");
    assert_eq!(outer_start_x.len(), 1, "expected outer start render");
    assert_eq!(outer_wrap_x.len(), 1, "expected outer wrap render");
    assert_eq!(nested_start_x.len(), 1, "expected nested start render");
    assert_eq!(nested_wrap_x.len(), 1, "expected nested wrap render");

    let epsilon_pt = 0.02f32;
    assert!(
        bullet_xs[1] > bullet_xs[0],
        "nested bullet should shift right: {bullet_xs:?}"
    );
    assert!(
        (outer_start_x[0] - 96.0).abs() <= epsilon_pt,
        "outer body x mismatch: {}",
        outer_start_x[0]
    );
    assert!(
        (outer_wrap_x[0] - outer_start_x[0]).abs() <= epsilon_pt,
        "outer continuation x mismatch: outer={}, wrap={}",
        outer_start_x[0],
        outer_wrap_x[0]
    );
    let outer_marker_gap = outer_start_x[0] - (bullet_xs[0] + segment_width_pt_v0(b"-"));
    let nested_marker_gap = nested_start_x[0] - (bullet_xs[1] + segment_width_pt_v0(b"-"));
    assert!(
        (outer_marker_gap - 8.0).abs() <= 0.25,
        "outer marker/body gap mismatch: {outer_marker_gap}"
    );
    assert!(
        (nested_marker_gap - 8.0).abs() <= 0.25,
        "nested marker/body gap mismatch: {nested_marker_gap}"
    );
    assert!(
        nested_start_x[0] > outer_start_x[0],
        "nested body should shift right: outer={}, nested={}",
        outer_start_x[0],
        nested_start_x[0]
    );
    assert!(
        (nested_wrap_x[0] - nested_start_x[0]).abs() <= epsilon_pt,
        "nested continuation x mismatch: nested={}, wrap={}",
        nested_start_x[0],
        nested_wrap_x[0]
    );
    assert!(
        (outer_start_y - outer_wrap_y - 13.0).abs() <= epsilon_pt,
        "outer wrapped list rhythm mismatch: outer_start_y={outer_start_y}, outer_wrap_y={outer_wrap_y}"
    );
    assert!(
        (nested_start_y - nested_wrap_y - 13.0).abs() <= epsilon_pt,
        "nested wrapped list rhythm mismatch: nested_start_y={nested_start_y}, nested_wrap_y={nested_wrap_y}"
    );
}
