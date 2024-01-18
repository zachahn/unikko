use crate::lexer::LexerToken;

#[derive(Debug, PartialEq)]
pub enum Node {
    NewLine,
    Eof,
    Document(Vec<Node>),
    Text(String),
    P(Vec<Node>),
    H1(Vec<Node>),
    H2(Vec<Node>),
    H3(Vec<Node>),
    H4(Vec<Node>),
    H5(Vec<Node>),
    H6(Vec<Node>),
    Pre(String),
    BlockCode(String),
    BlockQuote(Vec<Node>),
    NoTextile(String),
    Html(String),
}

impl Node {
    fn push_node(&mut self, node: Node) {
        match *self {
            Self::P(ref mut nodes)
            | Self::BlockQuote(ref mut nodes)
            | Self::H1(ref mut nodes)
            | Self::H2(ref mut nodes)
            | Self::H3(ref mut nodes)
            | Self::H4(ref mut nodes)
            | Self::H5(ref mut nodes)
            | Self::H6(ref mut nodes)
            | Self::Document(ref mut nodes) => nodes.push(node),
            _ => unreachable!(),
        }
    }
}

pub fn parse(lexer_tokens: Vec<LexerToken>) -> Result<Node, crate::Error> {
    let root = Node::Document(Vec::new());
    let mut stack = vec![root];

    for lexer_token in lexer_tokens.iter() {
        match lexer_token {
            LexerToken::Line(string) => {}
            LexerToken::NewLine => {
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::NewLine);
                }
            }
            LexerToken::Eof => {
                if let Some(last) = stack.last_mut() {
                    last.push_node(Node::Eof);
                }
            }
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
    fn something_works() {
        let input = vec![
            LexerToken::Line("h1. hello 😁".to_string()),
            LexerToken::NewLine,
            LexerToken::NewLine,
            LexerToken::Line("yay".to_string()),
            LexerToken::Eof,
        ];
        let nodes = parse(input).unwrap();
        assert_eq!(
            nodes,
            Node::Document(vec!(
                Node::H1(vec!(Node::Text("hello 😁".to_string()))),
                Node::NewLine,
                Node::NewLine,
                Node::Text("yay".to_string()),
                Node::Eof,
            ))
        );
    }
}
