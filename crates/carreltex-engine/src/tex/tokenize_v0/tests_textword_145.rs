use super::{tokenize_v0, TokenV0};

#[test]
fn control_words_leaf_145_map_to_expected_tokens_and_swallow_space() {
    for (input, expected) in [
        (
            b"\\textmu X".as_slice(),
            vec![TokenV0::Char(b'u'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textohm X".as_slice(),
            vec![TokenV0::Char(b'O'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textmho X".as_slice(),
            vec![TokenV0::Char(b'm'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textcelsius X".as_slice(),
            vec![TokenV0::Char(b'C'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textnaira X".as_slice(),
            vec![TokenV0::Char(b'N'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textpeso X".as_slice(),
            vec![TokenV0::Char(b'P'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textwon X".as_slice(),
            vec![TokenV0::Char(b'W'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textrupee X".as_slice(),
            vec![TokenV0::Char(b'R'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textbaht X".as_slice(),
            vec![TokenV0::Char(b'B'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textflorin X".as_slice(),
            vec![TokenV0::Char(b'f'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textcolonmonetary X".as_slice(),
            vec![TokenV0::Char(b'C'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdong X".as_slice(),
            vec![TokenV0::Char(b'd'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textlira X".as_slice(),
            vec![TokenV0::Char(b'l'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textestimated X".as_slice(),
            vec![TokenV0::Char(b'e'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textrecipe X".as_slice(),
            vec![TokenV0::Char(b'r'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textservicemark X".as_slice(),
            vec![TokenV0::Char(b'S'), TokenV0::Char(b'M'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textcopyleft X".as_slice(),
            vec![TokenV0::Char(b'c'), TokenV0::Char(b'c'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textinterrobang X".as_slice(),
            vec![TokenV0::Char(b'!'), TokenV0::Char(b'?'), TokenV0::Char(b'X')],
        ),
    ] {
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(tokens, expected);
    }
}
