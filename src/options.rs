use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Symbol {
    Apostrophe,
    Trademark,
    Registered,
    Copyright,
    Half,
    Quarter,
    ThreeQuarters,
    Degrees,
    PlusMinus,
}

enum Doctype {
    Html5,
    Xhtml,
}

pub struct Options {
    // setDocumentType
    document_type: Doctype,
    // setDocumentRootDirectory
    document_root_directory: Option<std::path::PathBuf>,
    // // setLite
    // lite_mode: bool,
    // // setImages
    // handle_images: bool,
    // // setLinkRelationShip
    // link_rel: Option<String>
    // // setRestricted
    // restricted_mode: bool
    // // setRawBlocks
    // setRawBlocks: bool, // defaults to false
    // // setAlignClasses
    // setAlignClasses: bool, // false
    // // setBlockTags
    // setBlockTags: bool, // true
    // // setLineWrap
    // setLineWrap: bool, // true
    // // setSymbol
    // setSymbol: (String, Option<String>), //
    pub symbols: HashMap<Symbol, String>,
    // // setImagePrefix
    // setImagePrefix: Option<String>, // none
    // // setLinkPrefix
    // setLinkPrefix: Option<String>,
    // // setRelativeImagePrefix
    // setRelativeImagePrefix: Option<String>, // deprecated
    // // setDimensionlessImages
    // setDimensionlessImages: bool, // false
}

impl Options {
    pub fn default() -> Self {
        Self {
            document_type: Doctype::Xhtml,
            document_root_directory: std::env::current_dir().ok(),
            symbols: Self::canonical_symbols(),
            // lite_mode: false,
            // handle_images: true,
            // link_rel: None,
            // restricted_mode: false,
        }
    }

    fn canonical_symbols() -> HashMap<Symbol, String> {
        let pairs = [
            // ("quote_single_open", "&#8216;"),
            // ("quote_single_close", "&#8217;"),
            // ("quote_double_open", "&#8220;"),
            // ("quote_double_close", "&#8221;"),
            (Symbol::Apostrophe, "&#8217;"),
            // ("prime", "&#8242;"),
            // ("prime_double", "&#8243;"),
            // ("ellipsis", "&#8230;"),
            // ("emdash", "&#8212;"),
            // ("endash", "&#8211;"),
            // ("dimension", "&#215;"),
            (Symbol::Trademark, "&#8482;"),
            (Symbol::Registered, "&#174;"),
            (Symbol::Copyright, "&#169;"),
            (Symbol::Half, "&#189;"),
            (Symbol::Quarter, "&#188;"),
            (Symbol::ThreeQuarters, "&#190;"),
            (Symbol::Degrees, "&#176;"),
            (Symbol::PlusMinus, "&#177;"),
            // ("fn_ref_pattern", "<sup{atts}>{marker}</sup>"),
            // ("fn_foot_pattern", "<sup{atts}>{marker}</sup>"),
            // ("nl_ref_pattern", "<sup{atts}>{marker}</sup>"),
            // ("caps", "<span class=\"caps\">{content}</span>"),
            // ("acronym", None),
        ];
        return HashMap::from(pairs.map(|(key, value)| (key, value.to_string())));
    }
}
