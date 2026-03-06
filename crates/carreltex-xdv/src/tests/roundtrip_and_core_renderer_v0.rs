#[test]
fn parse_roundtrips_writer_layout_for_wrap_and_paging() {
    let text = b"word word word word word word word word word word";
    let layout = plan_layout_v0(text, 65_536, 786_432, 10, 1).expect("layout plan");
    let bytes = write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(text, 65_536, 786_432, 10, 1)
        .expect("writer output");
    let parsed = parse_dvi_v2_text_page_to_layout_v0(&bytes, 786_432).expect("parsed layout");
    assert_eq!(parsed, layout);
    assert!(validate_dvi_v2_text_page_matches_layout_v0(
        &bytes, &layout, 786_432
    ));
}

#[test]
fn parse_roundtrips_for_wi_dot_metrics() {
    let layout = plan_layout_v0(b"Wi.", 65_536, 786_432, 80, 200).expect("layout plan");
    let bytes =
        write_dvi_v2_text_page_with_layout_v0(b"Wi.", 65_536, 786_432).expect("writer output");
    let parsed = parse_dvi_v2_text_page_to_layout_v0(&bytes, 786_432).expect("parsed layout");
    assert_eq!(parsed, layout);
    assert_eq!(parsed.pages[0].lines[0].glyphs[0].advance_sp, 98_304);
    assert_eq!(parsed.pages[0].lines[0].glyphs[1].advance_sp, 32_768);
    assert_eq!(parsed.pages[0].lines[0].glyphs[2].advance_sp, 32_768);
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
fn pdf_renderer_emits_valid_header_and_contains_text() {
    let bytes = write_dvi_v2_text_page_v0(b"Hello\nWorld").expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&bytes).expect("pdf render");
    assert!(pdf.starts_with(b"%PDF-1.4\n"));
    assert!(pdf.windows(b"Hello".len()).any(|w| w == b"Hello"));
    assert!(pdf.windows(b"World".len()).any(|w| w == b"World"));
}

#[test]
fn pdf_renderer_uses_helvetica_base_fonts_v0() {
    let bytes = write_dvi_v2_text_page_v0(b"A [B] {C}").expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&bytes).expect("pdf render");
    assert!(pdf.windows(b"/Helvetica".len()).any(|w| w == b"/Helvetica"));
    assert!(pdf
        .windows(b"/Helvetica-Oblique".len())
        .any(|w| w == b"/Helvetica-Oblique"));
    assert!(pdf
        .windows(b"/Helvetica-Bold".len())
        .any(|w| w == b"/Helvetica-Bold"));
}

#[test]
fn pdf_renderer_applies_maketitle_typography_v0() {
    let demo_text =
        b"CarrelTeX Minimal Typeset Vertical Slice\nAlice\n2026-03-04\n\nBody paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(pdf.windows(b"18 Tf".len()).any(|w| w == b"18 Tf"));

    let pdf_text = String::from_utf8_lossy(&pdf);
    let mut found_non_margin_tm = false;
    for line in pdf_text.lines() {
        if !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        let Ok(x_pt) = fields[4].parse::<f32>() else {
            continue;
        };
        if (x_pt - 72.0).abs() > 0.01 {
            found_non_margin_tm = true;
            break;
        }
    }
    assert!(found_non_margin_tm, "expected centered title transform");
}

#[test]
fn pdf_renderer_centers_title_using_layout_width_v0() {
    let wide = write_dvi_v2_text_page_v0(b"WW\nA\n2026-03-04\n\nBody").expect("wide xdv");
    let narrow = write_dvi_v2_text_page_v0(b"ii\nA\n2026-03-04\n\nBody").expect("narrow xdv");
    let wide_pdf = render_dvi_v2_text_page_to_pdf_v0(&wide).expect("wide pdf");
    let narrow_pdf = render_dvi_v2_text_page_to_pdf_v0(&narrow).expect("narrow pdf");

    fn first_tm_x_v0(pdf: &[u8]) -> Option<f32> {
        let text = String::from_utf8_lossy(pdf);
        for line in text.lines() {
            if !line.contains(" Tm ") {
                continue;
            }
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 7 || fields[6] != "Tm" {
                continue;
            }
            if let Ok(x) = fields[4].parse::<f32>() {
                return Some(x);
            }
        }
        None
    }

    let wide_x = first_tm_x_v0(&wide_pdf).expect("wide title tm");
    let narrow_x = first_tm_x_v0(&narrow_pdf).expect("narrow title tm");
    assert!(wide_x < narrow_x, "wide_x={wide_x}, narrow_x={narrow_x}");
}

#[test]
fn pdf_renderer_indents_body_paragraph_start_after_blank_line_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-04\n\nFirst body line after title.\n\nIndented paragraph starts here.\nContinuation line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let pdf_text = String::from_utf8_lossy(&pdf);
    let mut has_margin_x = false;
    let mut has_indent_x = false;
    for line in pdf_text.lines() {
        if !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        let Ok(x_pt) = fields[4].parse::<f32>() else {
            continue;
        };
        if (x_pt - 72.0).abs() <= 0.02 {
            has_margin_x = true;
        }
        if (x_pt - 96.0).abs() <= 0.02 {
            has_indent_x = true;
        }
    }
    assert!(has_margin_x, "expected at least one body line at margin x");
    assert!(has_indent_x, "expected paragraph-start indent x");
}

#[test]
fn pdf_renderer_footnote_block_renders_at_page_bottom_with_small_font_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Body marker^1 line.\n\n!f 1 Footnote text with [emph].")
        .expect("writer should accept footnote marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("10 Tf"),
        "footnote block should use smaller font size: {pdf_text}"
    );
    assert!(
        !pdf_text.contains("!f 1"),
        "internal footnote prefix should be hidden in pdf output: {pdf_text}"
    );

    let body_pos =
        tm_position_for_segment_substring_v0(&pdf, "(Body").expect("body segment position");
    let footnote_pos = tm_position_for_line_containing_text_v0(&pdf, "(1 Footnote text with ")
        .expect("footnote line position");
    assert!(
        footnote_pos.1 < body_pos.1,
        "footnote should render below body line: body_y={} footnote_y={}",
        body_pos.1,
        footnote_pos.1
    );
    assert!(
        body_pos.1 - footnote_pos.1 >= 24.0,
        "footnote block should preserve readable separation from body content: body_y={}, footnote_y={}",
        body_pos.1,
        footnote_pos.1
    );
    assert!(
        (72.0..=140.0).contains(&footnote_pos.1),
        "footnote block should stay near page bottom margin: y={}",
        footnote_pos.1
    );
}

#[test]
fn pdf_renderer_footnote_internal_rhythm_is_stable_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Body markers^1 and^2.\n\n!f 1 First footnote line.\n!f 2 Second footnote line.",
    )
    .expect("writer should accept footnote rhythm markers");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, first_footnote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(1 First footnote line.)")
            .expect("first footnote line");
    let (_, second_footnote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2 Second footnote line.)")
            .expect("second footnote line");
    let delta = first_footnote_y - second_footnote_y;
    assert!(
        (delta - 13.0).abs() <= 0.05,
        "footnote line rhythm should remain stable at FOOTNOTE_LEADING_PT_V0: delta={delta}"
    );
}

#[test]
fn pdf_renderer_link_style_segments_are_emitted_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Visit {Example link} now.")
        .expect("writer should accept link style markers");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("/F3 12 Tf (Example link) Tj"),
        "link segment should render in styled font: {pdf_text}"
    );
    assert!(
        pdf_text.contains("/F1 12 Tf (Visit ) Tj"),
        "leading regular segment should remain: {pdf_text}"
    );
    assert!(
        pdf_text.contains("/F1 12 Tf ( now.) Tj"),
        "trailing regular segment should remain: {pdf_text}"
    );
}

#[test]
fn pdf_renderer_rejects_footnote_block_overflow_v0() {
    let mut text = Vec::<u8>::new();
    text.extend_from_slice(b"Body line with markers ");
    for index in 0..80u8 {
        text.extend_from_slice(b"^");
        text.extend_from_slice((index + 1).to_string().as_bytes());
        text.push(b' ');
    }
    text.extend_from_slice(b"\n\n");
    for index in 0..80u8 {
        text.extend_from_slice(b"!f ");
        text.extend_from_slice((index + 1).to_string().as_bytes());
        text.extend_from_slice(b" Footnote overflow line.\n");
    }
    let xdv = write_dvi_v2_text_page_v0(&text).expect("writer should accept overflow case");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when footnote block exceeds reserved height"
    );
}

#[test]
fn pdf_renderer_link_style_near_punctuation_stays_single_matrix_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"See,{Example}. Tail")
        .expect("writer should accept styled punctuation link line");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "See,")
        .expect("link punctuation line should decode");
    assert_eq!(rendered, "See,Example. Tail");

    let see_x = tm_xs_for_segment_text_v0(&pdf, "See,")[0];
    let example_x = tm_xs_for_segment_text_v0(&pdf, "Example")[0];
    let tail_x = tm_x_for_segment_substring_v0(&pdf, "(See,)", "(. Tail)").expect("tail segment x");
    let epsilon_pt = 0.02f32;
    assert!(
        ((example_x - see_x) - segment_width_pt_v0(b"See,")).abs() <= epsilon_pt,
        "See->Example boundary drifted: see_x={see_x}, example_x={example_x}"
    );
    assert!(
        ((tail_x - example_x) - segment_width_pt_v0(b"Example")).abs() <= epsilon_pt,
        "Example->tail boundary drifted: example_x={example_x}, tail_x={tail_x}"
    );
}

#[test]
fn pdf_renderer_inline_math_placeholder_keeps_single_text_matrix_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Before MATH after.")
        .expect("writer should accept inline math placeholder line");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Before MATH after.)"),
        "inline math placeholder line should render: {pdf_text}"
    );
    let tm_count = tm_count_for_line_containing_v0(&pdf, "(Before MATH after.)");
    assert_eq!(
        tm_count, 1,
        "inline placeholder line should use a single Tm"
    );
}

#[test]
fn pdf_renderer_display_math_placeholder_line_is_centered_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n^ MATH DISPLAY\n\nAfter.")
        .expect("writer should accept display math placeholder marker line");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let display_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs.len() >= 2 && line.glyphs[0].byte == b'^' && line.glyphs[1].byte == b' '
        })
        .expect("display line with center prefix should exist");
    let display_width_pt: f32 = display_line.glyphs[2..]
        .iter()
        .map(|glyph| glyph.advance_sp as f32 / 65_536.0)
        .sum();
    let expected_x_pt = ((612.0 - display_width_pt) * 0.5).clamp(72.0, 540.0);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        !pdf_text.contains("^ MATH DISPLAY"),
        "internal center prefix should be hidden in pdf output: {pdf_text}"
    );
    let (actual_x_pt, _) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY)")
        .expect("display placeholder x coordinate");
    assert!(
        (actual_x_pt - expected_x_pt).abs() <= 0.05,
        "display placeholder should be centered: actual={actual_x_pt} expected={expected_x_pt}"
    );
}

#[test]
fn pdf_renderer_display_math_long_placeholder_is_wider_and_left_shifted_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before.\n\n^ MATH DISPLAY\n\n^ MATH DISPLAY LONG FORM\n\nAfter.",
    )
    .expect("writer should accept display math placeholder marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (short_x, _) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY)")
        .expect("short display placeholder x coordinate");
    let (long_x, _) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY LONG FORM)")
        .expect("long display placeholder x coordinate");
    assert!(
        long_x < short_x,
        "long placeholder should center from a wider line and shift left: short_x={short_x}, long_x={long_x}"
    );
}

#[test]
fn pdf_renderer_display_math_with_equation_metadata_renders_right_number_v1() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n^ MATH DISPLAY\n\nAfter.\n\n!eq 1 1")
        .expect("writer should accept equation metadata line");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (display_x, _) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY)")
        .expect("display placeholder x coordinate");
    let (number_x, _) = tm_position_for_segment_substring_v0(&pdf, "\\(1\\)")
        .expect("equation number x coordinate");
    assert!(
        number_x > display_x,
        "equation number should render to the right of placeholder: display_x={display_x}, number_x={number_x}"
    );
    assert!(
        number_x > 450.0,
        "equation number should be near right margin: number_x={number_x}"
    );
}

#[test]
fn pdf_renderer_emits_link_annotation_with_in_bounds_rect_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Visit <{Example link}> now.\n\n!u 1 https://example.com")
        .expect("writer should accept href marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("/Subtype /Link"),
        "link annotation subtype missing: {pdf_text}"
    );
    assert!(
        pdf_text.contains("/URI (https://example.com)"),
        "link annotation URI missing: {pdf_text}"
    );
    let rect = parse_first_link_rect_v0(&pdf).expect("link rect should parse");
    assert!(rect[2] > rect[0], "rect width must be positive: {rect:?}");
    assert!(rect[3] > rect[1], "rect height must be positive: {rect:?}");
    assert!((0.0..=612.0).contains(&rect[0]) && (0.0..=612.0).contains(&rect[2]));
    assert!((0.0..=792.0).contains(&rect[1]) && (0.0..=792.0).contains(&rect[3]));
}

#[test]
fn pdf_renderer_emits_internal_ref_and_external_href_annotations_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n@S {Intro}\n\nSee <1> and <{Example}>.\n\n!l sec:intro 1 heading 1 Intro\n!r sec:intro 5 1\n!ra 1 1\n!u 2 https://example.com",
    )
    .expect("writer should accept cross-ref marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(annots.len(), 2, "expected one ref annot and one href annot");

    let first_annot = parse_pdf_object_body_v0(&pdf, annots[0]).expect("first annotation");
    let second_annot = parse_pdf_object_body_v0(&pdf, annots[1]).expect("second annotation");
    assert!(
        first_annot.contains("/Dest ["),
        "first annotation should use internal destination: {first_annot}"
    );
    assert!(
        !first_annot.contains("/A "),
        "internal destination annotation must not use URI action: {first_annot}"
    );
    assert_eq!(
        parse_pdf_annotation_dest_page_id_v0(&first_annot),
        Some(3),
        "internal ref should target first page"
    );

    let action_id = parse_pdf_annotation_action_id_v0(&second_annot).expect("href action id");
    let action_body = parse_pdf_object_body_v0(&pdf, action_id).expect("href action body");
    assert_eq!(
        parse_pdf_action_uri_v0(&action_body).as_deref(),
        Some("https://example.com"),
        "href annotation should keep URI target"
    );
}

#[test]
fn pdf_renderer_emits_figref_annotation_targeting_figure_anchor_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n@S {Intro}\n\n!gbox\n!gcap Figure 1: Demo caption.\n\nSee <1>.\n\n!l fig:demo 2 figure 1 -\n!r fig:demo 9 2\n!ra 1 2",
    )
    .expect("writer should accept figure and cross-ref marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(See ) Tj") && pdf_text.contains("(1) Tj"),
        "figref text should render resolved figure ordinal without placeholder: {pdf_text}"
    );
    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(
        annots.len(),
        1,
        "expected one internal figure ref annotation"
    );
    let annotation = parse_pdf_object_body_v0(&pdf, annots[0]).expect("annotation");
    assert!(
        annotation.contains("/Dest ["),
        "figure ref annotation should use internal destination: {annotation}"
    );
    assert_eq!(
        parse_pdf_annotation_dest_page_id_v0(&annotation),
        Some(3),
        "figure ref destination should target page containing figure anchor"
    );
}

#[test]
fn pdf_renderer_emits_eqref_annotation_targeting_equation_anchor_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n^ MATH DISPLAY\n\nSee <1>.\n\n!eq 1 1\n!l eq:first 1 equation 1 -\n!r eq:first 5 1\n!ra 1 1",
    )
    .expect("writer should accept equation and cross-ref marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(See ) Tj") && pdf_text.contains("(1) Tj"),
        "eqref text should render resolved equation ordinal without placeholder: {pdf_text}"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(
        annots.len(),
        1,
        "expected one internal equation ref annotation"
    );
    let annotation = parse_pdf_object_body_v0(&pdf, annots[0]).expect("annotation");
    assert!(
        annotation.contains("/Dest ["),
        "equation ref annotation should use internal destination: {annotation}"
    );
    assert_eq!(
        parse_pdf_annotation_dest_page_id_v0(&annotation),
        Some(3),
        "equation ref destination should target page containing equation anchor"
    );
}

#[test]
fn pdf_renderer_resolves_pageref_marker_and_emits_page_destination_link_v2() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n@S {Intro}\x0cSee <@@PG:1@@>.\n\n!l sec:intro 1 heading 1 Intro\n!pr sec:intro 3 1\n!rp 1 1",
    )
    .expect("writer should accept pageref marker and metadata lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(See ) Tj") && pdf_text.contains("(1) Tj"),
        "pageref should render resolved destination page number: {pdf_text}"
    );
    assert!(
        !pdf_text.contains("@@PG:"),
        "internal pageref marker must not leak into rendered pdf text: {pdf_text}"
    );

    let page_two = parse_pdf_object_body_v0(&pdf, 4).expect("second page object");
    let annots = parse_pdf_ref_ids_v0(&page_two, "/Annots");
    assert_eq!(
        annots.len(),
        1,
        "expected one pageref annotation on page two"
    );
    let annotation = parse_pdf_object_body_v0(&pdf, annots[0]).expect("annotation");
    assert!(
        annotation.contains("/Dest [3 0 R /Fit]"),
        "pageref annotation must target stable page destination with /Fit: {annotation}"
    );
    assert!(
        !annotation.contains("/XYZ"),
        "pageref page destination must not use incidental /XYZ coordinates: {annotation}"
    );
}

#[test]
fn pdf_renderer_rejects_pageref_marker_with_unknown_anchor_v2() {
    let xdv = write_dvi_v2_text_page_v0(b"See <@@PG:9@@>.\n\n!pr sec:missing 1 9\n!rp 1 9")
        .expect("writer should accept pageref marker and metadata lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when pageref marker targets a missing anchor"
    );
}

#[test]
fn pdf_renderer_multi_figure_ref_annotations_follow_ordinal_order_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n@S {Intro}\n\n!gbox\n!gcap Figure 1: First caption.\n\n!gbox\n!gcap Figure 2: Second caption.\n\nSee <1> and <2>.\n\n!l fig:first 2 figure 1 -\n!l fig:second 3 figure 2 -\n!r fig:first 11 2\n!r fig:second 11 3\n!ra 1 2\n!ra 2 3",
    )
    .expect("writer should accept multi-figure cross-ref marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Figure 1: First caption.) Tj"),
        "first figure caption should render numbered prefix: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(Figure 2: Second caption.) Tj"),
        "second figure caption should render numbered prefix: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(See ) Tj")
            && pdf_text.contains("(1) Tj")
            && pdf_text.contains("(2) Tj"),
        "ref line should render both resolved ordinals: {pdf_text}"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(annots.len(), 2, "expected two figure ref annotations");
    let first_annot = parse_pdf_object_body_v0(&pdf, annots[0]).expect("first annotation");
    let second_annot = parse_pdf_object_body_v0(&pdf, annots[1]).expect("second annotation");
    assert_eq!(
        parse_pdf_annotation_dest_page_id_v0(&first_annot),
        Some(3),
        "first figure ref should target page containing first figure anchor"
    );
    assert_eq!(
        parse_pdf_annotation_dest_page_id_v0(&second_annot),
        Some(3),
        "second figure ref should target page containing second figure anchor"
    );
}

#[test]
fn pdf_renderer_rejects_ref_annotation_target_with_missing_anchor_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"See <1> only.\n\n!r sec:intro 1 1\n!ra 1 1")
        .expect("writer should accept marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when ref target anchor is missing"
    );
}

#[test]
fn pdf_renderer_rejects_figure_label_metadata_with_zero_ordinal_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!gbox\n!gcap Figure 1: Caption.\n\n!l fig:bad 1 figure 0 -",
    )
    .expect("writer should accept marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when figure label metadata carries zero ordinal"
    );
}

#[test]
fn pdf_renderer_rejects_equation_label_metadata_with_zero_ordinal_v1() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n^ MATH DISPLAY\n\n!eq 1 1\n!l eq:bad 1 equation 0 -",
    )
    .expect("writer should accept marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when equation label metadata carries zero ordinal"
    );
}

#[test]
fn pdf_renderer_footnote_marker_uses_smaller_raised_typography_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Body marker^1 line.\n\n!f 1 Footnote text.")
        .expect("writer should accept footnote marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("/F1 8 Tf (^1) Tj"),
        "footnote marker should render in smaller font: {pdf_text}"
    );
    assert!(
        pdf_text.contains("4 Ts /F1 8 Tf (^1) Tj"),
        "footnote marker should use positive text rise for superscript effect: {pdf_text}"
    );
}

#[test]
fn pdf_renderer_rejects_unknown_footnote_marker_id_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Body marker^2 line.\n\n!f 1 1 Known footnote.")
        .expect("writer should accept marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on unknown footnote marker id"
    );
}

#[test]
fn pdf_renderer_multipage_footnotes_and_annots_associate_per_page_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Page1 line with <{First link}> and marker^1.\x0cPage2 line with <{Second link}> and marker^2.\n\n!f 1 First page footnote text.\n!f 2 Second page footnote text.\n!u 1 https://example.com/page1\n!u 2 https://example.com/page2",
    )
    .expect("writer should accept multipage marker data");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert_eq!(
        count_pdf_page_objects_v0(&pdf),
        2,
        "expected two page objects"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page 1 object");
    let page_two = parse_pdf_object_body_v0(&pdf, 4).expect("page 2 object");
    let page_one_annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    let page_two_annots = parse_pdf_ref_ids_v0(&page_two, "/Annots");
    assert_eq!(
        page_one_annots.len(),
        1,
        "page 1 should have one annotation"
    );
    assert_eq!(
        page_two_annots.len(),
        1,
        "page 2 should have one annotation"
    );
    assert_ne!(
        page_one_annots[0], page_two_annots[0],
        "each page should reference its own annotation object"
    );

    let annotation_one =
        parse_pdf_object_body_v0(&pdf, page_one_annots[0]).expect("annotation one");
    let annotation_two =
        parse_pdf_object_body_v0(&pdf, page_two_annots[0]).expect("annotation two");
    let action_one =
        parse_pdf_annotation_action_id_v0(&annotation_one).expect("annotation one action");
    let action_two =
        parse_pdf_annotation_action_id_v0(&annotation_two).expect("annotation two action");
    let action_one_body = parse_pdf_object_body_v0(&pdf, action_one).expect("action one body");
    let action_two_body = parse_pdf_object_body_v0(&pdf, action_two).expect("action two body");
    assert_eq!(
        parse_pdf_action_uri_v0(&action_one_body).as_deref(),
        Some("https://example.com/page1"),
        "page 1 annotation should target page 1 href"
    );
    assert_eq!(
        parse_pdf_action_uri_v0(&action_two_body).as_deref(),
        Some("https://example.com/page2"),
        "page 2 annotation should target page 2 href"
    );

    for rect in [
        parse_pdf_annotation_rect_v0(&annotation_one).expect("annotation one rect"),
        parse_pdf_annotation_rect_v0(&annotation_two).expect("annotation two rect"),
    ] {
        assert!(
            rect[2] > rect[0],
            "annotation width must be positive: {rect:?}"
        );
        assert!(
            rect[3] > rect[1],
            "annotation height must be positive: {rect:?}"
        );
        assert!(
            rect[0] >= 0.0 && rect[1] >= 0.0 && rect[2] <= 612.0 && rect[3] <= 792.0,
            "annotation rect must stay within page bounds: {rect:?}"
        );
    }

    let stream_one = parse_pdf_object_body_v0(&pdf, 5).expect("stream one body");
    let stream_two = parse_pdf_object_body_v0(&pdf, 6).expect("stream two body");
    let page_one_footnote_y = tm_position_for_line_containing_text_in_body_v0(&stream_one, "(1")
        .expect("page one footnote line")
        .1;
    let page_two_footnote_y = tm_position_for_line_containing_text_in_body_v0(&stream_two, "(2")
        .expect("page two footnote line")
        .1;
    assert!(
        (72.0..=140.0).contains(&page_one_footnote_y),
        "page 1 footnote should render near bottom: y={page_one_footnote_y}"
    );
    assert!(
        (72.0..=140.0).contains(&page_two_footnote_y),
        "page 2 footnote should render near bottom: y={page_two_footnote_y}"
    );
}
