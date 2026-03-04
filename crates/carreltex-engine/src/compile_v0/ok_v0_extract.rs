use crate::tex::tokenize_v0::TokenV0;

use super::ok_v0_body::{
    consume_bracket_options_non_empty, consume_group_literal, consume_ok_body_token_v0,
    consume_ok_group_fragment_discard_v0,
    is_supported_ok_style_declaration_v0, skip_spaces,
};
use super::ok_v0_lists::ListStateV0;
use super::ok_v0_noops::{consume_ok_noop_command_v0, is_ok_noop_command_v0};
use super::ok_v0_title_state::OkTitleStateV0;

#[path = "ok_v0_extract_preamble.rs"]
mod ok_v0_extract_preamble;
#[path = "ok_v0_extract_preamble_metadata.rs"]
mod ok_v0_extract_preamble_metadata;
#[path = "ok_v0_extract_preamble_page_style.rs"]
mod ok_v0_extract_preamble_page_style;
#[path = "ok_v0_extract_preamble_sectioning_toc.rs"]
mod ok_v0_extract_preamble_sectioning_toc;
#[path = "ok_v0_extract_preamble_caption_footnote.rs"]
mod ok_v0_extract_preamble_caption_footnote;
#[path = "ok_v0_extract_preamble_caption_decls.rs"]
mod ok_v0_extract_preamble_caption_decls;
#[path = "ok_v0_extract_preamble_fancyhdr_glue.rs"]
mod ok_v0_extract_preamble_fancyhdr_glue;
#[path = "ok_v0_extract_preamble_biblatex_formats.rs"]
mod ok_v0_extract_preamble_biblatex_formats;
#[path = "ok_v0_extract_preamble_tikz_pgf.rs"]
mod ok_v0_extract_preamble_tikz_pgf;
#[path = "ok_v0_extract_preamble_fontspec_config.rs"]
mod ok_v0_extract_preamble_fontspec_config;
#[path = "ok_v0_extract_preamble_listings_algo_table.rs"]
mod ok_v0_extract_preamble_listings_algo_table;

use ok_v0_extract_preamble::{
    consume_biblatex_resource_preamble_command, consume_bibliography_preamble_command,
    consume_cite_ref_decl_preamble_command, consume_color_graphics_decl_preamble_command,
    consume_config_preamble_command, consume_declare_robust_command_preamble_command,
    consume_declare_text_command_preamble_command,
    consume_declare_text_composite_command_preamble_command,
    consume_declare_text_composite_preamble_command,
    consume_declare_text_font_command_preamble_command, consume_doc_hook_preamble_command,
    consume_font_decl_preamble_command, consume_math_accent_radical_decl_preamble_command,
    consume_math_alphabet_decl_preamble_command, consume_math_operator_preamble_command,
    consume_math_symbol_decl_preamble_command, consume_math_version_sizes_preamble_command,
    consume_mathcode_delcode_preamble_command,
    consume_package_option_plumbing_preamble_command, consume_label_aux_preamble_command,
    consume_length_counter_preamble_command, consume_mark_preamble_command,
    consume_hyperref_preamble_command, consume_float_listof_preamble_command,
    consume_setuptoc_preamble_command, consume_koma_config_preamble_command,
    consume_language_decl_preamble_command,
    consume_fancyhdr_preamble_command,
    consume_symbol_font_setter_preamble_command, consume_text_command_default_preamble_command,
    consume_text_decl_bundle_preamble_command, consume_theorem_preamble_command,
    consume_usepackage_preamble_command, is_supported_bibliography_preamble_command,
};
use ok_v0_extract_preamble_metadata::{
    consume_meta_preamble_command, consume_renewcommand_and_preamble_command,
    is_supported_meta_preamble_command,
};
use ok_v0_extract_preamble_page_style::consume_index_page_style_preamble_command;
use ok_v0_extract_preamble_caption_footnote::consume_caption_footnote_preamble_command;
use ok_v0_extract_preamble_caption_decls::consume_caption_decl_preamble_command;
use ok_v0_extract_preamble_fancyhdr_glue::consume_fancyhdr_glue_preamble_command;
use ok_v0_extract_preamble_biblatex_formats::consume_biblatex_format_preamble_command;
use ok_v0_extract_preamble_tikz_pgf::consume_tikz_pgf_preamble_command;
use ok_v0_extract_preamble_fontspec_config::consume_fontspec_config_preamble_command;
use ok_v0_extract_preamble_listings_algo_table::consume_listings_algo_table_preamble_command;
use ok_v0_extract_preamble_sectioning_toc::consume_sectioning_toc_preamble_command;

pub(crate) fn extract_strict_ok_text_body_v0(tokens: &[TokenV0]) -> Option<Vec<u8>> {
    let mut index = 0usize;
    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"documentclass"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    if matches!(tokens.get(index), Some(TokenV0::Char(b'['))) {
        index = consume_bracket_options_non_empty(tokens, index)?;
    }
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"article")?;
    index = skip_spaces(tokens, index);
    let mut title_state = OkTitleStateV0::default();
    loop {
        match tokens.get(index) {
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"usepackage" => {
                index = consume_usepackage_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"RequirePackage"
                        | b"PassOptionsToPackage"
                        | b"PassOptionsToClass"
                        | b"ExecuteOptions"
                ) =>
            {
                index = consume_package_option_plumbing_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"addbibresource"
                        | b"ExecuteBibliographyOptions"
                        | b"DeclareBibliographyCategory"
                        | b"addtocategory"
                        | b"DeclareLanguageMapping"
                        | b"DeclareBibliographyAlias"
                        | b"DeclareNameAlias"
                        | b"DeclareListAlias"
                        | b"DeclareFieldAlias"
                ) =>
            {
                index = consume_biblatex_resource_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareFieldFormat"
                        | b"DeclareNameFormat"
                        | b"DeclareDelimFormat"
                        | b"DeclareDelimAlias"
                        | b"DeclareBibliographyDriver"
                        | b"DefineBibliographyStrings"
                ) =>
            {
                index = consume_biblatex_format_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"usetikzlibrary"
                        | b"usepgfplotslibrary"
                        | b"tikzset"
                        | b"pgfplotsset"
                        | b"pgfkeys"
                        | b"pgfkeysalso"
                ) =>
            {
                index = consume_tikz_pgf_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"setmainfont"
                        | b"setsansfont"
                        | b"setmonofont"
                        | b"sisetup"
                        | b"microtypesetup"
                ) =>
            {
                index = consume_fontspec_config_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"lstset"
                        | b"lstdefinestyle"
                        | b"lstnewenvironment"
                        | b"SetKwInput"
                        | b"SetKw"
                        | b"newcolumntype"
                ) =>
            {
                index = consume_listings_algo_table_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"selectlanguage"
                        | b"setmainlanguage"
                        | b"setdefaultlanguage"
                        | b"setotherlanguage"
                ) =>
            {
                index = consume_language_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name)) if is_supported_meta_preamble_command(name.as_slice()) => {
                index = consume_meta_preamble_command(tokens, index, &mut title_state)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_bibliography_preamble_command(name.as_slice()) =>
            {
                index = consume_bibliography_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"label" | b"ref" | b"pageref" | b"addtocontents" | b"addcontentsline"
                ) =>
            {
                index = consume_label_aux_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"captionsetup" | b"floatname") =>
            {
                index = consume_caption_footnote_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareCaptionFormat"
                        | b"DeclareCaptionLabelFormat"
                        | b"DeclareCaptionLabelSeparator"
                        | b"DeclareCaptionFont"
                        | b"DeclareCaptionStyle"
                ) =>
            {
                index = consume_caption_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"leftmark"
                        | b"rightmark"
                        | b"nouppercase"
                        | b"MakeUppercase"
                        | b"MakeLowercase"
                ) =>
            {
                index = consume_fancyhdr_glue_preamble_command(tokens, index, &mut title_state)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"renewcommand" => {
                if let Some(next_index) = consume_caption_footnote_preamble_command(tokens, index) {
                    index = next_index;
                    continue;
                }
                if let Some(next_index) =
                    consume_fancyhdr_glue_preamble_command(tokens, index, &mut title_state)
                {
                    index = next_index;
                    continue;
                }
                if let Some(next_index) =
                    consume_sectioning_toc_preamble_command(tokens, index, &mut title_state)
                {
                    index = next_index;
                    continue;
                }
                if let Some(next_index) =
                    consume_renewcommand_and_preamble_command(tokens, index, &mut title_state)
                {
                    index = next_index;
                    continue;
                }
                return None;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"setcounter" | b"addto") =>
            {
                index = consume_sectioning_toc_preamble_command(tokens, index, &mut title_state)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"setlength" => {
                if let Some(next_index) = consume_caption_footnote_preamble_command(tokens, index) {
                    index = next_index;
                    continue;
                }
                if let Some(next_index) =
                    consume_fancyhdr_glue_preamble_command(tokens, index, &mut title_state)
                {
                    index = next_index;
                    continue;
                }
                index = consume_length_counter_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"setcounter" | b"addtolength") =>
            {
                index = consume_length_counter_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"markboth" | b"markright") =>
            {
                index = consume_mark_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"pdfstringdefDisableCommands" | b"AtBeginShipout" | b"AtBeginShipoutNext"
                ) =>
            {
                index = consume_hyperref_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"floatplacement") =>
            {
                index = consume_float_listof_preamble_command(tokens, index, &mut title_state)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"tableofcontents" | b"listoffigures" | b"listoftables"
                ) =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"setuptoc" =>
            {
                index = consume_setuptoc_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"KOMAoptions" | b"KOMAoption" | b"setkomafont" | b"addtokomafont"
                ) =>
            {
                index = consume_koma_config_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"fancyhf" | b"fancyhead" | b"fancyfoot" | b"fancypagestyle"
                ) =>
            {
                index = consume_fancyhdr_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"newtheorem" | b"theoremstyle" | b"newtheoremstyle" | b"swapnumbers"
                ) =>
            {
                index = consume_theorem_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"hypersetup"
                        | b"geometry"
                        | b"graphicspath"
                        | b"urlstyle"
                        | b"setlist"
                ) =>
            {
                index = consume_config_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"definecolor" | b"colorlet" | b"DeclareGraphicsExtensions"
                ) =>
            {
                index = consume_color_graphics_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"bibpunct"
                        | b"bibhang"
                        | b"citestyle"
                        | b"setcitestyle"
                        | b"crefname"
                        | b"Crefname"
                ) =>
            {
                index = consume_cite_ref_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"AtBeginDocument" | b"AtEndDocument") =>
            {
                index = consume_doc_hook_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"mathcode" | b"delcode") =>
            {
                index = consume_mathcode_delcode_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareMathVersion" | b"mathversion" | b"DeclareMathSizes"
                ) =>
            {
                index = consume_math_version_sizes_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareMathOperator" =>
            {
                index = consume_math_operator_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"DeclareMathAlphabet" | b"SetMathAlphabet") =>
            {
                index = consume_math_alphabet_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareSymbolFont" | b"DeclareMathSymbol" | b"DeclareMathDelimiter"
                ) =>
            {
                index = consume_math_symbol_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"SetSymbolFont" | b"DeclareSymbolFontAlphabet") =>
            {
                index = consume_symbol_font_setter_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"DeclareMathAccent" | b"DeclareMathRadical") =>
            {
                index = consume_math_accent_radical_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareFontEncoding"
                        | b"DeclareFontSubstitution"
                        | b"DeclareFontFamily"
                        | b"DeclareFontShape"
                        | b"DeclareFontEncodingDefaults"
                        | b"DeclareFontSeriesDefault"
                        | b"DeclareFontShapeDefault"
                        | b"DeclareFontFamilyDefault"
                ) =>
            {
                index = consume_font_decl_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareRobustCommand" =>
            {
                index = consume_declare_robust_command_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareTextFontCommand" =>
            {
                index = consume_declare_text_font_command_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"DeclareTextCommand" | b"ProvideTextCommand") =>
            {
                index = consume_declare_text_command_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"ProvideTextCommandDefault"
                        | b"DeclareTextCommandDefault"
                        | b"DeclareTextCompositeDefault"
                ) =>
            {
                index = consume_text_command_default_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"DeclareTextSymbol"
                        | b"DeclareTextAccent"
                        | b"DeclareTextAccentDefault"
                        | b"DeclareTextSymbolDefault"
                ) =>
            {
                index = consume_text_decl_bundle_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareTextCompositeCommand" =>
            {
                index = consume_declare_text_composite_command_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if name.as_slice() == b"DeclareTextComposite" =>
            {
                index = consume_declare_text_composite_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(name.as_slice(), b"protect" | b"relax") =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"makeindex"
                        | b"pagenumbering"
                        | b"pagestyle"
                        | b"thispagestyle"
                ) =>
            {
                index = consume_index_page_style_preamble_command(tokens, index)?;
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"makeatletter"
                        | b"makeatother"
                        | b"ExplSyntaxOn"
                        | b"ExplSyntaxOff"
                        | b"raggedbottom"
                        | b"flushbottom"
                        | b"sloppy"
                        | b"fussy"
                        | b"nofiles"
                        | b"listfiles"
                ) =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if matches!(
                    name.as_slice(),
                    b"frontmatter"
                        | b"mainmatter"
                        | b"backmatter"
                ) =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_supported_ok_style_declaration_v0(name.as_slice()) =>
            {
                index += 1;
                index = skip_spaces(tokens, index);
                continue;
            }
            Some(TokenV0::ControlSeq(name))
                if is_ok_noop_command_v0(name.as_slice()) =>
            {
                index = consume_ok_noop_command_v0(tokens, index, tokens.len(), name.as_slice())?;
                index = skip_spaces(tokens, index);
                continue;
            }
            _ => {}
        }
        break;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"begin"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);

    let mut body = Vec::<u8>::new();
    let mut previous_was_space = false;
    let mut list_env: Option<ListStateV0> = None;
    loop {
        if list_env.is_none()
            && matches!(
                tokens.get(index),
                Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
            )
            && consume_group_literal(tokens, skip_spaces(tokens, index + 1), b"document").is_some()
        {
            break;
        }
        if matches!(
            tokens.get(index),
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"index"
        ) {
            index = consume_ok_group_fragment_discard_v0(tokens, index + 1, tokens.len(), &mut title_state)?;
            continue;
        }
        if matches!(
            tokens.get(index),
            Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"printindex"
        ) {
            index += 1;
            continue;
        }
        index = consume_ok_body_token_v0(
            tokens,
            index,
            tokens.len(),
            false,
            &mut title_state,
            &mut list_env,
            &mut body,
            &mut previous_was_space,
        )?;
    }
    if list_env.is_some() {
        return None;
    }

    if !matches!(
        tokens.get(index),
        Some(TokenV0::ControlSeq(name)) if name.as_slice() == b"end"
    ) {
        return None;
    }
    index += 1;
    index = skip_spaces(tokens, index);
    index = consume_group_literal(tokens, index, b"document")?;
    index = skip_spaces(tokens, index);
    if index != tokens.len() {
        return None;
    }
    Some(body)
}
