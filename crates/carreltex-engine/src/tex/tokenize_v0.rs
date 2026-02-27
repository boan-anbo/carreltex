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
    CaretNotSupported,
    ControlSeqNonAscii,
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
/// - Decodes `^^hh` where `h` is hex (`[0-9a-fA-F]`) to one byte.
/// - Any other `^^` form is `CaretNotSupported`.
/// - `%` starts a comment that is skipped until `\n` or EOF; the newline itself
///   is not consumed by the comment and is processed normally. Caret decoding
///   is not applied while consuming comment bytes.
/// - Whitespace bytes (`' '`, `\t`, `\r`, `\n`) collapse into one `Space`.
/// - `{` and `}` become `BeginGroup` / `EndGroup`.
/// - `\` starts a control sequence:
///   - If followed by ASCII letters, consume a control word and emit
///     `ControlSeq(name_bytes)`. A following whitespace run is swallowed
///     (no `Space` token emitted).
///   - Control sequence bytes must be ASCII-only (`ControlSeqNonAscii` on
///     violations).
///   - Control word `\verb` is explicitly blocked in v0 (`InvalidInput`).
///   - Otherwise emit control symbol as `ControlSeq(vec![next_byte])`.
///   - A trailing terminal backslash is `InvalidInput`.
/// - All other bytes become `Char(byte)`.
/// - Fails with `TooManyTokens` if output would exceed `MAX_TOKENS_V0`.
pub fn tokenize_v0(input: &[u8]) -> Result<Vec<TokenV0>, TokenizeErrorV0> {
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < input.len() {
        if input[index] == b'%' {
            index += 1;
            while index < input.len() && input[index] != b'\n' {
                index += 1;
            }
            continue;
        }

        let (byte, next_index) = decode_caret_hex_v0(input, index)?;
        if byte == 0 {
            return Err(TokenizeErrorV0::InvalidInput);
        }

        if is_whitespace(byte) {
            index = next_index;
            while index < input.len() {
                let (next_byte, following_index) = decode_caret_hex_v0(input, index)?;
                if !is_whitespace(next_byte) {
                    break;
                }
                index = following_index;
            }
            push_token(&mut tokens, TokenV0::Space)?;
            continue;
        }

        match byte {
            b'{' => {
                push_token(&mut tokens, TokenV0::BeginGroup)?;
                index = next_index;
            }
            b'}' => {
                push_token(&mut tokens, TokenV0::EndGroup)?;
                index = next_index;
            }
            b'\\' => {
                index = next_index;
                if index >= input.len() {
                    return Err(TokenizeErrorV0::InvalidInput);
                }

                let (next, after_next_index) = decode_caret_hex_v0(input, index)?;
                if next == 0 {
                    return Err(TokenizeErrorV0::InvalidInput);
                }
                if next.is_ascii_alphabetic() {
                    let mut control_word = Vec::<u8>::new();
                    control_word.push(next);
                    index = after_next_index;
                    while index < input.len() {
                        let (word_byte, following_index) = decode_caret_hex_v0(input, index)?;
                        if word_byte == 0 {
                            return Err(TokenizeErrorV0::InvalidInput);
                        }
                        if !word_byte.is_ascii_alphabetic() {
                            break;
                        }
                        control_word.push(word_byte);
                        index = following_index;
                    }
                    if control_word.as_slice() == b"verb" {
                        return Err(TokenizeErrorV0::InvalidInput);
                    }
                    if !control_word.iter().all(|byte| byte.is_ascii()) {
                        return Err(TokenizeErrorV0::ControlSeqNonAscii);
                    }
                    push_token(&mut tokens, TokenV0::ControlSeq(control_word))?;
                    if index < input.len() {
                        let (space_probe, _) = decode_caret_hex_v0(input, index)?;
                        if !is_whitespace(space_probe) && !space_probe.is_ascii() {
                            return Err(TokenizeErrorV0::ControlSeqNonAscii);
                        }
                        if is_whitespace(space_probe) {
                            while index < input.len() {
                                let (space_byte, following_index) = decode_caret_hex_v0(input, index)?;
                                if !is_whitespace(space_byte) {
                                    break;
                                }
                                index = following_index;
                            }
                        }
                    }
                } else {
                    if !next.is_ascii() {
                        return Err(TokenizeErrorV0::ControlSeqNonAscii);
                    }
                    push_token(&mut tokens, TokenV0::ControlSeq(vec![next]))?;
                    index = after_next_index;
                }
            }
            _ => {
                push_token(&mut tokens, TokenV0::Char(byte))?;
                index = next_index;
            }
        }
    }

    Ok(tokens)
}

fn decode_caret_hex_v0(input: &[u8], index: usize) -> Result<(u8, usize), TokenizeErrorV0> {
    let byte = input[index];
    if byte != b'^' || index + 1 >= input.len() || input[index + 1] != b'^' {
        return Ok((byte, index + 1));
    }
    if index + 3 >= input.len() {
        return Err(TokenizeErrorV0::CaretNotSupported);
    }
    let high = parse_hex_nibble_v0(input[index + 2])?;
    let low = parse_hex_nibble_v0(input[index + 3])?;
    Ok((((high << 4) | low), index + 4))
}

fn parse_hex_nibble_v0(byte: u8) -> Result<u8, TokenizeErrorV0> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(TokenizeErrorV0::CaretNotSupported),
    }
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
    fn caret_hex_sequence_decodes_to_single_byte() {
        let tokens = tokenize_v0(b"A^^41B").expect("tokenize should succeed");
        assert_eq!(
            tokens,
            vec![TokenV0::Char(b'A'), TokenV0::Char(b'A'), TokenV0::Char(b'B')]
        );
    }

    #[test]
    fn caret_hex_ff_is_allowed() {
        let tokens = tokenize_v0(b"^^ff").expect("tokenize should succeed");
        assert_eq!(tokens, vec![TokenV0::Char(0xff)]);
    }

    #[test]
    fn caret_hex_uppercase_is_allowed() {
        let tokens = tokenize_v0(b"^^4A").expect("tokenize should succeed");
        assert_eq!(tokens, vec![TokenV0::Char(0x4a)]);
    }

    #[test]
    fn caret_hex_zero_decodes_to_nul_and_is_invalid() {
        assert_eq!(tokenize_v0(b"^^00"), Err(TokenizeErrorV0::InvalidInput));
    }

    #[test]
    fn unsupported_caret_form_is_caret_not_supported() {
        assert_eq!(tokenize_v0(b"^^ZZ"), Err(TokenizeErrorV0::CaretNotSupported));
    }

    #[test]
    fn caret_sequence_inside_comment_is_ignored_as_raw_text() {
        let tokens = tokenize_v0(b"% ^^ZZ\nX").expect("tokenize should succeed");
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token, TokenV0::Char(byte) if *byte == b'X'))
        );
    }

    #[test]
    fn control_sequence_bytes_must_be_ascii() {
        assert_eq!(
            tokenize_v0(b"\\foo^^ff"),
            Err(TokenizeErrorV0::ControlSeqNonAscii)
        );
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
