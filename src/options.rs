enum Doctype {
    Html5,
    Xhtml,
}

struct Options {
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
    fn new_canonical() -> Self {
        Self {
            document_type: Doctype::Xhtml,
            document_root_directory: std::env::current_dir().ok(),
            // lite_mode: false,
            // handle_images: true,
            // link_rel: None,
            // restricted_mode: false,
        }
    }
}
