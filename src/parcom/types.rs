use crate::options::Symbol;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Doc,
    Paragraph,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Blockquote,
    Strong,
    Bold,
    Emphasis,
    Italic,
    Anchor,
    Span,
    Footnote,
    FootnoteId,
    FootnoteRefLink,
    FootnoteRefPlain,
    Other(String),
}

impl From<&str> for Tag {
    fn from(tag_string: &str) -> Self {
        match tag_string {
            "doc" => Tag::Doc,
            "p" => Tag::Paragraph,
            "h1" => Tag::H1,
            "h2" => Tag::H2,
            "h3" => Tag::H3,
            "h4" => Tag::H4,
            "h5" => Tag::H5,
            "h6" => Tag::H6,
            "bq" => Tag::Blockquote,
            "a" => Tag::Anchor,
            "b" => Tag::Bold,
            "strong" => Tag::Strong,
            "%" => Tag::Span,
            _ => Tag::Other(tag_string.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub classes: Vec<String>,
    pub href: Option<String>,
    pub id: Option<String>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            href: None,
            id: None,
            classes: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag: Tag,
    pub attrs: Attributes,
    pub nodes: Vec<Node>,
    pub extended: bool,
}

impl Element {
    pub fn init(tag: Tag, attrs: Attributes, nodes: Vec<Node>, extended: bool) -> Self {
        Self {
            tag: tag,
            attrs: attrs,
            nodes: nodes,
            extended: extended,
        }
    }

    pub fn new(tag: impl Into<Tag>, extended: bool) -> Self {
        Self::init(tag.into(), Attributes::new(), vec![], extended)
    }

    pub fn empty(tag: Tag) -> Self {
        Self::init(tag, Attributes::new(), vec![], false)
    }
}

#[derive(Debug, PartialEq)]
pub struct Plain {
    pub content: String,
}

impl Plain {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[derive(PartialEq)]
pub enum Node {
    NewLine,

    Element(Element),
    Plain(Plain),
    Symbol(Symbol),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::NewLine => f.debug_struct("NewLine").finish(),
            Node::Element(element) => write!(f, "{:?}", element),
            Node::Plain(plain) => write!(f, "{:?}", plain),
            Node::Symbol(symbol) => write!(f, "{:?}", symbol),
        }
    }
}
