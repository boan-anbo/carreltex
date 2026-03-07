use super::super::*;

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

#[test]
fn pdf_renderer_caps_segment_tm_gap_for_styled_line_v0() {
    let xdv = write_dvi_v2_text_page_v0(b"Styled [emphasis] and {bold} run.")
        .expect("writer should accept styled text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let max_tm_gap = max_tm_gap_pt_for_line_containing_v0(&pdf, "Styled")
        .expect("pdf should include styled line");
    assert!(
        max_tm_gap <= 12.0,
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

    let epsilon_pt = 0.01f32;
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
fn pdf_renderer_body_wrap_balances_lines_and_preserves_styled_punctuation_seams_v12() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"P1START aa [mid],bb cc dd P1WRAP.",
        65_536,
        786_432,
        24,
    )
    .expect("writer should accept wrapped styled paragraph");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let (start_x, start_y) =
        tm_position_for_segment_substring_v0(&pdf, "P1START").expect("paragraph start");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "P1WRAP").expect("paragraph wrap line");
    let epsilon_pt = 0.05f32;
    assert!(
        (start_x - 72.0).abs() <= epsilon_pt && (wrap_x - 72.0).abs() <= epsilon_pt,
        "body paragraph wrapped continuation should stay in body column: start_x={start_x}, wrap_x={wrap_x}"
    );
    assert!(
        (start_y - wrap_y - 13.0).abs() <= epsilon_pt,
        "wrapped body continuation rhythm should be tightened and stable: start_y={start_y}, wrap_y={wrap_y}"
    );

    let rendered_pdf_text = String::from_utf8_lossy(&pdf);
    assert!(
        !rendered_pdf_text.contains("mid ,") && !rendered_pdf_text.contains(", gamma"),
        "styled punctuation seams in wrapped body paragraphs should avoid spacing artifacts"
    );
}

#[test]
fn pdf_renderer_body_paragraph_applies_style_scaling_for_styled_seams_v13() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose with [ITALICV13] seam and {BOLDV13} seam.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept body prose");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICV13) Tj"))
        .expect("italic body segment should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDV13) Tj"))
        .expect("bold body segment should render");

    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICV13) Tj 100 Tz"),
        "body prose italic segment should use v13 seam-scaling compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDV13) Tj 100 Tz"),
        "body prose bold segment should use v13 seam-scaling compensation"
    );
}

#[test]
fn pdf_renderer_body_pre_style_gap_is_tightened_v35() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose with [ITALICPREV35] tail and {BOLDPREV35} end.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept body prose v35 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let italic_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(ITALICPREV35)",
        "(Body prose with )",
    )
    .expect("body italic prefix x");
    let italic_x = tm_x_for_segment_substring_v0(&pdf, "(ITALICPREV35)", "(ITALICPREV35)")
        .expect("body italic x");
    let bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(BOLDPREV35)",
        "( tail and )",
    )
    .expect("body bold prefix x");
    let bold_x = tm_x_for_segment_substring_v0(&pdf, "(BOLDPREV35)", "(BOLDPREV35)")
        .expect("body bold x");

    let expected_italic_gap = segment_width_pt_v0(b"Body prose with ") - (12.0 * 0.12);
    let expected_bold_gap = segment_width_pt_v0(b" tail and ") - (12.0 * 0.15);
    let epsilon_pt = 0.75f32;
    assert!(
        ((italic_x - italic_prefix_x) - expected_italic_gap).abs() <= epsilon_pt,
        "body prose pre-italic seam should trim the preceding space-bounded gap: prefix_x={italic_prefix_x}, italic_x={italic_x}, expected_gap={expected_italic_gap}"
    );
    assert!(
        ((bold_x - bold_prefix_x) - expected_bold_gap).abs() <= epsilon_pt,
        "body prose pre-bold seam should trim the preceding space-bounded gap: prefix_x={bold_prefix_x}, bold_x={bold_x}, expected_gap={expected_bold_gap}"
    );
}

#[test]
fn pdf_renderer_centered_lines_do_not_use_prose_seam_scaling_v13() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\n^ Center [ITALICCENTERV13] text.\n\n> Quote {BOLDQUOTEV13} line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept non-paragraph blocks");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let centered_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICCENTERV13) Tj"))
        .expect("centered styled segment should render");

    assert!(
        !centered_line.contains("97 Tz"),
        "centered non-paragraph line should not use prose seam-scaling compensation"
    );
}

#[test]
fn pdf_renderer_body_paragraph_uses_inline_math_seam_scaling_profile_v15() {
    let demo_text =
        b"Title\nAuthor\n2026-03-05\n\nBody [ITALICMATHV15] MATH seam and {BOLDMATHV15} MATH seam.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept inline math seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICMATHV15) Tj"))
        .expect("italic inline-math-adjacent segment should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDMATHV15) Tj"))
        .expect("bold inline-math-adjacent segment should render");
    assert!(
        italic_line.contains("99 Tz"),
        "inline-math-adjacent italic segment should use v15 seam scaling"
    );
    assert!(
        bold_line.contains("97 Tz"),
        "inline-math-adjacent bold segment should use v15 seam scaling"
    );
}

#[test]
fn pdf_renderer_wrap_avoids_inline_math_placeholder_at_line_start_v15() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"PSTART alpha beta gamma MATH, delta epsilon zeta eta WRAPTOKEN",
        65_536,
        786_432,
        30,
    )
    .expect("writer should accept wrapped inline-math paragraph");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let rendered_math_line =
        rendered_text_for_line_containing_needle_v0(&pdf, "MATH").expect("inline math placeholder should render");
    let (start_x, start_y) =
        tm_position_for_segment_substring_v0(&pdf, "PSTART").expect("paragraph start");
    let (wrap_x, wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "WRAPTOKEN").expect("wrap line");
    let epsilon_pt = 0.05f32;
    assert!(
        (start_x - 72.0).abs() <= epsilon_pt && (wrap_x - 72.0).abs() <= epsilon_pt,
        "wrapped body paragraph columns should stay stable: start_x={start_x}, wrap_x={wrap_x}"
    );
    assert!(
        start_y > wrap_y,
        "wrapped inline-math paragraph line should render below paragraph start: start_y={start_y}, wrap_y={wrap_y}"
    );
    let line_steps = ((start_y - wrap_y) / 13.0).round();
    assert!(
        line_steps >= 1.0 && (start_y - wrap_y - (line_steps * 13.0)).abs() <= epsilon_pt,
        "wrapped inline-math paragraph rhythm should stay on stable 13pt steps: start_y={start_y}, wrap_y={wrap_y}, line_steps={line_steps}"
    );
    assert!(
        rendered_math_line.contains("MATH,") && !rendered_math_line.contains("MATH ,"),
        "inline math placeholder should keep punctuation-adjacent seam spacing stable under wrapping: rendered_math_line={rendered_math_line:?}"
    );
}

#[test]
fn pdf_renderer_footnote_styled_seams_track_scaled_advances_v26() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through <{VISIBLELINKV26}>,right beside punctuation.^1\n\n!f 1 Footnote text with [INLINEFOOTNOTEV26].\n!u 1 https://example.com/v26";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept v26 seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_line = String::from_utf8_lossy(&pdf)
        .lines()
        .find(|line| line.contains("(INLINEFOOTNOTEV26) Tj"))
        .expect("footnote line should render")
        .to_string();
    assert!(
        footnote_line.contains("97 Tz") && footnote_line.contains("(INLINEFOOTNOTEV26) Tj 100 Tz"),
        "footnote styled segment should use v26 seam compensation"
    );

    let footnote_italic_x = tm_x_for_segment_substring_v0(&pdf, "(1 Footnote text with ", "(INLINEFOOTNOTEV26)")
        .expect("footnote italic x");
    let footnote_period_x =
        tm_x_for_segment_substring_v0(&pdf, "(1 Footnote text with ", "(.)").expect("footnote period x");
    let expected_footnote_italic_width = segment_width_pt_v0(b"INLINEFOOTNOTEV26") * 0.97;
    assert!(
        ((footnote_period_x - footnote_italic_x) - expected_footnote_italic_width).abs() <= 0.3,
        "footnote styled seam should advance on compensated rendered width: italic_x={footnote_italic_x}, period_x={footnote_period_x}, expected={expected_footnote_italic_width}"
    );
}

#[test]
fn pdf_renderer_footnote_pre_style_gap_is_tightened_v33() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through punctuation.^1\n\n!f 1 Footnote text with [INLINEFOOTNOTEV33].";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept v33 footnote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(INLINEFOOTNOTEV33)",
        "(1 Footnote text with )",
    )
    .expect("footnote prefix x");
    let footnote_italic_x =
        tm_x_for_segment_substring_v0(&pdf, "(INLINEFOOTNOTEV33)", "(INLINEFOOTNOTEV33)")
            .expect("footnote italic x");
    let expected_gap = segment_width_pt_v0(b"1 Footnote text with ") - (10.0 * 0.12);
    let epsilon_pt = 0.35f32;
    assert!(
        ((footnote_italic_x - footnote_prefix_x) - expected_gap).abs() <= epsilon_pt,
        "footnote pre-style seam should trim the preceding space-bounded gap: prefix_x={footnote_prefix_x}, italic_x={footnote_italic_x}, expected_gap={expected_gap}"
    );
}

#[test]
fn pdf_renderer_live_footnote_long_prefix_gaps_are_tightened_v40() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through punctuation.^1^2\n\n!f 1 First demo footnote text with [inline emphasis].\n!f 2 Second demo footnote text with {bold emphasis}.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept live footnote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_inline_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(inline emphasis)",
        "(1 First demo footnote text with )",
    )
    .expect("inline footnote prefix x");
    let footnote_inline_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline emphasis)", "(inline emphasis)")
            .expect("inline footnote style x");
    let footnote_bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(bold emphasis)",
        "(2 Second demo footnote text with )",
    )
    .expect("bold footnote prefix x");
    let footnote_bold_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold emphasis)", "(bold emphasis)")
            .expect("bold footnote style x");

    assert!(
        footnote_inline_x - footnote_inline_prefix_x <= 195.0,
        "live inline footnote long-prefix seam should stay tightened: prefix_x={footnote_inline_prefix_x}, style_x={footnote_inline_x}"
    );
    assert!(
        footnote_bold_x - footnote_bold_prefix_x <= 205.0,
        "live bold footnote long-prefix seam should stay tightened: prefix_x={footnote_bold_prefix_x}, style_x={footnote_bold_x}"
    );
}

#[test]
fn pdf_renderer_live_footnote_medium_prefix_gaps_are_tightened_v43() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through punctuation.^1^2\n\n!f 1 Demo footnote prefix with [inline words].\n!f 2 Second footnote prefix with {bold words}.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept medium footnote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_inline_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(inline words)",
        "(1 Demo footnote prefix with )",
    )
    .expect("medium inline footnote prefix x");
    let footnote_inline_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium inline footnote style x");
    let footnote_bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(bold words)",
        "(2 Second footnote prefix with )",
    )
    .expect("medium bold footnote prefix x");
    let footnote_bold_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold footnote style x");

    assert!(
        footnote_inline_x - footnote_inline_prefix_x <= 138.0,
        "live inline footnote medium-prefix seam should stay tightened: prefix_x={footnote_inline_prefix_x}, style_x={footnote_inline_x}"
    );
    assert!(
        footnote_bold_x - footnote_bold_prefix_x <= 150.0,
        "live bold footnote medium-prefix seam should stay tightened: prefix_x={footnote_bold_prefix_x}, style_x={footnote_bold_x}"
    );
}

#[test]
fn pdf_renderer_live_footnote_medium_inline_prefix_gap_is_tightened_v88() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through punctuation.^1^2\n\n!f 1 Demo footnote prefix with [inline words].\n!f 2 Second footnote prefix with {bold words}.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept medium footnote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_inline_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(inline words)",
        "(1 Demo footnote prefix with )",
    )
    .expect("medium inline footnote prefix x");
    let footnote_inline_x =
        tm_x_for_segment_substring_v0(&pdf, "(inline words)", "(inline words)")
            .expect("medium inline footnote style x");

    assert!(
        footnote_inline_x - footnote_inline_prefix_x <= 136.0,
        "live inline footnote medium-prefix seam should stay slightly tighter after v88: prefix_x={footnote_inline_prefix_x}, style_x={footnote_inline_x}"
    );
}

#[test]
fn pdf_renderer_live_footnote_medium_bold_prefix_gap_is_tightened_v91() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nBody prose through punctuation.^1^2\n\n!f 1 Demo footnote prefix with [inline words].\n!f 2 Second footnote prefix with {bold words}.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept medium footnote seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let footnote_bold_prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(bold words)",
        "(2 Second footnote prefix with )",
    )
    .expect("medium bold footnote prefix x");
    let footnote_bold_x =
        tm_x_for_segment_substring_v0(&pdf, "(bold words)", "(bold words)")
            .expect("medium bold footnote style x");

    assert!(
        footnote_bold_x - footnote_bold_prefix_x <= 148.0,
        "live bold footnote medium-prefix seam should stay slightly tighter after v91: prefix_x={footnote_bold_prefix_x}, style_x={footnote_bold_x}"
    );
}

#[test]
fn pdf_renderer_wrapped_body_paragraph_styled_seams_track_scaled_advances_v27() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nWRAPSTART alpha alpha alpha alpha alpha alpha alpha alpha <{BODYLINKWRAPV27}> and [ITALICWRAPV27],right beside punctuation with {BOLDWRAPV27} seam before WRAPTOKENV27.\n\n!u 1 https://example.com/v27";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept wrapped v27 seam text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let pdf_text = String::from_utf8_lossy(&pdf);
    let link_line = pdf_text
        .lines()
        .find(|line| line.contains("(BODYLINKWRAPV27) Tj"))
        .expect("wrapped body link line should render");
    let italic_line = pdf_text
        .lines()
        .find(|line| line.contains("(ITALICWRAPV27) Tj"))
        .expect("wrapped body italic line should render");
    let bold_line = pdf_text
        .lines()
        .find(|line| line.contains("(BOLDWRAPV27) Tj"))
        .expect("wrapped body bold line should render");

    assert!(
        link_line.contains("95 Tz") && link_line.contains("(BODYLINKWRAPV27) Tj 100 Tz"),
        "wrapped body link segment should use v27 seam compensation"
    );
    assert!(
        italic_line.contains("97 Tz") && italic_line.contains("(ITALICWRAPV27) Tj 100 Tz"),
        "wrapped body italic segment should use v27 seam compensation"
    );
    assert!(
        bold_line.contains("95 Tz") && bold_line.contains("(BOLDWRAPV27) Tj 100 Tz"),
        "wrapped body bold segment should use v27 seam compensation"
    );
    let (_, wrap_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPSTART").expect("wrapped paragraph start");
    let (_, wrap_token_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTOKENV27").expect("wrapped paragraph tail");
    assert!(
        wrap_start_y > wrap_token_y,
        "fixture should wrap onto a later body line: wrap_start_y={wrap_start_y}, wrap_token_y={wrap_token_y}"
    );
}

#[test]
fn pdf_renderer_wrapped_body_pre_style_gap_is_tightened_v34() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"PRESTYLEV34 with [WRAPITALICV34] tail alpha alpha alpha alpha WRAPTOKENV34",
        65_536,
        786_432,
        42,
    )
    .expect("writer should accept wrapped body v34 text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let prefix_x = tm_x_for_segment_substring_v0(
        &pdf,
        "(WRAPITALICV34)",
        "(PRESTYLEV34 with )",
    )
    .expect("wrapped body prefix x");
    let (_, prefix_y) =
        tm_position_for_segment_substring_v0(&pdf, "PRESTYLEV34").expect("wrapped body prefix y");
    let italic_x =
        tm_x_for_segment_substring_v0(&pdf, "(WRAPITALICV34)", "(WRAPITALICV34)")
            .expect("wrapped body italic x");
    let (_, wrap_token_y) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTOKENV34").expect("wrap token position");

    let expected_gap = segment_width_pt_v0(b"PRESTYLEV34 with ") - (12.0 * 0.12);
    let epsilon_pt = 0.3f32;
    assert!(
        ((italic_x - prefix_x) - expected_gap).abs() <= epsilon_pt,
        "wrapped body pre-style seam should trim the preceding space-bounded gap: prefix_x={prefix_x}, italic_x={italic_x}, expected_gap={expected_gap}"
    );
    assert!(
        prefix_y > wrap_token_y,
        "fixture should still wrap after the tightened wrapped-body seam: prefix_y={prefix_y}, wrap_token_y={wrap_token_y}"
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
