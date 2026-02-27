#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenV0 {
    ControlSeq(Vec<u8>),
    Char(u8),
    BeginGroup,
    EndGroup,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeErrorV0 {
    InvalidInput,
    TooManyTokens,
}

pub const MAX_TOKENS_V0: usize = 1_000_000;

fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

fn push_token(tokens: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), TokenizeErrorV0> {
    if tokens.len() >= MAX_TOKENS_V0 {
        return Err(TokenizeErrorV0::TooManyTokens);
    }
    tokens.push(token);
    Ok(())
}

/// Tokenize TeX input bytes with strict v0 assumptions.
///
/// v0 rules:
/// - Rejects NUL (`0x00`) anywhere (`InvalidInput`).
/// - Rejects `^^` byte sequence anywhere (`InvalidInput`).
/// - `%` starts a comment that is skipped until `\n` or EOF; the newline itself
///   is not consumed by the comment and is processed normally.
/// - Whitespace bytes (`' '`, `\t`, `\r`, `\n`) collapse into one `Space`.
/// - `{` and `}` become `BeginGroup` / `EndGroup`.
/// - `\` starts a control sequence:
///   - If followed by ASCII letters, consume a control word and emit
///     `ControlSeq(name_bytes)`. A following whitespace run is swallowed
///     (no `Space` token emitted).
///   - Control word `\verb` is explicitly blocked in v0 (`InvalidInput`).
///   - Otherwise emit control symbol as `ControlSeq(vec![next_byte])`.
///   - A trailing terminal backslash is `InvalidInput`.
/// - All other bytes become `Char(byte)`.
/// - Fails with `TooManyTokens` if output would exceed `MAX_TOKENS_V0`.
pub fn tokenize_v0(input: &[u8]) -> Result<Vec<TokenV0>, TokenizeErrorV0> {
    if input.contains(&0) {
        return Err(TokenizeErrorV0::InvalidInput);
    }
    if input.windows(2).any(|pair| pair == b"^^") {
        return Err(TokenizeErrorV0::InvalidInput);
    }

    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < input.len() {
        let byte = input[index];

        if byte == b'%' {
            index += 1;
            while index < input.len() && input[index] != b'\n' {
                index += 1;
            }
            continue;
        }

        if is_whitespace(byte) {
            index += 1;
            while index < input.len() && is_whitespace(input[index]) {
                index += 1;
            }
            push_token(&mut tokens, TokenV0::Space)?;
            continue;
        }

        match byte {
            b'{' => {
                push_token(&mut tokens, TokenV0::BeginGroup)?;
                index += 1;
            }
            b'}' => {
                push_token(&mut tokens, TokenV0::EndGroup)?;
                index += 1;
            }
            b'\\' => {
                if index + 1 >= input.len() {
                    return Err(TokenizeErrorV0::InvalidInput);
                }

                let next = input[index + 1];
                if next.is_ascii_alphabetic() {
                    let mut end = index + 2;
                    while end < input.len() && input[end].is_ascii_alphabetic() {
                        end += 1;
                    }
                    let control_word = &input[index + 1..end];
                    if control_word == b"verb" {
                        return Err(TokenizeErrorV0::InvalidInput);
                    }
                    push_token(&mut tokens, TokenV0::ControlSeq(control_word.to_vec()))?;
                    index = end;
                    if index < input.len() && is_whitespace(input[index]) {
                        while index < input.len() && is_whitespace(input[index]) {
                            index += 1;
                        }
                    }
                } else {
                    push_token(&mut tokens, TokenV0::ControlSeq(vec![next]))?;
                    index += 2;
                }
            }
            _ => {
                push_token(&mut tokens, TokenV0::Char(byte))?;
                index += 1;
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{tokenize_v0, TokenV0, TokenizeErrorV0, MAX_TOKENS_V0};

    fn contains_control_seq(tokens: &[TokenV0], name: &[u8]) -> bool {
        tokens
            .iter()
            .any(|token| matches!(token, TokenV0::ControlSeq(bytes) if bytes.as_slice() == name))
    }

    #[test]
    fn tokenizes_minimal_document_and_contains_expected_control_words() {
        let input = b"\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}\n";
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert!(contains_control_seq(&tokens, b"documentclass"));
        assert!(contains_control_seq(&tokens, b"begin"));
        assert!(contains_control_seq(&tokens, b"end"));
    }

    #[test]
    fn percent_comment_is_skipped_until_newline() {
        let input = b"a%comment ignored\nb";
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(
            tokens,
            vec![TokenV0::Char(b'a'), TokenV0::Space, TokenV0::Char(b'b')]
        );
    }

    #[test]
    fn whitespace_is_coalesced_to_single_space_token() {
        let input = b"a \t\r\n b";
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(
            tokens,
            vec![TokenV0::Char(b'a'), TokenV0::Space, TokenV0::Char(b'b')]
        );
    }

    #[test]
    fn space_after_control_word_is_ignored() {
        let input = b"\\foo bar";
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(
            tokens,
            vec![
                TokenV0::ControlSeq(b"foo".to_vec()),
                TokenV0::Char(b'b'),
                TokenV0::Char(b'a'),
                TokenV0::Char(b'r')
            ]
        );
    }

    #[test]
    fn nul_byte_is_invalid_input() {
        let input = b"abc\0def";
        assert_eq!(tokenize_v0(input), Err(TokenizeErrorV0::InvalidInput));
    }

    #[test]
    fn double_caret_sequence_is_invalid_input() {
        assert_eq!(tokenize_v0(b"a^^b"), Err(TokenizeErrorV0::InvalidInput));
    }

    #[test]
    fn verb_control_word_is_invalid_input() {
        assert_eq!(
            tokenize_v0(b"\\verb|x|"),
            Err(TokenizeErrorV0::InvalidInput)
        );
    }

    #[test]
    fn too_many_tokens_is_fail_closed() {
        let input = vec![b'x'; MAX_TOKENS_V0 + 1];
        assert_eq!(tokenize_v0(&input), Err(TokenizeErrorV0::TooManyTokens));
    }
}
