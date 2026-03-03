#[path = "ok_v0_env_support.rs"]
mod ok_v0_env_support;
#[path = "ok_v0_env_refs.rs"]
mod ok_v0_env_refs;
#[path = "ok_v0_optional_brackets.rs"]
mod ok_v0_optional_brackets;
#[path = "ok_v0_dollar_math.rs"]
mod ok_v0_dollar_math;
#[path = "ok_v0_ensuremath.rs"]
mod ok_v0_ensuremath;
#[path = "ok_v0_lists.rs"]
mod ok_v0_lists;
#[path = "ok_v0_biblabel.rs"]
mod ok_v0_biblabel;
#[path = "ok_v0_noops.rs"]
mod ok_v0_noops;
#[path = "ok_v0_markers.rs"]
mod ok_v0_markers;
#[path = "ok_v0_body.rs"]
mod ok_v0_body;
#[path = "ok_v0_extract.rs"]
mod ok_v0_extract;

pub(crate) const MAX_OK_TEXT_BYTES_V0: usize = 64 * 1024;
pub(crate) const OK_GLYPH_ADVANCE_SP_V0: i32 = 65_536;
pub(crate) const OK_LINE_ADVANCE_SP_V0: i32 = 786_432;

const MAX_OK_GROUP_DEPTH_V0: usize = 64;
const MAX_OK_BRACKET_BYTES_V0: usize = 256;
const MAX_OK_MATH_SCAN_TOKENS_V0: usize = 4096;
const MAX_OK_MATH_ENV_TOKENS_V0: usize = 4096;
const MAX_OK_DOLLAR_MATH_TOKENS_V0: usize = 4096;
const MAX_OK_ENSUREMATH_TOKENS_V0: usize = 4096;
const MAX_OK_HEADING_SHORT_TOKENS_V0: usize = 2048;
const MAX_OK_CITE_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_REF_NOTE_TOKENS_V0: usize = 2048;
const MAX_OK_BIBLABEL_TOKENS_V0: usize = 256;

use ok_v0_body::consume_char_space_nested_group_v0;
use ok_v0_optional_brackets::consume_optional_nested_bracket_span_v0;
pub(crate) use ok_v0_extract::extract_strict_ok_text_body_v0;
