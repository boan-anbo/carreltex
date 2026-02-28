use super::{tokenize_v0, TokenV0};

#[test]
fn control_words_leaf_148_map_to_expected_tokens_and_swallow_space() {
    for (input, expected) in [
        (
            b"\\textalpha X".as_slice(),
            vec![TokenV0::Char(b'a'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textbeta X".as_slice(),
            vec![TokenV0::Char(b'b'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textgamma X".as_slice(),
            vec![TokenV0::Char(b'g'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdelta X".as_slice(),
            vec![TokenV0::Char(b'd'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textepsilon X".as_slice(),
            vec![TokenV0::Char(b'e'), TokenV0::Char(b'X')],
        ),
        (
            b"\\texttheta X".as_slice(),
            vec![TokenV0::Char(b't'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textlambda X".as_slice(),
            vec![TokenV0::Char(b'l'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textpi X".as_slice(),
            vec![TokenV0::Char(b'p'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textrho X".as_slice(),
            vec![TokenV0::Char(b'r'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textsigma X".as_slice(),
            vec![TokenV0::Char(b's'), TokenV0::Char(b'X')],
        ),
        (
            b"\\texttau X".as_slice(),
            vec![TokenV0::Char(b'u'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textphi X".as_slice(),
            vec![TokenV0::Char(b'f'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textchi X".as_slice(),
            vec![TokenV0::Char(b'c'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textpsi X".as_slice(),
            vec![TokenV0::Char(b'y'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textomega X".as_slice(),
            vec![TokenV0::Char(b'w'), TokenV0::Char(b'X')],
        ),
    ] {
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(tokens, expected);
    }
}
