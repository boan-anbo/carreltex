use crate::{
    layout_v0::glyph_width_sp_v0, parse_dvi_v2_text_page_to_layout_v0, GlyphPlanV0, LinePlanV0,
    PagePlanV0, DEFAULT_LINE_ADVANCE_SP_V0,
};
use std::collections::BTreeMap;

const PDF_VERSION: &[u8] = b"%PDF-1.4\n";
const PDF_EOF: &[u8] = b"%%EOF\n";
const NEWLINE_MARKER_V0: u8 = 0x0a;
const PAGE_BREAK_MARKER_V0: u8 = 0x0c;

const PAGE_WIDTH_PT_V0: f32 = 612.0;
const PAGE_HEIGHT_PT_V0: f32 = 792.0;
const MARGIN_PT_V0: f32 = 72.0;
const FONT_SIZE_PT_V0: f32 = 12.0;
const TITLE_FONT_SIZE_PT_V0: f32 = 18.0;
const SECTION_HEADING_FONT_SIZE_PT_V0: f32 = 15.5;
const SUBSECTION_HEADING_FONT_SIZE_PT_V0: f32 = 13.0;
const INDENT_PT_V0: f32 = FONT_SIZE_PT_V0 * 2.0;
const LIST_BODY_INDENT_PT_V0: f32 = INDENT_PT_V0;
const ENUM_NUMBER_COLUMN_RIGHT_PT_V0: f32 = MARGIN_PT_V0 + (FONT_SIZE_PT_V0 * 1.33);
const QUOTE_BODY_INDENT_PT_V0: f32 = INDENT_PT_V0 + 6.0;
const QUOTE_PREFIX_GAP_PT_V0: f32 = FONT_SIZE_PT_V0 * 0.5;
const LEADING_PT_V0: f32 = 14.0;
const LIST_ENTRY_LEADING_PT_V7: f32 = 13.0;
const QUOTE_ENTRY_LEADING_PT_V7: f32 = 13.0;
const BLOCK_TRANSITION_GAP_PT_V7: f32 = 24.0;
const TITLE_EXTRA_GAP_PT_V0: f32 = LEADING_PT_V0;
const FOOTNOTE_FONT_SIZE_PT_V0: f32 = 10.0;
const FOOTNOTE_LEADING_PT_V0: f32 = 13.0;
const FOOTNOTE_BLOCK_GAP_PT_V0: f32 = 16.0;
const FOOTNOTE_LINE_PREFIX_MARKER_V0: &[u8] = b"!f ";
const HREF_URL_LINE_PREFIX_MARKER_V0: &[u8] = b"!u ";
const LABEL_LINE_PREFIX_MARKER_V0: &[u8] = b"!l ";
const REF_LINE_PREFIX_MARKER_V0: &[u8] = b"!r ";
const PAGEREF_LINE_PREFIX_MARKER_V0: &[u8] = b"!pr ";
const REF_ANCHOR_LINK_LINE_PREFIX_MARKER_V0: &[u8] = b"!ra ";
const PAGEREF_PAGE_LINK_LINE_PREFIX_MARKER_V0: &[u8] = b"!rp ";
const EQUATION_LINE_PREFIX_MARKER_V0: &[u8] = b"!eq ";
const BIBITEM_LINE_PREFIX_MARKER_V0: &[u8] = b"!b ";
const CITE_LINE_PREFIX_MARKER_V0: &[u8] = b"!c ";
const NOINDENT_PREFIX_MARKER_V0: u8 = b'~';
const LINK_START_MARKER_V0: u8 = b'<';
const LINK_END_MARKER_V0: u8 = b'>';
const FOOTNOTE_MARKER_PREFIX_V0: u8 = b'^';
const FOOTNOTE_MARKER_FONT_SIZE_PT_V0: f32 = 8.0;
const FOOTNOTE_MARKER_RISE_PT_V0: f32 = 4.0;
const TABLE_SPEC_PREFIX_MARKER_V0: &[u8] = b"!ts ";
const TABLE_ROW_PREFIX_MARKER_V0: &[u8] = b"!t ";
const FIGURE_BOX_PREFIX_MARKER_V0: &[u8] = b"!gbox";
const FIGURE_IMAGE_PREFIX_MARKER_V0: &[u8] = b"!gimg ";
const FIGURE_CAPTION_PREFIX_MARKER_V0: &[u8] = b"!gcap ";
const TOC_PLACEHOLDER_MARKER_V0: &[u8] = b"!toc";
const TOC_ENTRY_LINE_PREFIX_MARKER_V0: &[u8] = b"!toc ";
const TABLE_CELL_PADDING_PT_V0: f32 = 7.0;
const TABLE_ROW_LEADING_PT_V0: f32 = 14.0;
const TABLE_BORDER_LINE_WIDTH_PT_V0: f32 = 0.5;
const TABLE_BORDER_TOP_OFFSET_PT_V0: f32 = 5.0;
const TABLE_BORDER_BOTTOM_OFFSET_PT_V0: f32 = 5.0;
const ANNOTATION_RECT_DESCENT_RATIO_V9: f32 = 0.22;
const ANNOTATION_RECT_ASCENT_RATIO_V9: f32 = 0.78;
const ANNOTATION_RECT_MIN_HEIGHT_PT_V9: f32 = 8.0;
const FIGURE_PLACEHOLDER_LINE_V0: &[u8] = b"[ Figure placeholder ]";
const FIGURE_CAPTION_FONT_SIZE_PT_V0: f32 = 11.0;
const DEFAULT_FIGURE_PLACEHOLDER_WIDTH_PT_V0: f32 = 180.0;
const DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_PT_V0: f32 = 120.0;
const MAX_FIGURE_PLACEHOLDER_WIDTH_PT_V0: f32 = PAGE_WIDTH_PT_V0 - (2.0 * MARGIN_PT_V0);
const MAX_FIGURE_PLACEHOLDER_HEIGHT_PT_V0: f32 = 288.0;
const FIGURE_PLACEHOLDER_LABEL_INSET_PT_V0: f32 = 8.0;
const FIGURE_PLACEHOLDER_TO_CAPTION_GAP_PT_V0: f32 = 10.0;
const TOC_TITLE_TEXT_V0: &[u8] = b"Contents";
const TOC_TITLE_FONT_SIZE_PT_V0: f32 = 14.0;
const TOC_TITLE_TO_FIRST_ENTRY_GAP_PT_V5: f32 = 12.0;
const TOC_ENTRY_LEADING_PT_V5: f32 = 13.0;
const TOC_ENTRY_INDENT_STEP_PT_V0: f32 = 18.0;
const TOC_PAGE_NO_COLUMN_WIDTH_PT_V2: f32 = 48.0;
const TOC_PAGE_NO_COLUMN_GAP_PT_V2: f32 = 12.0;
const TOC_PAGE_NO_COLUMN_RIGHT_INSET_PT_V5: f32 = 2.0;
const BIBLIOGRAPHY_HEADING_TEXT_V0: &[u8] = b"References";
const BIBLIOGRAPHY_HEADING_TO_FIRST_ENTRY_GAP_PT_V6: f32 = 12.0;
const BIBLIOGRAPHY_ENTRY_LEADING_PT_V6: f32 = 13.0;
const BIBLIOGRAPHY_BODY_INDENT_PT_V6: f32 = INDENT_PT_V0 + 8.0;
const BIBLIOGRAPHY_LABEL_COLUMN_RIGHT_PT_V6: f32 = 26.0;
const SECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@S ";
const SUBSECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@s ";
const DISPLAY_MATH_PLACEHOLDER_SHORT_V0: &[u8] = b"MATH DISPLAY";
const DISPLAY_MATH_PLACEHOLDER_MEDIUM_V0: &[u8] = b"MATH DISPLAY MEDIUM";
const DISPLAY_MATH_PLACEHOLDER_LONG_V0: &[u8] = b"MATH DISPLAY LONG FORM";
const PAGEREF_RENDER_MARKER_PREFIX_V0: &[u8] = b"@@PG:";
const PAGEREF_RENDER_MARKER_SUFFIX_V0: &[u8] = b"@@";
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';

#[derive(Clone, Copy, PartialEq, Eq)]
enum PdfTextStyleV0 {
    Regular,
    Italic,
    Bold,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HeadingKindV0 {
    Section,
    Subsection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FigurePlacementHintV0 {
    Inline,
    Top,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InlineBlockAlignmentV0 {
    Center,
    Right,
}


// Inline/text segmentation and marker parsing.
include!("pdf_v0/style_and_segments_v0.rs");
include!("pdf_v0/line_markers_and_blocks_v0.rs");
include!("pdf_v0/text_emit_v0.rs");
// Table/float/toc block rendering and metadata extraction.
include!("pdf_v0/table_float_toc_emit_v0.rs");
include!("pdf_v0/metadata_parse_v0.rs");
// Link/annotation collection and page-level render assembly.
include!("pdf_v0/annotations_and_metadata_flow_v0.rs");
include!("pdf_v0/content_stream_and_pdf_v0.rs");
include!("pdf_v0/render_entrypoint_v0.rs");
