use crate::options::Symbol;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Doc,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Bq,
    Strong,
    B,
    A,
    Other(String),
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            Tag::Doc => "doc",
            Tag::P => "p",
            Tag::H1 => "h1",
            Tag::H2 => "h2",
            Tag::H3 => "h3",
            Tag::H4 => "h4",
            Tag::H5 => "h5",
            Tag::H6 => "h6",
            Tag::Bq => "bq",
            Tag::A => "a",
            Tag::B => "b",
            Tag::Strong => "strong",
            Tag::Other(desc) => desc,
        };
        write!(f, "{}", txt)
    }
}

impl From<&str> for Tag {
    fn from(tag_string: &str) -> Self {
        match tag_string {
            "doc" => Tag::Doc,
            "p" => Tag::P,
            "h1" => Tag::H1,
            "h2" => Tag::H2,
            "h3" => Tag::H3,
            "h4" => Tag::H4,
            "h5" => Tag::H5,
            "h6" => Tag::H6,
            "bq" => Tag::Bq,
            "a" => Tag::A,
            "b" => Tag::B,
            "strong" => Tag::Strong,
            _ => Tag::Other(tag_string.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub href: Option<String>,
}

impl Attributes {
    pub fn new() -> Self {
        Self { href: None }
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

#[derive(Debug, PartialEq)]
pub enum Node {
    NewLine,

    Element(Element),
    Plain(Plain),
    Symbol(Symbol),
}
