use super::{tokenize_v0, TokenV0};

#[test]
fn control_word_textfractionsolidus_maps_to_slash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textfractionsolidus X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'/'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasterisklow_maps_to_asterisk_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasterisklow X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textdoublepipe_maps_to_double_pipe_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textdoublepipe X").expect("tokenize should succeed");
    assert_eq!(
        tokens,
        vec![TokenV0::Char(b'|'), TokenV0::Char(b'|'), TokenV0::Char(b'X')]
    );
}

#[test]
fn control_word_textasciicomma_maps_to_comma_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciicomma X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b','), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciiperiod_maps_to_dot_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciiperiod X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'.'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciicolon_maps_to_colon_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciicolon X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b':'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciiplus_maps_to_plus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciiplus X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'+'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciiminus_maps_to_minus_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciiminus X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'-'), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciiequal_maps_to_equal_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciiequal X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'='), TokenV0::Char(b'X')]);
}

#[test]
fn control_word_textasciislash_maps_to_slash_and_swallows_space() {
    let tokens = tokenize_v0(b"\\textasciislash X").expect("tokenize should succeed");
    assert_eq!(tokens, vec![TokenV0::Char(b'/'), TokenV0::Char(b'X')]);
}
