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
fn crlf_collapses_to_single_space_token() {
    let tokens = tokenize_v0(b"A\r\nB").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'A'), TokenV0::Space, TokenV0::Char(b'B')]
    );
}

#[test]
fn lone_cr_collapses_to_single_space_token() {
    let tokens = tokenize_v0(b"A\rB").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'A'), TokenV0::Space, TokenV0::Char(b'B')]
    );
}

#[test]
fn percent_comment_terminated_by_cr_does_not_emit_double_space() {
    let input = b"a%comment\rb";
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
fn control_symbol_comma_maps_to_space_char_without_swallowing_following_space() {
    let tokens = tokenize_v0(b"\\, X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b' '), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_percent_maps_to_percent_char() {
    let tokens = tokenize_v0(b"\\%X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'%'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_percent_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\% X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'%'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_underscore_maps_to_underscore_char() {
    let tokens = tokenize_v0(b"\\_X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'_'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_underscore_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\_ X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'_'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_hash_maps_to_hash_char() {
    let tokens = tokenize_v0(b"\\#X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'#'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_hash_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\# X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'#'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_dollar_maps_to_dollar_char() {
    let tokens = tokenize_v0(b"\\$X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'$'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_dollar_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\$ X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'$'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_ampersand_maps_to_ampersand_char() {
    let tokens = tokenize_v0(b"\\&X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'&'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_ampersand_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\& X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'&'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_lbrace_maps_to_lbrace_char() {
    let tokens = tokenize_v0(b"\\{X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'{'), TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_rbrace_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\} X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'}'), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_word_textbackslash_maps_to_backslash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbackslash X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\\'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbackslash_then_percent_symbol_maps_to_backslash_and_percent() {
    let tokens = tokenize_v0(b"\\textbackslash\\%").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\\'), TokenV0::Char(b'%')]);
}

#[test]
fn control_word_par_maps_to_space_and_swallows_following_whitespace() {
    let tokens = tokenize_v0(b"A\\par B").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'A'), TokenV0::Space, TokenV0::Char(b'B')]
    );
}

#[test]
fn repeated_control_word_par_emits_repeated_spaces() {
    let tokens = tokenize_v0(b"A\\par\\par B").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'A'),
            TokenV0::Space,
            TokenV0::Space,
            TokenV0::Char(b'B')
        ]
    );
}

#[test]
fn control_word_parxyz_is_not_par_prefix() {
    let tokens = tokenize_v0(b"\\parXYZ").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::ControlSeq(b"parXYZ".to_vec())]);
}

#[test]
fn control_word_partial_is_not_par_prefix() {
    let tokens = tokenize_v0(b"\\partial").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::ControlSeq(b"partial".to_vec())]);
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
