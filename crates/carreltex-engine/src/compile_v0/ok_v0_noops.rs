use crate::tex::tokenize_v0::TokenV0;

use super::consume_char_space_nested_group_v0;

fn skip_spaces_until(tokens: &[TokenV0], mut index: usize, end: usize) -> usize {
    while index < end {
        if matches!(tokens.get(index), Some(TokenV0::Space)) {
            index += 1;
            continue;
        }
        break;
    }
    index
}

pub(super) fn is_ok_noop_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"bibliographystyle"
            | b"bibliography"
            | b"nocite"
            | b"phantomsection"
            | b"addcontentsline"
            | b"addtocontents"
            | b"markboth"
            | b"markright"
            | b"thispagestyle"
            | b"pagestyle"
            | b"smallskip"
            | b"medskip"
            | b"bigskip"
            | b"hfill"
            | b"vfill"
            | b"newpage"
            | b"clearpage"
            | b"pagebreak"
            | b"nopagebreak"
            | b"linebreak"
            | b"nolinebreak"
            | b"vspace"
            | b"hspace"
    )
}

pub(super) fn consume_ok_noop_command_v0(
    tokens: &[TokenV0],
    index: usize,
    end: usize,
    name: &[u8],
) -> Option<usize> {
    match name {
        b"phantomsection"
        | b"smallskip"
        | b"medskip"
        | b"bigskip"
        | b"hfill"
        | b"vfill"
        | b"newpage"
        | b"clearpage"
        | b"pagebreak"
        | b"nopagebreak"
        | b"linebreak"
        | b"nolinebreak" => Some(index + 1),
        b"vspace" | b"hspace" => {
            let mut cursor = skip_spaces_until(tokens, index + 1, end);
            if matches!(tokens.get(cursor), Some(TokenV0::Char(b'*'))) {
                cursor += 1;
                cursor = skip_spaces_until(tokens, cursor, end);
            }
            consume_char_space_nested_group_v0(tokens, cursor, end)
        }
        b"bibliographystyle"
        | b"bibliography"
        | b"nocite"
        | b"markright"
        | b"thispagestyle"
        | b"pagestyle" => consume_char_space_nested_group_v0(tokens, index + 1, end),
        b"addtocontents" | b"markboth" => {
            let next = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
            consume_char_space_nested_group_v0(tokens, next, end)
        }
        b"addcontentsline" => {
            let next = consume_char_space_nested_group_v0(tokens, index + 1, end)?;
            let next = consume_char_space_nested_group_v0(tokens, next, end)?;
            consume_char_space_nested_group_v0(tokens, next, end)
        }
        _ => None,
    }
}
