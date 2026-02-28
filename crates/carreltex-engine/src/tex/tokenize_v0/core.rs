use super::caret::decode_caret_hex_v0;
use super::comment::skip_comment_raw_v0;
use super::control_seq::parse_control_seq_v0;
use super::whitespace::{consume_whitespace_run_v0, is_whitespace_v0};
use super::{TokenV0, TokenizeErrorV0, MAX_TOKENS_V0};

fn push_token_v0(tokens: &mut Vec<TokenV0>, token: TokenV0) -> Result<(), TokenizeErrorV0> {
    if tokens.len() >= MAX_TOKENS_V0 {
        return Err(TokenizeErrorV0::TooManyTokens);
    }
    tokens.push(token);
    Ok(())
}

pub fn tokenize_v0(input: &[u8]) -> Result<Vec<TokenV0>, TokenizeErrorV0> {
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < input.len() {
        if input[index] == b'%' {
            index = skip_comment_raw_v0(input, index);
            continue;
        }

        let (byte, next_index) = decode_caret_hex_v0(input, index)?;
        if byte == 0 {
            return Err(TokenizeErrorV0::InvalidInput);
        }

        if is_whitespace_v0(byte) {
            index = consume_whitespace_run_v0(input, next_index)?;
            push_token_v0(&mut tokens, TokenV0::Space)?;
            continue;
        }

        match byte {
            b'{' => {
                push_token_v0(&mut tokens, TokenV0::BeginGroup)?;
                index = next_index;
            }
            b'}' => {
                push_token_v0(&mut tokens, TokenV0::EndGroup)?;
                index = next_index;
            }
            b'\\' => {
                let parsed = parse_control_seq_v0(input, next_index)?;
                for token in parsed.tokens {
                    push_token_v0(&mut tokens, token)?;
                }
                index = parsed.next_index;
            }
            _ => {
                push_token_v0(&mut tokens, TokenV0::Char(byte))?;
                index = next_index;
            }
        }
    }

    Ok(tokens)
}
