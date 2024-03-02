use crate::lexer::Token;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Attributes {
    unparsed: String,
}

impl Attributes {
    fn new() -> Self {
        Self {
            unparsed: String::new(),
        }
    }
}

use Attributes as A;

#[derive(Debug, PartialEq)]
pub enum Node {
    NewLine,
    Eof,

    Document(Vec<Node>),

    Text(String),
    NoTextile(String),
    Html(String),

    Pre(String, A),
    BlockCode(String, A),

    BlockQuote(Vec<Node>, A),
    H1(Vec<Node>, A),
    H2(Vec<Node>, A),
    H3(Vec<Node>, A),
    H4(Vec<Node>, A),
    H5(Vec<Node>, A),
    H6(Vec<Node>, A),
    P(Vec<Node>, A),
}

impl Node {
    fn push_node(&mut self, node: Node) {
        match self {
            Self::P(nodes, _)
            | Self::BlockQuote(nodes, _)
            | Self::H1(nodes, _)
            | Self::H2(nodes, _)
            | Self::H3(nodes, _)
            | Self::H4(nodes, _)
            | Self::H5(nodes, _)
            | Self::H6(nodes, _)
            | Self::Document(nodes) => nodes.push(node),
            _ => unreachable!(),
        }
    }
}

pub fn parse(lexer_tokens: Vec<Token>) -> Result<Node, crate::Error> {
    let mut stack = Vec::<Node>::new();
    let root = Node::Document(Vec::new());
    stack.push(root);

    let block = Regex::new(r"^(?:(?<block_tag>(h[1-6]|p)+.)\s)?(?<rest>\s*.*)$").unwrap();

    for lexer_token in lexer_tokens.iter() {
        match lexer_token {
            Token::Unparsed(string) => match block.captures(string) {
                Some(captures) => {
                    if let Some(last) = stack.last_mut() {
                        let block_tag = captures.name("block_tag").map_or("", |m| m.as_str());
                        let rest = &captures["rest"];
                        match block_tag {
                            "h1." => {
                                let mut node = Node::H1(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "h2." => {
                                let mut node = Node::H2(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "h3." => {
                                let mut node = Node::H3(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "h4." => {
                                let mut node = Node::H4(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "h5." => {
                                let mut node = Node::H5(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "h6." => {
                                let mut node = Node::H6(Vec::new(), A::new());
                                if rest.len() > 0 {
                                    node.push_node(Node::Text(rest.to_string()));
                                }
                                last.push_node(node);
                            }
                            "" => {
                                if rest.len() > 0 {
                                    last.push_node(Node::Text(rest.to_string()));
                                }
                            }
                            _ => {
                                println!("{:?}", block_tag);
                                unreachable!()
                            }
                        }
                    }
                }
                None => unreachable!(),
            },
            Token::BlockStart(block_tag) => match block_tag {
                None => {
                }
                Some(block_tag) => {
                }
            }
            Token::BlockEnd => {
            }
            Token::NewLine => {
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::NewLine);
                }
            }
            Token::Eof => {
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::Eof);
                }
            }
            _ => todo!("{:?}", lexer_token),
        }
    }

    match stack.pop() {
        Some(node) => {
            if stack.len() > 0 {
                Err(crate::Error::ParserError)
            } else {
                Ok(node)
            }
        }
        None => Err(crate::Error::ParserError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks() {
        let input = vec![
            Token::Unparsed("h1. hello 😁".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Unparsed("yay".to_string()),
            Token::Eof,
        ];
        let nodes = parse(input).unwrap();
        assert_eq!(
            nodes,
            Node::Document(vec!(
                Node::H1(vec!(Node::Text("hello 😁".to_string())), A::new()),
                Node::NewLine,
                Node::NewLine,
                Node::Text("yay".to_string()),
                Node::Eof,
            ))
        );
    }
}
