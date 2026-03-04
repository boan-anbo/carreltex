use crate::tex::tokenize_v0::TokenV0;

use super::super::ok_v0_body::{consume_char_space_group_non_empty, skip_spaces};
use super::ok_v0_extract_preamble::consume_balanced_group_discard_non_empty_v0;

pub(super) fn consume_biblatex_format_preamble_command(tokens: &[TokenV0], index: usize) -> Option<usize> {
    let name = match tokens.get(index) {
        Some(TokenV0::ControlSeq(name)) => name.as_slice(),
        _ => return None,
    };
    match name {
        b"DeclareFieldFormat" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
                cursor = skip_spaces(tokens, cursor);
            }
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_balanced_group_discard_non_empty_v0(tokens, cursor, 4096)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareNameFormat" | b"DeclareDelimFormat" | b"DeclareBibliographyDriver" | b"DefineBibliographyStrings" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_balanced_group_discard_non_empty_v0(tokens, cursor, 4096)?;
            Some(skip_spaces(tokens, cursor))
        }
        b"DeclareDelimAlias" => {
            let mut cursor = skip_spaces(tokens, index + 1);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            cursor = skip_spaces(tokens, cursor);
            cursor = consume_char_space_group_non_empty(tokens, cursor)?;
            Some(skip_spaces(tokens, cursor))
        }
        _ => None,
    }
}
