use super::{tokenize_v0, TokenV0};

#[test]
fn control_word_newline_maps_to_linefeed_char_and_swallows_following_whitespace() {
    let tokens = tokenize_v0(b"A\\newline B").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'A'), TokenV0::Char(0x0a), TokenV0::Char(b'B')]
    );
}
