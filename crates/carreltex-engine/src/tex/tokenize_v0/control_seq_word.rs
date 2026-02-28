use super::super::caret::decode_caret_hex_v0;
use super::super::whitespace::{consume_whitespace_run_v0, is_whitespace_v0};
use super::{ParsedControlSeqV0, TokenV0, TokenizeErrorV0};

pub(super) fn parse_control_word_v0(
    input: &[u8],
    first_byte: u8,
    mut index: usize,
) -> Result<ParsedControlSeqV0, TokenizeErrorV0> {
    let mut control_word = Vec::<u8>::new();
    control_word.push(first_byte);
    while index < input.len() {
        let (word_byte, following_index) = decode_caret_hex_v0(input, index)?;
        if word_byte == 0 {
            return Err(TokenizeErrorV0::InvalidInput);
        }
        if !word_byte.is_ascii_alphabetic() {
            break;
        }
        control_word.push(word_byte);
        index = following_index;
    }
    if control_word.as_slice() == b"verb" {
        return Err(TokenizeErrorV0::InvalidInput);
    }
    if !control_word.iter().all(|byte| byte.is_ascii()) {
        return Err(TokenizeErrorV0::ControlSeqNonAscii);
    }
    if index < input.len() {
        let (space_probe, _) = decode_caret_hex_v0(input, index)?;
        if !is_whitespace_v0(space_probe) && !space_probe.is_ascii() {
            return Err(TokenizeErrorV0::ControlSeqNonAscii);
        }
        if is_whitespace_v0(space_probe) {
            index = consume_whitespace_run_v0(input, index)?;
        }
    }
    let tokens = if control_word.as_slice() == b"textbackslash" {
        vec![TokenV0::Char(b'\\')]
    } else if control_word.as_slice() == b"textasciitilde" {
        vec![TokenV0::Char(b'~')]
    } else if control_word.as_slice() == b"textasciicircum" {
        vec![TokenV0::Char(b'^')]
    } else if control_word.as_slice() == b"textquotedbl" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textless" {
        vec![TokenV0::Char(b'<')]
    } else if control_word.as_slice() == b"textgreater" {
        vec![TokenV0::Char(b'>')]
    } else if control_word.as_slice() == b"textbar" {
        vec![TokenV0::Char(b'|')]
    } else if control_word.as_slice() == b"textbraceleft" {
        vec![TokenV0::Char(b'{')]
    } else if control_word.as_slice() == b"textbraceright" {
        vec![TokenV0::Char(b'}')]
    } else if control_word.as_slice() == b"textunderscore" {
        vec![TokenV0::Char(b'_')]
    } else if control_word.as_slice() == b"textquotesingle" {
        vec![TokenV0::Char(b'\'')]
    } else if control_word.as_slice() == b"textasciigrave" {
        vec![TokenV0::Char(b'`')]
    } else if control_word.as_slice() == b"textquotedblleft" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textquotedblright" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textendash" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textemdash" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textellipsis" {
        vec![TokenV0::Char(b'.'), TokenV0::Char(b'.'), TokenV0::Char(b'.')]
    } else if control_word.as_slice() == b"textbullet" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textdegree" {
        vec![TokenV0::Char(b'o')]
    } else if control_word.as_slice() == b"textdagger" {
        vec![TokenV0::Char(b'+')]
    } else if control_word.as_slice() == b"textdaggerdbl" {
        vec![TokenV0::Char(b'#')]
    } else if control_word.as_slice() == b"textsection" {
        vec![TokenV0::Char(b'S')]
    } else if control_word.as_slice() == b"textparagraph" {
        vec![TokenV0::Char(b'P')]
    } else if control_word.as_slice() == b"textcopyright" {
        vec![TokenV0::Char(b'c')]
    } else if control_word.as_slice() == b"textregistered" {
        vec![TokenV0::Char(b'R')]
    } else if control_word.as_slice() == b"textordfeminine" {
        vec![TokenV0::Char(b'a')]
    } else if control_word.as_slice() == b"textordmasculine" {
        vec![TokenV0::Char(b'o')]
    } else if control_word.as_slice() == b"textyen" {
        vec![TokenV0::Char(b'Y')]
    } else if control_word.as_slice() == b"textsterling" {
        vec![TokenV0::Char(b'L')]
    } else if control_word.as_slice() == b"textasteriskcentered" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textperiodcentered" {
        vec![TokenV0::Char(b'.')]
    } else if control_word.as_slice() == b"texttrademark" {
        vec![TokenV0::Char(b'T')]
    } else if control_word.as_slice() == b"textbrokenbar" {
        vec![TokenV0::Char(b'|')]
    } else if control_word.as_slice() == b"textcurrency" {
        vec![TokenV0::Char(b'C')]
    } else if control_word.as_slice() == b"textexclamdown" {
        vec![TokenV0::Char(b'!')]
    } else if control_word.as_slice() == b"textquestiondown" {
        vec![TokenV0::Char(b'?')]
    } else if control_word.as_slice() == b"textguillemotleft" {
        vec![TokenV0::Char(b'<')]
    } else if control_word.as_slice() == b"textguillemotright" {
        vec![TokenV0::Char(b'>')]
    } else if control_word.as_slice() == b"textquoteleft" {
        vec![TokenV0::Char(b'\'')]
    } else if control_word.as_slice() == b"textquoteright" {
        vec![TokenV0::Char(b'\'')]
    } else if control_word.as_slice() == b"textquotedblbase" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textquotesinglbase" {
        vec![TokenV0::Char(b'\'')]
    } else if control_word.as_slice() == b"textminus" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textplus" {
        vec![TokenV0::Char(b'+')]
    } else if control_word.as_slice() == b"textequals" {
        vec![TokenV0::Char(b'=')]
    } else if control_word.as_slice() == b"textcolon" {
        vec![TokenV0::Char(b':')]
    } else if control_word.as_slice() == b"textsemicolon" {
        vec![TokenV0::Char(b';')]
    } else if control_word.as_slice() == b"textcomma" {
        vec![TokenV0::Char(b',')]
    } else if control_word.as_slice() == b"textperiod" {
        vec![TokenV0::Char(b'.')]
    } else if control_word.as_slice() == b"textslash" {
        vec![TokenV0::Char(b'/')]
    } else if control_word.as_slice() == b"textparenleft" {
        vec![TokenV0::Char(b'(')]
    } else if control_word.as_slice() == b"textparenright" {
        vec![TokenV0::Char(b')')]
    } else if control_word.as_slice() == b"textasciimacron" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textasciibreve" {
        vec![TokenV0::Char(b'u')]
    } else if control_word.as_slice() == b"textasciidieresis" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textasciicaron" {
        vec![TokenV0::Char(b'v')]
    } else if control_word.as_slice() == b"textnumero" {
        vec![TokenV0::Char(b'N')]
    } else if control_word.as_slice() == b"textordmhyphen" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textopenbullet" {
        vec![TokenV0::Char(b'o')]
    } else if control_word.as_slice() == b"textleaf" {
        vec![TokenV0::Char(b'L')]
    } else if control_word.as_slice() == b"textmusicalnote" {
        vec![TokenV0::Char(b'n')]
    } else if control_word.as_slice() == b"textreferencemark" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textonehalf" {
        vec![TokenV0::Char(b'1'), TokenV0::Char(b'/'), TokenV0::Char(b'2')]
    } else if control_word.as_slice() == b"textonequarter" {
        vec![TokenV0::Char(b'1'), TokenV0::Char(b'/'), TokenV0::Char(b'4')]
    } else if control_word.as_slice() == b"textthreequarters" {
        vec![TokenV0::Char(b'3'), TokenV0::Char(b'/'), TokenV0::Char(b'4')]
    } else if control_word.as_slice() == b"texttimes" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textdiv" {
        vec![TokenV0::Char(b'/')]
    } else if control_word.as_slice() == b"textpm" {
        vec![TokenV0::Char(b'+'), TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textdag" {
        vec![TokenV0::Char(b'+')]
    } else if control_word.as_slice() == b"textbardbl" {
        vec![TokenV0::Char(b'|'), TokenV0::Char(b'|')]
    } else if control_word.as_slice() == b"textasciiacute" {
        vec![TokenV0::Char(b'\'')]
    } else if control_word.as_slice() == b"textasciidblquote" {
        vec![TokenV0::Char(b'"')]
    } else if control_word.as_slice() == b"textcent" {
        vec![TokenV0::Char(b'c')]
    } else if control_word.as_slice() == b"texteuro" {
        vec![TokenV0::Char(b'E')]
    } else if control_word.as_slice() == b"textperthousand" {
        vec![
            TokenV0::Char(b'0'),
            TokenV0::Char(b'/'),
            TokenV0::Char(b'0'),
            TokenV0::Char(b'0'),
        ]
    } else if control_word.as_slice() == b"textpertenthousand" {
        vec![
            TokenV0::Char(b'0'),
            TokenV0::Char(b'/'),
            TokenV0::Char(b'0'),
            TokenV0::Char(b'0'),
            TokenV0::Char(b'0'),
        ]
    } else if control_word.as_slice() == b"textlangle" {
        vec![TokenV0::Char(b'<')]
    } else if control_word.as_slice() == b"textrangle" {
        vec![TokenV0::Char(b'>')]
    } else if control_word.as_slice() == b"textleftarrow" {
        vec![TokenV0::Char(b'<'), TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textrightarrow" {
        vec![TokenV0::Char(b'-'), TokenV0::Char(b'>')]
    } else if control_word.as_slice() == b"textuparrow" {
        vec![TokenV0::Char(b'^')]
    } else if control_word.as_slice() == b"textdownarrow" {
        vec![TokenV0::Char(b'v')]
    } else if control_word.as_slice() == b"textlbrack" {
        vec![TokenV0::Char(b'[')]
    } else if control_word.as_slice() == b"textrbrack" {
        vec![TokenV0::Char(b']')]
    } else if control_word.as_slice() == b"textlbrace" {
        vec![TokenV0::Char(b'{')]
    } else if control_word.as_slice() == b"textrbrace" {
        vec![TokenV0::Char(b'}')]
    } else if control_word.as_slice() == b"textleftparen" {
        vec![TokenV0::Char(b'(')]
    } else if control_word.as_slice() == b"textrightparen" {
        vec![TokenV0::Char(b')')]
    } else if control_word.as_slice() == b"textpipe" {
        vec![TokenV0::Char(b'|')]
    } else if control_word.as_slice() == b"textasciispace" {
        vec![TokenV0::Space]
    } else if control_word.as_slice() == b"textvisiblehyphen" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textvisiblespace" {
        vec![TokenV0::Char(b'_')]
    } else if control_word.as_slice() == b"textfractionsolidus" {
        vec![TokenV0::Char(b'/')]
    } else if control_word.as_slice() == b"textasterisklow" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textdoublepipe" {
        vec![TokenV0::Char(b'|'), TokenV0::Char(b'|')]
    } else if control_word.as_slice() == b"textasciicomma" {
        vec![TokenV0::Char(b',')]
    } else if control_word.as_slice() == b"textasciiperiod" {
        vec![TokenV0::Char(b'.')]
    } else if control_word.as_slice() == b"textasciicolon" {
        vec![TokenV0::Char(b':')]
    } else if control_word.as_slice() == b"textasciiplus" {
        vec![TokenV0::Char(b'+')]
    } else if control_word.as_slice() == b"textasciiminus" {
        vec![TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textasciiequal" {
        vec![TokenV0::Char(b'=')]
    } else if control_word.as_slice() == b"textasciislash" {
        vec![TokenV0::Char(b'/')]
    } else if control_word.as_slice() == b"textmu" {
        vec![TokenV0::Char(b'u')]
    } else if control_word.as_slice() == b"textohm" {
        vec![TokenV0::Char(b'O')]
    } else if control_word.as_slice() == b"textmho" {
        vec![TokenV0::Char(b'm')]
    } else if control_word.as_slice() == b"textcelsius" {
        vec![TokenV0::Char(b'C')]
    } else if control_word.as_slice() == b"textnaira" {
        vec![TokenV0::Char(b'N')]
    } else if control_word.as_slice() == b"textpeso" {
        vec![TokenV0::Char(b'P')]
    } else if control_word.as_slice() == b"textwon" {
        vec![TokenV0::Char(b'W')]
    } else if control_word.as_slice() == b"textrupee" {
        vec![TokenV0::Char(b'R')]
    } else if control_word.as_slice() == b"textbaht" {
        vec![TokenV0::Char(b'B')]
    } else if control_word.as_slice() == b"textflorin" {
        vec![TokenV0::Char(b'f')]
    } else if control_word.as_slice() == b"textcolonmonetary" {
        vec![TokenV0::Char(b'C')]
    } else if control_word.as_slice() == b"textdong" {
        vec![TokenV0::Char(b'd')]
    } else if control_word.as_slice() == b"textlira" {
        vec![TokenV0::Char(b'l')]
    } else if control_word.as_slice() == b"textestimated" {
        vec![TokenV0::Char(b'e')]
    } else if control_word.as_slice() == b"textrecipe" {
        vec![TokenV0::Char(b'r')]
    } else if control_word.as_slice() == b"textservicemark" {
        vec![TokenV0::Char(b'S'), TokenV0::Char(b'M')]
    } else if control_word.as_slice() == b"textcopyleft" {
        vec![TokenV0::Char(b'c'), TokenV0::Char(b'c')]
    } else if control_word.as_slice() == b"textinterrobang" {
        vec![TokenV0::Char(b'!'), TokenV0::Char(b'?')]
    } else if control_word.as_slice() == b"textalpha" {
        vec![TokenV0::Char(b'a')]
    } else if control_word.as_slice() == b"textbeta" {
        vec![TokenV0::Char(b'b')]
    } else if control_word.as_slice() == b"textgamma" {
        vec![TokenV0::Char(b'g')]
    } else if control_word.as_slice() == b"textdelta" {
        vec![TokenV0::Char(b'd')]
    } else if control_word.as_slice() == b"textepsilon" {
        vec![TokenV0::Char(b'e')]
    } else if control_word.as_slice() == b"texttheta" {
        vec![TokenV0::Char(b't')]
    } else if control_word.as_slice() == b"textlambda" {
        vec![TokenV0::Char(b'l')]
    } else if control_word.as_slice() == b"textpi" {
        vec![TokenV0::Char(b'p')]
    } else if control_word.as_slice() == b"textrho" {
        vec![TokenV0::Char(b'r')]
    } else if control_word.as_slice() == b"textsigma" {
        vec![TokenV0::Char(b's')]
    } else if control_word.as_slice() == b"texttau" {
        vec![TokenV0::Char(b'u')]
    } else if control_word.as_slice() == b"textphi" {
        vec![TokenV0::Char(b'f')]
    } else if control_word.as_slice() == b"textchi" {
        vec![TokenV0::Char(b'c')]
    } else if control_word.as_slice() == b"textpsi" {
        vec![TokenV0::Char(b'y')]
    } else if control_word.as_slice() == b"textomega" {
        vec![TokenV0::Char(b'w')]
    } else if control_word.as_slice() == b"textoneeighth" {
        vec![TokenV0::Char(b'1'), TokenV0::Char(b'/'), TokenV0::Char(b'8')]
    } else if control_word.as_slice() == b"textthreeeighths" {
        vec![TokenV0::Char(b'3'), TokenV0::Char(b'/'), TokenV0::Char(b'8')]
    } else if control_word.as_slice() == b"textfiveeighths" {
        vec![TokenV0::Char(b'5'), TokenV0::Char(b'/'), TokenV0::Char(b'8')]
    } else if control_word.as_slice() == b"textseveneighths" {
        vec![TokenV0::Char(b'7'), TokenV0::Char(b'/'), TokenV0::Char(b'8')]
    } else if control_word.as_slice() == b"textlnot" {
        vec![TokenV0::Char(b'!')]
    } else if control_word.as_slice() == b"textbigcircle" {
        vec![TokenV0::Char(b'O')]
    } else if control_word.as_slice() == b"textmarried" {
        vec![TokenV0::Char(b'M')]
    } else if control_word.as_slice() == b"textdivorced" {
        vec![TokenV0::Char(b'D')]
    } else if control_word.as_slice() == b"textopenstar" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textborn" {
        vec![TokenV0::Char(b'*')]
    } else if control_word.as_slice() == b"textdied" {
        vec![TokenV0::Char(b'+')]
    } else if control_word.as_slice() == b"texttildelow" {
        vec![TokenV0::Char(b'~')]
    } else if control_word.as_slice() == b"textdblhyphen" {
        vec![TokenV0::Char(b'-'), TokenV0::Char(b'-')]
    } else if control_word.as_slice() == b"textdiscount" {
        vec![TokenV0::Char(b'%')]
    } else if control_word.as_slice() == b"textpilcrow" {
        vec![TokenV0::Char(b'P')]
    } else if control_word.as_slice() == b"pagebreak" {
        vec![TokenV0::Char(0x0c)]
    } else if control_word.as_slice() == b"newline" {
        vec![TokenV0::Char(0x0a)]
    } else if control_word.as_slice() == b"par" {
        vec![TokenV0::Space]
    } else {
        vec![TokenV0::ControlSeq(control_word)]
    };
    Ok(ParsedControlSeqV0 {
        tokens,
        next_index: index,
    })
}
