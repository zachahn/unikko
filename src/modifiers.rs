use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct BlockTag {
    pub signature: Option<String>,
    pub extended: bool,
    pub modifiers: Vec<String>,
    pub css: Option<String>,
    pub lang: Option<String>,
    pub selector: Option<String>,
}

impl BlockTag {
    pub fn new(
        signature: Option<impl Into<String>>,
        extended: bool,
        modifiers: Vec<String>,
        css: Option<String>,
        lang: Option<String>,
        selector: Option<String>,
    ) -> Self {
        Self {
            signature: signature.and_then(|s| Some(s.into())),
            extended: extended,
            modifiers: modifiers,
            css: css,
            lang: lang,
            selector: selector,
        }
    }

    fn blank() -> Self {
        let sig: Option<String> = None;
        Self::new(sig, false, Vec::new(), None, None, None)
    }
}

pub fn extract_block(line: String) -> (Option<BlockTag>, String) {
    let start_pattern = Regex::new(
        r"(?x)                      # Enable extended mode to add these comments
        ^(?<signature>p|h[1-6]|pre|bc|bq|\#\#\#|notextile)
                                    # Check if first char is the start of a signature (above)
        \.                          # Finally we require a dot
        \                           # And we will capture exactly one space
        (?<inner>.*)                # Everything else is stuff that belongs inside
        $
        ",
    )
    .unwrap();
    let captures = start_pattern.captures(line.as_str());

    match captures {
        None => (None, line),
        Some(captures) => {
            let mut block_tag = BlockTag::blank();
            if let Some(signature) = captures.name("signature") {
                block_tag.signature = Some(signature.as_str().to_string());
            }

            (
                Some(block_tag),
                captures.name("inner").unwrap().as_str().to_string(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_tag(signature: &str) -> BlockTag {
        BlockTag::new(
            Some(signature.to_string()),
            false,
            Vec::new(),
            None,
            None,
            None,
        )
    }

    #[test]
    fn invalid() {
        let result = extract_block("p.".to_string());
        assert_eq!(result, (None, "p.".to_string()));
    }

    #[test]
    fn signature() {
        let (block, text) = extract_block("p.  orange".to_string());
        assert_eq!(block.unwrap(), block_tag("p"));
        assert_eq!(text, " orange");
    }
}
