use super::super::*;

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
        (heading_x - expected_heading_x).abs() <= 0.01,
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

    let epsilon_pt = 0.01f32;
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
fn pdf_renderer_title_centering_stays_stable_with_inline_style_segments_v1() {
    let demo_text =
        b"Center [Accurate] {Title}\nAlice Bob\n2026-03-05\n\nBody line after styled title.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept styled title demo");
    let layout = parse_dvi_v2_text_page_to_layout_v0(&xdv, 786_432).expect("layout parse");
    let title_width = layout_line_width_for_exact_bytes_v0(&layout, b"Center [Accurate] {Title}")
        .expect("styled title width");
    let expected_title_x = expected_center_x_pt_v0(title_width);

    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");
    let title_x = tm_x_for_line_containing_text_v0(&pdf, "Center")
        .expect("styled title first segment x");
    assert!(
        (title_x - expected_title_x).abs() <= 0.01,
        "styled title centering mismatch: actual={title_x}, expected={expected_title_x}"
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
        (section_size - 15.5).abs() <= 0.02,
        "section font size mismatch: {section_size}"
    );
    assert!(
        (subsection_size - 13.0).abs() <= 0.02,
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
        (first_y - same_para_y - 13.0).abs() <= epsilon_pt,
        "line gap mismatch inside paragraph: first_y={first_y}, same_para_y={same_para_y}"
    );
    assert!(
        (same_para_y - second_para_y - 27.0).abs() <= epsilon_pt,
        "paragraph break gap mismatch: same_para_y={same_para_y}, second_para_y={second_para_y}"
    );
    assert!(
        (second_para_y - heading_y - 24.0).abs() <= epsilon_pt,
        "paragraph->heading gap mismatch: second_para_y={second_para_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - after_heading_y - 24.0).abs() <= epsilon_pt,
        "heading->noindent gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - indented_y - 27.0).abs() <= epsilon_pt,
        "noindent->indented paragraph gap mismatch: after_heading_y={after_heading_y}, indented_y={indented_y}"
    );
    assert!(
        (heading_x - 72.0).abs() > 0.5,
        "heading should be centered: {heading_x}"
    );
}

#[test]
fn pdf_renderer_consecutive_blank_lines_collapse_to_single_rhythm_gap_v1() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\nFirst paragraph line.\n\n\nSecond paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept blank-line rhythm demo");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (first_x, first_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First paragraph line.)")
            .expect("first paragraph line");
    let (second_x, second_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Second paragraph line.)")
            .expect("second paragraph line");

    let epsilon_pt = 0.02f32;
    assert!(
        (first_x - 72.0).abs() <= epsilon_pt,
        "first paragraph should start at body margin: {first_x}"
    );
    assert!(
        (second_x - 96.0).abs() <= epsilon_pt,
        "second paragraph should keep paragraph indent: {second_x}"
    );
    assert!(
        (first_y - second_y - 27.0).abs() <= epsilon_pt,
        "consecutive blank lines should collapse to a single paragraph gap: first_y={first_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_list_rhythm_and_wrap_indent_invariants_v0() {
    let demo_text = b"\nParagraph before list.\n\n- ITEMONE lead words with deterministic wrapping content to force continuation line token WRAPONE after many repeated words in this same item.\n- ITEMTWO lead words with deterministic wrapping content to force continuation line token WRAPTWO after many repeated words in this same item.\n\nParagraph after list.";
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
    let (item_two_body_x, item_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "(ITEMTWO").expect("item two body position");
    let (item_two_wrap_x, _) =
        tm_position_for_segment_substring_v0(&pdf, "WRAPTWO").expect("item two wrap position");
    let (_, after_list_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Paragraph after list.)")
            .expect("after list paragraph");

    let epsilon_pt = 0.02f32;
    assert!(
        (before_list_y - item_one_y - 24.0).abs() <= epsilon_pt,
        "before->list top gap out of range: before_list_y={before_list_y}, item_one_y={item_one_y}"
    );
    assert!(
        (item_two_y - after_list_y).abs() >= 24.0 - epsilon_pt,
        "list->after paragraph gap must be at least one paragraph break: item_two_y={item_two_y}, after_list_y={after_list_y}"
    );
    assert!(
        (item_one_body_x - 96.0).abs() <= epsilon_pt,
        "item body x mismatch: {item_one_body_x}"
    );
    let item_one_marker_gap_pt = item_one_body_x - (item_one_bullet_x + segment_width_pt_v0(b"-"));
    assert!(
        (item_one_marker_gap_pt - 8.0).abs() <= 0.25,
        "item marker/body gap mismatch: marker_gap={item_one_marker_gap_pt}"
    );
    assert!(
        (item_one_wrap_x - item_one_body_x).abs() <= epsilon_pt,
        "item one wrap continuation should keep hanging indent: body={item_one_body_x}, wrap={item_one_wrap_x}"
    );
    assert!(
        (item_two_body_x - 96.0).abs() <= epsilon_pt,
        "item two body x mismatch: {item_two_body_x}"
    );
    assert!(
        (item_two_wrap_x - item_two_body_x).abs() <= epsilon_pt,
        "item two wrap continuation should keep hanging indent: body={item_two_body_x}, wrap={item_two_wrap_x}"
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
        (first_y - second_y - 27.0).abs() <= 0.02,
        "paragraph y-gap mismatch: first_y={first_y}, second_y={second_y}"
    );
}

#[test]
fn pdf_renderer_body_only_long_page_paragraph_block_rhythm_is_stable_v12() {
    let xdv = write_dvi_v2_text_page_with_layout_and_wrap_v0(
        b"\nP1START aa bb cc dd ee ff gg hh ii jj kk ll mm nn P1LAST.\n\nP2START aa bb cc dd ee P2WRAP ff gg hh ii jj kk ll mm nn.",
        65_536,
        786_432,
        20,
    )
    .expect("writer should accept wrapped body-only paragraphs");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, p1_last_y) =
        tm_position_for_segment_substring_v0(&pdf, "P1LAST").expect("first paragraph tail");
    let (p2_start_x, p2_start_y) =
        tm_position_for_segment_substring_v0(&pdf, "P2START").expect("second paragraph start");
    let (p2_wrap_x, p2_wrap_y) =
        tm_position_for_line_containing_text_v0(&pdf, "P2WRAP").expect("second paragraph wrap");
    let epsilon_pt = 0.05f32;
    assert!(
        (p1_last_y - p2_start_y - 27.0).abs() <= epsilon_pt,
        "paragraph-to-paragraph block gap on long body-only pages should stay stable: p1_last_y={p1_last_y}, p2_start_y={p2_start_y}"
    );
    assert!(
        (p2_start_x - 96.0).abs() <= epsilon_pt && (p2_wrap_x - 72.0).abs() <= epsilon_pt,
        "second paragraph start/wrap columns should remain stable: p2_start_x={p2_start_x}, p2_wrap_x={p2_wrap_x}"
    );
    assert!(
        (p2_start_y - p2_wrap_y - 13.0).abs() <= epsilon_pt,
        "wrapped continuation rhythm in long body-only pages should stay tightened: p2_start_y={p2_start_y}, p2_wrap_y={p2_wrap_y}"
    );
}

#[test]
fn pdf_renderer_front_matter_title_to_first_body_rhythm_is_tightened_v11() {
    let demo_text = b"Front Matter Title\nAuthor Name\n2026-03-05\n\nFirst body paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept demo text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) = tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)")
        .expect("date line position");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("first body line position");
    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - body_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->first-body rhythm should stay tightened and deterministic: date_y={date_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after front matter should remain unindented: body_x={body_x}"
    );
}

#[test]
fn pdf_renderer_tall_title_block_spacing_and_body_transition_polish_v23() {
    let demo_text = b"Front Matter Main Title\nFront Matter Subtitle\nAuthor One\nAuthor Two\n2026-03-05\n\nFirst body paragraph line.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept tall title block text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, title_y) = tm_position_for_line_containing_text_v0(&pdf, "(Front Matter Main Title)")
        .expect("title line");
    let (_, subtitle_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Front Matter Subtitle)")
            .expect("subtitle line");
    let (_, author_one_y) = tm_position_for_line_containing_text_v0(&pdf, "(Author One)")
        .expect("author one line");
    let (_, author_two_y) = tm_position_for_line_containing_text_v0(&pdf, "(Author Two)")
        .expect("author two line");
    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(First body paragraph line.)")
            .expect("body line");

    let epsilon_pt = 0.05f32;
    assert!(
        (title_y - subtitle_y - 13.0).abs() <= epsilon_pt
            && (subtitle_y - author_one_y - 13.0).abs() <= epsilon_pt
            && (author_one_y - author_two_y - 13.0).abs() <= epsilon_pt
            && (author_two_y - date_y - 13.0).abs() <= epsilon_pt,
        "tall title-block internal line spacing should stay compact and stable: title_y={title_y}, subtitle_y={subtitle_y}, author_one_y={author_one_y}, author_two_y={author_two_y}, date_y={date_y}"
    );
    assert!(
        (date_y - body_y - 33.0).abs() <= epsilon_pt,
        "tall title-block date->first-body transition should stay tightened: date_y={date_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after tall title block should remain unindented: body_x={body_x}"
    );
}

#[test]
fn pdf_renderer_tall_title_block_to_heading_transition_polish_v23() {
    let demo_text = b"Front Matter Main Title\nFront Matter Subtitle\nAuthor One\nAuthor Two\n2026-03-05\n\n@S {Heading After Tall Front Matter}\n\n~ Body after heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept tall title->heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date line");
    let (_, heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Heading After Tall Front Matter)")
            .expect("heading line");
    let (body_x, body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after heading.)").expect("body line");

    let epsilon_pt = 0.05f32;
    assert!(
        (date_y - heading_y - 33.0).abs() <= epsilon_pt,
        "tall title-block->heading opening gap mismatch: date_y={date_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= epsilon_pt,
        "heading->first-body gap after tall title block mismatch: heading_y={heading_y}, body_y={body_y}"
    );
    assert!(
        (body_x - 72.0).abs() <= 0.02,
        "first body line after heading should remain unindented: body_x={body_x}"
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
        (intro_y - heading_y - 24.0).abs() <= 0.02,
        "intro->heading y-gap mismatch: intro_y={intro_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= 0.02,
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
fn pdf_renderer_front_matter_heading_opening_rhythm_polish_v22() {
    let demo_text = b"Title\nAuthor\n2026-03-05\n\n@S {Front Heading}\n\n~ Body after front heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept front-matter heading text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, date_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(2026-03-05)").expect("date position");
    let (_, heading_y) = tm_position_for_line_containing_text_v0(&pdf, "(Front Heading)")
        .expect("heading position");
    let (_, body_y) = tm_position_for_line_containing_text_v0(&pdf, "(Body after front heading.)")
        .expect("body position");

    let epsilon_pt = 0.02f32;
    assert!(
        (date_y - heading_y - 38.0).abs() <= epsilon_pt,
        "front-matter->heading opening gap mismatch: date_y={date_y}, heading_y={heading_y}"
    );
    assert!(
        (heading_y - body_y - 24.0).abs() <= epsilon_pt,
        "heading->first-body gap mismatch: heading_y={heading_y}, body_y={body_y}"
    );
}

#[test]
fn pdf_renderer_heading_transitions_across_list_quote_table_polish_v22() {
    let demo_text = b"\nPrelude paragraph.\n\n- List line one.\n- List line two.\n\n@S {After List Heading}\n\n~ Body after list heading.\n\n> Quote line one.\n> Quote line two.\n\n@S {After Quote Heading}\n\n~ Body after quote heading.\n\n!ts ll\n!t TROWONEA||TROWONEB\n!t TROWTWOA||TROWTWOB\n\n@S {After Table Heading}\n\n~ Body after table heading.";
    let xdv = write_dvi_v2_text_page_v0(demo_text).expect("writer should accept mixed heading transition text");
    let pdf = render_dvi_v2_text_page_to_pdf_v0(&xdv).expect("pdf render");

    let (_, list_two_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(List line two.)").expect("list line two");
    let (_, after_list_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After List Heading)")
            .expect("after-list heading");
    let (_, after_list_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after list heading.)")
            .expect("after-list body");
    let (_, quote_two_y) = tm_position_for_line_containing_text_v0(&pdf, "(Quote line two.)")
        .expect("quote line two");
    let (_, after_quote_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After Quote Heading)")
            .expect("after-quote heading");
    let (_, after_quote_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after quote heading.)")
            .expect("after-quote body");
    let (_, table_row_two_y) =
        tm_position_for_segment_substring_v0(&pdf, "TROWTWOA").expect("table row two");
    let (_, after_table_heading_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(After Table Heading)")
            .expect("after-table heading");
    let (_, after_table_body_y) =
        tm_position_for_line_containing_text_v0(&pdf, "(Body after table heading.)")
            .expect("after-table body");

    let epsilon_pt = 0.2f32;
    assert!(
        (list_two_y - after_list_heading_y - 24.0).abs() <= epsilon_pt,
        "list->heading gap mismatch: list_two_y={list_two_y}, after_list_heading_y={after_list_heading_y}"
    );
    assert!(
        (after_list_heading_y - after_list_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after list: after_list_heading_y={after_list_heading_y}, after_list_body_y={after_list_body_y}"
    );
    assert!(
        (quote_two_y - after_quote_heading_y - 23.0).abs() <= epsilon_pt,
        "quote->heading gap mismatch: quote_two_y={quote_two_y}, after_quote_heading_y={after_quote_heading_y}"
    );
    assert!(
        (after_quote_heading_y - after_quote_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after quote: after_quote_heading_y={after_quote_heading_y}, after_quote_body_y={after_quote_body_y}"
    );
    assert!(
        (table_row_two_y - after_table_heading_y - 24.0).abs() <= epsilon_pt,
        "table->heading gap mismatch: table_row_two_y={table_row_two_y}, after_table_heading_y={after_table_heading_y}"
    );
    assert!(
        (after_table_heading_y - after_table_body_y - 24.0).abs() <= epsilon_pt,
        "heading->paragraph gap mismatch after table: after_table_heading_y={after_table_heading_y}, after_table_body_y={after_table_body_y}"
    );
}

#[test]
fn pdf_renderer_front_matter_list_and_table_opening_transitions_are_tightened_v25() {
    let list_pdf = render_dvi_v2_text_page_to_pdf_v0(
        &write_dvi_v2_text_page_v0(
            b"Front Matter Title\nAuthor Name\n2026-03-05\n\n- LISTOPEN first list item.\n- LISTNEXT second list item.",
        )
        .expect("writer should accept front-matter list text"),
    )
    .expect("list pdf render");
    let (_, list_date_y) =
        tm_position_for_line_containing_text_v0(&list_pdf, "(2026-03-05)").expect("list date");
    let (_, list_open_y) = tm_position_for_line_containing_text_v0(&list_pdf, "(LISTOPEN first list item.)")
        .expect("list opening line");
    let (_, list_next_y) = tm_position_for_line_containing_text_v0(&list_pdf, "(LISTNEXT second list item.)")
        .expect("list second line");

    let table_pdf = render_dvi_v2_text_page_to_pdf_v0(
        &write_dvi_v2_text_page_v0(
            b"Front Matter Title\nAuthor Name\n2026-03-05\n\n!ts ll\n!t TABOPENA||TABOPENB\n!t TABNEXTA||TABNEXTB",
        )
        .expect("writer should accept front-matter table text"),
    )
    .expect("table pdf render");
    let (_, table_date_y) =
        tm_position_for_line_containing_text_v0(&table_pdf, "(2026-03-05)").expect("table date");
    let (_, table_open_y) =
        tm_position_for_segment_substring_v0(&table_pdf, "TABOPENA").expect("table opening row");
    let (_, table_next_y) =
        tm_position_for_segment_substring_v0(&table_pdf, "TABNEXTA").expect("table second row");

    let epsilon_pt = 0.05f32;
    assert!(
        (list_date_y - list_open_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->list opening transition should stay tightened: list_date_y={list_date_y}, list_open_y={list_open_y}"
    );
    assert!(
        (list_open_y - list_next_y - 13.0).abs() <= epsilon_pt,
        "list internal rhythm should stay stable after front-matter opening: list_open_y={list_open_y}, list_next_y={list_next_y}"
    );
    assert!(
        (table_date_y - table_open_y - 38.0).abs() <= epsilon_pt,
        "front-matter date->table opening transition should stay tightened: table_date_y={table_date_y}, table_open_y={table_open_y}"
    );
    assert!(
        (table_open_y - table_next_y - 13.0).abs() <= epsilon_pt,
        "table row rhythm should stay stable after front-matter opening: table_open_y={table_open_y}, table_next_y={table_next_y}"
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
    let (list_one_x, list_one_y) =
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
    assert!((prelude_y - heading_y - 24.0).abs() <= epsilon_pt);
    assert!(
        (heading_y - after_heading_y - 24.0).abs() <= epsilon_pt,
        "heading->first paragraph gap mismatch: heading_y={heading_y}, after_heading_y={after_heading_y}"
    );
    assert!(
        (after_heading_y - list_one_y - 24.0).abs() <= epsilon_pt,
        "paragraph->list gap mismatch: after_heading_y={after_heading_y}, list_one_y={list_one_y}"
    );
    assert!(
        (list_one_y - list_two_y - 13.0).abs() <= epsilon_pt,
        "list line gap mismatch: list_one_y={list_one_y}, list_two_y={list_two_y}"
    );
    assert!(
        (list_two_y - quote_one_y - 23.0).abs() <= epsilon_pt,
        "list->quote gap mismatch: list_two_y={list_two_y}, quote_one_y={quote_one_y}"
    );
    assert!(
        (quote_one_y - quote_two_y - 12.5).abs() <= epsilon_pt,
        "quote line gap mismatch: quote_one_y={quote_one_y}, quote_two_y={quote_two_y}"
    );
    assert!(
        (quote_two_y - after_quote_y - 23.0).abs() <= epsilon_pt,
        "quote->paragraph gap mismatch: quote_two_y={quote_two_y}, after_quote_y={after_quote_y}"
    );
    assert!(quote_one_x > 72.0, "quote line should be indented");
    assert!(
        quote_one_x >= list_one_x + 6.0,
        "quote indent should be visibly deeper than list body indent: list_one_x={list_one_x}, quote_one_x={quote_one_x}"
    );
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

    let (item_x, item_y) = tm_position_for_segment_substring_v0(&pdf, "item").expect("item");
    let (continuation_x, continuation_y) =
        tm_position_for_segment_substring_v0(&pdf, "continuation").expect("continuation");
    let epsilon_pt = 0.02f32;
    assert!(
        (item_x - 96.0).abs() <= epsilon_pt,
        "item line body x mismatch: {item_x}"
    );
    assert!(
        (continuation_x - item_x).abs() <= epsilon_pt,
        "continuation should keep hanging indent: item_x={item_x}, continuation_x={continuation_x}"
    );
    assert!(
        (item_y - continuation_y - 13.0).abs() <= epsilon_pt,
        "list continuation line rhythm mismatch: item_y={item_y}, continuation_y={continuation_y}"
    );
}
