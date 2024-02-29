use regex::Regex;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, PartialEq)]
pub enum Token {
    NewLine,
    Eof,

    BlockTag(String),
    Text(String),
    HtmlTag(String),
}

pub fn tokenize_1(input: &mut dyn io::BufRead) -> Result<VecDeque<Token>, crate::UnikkoError> {
    let mut tokens = VecDeque::<Token>::new();
    let mut line = String::new();

    loop {
        match input.read_line(&mut line) {
            Err(_) => return Err(crate::UnikkoError::LexerError),
            Ok(0) => {
                tokens.push_back(Token::Eof);
                break;
            }
            Ok(_) => {
                match line.strip_suffix("\n") {
                    Some(stripped) => {
                        if stripped != "" {
                            tokens.push_back(Token::Text(stripped.to_string()));
                        }
                        tokens.push_back(Token::NewLine);
                    }
                    None => {
                        if line != "" {
                            tokens.push_back(Token::Text(line.clone()));
                        }
                    }
                };
            }
        }
        line.clear();
    }

    Ok(tokens)
}

pub fn tokenize_2(mut input: VecDeque<Token>) -> Result<VecDeque<Token>, crate::UnikkoError> {
    let mut result = VecDeque::<Token>::new();

    let block =
        Regex::new(r"^(?:(?<block_tag>(p|h[1-6]|pre|bc|bq|###|notextile)+.)\s)?(?<rest>.*)$")
            .unwrap();

    loop {
        match input.pop_front() {
            None => break,
            Some(current) => match current {
                Token::Eof => result.push_back(current),
                Token::NewLine => result.push_back(current),
                Token::Text(text) => {
                    let captures = block.captures(text.as_str()).unwrap();
                    let block_tag = captures.name("block_tag").map_or("", |m| m.as_str());
                    let rest = captures.name("rest").map_or("", |m| m.as_str());
                    if block_tag.len() > 0 {
                        result.push_back(Token::BlockTag(block_tag.to_string()))
                    }
                    result.push_back(Token::Text(rest.to_string()))
                }
                // `tokenize_1` only returns one of three kinds of Tokens
                _ => unreachable!(),
            },
        }
    }

    return Ok(result);
}

pub fn tokenize(input: &mut dyn io::BufRead) -> Result<VecDeque<Token>, crate::UnikkoError> {
    let result1 = tokenize_1(input)?;
    let result2 = tokenize_2(result1)?;

    return Ok(result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_newline() {
        let mut input = io::Cursor::new("hello 😁".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(Token::Text("hello 😁".to_string()), Token::Eof)
        );
    }

    #[test]
    fn with_new_line() {
        let mut input = io::Cursor::new("hello 😁\n".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::Text("hello 😁".to_string()),
                Token::NewLine,
                Token::Eof
            )
        );
    }

    #[test]
    fn implicit_paragraphs() {
        let mut input = io::Cursor::new("hello 😁\n\nyay".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::Text("hello 😁".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Text("yay".to_string()),
                Token::Eof
            )
        );
    }

    #[test]
    fn block_tags() {
        let mut input = io::Cursor::new("h1.  orange\n\nmocha. frappuccino\n");
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::BlockTag("h1.".to_string()),
                Token::Text(" orange".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Text("mocha. frappuccino".to_string()),
                Token::NewLine,
                Token::Eof
            )
        );
    }
}
