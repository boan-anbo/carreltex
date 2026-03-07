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
fn pdf_renderer_wrapped_aligned_grouped_short_and_very_short_style_plain_surfaces_stay_bounded() {
    let cases = [
        (
            b"\n| GO [core words] trail words words words WRAPRIGHTVSHORTITALIC tail.".as_slice(),
            "GO",
            "WRAPRIGHTVSHORTITALIC",
            "GO core words trail words",
            103.5f32,
            "right very-short italic grouped seam",
        ),
        (
            b"\n^ GO [core words] trail words words words WRAPCENTERVSHORT tail.".as_slice(),
            "GO",
            "WRAPCENTERVSHORT",
            "GO core words trail words",
            103.0f32,
            "centered very-short italic grouped seam",
        ),
        (
            b"\n| GO {core words} trail words words words WRAPRIGHTVSHORTB tail.".as_slice(),
            "GO",
            "WRAPRIGHTVSHORTB",
            "GO core words trail words",
            102.5f32,
            "right very-short bold grouped seam",
        ),
        (
            b"\n^ GO {core words} trail words words words WRAPCENTERVSHORTB tail.".as_slice(),
            "GO",
            "WRAPCENTERVSHORTB",
            "GO core words trail words",
            103.0f32,
            "centered very-short bold grouped seam",
        ),
        (
            b"\n| GO core words trail words words words WRAPRIGHTVSHORTPLAIN tail.".as_slice(),
            "GO",
            "WRAPRIGHTVSHORTPLAIN",
            "GO core words trail words",
            102.5f32,
            "right very-short plain grouped seam",
        ),
        (
            b"\n^ GO core words trail words words words WRAPCENTERVSHORTPLAIN tail.".as_slice(),
            "GO",
            "WRAPCENTERVSHORTPLAIN",
            "GO core words trail words",
            102.5f32,
            "centered very-short plain grouped seam",
        ),
        (
            b"\n| RIGHT [core words] trail words words WRAPRIGHTSHORTITALIC tail.".as_slice(),
            "RIGHT",
            "WRAPRIGHTSHORTITALIC",
            "RIGHT core words trail",
            104.0f32,
            "right short italic grouped seam",
        ),
        (
            b"\n^ CENTER [core words] trail words words WRAPCENTERSHORTITALIC tail.".as_slice(),
            "CENTER",
            "WRAPCENTERSHORTITALIC",
            "CENTER core words trail",
            104.0f32,
            "centered short italic grouped seam",
        ),
        (
            b"\n| RIGHT {core words} trail words words WRAPRIGHTSHORTB tail.".as_slice(),
            "RIGHT",
            "WRAPRIGHTSHORTB",
            "RIGHT core words trail",
            105.0f32,
            "right short bold grouped seam",
        ),
        (
            b"\n^ CENTER {core words} trail words words WRAPCENTERSHORT tail.".as_slice(),
            "CENTER",
            "WRAPCENTERSHORT",
            "CENTER core words trail",
            103.0f32,
            "centered short bold grouped seam",
        ),
        (
            b"\n| RIGHT core words trail words words WRAPRIGHTSHORTPLAIN tail.".as_slice(),
            "RIGHT",
            "WRAPRIGHTSHORTPLAIN",
            "RIGHT core words trail",
            104.0f32,
            "right short plain grouped seam",
        ),
        (
            b"\n^ CENTER core words trail words words WRAPCENTERSHORTPLAIN tail.".as_slice(),
            "CENTER",
            "WRAPCENTERSHORTPLAIN",
            "CENTER core words trail",
            103.5f32,
            "centered short plain grouped seam",
        ),
    ];

    for (input, line_needle, wrap_needle, expected_rendered, max_gap_pt, label) in cases {
        let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(input, 65_536, 786_432, 30)
            .expect("writer should accept grouped wrapped aligned short surface text");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

        let rendered = rendered_text_for_line_containing_needle_v0(&pdf, line_needle)
            .expect("grouped short/very-short rendered text");
        let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "core words")
            .expect("grouped short/very-short tm gap");
        let (_, prefix_y) =
            tm_position_for_segment_substring_v0(&pdf, line_needle).expect("grouped prefix y");
        let (_, wrap_y) =
            tm_position_for_segment_substring_v0(&pdf, wrap_needle).expect("grouped wrap y");

        assert!(
            rendered == expected_rendered,
            "{label} should preserve stable spacing on the grouped visible surface: rendered={rendered:?}, expected={expected_rendered:?}"
        );
        assert!(
            actual_gap <= max_gap_pt,
            "{label} should stay within the grouped closure band: actual_gap={actual_gap}, max_gap_pt={max_gap_pt}"
        );
        assert!(
            prefix_y > wrap_y,
            "{label} should still wrap after the grouped seam tightening: prefix_y={prefix_y}, wrap_y={wrap_y}"
        );
    }
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_grouped_short_and_medium_gaps_stay_bounded() {
    let cases = [
        (
            b"\n^ CENTERSTART edge, [core], trail words words words words WRAPCENTER tail.".as_slice(),
            "CENTERSTART",
            13.40f32,
            "centered short plain punctuated grouped tm gap",
        ),
        (
            b"\n| RIGHTSTART edge, [core], trail words words words words WRAPRIGHT tail.".as_slice(),
            "RIGHTSTART",
            11.90f32,
            "right short plain punctuated grouped tm gap",
        ),
        (
            b"\n^ CENTERMEDPFX [core words], trail words words WRAPCENTERMED tail.".as_slice(),
            "CENTERMEDPFX",
            10.90f32,
            "centered medium plain punctuated grouped tm gap",
        ),
        (
            b"\n| RIGHTMEDPFX [core words], trail words words WRAPRIGHTMED tail.".as_slice(),
            "RIGHTMEDPFX",
            10.10f32,
            "right medium plain punctuated grouped tm gap",
        ),
    ];

    for (input, needle, max_gap_pt, label) in cases {
        let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(input, 65_536, 786_432, 30)
            .expect("writer should accept grouped wrapped aligned plain text");
        let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

        let actual_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, needle)
            .expect("grouped wrapped aligned plain tm gap");
        assert!(
            actual_gap <= max_gap_pt,
            "{label} should stay bounded in the grouped wrapped aligned plain surface: actual_gap={actual_gap}, max_gap_pt={max_gap_pt}"
        );
    }
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_grouped_center_right_medium_continuity_stays_coherent() {
    let center_xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERCONTPFX [core words], trail words words WRAPALIGNPLAINBUNDLE tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept centered continuity text");
    let right_xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n| RIGHTCONTPFX [core words], trail words words WRAPALIGNPLAINBUNDLE tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept right continuity text");
    let center_pdf = render_dvi_v2_text_page_to_pdf_v0(&center_xdv).expect("center pdf render");
    let right_pdf = render_dvi_v2_text_page_to_pdf_v0(&right_xdv).expect("right pdf render");

    let center_gap = max_tm_gap_pt_for_line_containing_v0(&center_pdf, "CENTERCONTPFX")
        .expect("center continuity tm gap");
    let right_gap = max_tm_gap_pt_for_line_containing_v0(&right_pdf, "RIGHTCONTPFX")
        .expect("right continuity tm gap");
    assert!(
        center_gap <= 10.90 && right_gap <= 10.10,
        "bundled aligned plain medium continuity gaps should both stay tightened: center_gap={center_gap}, right_gap={right_gap}"
    );
    assert!(
        (center_gap - right_gap).abs() <= 1.6,
        "bundled aligned plain medium continuity should stay coherent across centered/right profiles: center_gap={center_gap}, right_gap={right_gap}"
    );
}

#[test]
fn pdf_renderer_wrapped_aligned_plain_grouped_acceptance_surface_stays_coherent() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\n^ CENTERSTART edge, [CSHORTCORE], trail words words words words WRAPCENTERACCEPTSHORT tail.\n\n| RIGHTSTART edge, [RSHORTCORE], trail words words words words WRAPRIGHTACCEPTSHORT tail.\n\n^ CENTERMEDPFX [CMEDCORE], trail words words WRAPCENTERACCEPTMED tail.\n\n| RIGHTMEDPFX [RMEDCORE], trail words words WRAPRIGHTACCEPTMED tail.",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept grouped wrapped aligned plain surface");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let short_center_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CENTERSTART")
        .expect("center short acceptance tm gap");
    let short_right_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "RIGHTSTART")
        .expect("right short acceptance tm gap");
    let medium_center_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "CENTERMEDPFX")
        .expect("center medium acceptance tm gap");
    let medium_right_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "RIGHTMEDPFX")
        .expect("right medium acceptance tm gap");
    let center_short_rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERSTART")
        .expect("center short acceptance rendered text");
    let right_short_rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTSTART")
        .expect("right short acceptance rendered text");
    let center_medium_rendered = rendered_text_for_line_containing_needle_v0(&pdf, "CENTERMEDPFX")
        .expect("center medium acceptance rendered text");
    let right_medium_rendered = rendered_text_for_line_containing_needle_v0(&pdf, "RIGHTMEDPFX")
        .expect("right medium acceptance rendered text");
    let (_, center_short_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTERSTART").expect("center short start");
    let (_, center_short_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERACCEPTSHORT")
        .expect("center short wrap");
    let (_, right_short_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHTSTART").expect("right short start");
    let (_, right_short_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTACCEPTSHORT")
        .expect("right short wrap");
    let (_, center_medium_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "CENTERMEDPFX").expect("center medium start");
    let (_, center_medium_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPCENTERACCEPTMED")
        .expect("center medium wrap");
    let (_, right_medium_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "RIGHTMEDPFX").expect("right medium start");
    let (_, right_medium_wrap_y) = tm_position_for_segment_substring_v0(&pdf, "WRAPRIGHTACCEPTMED")
        .expect("right medium wrap");

    let epsilon_pt = 0.2f32;
    assert!(
        short_center_gap <= 5.10
            && short_right_gap <= 5.10
            && medium_center_gap <= 10.90
            && medium_right_gap <= 10.10,
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
    assert!(
        center_short_rendered == "CENTERSTART edge,"
            && right_short_rendered == "RIGHTSTART edge,"
            && center_medium_rendered == "CENTERMEDPFX CMEDCORE,"
            && right_medium_rendered == "RIGHTMEDPFX RMEDCORE,",
        "grouped acceptance surface should keep punctuation adjacent to wrapped styled seams across centered/right surfaces: center_short_rendered={center_short_rendered:?}, right_short_rendered={right_short_rendered:?}, center_medium_rendered={center_medium_rendered:?}, right_medium_rendered={right_medium_rendered:?}"
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
