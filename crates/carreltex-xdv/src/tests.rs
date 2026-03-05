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
fn planner_space_and_punctuation_advances_are_stable_v0() {
    let plan = plan_layout_width_v0(b"A ,.!? B", 65_536, 786_432, 10_000_000, 200)
        .expect("layout plan");
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
    let plan = plan_layout_width_v0(b"Wm i|.M", 65_536, 786_432, 10_000_000, 200)
        .expect("layout plan");
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

fn tm_count_for_line_containing_v0(pdf: &[u8], needle: &str) -> usize {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut count = 0usize;
        let mut index = 0usize;
        while index + 6 < fields.len() {
            if fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm"
            {
                count += 1;
                index += 7;
                continue;
            }
            index += 1;
        }
        return count;
    }
    0
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

fn tm_position_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Option<(f32, f32)> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 7 || fields[6] != "Tm" {
            continue;
        }
        let x_pt = fields[4].parse::<f32>().ok()?;
        let y_pt = fields[5].parse::<f32>().ok()?;
        return Some((x_pt, y_pt));
    }
    None
}

fn tm_xs_for_segment_text_v0(pdf: &[u8], segment_text: &str) -> Vec<f32> {
    let target_token = format!("({segment_text})");
    let text = String::from_utf8_lossy(pdf);
    let mut xs = Vec::<f32>::new();
    for line in text.lines() {
        if !line.contains(&target_token) || !line.contains(" Tm ") {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index + 6 < fields.len() {
            let is_tm = fields[index] == "1"
                && fields[index + 1] == "0"
                && fields[index + 2] == "0"
                && fields[index + 3] == "1"
                && fields[index + 6] == "Tm";
            if !is_tm {
                index += 1;
                continue;
            }
            let Some(x_pt) = fields[index + 4].parse::<f32>().ok() else {
                index += 7;
                continue;
            };
            let mut cursor = index + 7;
            let mut matched = false;
            while cursor < fields.len() {
                if fields[cursor] == "1"
                    && cursor + 6 < fields.len()
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor] == target_token {
                    matched = true;
                    break;
                }
                cursor += 1;
            }
            if matched {
                xs.push(x_pt);
            }
            index += 7;
        }
    }
    xs
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

fn layout_line_width_for_exact_bytes_v0(layout: &super::LayoutPlanV0, target: &[u8]) -> Option<u32> {
    for page in &layout.pages {
        for line in &page.lines {
            let bytes: Vec<u8> = line.glyphs.iter().map(|glyph| glyph.byte).collect();
            if bytes == target {
                return Some(line.width_sp);
            }
        }
    }
    None
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
fn pdf_renderer_inline_wrapper_spacing_invariants_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"word[mid]word word [lead] trail,{bold}!")
            .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_text = String::from_utf8_lossy(&pdf);
    assert!(
        line_text.contains("(word) Tj /F2 12 Tf (mid) Tj /F1 12 Tf (word word ) Tj /F2 12 Tf (lead) Tj"),
        "styled boundary sequence missing: {line_text}"
    );
    assert!(
        line_text.contains("/F1 12 Tf ( trail,) Tj /F3 12 Tf (bold) Tj /F1 12 Tf (!) Tj"),
        "styled punctuation sequence missing: {line_text}"
    );
    assert!(
        !line_text.contains("(word ) Tj /F2 12 Tf (mid)"),
        "unexpected extra space before inline emph boundary: {line_text}"
    );
    let tm_count = tm_count_for_line_containing_v0(&pdf, "(word)");
    assert_eq!(tm_count, 1, "styled line should use a single Tm: {line_text}");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "(word)")
        .expect("styled line should parse");
    assert!(
        max_tm_gap <= 0.02,
        "styled inline boundary should not create matrix gaps: {max_tm_gap}"
    );
}

#[test]
fn pdf_renderer_punctuation_adjacent_wrapper_gap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"word[mid],word word,[mid]word word{mid}. (alpha[mid]beta) [lead], trail lead, [trail]",
    )
    .expect("writer should accept punctuation-adjacent styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_text = String::from_utf8_lossy(&pdf);
    assert!(
        line_text.contains("(word) Tj /F2 12 Tf (mid) Tj /F1 12 Tf (,word word,) Tj /F2 12 Tf (mid) Tj"),
        "first punctuation-adjacent styled sequence missing: {line_text}"
    );
    assert!(
        line_text.contains("/F3 12 Tf (mid) Tj /F1 12 Tf (. \\(alpha) Tj /F2 12 Tf (mid) Tj"),
        "second punctuation-adjacent styled sequence missing: {line_text}"
    );
    assert!(
        line_text.contains("/F1 12 Tf (beta\\) ) Tj /F2 12 Tf (lead) Tj /F1 12 Tf (, trail lead,) Tj"),
        "wrapper-near-punctuation trail sequence missing: {line_text}"
    );
    assert!(
        line_text.contains("/F2 12 Tf (trail) Tj"),
        "trailing styled segment missing: {line_text}"
    );
    assert!(
        !line_text.contains("word ) Tj /F2 12 Tf (mid)"),
        "unexpected extra space before wrapper boundary: {line_text}"
    );
    assert!(
        !line_text.contains(") Tj /F1 12 Tf ( ,"),
        "unexpected space inserted before punctuation at boundary: {line_text}"
    );
    let tm_count = tm_count_for_line_containing_v0(&pdf, "(word)");
    assert_eq!(
        tm_count, 1,
        "punctuation-adjacent styled line should use single Tm: {line_text}"
    );
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "(word)")
        .expect("punctuation-adjacent styled line should parse");
    assert!(
        max_tm_gap <= 0.02,
        "punctuation-adjacent styled line should not create matrix gaps: {max_tm_gap}"
    );
}

#[test]
fn pdf_renderer_wrapper_punctuation_patterns_are_stable_v0() {
    let cases: [(&[u8], &str, &str); 6] = [
        (
            b"alpha[beta],gamma",
            "(alpha) Tj /F2 12 Tf (beta) Tj /F1 12 Tf (,gamma) Tj",
            "(alpha ) Tj /F2 12 Tf (beta)",
        ),
        (
            b"alpha,[beta]gamma",
            "(alpha,) Tj /F2 12 Tf (beta) Tj /F1 12 Tf (gamma) Tj",
            "(alpha, ) Tj /F2 12 Tf (beta)",
        ),
        (
            b"(alpha[beta]gamma)",
            "(\\(alpha) Tj /F2 12 Tf (beta) Tj /F1 12 Tf (gamma\\)) Tj",
            "(\\(alpha ) Tj /F2 12 Tf (beta)",
        ),
        (
            b"alpha{beta}. gamma",
            "(alpha) Tj /F3 12 Tf (beta) Tj /F1 12 Tf (. gamma) Tj",
            "(alpha ) Tj /F3 12 Tf (beta)",
        ),
        (
            b"{lead}, trail",
            "/F3 12 Tf (lead) Tj /F1 12 Tf (, trail) Tj",
            "/F3 12 Tf (lead) Tj /F1 12 Tf ( , trail) Tj",
        ),
        (
            b"lead, [trail]",
            "(lead, ) Tj /F2 12 Tf (trail) Tj",
            "(lead,) Tj /F2 12 Tf (trail) Tj",
        ),
    ];

    for (input, expected_fragment, forbidden_fragment) in cases {
        let xdv = write_dvi_v2_text_page_v0(input).expect("writer should accept punctuation case");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
        let line_text = String::from_utf8_lossy(&pdf);
        assert!(
            line_text.contains(expected_fragment),
            "expected punctuation fragment missing for input {:?}: {line_text}",
            String::from_utf8_lossy(input),
        );
        assert!(
            !line_text.contains(forbidden_fragment),
            "forbidden punctuation fragment present for input {:?}: {line_text}",
            String::from_utf8_lossy(input),
        );
        let tm_count = tm_count_for_line_containing_v0(&pdf, "(");
        assert_eq!(
            tm_count, 1,
            "punctuation wrapper case should remain single-line Tm for input {:?}: {line_text}",
            String::from_utf8_lossy(input),
        );
        let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "(")
            .expect("punctuation wrapper line should parse");
        assert!(
            max_tm_gap <= 0.02,
            "punctuation wrapper case should not add matrix gaps for input {:?}: {max_tm_gap}",
            String::from_utf8_lossy(input),
        );
    }
}

#[test]
fn pdf_renderer_wrapper_punctuation_segment_positions_progress_monotonically_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"A[mid],B C,[mid]D E{mid}. F")
        .expect("writer should accept wrapper punctuation sequence");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let line_text = String::from_utf8_lossy(&pdf);
    assert!(
        line_text.contains("(A) Tj /F2 12 Tf (mid) Tj /F1 12 Tf (,B C,) Tj /F2 12 Tf (mid) Tj"),
        "expected first styled punctuation sequence missing: {line_text}"
    );
    assert!(
        line_text.contains("/F1 12 Tf (D E) Tj /F3 12 Tf (mid) Tj /F1 12 Tf (. F) Tj"),
        "expected trailing styled punctuation sequence missing: {line_text}"
    );
    let tm_count = tm_count_for_line_containing_v0(&pdf, "(A)");
    assert_eq!(tm_count, 1, "expected single matrix for test line: {line_text}");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "(A)")
        .expect("wrapper punctuation line should parse");
    assert!(
        max_tm_gap <= 0.02,
        "wrapper punctuation matrix gaps should remain zero: {max_tm_gap}"
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
fn pdf_renderer_centers_section_headings_within_epsilon_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nPrelude paragraph.\n\n{Centered Section Heading}\n\n~ Body after centered heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept centered heading text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    assert_eq!(layout.pages.len(), 1);
    let heading_line = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"{Centered Section Heading}"
        })
        .expect("heading line in layout");
    let expected_heading_x = expected_center_x_pt_v0(heading_line.width_sp);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let heading_x = tm_x_for_line_containing_text_v0(&pdf, "(Centered Section Heading)")
        .expect("centered heading position");
    assert!(
        (heading_x - expected_heading_x).abs() <= 0.02,
        "centered heading x mismatch: actual={heading_x}, expected={expected_heading_x}"
    );
    assert!(
        (heading_x - 72.0).abs() > 0.5,
        "heading should not be left-margin aligned: {heading_x}"
    );
}

#[test]
fn pdf_renderer_title_and_heading_centering_per_line_width_v0() {
    let demo_text = b"Centered Title Line\nAlice Bob\n2026-03-05\n\nPrelude paragraph.\n\n{Heading Alpha}\n\n~ Body alpha paragraph.\n\n{Heading Beta}\n\n~ Body beta paragraph.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept title+heading demo");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let title_width = layout_line_width_for_exact_bytes_v0(&layout, b"Centered Title Line")
        .expect("title line width");
    let heading_alpha_width = layout_line_width_for_exact_bytes_v0(&layout, b"{Heading Alpha}")
        .expect("heading alpha width");
    let heading_beta_width = layout_line_width_for_exact_bytes_v0(&layout, b"{Heading Beta}")
        .expect("heading beta width");

    let expected_title_x = expected_center_x_pt_v0(title_width);
    let expected_heading_alpha_x = expected_center_x_pt_v0(heading_alpha_width);
    let expected_heading_beta_x = expected_center_x_pt_v0(heading_beta_width);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "(Centered Title Line)")
        .expect("title x");
    let heading_alpha_x = tm_x_for_line_containing_text_v0(&pdf, "(Heading Alpha)")
        .expect("heading alpha x");
    let heading_beta_x = tm_x_for_line_containing_text_v0(&pdf, "(Heading Beta)")
        .expect("heading beta x");

    let epsilon_pt = 0.02f32;
    assert!(
        (title_x - expected_title_x).abs() <= epsilon_pt,
        "title centering mismatch: actual={title_x}, expected={expected_title_x}"
    );
    assert!(
        (heading_alpha_x - expected_heading_alpha_x).abs() <= epsilon_pt,
        "heading alpha centering mismatch: actual={heading_alpha_x}, expected={expected_heading_alpha_x}"
    );
    assert!(
        (heading_beta_x - expected_heading_beta_x).abs() <= epsilon_pt,
        "heading beta centering mismatch: actual={heading_beta_x}, expected={expected_heading_beta_x}"
    );
    assert!(
        (heading_alpha_x - 72.0).abs() > 0.5 && (heading_beta_x - 72.0).abs() > 0.5,
        "heading lines should not be left-margin aligned: alpha={heading_alpha_x}, beta={heading_beta_x}"
    );
}

#[test]
fn pdf_renderer_paragraph_indent_and_line_gap_invariants_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nFirst body paragraph line.\n\nSecond paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) = tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
        .expect("first body line position");
    let (second_x, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line position");

    assert!((first_x - 72.0).abs() <= 0.02, "first paragraph x mismatch: {first_x}");
    assert!(
        (second_x - 96.0).abs() <= 0.02,
        "indented paragraph x mismatch: {second_x}"
    );
    assert!(
        (first_y - second_y - 28.0).abs() <= 0.02,
        "paragraph y-gap mismatch: first_y={first_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_section_heading_spacing_invariants_v0() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nIntro paragraph.\n\n{Section Heading}\n\n~ Body after heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (intro_x, intro_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Intro paragraph.)").expect("intro position");
    let (_, heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(Section Heading)")
        .expect("heading position");
    assert!(
        !pdf.windows(b"(~ Body after heading.) Tj".len())
            .any(|w| w == b"(~ Body after heading.) Tj")
    );
    let (body_x, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body after heading.)")
        .expect("body position");

    assert!(
        (intro_y - heading_y - 28.0).abs() <= 0.02,
        "intro->heading y-gap mismatch: intro_y={intro_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 28.0).abs() <= 0.02,
        "heading->body y-gap mismatch: heading_y={heading_y}, body_y={body_y}"
    );
    assert!((intro_x - 72.0).abs() <= 0.02, "intro x mismatch: {intro_x}");
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first paragraph after heading should not indent: {body_x}"
    );
}

#[test]
fn pdf_renderer_heading_list_quote_rhythm_invariants_v0() {
    let demo_text = b"\nPrelude paragraph.\n\n{Heading}\n\n~ After heading paragraph.\n\n- First list item\n- Second list item\n\n> Quote line one\n> Quote line two\n\nAfter quote paragraph.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept rhythm text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prelude_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Prelude paragraph.)").expect("prelude position");
    let (_, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading)").expect("heading position");
    let (_, after_heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(After heading paragraph.)")
        .expect("after heading position");
    let (_, list_one_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First list item)").expect("list one");
    let (_, list_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second list item)").expect("list two");
    let (quote_one_x, quote_one_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Quote line one)").expect("quote one");
    let (quote_two_x, quote_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Quote line two)").expect("quote two");
    let (_, after_quote_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After quote paragraph.)")
            .expect("after quote");

    let epsilon_pt = 0.02f32;
    assert!((prelude_y - heading_y - 28.0).abs() <= epsilon_pt);
    assert!(
        (heading_y - after_heading_y - 28.0).abs() <= epsilon_pt,
        "heading->first paragraph gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - list_one_y - 28.0).abs() <= epsilon_pt,
        "paragraph->list gap mismatch: after_heading_y={after_heading_y}, list_one_y={list_one_y}"
    );
    assert!(
        (list_one_y - list_two_y - 14.0).abs() <= epsilon_pt,
        "list line gap mismatch: list_one_y={list_one_y}, list_two_y={list_two_y}"
    );
    assert!(
        (list_two_y - quote_one_y - 28.0).abs() <= epsilon_pt,
        "list->quote gap mismatch: list_two_y={list_two_y}, quote_one_y={quote_one_y}"
    );
    assert!(
        (quote_one_y - quote_two_y - 14.0).abs() <= epsilon_pt,
        "quote line gap mismatch: quote_one_y={quote_one_y}, quote_two_y={quote_two_y}"
    );
    assert!(
        (quote_two_y - after_quote_y - 28.0).abs() <= epsilon_pt,
        "quote->paragraph gap mismatch: quote_two_y={quote_two_y}, after_quote_y={after_quote_y}"
    );
    assert!(quote_one_x > 72.0, "quote line should be indented");
    assert!(
        (quote_one_x - quote_two_x).abs() <= epsilon_pt,
        "quote x drift mismatch"
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
fn pdf_renderer_itemize_bullet_and_body_x_offsets_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- alpha\ncontinuation\n- beta\ncontinuationtwo")
        .expect("writer should accept list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(- alpha) Tj".len()).any(|w| w == b"(- alpha) Tj"),
        "prefix should be split from body"
    );

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let alpha_xs = tm_xs_for_segment_text_v0(&pdf, "alpha");
    let beta_xs = tm_xs_for_segment_text_v0(&pdf, "beta");
    let continuation_xs = tm_xs_for_segment_text_v0(&pdf, "continuation");
    let continuation_two_xs = tm_xs_for_segment_text_v0(&pdf, "continuationtwo");

    assert_eq!(bullet_xs.len(), 2, "expected two bullet renders: {bullet_xs:?}");
    assert_eq!(alpha_xs.len(), 1, "expected alpha render");
    assert_eq!(beta_xs.len(), 1, "expected beta render");
    assert_eq!(continuation_xs.len(), 1, "expected continuation render");
    assert_eq!(continuation_two_xs.len(), 1, "expected continuationtwo render");

    let epsilon_pt = 0.02f32;
    for x in &bullet_xs {
        assert!((*x - 72.0).abs() <= epsilon_pt, "bullet x mismatch: {x}");
    }
    for x in [
        alpha_xs[0],
        beta_xs[0],
        continuation_xs[0],
        continuation_two_xs[0],
    ] {
        assert!((x - 96.0).abs() <= epsilon_pt, "item body x mismatch: {x}");
    }
}

#[test]
fn pdf_renderer_enumerate_number_column_alignment_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"9. nine\n10. ten")
        .expect("writer should accept enumerate text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(
        !pdf.windows(b"(9. nine) Tj".len()).any(|w| w == b"(9. nine) Tj"),
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
    let nine_wrap_x = tm_xs_for_segment_text_v0(&pdf, "WRAPNINE");
    let ten_wrap_x = tm_xs_for_segment_text_v0(&pdf, "WRAPTEN");

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
}

#[test]
fn pdf_renderer_nested_list_indentation_and_wrap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- [OUTERSTART] item with enough repeated words to force wrapping in the first list level before token [OUTERWRAPTOKEN]\n  - [NESTEDSTART] item with enough repeated words to force wrapping in the second list level before token [NESTEDWRAPTOKEN]")
        .expect("writer should accept nested list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let outer_start_x = tm_xs_for_segment_text_v0(&pdf, "OUTERSTART");
    let outer_wrap_x = tm_xs_for_segment_text_v0(&pdf, "OUTERWRAPTOKEN");
    let nested_start_x = tm_xs_for_segment_text_v0(&pdf, "NESTEDSTART");
    let nested_wrap_x = tm_xs_for_segment_text_v0(&pdf, "NESTEDWRAPTOKEN");

    assert_eq!(bullet_xs.len(), 2, "expected two bullets: {bullet_xs:?}");
    assert_eq!(outer_start_x.len(), 1, "expected outer start render");
    assert_eq!(outer_wrap_x.len(), 1, "expected outer wrap render");
    assert_eq!(nested_start_x.len(), 1, "expected nested start render");
    assert_eq!(nested_wrap_x.len(), 1, "expected nested wrap render");

    let epsilon_pt = 0.02f32;
    assert!((bullet_xs[0] - 72.0).abs() <= epsilon_pt, "outer bullet x mismatch");
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
    assert!((x1 - x2).abs() <= epsilon_pt, "quote line x drift: {x1} vs {x2}");
    assert!((x1 - x3).abs() <= epsilon_pt, "quote paragraph x drift: {x1} vs {x3}");
    assert!((x1 - x4).abs() <= epsilon_pt, "quote line x drift: {x1} vs {x4}");
    assert!((y1 - y2 - 14.0).abs() <= epsilon_pt, "quote line gap mismatch");
    assert!(
        (y2 - y3 - 28.0).abs() <= epsilon_pt,
        "quote paragraph gap mismatch"
    );
    assert!((y3 - y4 - 14.0).abs() <= epsilon_pt, "quote line gap mismatch");
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
