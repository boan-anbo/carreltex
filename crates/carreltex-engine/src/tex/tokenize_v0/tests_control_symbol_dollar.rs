use super::{TokenV0, tokenize_v0};

#[test]
fn control_symbol_dollar_maps_to_distinct_control_symbol_token() {
    let tokens = tokenize_v0(b"\\$X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::ControlSeq(vec![b'$']), TokenV0::Char(b'X')]
    );
}

#[test]
fn control_symbol_dollar_keeps_following_space_token() {
    let tokens = tokenize_v0(b"\\$ X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![
            TokenV0::ControlSeq(vec![b'$']),
            TokenV0::Space,
            TokenV0::Char(b'X')
        ]
    );
}
