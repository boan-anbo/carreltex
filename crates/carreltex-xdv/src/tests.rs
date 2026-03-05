use super::{
    count_dvi_v2_text_movements_v0, count_dvi_v2_text_pages_v0,
    count_dvi_v2_text_pages_with_advance_v0, parse_dvi_v2_text_page_to_layout_v0, plan_layout_v0,
    plan_layout_width_v0, recompute_line_width_sp_v0, render_dvi_v2_text_page_to_pdf_v0,
    sum_dvi_v2_positive_right3_amounts_with_layout_v0, validate_dvi_v2_empty_page_v0,
    validate_dvi_v2_text_page_matches_layout_v0, validate_dvi_v2_text_page_v0,
    validate_dvi_v2_text_page_with_layout_v0, write_dvi_v2_empty_page_v0,
    write_dvi_v2_text_page_from_layout_v0, write_dvi_v2_text_page_v0,
    write_dvi_v2_text_page_with_advance_v0, write_dvi_v2_text_page_with_layout_and_wrap_v0,
    write_dvi_v2_text_page_with_layout_v0, write_dvi_v2_text_page_with_layout_wrap_and_paging_v0,
    LinePlanV0, DVI_DOWN3, DVI_EOP, DVI_FNT_DEF1, DVI_PRE, DVI_RIGHT3, DVI_TRAILER_BYTE,
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

#[test]
fn pdf_renderer_keeps_multi_space_line_unwrapped_under_width_limit_v0() {
    let layout =
        plan_layout_width_v0(b"A     B", 65_536, 786_432, 300_000, 200).expect("layout plan");
    assert_eq!(layout.pages.len(), 1);
    assert_eq!(layout.pages[0].lines.len(), 1);

    let xdv = write_dvi_v2_text_page_from_layout_v0(&layout, 786_432).expect("xdv bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    assert!(pdf
        .windows(b"(A     B) Tj".len())
        .any(|w| w == b"(A     B) Tj"));
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

fn tm_line_start_xs_for_segment_text_v0(pdf: &[u8], segment_text: &str) -> Vec<f32> {
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
            if let Ok(x_pt) = fields[index + 4].parse::<f32>() {
                xs.push(x_pt);
            }
            break;
        }
    }
    xs
}

fn segment_width_pt_v0(segment: &[u8]) -> f32 {
    let layout = plan_layout_width_v0(segment, 65_536, 786_432, 10_000_000, 16)
        .expect("segment layout should parse");
    let line = &layout.pages[0].lines[0];
    line.width_sp as f32 / 65_536.0
}

fn decode_pdf_text_segments_from_line_v0(line: &str) -> Option<String> {
    let mut out = Vec::<u8>::new();
    let bytes = line.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'(' {
            index += 1;
            continue;
        }
        index += 1;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\\' {
                index += 1;
                if index < bytes.len() {
                    out.push(bytes[index]);
                    index += 1;
                }
                continue;
            }
            if byte == b')' {
                index += 1;
                break;
            }
            out.push(byte);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn rendered_text_for_line_containing_segment_v0(pdf: &[u8], segment_text: &str) -> Option<String> {
    let target_token = format!("({segment_text})");
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(&target_token) || !line.contains(" Tj ") {
            continue;
        }
        return decode_pdf_text_segments_from_line_v0(line);
    }
    None
}

fn rendered_text_for_first_text_line_v0(pdf: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if line.contains(" Tj ") {
            return decode_pdf_text_segments_from_line_v0(line);
        }
    }
    None
}

fn parse_tm_positions_in_line_v0(line: &str) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::<(usize, f32, f32)>::new();
    let mut search_from = 0usize;
    while let Some(rel_start) = line[search_from..].find("1 0 0 1 ") {
        let start = search_from + rel_start + "1 0 0 1 ".len();
        let x_end = match line[start..].find(' ') {
            Some(value) => start + value,
            None => break,
        };
        let x_pt = match line[start..x_end].parse::<f32>() {
            Ok(value) => value,
            Err(_) => break,
        };
        let y_start = x_end + 1;
        let y_end = match line[y_start..].find(' ') {
            Some(value) => y_start + value,
            None => break,
        };
        let y_pt = match line[y_start..y_end].parse::<f32>() {
            Ok(value) => value,
            Err(_) => break,
        };
        if !line[y_end..].starts_with(" Tm") {
            break;
        }
        let tm_end = y_end + " Tm".len();
        positions.push((tm_end, x_pt, y_pt));
        search_from = tm_end;
    }
    positions
}

fn tm_x_for_segment_substring_v0(
    pdf: &[u8],
    line_needle: &str,
    segment_substring: &str,
) -> Option<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(line_needle) || !line.contains(segment_substring) {
            continue;
        }
        let target_index = line.find(segment_substring)?;
        let positions = parse_tm_positions_in_line_v0(line);
        let mut best_x = None::<f32>;
        for (tm_end, x_pt, _) in positions {
            if tm_end <= target_index {
                best_x = Some(x_pt);
            }
        }
        return best_x;
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

fn tf_sizes_for_line_containing_text_v0(pdf: &[u8], needle: &str) -> Vec<f32> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let mut sizes = Vec::<f32>::new();
        let mut index = 0usize;
        while index + 1 < fields.len() {
            if fields[index + 1] == "Tf" {
                if let Ok(size_pt) = fields[index].parse::<f32>() {
                    sizes.push(size_pt);
                }
            }
            index += 1;
        }
        return sizes;
    }
    Vec::new()
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

fn tm_position_for_segment_substring_v0(pdf: &[u8], needle: &str) -> Option<(f32, f32)> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
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
            let x_pt = fields[index + 4].parse::<f32>().ok()?;
            let y_pt = fields[index + 5].parse::<f32>().ok()?;
            let mut cursor = index + 7;
            while cursor < fields.len() {
                if cursor + 6 < fields.len()
                    && fields[cursor] == "1"
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor].contains(needle) {
                    return Some((x_pt, y_pt));
                }
                cursor += 1;
            }
            index += 7;
        }
    }
    None
}

fn parse_first_link_rect_v0(pdf: &[u8]) -> Option<[f32; 4]> {
    let text = String::from_utf8_lossy(pdf);
    for line in text.lines() {
        if !line.contains("/Subtype /Link") || !line.contains("/Rect [") {
            continue;
        }
        let rect_start = line.find("/Rect [")?;
        let rect_body_start = rect_start + "/Rect [".len();
        let rect_body_end = line[rect_body_start..].find(']')?;
        let rect_text = &line[rect_body_start..rect_body_start + rect_body_end];
        let values: Vec<f32> = rect_text
            .split_whitespace()
            .filter_map(|value| value.parse::<f32>().ok())
            .collect();
        if values.len() != 4 {
            return None;
        }
        return Some([values[0], values[1], values[2], values[3]]);
    }
    None
}

fn count_pdf_page_objects_v0(pdf: &[u8]) -> usize {
    String::from_utf8_lossy(pdf)
        .matches("/Type /Page /Parent")
        .count()
}

fn parse_pdf_object_body_v0(pdf: &[u8], id: u32) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    let start_token = format!("{id} 0 obj\n");
    let start = text.find(&start_token)? + start_token.len();
    let end = text[start..].find("\nendobj\n")? + start;
    Some(text[start..end].to_string())
}

fn parse_pdf_ref_ids_v0(body: &str, key: &str) -> Vec<u32> {
    let marker = format!("{key} [");
    let Some(start) = body.find(&marker) else {
        return Vec::new();
    };
    let values_start = start + marker.len();
    let Some(values_end_rel) = body[values_start..].find(']') else {
        return Vec::new();
    };
    body[values_start..values_start + values_end_rel]
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| match chunk {
            [id, "0", "R"] => id.parse::<u32>().ok(),
            _ => None,
        })
        .collect()
}

fn parse_pdf_annotation_action_id_v0(body: &str) -> Option<u32> {
    let marker = "/A ";
    let start = body.find(marker)? + marker.len();
    let fields = body[start..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 || fields[1] != "0" || fields[2] != "R" {
        return None;
    }
    fields[0].parse::<u32>().ok()
}

fn parse_pdf_annotation_dest_page_id_v0(body: &str) -> Option<u32> {
    let marker = "/Dest [";
    let start = body.find(marker)? + marker.len();
    let fields = body[start..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 || fields[1] != "0" || fields[2] != "R" {
        return None;
    }
    fields[0].parse::<u32>().ok()
}

fn parse_pdf_action_uri_v0(body: &str) -> Option<String> {
    let marker = "/URI (";
    let start = body.find(marker)? + marker.len();
    let end = body[start..].find(')')? + start;
    Some(body[start..end].to_string())
}

fn parse_pdf_annotation_rect_v0(body: &str) -> Option<[f32; 4]> {
    let marker = "/Rect [";
    let start = body.find(marker)? + marker.len();
    let end = body[start..].find(']')? + start;
    let values = body[start..end]
        .split_whitespace()
        .filter_map(|value| value.parse::<f32>().ok())
        .collect::<Vec<_>>();
    if values.len() != 4 {
        return None;
    }
    Some([values[0], values[1], values[2], values[3]])
}

fn tm_position_for_line_containing_text_in_body_v0(body: &str, needle: &str) -> Option<(f32, f32)> {
    for line in body.lines() {
        if !line.contains(needle) || !line.contains(" Tm ") {
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
            let x_pt = fields[index + 4].parse::<f32>().ok()?;
            let y_pt = fields[index + 5].parse::<f32>().ok()?;
            let mut cursor = index + 7;
            while cursor < fields.len() {
                if cursor + 6 < fields.len()
                    && fields[cursor] == "1"
                    && fields[cursor + 1] == "0"
                    && fields[cursor + 2] == "0"
                    && fields[cursor + 3] == "1"
                    && fields[cursor + 6] == "Tm"
                {
                    break;
                }
                if fields[cursor].contains(needle) {
                    return Some((x_pt, y_pt));
                }
                cursor += 1;
            }
            index += 7;
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

fn layout_line_width_for_exact_bytes_v0(
    layout: &super::LayoutPlanV0,
    target: &[u8],
) -> Option<u32> {
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
    let xdv = write_dvi_v2_text_page_v0(b"word[mid]word word [lead] trail,{bold}!")
        .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "word")
        .expect("styled line should decode");
    assert_eq!(rendered, "wordmidword word lead trail,bold!");

    let word_x = tm_xs_for_segment_text_v0(&pdf, "word")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let trailing_word_x =
        tm_x_for_segment_substring_v0(&pdf, "(word)", "(word word )").expect("word word segment x");
    let lead_x = tm_xs_for_segment_text_v0(&pdf, "lead")[0];
    let trail_x =
        tm_x_for_segment_substring_v0(&pdf, "(word)", "( trail,)").expect("trail segment x");
    let bold_x = tm_xs_for_segment_text_v0(&pdf, "bold")[0];

    let epsilon_pt = 0.02f32;
    assert!(
        ((mid_x - word_x) - segment_width_pt_v0(b"word")).abs() <= epsilon_pt,
        "word->mid advance mismatch: word_x={word_x}, mid_x={mid_x}"
    );
    assert!(
        ((trailing_word_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->trailing advance mismatch: mid_x={mid_x}, trailing_word_x={trailing_word_x}"
    );
    assert!(
        ((trail_x - lead_x) - segment_width_pt_v0(b"lead")).abs() <= epsilon_pt,
        "lead->trail advance mismatch: lead_x={lead_x}, trail_x={trail_x}"
    );
    assert!(
        ((bold_x - trail_x) - segment_width_pt_v0(b" trail,")).abs() <= epsilon_pt,
        "trail->bold advance mismatch: trail_x={trail_x}, bold_x={bold_x}"
    );
}

#[test]
fn pdf_renderer_punctuation_adjacent_wrapper_gap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"word[mid],word word,[mid]word word{mid}. (a[mid]b) lead, [trail]",
    )
    .expect("writer should accept punctuation-adjacent styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "word")
        .expect("punctuation-adjacent line should decode");
    assert_eq!(
        rendered,
        "wordmid,word word,midword wordmid. (amidb) lead, trail"
    );

    let word_x = tm_xs_for_segment_text_v0(&pdf, "word")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let comma_word_x = tm_x_for_segment_substring_v0(&pdf, "(word)", "(,word word,)")
        .expect("comma-word segment x");
    let epsilon_pt = 0.02f32;
    assert!(
        ((mid_x - word_x) - segment_width_pt_v0(b"word")).abs() <= epsilon_pt,
        "word->mid punctuation boundary drifted: word_x={word_x}, mid_x={mid_x}"
    );
    assert!(
        ((comma_word_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->comma segment boundary drifted: mid_x={mid_x}, comma_word_x={comma_word_x}"
    );
}

#[test]
fn pdf_renderer_wrapper_punctuation_patterns_are_stable_v0() {
    let cases: [(&[u8], &str); 6] = [
        (b"alpha[beta],gamma", "alphabeta,gamma"),
        (b"alpha,[beta]gamma", "alpha,betagamma"),
        (b"(alpha[beta]gamma)", "(alphabetagamma)"),
        (b"alpha{beta}. gamma", "alphabeta. gamma"),
        (b"{lead}, trail", "lead, trail"),
        (b"lead, [trail]", "lead, trail"),
    ];

    for (input, expected_rendered) in cases {
        let xdv = write_dvi_v2_text_page_v0(input).expect("writer should accept punctuation case");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
        let rendered =
            rendered_text_for_first_text_line_v0(&pdf).expect("punctuation case should decode");
        assert!(
            rendered == expected_rendered,
            "rendered punctuation mismatch for input {:?}: got {:?}, want {:?}",
            String::from_utf8_lossy(input),
            rendered,
            expected_rendered,
        );
    }
}

#[test]
fn pdf_renderer_wrapper_punctuation_segment_positions_progress_monotonically_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"A[mid],B C,[mid]D E{mid}. F")
        .expect("writer should accept wrapper punctuation sequence");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered = rendered_text_for_line_containing_segment_v0(&pdf, "A")
        .expect("wrapper punctuation line should decode");
    assert_eq!(rendered, "Amid,B C,midD Emid. F");

    let a_x = tm_xs_for_segment_text_v0(&pdf, "A")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let trailing_x = tm_x_for_segment_substring_v0(&pdf, "(A)", "(,B C,)")
        .expect("trailing punctuation segment x");
    let mid_two_x = tm_xs_for_segment_text_v0(&pdf, "mid")[1];
    assert!(a_x < mid_x && mid_x < trailing_x && trailing_x < mid_two_x);
    let epsilon_pt = 0.02f32;
    assert!(
        ((mid_x - a_x) - segment_width_pt_v0(b"A")).abs() <= epsilon_pt,
        "A->mid boundary drifted: a_x={a_x}, mid_x={mid_x}"
    );
    assert!(
        ((trailing_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->trail boundary drifted: mid_x={mid_x}, trailing_x={trailing_x}"
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
    let title_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Centering Accuracy Title)").expect("title line x");
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
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept centered heading text");
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
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept title+heading demo");
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
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "(Centered Title Line)").expect("title x");
    let heading_alpha_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Heading Alpha)").expect("heading alpha x");
    let heading_beta_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Heading Beta)").expect("heading beta x");

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
fn pdf_renderer_heading_font_hierarchy_invariants_v0() {
    let demo_text = b"Typography Title\nAlice Bob\n2026-03-05\n\n@S {Section Heading}\n\n@s {Subsection Heading}\n\nBody paragraph text.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept heading font demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let title_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Typography Title)");
    let section_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Section Heading)");
    let subsection_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Subsection Heading)");
    let body_sizes = tf_sizes_for_line_containing_text_v0(&pdf, "(Body paragraph text.)");

    assert!(!title_sizes.is_empty(), "missing title sizes");
    assert!(!section_sizes.is_empty(), "missing section sizes");
    assert!(!subsection_sizes.is_empty(), "missing subsection sizes");
    assert!(!body_sizes.is_empty(), "missing body sizes");

    let title_size = title_sizes[0];
    let section_size = section_sizes[0];
    let subsection_size = subsection_sizes[0];
    let body_size = body_sizes[0];

    assert!(
        (title_size - 18.0).abs() <= 0.02,
        "title font size mismatch: {title_size}"
    );
    assert!(
        (section_size - 16.0).abs() <= 0.02,
        "section font size mismatch: {section_size}"
    );
    assert!(
        (subsection_size - 14.0).abs() <= 0.02,
        "subsection font size mismatch: {subsection_size}"
    );
    assert!(
        (body_size - 12.0).abs() <= 0.02,
        "body font size mismatch: {body_size}"
    );
    assert!(
        title_size > section_size && section_size > subsection_size && subsection_size > body_size,
        "font hierarchy must be strict: title={title_size}, section={section_size}, subsection={subsection_size}, body={body_size}"
    );
}

#[test]
fn pdf_renderer_paragraph_rhythm_and_noindent_invariants_v0() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nFirst paragraph line one.\nSecond line same paragraph.\n\nSecond paragraph line.\n\n@S {Heading}\n\n~ After heading noindent line.\n\nIndented paragraph line.";
    let xdv =
        write_dvi_v2_text_page_v0(demo_text).expect("writer should accept paragraph rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First paragraph line one.)")
            .expect("first paragraph line one");
    let (same_para_x, same_para_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second line same paragraph.)")
            .expect("same paragraph line");
    let (second_para_x, second_para_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line");
    let (heading_x, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading)").expect("heading line");
    let (after_heading_x, after_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After heading noindent line.)")
            .expect("after heading line");
    let (indented_x, indented_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Indented paragraph line.)")
            .expect("indented paragraph line");

    let epsilon_pt = 0.02f32;
    assert!(
        (first_x - 72.0).abs() <= epsilon_pt,
        "first paragraph x mismatch: {first_x}"
    );
    assert!(
        (same_para_x - 72.0).abs() <= epsilon_pt,
        "same paragraph line should stay non-indented: {same_para_x}"
    );
    assert!(
        (second_para_x - 96.0).abs() <= epsilon_pt,
        "second paragraph indent mismatch: {second_para_x}"
    );
    assert!(
        (after_heading_x - 72.0).abs() <= epsilon_pt,
        "first paragraph after heading should noindent: {after_heading_x}"
    );
    assert!(
        (indented_x - 96.0).abs() <= epsilon_pt,
        "paragraph after noindent should restore indent: {indented_x}"
    );
    assert!(
        (first_y - same_para_y - 14.0).abs() <= epsilon_pt,
        "line gap mismatch inside paragraph: first_y={first_y}, same_para_y={same_para_y}"
    );
    assert!(
        (same_para_y - second_para_y - 28.0).abs() <= epsilon_pt,
        "paragraph break gap mismatch: same_para_y={same_para_y}, second_para_y={second_para_y}"
    );
    assert!(
        (second_para_y - heading_y - 28.0).abs() <= epsilon_pt,
        "paragraph->heading gap mismatch: second_para_y={second_para_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - after_heading_y - 28.0).abs() <= epsilon_pt,
        "heading->noindent gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - indented_y - 28.0).abs() <= epsilon_pt,
        "noindent->indented paragraph gap mismatch: after_heading_y={after_heading_y}, indented_y={indented_y}"
    );
    assert!(
        (heading_x - 72.0).abs() > 0.5,
        "heading should be centered: {heading_x}"
    );
}

#[test]
fn pdf_renderer_list_rhythm_and_wrap_indent_invariants_v0() {
    let demo_text = b"Paragraph before list.\n\n- ITEMONE lead words with deterministic wrapping content to force continuation line token WRAPONE after many repeated words in this same item.\n- ITEMTWO lead words with deterministic wrapping content to force continuation line token WRAPTWO after many repeated words in this same item.\n\nParagraph after list.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept list rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, before_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph before list.)")
            .expect("before list paragraph");
    let (item_one_bullet_x, item_one_y) =
        tm_position_for_segment_substring_v0(&pdf, "(-)").expect("item one bullet position");
    let (item_one_body_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "(ITEMONE").expect("item one body position");
    let (item_one_wrap_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPONE").expect("item one wrap position");
    let (item_two_bullet_x, item_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "(ITEMTWO").expect("item two body position");
    let (item_two_wrap_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTWO").expect("item two wrap position");
    let (_, after_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after list.)")
            .expect("after list paragraph");

    let epsilon_pt = 0.02f32;
    assert!(
        (28.0 - epsilon_pt..=56.0 + epsilon_pt).contains(&(before_list_y - item_one_y)),
        "before->list top gap out of range: before_list_y={before_list_y}, item_one_y={item_one_y}"
    );
    assert!(
        (item_two_y - after_list_y).abs() >= 28.0 - epsilon_pt,
        "list->after paragraph gap must be at least one paragraph break: item_two_y={item_two_y}, after_list_y={after_list_y}"
    );
    assert!(
        (item_one_bullet_x - 72.0).abs() <= epsilon_pt,
        "item bullet column x mismatch: {item_one_bullet_x}"
    );
    assert!(
        (item_one_body_x - 96.0).abs() <= epsilon_pt,
        "item body x mismatch: {item_one_body_x}"
    );
    assert!(
        (item_one_wrap_x - item_one_body_x).abs() <= epsilon_pt,
        "item one wrap continuation should keep hanging indent: body={item_one_body_x}, wrap={item_one_wrap_x}"
    );
    assert!(
        (item_two_bullet_x - 96.0).abs() <= epsilon_pt,
        "item two body x mismatch: {item_two_bullet_x}"
    );
    assert!(
        (item_two_wrap_x - item_two_bullet_x).abs() <= epsilon_pt,
        "item two wrap continuation should keep hanging indent: body={item_two_bullet_x}, wrap={item_two_wrap_x}"
    );
}

#[test]
fn pdf_renderer_paragraph_indent_and_line_gap_invariants_v0() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nFirst body paragraph line.\n\nSecond paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("first body line position");
    let (second_x, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line position");

    assert!(
        (first_x - 72.0).abs() <= 0.02,
        "first paragraph x mismatch: {first_x}"
    );
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

    let (intro_x, intro_y) = tm_position_for_line_containing_text_v0(&pdf, "(Intro paragraph.)")
        .expect("intro position");
    let (_, heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(Section Heading)")
        .expect("heading position");
    assert!(!pdf
        .windows(b"(~ Body after heading.) Tj".len())
        .any(|w| w == b"(~ Body after heading.) Tj"));
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
    assert!(
        (intro_x - 72.0).abs() <= 0.02,
        "intro x mismatch: {intro_x}"
    );
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

    let (_, prelude_y) = tm_position_for_line_containing_text_v0(&pdf, "(Prelude paragraph.)")
        .expect("prelude position");
    let (_, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading)").expect("heading position");
    let (_, after_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After heading paragraph.)")
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
    let xdv =
        write_dvi_v2_text_page_v0(b"- item\ncontinuation").expect("writer should accept text");
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
    assert!(
        xs.len() >= 2,
        "expected at least two text lines, got {xs:?}"
    );
    assert!((xs[0] - 72.0).abs() <= 0.02, "first list line x={}", xs[0]);
    assert!(xs[1] > xs[0], "continuation should hang-indent: {xs:?}");
}

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
}

#[test]
fn pdf_renderer_nested_list_indentation_and_wrap_invariants_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"- [OUTERSTART] item with enough repeated words to force wrapping in the first list level before token [OUTERWRAPTOKEN]\n  - [NESTEDSTART] item with enough repeated words to force wrapping in the second list level before token [NESTEDWRAPTOKEN]")
        .expect("writer should accept nested list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let bullet_xs = tm_xs_for_segment_text_v0(&pdf, "-");
    let outer_start_x = tm_xs_for_segment_text_v0(&pdf, "OUTERSTART");
    let outer_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "OUTERWRAPTOKEN");
    let nested_start_x = tm_xs_for_segment_text_v0(&pdf, "NESTEDSTART");
    let nested_wrap_x = tm_line_start_xs_for_segment_text_v0(&pdf, "NESTEDWRAPTOKEN");

    assert_eq!(bullet_xs.len(), 2, "expected two bullets: {bullet_xs:?}");
    assert_eq!(outer_start_x.len(), 1, "expected outer start render");
    assert_eq!(outer_wrap_x.len(), 1, "expected outer wrap render");
    assert_eq!(nested_start_x.len(), 1, "expected nested start render");
    assert_eq!(nested_wrap_x.len(), 1, "expected nested wrap render");

    let epsilon_pt = 0.02f32;
    assert!(
        (bullet_xs[0] - 72.0).abs() <= epsilon_pt,
        "outer bullet x mismatch"
    );
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
    assert!(xs[0] > 72.0, "quote line should be indented: {xs:?}");
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
        (y1 - y2 - 14.0).abs() <= epsilon_pt,
        "quote line gap mismatch"
    );
    assert!(
        (y2 - y3 - 28.0).abs() <= epsilon_pt,
        "quote paragraph gap mismatch"
    );
    assert!(
        (y3 - y4 - 14.0).abs() <= epsilon_pt,
        "quote line gap mismatch"
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
fn pdf_renderer_hides_right_prefix_and_right_aligns_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| right aligned line")
        .expect("writer should accept right text");
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
    assert!(!pdf
        .windows(b"(| right aligned line) Tj".len())
        .any(|w| w == b"(| right aligned line) Tj"));
    assert!(pdf
        .windows(b"(right aligned line) Tj".len())
        .any(|w| w == b"(right aligned line) Tj"));
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
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ center one"
        })
        .expect("center line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ center line two"
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
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| right one"
        })
        .expect("right line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| right line two"
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
fn pdf_renderer_center_alignment_handles_styled_segments_without_drift_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n^ alpha[mid],gamma\n^ short{bold}.")
        .expect("writer should accept styled centered lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ alpha[mid],gamma"
        })
        .expect("center styled line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"^ short{bold}."
        })
        .expect("center styled line two");

    let expected_one = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'^', b' ']).expect("line one width"),
    );
    let expected_two = expected_center_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'^', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_one_x = tm_x_for_line_containing_text_v0(&pdf, "(alpha)").expect("line one x");
    let line_two_x = tm_x_for_line_containing_text_v0(&pdf, "(short)").expect("line two x");
    let epsilon_pt = 0.02f32;
    assert!(
        (line_one_x - expected_one).abs() <= epsilon_pt,
        "line one center drift: actual={line_one_x}, expected={expected_one}"
    );
    assert!(
        (line_two_x - expected_two).abs() <= epsilon_pt,
        "line two center drift: actual={line_two_x}, expected={expected_two}"
    );

    let alpha_x = tm_xs_for_segment_text_v0(&pdf, "alpha")[0];
    let mid_x = tm_xs_for_segment_text_v0(&pdf, "mid")[0];
    let gamma_x =
        tm_x_for_segment_substring_v0(&pdf, "(alpha)", "(,gamma)").expect("gamma segment x");
    assert!(
        ((mid_x - alpha_x) - segment_width_pt_v0(b"alpha")).abs() <= epsilon_pt,
        "alpha->mid spacing drift: alpha_x={alpha_x}, mid_x={mid_x}"
    );
    assert!(
        ((gamma_x - mid_x) - segment_width_pt_v0(b"mid")).abs() <= epsilon_pt,
        "mid->gamma spacing drift: mid_x={mid_x}, gamma_x={gamma_x}"
    );
}

#[test]
fn pdf_renderer_right_alignment_handles_styled_segments_without_drift_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"\n| edge, [core] trail\n| alpha{beta}.")
        .expect("writer should accept styled right-aligned lines");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let line_one = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| edge, [core] trail"
        })
        .expect("right styled line one");
    let line_two = layout.pages[0]
        .lines
        .iter()
        .find(|line| {
            line.glyphs
                .iter()
                .map(|glyph| glyph.byte)
                .collect::<Vec<_>>()
                == b"| alpha{beta}."
        })
        .expect("right styled line two");

    let expected_one = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_one, [b'|', b' ']).expect("line one width"),
    );
    let expected_two = expected_right_x_pt_v0(
        width_sp_for_prefixed_rendered_line_v0(line_two, [b'|', b' ']).expect("line two width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let line_one_x = tm_x_for_line_containing_text_v0(&pdf, "(edge, )").expect("line one x");
    let line_two_x = tm_x_for_line_containing_text_v0(&pdf, "(alpha)").expect("line two x");
    let epsilon_pt = 0.02f32;
    assert!(
        (line_one_x - expected_one).abs() <= epsilon_pt,
        "line one right drift: actual={line_one_x}, expected={expected_one}"
    );
    assert!(
        (line_two_x - expected_two).abs() <= epsilon_pt,
        "line two right drift: actual={line_two_x}, expected={expected_two}"
    );

    let edge_x =
        tm_x_for_segment_substring_v0(&pdf, "(edge, )", "(edge, )").expect("edge segment x");
    let core_x = tm_xs_for_segment_text_v0(&pdf, "core")[0];
    let trail_x =
        tm_x_for_segment_substring_v0(&pdf, "(edge, )", "( trail)").expect("trail segment x");
    assert!(
        ((core_x - edge_x) - segment_width_pt_v0(b"edge, ")).abs() <= epsilon_pt,
        "edge->core spacing drift: edge_x={edge_x}, core_x={core_x}"
    );
    assert!(
        ((trail_x - core_x) - segment_width_pt_v0(b"core")).abs() <= epsilon_pt,
        "core->trail spacing drift: core_x={core_x}, trail_x={trail_x}"
    );
}

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
        (72.0..=140.0).contains(&footnote_pos.1),
        "footnote block should stay near page bottom margin: y={}",
        footnote_pos.1
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
    assert_eq!(annots.len(), 1, "expected one internal figure ref annotation");
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
        b"Before.\n\n!t Alpha||Beta||Gamma\n!t Delta||Epsilon||Zeta\n\nAfter.",
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
        b"Before.\n\n!t A||WideMiddle||9.9\n!t LongLeft||B||123.45\n\nAfter.",
    )
    .expect("writer should accept table marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let left_margin_pt = 72.0f32;
    let cell_padding_pt = 6.0f32;
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
fn pdf_renderer_table_grid_lines_render_deterministically_v1() {
    let xdv = write_dvi_v2_text_page_v0(b"Before.\n\n!t A||B||C\n!t D||E||F\n\nAfter.")
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
fn pdf_renderer_rejects_table_width_overflow_v0() {
    let mut row = Vec::<u8>::new();
    row.extend_from_slice(b"!t ");
    row.extend_from_slice("W".repeat(400).as_bytes());
    row.extend_from_slice(b"||Center||Right");
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
fn pdf_renderer_renders_toc_block_with_level_indentation_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before paragraph.\n\n!toc\n\nAfter paragraph.\n\n!toc 1 1 Intro entry\n!toc 2 2 Detail entry",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert!(
        !pdf_text.contains("!toc 1 1"),
        "toc metadata lines must not be visible"
    );
    assert!(
        !pdf_text.contains("!toc 2 2"),
        "toc metadata lines must not be visible"
    );
    assert!(
        pdf_text.contains("(Contents) Tj"),
        "toc title should render"
    );
    assert!(
        pdf_text.contains("(Intro entry) Tj"),
        "toc level 1 entry should render"
    );
    assert!(
        pdf_text.contains("(Detail entry) Tj"),
        "toc level 2 entry should render"
    );

    let intro_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Intro entry)").expect("toc level 1 position");
    let detail_x =
        tm_x_for_line_containing_text_v0(&pdf, "(Detail entry)").expect("toc level 2 position");
    assert!(
        detail_x > intro_x + 8.0,
        "toc level 2 should be indented from level 1: {intro_x} vs {detail_x}"
    );
}

#[test]
fn pdf_renderer_emits_toc_link_annotations_targeting_heading_anchors_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Prelude.\n\n!toc\n\n@S {Intro section}\n\n@s {Detail section}\n\n!toc 1 1 <Intro section>\n!toc 2 2 <Detail section>",
    )
    .expect("writer should accept toc metadata and heading lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Intro section) Tj"),
        "toc section title should render: {pdf_text}"
    );
    assert!(
        pdf_text.contains("(Detail section) Tj"),
        "toc subsection title should render: {pdf_text}"
    );

    let page_one = parse_pdf_object_body_v0(&pdf, 3).expect("page object");
    let annots = parse_pdf_ref_ids_v0(&page_one, "/Annots");
    assert_eq!(annots.len(), 2, "toc block should emit two annotations");

    for annotation_id in annots {
        let annotation = parse_pdf_object_body_v0(&pdf, annotation_id).expect("annotation body");
        assert!(
            annotation.contains("/Dest ["),
            "toc links should use internal destinations: {annotation}"
        );
        assert!(
            !annotation.contains("/A "),
            "toc links should not use URI actions: {annotation}"
        );
        assert_eq!(
            parse_pdf_annotation_dest_page_id_v0(&annotation),
            Some(3),
            "toc links should target page containing heading anchors"
        );
        let rect = parse_pdf_annotation_rect_v0(&annotation).expect("annotation rect");
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
}

#[test]
fn pdf_renderer_rejects_toc_link_annotation_with_missing_anchor_destination_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Prelude.\n\n!toc\n\n!toc 1 9 <Missing anchor>")
        .expect("writer bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed when toc link anchor target is missing"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entries_with_unsupported_level_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 3 1 Too deep")
        .expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed for unsupported toc level"
    );
}

#[test]
fn pdf_renderer_toc_block_renders_between_surrounding_paragraphs_v0() {
    let xdv = write_dvi_v2_text_page_v0(
        b"Before block.\n\n!toc\n\nAfter block.\n\n!toc 1 1 Intro\n!toc 2 2 Detail",
    )
    .expect("writer should accept toc marker lines");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (before_x, before_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Before block.)").expect("before");
    let (toc_title_x, toc_title_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Contents)").expect("toc title");
    let (_, toc_intro_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Intro)").expect("toc intro");
    let (_, toc_detail_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Detail)").expect("toc detail");
    let (after_x, after_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After block.)").expect("after");

    assert!(
        before_y > toc_title_y,
        "toc block should appear below preceding paragraph"
    );
    assert!(
        toc_title_y > toc_intro_y,
        "toc entries should appear below toc title"
    );
    assert!(
        toc_intro_y > toc_detail_y,
        "toc level 2 line should appear below level 1 line"
    );
    assert!(
        toc_detail_y > after_y,
        "toc block should appear above following paragraph"
    );
    assert!(
        toc_title_x >= 72.0,
        "toc title should remain in printable area"
    );
    assert!(
        before_x > 0.0 && after_x > 0.0,
        "paragraph coordinates must be valid"
    );
}

#[test]
fn pdf_renderer_toc_without_entries_renders_title_only_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"Intro.\n\n!toc\n\nOutro.").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        pdf_text.contains("(Contents) Tj"),
        "toc title should render"
    );
    assert!(
        !pdf_text.contains("!toc"),
        "toc placeholder should be hidden"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_non_numeric_anchor_id_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 abc Intro").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on non-numeric toc anchor ids"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_empty_title_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 1 ").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on empty toc titles"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_zero_anchor_id_v0() {
    let xdv =
        write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 0 Intro").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on zero toc anchors"
    );
}

#[test]
fn pdf_renderer_rejects_toc_entry_with_unterminated_metadata_shape_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1").expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on malformed toc metadata lines"
    );
}

#[test]
fn pdf_renderer_rejects_duplicate_toc_anchor_ids_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"!toc\n\n!toc 1 1 Intro\n!toc 2 1 Duplicate anchor")
        .expect("writer should accept bytes");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv);
    assert!(
        pdf.is_none(),
        "renderer should fail-closed on duplicate toc anchors"
    );
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
