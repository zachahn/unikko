use std::io;

#[derive(Debug, PartialEq)]
pub enum LexerToken {
    Line(String),
    NewLine,
    Eof,
}

pub fn tokenize(input: &mut dyn io::BufRead) -> Result<Vec<LexerToken>, io::Error> {
    let mut tokens = Vec::<LexerToken>::new();
    let mut line = String::new();

    loop {
        match input.read_line(&mut line) {
            Ok(num) => {
                if num == 0 {
                    tokens.push(LexerToken::Eof);
                    break;
                }
                match line.strip_suffix("\n") {
                    Some(stripped) => {
                        if stripped != "" {
                            tokens.push(LexerToken::Line(stripped.to_string()));
                        }
                        tokens.push(LexerToken::NewLine);
                    }
                    None => {
                        if line != "" {
                            tokens.push(LexerToken::Line(line.clone()));
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
            vec!(LexerToken::Line("hello 😁".to_string()), LexerToken::Eof)
        );
    }

    #[test]
    fn with_new_line() {
        let mut input = io::Cursor::new("hello 😁\n".as_bytes());
        let tokens = tokenize(&mut input).unwrap();
        assert_eq!(
            tokens,
            vec!(
                LexerToken::Line("hello 😁".to_string()),
                LexerToken::NewLine,
                LexerToken::Eof
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
                LexerToken::Line("hello 😁".to_string()),
                LexerToken::NewLine,
                LexerToken::NewLine,
                LexerToken::Line("yay".to_string()),
                LexerToken::Eof
            )
        );
    }
}
