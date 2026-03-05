use super::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0,
    count_dvi_v2_text_pages_with_advance_v0, validate_dvi_v2_empty_page_v0,
    LinePlanV0,
    parse_dvi_v2_text_page_to_layout_v0, plan_layout_v0, plan_layout_width_v0,
    recompute_line_width_sp_v0,
    render_dvi_v2_text_page_to_pdf_v0,
    sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_text_page_matches_layout_v0,
    validate_dvi_v2_text_page_v0, validate_dvi_v2_text_page_with_layout_v0,
    write_dvi_v2_empty_page_v0, write_dvi_v2_text_page_v0,
    write_dvi_v2_text_page_from_layout_v0,
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
    let plan = plan_layout_v0(b"ABCDEFGH\x0cIJKLMNOP", 65_536, 786_432, 4, 10).expect("layout plan");
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
fn planner_width_wraps_long_line_at_spaces() {
    let plan =
        plan_layout_width_v0(b"aaaa bbbb cccc", 65_536, 786_432, 327_680, 200).expect("layout plan");
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
    let plan =
        plan_layout_width_v0(b"A A", 100, 786_432, 250, 200).expect("layout plan");
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
    let layout = plan_layout_width_v0(b"A [B] {C}", 65_536, 786_432, 10_000_000, 200)
        .expect("layout plan");
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

#[test]
fn pdf_renderer_keeps_multi_space_line_unwrapped_under_width_limit_v0() {
    let layout =
        plan_layout_width_v0(b"A     B", 65_536, 786_432, 300_000, 200).expect("layout plan");
    assert_eq!(layout.pages.len(), 1);
    assert_eq!(layout.pages[0].lines.len(), 1);

    let xdv = write_dvi_v2_text_page_from_layout_v0(&layout, 786_432).expect("xdv bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(pdf.windows(b"(A     B) Tj".len()).any(|w| w == b"(A     B) Tj"));
}

fn max_tm_gap_pt_for_line_containing_v0(pdf: &[u8], needle: &str) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let mut xs = Vec::<f32>::new();
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            if fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm"
            {
                let x_pt = fields[index + 4].parse::<f32>().ok()?;
                xs.push(x_pt);
                index += 7;
                continue;
            }
            index += 1;
        }
        if xs.len() < 2 {
            return Some(0.0);
        }
        let mut max_gap = 0.0f32;
        for pair in xs.windows(2) {
            let gap = pair[1] - pair[0];
            if gap > max_gap {
                max_gap = gap;
            }
        }
        return Some(max_gap);
    }
    None
}

fn tm_x_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        if let Ok(x_pt) = fields[4].parse::<f32>() {
            return Some(x_pt);
        }
    }
    None
}

fn expected_center_x_pt_v0(width_sp: u32) -> f32 {
    let width_pt = (width_sp as f32) / 65_536.0;
    ((612.0 - width_pt) * 0.5).clamp(72.0, 612.0 - 72.0)
}

fn expected_right_x_pt_v0(width_sp: u32) -> f32 {
    let width_pt = (width_sp as f32) / 65_536.0;
    (612.0 - 72.0 - width_pt).max(72.0)
}

fn width_sp_for_prefixed_rendered_line_v0(line: &LinePlanV0, prefix: [u8; 2]) -> Option<u32> {
    if line.glyphs.len() < 2 {
        return None;
    }
    if line.glyphs[0].byte != prefix[0] || line.glyphs[1].byte != prefix[1] {
        return None;
    }
    let mut width_sp = 0u32;
    for glyph in &line.glyphs[2..] {
        let advance = u32::try_from(glyph.advance_sp).ok()?;
        width_sp = width_sp.checked_add(advance)?;
    }
    Some(width_sp)
}

#[test]
fn pdf_renderer_caps_segment_tm_gap_for_styled_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Styled [emphasis] and {bold} run.")
        .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "Styled")
        .expect("pdf should include styled line");
    assert!(
        max_tm_gap <= 24.0,
        "styled line tm gap should be capped, got {max_tm_gap}"
    );
}

#[test]
fn pdf_renderer_centers_title_block_lines_within_epsilon_v0() {
    let demo_text = b"Centering Accuracy Title\nAlice Bob\n2026-03-05\n\nBody line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    assert!(!layout.pages.is_empty(), "layout should contain a page");
    assert!(
        layout.pages[0].lines.len() >= 3,
        "layout should contain title block lines"
    );

    let expected_title_x = expected_center_x_pt_v0(layout.pages[0].lines[0].width_sp);
    let expected_author_x = expected_center_x_pt_v0(layout.pages[0].lines[1].width_sp);
    let expected_date_x = expected_center_x_pt_v0(layout.pages[0].lines[2].width_sp);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "(Centering Accuracy Title)")
        .expect("title line x");
    let author_x = tm_x_for_line_containing_text_v0(&pdf, "(Alice Bob)").expect("author line x");
    let date_x = tm_x_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line x");

    let epsilon_pt = 0.02f32;
    assert!(
        (title_x - expected_title_x).abs() <= epsilon_pt,
        "title x mismatch: actual={title_x}, expected={expected_title_x}"
    );
    assert!(
        (author_x - expected_author_x).abs() <= epsilon_pt,
        "author x mismatch: actual={author_x}, expected={expected_author_x}"
    );
    assert!(
        (date_x - expected_date_x).abs() <= epsilon_pt,
        "date x mismatch: actual={date_x}, expected={expected_date_x}"
    );
}

#[test]
fn pdf_renderer_applies_hanging_indent_for_list_continuation_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- item\ncontinuation").expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

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
    assert!(xs.len() >= 2, "expected at least two text lines, got {xs:?}");
    assert!((xs[0] - 72.0).abs() <= 0.02, "first list line x={}", xs[0]);
    assert!(xs[1] > xs[0], "continuation should hang-indent: {xs:?}");
}

#[test]
fn pdf_renderer_applies_quote_indent_and_hides_prefix_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n> quoted line\ncontinuation line")
        .expect("writer should accept text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(> quoted line) Tj".len())
            .any(|w| w == b"(> quoted line) Tj")
    );
    assert!(
        pdf.windows(b"(quoted line) Tj".len())
            .any(|w| w == b"(quoted line) Tj")
    );

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
    assert!(xs.len() >= 2, "expected at least two rendered lines, got {xs:?}");
    assert!(xs[0] > 72.0, "quote line should be indented: {xs:?}");
    assert!((xs[0] - xs[1]).abs() <= 0.02, "quote continuation should keep indent: {xs:?}");
}

#[test]
fn pdf_renderer_hides_center_prefix_and_centers_line_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"\n^ centered line").expect("writer should accept centered text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let centered_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| width_sp_for_prefixed_rendered_line_v0(line, [b'^', b' ']).is_some())
        .expect("center-prefixed line");
    let expected_x = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(centered_line, [b'^', b' ']).expect("prefixed width"),
    );
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(^ centered line) Tj".len())
            .any(|w| w == b"(^ centered line) Tj")
    );
    assert!(
        pdf.windows(b"(centered line) Tj".len())
            .any(|w| w == b"(centered line) Tj")
    );
    let x_pt = tm_x_for_line_containing_text_v0(&pdf, "(centered line)")
        .expect("centered Tm position");
    let epsilon_pt = 0.02f32;
    assert!(
        (x_pt - expected_x).abs() <= epsilon_pt,
        "center line x mismatch: actual={x_pt}, expected={expected_x}"
    );
}

#[test]
fn pdf_renderer_hides_right_prefix_and_right_aligns_line_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"\n| right aligned line").expect("writer should accept right text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let right_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| width_sp_for_prefixed_rendered_line_v0(line, [b'|', b' ']).is_some())
        .expect("right-prefixed line");
    let expected_x = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(right_line, [b'|', b' ']).expect("prefixed width"),
    );
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(| right aligned line) Tj".len())
            .any(|w| w == b"(| right aligned line) Tj")
    );
    assert!(
        pdf.windows(b"(right aligned line) Tj".len())
            .any(|w| w == b"(right aligned line) Tj")
    );
    let x_pt = tm_x_for_line_containing_text_v0(&pdf, "(right aligned line)")
        .expect("right-aligned Tm position");
    let epsilon_pt = 0.02f32;
    assert!(
        (x_pt - expected_x).abs() <= epsilon_pt,
        "right line x mismatch: actual={x_pt}, expected={expected_x}"
    );
}

#[test]
fn pdf_renderer_applies_center_alignment_per_line_width_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ center one\n^ center line two")
        .expect("writer should accept centered lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs.iter().map(|glyph| glyph.byte).collect::<Vec<_>>() == b"^ center one"
        })
        .expect("center line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs.iter().map(|glyph| glyph.byte).collect::<Vec<_>>() == b"^ center line two"
        })
        .expect("center line two");

    let expected_one = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'^', b' ']).expect("line one width"),
    );
    let expected_two = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'^', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let x_one = tm_x_for_line_containing_text_v0(&pdf, "(center one)").expect("center one x");
    let x_two =
        tm_x_for_line_containing_text_v0(&pdf, "(center line two)").expect("center line two x");
    let epsilon_pt = 0.02f32;
    assert!((x_one - expected_one).abs() <= epsilon_pt);
    assert!((x_two - expected_two).abs() <= epsilon_pt);
}

#[test]
fn pdf_renderer_applies_right_alignment_per_line_width_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| right one\n| right line two")
        .expect("writer should accept right-aligned lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs.iter().map(|glyph| glyph.byte).collect::<Vec<_>>() == b"| right one"
        })
        .expect("right line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs.iter().map(|glyph| glyph.byte).collect::<Vec<_>>() == b"| right line two"
        })
        .expect("right line two");

    let expected_one = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'|', b' ']).expect("line one width"),
    );
    let expected_two = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'|', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let x_one = tm_x_for_line_containing_text_v0(&pdf, "(right one)").expect("right one x");
    let x_two =
        tm_x_for_line_containing_text_v0(&pdf, "(right line two)").expect("right line two x");
    let epsilon_pt = 0.02f32;
    assert!((x_one - expected_one).abs() <= epsilon_pt);
    assert!((x_two - expected_two).abs() <= epsilon_pt);
}

#[test]
fn parse_roundtrips_writer_layout_for_wrap_and_paging() {
    let text = b"word word word word word word word word word word";
    let layout = plan_layout_v0(text, 65_536, 786_432, 10, 1).expect("layout plan");
    let bytes = write_dvi_v2_text_page_with_layout_wrap_and_paging_v0(text, 65_536, 786_432, 10, 1)
        .expect("writer output");
    let parsed = parse_dvi_v2_text_page_to_layout_v0(&bytes, 786_432).expect("parsed layout");
    assert_eq!(parsed, layout);
    assert!(validate_dvi_v2_text_page_matches_layout_v0(&bytes, &layout, 786_432));
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
    assert!(
        pdf.windows(b"/Helvetica-Oblique".len())
            .any(|w| w == b"/Helvetica-Oblique")
    );
    assert!(
        pdf.windows(b"/Helvetica-Bold".len())
            .any(|w| w == b"/Helvetica-Bold")
    );
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
