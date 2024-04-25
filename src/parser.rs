use crate::lexer::Token;
use crate::Options;
use std::collections::VecDeque;
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
    Other(String),
}

impl From<String> for Tag {
    fn from(tag_string: String) -> Self {
        match tag_string.to_lowercase().as_str() {
            "doc" => Tag::Doc,
            "p" => Tag::P,
            "h1" => Tag::H1,
            "h2" => Tag::H2,
            "h3" => Tag::H3,
            "h4" => Tag::H4,
            "h5" => Tag::H5,
            "h6" => Tag::H6,
            "bq" => Tag::Bq,
            _ => Tag::Other(tag_string),
        }
    }
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
            Tag::Other(desc) => desc,
        };
        write!(f, "{}", txt)
    }
}

#[derive(Debug, PartialEq)]
pub struct Attributes {
    unparsed: String,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub identifier: Tag,
    attrs: Attributes,
    pub nodes: Vec<Node>,
    pub extended: bool,
}

#[derive(Debug, PartialEq)]
pub struct Plain {
    identifier: String,
    pub content: String,
}

impl Attributes {
    fn new() -> Self {
        Self {
            unparsed: String::new(),
        }
    }
}

impl Element {
    pub fn new(identifier: Tag, attrs: Attributes, nodes: Vec<Node>) -> Self {
        Self {
            identifier: identifier,
            attrs: attrs,
            nodes: nodes,
            extended: false,
        }
    }

    pub fn empty(identifier: Tag, attrs: Attributes) -> Self {
        Self::new(identifier, attrs, vec![])
    }

    fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn set_identifier(&mut self, identifier: Tag) -> Result<(), crate::Error> {
        self.identifier = identifier;
        Ok(())
    }
}

impl Plain {
    pub fn new(identifier: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    NewLine,

    Element(Element),
    Plain(Plain),
}

struct PreparseContext {
    tag: Option<String>,
    extended: bool,
}

/// ## Handle blockquotes
///
/// 1. Wrap each implicit block inside a `bq` as a paragraph.
/// 2. End on explicit bq
fn preparse(mut lexer_tokens: VecDeque<Token>) -> VecDeque<Token> {
    let mut result = VecDeque::<Token>::new();
    let mut context = Vec::<PreparseContext>::new();
    let mut placeholders = Vec::<usize>::new();

    while let Some(token) = lexer_tokens.pop_front() {
        match token {
            Token::BlockStart => {
                if let Some(current_ctx) = context.last() {
                    if let Some(ref tag) = current_ctx.tag {
                        if tag.as_str() == "bq" && current_ctx.extended {
                            placeholders.push(context.len());
                            result.push_back(Token::NoOpPlaceholder);
                        }
                    }
                }
                context.push(PreparseContext {
                    tag: None,
                    extended: false,
                });
                result.push_back(token);
            }
            Token::BlockEnd => {
                let ended_context = context.pop().unwrap();
                if ended_context.tag == None {
                    if let Some(outer_ctx) = context.last() {
                        if let Some(ref tag) = outer_ctx.tag {
                            if tag.as_str() == "bq" {
                                if !outer_ctx.extended {
                                    result.push_back(Token::BlockEnd);
                                    context.pop().unwrap();
                                }
                            }
                        }
                    }
                }
                result.push_back(token);
            }
            Token::SignatureStart(ref name) => {
                let current = context.last_mut().unwrap();
                current.tag = Some(name.to_string());
                result.push_back(token);
            }
            Token::SignatureEnd => {
                result.push_back(token);
                let current = context.last_mut().unwrap();
                if let Some(tag) = &current.tag {
                    if tag.as_str() == "bq" {
                        // Insert implicit paragraph
                        result.push_back(Token::BlockStart);
                        context.push(PreparseContext {
                            tag: None,
                            extended: false,
                        });
                    }
                }
            }
            Token::ModifierExtended => {
                let current = context.last_mut().unwrap();
                if let Some(tag) = &current.tag {
                    if tag.as_str() == "bq" {
                        current.extended = true;
                    }
                }
            }
            _ => result.push_back(token),
        };
    }

    return result;
}

/// Document trees are very shallow and often have a height of 1 (root -> p, root -> h1). There are
/// some exceptions I'm aware of:
///
/// * `root -> bq -> p`
///    * paragraphs are implicit, and they cannot have any modifiers. this is true for extended and
///      non-extended blockquotes.
/// * `root -> table -> ...`
/// * Inline elements, though we can largely ignore that scenario here
/// * `root -> bq -> p -> table` is not valid since paragraph tags cannot contain other block
///   elements.
///
/// And as mentioned, we have to consider extended blocks. As far as I'm aware, the only _valid_
/// and defined extended blocks are the following:
///
/// * `bq`
/// * `bc`
///
/// This gives us a few permutations to work through:
///
/// * "regular" blocks
/// * non-extended bc
/// * non-extended bq
/// * extended bc
/// * extended bq
/// * tables
fn recursively_parse(
    lexer_tokens: &mut VecDeque<Token>,
    parent: &mut Element,
) -> Result<(), crate::Error> {
    while let Some(lexer_token) = lexer_tokens.pop_front() {
        match lexer_token {
            Token::BlockStart => {
                let mut block = Element::empty(Tag::P, Attributes::new());
                recursively_parse(lexer_tokens, &mut block)?;
                parent.push_node(Node::Element(block));
            }
            Token::BlockEnd => {
                return Ok(());
            }
            Token::SignatureStart(identifier) => {
                let identifier: Tag = identifier.into();
                match identifier {
                    Tag::Bq => {
                        parent.set_identifier(identifier)?;
                    }
                    _ => parent.set_identifier(identifier)?,
                };
            }
            Token::SignatureEnd => {}
            Token::NewLine => parent.push_node(Node::NewLine),
            Token::Text(text) => parent.push_node(Node::Plain(Plain::new("text", text))),
            Token::Eof => {
                if !lexer_tokens.is_empty() {
                    return Err(crate::Error::ParserError);
                }
                return Ok(());
            }
            Token::ModifierCurlyOpen => {}
            Token::ModifierCurlyClose => {}
            Token::ModifierParenOpen => {}
            Token::ModifierParenClose => {}
            Token::ModifierSquareOpen => {}
            Token::ModifierSquareClose => {}
            Token::Modifier(_) => {}
            Token::ModifierExtended => {
                parent.extended = true;
            }
            Token::NoOpPlaceholder => {}
            _ => todo!("{:?} (by parser)", lexer_token),
        }
    }
    Ok(())
}

pub fn parse(lexer_tokens: VecDeque<Token>, _options: &Options) -> Result<Node, crate::Error> {
    let mut doc = Element::empty(Tag::Doc, Attributes::new());
    let mut lexer_tokens = preparse(lexer_tokens);
    recursively_parse(&mut lexer_tokens, &mut doc)?;
    return Ok(Node::Element(doc));
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::Cursor;
    use Attributes as A;

    #[test]
    fn blocks() -> Result<()> {
        let mut input = Cursor::new("h1. they're\n\nin the computer");
        let options = Options::default();
        let input = crate::tokenize(&mut input, &options)?;
        let nodes = parse(input, &options)?;
        assert_eq!(
            nodes,
            doc(vec!(
                h1(A::new(), vec!(text("they're"))),
                Node::NewLine,
                Node::NewLine,
                p(A::new(), vec!(text("in the computer"))),
            ))
        );
        Ok(())
    }

    #[test]
    fn blockquote_implicit_p() -> Result<()> {
        let mut input = Cursor::new("bq. they're in the computer\n\nthey're in the computer?");
        let options = Options::default();
        let input = crate::tokenize(&mut input, &options)?;
        let nodes = parse(input, &options)?;
        assert_eq!(
            nodes,
            doc(vec!(
                bq(
                    A::new(),
                    vec!(p(A::new(), vec!(text("they're in the computer"))),)
                ),
                n(),
                n(),
                p(A::new(), vec!(text("they're in the computer?")))
            ))
        );
        Ok(())
    }

    #[test]
    fn blockquote_extended_explicit_end() -> Result<()> {
        let mut input = Cursor::new("bq.. they're in the computer\n\nthey're in the computer?\n\np. what do they look like?");
        let options = Options::default();
        let input = crate::tokenize(&mut input, &options)?;
        let nodes = parse(input, &options)?;
        assert_eq!(
            nodes,
            doc(vec!(
                bq(
                    A::new(),
                    vec!(
                        p(A::new(), vec!(text("they're in the computer"))),
                        n(),
                        n(),
                        p(A::new(), vec!(text("they're in the computer?"))),
                    )
                ),
                n(),
                n(),
                p(A::new(), vec!(text("what do they look like?"))),
            ))
        );
        Ok(())
    }

    fn doc(nodes: Vec<Node>) -> Node {
        Node::Element(Element::new(Tag::Doc, A::new(), nodes))
    }

    fn text(content: &str) -> Node {
        Node::Plain(Plain::new("text", content))
    }

    fn n() -> Node {
        Node::NewLine
    }

    fn h1(attrs: A, nodes: Vec<Node>) -> Node {
        Node::Element(Element::new(Tag::H1, attrs, nodes))
    }

    fn p(attrs: A, nodes: Vec<Node>) -> Node {
        Node::Element(Element::new(Tag::P, attrs, nodes))
    }

    fn bq(attrs: A, nodes: Vec<Node>) -> Node {
        Node::Element(Element::new(Tag::Bq, attrs, nodes))
    }
}
