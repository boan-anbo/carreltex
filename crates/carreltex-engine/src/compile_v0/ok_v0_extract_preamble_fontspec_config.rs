use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{
    consume_bracket_options_non_empty, consume_char_space_group_non_empty, skip_spaces,
};
use super::ok_v0_extract_preamble::consume_balanced_group_discard_non_empty_v0;

pub(super) fn consume_fontspec_config_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };

    match name {
        b"setmainfont" | b"setsansfont" | b"setmonofont" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'['))) {
                cursor = consume_bracket_options_non_empty(tokens, cursor)?;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"sisetup" | b"microtypesetup" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_balanced_group_discard_non_empty_v0(tokens, cursor, 4096)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
