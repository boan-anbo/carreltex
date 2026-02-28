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
fn accent_control_symbol_braced_tilde_passthrough_maps_to_payload_char() {
    let tokens = tokenize_v0(b"\\~{a}").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'a')]);
}

#[test]
fn accent_control_symbol_braced_caret_passthrough_maps_to_payload_char() {
    let tokens = tokenize_v0(b"\\^{o}").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'o')]);
}

#[test]
fn accent_control_symbol_braced_quote_passthrough_maps_to_payload_char() {
    let tokens = tokenize_v0(b"\\\"{u}").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'u')]);
}

#[test]
fn accent_control_symbol_braced_payload_accepts_literal_control_symbol_percent() {
    let tokens = tokenize_v0(b"\\~{\\%}").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'%')]);
}

#[test]
fn accent_control_symbol_braced_payload_accepts_literal_control_symbol_comma() {
    let tokens = tokenize_v0(b"\\~{\\,}").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b' ')]);
}

#[test]
fn unsupported_accent_forms_are_accent_not_supported() {
    assert_eq!(
        tokenize_v0(b"\\~a"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
    assert_eq!(
        tokenize_v0(b"\\~{}"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
    assert_eq!(
        tokenize_v0(b"\\\"{ab}"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
    assert_eq!(
        tokenize_v0(b"\\~{\\}"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
    assert_eq!(
        tokenize_v0(b"\\~^^7ba^^7d"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
    assert_eq!(
        tokenize_v0(b"\\~{\\par}"),
        Err(TokenizeErrorV0::AccentNotSupported)
    );
}

#[test]
fn accent_payload_nul_is_invalid_input() {
    assert_eq!(tokenize_v0(b"\\~{^^00}"), Err(TokenizeErrorV0::InvalidInput));
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
fn control_symbol_bang_is_noop_and_drops_token() {
    let tokens = tokenize_v0(b"\\!X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_bang_noop_does_not_swallow_following_whitespace() {
    let tokens = tokenize_v0(b"\\! X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Space, TokenV0::Char(b'X')]);
}

#[test]
fn control_symbol_semicolon_maps_to_space_char_without_swallowing_following_space() {
    let tokens = tokenize_v0(b"\\; X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b' '), TokenV0::Space, TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_semicolon_between_chars_emits_single_space_char() {
    let tokens = tokenize_v0(b"A\\;B").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'A'), TokenV0::Char(b' '), TokenV0::Char(b'B')]
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
fn control_word_textasciitilde_maps_to_tilde_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciitilde X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'~'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciicircum_maps_to_caret_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciicircum X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'^'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotedbl_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotedbl X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textless_maps_to_less_than_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textless X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'<'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textgreater_maps_to_greater_than_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textgreater X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'>'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbar_maps_to_pipe_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbar X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'|'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbraceleft_maps_to_lbrace_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbraceleft X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'{'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbraceright_maps_to_rbrace_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbraceright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'}'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textunderscore_maps_to_underscore_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textunderscore X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'_'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotesingle_maps_to_single_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotesingle X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\''), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciigrave_maps_to_backtick_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciigrave X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'`'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotedblleft_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotedblleft X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotedblright_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotedblright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textendash_maps_to_dash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textendash X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textemdash_maps_to_dash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textemdash X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textellipsis_maps_to_three_dots_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textellipsis X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'.'),
            TokenV0::Char(b'.'),
            TokenV0::Char(b'.'),
            TokenV0::Char(b'X')
        ]
    );
}

#[test]
fn control_word_textbullet_maps_to_asterisk_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbullet X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textdegree_maps_to_o_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdegree X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'o'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textdagger_maps_to_plus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdagger X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'+'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textdaggerdbl_maps_to_hash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdaggerdbl X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'#'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textsection_maps_to_s_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textsection X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'S'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textparagraph_maps_to_p_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textparagraph X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'P'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textcopyright_maps_to_c_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textcopyright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'c'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textregistered_maps_to_r_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textregistered X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'R'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textordfeminine_maps_to_a_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textordfeminine X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'a'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textordmasculine_maps_to_o_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textordmasculine X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'o'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textyen_maps_to_y_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textyen X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'Y'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textsterling_maps_to_l_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textsterling X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'L'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbrokenbar_maps_to_pipe_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbrokenbar X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'|'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textcurrency_maps_to_c_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textcurrency X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'C'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textexclamdown_maps_to_exclamation_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textexclamdown X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'!'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquestiondown_maps_to_question_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquestiondown X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'?'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textguillemotleft_maps_to_less_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textguillemotleft X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'<'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textguillemotright_maps_to_greater_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textguillemotright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'>'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquoteleft_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquoteleft X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\''), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquoteright_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquoteright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\''), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotedblbase_maps_to_double_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotedblbase X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textquotesinglbase_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textquotesinglbase X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\''), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textminus_maps_to_dash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textminus X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textplus_maps_to_plus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textplus X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'+'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textequals_maps_to_equals_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textequals X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'='), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textcolon_maps_to_colon_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textcolon X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b':'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textsemicolon_maps_to_semicolon_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textsemicolon X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b';'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textcomma_maps_to_comma_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textcomma X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b','), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textperiod_maps_to_dot_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textperiod X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'.'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textslash_maps_to_slash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textslash X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'/'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textparenleft_maps_to_left_paren_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textparenleft X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'('), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textparenright_maps_to_right_paren_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textparenright X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b')'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciimacron_maps_to_dash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciimacron X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciibreve_maps_to_u_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciibreve X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'u'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciidieresis_maps_to_double_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciidieresis X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciicaron_maps_to_v_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciicaron X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'v'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textnumero_maps_to_n_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textnumero X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'N'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textordmhyphen_maps_to_dash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textordmhyphen X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textopenbullet_maps_to_o_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textopenbullet X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'o'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textleaf_maps_to_l_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textleaf X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'L'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textmusicalnote_maps_to_n_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textmusicalnote X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'n'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textreferencemark_maps_to_asterisk_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textreferencemark X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textonehalf_maps_to_fraction_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textonehalf X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'1'),
            TokenV0::Char(b'/'),
            TokenV0::Char(b'2'),
            TokenV0::Char(b'X'),
        ]
    );
}

#[test]
fn control_word_textonequarter_maps_to_fraction_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textonequarter X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'1'),
            TokenV0::Char(b'/'),
            TokenV0::Char(b'4'),
            TokenV0::Char(b'X'),
        ]
    );
}

#[test]
fn control_word_textthreequarters_maps_to_fraction_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textthreequarters X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'3'),
            TokenV0::Char(b'/'),
            TokenV0::Char(b'4'),
            TokenV0::Char(b'X'),
        ]
    );
}

#[test]
fn control_word_texttimes_maps_to_star_and_swallows_space() {
    let tokens = tokenize_v0(b"\\texttimes X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textdiv_maps_to_slash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdiv X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'/'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textpm_maps_to_plus_minus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textpm X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'+'), TokenV0::Char(b'-'), TokenV0::Char(b'X')]
    );
}

#[test]
fn control_word_textdag_maps_to_plus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdag X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'+'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textbardbl_maps_to_double_bar_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textbardbl X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'|'), TokenV0::Char(b'|'), TokenV0::Char(b'X')]
    );
}

#[test]
fn control_word_textasciiacute_maps_to_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciiacute X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'\''), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciidblquote_maps_to_double_quote_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciidblquote X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'"'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasteriskcentered_maps_to_asterisk_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasteriskcentered X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textperiodcentered_maps_to_dot_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textperiodcentered X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'.'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_texttrademark_maps_to_t_and_swallows_space() {
    let tokens = tokenize_v0(b"\\texttrademark X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'T'), TokenV0::Char(b'X')]);
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
