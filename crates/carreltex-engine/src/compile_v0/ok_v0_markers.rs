pub(super) fn ok_marker_command_v0(name: &[u8]) -> Option<&'static [u8]> {
    match name {
        b"cite"
        | b"citet"
        | b"citep"
        | b"citealt"
        | b"citealp"
        | b"citeauthor"
        | b"citeyear"
        | b"citeyearpar"
        | b"parencite"
        | b"textcite"
        | b"autocite"
        | b"footcite" => Some(b"CITE"),
        b"ref" | b"autoref" | b"cref" | b"Cref" => Some(b"REF"),
        b"pageref" => Some(b"PAGEREF"),
        b"eqref" => Some(b"EQREF"),
        _ => None,
    }
}

pub(super) fn is_cite_marker_command_v0(name: &[u8]) -> bool {
    matches!(
        name,
        b"cite"
            | b"citet"
            | b"citep"
            | b"citealt"
            | b"citealp"
            | b"citeauthor"
            | b"citeyear"
            | b"citeyearpar"
            | b"parencite"
            | b"textcite"
            | b"autocite"
            | b"footcite"
    )
}

pub(super) fn is_ref_marker_command_v0(name: &[u8]) -> bool {
    matches!(name, b"ref" | b"autoref" | b"eqref" | b"pageref" | b"cref" | b"Cref")
}
