use super::{tokenize_v0, TokenV0};

#[test]
fn control_words_leaf_147_map_to_expected_tokens_and_swallow_space() {
    for (input, expected) in [
        (
            b"\\textoneeighth X".as_slice(),
            vec![
                TokenV0::Char(b'1'),
                TokenV0::Char(b'/'),
                TokenV0::Char(b'8'),
                TokenV0::Char(b'X'),
            ],
        ),
        (
            b"\\textthreeeighths X".as_slice(),
            vec![
                TokenV0::Char(b'3'),
                TokenV0::Char(b'/'),
                TokenV0::Char(b'8'),
                TokenV0::Char(b'X'),
            ],
        ),
        (
            b"\\textfiveeighths X".as_slice(),
            vec![
                TokenV0::Char(b'5'),
                TokenV0::Char(b'/'),
                TokenV0::Char(b'8'),
                TokenV0::Char(b'X'),
            ],
        ),
        (
            b"\\textseveneighths X".as_slice(),
            vec![
                TokenV0::Char(b'7'),
                TokenV0::Char(b'/'),
                TokenV0::Char(b'8'),
                TokenV0::Char(b'X'),
            ],
        ),
        (
            b"\\textlnot X".as_slice(),
            vec![TokenV0::Char(b'!'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textbigcircle X".as_slice(),
            vec![TokenV0::Char(b'O'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textmarried X".as_slice(),
            vec![TokenV0::Char(b'M'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdivorced X".as_slice(),
            vec![TokenV0::Char(b'D'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textopenstar X".as_slice(),
            vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textborn X".as_slice(),
            vec![TokenV0::Char(b'*'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdied X".as_slice(),
            vec![TokenV0::Char(b'+'), TokenV0::Char(b'X')],
        ),
        (
            b"\\texttildelow X".as_slice(),
            vec![TokenV0::Char(b'~'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdblhyphen X".as_slice(),
            vec![TokenV0::Char(b'-'), TokenV0::Char(b'-'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textdiscount X".as_slice(),
            vec![TokenV0::Char(b'%'), TokenV0::Char(b'X')],
        ),
        (
            b"\\textpilcrow X".as_slice(),
            vec![TokenV0::Char(b'P'), TokenV0::Char(b'X')],
        ),
    ] {
        let tokens = tokenize_v0(input).expect("tokenize should succeed");
        assert_eq!(tokens, expected);
    }
}
