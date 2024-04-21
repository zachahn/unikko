use crate::lexer::Token;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Attributes {
    unparsed: String,
}

#[derive(Debug, PartialEq)]
pub struct Document {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    identifier: String,
    attrs: Attributes,
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Plain {
    identifier: String,
    content: String,
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

impl Node {
    fn push_node(&mut self, node: Node) {
        match self {
            Self::Document(n) => n.nodes.push(node),
            Self::Element(n) => n.nodes.push(node),
            _ => unreachable!("{:?}", self),
        }
    }
}

fn insert_implicit_element(
    stack: &Vec<&mut Node>,
    context: &VecDeque<Token>,
) -> Result<(), crate::Error> {
    match context.get(0) {
        Some(Token::BlockStart) => {}
        _ => Err(crate::Error::ParserError)?,
    }

    // if Some(&Token::SignatureStart) == context.get(1) {
    //     return Ok(());
    // }

    // stack.push_back
    Ok(())
}

pub fn parse(lexer_tokens: Vec<Token>) -> Result<Node, crate::Error> {
    let mut stack = Vec::<&mut Node>::new();
    let mut context = VecDeque::<Token>::new();
    let mut root = Node::Document(Document::empty());
    stack.push(&mut root);
    println!("{:?}", lexer_tokens);

    for lexer_token in lexer_tokens {
        match lexer_token {
            Token::NewLine => {
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::Plain(Plain::new("newline", "\n")))
                }
            }
            Token::BlockStart => context.push_back(lexer_token),
            Token::BlockEnd => {
                println!("Popping stack");
                stack.pop().ok_or(crate::Error::ParserError)?;
            }
            Token::SignatureStart(identifier) => {
                println!("signaturestart");
                match context.back() {
                    Some(Token::BlockStart) => {
                        context.push_back(Token::SignatureStart(identifier));
                    }
                    _ => Err(crate::Error::ParserError)?,
                }
            }
            Token::SignatureEnd => {
                match context.pop_front() {
                    Some(Token::BlockStart) => {}
                    _ => Err(crate::Error::ParserError)?,
                }
                println!("signatureend - text");
                match context.pop_front() {
                    Some(Token::SignatureStart(identifier)) => {
                        let mut element = Node::Element(Element::empty(identifier, Attributes::new()));
                        stack.push(&mut element);
                        if let Some(last) = stack.last_mut() {
                            last.push_node(element);
                        }
                    }
                    // _ => Err(crate::Error::ParserError)?
                    x => todo!("{:?}", x),
                }
            }
            Token::Text(text) => {
                insert_implicit_element(&stack, &context)?;
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::Plain(Plain::new("text", text)))
                }
            }
            Token::Eof => {}
            _ => todo!("{:?}", lexer_token),
        }
    }

    if stack.len() == 1 {
        Ok(root)
    } else {
        Err(crate::Error::ParserError)
    }
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
