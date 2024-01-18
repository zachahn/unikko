use std::io;

#[derive(Debug, PartialEq)]
pub enum Token {
    Line(String),
    NewLine,
    Eof,
}

pub fn tokenize(input: &mut dyn io::BufRead) -> Result<Vec<Token>, io::Error> {
    let mut tokens = Vec::<Token>::new();
    let mut line = String::new();

    loop {
        match input.read_line(&mut line) {
            Ok(num) => {
                if num == 0 {
                    tokens.push(Token::Eof);
                    break;
                }
                match line.strip_suffix("\n") {
                    Some(stripped) => {
                        if stripped != "" {
                            tokens.push(Token::Line(stripped.to_string()));
                        }
                        tokens.push(Token::NewLine);
                    }
                    None => {
                        if line != "" {
                            tokens.push(Token::Line(line.clone()));
                        }
                    }
                };
            }
            Err(e) => return Err(e),
        }
        line.clear();
    }

    Ok(tokens)
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
            vec!(Token::Line("hello 😁".to_string()), Token::Eof)
        );
    }

    #[test]
    fn with_new_line() {
        let mut input = io::Cursor::new("hello 😁\n".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                Token::Line("hello 😁".to_string()),
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
                Token::Line("hello 😁".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Line("yay".to_string()),
                Token::Eof
            )
        );
    }
}
