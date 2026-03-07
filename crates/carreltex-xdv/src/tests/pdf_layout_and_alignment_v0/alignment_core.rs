use super::super::*;

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
fn pdf_renderer_center_alignment_keeps_wrapped_continuation_centered_v1() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART alpha [mid] gamma words words words words WRAPCENTER tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let expected_start_width_pt =
        segment_width_pt_v0(b"CENTERSTART alpha ") + scaled_segment_width_pt_v0(b"mid", 95);
    let expected_start_x =
        (612.0 - expected_start_width_pt) / 2.0;
    let expected_wrap_x = expected_center_x_pt_v0(
        layout_render_width_for_substring_v0(&layout, b"WRAPCENTER").expect("center wrap width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) = tm_position_for_line_containing_text_v0(&pdf, "CENTERSTART")
        .expect("center start line position");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPCENTER").expect("center wrap line position");
    let epsilon_pt = 0.02f32;
    assert!(
        (start_x - expected_start_x).abs() <= epsilon_pt,
        "center wrapped first line drift: actual={start_x}, expected={expected_start_x}"
    );
    assert!(
        (wrap_x - expected_wrap_x).abs() <= epsilon_pt,
        "center wrapped continuation drift: actual={wrap_x}, expected={expected_wrap_x}"
    );
    assert!(start_y > wrap_y, "wrapped continuation should render below first line");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERSTART")
        .expect("center wrapped styled line should decode");
    assert!(
        rendered == "CENTERSTART alpha mid",
        "center wrapped style boundaries should retain stable spacing: {rendered}"
    );
    let pdf_text = String::from_utf8_lossy(&pdf);
    let centered_line = pdf_text
        .lines()
        .find(|line| line.contains("(mid) Tj"))
        .expect("center wrapped styled segment should render");
    assert!(
        centered_line.contains("95 Tz") && centered_line.contains("(mid) Tj 100 Tz"),
        "wrapped centered styled segment should use v28 seam compensation"
    );
}

#[test]
fn pdf_renderer_right_alignment_keeps_wrapped_continuation_right_v1() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHTSTART edge, [core] trail words words words words WRAPRIGHT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right-aligned text");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let expected_start_width_pt =
        segment_width_pt_v0(b"RIGHTSTART edge, ") + scaled_segment_width_pt_v0(b"core", 95);
    let expected_start_x = 540.0 - expected_start_width_pt;
    let expected_wrap_x = expected_right_x_pt_v0(
        layout_render_width_for_substring_v0(&layout, b"WRAPRIGHT").expect("right wrap width"),
    );

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) = tm_position_for_line_containing_text_v0(&pdf, "RIGHTSTART")
        .expect("right start line position");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPRIGHT").expect("right wrap line position");
    let epsilon_pt = 0.02f32;
    assert!(
        (start_x - expected_start_x).abs() <= epsilon_pt,
        "right wrapped first line drift: actual={start_x}, expected={expected_start_x}"
    );
    assert!(
        (wrap_x - expected_wrap_x).abs() <= epsilon_pt,
        "right wrapped continuation drift: actual={wrap_x}, expected={expected_wrap_x}"
    );
    assert!(start_y > wrap_y, "wrapped continuation should render below first line");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTSTART")
        .expect("right wrapped styled line should decode");
    assert!(
        rendered == "RIGHTSTART edge, core",
        "right wrapped style boundaries should retain stable spacing: {rendered}"
    );
    let pdf_text = String::from_utf8_lossy(&pdf);
    let right_line = pdf_text
        .lines()
        .find(|line| line.contains("(core) Tj"))
        .expect("right wrapped styled segment should render");
    assert!(
        right_line.contains("95 Tz") && right_line.contains("(core) Tj 100 Tz"),
        "wrapped right styled segment should use v28 seam compensation"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_pre_style_gap_is_tightened_v41() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART alpha [mid] gamma words words words words WRAPCENTER tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered v41 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTERSTART").expect("centered prefix y");
    let (_, wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPCENTER").expect("centered wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERSTART")
        .expect("centered rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "mid").expect("centered tm gap");
    assert!(
        rendered == "CENTERSTART alpha mid",
        "wrapped centered line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 128.0,
        "wrapped centered pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}
