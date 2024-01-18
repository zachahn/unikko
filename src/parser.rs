use crate::lexer::LexerToken;

#[derive(Debug, PartialEq)]
pub enum Token {
    Document,
    NewLine,
    Eof,
    Text(String),
    P(String),
    H1(String),
    H2(String),
    H3(String),
    H4(String),
    H5(String),
    H6(String),
    Pre(String),
    BlockCode(String),
    BlockQuote(String),
    NoTextile(String),
    Html(String),
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    token: Token,
    children: Vec<Node>,
}

impl Node {
    pub fn new(token: Token) -> Self {
        Self {
            token: token,
            children: Vec::new(),
        }
    }

    // pub fn new(token: Token, children: Vec<Node>) {
    //     Self {
    //         token: token,
    //         children: children,
    //     }
    // }
}

pub fn parse(lexer_tokens: Vec<LexerToken>) -> Result<Node, crate::Error> {
    let root = Node::new(Token::Document);
    let mut stack = vec![root];

    for lexer_token in lexer_tokens.iter() {
        match lexer_token {
            LexerToken::Line(string) => {}
            LexerToken::NewLine => {
                if let Some(last) = stack.last_mut() {
                    last.children.push(Node::new(Token::NewLine));
                }
            }
            LexerToken::Eof => {
                if let Some(last) = stack.last_mut() {
                    last.children.push(Node::new(Token::Eof));
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
            Node {
                token: Token::Document,
                children: vec!(
                    Node::new(Token::H1("hello 😁".to_string())),
                    Node::new(Token::NewLine),
                    Node::new(Token::NewLine),
                    Node::new(Token::Text("yay".to_string())),
                    Node::new(Token::Eof),
                )
            },
        );
    }
}
