use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Attributes {
    unparsed: String,
}

#[derive(Debug, PartialEq)]
pub struct Document {
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub identifier: String,
    attrs: Attributes,
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Plain {
    identifier: String,
    pub content: String,
}

trait HasNodes {
    fn push_node(&mut self, node: Node);
    fn set_identifier(&mut self, identifier: String) -> Result<(), crate::Error>;
}

impl HasNodes for Document {
    fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn set_identifier(&mut self, _identifier: String) -> Result<(), crate::Error> {
        Err(crate::Error::ParserError)
    }
}

impl HasNodes for Element {
    fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn set_identifier(&mut self, identifier: String) -> Result<(), crate::Error> {
        self.identifier = identifier;
        Ok(())
    }
}

impl Attributes {
    fn new() -> Self {
        Self {
            unparsed: String::new(),
        }
    }
}

impl Document {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes: nodes }
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }
}

impl Element {
    pub fn new(identifier: impl Into<String>, attrs: Attributes, nodes: Vec<Node>) -> Self {
        Self {
            identifier: identifier.into(),
            attrs: attrs,
            nodes: nodes,
        }
    }

    pub fn empty(identifier: impl Into<String>, attrs: Attributes) -> Self {
        Self::new(identifier, attrs, vec![])
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

    Document(Document),
    Element(Element),
    Plain(Plain),
}

fn recursively_parse(
    lexer_tokens: &mut VecDeque<Token>,
    parent: &mut dyn HasNodes,
) -> Result<(), crate::Error> {
    while let Some(lexer_token) = lexer_tokens.pop_front() {
        match lexer_token {
            Token::BlockStart => {
                let mut block = Element::empty("p", Attributes::new());
                recursively_parse(lexer_tokens, &mut block)?;
                parent.push_node(Node::Element(block));
            }
            Token::BlockEnd => {
                return Ok(());
            }
            Token::SignatureStart(identifier) => {
                parent.set_identifier(identifier)?;
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
            _ => todo!("{:?} (by parser)", lexer_token),
        }
    }
    Ok(())
}

pub fn parse(lexer_tokens: Vec<Token>) -> Result<Node, crate::Error> {
    let mut root = Document::empty();
    recursively_parse(&mut VecDeque::from(lexer_tokens), &mut root)?;
    return Ok(Node::Document(root));
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
        let input = crate::tokenize(&mut input)?;
        let nodes = parse(input)?;
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

    fn doc(nodes: Vec<Node>) -> Node {
        Node::Document(Document::new(nodes))
    }

    fn text(content: &str) -> Node {
        Node::Plain(Plain::new("text", content))
    }

    fn h1(attrs: A, nodes: Vec<Node>) -> Node {
        Node::Element(Element::new("h1", attrs, nodes))
    }

    fn p(attrs: A, nodes: Vec<Node>) -> Node {
        Node::Element(Element::new("p", attrs, nodes))
    }
}
