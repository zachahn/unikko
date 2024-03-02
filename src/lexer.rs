use std::collections::VecDeque;
use std::io;

use crate::modifiers::{extract_block, BlockTag};

#[derive(Debug, PartialEq)]
pub enum Token {
    // Phase 1
    NewLine,
    Eof,
    Unparsed(String),

    // Phase 2
    BlockStart(Option<BlockTag>),
    BlockEnd,
}

fn tokenize_lines(input: &mut dyn io::BufRead) -> Result<VecDeque<Token>, crate::Error> {
    let mut tokens = VecDeque::<Token>::new();
    let mut line = String::new();

    loop {
        match input.read_line(&mut line) {
            Err(_) => return Err(crate::Error::LexerError),
            Ok(0) => {
                tokens.push_back(Token::Eof);
                break;
            }
            Ok(_) => {
                match line.strip_suffix("\n") {
                    Some(stripped) => {
                        if stripped != "" {
                            tokens.push_back(Token::Unparsed(stripped.to_string()));
                        }
                        tokens.push_back(Token::NewLine);
                    }
                    None => {
                        if !line.is_empty() {
                            tokens.push_back(Token::Unparsed(line.clone()));
                        }
                    }
                };
            }
        }
        line.clear();
    }

    Ok(tokens)
}

#[derive(Debug)]
struct CurrentBlockProcessor {
    backing: VecDeque<Token>,
}

impl CurrentBlockProcessor {
    pub fn new() -> Self {
        CurrentBlockProcessor {
            backing: VecDeque::<Token>::new(),
        }
    }

    pub fn push_back(&mut self, token: Token) {
        if self.backing.is_empty() {
            println!("🤔 but why male models? {:?}", token);
        }
        self.backing.push_back(token);
    }

    pub fn back(&self) -> Option<&Token> {
        self.backing.back()
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.backing.pop_front()
    }
}

fn tokenize_blocks(mut input: VecDeque<Token>) -> Result<VecDeque<Token>, crate::Error> {
    let mut result = VecDeque::<Token>::new();
    let mut current_block = CurrentBlockProcessor::new();

    loop {
        match input.pop_front() {
            None => break,
            Some(current) => match current {
                Token::Unparsed(text) => current_block.push_back(Token::Unparsed(text)),
                Token::Eof => {
                    let count = shove_block_into_result(&mut result, &mut current_block);
                    if count > 0 {
                        result.push_back(Token::BlockEnd);
                    }
                    result.push_back(current);
                    break;
                }
                Token::NewLine => match current_block.back() {
                    Some(Token::NewLine) => {
                        let count = shove_block_into_result(&mut result, &mut current_block);
                        result.push_back(current);
                        if count > 0 {
                            result.push_back(Token::BlockEnd);
                        }
                    }
                    Some(_) => {
                        current_block.push_back(current);
                    }
                    None => {
                        result.push_back(current);
                    }
                },
                // `tokenize_lines` only returns one of three kinds of Tokens
                _ => unreachable!(),
            },
        }
    }

    return Ok(result);
}

fn shove_block_into_result(
    result: &mut VecDeque<Token>,
    current_block: &mut CurrentBlockProcessor,
) -> usize {
    println!("🚽🚽🚽 PLUNGING!");
    println!("🔎 {:?}", current_block);

    let mut count = 0;

    while let Some(wip_block) = current_block.pop_front() {
        count += 1;
        match wip_block {
            Token::NewLine => result.push_back(wip_block),
            Token::Unparsed(line) => {
                if count == 1 {
                    let (block, inner) = extract_block(line);
                    result.push_back(Token::BlockStart(block));
                    result.push_back(Token::Unparsed(inner));
                } else {
                    result.push_back(Token::Unparsed(line));
                }
            }
            _ => unreachable!(),
        }
    }

    count
}

pub fn tokenize(input: &mut dyn io::BufRead) -> Result<VecDeque<Token>, crate::Error> {
    let result1 = tokenize_lines(input)?;
    let result2 = tokenize_blocks(result1)?;

    return Ok(result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_eol() {
        let mut input = io::Cursor::new("orange".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart(None),
                Token::Unparsed("orange".to_string()),
                Token::BlockEnd,
                Token::Eof
            )
        );
    }

    #[test]
    fn with_eol() {
        let mut input = io::Cursor::new("orange\n".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart(None),
                Token::Unparsed("orange".to_string()),
                Token::NewLine,
                Token::BlockEnd,
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
                Token::BlockStart(None),
                Token::Unparsed("hello 😁".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::BlockEnd,
                Token::BlockStart(None),
                Token::Unparsed("yay".to_string()),
                Token::BlockEnd,
                Token::Eof
            )
        );
    }

    #[test]
    fn linebreaks() {
        let mut input = io::Cursor::new("orange\nmocha\n".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart(None),
                Token::Unparsed("orange".to_string()),
                Token::NewLine,
                Token::Unparsed("mocha".to_string()),
                Token::NewLine,
                Token::BlockEnd,
                Token::Eof
            )
        );
    }

    #[test]
    fn block_tags() {
        let mut input = io::Cursor::new("h1.  orange\n\nmocha. frappuccino\n");
        let tokens = tokenize(&mut input).unwrap();
        let bt = BlockTag::new("h1".to_string(), false, Vec::new(), None, None, None);
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart(Some(bt)),
                Token::Unparsed(" orange".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::BlockEnd,
                Token::BlockStart(None),
                Token::Unparsed("mocha. frappuccino".to_string()),
                Token::NewLine,
                Token::BlockEnd,
                Token::Eof
            )
        );
    }

    #[test]
    fn empty_doc() {
        let mut input = io::Cursor::new("");
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(tokens, vec!(Token::Eof));
    }

    #[test]
    fn newlines_only_doc() {
        let mut input = io::Cursor::new("\n\n\n");
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(Token::NewLine, Token::NewLine, Token::NewLine, Token::Eof)
        );
    }
}
