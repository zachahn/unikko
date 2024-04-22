use crate::Options;
use regex::Regex;
use std::collections::VecDeque;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Phase 1
    // First, we read the entire input. We'll mark almost everything "Unparsed", then chip away at the unparsed bits
    NewLine,
    Eof,
    Unparsed(String),

    // Phase 2
    // Lexers usually don't care about semantics, but it's helpful for the rest of the tokenizing
    // process.
    BlockStart,
    BlockEnd,

    // Phase 3
    // Actually handle blocks
    SignatureStart(String),
    SignatureEnd,
    Modifier(String),
    ModifierParenOpen,
    ModifierParenClose,
    ModifierSquareOpen,
    ModifierSquareClose,
    ModifierCurlyOpen,
    ModifierCurlyClose,

    // Final Phase
    Text(String),
}

fn tokenize_lines(input: &mut dyn BufRead) -> Result<VecDeque<Token>, crate::Error> {
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
                        close_block_with_newlines(&mut result);
                    }
                    result.push_back(current);
                    break;
                }
                Token::NewLine => match current_block.back() {
                    Some(Token::NewLine) => {
                        let count = shove_block_into_result(&mut result, &mut current_block);
                        if count > 0 {
                            close_block_with_newlines(&mut result);
                        }
                        result.push_back(current);
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
    let mut count = 0;

    while let Some(wip_block) = current_block.pop_front() {
        count += 1;
        match wip_block {
            Token::NewLine => result.push_back(wip_block),
            Token::Unparsed(line) => {
                if count == 1 {
                    result.push_back(Token::BlockStart);
                }
                result.push_back(Token::Unparsed(line));
            }
            _ => unreachable!(),
        }
    }

    count
}

fn close_block_with_newlines(result: &mut VecDeque<Token>) {
    let mut newlines_count = 0;
    loop {
        match result.back() {
            Some(Token::NewLine) => {
                result.pop_back();
                newlines_count += 1;
            }
            Some(_) | None => {
                break;
            }
        }
    }
    result.push_back(Token::BlockEnd);
    for _ in 0..newlines_count {
        result.push_back(Token::NewLine);
    }
}

fn tokenize_signatures(mut input: VecDeque<Token>) -> Result<VecDeque<Token>, crate::Error> {
    let mut result = VecDeque::<Token>::new();
    let mut is_first_line = false;

    loop {
        match input.pop_front() {
            None => break,
            Some(current) => match current {
                Token::NewLine | Token::Eof | Token::BlockEnd => {
                    if is_first_line {
                        return Err(crate::Error::LexerError);
                    }
                    result.push_back(current);
                }
                Token::BlockStart => {
                    if is_first_line {
                        return Err(crate::Error::LexerError);
                    }
                    is_first_line = true;
                    result.push_back(current);
                }
                Token::Unparsed(line) => {
                    let mut added = false;
                    if is_first_line {
                        let pattern = Regex::new(
                            "^(?<signature>p|h[1-6]|bq)(?<modifiers>[^.]*)\\. (?<inner>.*)$",
                        )
                        .unwrap();
                        match pattern.captures(&line) {
                            None => {}
                            Some(captures) => {
                                let mut buffer = VecDeque::<Token>::new();
                                buffer.push_back(Token::SignatureStart(
                                    captures["signature"].to_string(),
                                ));
                                let modifiers = &captures["modifiers"];
                                let mut start_of_str: Option<usize> = None;
                                for (i, c) in modifiers.char_indices() {
                                    match c {
                                        '(' | ')' | '[' | ']' | '{' | '}' => {
                                            if let Some(start) = start_of_str {
                                                let sub = &modifiers[start..i];
                                                buffer.push_back(Token::Modifier(sub.to_string()));
                                                start_of_str = None;
                                            }
                                            match c {
                                                '(' => buffer.push_back(Token::ModifierParenOpen),
                                                ')' => buffer.push_back(Token::ModifierParenClose),
                                                '[' => buffer.push_back(Token::ModifierSquareOpen),
                                                ']' => buffer.push_back(Token::ModifierSquareClose),
                                                '{' => buffer.push_back(Token::ModifierCurlyOpen),
                                                '}' => buffer.push_back(Token::ModifierCurlyClose),
                                                _ => unreachable!("{}", c),
                                            }
                                        }
                                        _ => {
                                            if start_of_str == None {
                                                start_of_str = Some(i)
                                            }
                                        }
                                    }
                                }
                                buffer.push_back(Token::SignatureEnd);
                                buffer.push_back(Token::Unparsed(captures["inner"].to_string()));
                                result.append(&mut buffer);
                                added = true;
                            }
                        }
                    }
                    is_first_line = false;
                    if added == false {
                        result.push_back(Token::Unparsed(line));
                    }
                }
                _ => unreachable!("{:?}", current),
            },
        }
    }

    Ok(result)
}

fn tokenize_text(mut input: VecDeque<Token>) -> Result<Vec<Token>, crate::Error> {
    let mut result = Vec::<Token>::new();

    loop {
        match input.pop_front() {
            None => break,
            Some(current) => match current {
                Token::Unparsed(text) => result.push(Token::Text(text)),
                _ => result.push(current),
            },
        }
    }

    Ok(result)
}

pub fn tokenize(input: &mut dyn BufRead, _options: &Options) -> Result<Vec<Token>, crate::Error> {
    tokenize_lines(input)
        .and_then(tokenize_blocks)
        .and_then(tokenize_signatures)
        .and_then(tokenize_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::Cursor;

    #[test]
    fn no_eol() -> Result<()> {
        let mut input = Cursor::new("orange".as_bytes());
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::Text("orange".to_string()),
                Token::BlockEnd,
                Token::Eof
            )
        );
        Ok(())
    }

    #[test]
    fn with_eol() -> Result<()> {
        let mut input = Cursor::new("orange\n".as_bytes());
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::Text("orange".to_string()),
                Token::BlockEnd,
                Token::NewLine,
                Token::Eof
            )
        );
        Ok(())
    }

    #[test]
    fn implicit_paragraphs() -> Result<()> {
        let mut input = Cursor::new("hello 😁\n\nyay".as_bytes());
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::Text("hello 😁".to_string()),
                Token::BlockEnd,
                Token::NewLine,
                Token::NewLine,
                Token::BlockStart,
                Token::Text("yay".to_string()),
                Token::BlockEnd,
                Token::Eof
            )
        );
        Ok(())
    }

    #[test]
    fn linebreaks() -> Result<()> {
        let mut input = Cursor::new("orange\nmocha\n".as_bytes());
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::Text("orange".to_string()),
                Token::NewLine,
                Token::Text("mocha".to_string()),
                Token::BlockEnd,
                Token::NewLine,
                Token::Eof
            )
        );
        Ok(())
    }

    #[test]
    fn block_tags() -> Result<()> {
        let mut input = Cursor::new("h1.  orange\n\nmocha. frappuccino\n");
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::SignatureStart("h1".to_string()),
                Token::SignatureEnd,
                Token::Text(" orange".to_string()),
                Token::BlockEnd,
                Token::NewLine,
                Token::NewLine,
                Token::BlockStart,
                Token::Text("mocha. frappuccino".to_string()),
                Token::BlockEnd,
                Token::NewLine,
                Token::Eof
            )
        );
        Ok(())
    }

    #[test]
    fn empty_doc() -> Result<()> {
        let mut input = Cursor::new("");
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(tokens, vec!(Token::Eof));
        Ok(())
    }

    #[test]
    fn newlines_only_doc() -> Result<()> {
        let mut input = Cursor::new("\n\n\n");
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(Token::NewLine, Token::NewLine, Token::NewLine, Token::Eof)
        );
        Ok(())
    }

    #[test]
    fn modifiers() -> Result<()> {
        let mut input = Cursor::new("h1(so-hot). hansel");
        let tokens = tokenize(&mut input, &Options::default())?;
        assert_eq!(
            tokens,
            vec!(
                Token::BlockStart,
                Token::SignatureStart("h1".to_string()),
                Token::ModifierParenOpen,
                Token::Modifier("so-hot".to_string()),
                Token::ModifierParenClose,
                Token::SignatureEnd,
                Token::Text("hansel".to_string()),
                Token::BlockEnd,
                Token::Eof
            )
        );
        Ok(())
    }
}
