use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_bracket_options_non_empty, consume_char_space_group_non_empty, skip_spaces,
};

pub(super) fn consume_index_page_style_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    let mut cursor = skip_spaces(tokens, index + 1);
    match name {
        b"makeindex" => {
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
            }
            Some(skip_spaces(tokens, cursor))
        }
        b"pagenumbering" | b"pagestyle" | b"thispagestyle" => {
            if matches!(tokens.get(cursor), Some(TokenV0::BeginGroup)) {
                cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            }
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
