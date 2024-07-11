use crate::options::Symbol;
use crate::Error;
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
    FootnoteRef,
}

impl TryFrom<&str> for Tag {
    type Error = Error;

    fn try_from(tag_string: &str) -> Result<Self, Error> {
        match tag_string {
            "p" => Ok(Tag::Paragraph),
            "h1" => Ok(Tag::H1),
            "h2" => Ok(Tag::H2),
            "h3" => Ok(Tag::H3),
            "h4" => Ok(Tag::H4),
            "h5" => Ok(Tag::H5),
            "h6" => Ok(Tag::H6),
            _ => Err(Error::IntoTagError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub classes: Vec<String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            href: None,
            id: None,
            classes: Vec::new(),
            style: None,
        }
    }

    pub fn href(href: String) -> Self {
        Self {
            href: Some(href),
            id: None,
            classes: Vec::new(),
            style: None,
        }
    }

    pub fn classes(classes: Vec<String>) -> Self {
        Self {
            href: None,
            id: None,
            classes: classes,
            style: None,
        }
    }

    pub fn classes_id(classes: Vec<String>, id: String) -> Self {
        Self {
            href: None,
            id: Some(id),
            classes: classes,
            style: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag: Tag,
    pub attrs: Attributes,
    pub nodes: Vec<Node>,
}

impl Element {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag: tag,
            attrs: Attributes::new(),
            nodes: vec![],
        }
    }

    pub fn attrs(tag: Tag, attrs: Attributes) -> Self {
        Self {
            tag: tag,
            attrs: attrs,
            nodes: vec![],
        }
    }

    pub fn nodes(tag: Tag, nodes: Vec<Node>) -> Self {
        Self {
            tag: tag,
            attrs: Attributes::new(),
            nodes: nodes,
        }
    }

    pub fn attrs_nodes(tag: Tag, attrs: Attributes, nodes: Vec<Node>) -> Self {
        Self {
            tag: tag,
            attrs: attrs,
            nodes: nodes,
        }
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

    Multiple(Vec<Node>),
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Node::Element(element)
    }
}

impl From<Plain> for Node {
    fn from(plain: Plain) -> Self {
        Node::Plain(plain)
    }
}

impl From<Symbol> for Node {
    fn from(symbol: Symbol) -> Self {
        Node::Symbol(symbol)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::NewLine => f.debug_struct("NewLine").finish(),
            Node::Element(element) => write!(f, "{:?}", element),
            Node::Plain(plain) => write!(f, "{:?}", plain),
            Node::Symbol(symbol) => write!(f, "{:?}", symbol),
            Node::Multiple(nodes) => {
                for node in nodes {
                    node.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}
