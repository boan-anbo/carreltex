use super::{TokenV0, TokenizeErrorV0, tokenize_v0};

#[test]
fn verb_payload_maps_to_literal_chars() {
    let tokens = tokenize_v0(b"\\verb|abc|X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'a'),
            TokenV0::Char(b'b'),
            TokenV0::Char(b'c'),
            TokenV0::Char(b'X')
        ]
    );
}

#[test]
fn verb_payload_keeps_backslash_literal() {
    let tokens = tokenize_v0(b"\\verb|\\alpha|Z").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'\\'),
            TokenV0::Char(b'a'),
            TokenV0::Char(b'l'),
            TokenV0::Char(b'p'),
            TokenV0::Char(b'h'),
            TokenV0::Char(b'a'),
            TokenV0::Char(b'Z')
        ]
    );
}

#[test]
fn verb_payload_keeps_braces_percent_and_control_chars_literal() {
    let tokens = tokenize_v0(b"\\verb|{\\%}|Q").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::Char(b'{'),
            TokenV0::Char(b'\\'),
            TokenV0::Char(b'%'),
            TokenV0::Char(b'}'),
            TokenV0::Char(b'Q')
        ]
    );
}

#[test]
fn unclosed_verb_is_not_supported() {
    assert_eq!(
        tokenize_v0(b"\\verb|abc"),
        Err(TokenizeErrorV0::VerbNotSupported)
    );
}

#[test]
fn verb_star_variant_is_not_supported() {
    assert_eq!(
        tokenize_v0(b"\\verb*|a|"),
        Err(TokenizeErrorV0::VerbNotSupported)
    );
}
