use crate::tex::tokenize_v0::TokenV0;
use std::collections::BTreeMap;

const NEWLINE_MARKER_V0: u8 = 0x0a;
const PAGE_BREAK_MARKER_V0: u8 = 0x0c;
const CARRELPAR_MARKER_CONTROL_V0: &[u8] = b"carrelpar";
const CARRELNEWLINE_MARKER_CONTROL_V0: &[u8] = b"carrelnewline";
const HARD_LINE_BREAK_CONTROL_V0: &[u8] = b"\\";
const NEWLINE_ALIAS_CONTROL_V0: &[u8] = b"newline";
const LINEBREAK_ALIAS_CONTROL_V0: &[u8] = b"linebreak";
const PAGEBREAK_ALIAS_CONTROL_V0: &[u8] = b"pagebreak";
const TABLEOFCONTENTS_CONTROL_V0: &[u8] = b"tableofcontents";
const BEGIN_CONTROL_V0: &[u8] = b"begin";
const END_CONTROL_V0: &[u8] = b"end";
const ITEM_CONTROL_V0: &[u8] = b"item";
const DOCUMENT_ENV_V0: &[u8] = b"document";
const ITEMIZE_ENV_V0: &[u8] = b"itemize";
const ENUMERATE_ENV_V0: &[u8] = b"enumerate";
const QUOTE_ENV_V0: &[u8] = b"quote";
const CENTER_ENV_V0: &[u8] = b"center";
const CENTERLINE_CONTROL_V0: &[u8] = b"centerline";
const FLUSHRIGHT_ENV_V0: &[u8] = b"flushright";
const RIGHTLINE_CONTROL_V0: &[u8] = b"rightline";
const TABULAR_ENV_V0: &[u8] = b"tabular";
const FIGURE_ENV_V0: &[u8] = b"figure";
const THEBIBLIOGRAPHY_ENV_V0: &[u8] = b"thebibliography";
const DOCUMENTCLASS_CONTROL_V0: &[u8] = b"documentclass";
const USEPACKAGE_CONTROL_V0: &[u8] = b"usepackage";
const REQUIREPACKAGE_CONTROL_V0: &[u8] = b"RequirePackage";
const REQUIREPACKAGEWITHOPTIONS_CONTROL_V0: &[u8] = b"RequirePackageWithOptions";
const PASSOPTIONSTOPACKAGE_CONTROL_V0: &[u8] = b"PassOptionsToPackage";
const PASSOPTIONSTOCLASS_CONTROL_V0: &[u8] = b"PassOptionsToClass";
const ADDBIBRESOURCE_CONTROL_V0: &[u8] = b"addbibresource";
const PRINTBIBLIOGRAPHY_CONTROL_V0: &[u8] = b"printbibliography";
const CAPTION_CONTROL_V0: &[u8] = b"caption";
const GRAPHICSPATH_CONTROL_V0: &[u8] = b"graphicspath";
const INCLUDEGRAPHICS_CONTROL_V0: &[u8] = b"includegraphics";
const INCLUDEONLY_CONTROL_V0: &[u8] = b"includeonly";
const BIBITEM_CONTROL_V0: &[u8] = b"bibitem";
const BIBLIOGRAPHY_CONTROL_V0: &[u8] = b"bibliography";
const BIBLIOGRAPHYSTYLE_CONTROL_V0: &[u8] = b"bibliographystyle";
const CITE_CONTROL_V0: &[u8] = b"cite";
const FOOTNOTE_CONTROL_V0: &[u8] = b"footnote";
const HREF_CONTROL_V0: &[u8] = b"href";
const LABEL_CONTROL_V0: &[u8] = b"label";
const REF_CONTROL_V0: &[u8] = b"ref";
const PAGEREF_CONTROL_V0: &[u8] = b"pageref";
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
const TABLE_SPEC_PREFIX_MARKER_V0: &[u8] = b"!ts ";
const TABLE_ROW_PREFIX_MARKER_V0: &[u8] = b"!t ";
const FIGURE_BOX_PREFIX_MARKER_V0: &[u8] = b"!gbox";
const FIGURE_IMAGE_PREFIX_MARKER_V0: &[u8] = b"!gimg ";
const FIGURE_CAPTION_PREFIX_MARKER_V0: &[u8] = b"!gcap ";
const DEFAULT_FIGURE_PLACEHOLDER_WIDTH_MPT_V0: u32 = 180_000;
const DEFAULT_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0: u32 = 120_000;
const MAX_FIGURE_PLACEHOLDER_WIDTH_MPT_V0: u32 = 468_000;
const MAX_FIGURE_PLACEHOLDER_HEIGHT_MPT_V0: u32 = 288_000;
const INCLUDEGRAPHICS_ALLOWED_WIDTH_UNITS_V0: [&[u8]; 4] = [b"pt", b"mm", b"cm", b"in"];
const TOC_PLACEHOLDER_MARKER_V0: &[u8] = b"!toc";
const TOC_ENTRY_LINE_PREFIX_MARKER_V0: &[u8] = b"!toc ";
const REF_MARKER_PREFIX_V0: &[u8] = b"@@REF:";
const PAGEREF_MARKER_PREFIX_V0: &[u8] = b"@@PAGEREF:";
const REF_MARKER_SUFFIX_V0: &[u8] = b"@@";
const PAGEREF_RENDER_MARKER_PREFIX_V0: &[u8] = b"@@PG:";
const PAGEREF_RENDER_MARKER_SUFFIX_V0: &[u8] = b"@@";
const CITE_MARKER_PREFIX_V0: &[u8] = b"@@CITE:";
const CITE_MARKER_SUFFIX_V0: &[u8] = b"@@";
const INLINE_MATH_PLACEHOLDER_V0: &[u8] = b"MATH";
const DISPLAY_MATH_PLACEHOLDER_SHORT_V0: &[u8] = b"MATH DISPLAY";
const DISPLAY_MATH_PLACEHOLDER_MEDIUM_V0: &[u8] = b"MATH DISPLAY MEDIUM";
const DISPLAY_MATH_PLACEHOLDER_LONG_V0: &[u8] = b"MATH DISPLAY LONG FORM";
const DISPLAY_MATH_SHORT_MAX_PAYLOAD_BYTES_V0: usize = 24;
const DISPLAY_MATH_MEDIUM_MAX_PAYLOAD_BYTES_V0: usize = 72;
const LINK_START_MARKER_V0: u8 = b'<';
const LINK_END_MARKER_V0: u8 = b'>';
const NOINDENT_PREFIX_MARKER_V0: &[u8] = b"~ ";
const SECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@S ";
const SUBSECTION_HEADING_PREFIX_MARKER_V0: &[u8] = b"@s ";
const ITALIC_START_MARKER_V0: u8 = b'[';
const ITALIC_END_MARKER_V0: u8 = b']';
const BOLD_START_MARKER_V0: u8 = b'{';
const BOLD_END_MARKER_V0: u8 = b'}';
const SECTION_CONTROL_V0: &[u8] = b"section";
const SUBSECTION_CONTROL_V0: &[u8] = b"subsection";
const SUBSUBSECTION_CONTROL_V0: &[u8] = b"subsubsection";
const PARAGRAPH_CONTROL_V0: &[u8] = b"paragraph";
const SUBPARAGRAPH_CONTROL_V0: &[u8] = b"subparagraph";
const MAX_TABULAR_COLUMNS_V0: usize = 16;

#[derive(Clone)]
struct TocEntryMetaV0 {
    level: u8,
    anchor_id: u32,
    title: Vec<u8>,
}

#[derive(Clone, Copy)]
enum LabelKindV0 {
    Heading,
    Figure,
    Equation,
}

#[derive(Clone)]
struct LabelEntryMetaV0 {
    anchor_id: u32,
    kind: LabelKindV0,
    level: Option<u8>,
    figure_ordinal: Option<u32>,
    equation_ordinal: Option<u32>,
    title: Option<Vec<u8>>,
}

#[derive(Clone)]
struct PendingLabelTargetV0 {
    anchor_id: u32,
    kind: LabelKindV0,
    level: Option<u8>,
    figure_ordinal: Option<u32>,
    equation_ordinal: Option<u32>,
    title: Option<Vec<u8>>,
}

#[derive(Clone)]
struct RefOccurrenceMetaV0 {
    kind: RefKindV0,
    key: Vec<u8>,
    line_index: u32,
    resolved_anchor_id: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RefKindV0 {
    Ref,
    Pageref,
}

#[derive(Clone)]
struct HrefLinkMetaV0 {
    link_id: u32,
    url: Vec<u8>,
}

#[derive(Clone)]
struct RefAnchorLinkMetaV0 {
    link_id: u32,
    anchor_id: u32,
}

#[derive(Clone)]
struct PagerefPageLinkMetaV0 {
    link_id: u32,
    anchor_id: u32,
}

#[derive(Clone)]
struct EquationMetaV0 {
    anchor_id: u32,
    ordinal: u32,
}

struct CrossRefArtifactsV1 {
    labels_by_key: BTreeMap<Vec<u8>, LabelEntryMetaV0>,
    heading_anchor_ids: BTreeMap<u32, ()>,
    hyperref_enabled: bool,
}

#[derive(Clone)]
struct BibItemMetaV0 {
    key: Vec<u8>,
    text: Vec<u8>,
    text_len: u32,
}

#[derive(Clone)]
struct CiteOccurrenceMetaV0 {
    key: Vec<u8>,
    line_index: u32,
    resolved_ordinal: Option<u32>,
}

#[derive(Clone, Copy)]
struct FigureSizingMptV0 {
    width_mpt: u32,
    height_mpt: u32,
}

#[derive(Clone)]
struct IncludeGraphicsCommandV0 {
    path: Vec<u8>,
    sizing: FigureSizingMptV0,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FigurePlacementHintV0 {
    Inline,
    Top,
}

#[derive(Default)]
struct TitleMetaV0 {
    title: Option<Vec<u8>>,
    author: Option<Vec<u8>>,
    date: Option<Vec<u8>>,
}

include!("preamble.rs");
include!("math.rs");
include!("text_flow.rs");
include!("graphics_float.rs");
include!("bibliography.rs");
include!("labels_refs.rs");
include!("extract.rs");
