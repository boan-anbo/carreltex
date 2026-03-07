use super::super::*;

#[test]
fn pdf_renderer_wrapped_right_pre_style_gap_is_tightened_v41() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHTSTART edge, [core] trail words words words words WRAPRIGHT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right v41 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHTSTART").expect("right prefix y");
    let (_, wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHT").expect("right wrap y");
    let rendered =
        rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTSTART").expect("right rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core").expect("right tm gap");
    assert!(
        rendered == "RIGHTSTART edge, core",
        "wrapped right line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 110.0,
        "wrapped right pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_bold_pre_style_gap_is_tightened_v49() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHTSTART edge, {core} trail words words words words WRAPRIGHT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (prefix_x, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHTSTART").expect("right bold prefix position");
    let (_, wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHT").expect("right bold wrap y");
    let rendered =
        rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTSTART").expect("right bold rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core").expect("right bold tm gap");

    assert!(
        rendered == "RIGHTSTART edge, core",
        "wrapped right bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 112.0,
        "wrapped right bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y && prefix_x >= 72.0,
        "right bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}, prefix_x={prefix_x}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_bold_pre_style_gap_is_tightened_v50() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART edge, {core} trail words words words words WRAPCENTER tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTERSTART").expect("centered bold prefix y");
    let (_, wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPCENTER").expect("centered bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERSTART")
        .expect("centered bold rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core").expect("centered bold tm gap");

    assert!(
        rendered == "CENTERSTART edge, core",
        "wrapped centered bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 126.0,
        "wrapped centered bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_bold_pre_style_gap_is_tightened_v51() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface {core words} trail words words WRAPCENTERMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) = tm_position_for_segment_substring_v0(&pdf, "CENTER")
        .expect("centered medium bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERMED")
        .expect("centered medium bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered medium bold rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core words").expect("centered medium bold tm gap");

    assert!(
        rendered == "CENTER preface core words",
        "wrapped centered medium bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 120.0,
        "wrapped centered medium bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered medium bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_pre_style_gap_is_tightened_v52() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface [core words] trail words words WRAPRIGHTMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHT").expect("right medium prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTMED")
        .expect("right medium wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHT")
        .expect("right medium rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core words").expect("right medium tm gap");

    assert!(
        rendered == "RIGHT preface core words",
        "wrapped right medium line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 118.0,
        "wrapped right medium pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right medium fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_pre_style_gap_is_tightened_v53() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface [core words] trail words words WRAPCENTERMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) = tm_position_for_segment_substring_v0(&pdf, "CENTER")
        .expect("centered medium prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERMED")
        .expect("centered medium wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered medium rendered text");
    let max_tm_gap =
        max_tm_gap_pt_for_line_containing_v0(&pdf, "core words").expect("centered medium tm gap");

    assert!(
        rendered == "CENTER preface core words",
        "wrapped centered medium line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 118.0,
        "wrapped centered medium pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered medium fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_bold_pre_style_gap_is_tightened_v54() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface {core words} trail words words WRAPRIGHTMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHT").expect("right medium bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTMED")
        .expect("right medium bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHT")
        .expect("right medium bold rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium bold tm gap");

    assert!(
        rendered == "RIGHT preface core words",
        "wrapped right medium bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 116.0,
        "wrapped right medium bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right medium bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_pre_style_gap_is_tightened_v55() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER [core words] trail words words WRAPCENTERSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTER").expect("centered short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERSHORT")
        .expect("centered short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short tm gap");

    assert!(
        rendered == "CENTER core words trail",
        "wrapped centered short line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 108.0,
        "wrapped centered short pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered short fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_bold_pre_style_gap_is_tightened_v56() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT {core words} trail words words WRAPRIGHTSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHT").expect("right short bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTSHORT")
        .expect("right short bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHT")
        .expect("right short bold rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short bold tm gap");

    assert!(
        rendered == "RIGHT core words trail",
        "wrapped right short bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 106.0,
        "wrapped right short bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right short bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_bold_pre_style_gap_is_tightened_v57() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER {core words} trail words words WRAPCENTERSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTER").expect("centered short bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERSHORT")
        .expect("centered short bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered short bold rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short bold tm gap");

    assert!(
        rendered == "CENTER core words trail",
        "wrapped centered short bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped centered short bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered short bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_italic_pre_style_gap_is_tightened_v58() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER [core words] trail words words WRAPCENTERSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) = tm_position_for_segment_substring_v0(&pdf, "CENTER")
        .expect("centered short italic prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERSHORTITALIC")
        .expect("centered short italic wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered short italic rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short italic tm gap");

    assert!(
        rendered == "CENTER core words trail",
        "wrapped centered short italic line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 106.0,
        "wrapped centered short italic pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered short italic fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_italic_pre_style_gap_is_tightened_v59() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT [core words] trail words words WRAPRIGHTSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHT").expect("right short italic prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTSHORTITALIC")
        .expect("right short italic wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHT")
        .expect("right short italic rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short italic tm gap");

    assert!(
        rendered == "RIGHT core words trail",
        "wrapped right short italic line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 105.0,
        "wrapped right short italic pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right short italic fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_italic_pre_style_gap_is_tightened_v60() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO [core words] trail words words words WRAPCENTERVSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("centered very-short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERVSHORT")
        .expect("centered very-short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("centered very-short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered very-short tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped centered very-short italic line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped centered very-short italic pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered very-short italic fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_bold_pre_style_gap_is_tightened_v61() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO {core words} trail words words words WRAPRIGHTVSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("right very-short bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTVSHORTB")
        .expect("right very-short bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("right very-short bold rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right very-short bold tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped right very-short bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 103.0,
        "wrapped right very-short bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right very-short bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_bold_pre_style_gap_is_tightened_v62() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO {core words} trail words words words WRAPCENTERVSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("centered very-short bold prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERVSHORTB")
        .expect("centered very-short bold wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("centered very-short bold rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered very-short bold tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped centered very-short bold line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped centered very-short bold pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered very-short bold fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_pre_style_gap_is_tightened_v63() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO core words trail words words words WRAPRIGHTVSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("right very-short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTVSHORT")
        .expect("right very-short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("right very-short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right very-short tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped right very-short line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped right very-short pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right very-short fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_pre_style_gap_is_tightened_v64() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO core words trail words words words WRAPCENTERVSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("centered very-short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERVSHORT")
        .expect("centered very-short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("centered very-short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered very-short tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped centered very-short line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped centered very-short pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered very-short fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_pre_style_gap_is_tightened_v65() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT core words trail words words WRAPRIGHTSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHT").expect("right short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTSHORTPLAIN")
        .expect("right short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHT")
        .expect("right short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short tm gap");

    assert!(
        rendered == "RIGHT core words trail",
        "wrapped right short line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 106.0,
        "wrapped right short pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right short fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_pre_style_gap_is_tightened_v66() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTER").expect("centered short prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERSHORTPLAIN")
        .expect("centered short wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTER")
        .expect("centered short rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short tm gap");

    assert!(
        rendered == "CENTER core words trail",
        "wrapped centered short line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 106.0,
        "wrapped centered short pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "centered short fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_italic_pre_style_gap_is_tightened_v67() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO [core words] trail words words words WRAPRIGHTVSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "GO").expect("right very-short italic prefix y");
    let (_, wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTVSHORTITALIC")
        .expect("right very-short italic wrap y");
    let rendered = rendered_text_for_line_containing_needle_v0(&pdf, "GO")
        .expect("right very-short italic rendered text");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right very-short italic tm gap");

    assert!(
        rendered == "GO core words trail words",
        "wrapped right very-short italic line should preserve stable spacing: {rendered}"
    );
    assert!(
        max_tm_gap <= 104.0,
        "wrapped right very-short italic pre-style seam should stay tightened: tm_gap={max_tm_gap}"
    );
    assert!(
        prefix_y > wrap_y,
        "right very-short italic fixture should still wrap after the tightened seam: prefix_y={prefix_y}, wrap_y={wrap_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_italic_low_tier_gap_is_tightened_v68() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO [core words] trail words words words WRAPRIGHTVSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right italic low-tier tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped right very-short italic low-tier seam should stay slightly tighter after v68: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_italic_low_tier_gap_is_tightened_v69() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO [core words] trail words words words WRAPCENTERVSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered italic low-tier tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped centered very-short italic low-tier seam should stay slightly tighter after v69: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_italic_low_tier_gap_is_tightened_v93() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO [core words] trail words words words WRAPCENTERVSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered italic tighter low-tier tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped centered very-short italic seam should stay slightly tighter after v93: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_bold_low_tier_gap_is_tightened_v70() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO {core words} trail words words words WRAPCENTERVSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered bold low-tier tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped centered very-short bold low-tier seam should stay slightly tighter after v70: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_bold_low_tier_gap_is_tightened_v94() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO {core words} trail words words words WRAPCENTERVSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered bold tighter low-tier tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped centered very-short bold seam should stay slightly tighter after v94: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_bold_low_tier_gap_is_tightened_v71() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO {core words} trail words words words WRAPRIGHTVSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right bold low-tier tm gap");
    assert!(
        actual_gap <= 102.5,
        "wrapped right very-short bold low-tier seam should stay slightly tighter after v71: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_italic_low_tier_gap_is_tightened_v72() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT [core words] trail words words WRAPRIGHTSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short italic low-tier tm gap");
    assert!(
        actual_gap <= 104.5,
        "wrapped right short italic low-tier seam should stay slightly tighter after v72: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_italic_low_tier_gap_is_tightened_v97() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT [core words] trail words words WRAPRIGHTSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short italic tighter low-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped right short italic seam should stay slightly tighter after v97: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_italic_low_tier_gap_is_tightened_v73() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER [core words] trail words words WRAPCENTERSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short italic low-tier tm gap");
    assert!(
        actual_gap <= 104.5,
        "wrapped centered short italic low-tier seam should stay slightly tighter after v73: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_italic_low_tier_gap_is_tightened_v96() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER [core words] trail words words WRAPCENTERSHORTITALIC tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short italic text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short italic tighter low-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped centered short italic seam should stay slightly tighter after v96: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_bold_low_tier_gap_is_tightened_v74() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT {core words} trail words words WRAPRIGHTSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short bold low-tier tm gap");
    assert!(
        actual_gap <= 105.5,
        "wrapped right short bold low-tier seam should stay slightly tighter after v74: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_bold_low_tier_gap_is_tightened_v98() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT {core words} trail words words WRAPRIGHTSHORTB tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short bold tighter low-tier tm gap");
    assert!(
        actual_gap <= 105.0,
        "wrapped right short bold seam should stay slightly tighter after v98: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_bold_low_tier_gap_is_tightened_v75() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER {core words} trail words words WRAPCENTERSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short bold low-tier tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped centered short bold low-tier seam should stay slightly tighter after v75: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_bold_low_tier_gap_is_tightened_v95() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER {core words} trail words words WRAPCENTERSHORT tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short bold text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short bold tighter low-tier tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped centered short bold seam should stay slightly tighter after v95: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_plain_low_tier_gap_is_tightened_v76() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short plain low-tier tm gap");
    assert!(
        actual_gap <= 105.0,
        "wrapped centered short plain low-tier seam should stay slightly tighter after v76: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_plain_low_tier_gap_is_tightened_v99() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short plain tighter low-tier tm gap");
    assert!(
        actual_gap <= 104.5,
        "wrapped centered short plain seam should stay slightly tighter after v99: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_plain_low_tier_gap_is_tightened_v77() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT core words trail words words WRAPRIGHTSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short plain low-tier tm gap");
    assert!(
        actual_gap <= 105.0,
        "wrapped right short plain low-tier seam should stay slightly tighter after v77: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_plain_low_tier_gap_is_tightened_v100() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT core words trail words words WRAPRIGHTSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short plain tighter low-tier tm gap");
    assert!(
        actual_gap <= 104.5,
        "wrapped right short plain seam should stay slightly tighter after v100: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_plain_low_tier_gap_is_tightened_v78() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO core words trail words words words WRAPCENTERVSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered very-short plain low-tier tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped centered very-short plain low-tier seam should stay slightly tighter after v78: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_very_short_plain_low_tier_gap_is_tightened_v101() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ GO core words trail words words words WRAPCENTERVSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered very-short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered very-short plain tighter low-tier tm gap");
    assert!(
        actual_gap <= 102.5,
        "wrapped centered very-short plain seam should stay slightly tighter after v101: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_plain_low_tier_gap_is_tightened_v79() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO core words trail words words words WRAPRIGHTVSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right very-short plain low-tier tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped right very-short plain low-tier seam should stay slightly tighter after v79: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_very_short_plain_low_tier_gap_is_tightened_v102() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| GO core words trail words words words WRAPRIGHTVSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right very-short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right very-short plain tighter low-tier tm gap");
    assert!(
        actual_gap <= 102.5,
        "wrapped right very-short plain seam should stay slightly tighter after v102: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_short_plain_medium_tier_gap_is_tightened_v80() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT core words trail words words WRAPRIGHTSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right short plain medium-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped right short plain medium-tier seam should stay slightly tighter after v80: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_plain_medium_tier_gap_is_tightened_v81() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short plain medium-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped centered short plain medium-tier seam should stay slightly tighter after v81: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_short_plain_medium_tier_gap_is_tightened_v103() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered short plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered short plain tighter medium-tier tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped centered short plain seam should stay slightly tighter after v103: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v82() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain medium-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped right medium plain medium-tier seam should stay slightly tighter after v82: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v83() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain medium-tier tm gap");
    assert!(
        actual_gap <= 104.0,
        "wrapped centered medium plain medium-tier seam should stay slightly tighter after v83: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v84() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped centered medium plain seam should stay slightly tighter after v84: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v85() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 103.5,
        "wrapped right medium plain seam should stay slightly tighter after v85: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v86() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped centered medium plain seam should stay slightly tighter after v86: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v105() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 102.5,
        "wrapped centered medium plain seam should stay slightly tighter after v105: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v107() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 102.0,
        "wrapped centered medium plain seam should stay slightly tighter after v107: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v109() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 101.5,
        "wrapped centered medium plain seam should stay slightly tighter after v109: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v111() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 101.0,
        "wrapped centered medium plain seam should stay slightly tighter after v111: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v113() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 100.5,
        "wrapped centered medium plain seam should stay slightly tighter after v113: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v115() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 100.0,
        "wrapped centered medium plain seam should stay slightly tighter after v115: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_centered_medium_plain_medium_tier_gap_is_tightened_v116() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTER preface core words trail words words WRAPCENTERMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped centered medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("centered medium plain tighter tm gap");
    assert!(
        actual_gap <= 99.5,
        "wrapped centered medium plain seam should stay slightly tighter after v116: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v87() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 103.0,
        "wrapped right medium plain seam should stay slightly tighter after v87: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v104() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 102.5,
        "wrapped right medium plain seam should stay slightly tighter after v104: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v106() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 102.0,
        "wrapped right medium plain seam should stay slightly tighter after v106: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v108() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 101.5,
        "wrapped right medium plain seam should stay slightly tighter after v108: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v110() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 101.0,
        "wrapped right medium plain seam should stay slightly tighter after v110: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v112() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 100.5,
        "wrapped right medium plain seam should stay slightly tighter after v112: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v114() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 100.0,
        "wrapped right medium plain seam should stay slightly tighter after v114: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v117() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 99.5,
        "wrapped right medium plain seam should stay slightly tighter after v117: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v118() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 99.0,
        "wrapped right medium plain seam should stay slightly tighter after v118: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v119() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 98.5,
        "wrapped right medium plain seam should stay slightly tighter after v119: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v120() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 98.0,
        "wrapped right medium plain seam should stay slightly tighter after v120: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v121() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 97.5,
        "wrapped right medium plain seam should stay slightly tighter after v121: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v122() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 97.0,
        "wrapped right medium plain seam should stay slightly tighter after v122: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v123() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 96.5,
        "wrapped right medium plain seam should stay slightly tighter after v123: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v124() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 96.0,
        "wrapped right medium plain seam should stay slightly tighter after v124: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v125() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 95.5,
        "wrapped right medium plain seam should stay slightly tighter after v125: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v126() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 95.0,
        "wrapped right medium plain seam should stay slightly tighter after v126: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v127() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 94.5,
        "wrapped right medium plain seam should stay slightly tighter after v127: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v128() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 94.0,
        "wrapped right medium plain seam should stay slightly tighter after v128: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v129() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 93.5,
        "wrapped right medium plain seam should stay slightly tighter after v129: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v130() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 93.0,
        "wrapped right medium plain seam should stay slightly tighter after v130: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v131() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 92.5,
        "wrapped right medium plain seam should stay slightly tighter after v131: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v132() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 92.0,
        "wrapped right medium plain seam should stay slightly tighter after v132: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_right_medium_plain_medium_tier_gap_is_tightened_v133() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHT preface core words trail words words WRAPRIGHTMEDPLAIN tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped right medium plain text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
        .expect("right medium plain tighter tm gap");
    assert!(
        actual_gap <= 91.5,
        "wrapped right medium plain seam should stay slightly tighter after v133: actual_gap={actual_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_bundle_short_and_medium_gaps_are_tightened_v743() {
    let cases = [
        (
            b"\n^ CENTERSTART edge, [core] trail words words words words WRAPCENTER tail.".as_slice(),
            "core",
            13.40f32,
            "centered short plain bundled tm gap",
        ),
        (
            b"\n| RIGHTSTART edge, [core] trail words words words words WRAPRIGHT tail.".as_slice(),
            "core",
            11.90f32,
            "right short plain bundled tm gap",
        ),
        (
            b"\n^ CENTER preface [core words] trail words words WRAPCENTERMED tail.".as_slice(),
            "core words",
            10.90f32,
            "centered medium plain bundled tm gap",
        ),
        (
            b"\n| RIGHT preface [core words] trail words words WRAPRIGHTMED tail.".as_slice(),
            "core words",
            9.50f32,
            "right medium plain bundled tm gap",
        ),
    ];

    for (input, needle, max_gap_pt, label) in cases {
        let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(input, 65_536, 786_432, 30)
            .expect("writer should accept bundled wrapped aligned plain text");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

        let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, needle)
            .expect("bundled wrapped aligned plain tm gap");
        assert!(
            actual_gap <= max_gap_pt,
            "{label} should stay tightened in the v743 bundle: actual_gap={actual_gap}, max_gap_pt={max_gap_pt}"
        );
    }
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_center_right_medium_continuity_stays_coherent_v743() {
    let center_xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ preface [core words] trail words words WRAPALIGNPLAINBUNDLE tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept centered continuity text");
    let right_xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| preface [core words] trail words words WRAPALIGNPLAINBUNDLE tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept right continuity text");
    let center_pdf = render_dvi_v2_text_page_to_pdf_v0(&center_xdv).expect("center pdf render");
    let right_pdf = render_dvi_v2_text_page_to_pdf_v0(&right_xdv).expect("right pdf render");

    let center_gap = max_tm_gap_pt_for_line_containing_v0(&center_pdf, "core words")
        .expect("center continuity tm gap");
    let right_gap = max_tm_gap_pt_for_line_containing_v0(&right_pdf, "core words")
        .expect("right continuity tm gap");
    assert!(
        center_gap <= 9.00 && right_gap <= 9.00,
        "bundled aligned plain medium continuity gaps should both stay tightened: center_gap={center_gap}, right_gap={right_gap}"
    );
    assert!(
        (center_gap - right_gap).abs() <= 0.02,
        "bundled aligned plain medium continuity should stay coherent across centered/right profiles: center_gap={center_gap}, right_gap={right_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_acceptance_surface_stays_coherent_v743() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART edge, [CSHORTCORE] trail words words words words WRAPCENTERACCEPTSHORT tail.\n\n| RIGHTSTART edge, [RSHORTCORE] trail words words words words WRAPRIGHTACCEPTSHORT tail.\n\n^ CENTER preface [CMEDCORE] trail words words WRAPCENTERACCEPTMED tail.\n\n| RIGHT preface [RMEDCORE] trail words words WRAPRIGHTACCEPTMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept grouped wrapped aligned plain surface");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let short_center_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CSHORTCORE")
        .expect("center short acceptance tm gap");
    let short_right_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "RSHORTCORE")
        .expect("right short acceptance tm gap");
    let medium_center_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CMEDCORE")
        .expect("center medium acceptance tm gap");
    let medium_right_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "RMEDCORE")
        .expect("right medium acceptance tm gap");
    let (_, center_short_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTERSTART").expect("center short start");
    let (_, center_short_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERACCEPTSHORT")
        .expect("center short wrap");
    let (_, right_short_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHTSTART").expect("right short start");
    let (_, right_short_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTACCEPTSHORT")
        .expect("right short wrap");
    let (_, center_medium_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "CMEDCORE").expect("center medium start");
    let (_, center_medium_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERACCEPTMED")
        .expect("center medium wrap");
    let (_, right_medium_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "RMEDCORE").expect("right medium start");
    let (_, right_medium_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTACCEPTMED")
        .expect("right medium wrap");

    let epsilon_pt = 0.2f32;
    assert!(
        short_center_gap <= 9.00
            && short_right_gap <= 9.00
            && medium_center_gap <= 10.90
            && medium_right_gap <= 9.50,
        "grouped acceptance surface should keep the bundled centered/right seams bounded: short_center_gap={short_center_gap}, short_right_gap={short_right_gap}, medium_center_gap={medium_center_gap}, medium_right_gap={medium_right_gap}"
    );
    assert!(
        center_short_start_y > center_short_wrap_y
            && center_short_wrap_y > right_short_start_y
            && right_short_start_y > right_short_wrap_y
            && right_short_wrap_y > center_medium_start_y
            && center_medium_start_y > center_medium_wrap_y
            && center_medium_wrap_y > right_medium_start_y
            && right_medium_start_y > right_medium_wrap_y,
        "grouped acceptance surface should preserve centered/right wrapped block ordering: center_short_start_y={center_short_start_y}, center_short_wrap_y={center_short_wrap_y}, right_short_start_y={right_short_start_y}, right_short_wrap_y={right_short_wrap_y}, center_medium_start_y={center_medium_start_y}, center_medium_wrap_y={center_medium_wrap_y}, right_medium_start_y={right_medium_start_y}, right_medium_wrap_y={right_medium_wrap_y}"
    );
    assert!(
        ((center_short_wrap_y - right_short_start_y) - (right_short_wrap_y - center_medium_start_y)).abs()
            <= epsilon_pt
            && ((right_short_wrap_y - center_medium_start_y) - (center_medium_wrap_y - right_medium_start_y)).abs()
                <= epsilon_pt,
        "grouped acceptance surface should keep adjacent centered/right block seams coherent: center_short_wrap_y={center_short_wrap_y}, right_short_start_y={right_short_start_y}, right_short_wrap_y={right_short_wrap_y}, center_medium_start_y={center_medium_start_y}, center_medium_wrap_y={center_medium_wrap_y}, right_medium_start_y={right_medium_start_y}"
    );
    assert!(
        (medium_center_gap - medium_right_gap).abs() <= 1.6,
        "grouped acceptance surface should keep medium centered/right seams in the same closure band: medium_center_gap={medium_center_gap}, medium_right_gap={medium_right_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_quote_and_list_styled_seams_use_v29_profile() {
    let xdv = write_dvi_v2_text_page_v0(
        b"\n- LISTSTART alpha alpha alpha alpha alpha alpha alpha [LISTITALICV29] beta beta beta beta beta beta LISTWRAPV29.\n\n> QUOTESTART gamma gamma gamma gamma gamma gamma gamma {QUOTEBOLDV29} delta delta delta delta delta QUOTEWRAPV29.",
    )
    .expect("writer should accept wrapped quote/list text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let list_line = pdf_text
        .lines()
        .find(|line| line.contains("(LISTITALICV29) Tj"))
        .expect("wrapped list styled line should render");
    let quote_line = pdf_text
        .lines()
        .find(|line| line.contains("(QUOTEBOLDV29) Tj"))
        .expect("wrapped quote styled line should render");

    assert!(
        list_line.contains("97 Tz") && list_line.contains("(LISTITALICV29) Tj 100 Tz"),
        "wrapped list styled segment should use v29 seam compensation"
    );
    assert!(
        quote_line.contains("95 Tz") && quote_line.contains("(QUOTEBOLDV29) Tj 100 Tz"),
        "wrapped quote styled segment should use v29 seam compensation"
    );
    let (_, list_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTSTART").expect("wrapped list start");
    let (_, list_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "LISTWRAPV29").expect("wrapped list wrap");
    let (_, quote_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTESTART").expect("wrapped quote start");
    let (_, quote_wrap_y) =
        tm_position_for_segment_substring_v0(&pdf, "QUOTEWRAPV29").expect("wrapped quote wrap");
    assert!(
        list_start_y > list_wrap_y,
        "list fixture should wrap onto a later line: list_start_y={list_start_y}, list_wrap_y={list_wrap_y}"
    );
    assert!(
        quote_start_y > quote_wrap_y,
        "quote fixture should wrap onto a later line: quote_start_y={quote_start_y}, quote_wrap_y={quote_wrap_y}"
    );
}
