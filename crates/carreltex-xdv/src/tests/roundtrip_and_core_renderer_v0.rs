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
        (delta - 12.5).abs() <= 0.05,
        "footnote entry rhythm should remain stable after v18 polish: delta={delta}"
    );
}

#[test]
fn pdf_renderer_footnote_continuation_indent_and_rhythm_are_stable_v18() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Body markers^1 and^2.\n\n!f 1 First entry lead line.\nContinuation for first entry.\n!f 2 Second entry lead line.",
    )
    .expect("writer should accept wrapped footnote entries");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(1 First entry lead line.)")
            .expect("first footnote entry");
    let (cont_x, cont_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Continuation for first entry.)")
            .expect("footnote continuation");
    let (_, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2 Second entry lead line.)")
            .expect("second footnote entry");

    assert!(
        cont_x > 72.0 + 6.0,
        "footnote continuation should keep readable hanging indent from margin: cont_x={cont_x}"
    );
    assert!(
        (first_y - cont_y - 12.0).abs() <= 0.05,
        "footnote continuation rhythm should stay compact and stable: first_y={first_y}, cont_y={cont_y}"
    );
    assert!(
        (cont_y - second_y - 12.5).abs() <= 0.05,
        "footnote entry-to-entry rhythm should stay stable: cont_y={cont_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_footnote_wrapped_links_keep_annotation_alignment_v18() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Body line with <{BODYLINK}> and marker^1.\n\n!f 1 <{FOOTLINKONE}> first footnote link line.\n<{FOOTLINKTWO}> second footnote continuation.\n!u 1 https://example.com/body\n!u 2 https://example.com/foot1\n!u 3 https://example.com/foot2",
    )
    .expect("writer should accept footnote link and href metadata");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page 1 object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(
        annots.len(),
        3,
        "expected one body link annotation and two footnote link annotations"
    );

    let mut body_rect = None::<[f32; 4]>;
    let mut foot_one_rect = None::<[f32; 4]>;
    let mut foot_two_rect = None::<[f32; 4]>;
    for annot_id in annots {
        let annotation = parse_pdf_object_body_v0(&pdf, annot_id).expect("annotation body");
        let rect = parse_pdf_annotation_rect_v0(&annotation).expect("annotation rect");
        let action_id = parse_pdf_annotation_action_id_v0(&annotation).expect("annotation action");
        let action = parse_pdf_object_body_v0(&pdf, action_id).expect("annotation action body");
        match parse_pdf_action_uri_v0(&action).as_deref() {
            Some("https://example.com/body") => body_rect = Some(rect),
            Some("https://example.com/foot1") => foot_one_rect = Some(rect),
            Some("https://example.com/foot2") => foot_two_rect = Some(rect),
            other => panic!("unexpected annotation uri target: {other:?}"),
        }
    }

    let body_rect = body_rect.expect("body link rect");
    let foot_one_rect = foot_one_rect.expect("footnote link one rect");
    let foot_two_rect = foot_two_rect.expect("footnote link two rect");
    let (body_x, body_y) =
        tm_position_for_segment_substring_v0(&pdf, "BODYLINK").expect("body link position");
    let (foot_one_x, foot_one_y) =
        tm_position_for_segment_substring_v0(&pdf, "FOOTLINKONE").expect("footnote link one position");
    let (foot_two_x, foot_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "FOOTLINKTWO").expect("footnote link two position");

    assert!(
        (body_rect[0] - body_x).abs() <= 0.2,
        "body link annotation x should align with rendered body link text: rect={body_rect:?}, body_x={body_x}"
    );
    assert!(
        (foot_one_rect[0] - foot_one_x).abs() <= 0.2
            && (foot_two_rect[0] - foot_two_x).abs() <= 0.2,
        "footnote link annotations should align with rendered footnote link text columns"
    );
    assert!(
        foot_two_x > foot_one_x + 8.0,
        "wrapped footnote continuation should keep readable hanging indent from first line: foot_one_x={foot_one_x}, foot_two_x={foot_two_x}"
    );
    assert!(
        body_y > foot_one_y && body_y > foot_two_y,
        "body link should render above footnote link rows: body_y={body_y}, foot_one_y={foot_one_y}, foot_two_y={foot_two_y}"
    );
    assert!(
        foot_one_rect[3] - foot_one_rect[1] <= 9.2
            && foot_two_rect[3] - foot_two_rect[1] <= 9.2,
        "footnote annotation hitboxes should remain compact and stable after v18 polish: foot_one_rect={foot_one_rect:?}, foot_two_rect={foot_two_rect:?}"
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
fn pdf_renderer_display_math_blank_transition_rhythm_is_tightened_v16() {
    let xdv =
        write_dvi_v2_text_page_v0(b"~ Paragraph before.\n\n^ MATH DISPLAY\n\n~ Paragraph after.")
        .expect("writer should accept display-math transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (_, before_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before.)")
        .expect("paragraph before y");
    let (_, math_y) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY)")
        .expect("display math y");
    let (_, after_y) = tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after.)")
        .expect("paragraph after y");
    let before_gap = before_y - math_y;
    let after_gap = math_y - after_y;
    assert!(
        (before_gap - 24.0).abs() <= 0.05,
        "paragraph->display baseline gap should tighten to 24pt: {before_gap}"
    );
    assert!(
        (after_gap - 24.0).abs() <= 0.05,
        "display->paragraph baseline gap should tighten to 24pt: {after_gap}"
    );
}

#[test]
fn pdf_renderer_display_math_does_not_force_centered_continuation_v16() {
    let xdv = write_dvi_v2_text_page_v0(
        b"~ Body lead line with [style] seam.\n^ MATH DISPLAY\nBody continuation after display.",
    )
    .expect("writer should accept display-math continuation text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (math_x, math_y) = tm_position_for_line_containing_text_v0(&pdf, "(MATH DISPLAY)")
        .expect("display math position");
    let (body_x, body_y) = tm_position_for_line_containing_text_v0(
        &pdf,
        "(Body continuation after display.)",
    )
    .expect("body continuation position");
    assert!(
        (body_x - 72.0).abs() <= 0.05,
        "body line after display math should return to paragraph margin, not stay centered: x={body_x}"
    );
    assert!(
        math_x > body_x + 20.0,
        "display math should remain centered away from paragraph margin: math_x={math_x}, body_x={body_x}"
    );
    let baseline_gap = math_y - body_y;
    assert!(
        (baseline_gap - 14.0).abs() <= 0.05,
        "display line to immediate following body line should keep stable line-leading rhythm: gap={baseline_gap}"
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
fn pdf_renderer_bibliography_wrapped_link_annotations_follow_hanging_indent_v14() {
    let xdv = write_dvi_v2_text_page_v0(
        b"@S {References}\n\n[1] ALPHASTART <{linked phrase begins on bibliography line\nand LINKCONT closes here}> tail.\n\n!u 1 https://example.com/bib",
    )
    .expect("writer should accept bibliography link marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (alpha_start_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "ALPHASTART").expect("alpha start x");
    let first_line_link_x = tm_x_for_segment_substring_v0(
        &pdf,
        "ALPHASTART",
        "(linked phrase begins on bibliography line)",
    )
    .expect("first bibliography link segment x");
    let (link_cont_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "LINKCONT").expect("continuation line x");
    let second_line_link_x =
        tm_x_for_segment_substring_v0(&pdf, "LINKCONT", "(and LINKCONT closes here)")
            .expect("second bibliography link segment x");
    let first_line_link_width =
        segment_width_pt_v0(b"linked phrase begins on bibliography line");
    let second_line_link_width = segment_width_pt_v0(b"and LINKCONT closes here");
    let epsilon_pt = 0.2f32;
    assert!(
        (alpha_start_x - link_cont_x).abs() <= epsilon_pt,
        "wrapped bibliography continuation should preserve hanging-indent column: alpha_start_x={alpha_start_x}, link_cont_x={link_cont_x}"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(
        annots.len(),
        2,
        "wrapped bibliography link should emit one annotation rect per wrapped line"
    );
    for annotation_id in &annots {
        let annotation =
            parse_pdf_object_body_v0(&pdf, *annotation_id).expect("annotation body");
        let rect = parse_pdf_annotation_rect_v0(&annotation).expect("annotation rect");
        assert!(
            rect[0] >= alpha_start_x - epsilon_pt,
            "wrapped bibliography link annotation should stay in body column: rect={rect:?}, alpha_start_x={alpha_start_x}"
        );
        assert!(
            rect[2] > rect[0] && rect[3] > rect[1],
            "annotation rect must stay positive: {rect:?}"
        );
        let rect_height = rect[3] - rect[1];
        assert!(
            (8.0..=11.2).contains(&rect_height),
            "bibliography wrapped link annotation height should stay tightly bounded: rect={rect:?}, height={rect_height}"
        );
        let action_id = parse_pdf_annotation_action_id_v0(&annotation).expect("annotation action");
        let action_body = parse_pdf_object_body_v0(&pdf, action_id).expect("action body");
        assert_eq!(
            parse_pdf_action_uri_v0(&action_body).as_deref(),
            Some("https://example.com/bib"),
            "wrapped bibliography link annotation should keep href target"
        );
    }
    let first_rect =
        parse_pdf_annotation_rect_v0(&parse_pdf_object_body_v0(&pdf, annots[0]).expect("first annotation body"))
            .expect("first annotation rect");
    let second_rect =
        parse_pdf_annotation_rect_v0(&parse_pdf_object_body_v0(&pdf, annots[1]).expect("second annotation body"))
            .expect("second annotation rect");
    assert!(
        (first_rect[0] - first_line_link_x).abs() <= 0.2,
        "first bibliography annotation x should align with first rendered link line: rect={first_rect:?}, line_x={first_line_link_x}"
    );
    assert!(
        (second_rect[0] - second_line_link_x).abs() <= 0.2,
        "second bibliography annotation x should align with wrapped link continuation: rect={second_rect:?}, line_x={second_line_link_x}"
    );
    assert!(
        ((first_rect[2] - first_rect[0]) - first_line_link_width).abs() <= 1.2,
        "first bibliography annotation width should track rendered link width: rect={first_rect:?}, expected_width={first_line_link_width}"
    );
    assert!(
        ((second_rect[2] - second_rect[0]) - second_line_link_width).abs() <= 1.2,
        "second bibliography annotation width should track wrapped link width: rect={second_rect:?}, expected_width={second_line_link_width}"
    );
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
    let ref_rect = parse_pdf_annotation_rect_v0(&first_annot).expect("internal ref rect");
    let ref_x =
        tm_x_for_segment_substring_v0(&pdf, "(See ", "(1)").expect("internal ref text x");
    let ref_width = ref_rect[2] - ref_rect[0];
    let expected_ref_width = segment_width_pt_v0(b"1");
    assert!(
        (ref_rect[0] - ref_x).abs() <= 0.2,
        "internal ref hitbox x should align with rendered ref text: rect={ref_rect:?}, ref_x={ref_x}"
    );
    assert!(
        (ref_width - expected_ref_width).abs() <= 1.0,
        "internal ref hitbox width should track rendered text width: rect_width={ref_width}, expected={expected_ref_width}"
    );
    let ref_height = ref_rect[3] - ref_rect[1];
    assert!(
        (8.0..=13.0).contains(&ref_height),
        "internal ref hitbox height should stay in stable bounds: height={ref_height}"
    );

    let action_id = parse_pdf_annotation_action_id_v0(&second_annot).expect("href action id");
    let action_body = parse_pdf_object_body_v0(&pdf, action_id).expect("href action body");
    assert_eq!(
        parse_pdf_action_uri_v0(&action_body).as_deref(),
        Some("https://example.com"),
        "href annotation should keep URI target"
    );
    let href_rect = parse_pdf_annotation_rect_v0(&second_annot).expect("href rect");
    let href_x =
        tm_x_for_segment_substring_v0(&pdf, "(See ", "(Example)").expect("href text x");
    let href_width = href_rect[2] - href_rect[0];
    let expected_href_width = segment_width_pt_v0(b"Example");
    assert!(
        (href_rect[0] - href_x).abs() <= 0.2,
        "href hitbox x should align with rendered link text: rect={href_rect:?}, href_x={href_x}"
    );
    assert!(
        (href_width - expected_href_width).abs() <= 1.0,
        "href hitbox width should track rendered link text width: rect_width={href_width}, expected={expected_href_width}"
    );
    let href_height = href_rect[3] - href_rect[1];
    assert!(
        (8.0..=13.0).contains(&href_height),
        "href hitbox height should stay in stable bounds: height={href_height}"
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
    let rect = parse_pdf_annotation_rect_v0(&annotation).expect("figure ref rect");
    let ref_x = tm_x_for_segment_substring_v0(&pdf, "(See ", "(1)").expect("figure ref text x");
    let width = rect[2] - rect[0];
    let expected_width = segment_width_pt_v0(b"1");
    assert!(
        (rect[0] - ref_x).abs() <= 0.2,
        "figure ref hitbox x should align to rendered ordinal: rect={rect:?}, ref_x={ref_x}"
    );
    assert!(
        (width - expected_width).abs() <= 1.0,
        "figure ref hitbox width should track rendered ordinal width: width={width}, expected={expected_width}"
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
    let rect = parse_pdf_annotation_rect_v0(&annotation).expect("equation ref rect");
    let ref_x = tm_x_for_segment_substring_v0(&pdf, "(See ", "(1)").expect("equation ref text x");
    let width = rect[2] - rect[0];
    let expected_width = segment_width_pt_v0(b"1");
    assert!(
        (rect[0] - ref_x).abs() <= 0.2,
        "equation ref hitbox x should align to rendered ordinal: rect={rect:?}, ref_x={ref_x}"
    );
    assert!(
        (width - expected_width).abs() <= 1.0,
        "equation ref hitbox width should track rendered ordinal width: width={width}, expected={expected_width}"
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
    let rect = parse_pdf_annotation_rect_v0(&annotation).expect("pageref rect");
    let ref_x = tm_x_for_segment_substring_v0(&pdf, "(See ", "(1)").expect("pageref text x");
    let width = rect[2] - rect[0];
    let expected_width = segment_width_pt_v0(b"1");
    assert!(
        (rect[0] - ref_x).abs() <= 0.2,
        "pageref hitbox x should align to rendered page number: rect={rect:?}, ref_x={ref_x}"
    );
    assert!(
        (width - expected_width).abs() <= 1.0,
        "pageref hitbox width should track rendered page number width: width={width}, expected={expected_width}"
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

#[test]
fn pdf_renderer_preserves_wrapped_list_and_quote_continuation_indent_across_pages_v8() {
    let text = b"- LISTSTART alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha LISTWRAPTOKEN\n\n> QUOTESTART beta beta beta beta beta beta beta beta beta beta beta beta QUOTEWRAPTOKEN";
    let xdv = write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(text, 65_536, 786_432, 32, 1)
        .expect("writer should accept multipage wrapped list/quote text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let page_count = count_pdf_page_objects_v0(&pdf);
    assert!(page_count >= 4, "expected multipage output for wrapped blocks");

    let first_stream_id = 3u32 + page_count as u32;
    let mut list_start = None::<(usize, f32)>;
    let mut list_wrap = None::<(usize, f32)>;
    let mut quote_start = None::<(usize, f32)>;
    let mut quote_wrap = None::<(usize, f32)>;
    for page_index in 0..page_count {
        let stream_body = parse_pdf_object_body_v0(&pdf, first_stream_id + page_index as u32)
            .expect("stream body");
        if list_start.is_none() {
            if let Some((x, _)) =
                tm_position_for_line_containing_text_in_body_v0(&stream_body, "LISTSTART")
            {
                list_start = Some((page_index, x));
            }
        }
        if list_wrap.is_none() {
            if let Some((x, _)) =
                tm_position_for_line_containing_text_in_body_v0(&stream_body, "LISTWRAPTOKEN")
            {
                list_wrap = Some((page_index, x));
            }
        }
        if quote_start.is_none() {
            if let Some((x, _)) =
                tm_position_for_line_containing_text_in_body_v0(&stream_body, "QUOTESTART")
            {
                quote_start = Some((page_index, x));
            }
        }
        if quote_wrap.is_none() {
            if let Some((x, _)) =
                tm_position_for_line_containing_text_in_body_v0(&stream_body, "QUOTEWRAPTOKEN")
            {
                quote_wrap = Some((page_index, x));
            }
        }
    }

    let (list_start_page, list_start_x) = list_start.expect("LISTSTART position");
    let (list_wrap_page, list_wrap_x) = list_wrap.expect("LISTWRAPTOKEN position");
    let (quote_start_page, quote_start_x) = quote_start.expect("QUOTESTART position");
    let (quote_wrap_page, quote_wrap_x) = quote_wrap.expect("QUOTEWRAPTOKEN position");
    let epsilon_pt = 0.2f32;
    assert!(
        list_wrap_page > list_start_page,
        "wrapped list continuation should cross a page boundary: start_page={list_start_page}, wrap_page={list_wrap_page}"
    );
    assert!(
        quote_wrap_page > quote_start_page,
        "wrapped quote continuation should cross a page boundary: start_page={quote_start_page}, wrap_page={quote_wrap_page}"
    );
    assert!(
        (list_start_x - list_wrap_x).abs() <= epsilon_pt,
        "list continuation x must remain stable across page boundary: start_x={list_start_x}, wrap_x={list_wrap_x}"
    );
    assert!(
        (quote_start_x - quote_wrap_x).abs() <= epsilon_pt,
        "quote continuation x must remain stable across page boundary: start_x={quote_start_x}, wrap_x={quote_wrap_x}"
    );
    assert!(
        quote_start_x >= list_start_x + 6.0,
        "quote indentation should remain deeper than list indentation across pages: list_x={list_start_x}, quote_x={quote_start_x}"
    );
}

#[test]
fn pdf_renderer_keeps_link_annotations_and_footnotes_stable_when_link_wraps_across_pages_v8() {
    let text = b"- marker^1 <{LINKSTART gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma LINKMID gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma gamma LINKEND}>tail tail marker^2\n\n!f 1 First split footnote.\n!f 2 Second split footnote.\n!u 1 https://example.com/split";
    let xdv = write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(text, 65_536, 786_432, 64, 1)
        .expect("writer should accept wrapped multipage link and footnote text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let page_count = count_pdf_page_objects_v0(&pdf);
    assert!(page_count >= 2, "expected at least two pages for wrapped link");

    let mut uri_annotation_count = 0usize;
    for page_index in 0..page_count {
        let page_obj = parse_pdf_object_body_v0(&pdf, 3 + page_index as u32).expect("page object");
        let annots = parse_pdf_ref_ids_v0(&page_obj, "/Annots");
        for annot_id in annots {
            let annotation = parse_pdf_object_body_v0(&pdf, annot_id).expect("annotation body");
            let action_id =
                parse_pdf_annotation_action_id_v0(&annotation).expect("annotation action id");
            let action_body = parse_pdf_object_body_v0(&pdf, action_id).expect("action body");
            if parse_pdf_action_uri_v0(&action_body).as_deref() == Some("https://example.com/split")
            {
                uri_annotation_count += 1;
                let rect = parse_pdf_annotation_rect_v0(&annotation).expect("annotation rect");
                assert!(rect[2] > rect[0], "annotation width must be positive: {rect:?}");
                assert!(rect[3] > rect[1], "annotation height must be positive: {rect:?}");
                assert!(
                    rect[0] >= 0.0 && rect[1] >= 0.0 && rect[2] <= 612.0 && rect[3] <= 792.0,
                    "annotation rect must stay within page bounds: {rect:?}"
                );
            }
        }
    }
    assert!(
        uri_annotation_count >= 2,
        "wrapped link should keep annotation association across page boundaries"
    );

}
