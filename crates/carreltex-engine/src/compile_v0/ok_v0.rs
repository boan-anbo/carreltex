use crate::tex::tokenize_v0::TokenV0;

fn skip_spaces(tokens: &[TokenV0], mut index: usize) -> usize {
    while matches!(tokens.get(index), Some(TokenV0::Space)) {
        index += 1;
    }
    index
}

fn consume_group_literal(tokens: &[TokenV0], mut index: usize, literal: &[u8]) -> Option<usize> {
    if !matches!(tokens.get(index), Some(TokenV0::BeginGroup)) {
        return None;
    }
    index += 1;
    for expected in literal {
        if !matches!(tokens.get(index), Some(TokenV0::Char(value)) if value == expected) {
            return None;
        }
        index += 1;
    }
    if !matches!(tokens.get(index), Some(TokenV0::EndGroup)) {
        return None;
    }
    Some(index + 1)
}

pub(crate) fn is_strict_empty_article_doc_v0(tokens: &[TokenV0]) -> bool {
    let mut index = 0usize;
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return false;
    }
    index += 1;
    index = match consume_group_literal(tokens, index, b"article") {
        Some(next) => next,
        None => return false,
    };
    index = skip_spaces(tokens, index);

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin"
    ) {
        return false;
    }
    index += 1;
    index = match consume_group_literal(tokens, index, b"document") {
        Some(next) => next,
        None => return false,
    };
    index = skip_spaces(tokens, index);

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        return false;
    }
    index += 1;
    index = match consume_group_literal(tokens, index, b"document") {
        Some(next) => next,
        None => return false,
    };
    index = skip_spaces(tokens, index);
    index == tokens.len()
}
